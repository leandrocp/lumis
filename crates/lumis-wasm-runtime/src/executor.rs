//! A worker pool that owns a [`Runtime`] and answers highlight and load
//! requests on its own threads.
//!
//! Hosts whose calling thread has a small stack cannot drive [`Runtime`]
//! directly. Nested injections recurse once per layer and a BEAM dirty
//! scheduler's default stack overflows rather than erroring, taking the whole
//! emulator with it. Both Lumis NIFs therefore hand work to these threads,
//! which are sized for it.

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use lumis_core::events::HighlightEvent;

use crate::catalog;
use crate::runtime::{Runtime, RuntimeError};
use crate::store::LanguageStore;

/// Deep enough for the injection nesting real documents reach; a BEAM dirty
/// scheduler gives roughly a tenth of this.
const STACK_SIZE: usize = 8 * 1024 * 1024;

/// Builds a [`Runtime`] that knows every catalog language by id and alias.
///
/// Declaring the catalog is what lets [`Runtime::highlight`] resolve a name it
/// has never loaded, so every embedder wants it and none of them should be
/// spelling the loop out again.
pub fn runtime_with_catalog(store: LanguageStore, workers: usize) -> Result<Runtime, RuntimeError> {
    let runtime = Runtime::with_worker_limit(workers)?.with_store(store);

    for language in catalog::LANGUAGES {
        runtime.declare_language(language.id, language.aliases);
    }

    Ok(runtime)
}

enum Job {
    LoadNamed {
        name: String,
        reply: mpsc::SyncSender<Result<(), RuntimeError>>,
    },
    Highlight {
        source: String,
        language: String,
        rainbow_brackets: bool,
        reply: mpsc::SyncSender<Result<Vec<HighlightEvent>, RuntimeError>>,
    },
    Precompile {
        names: Vec<String>,
        concurrency: usize,
        reply: mpsc::SyncSender<Vec<Result<(), RuntimeError>>>,
    },
}

/// Why a load failed, at the granularity a caller can act on.
///
/// A caller decides between "I typed the name wrong" and "it could not be
/// obtained"; the detail behind the second is not something a branch can use.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoadFailure {
    UnknownLanguage,
    Parser,
}

#[derive(Debug, thiserror::Error)]
pub enum ExecutorError {
    #[error("could not spawn the Lumis WASM runtime initializer: {0}")]
    Spawn(#[from] std::io::Error),
    #[error("Lumis WASM runtime initialization panicked")]
    InitPanicked,
    #[error(transparent)]
    Runtime(#[from] RuntimeError),
}

fn unavailable(count: usize) -> Vec<Result<(), RuntimeError>> {
    (0..count)
        .map(|_| {
            Err(RuntimeError::Highlight(
                "WASM executor is unavailable".into(),
            ))
        })
        .collect()
}

pub struct Executor {
    runtime: Arc<Runtime>,
    sender: mpsc::SyncSender<Job>,
}

impl Executor {
    /// Sizes the pool to the machine.
    pub fn new(store: LanguageStore) -> Result<Self, ExecutorError> {
        let workers = thread::available_parallelism().map_or(1, usize::from);

        Self::with_workers(store, workers)
    }

    pub fn with_workers(store: LanguageStore, workers: usize) -> Result<Self, ExecutorError> {
        let workers = workers.max(1);

        // Built on a worker-sized stack too: loading the catalog's queries
        // recurses as deeply as highlighting does.
        let runtime = thread::Builder::new()
            .name("lumis-wasm-init".into())
            .stack_size(STACK_SIZE)
            .spawn(move || runtime_with_catalog(store, workers))?
            .join()
            .map_err(|_| ExecutorError::InitPanicked)??;

        let runtime = Arc::new(runtime);
        let (sender, receiver) = mpsc::sync_channel::<Job>(workers * 2);
        let receiver = Arc::new(Mutex::new(receiver));

        for index in 0..workers {
            let runtime = Arc::clone(&runtime);
            let receiver = Arc::clone(&receiver);
            thread::Builder::new()
                .name(format!("lumis-wasm-{index}"))
                .stack_size(STACK_SIZE)
                .spawn(move || loop {
                    let Ok(job) = receiver.lock().expect("executor lock poisoned").recv() else {
                        return;
                    };
                    match job {
                        Job::LoadNamed { name, reply } => {
                            let _ = reply.send(runtime.load_named_language(&name));
                        }
                        Job::Highlight {
                            source,
                            language,
                            rainbow_brackets,
                            reply,
                        } => {
                            let _ =
                                reply.send(runtime.highlight(&source, &language, rainbow_brackets));
                        }
                        Job::Precompile {
                            names,
                            concurrency,
                            reply,
                        } => {
                            let _ = reply.send(runtime.precompile_languages(&names, concurrency));
                        }
                    }
                })?;
        }

        Ok(Self { runtime, sender })
    }

    /// Whether a language is resolved and held in memory.
    ///
    /// A catalog read, so it does not need the pool's stack.
    pub fn has_language(&self, name_or_alias: &str) -> bool {
        self.runtime.has_language(name_or_alias)
    }

    /// Ids of the languages resolved and held in memory, sorted.
    pub fn loaded_languages(&self) -> Vec<String> {
        self.runtime.loaded_languages()
    }

    /// Compile parsers into the module cache ahead of use.
    ///
    /// Cranelift recurses as deeply as highlighting does, so this runs on the
    /// pool's threads. Answers positionally, one result per name.
    pub fn precompile_languages(
        &self,
        names: Vec<String>,
        concurrency: usize,
    ) -> Vec<Result<(), RuntimeError>> {
        let count = names.len();
        let (reply, result) = mpsc::sync_channel(1);

        if self
            .sender
            .send(Job::Precompile {
                names,
                concurrency,
                reply,
            })
            .is_err()
        {
            return unavailable(count);
        }

        result.recv().unwrap_or_else(|_| unavailable(count))
    }

    /// Resolving a language does a TLS handshake, which needs far more stack
    /// than a small host thread has; run it on the pool's own threads.
    pub fn load_named_language(&self, name: &str) -> Result<(), LoadFailure> {
        let (reply, result) = mpsc::sync_channel(1);
        self.sender
            .send(Job::LoadNamed {
                name: name.to_string(),
                reply,
            })
            .map_err(|_| LoadFailure::Parser)?;

        match result.recv().map_err(|_| LoadFailure::Parser)? {
            Ok(()) => Ok(()),
            Err(RuntimeError::LanguageNotLoaded(_)) => Err(LoadFailure::UnknownLanguage),
            Err(_) => Err(LoadFailure::Parser),
        }
    }

    pub fn highlight(
        &self,
        source: &str,
        language: &str,
        rainbow_brackets: bool,
    ) -> Result<Vec<HighlightEvent>, RuntimeError> {
        let (reply, result) = mpsc::sync_channel(1);
        self.sender
            .send(Job::Highlight {
                source: source.to_string(),
                language: language.to_string(),
                rainbow_brackets,
                reply,
            })
            .map_err(|_| RuntimeError::Highlight("WASM executor is unavailable".into()))?;

        result.recv().map_err(|_| {
            RuntimeError::Highlight("WASM executor stopped before highlighting".into())
        })?
    }
}

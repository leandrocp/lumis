//! A worker pool that owns a [`Runtime`] and answers highlight and load
//! requests on its own threads.
//!
//! Hosts whose calling thread has a small stack cannot drive [`Runtime`]
//! directly. Nested injections recurse once per layer and a BEAM dirty
//! scheduler's default stack overflows rather than erroring, taking the whole
//! emulator with it. Both Lumis NIFs therefore hand work to these threads,
//! which are sized for it.

use std::sync::Arc;
use std::thread;

use lumis_core::events::HighlightEvent;
use rayon::{ThreadPool, ThreadPoolBuildError, ThreadPoolBuilder};

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
    #[error("could not start the Lumis WASM worker pool: {0}")]
    Pool(#[from] ThreadPoolBuildError),
    #[error("Lumis WASM runtime initialization panicked")]
    InitPanicked,
    #[error(transparent)]
    Runtime(#[from] RuntimeError),
}

pub struct Executor {
    runtime: Arc<Runtime>,
    pool: ThreadPool,
}

impl Executor {
    /// Sizes the pool to the machine.
    pub fn new(store: LanguageStore) -> Result<Self, ExecutorError> {
        let workers = thread::available_parallelism().map_or(1, usize::from);

        Self::with_workers(store, workers)
    }

    pub fn with_workers(store: LanguageStore, workers: usize) -> Result<Self, ExecutorError> {
        let workers = workers.max(1);

        let pool = ThreadPoolBuilder::new()
            .num_threads(workers)
            .stack_size(STACK_SIZE)
            .thread_name(|index| format!("lumis-wasm-{index}"))
            .build()?;

        // Built on a worker-sized stack too: loading the catalog's queries
        // recurses as deeply as highlighting does. A panic in there resumes on
        // this thread, so it is caught here rather than left to the host.
        let runtime = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            pool.install(|| runtime_with_catalog(store, workers))
        }))
        .map_err(|_| ExecutorError::InitPanicked)??;

        Ok(Self {
            runtime: Arc::new(runtime),
            pool,
        })
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
        self.pool
            .install(|| self.runtime.precompile_languages(&names, concurrency))
    }

    /// Resolving a language does a TLS handshake, which needs far more stack
    /// than a small host thread has; run it on the pool's own threads.
    pub fn load_named_language(&self, name: &str) -> Result<(), LoadFailure> {
        match self.pool.install(|| self.runtime.load_named_language(name)) {
            Ok(()) => Ok(()),
            Err(RuntimeError::LanguageNotLoaded(_)) => Err(LoadFailure::UnknownLanguage),
            Err(_) => Err(LoadFailure::Parser),
        }
    }

    /// Runs on a pool thread; the caller blocks until it is done. A panic in
    /// the highlighter resumes on the caller's thread, where the NIF's own
    /// guard catches it.
    pub fn highlight(
        &self,
        source: &str,
        language: &str,
        rainbow_brackets: bool,
    ) -> Result<Vec<HighlightEvent<'static>>, RuntimeError> {
        self.pool
            .install(|| self.runtime.highlight(source, language, rainbow_brackets))
    }
}

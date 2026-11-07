//! Highlighting off the UI thread: a parser that is not cached yet downloads and compiles
//! for seconds, and the window keeps answering while it does.

use std::ops::Range;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::{Duration, Instant};

use lumis_core::events::HighlightEvent;
use lumis_wasm_runtime::inspect::Inspection;
use lumis_wasm_runtime::{
    HighlightOptions, HttpFetcher, LanguageStore, Runtime, StoreConfig, catalog, store,
};
use rune::Waker;

pub(crate) struct Request {
    pub(crate) id: u64,
    pub(crate) language: String,
    pub(crate) text: String,
}

/// One highlight of the editor's text, and everything the window shows about it.
pub(crate) struct Analysis {
    pub(crate) id: u64,
    pub(crate) language: String,
    pub(crate) inspection: Inspection,
    /// Each run of source text with the captures open over it, outermost first.
    pub(crate) runs: Vec<Run>,
    pub(crate) unresolved: Vec<String>,
    pub(crate) elapsed: Duration,
}

pub(crate) struct Run {
    pub(crate) bytes: Range<usize>,
    /// `(scope index, language)` pairs.
    pub(crate) scopes: Vec<(usize, String)>,
}

pub(crate) struct Outcome {
    pub(crate) id: u64,
    pub(crate) result: Result<Analysis, String>,
}

pub(crate) struct Worker {
    requests: Sender<Request>,
    outcomes: Receiver<Outcome>,
}

impl Worker {
    pub(crate) fn spawn(wake: Waker) -> Self {
        let (requests, inbox) = mpsc::channel::<Request>();
        let (outbox, outcomes) = mpsc::channel();
        std::thread::spawn(move || {
            let runtime = runtime();
            while let Ok(mut request) = inbox.recv() {
                while let Ok(newer) = inbox.try_recv() {
                    request = newer;
                }
                let id = request.id;
                let result = match &runtime {
                    Ok(runtime) => analyze(runtime, request),
                    Err(error) => Err(error.clone()),
                };
                if outbox.send(Outcome { id, result }).is_err() {
                    return;
                }
                wake();
            }
        });
        Self { requests, outcomes }
    }

    pub(crate) fn send(&self, request: Request) {
        let _ = self.requests.send(request);
    }

    /// The newest outcome waiting, if any. `wait` blocks for one, for headless renders that
    /// have no event loop to wake.
    pub(crate) fn receive(&self, wait: bool) -> Option<Outcome> {
        let mut latest = if wait {
            self.outcomes.recv_timeout(Duration::from_secs(120)).ok()
        } else {
            None
        };
        while let Ok(outcome) = self.outcomes.try_recv() {
            latest = Some(outcome);
        }
        latest
    }
}

fn runtime() -> Result<Runtime, String> {
    let data_dir = store::resolve_data_dir(None);
    lumis_wasm_runtime::set_compile_cache_dir(data_dir.clone());
    std::fs::create_dir_all(data_dir.join("parsers")).map_err(|e| e.to_string())?;
    let store = LanguageStore::new(
        StoreConfig {
            cache_dir: data_dir,
            installed_dirs: None,
        },
        Box::new(HttpFetcher),
    );
    let runtime = Runtime::with_worker_limit(1)
        .map_err(|e| e.to_string())?
        .with_store(store);
    for language in catalog::LANGUAGES {
        runtime.declare_language(language.id, language.aliases);
    }
    Ok(runtime)
}

fn analyze(runtime: &Runtime, request: Request) -> Result<Analysis, String> {
    let options = HighlightOptions {
        layers: true,
        ..HighlightOptions::default()
    };
    let started = Instant::now();
    let output = runtime
        .highlight_with(&request.text, &request.language, &options)
        .map_err(|e| e.to_string())?;
    let elapsed = started.elapsed();
    let inspection = Inspection::new(&request.text, &output.layers, &output.events)
        .map_err(|e| e.to_string())?;
    Ok(Analysis {
        id: request.id,
        language: request.language,
        runs: runs(&output.events),
        inspection,
        unresolved: output.unresolved,
        elapsed,
    })
}

fn runs(events: &[HighlightEvent<'_>]) -> Vec<Run> {
    let mut open: Vec<(usize, String)> = Vec::new();
    let mut out = Vec::new();
    for event in events {
        match event {
            HighlightEvent::Start {
                scope_index,
                language,
            } => open.push((*scope_index, language.clone())),
            HighlightEvent::Source { start, end } if start < end => out.push(Run {
                bytes: *start..*end,
                scopes: open.clone(),
            }),
            HighlightEvent::End => {
                open.pop();
            }
            _ => {}
        }
    }
    out
}

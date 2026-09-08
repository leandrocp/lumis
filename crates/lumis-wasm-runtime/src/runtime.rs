//! Wasmtime-backed Tree-sitter highlighting runtime.

use lumis_core::events::HighlightEvent;
use lumis_core::highlights::HIGHLIGHT_NAMES;
use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex, OnceLock, RwLock};
use std::time::{Duration, Instant};
use thiserror::Error;
use tree_sitter::{Language, Parser, Query, Tree, WasmStore};
use wasmtime::{Cache, CacheConfig, Config, Engine};

use crate::brackets::{bracket_pairs, colorize_bracket_pairs, RainbowRange};
use crate::store::LanguageStore;
use crate::tree_sitter_highlight::{HighlightConfiguration, Highlighter};

/// Everything needed to register a parser and its highlighting queries.
#[derive(Clone)]
pub struct LanguageSpec {
    pub id: String,
    pub aliases: Vec<String>,
    pub grammar_name: String,
    pub wasm: Vec<u8>,
    pub highlights: String,
    pub injections: String,
    pub locals: String,
    pub brackets: String,
}

struct LoadedLanguage {
    highlight: HighlightConfiguration,
    brackets_source: String,
    brackets: OnceLock<Option<Query>>,
}

struct Loader {
    modules: HashMap<(String, String), Result<Language, String>>,
    store: WasmStore,
    #[cfg(test)]
    module_load_attempts: usize,
}

#[derive(Default)]
struct Catalog {
    languages: Arc<HashMap<String, Arc<LoadedLanguage>>>,
    aliases: Arc<HashMap<String, String>>,
}

struct Worker {
    highlighter: Highlighter,
}

impl Worker {
    fn new(engine: &Engine) -> Result<Self, RuntimeError> {
        let mut highlighter = Highlighter::new();
        highlighter
            .parser()
            .set_wasm_store(
                WasmStore::new(engine)
                    .map_err(|error| RuntimeError::TreeSitter(error.to_string()))?,
            )
            .map_err(|error| RuntimeError::TreeSitter(error.to_string()))?;
        Ok(Self { highlighter })
    }
}

#[derive(Default)]
struct WorkerPoolState {
    idle: Vec<Worker>,
    total: usize,
}

struct WorkerPool {
    engine: Engine,
    limit: usize,
    state: Mutex<WorkerPoolState>,
    available: Condvar,
}

impl WorkerPool {
    fn new(engine: Engine, limit: usize) -> Self {
        Self {
            engine,
            limit: limit.max(1),
            state: Mutex::new(WorkerPoolState::default()),
            available: Condvar::new(),
        }
    }

    fn engine(&self) -> &Engine {
        &self.engine
    }

    fn lease(&self) -> Result<WorkerLease<'_>, RuntimeError> {
        let mut state = self.state.lock().expect("worker pool lock poisoned");
        loop {
            if let Some(worker) = state.idle.pop() {
                return Ok(WorkerLease {
                    pool: self,
                    worker: Some(worker),
                });
            }

            if state.total < self.limit {
                state.total += 1;
                drop(state);
                match Worker::new(&self.engine) {
                    Ok(worker) => {
                        return Ok(WorkerLease {
                            pool: self,
                            worker: Some(worker),
                        });
                    }
                    Err(error) => {
                        let mut state = self.state.lock().expect("worker pool lock poisoned");
                        state.total -= 1;
                        self.available.notify_one();
                        return Err(error);
                    }
                }
            }

            state = self
                .available
                .wait(state)
                .expect("worker pool lock poisoned while waiting");
        }
    }
}

struct WorkerLease<'a> {
    pool: &'a WorkerPool,
    worker: Option<Worker>,
}

impl WorkerLease<'_> {
    fn worker(&mut self) -> &mut Worker {
        self.worker.as_mut().expect("worker lease is empty")
    }
}

impl Drop for WorkerLease<'_> {
    fn drop(&mut self) {
        if let Some(worker) = self.worker.take() {
            let mut state = self.pool.state.lock().expect("worker pool lock poisoned");
            state.idle.push(worker);
            self.pool.available.notify_one();
        }
    }
}

/// A reusable runtime with a shared Wasmtime engine, a lazy language catalog,
/// and a bounded pool of independent highlighter workers.
pub struct Runtime {
    loader: Mutex<Loader>,
    catalog: RwLock<Catalog>,
    workers: WorkerPool,
    /// Resolves, verifies and caches language packages. Present so an injected
    /// language can be loaded during the walk that discovered it, which is what
    /// lets one pass highlight a document whatever it turns out to contain.
    store: Option<LanguageStore>,
    /// One gate per language, so ten requests that all mention `rust` produce
    /// one download rather than ten. Keyed by `&'static str` so the table is
    /// bounded by the catalog: a Markdown fence can name anything, and an
    /// owned key would let a caller grow this map without limit.
    loading: Mutex<HashMap<&'static str, Arc<Mutex<()>>>>,
}

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("failed to initialize Wasmtime: {0}")]
    Wasmtime(#[from] wasmtime::Error),
    #[error("failed to initialize Tree-sitter WASM: {0}")]
    TreeSitter(String),
    #[error("failed to load parser for language '{language}': {message}")]
    Parser { language: String, message: String },
    #[error("failed to compile queries for language '{language}': {message}")]
    Query { language: String, message: String },
    #[error("language '{0}' is not loaded")]
    LanguageNotLoaded(String),
    #[error("unknown language '{0}'")]
    UnknownLanguage(String),
    #[error("language store is not configured")]
    LanguageStoreUnavailable,
    #[error("language '{0}' is not cached")]
    LanguageNotCached(String),
    #[error("highlighting failed: {0}")]
    Highlight(String),
}

impl Runtime {
    /// Construct a runtime whose concurrent highlighting is bounded by
    /// `worker_limit`, which should match the number of threads that will call
    /// it. A worker costs roughly 110 KB of resident memory, against about
    /// 15 MB for each loaded language, so this is a cheap dial.
    ///
    /// Values below one are treated as one.
    pub fn with_worker_limit(worker_limit: usize) -> Result<Self, RuntimeError> {
        static ENGINE: OnceLock<Engine> = OnceLock::new();

        if let Some(engine) = ENGINE.get() {
            return Self::with_engine(engine.clone(), worker_limit);
        }

        let engine = cached_engine()?;
        let _ = ENGINE.set(engine.clone());
        Self::with_engine(ENGINE.get().cloned().unwrap_or(engine), worker_limit)
    }

    /// Construct a runtime with an independent engine whose compiled modules
    /// are persisted under `data_dir`.
    ///
    /// This is for build-time preparation of a caller-selected data directory.
    /// The ordinary runtime uses one process-global engine and therefore cannot
    /// change its cache directory after its first use.
    ///
    /// # Errors
    /// Fails when Wasmtime or Tree-sitter's WASM store cannot be initialized.
    pub fn with_compile_cache_dir(
        worker_limit: usize,
        data_dir: std::path::PathBuf,
    ) -> Result<Self, RuntimeError> {
        let data_dir = crate::store::resolve_data_dir(Some(data_dir));
        Self::with_engine(engine_with_compile_cache(data_dir)?, worker_limit)
    }

    fn with_engine(engine: Engine, worker_limit: usize) -> Result<Self, RuntimeError> {
        let store =
            WasmStore::new(&engine).map_err(|error| RuntimeError::TreeSitter(error.to_string()))?;
        Ok(Self {
            loader: Mutex::new(Loader {
                modules: HashMap::new(),
                store,
                #[cfg(test)]
                module_load_attempts: 0,
            }),
            catalog: RwLock::new(Catalog::default()),
            workers: WorkerPool::new(engine, worker_limit),
            store: None,
            loading: Mutex::new(HashMap::new()),
        })
    }

    /// Attach the store this runtime downloads and caches through.
    #[must_use]
    pub fn with_store(mut self, store: LanguageStore) -> Self {
        self.store = Some(store);
        self
    }

    /// The store this runtime downloads and caches through, if any.
    #[must_use]
    pub fn store(&self) -> Option<&LanguageStore> {
        self.store.as_ref()
    }

    /// Resolve, download, verify and load `name` through the store.
    ///
    /// # Errors
    /// Fails when no store is attached, the name is unknown, or the package or
    /// parser cannot be obtained or verified.
    pub fn load_named_language(&self, name: &str) -> Result<(), RuntimeError> {
        self.load_through_store(name).map(|_| ())
    }

    /// Compile and validate every cached parser and its queries in `names`, up
    /// to `concurrency` at a time.
    ///
    /// Each parser gets a disposable Tree-sitter [`WasmStore`]. Loading through
    /// that store performs the link and scanner validation that compiling a raw
    /// Wasmtime module does not; building its highlight configuration validates
    /// the queries. Dropping the store after one parser avoids the shared 128 MiB
    /// address space filling across a full catalog. The load also writes the
    /// compiled module to the engine's on-disk cache.
    ///
    /// Results come back in the order the names were given, one per name.
    #[must_use]
    pub fn precompile_languages(
        &self,
        names: &[String],
        concurrency: usize,
    ) -> Vec<Result<(), RuntimeError>> {
        self.precompile_languages_detailed(names, concurrency)
            .into_iter()
            .map(|result| result.map(|_| ()))
            .collect()
    }

    /// Compile and validate every cached parser and report per-language timing.
    #[must_use]
    pub fn precompile_languages_detailed(
        &self,
        names: &[String],
        concurrency: usize,
    ) -> Vec<Result<Duration, RuntimeError>> {
        crate::parallel_map(names, concurrency, |name| {
            let started = Instant::now();
            self.precompile_language(name).map(|()| started.elapsed())
        })
    }

    /// Compile and validate one cached parser and its queries without retaining it.
    fn precompile_language(&self, name: &str) -> Result<(), RuntimeError> {
        let store = self
            .store
            .as_ref()
            .ok_or(RuntimeError::LanguageStoreUnavailable)?;
        let location =
            crate::catalog::find(name).ok_or_else(|| RuntimeError::UnknownLanguage(name.into()))?;
        let package = store
            .local_package(location.package_name)
            .ok_or_else(|| RuntimeError::LanguageNotCached(name.into()))?;
        let wasm = store
            .local_parser(&package)
            .ok_or_else(|| RuntimeError::LanguageNotCached(name.into()))?;

        let mut wasm_store = WasmStore::new(self.workers.engine())
            .map_err(|error| RuntimeError::TreeSitter(error.to_string()))?;
        let language = wasm_store
            .load_language(&package.parser.grammar_name, &wasm)
            .map_err(|error| RuntimeError::Parser {
                language: name.to_string(),
                message: error.to_string(),
            })?;
        let (id, definition) = package
            .language(name)
            .ok_or_else(|| RuntimeError::UnknownLanguage(name.into()))?;
        let mut highlight = HighlightConfiguration::new(
            language,
            id.to_string(),
            &definition.highlights,
            &definition.injections,
            &definition.locals,
        )
        .map_err(|error| RuntimeError::Query {
            language: id.to_string(),
            message: error.to_string(),
        })?;
        highlight.configure(&HIGHLIGHT_NAMES);
        Ok(())
    }

    /// Resolve, download and load `id` through the store, if one is attached.
    ///
    /// # Errors
    /// Fails when there is no store, the id is not a known language, or the
    /// package or parser cannot be obtained.
    fn load_through_store(&self, id: &str) -> Result<Arc<LoadedLanguage>, RuntimeError> {
        if let Some(loaded) = self.loaded(id) {
            return Ok(loaded);
        }

        let store = self
            .store
            .as_ref()
            .ok_or_else(|| RuntimeError::LanguageNotLoaded(id.into()))?;
        let location =
            crate::catalog::find(id).ok_or_else(|| RuntimeError::LanguageNotLoaded(id.into()))?;

        let gate = self.load_gate(location.id);
        let _guard = gate.lock().expect("language load gate poisoned");
        if let Some(loaded) = self.loaded(id) {
            return Ok(loaded);
        }

        let parser_error = |message: String| RuntimeError::Parser {
            language: id.to_string(),
            message,
        };
        let package = store
            .package(location.package_name)
            .map_err(|error| parser_error(error.to_string()))?;
        let (resolved_id, definition) = package
            .language(id)
            .ok_or_else(|| RuntimeError::LanguageNotLoaded(id.into()))?;
        let wasm = store
            .parser(&package)
            .map_err(|error| parser_error(error.to_string()))?;

        self.load_language(LanguageSpec {
            id: resolved_id.to_string(),
            aliases: definition.aliases.clone(),
            grammar_name: package.parser.grammar_name.clone(),
            wasm,
            highlights: definition.highlights.clone(),
            injections: definition.injections.clone(),
            locals: definition.locals.clone(),
            brackets: definition.brackets.clone(),
        })?;

        self.loaded(resolved_id)
            .ok_or_else(|| RuntimeError::LanguageNotLoaded(id.into()))
    }

    fn loaded(&self, name_or_alias: &str) -> Option<Arc<LoadedLanguage>> {
        let catalog = self.catalog.read().expect("language catalog lock poisoned");
        let id = catalog
            .aliases
            .get(name_or_alias)
            .map_or(name_or_alias, String::as_str);
        catalog.languages.get(id).cloned()
    }

    fn load_gate(&self, id: &'static str) -> Arc<Mutex<()>> {
        let mut loading = self.loading.lock().expect("language load table poisoned");
        Arc::clone(loading.entry(id).or_default())
    }

    #[cfg(test)]
    fn load_gate_count(&self) -> usize {
        self.loading
            .lock()
            .expect("language load table poisoned")
            .len()
    }

    pub fn load_language(&self, spec: LanguageSpec) -> Result<(), RuntimeError> {
        if self
            .catalog
            .read()
            .expect("language catalog lock poisoned")
            .languages
            .contains_key(&spec.id)
        {
            return Ok(());
        }

        // Loading is rare and WasmStore is not re-entrant. Keeping one loader
        // also makes concurrent requests for the same language idempotent.
        let mut loader = self.loader.lock().expect("language loader lock poisoned");
        if self
            .catalog
            .read()
            .expect("language catalog lock poisoned")
            .languages
            .contains_key(&spec.id)
        {
            return Ok(());
        }

        let module_key = (spec.grammar_name.clone(), crate::sha256_hex(&spec.wasm));
        let language = if let Some(module) = loader.modules.get(&module_key) {
            module.clone().map_err(|message| RuntimeError::Parser {
                language: spec.id.clone(),
                message,
            })?
        } else {
            let actual_grammar = crate::grammar_name(&spec.wasm).map_err(|error| {
                let message = error.to_string();
                loader
                    .modules
                    .insert(module_key.clone(), Err(message.clone()));
                RuntimeError::Parser {
                    language: spec.id.clone(),
                    message,
                }
            })?;
            if actual_grammar != spec.grammar_name {
                let message = format!(
                    "invalid parser grammar: expected '{}', got '{actual_grammar}'",
                    spec.grammar_name
                );
                loader.modules.insert(module_key, Err(message.clone()));
                return Err(RuntimeError::Parser {
                    language: spec.id,
                    message,
                });
            }

            #[cfg(test)]
            {
                loader.module_load_attempts += 1;
            }
            match loader.store.load_language(&spec.grammar_name, &spec.wasm) {
                Ok(language) => {
                    loader.modules.insert(module_key, Ok(language.clone()));
                    language
                }
                Err(error) => {
                    let message = error.to_string();
                    loader.modules.insert(module_key, Err(message.clone()));
                    return Err(RuntimeError::Parser {
                        language: spec.id,
                        message,
                    });
                }
            }
        };
        let mut highlight = HighlightConfiguration::new(
            language.clone(),
            spec.id.clone(),
            &spec.highlights,
            &spec.injections,
            &spec.locals,
        )
        .map_err(|error| RuntimeError::Query {
            language: spec.id.clone(),
            message: error.to_string(),
        })?;
        highlight.configure(&HIGHLIGHT_NAMES);

        let mut catalog = self
            .catalog
            .write()
            .expect("language catalog lock poisoned");
        for alias in &spec.aliases {
            Arc::make_mut(&mut catalog.aliases).insert(alias.clone(), spec.id.clone());
        }
        Arc::make_mut(&mut catalog.languages).insert(
            spec.id,
            Arc::new(LoadedLanguage {
                highlight,
                brackets_source: spec.brackets,
                brackets: OnceLock::new(),
            }),
        );
        Ok(())
    }

    #[cfg(test)]
    fn module_load_attempt_count(&self) -> usize {
        self.loader
            .lock()
            .expect("language loader lock poisoned")
            .module_load_attempts
    }

    /// Declare a supported language before its parser is loaded.
    ///
    /// Register a language's aliases so `#{lang}` in an injection query and a
    /// caller's name resolve to the same id. Declaring does not load anything.
    pub fn declare_language(&self, id: &str, aliases: &[&str]) {
        let id = id.to_string();
        let mut catalog = self
            .catalog
            .write()
            .expect("language catalog lock poisoned");
        for alias in aliases {
            Arc::make_mut(&mut catalog.aliases).insert((*alias).to_string(), id.clone());
        }
    }

    pub fn has_language(&self, name_or_alias: &str) -> bool {
        self.loaded(name_or_alias).is_some()
    }

    /// Ids of the languages resolved and held in memory, sorted.
    ///
    /// The complement of the catalog: what a host can highlight right now
    /// without touching the network.
    #[must_use]
    pub fn loaded_languages(&self) -> Vec<String> {
        let catalog = self.catalog.read().expect("language catalog lock poisoned");
        let mut ids: Vec<String> = catalog.languages.keys().cloned().collect();
        ids.sort();
        ids
    }

    pub fn configure_language(
        &self,
        name_or_alias: &str,
        highlights: &str,
        injections: &str,
        locals: &str,
    ) -> Result<(), RuntimeError> {
        let (id, language, brackets_source) = {
            let catalog = self.catalog.read().expect("language catalog lock poisoned");
            let id = catalog
                .aliases
                .get(name_or_alias)
                .cloned()
                .unwrap_or_else(|| name_or_alias.to_string());
            let loaded = catalog
                .languages
                .get(&id)
                .ok_or_else(|| RuntimeError::LanguageNotLoaded(name_or_alias.to_string()))?;
            (
                id,
                loaded.highlight.language.clone(),
                loaded.brackets_source.clone(),
            )
        };
        let mut highlight =
            HighlightConfiguration::new(language, id.clone(), highlights, injections, locals)
                .map_err(|error| RuntimeError::Query {
                    language: id.clone(),
                    message: error.to_string(),
                })?;
        highlight.configure(&HIGHLIGHT_NAMES);

        let mut catalog = self
            .catalog
            .write()
            .expect("language catalog lock poisoned");
        Arc::make_mut(&mut catalog.languages).insert(
            id,
            Arc::new(LoadedLanguage {
                highlight,
                brackets_source,
                brackets: OnceLock::new(),
            }),
        );
        Ok(())
    }

    pub fn highlight(
        &self,
        source: &str,
        name_or_alias: &str,
        rainbow_brackets: bool,
    ) -> Result<Vec<HighlightEvent<'static>>, RuntimeError> {
        self.highlight_with(
            source,
            name_or_alias,
            &HighlightOptions {
                rainbow_brackets,
                ..HighlightOptions::default()
            },
        )
        .map(|output| output.events)
    }

    /// Parse `source` without highlighting it, for tools that want the tree.
    ///
    /// # Errors
    /// Fails when the language cannot be loaded or the parser returns no tree.
    pub fn parse_tree(&self, source: &str, name_or_alias: &str) -> Result<Tree, RuntimeError> {
        let loaded = self.load_through_store(name_or_alias)?;
        let mut lease = self.workers.lease()?;
        let parser = lease.worker().highlighter.parser();
        parser
            .set_language(&loaded.highlight.language)
            .map_err(|error| RuntimeError::Parser {
                language: name_or_alias.to_string(),
                message: error.to_string(),
            })?;
        parser.parse(source.as_bytes(), None).ok_or_else(|| {
            RuntimeError::Highlight(format!("parser returned no tree for '{name_or_alias}'"))
        })
    }

    /// Highlight, with control over injections and the parsed layers.
    ///
    /// # Errors
    /// Fails when the root language cannot be loaded, or highlighting does.
    pub fn highlight_with(
        &self,
        source: &str,
        name_or_alias: &str,
        options: &HighlightOptions,
    ) -> Result<HighlightOutput, RuntimeError> {
        self.highlight_with_resolver(source, name_or_alias, options, |_| {
            InjectionResolution::Fallback
        })
    }

    /// Highlight while allowing a host to resolve an injected public id to a
    /// language it loaded into this runtime.
    ///
    /// The callback runs during the same tree walk that discovered the
    /// injection. [`InjectionResolution::Loaded`] only selects an
    /// already-loaded language; the host performs any resolution and calls
    /// [`Self::load_language`] before it returns.
    ///
    /// # Errors
    /// Fails when the root language cannot be loaded, or highlighting does.
    pub fn highlight_with_resolver(
        &self,
        source: &str,
        name_or_alias: &str,
        options: &HighlightOptions,
        mut resolve_injected: impl FnMut(&str) -> InjectionResolution,
    ) -> Result<HighlightOutput, RuntimeError> {
        let root = self.load_through_store(name_or_alias)?;
        let (root_id, languages, aliases) = {
            let catalog = self.catalog.read().expect("language catalog lock poisoned");
            let root_id = catalog
                .aliases
                .get(name_or_alias)
                .cloned()
                .unwrap_or_else(|| name_or_alias.to_string());
            (
                root_id,
                Arc::clone(&catalog.languages),
                Arc::clone(&catalog.aliases),
            )
        };

        let mut lease = self.workers.lease()?;
        let worker = lease.worker();
        worker.highlighter.record_parsed_layers(options.layers);
        // Holds languages loaded during this walk. The callback has to hand back a
        // reference that outlives it, and an arena gives a stable address while
        // still allowing inserts, which a RefCell<Vec<_>> cannot.
        let loaded_here: typed_arena::Arena<Arc<LoadedLanguage>> = typed_arena::Arena::new();
        let unresolved: std::cell::RefCell<Vec<String>> = std::cell::RefCell::new(Vec::new());

        // An injection query can name something that is not a language at all.
        // html captures the raw `<script type=...>` value, so `type="module"`
        // asks for "module" and `type="importmap"` for "importmap", each just
        // before a more specific pattern injects javascript or json into the
        // same block. Those blocks do highlight, so reporting the discarded
        // name would warn about output that is correct.
        let record_unresolved = |name: &str| {
            if crate::catalog::find(name).is_none() {
                return;
            }
            let mut unresolved = unresolved.borrow_mut();
            if !unresolved.iter().any(|existing| existing == name) {
                unresolved.push(name.to_string());
            }
        };

        let events = worker
            .highlighter
            .highlight(&root.highlight, source.as_bytes(), None, |injected| {
                if !options.injections {
                    return None;
                }

                // A host resolver has precedence over the process catalog. In
                // particular, two Node highlighters may resolve the same
                // public id to different packages, each loaded under its own
                // internal id. Calling this before the shared alias map keeps
                // one instance from selecting another instance's definition.
                match resolve_injected(injected) {
                    InjectionResolution::Loaded(resolved) => {
                        if let Some(loaded) = self.loaded(&resolved) {
                            return Some(&loaded_here.alloc(loaded).highlight);
                        }
                    }
                    InjectionResolution::Unresolved => {
                        record_unresolved(injected);
                        return None;
                    }
                    InjectionResolution::Fallback => {}
                }

                let id = aliases.get(injected).map_or(injected, String::as_str);

                if let Some(loaded) = languages.get(id) {
                    return Some(&loaded.highlight);
                }

                // Loading here is what makes one pass enough: the walk descends
                // into the language it just fetched and finds whatever that
                // contains, however deeply nested. A language that cannot be
                // fetched leaves its block unhighlighted rather than failing the
                // document around it.
                if let Ok(loaded) = self.load_through_store(id) {
                    Some(&loaded_here.alloc(loaded).highlight)
                } else {
                    record_unresolved(id);
                    None
                }
            })
            .map_err(|error| RuntimeError::Highlight(error.to_string()))?;

        let mut collected = Vec::new();
        for event in events {
            match event.map_err(|error| RuntimeError::Highlight(error.to_string()))? {
                crate::tree_sitter_highlight::HighlightEvent::Source { start, end } => {
                    collected.push(HighlightEvent::Source { start, end });
                }
                crate::tree_sitter_highlight::HighlightEvent::HighlightStart {
                    highlight,
                    language,
                } => collected.push(HighlightEvent::Start {
                    scope_index: highlight.0,
                    language,
                }),
                crate::tree_sitter_highlight::HighlightEvent::HighlightEnd => {
                    collected.push(HighlightEvent::End);
                }
            }
        }

        if options.rainbow_brackets {
            let ranges = rainbow_ranges(worker.highlighter.parser(), &root, source)?;
            collected = apply_rainbow_brackets(collected, ranges, &root_id);
        }

        Ok(HighlightOutput {
            events: collected,
            layers: worker.highlighter.take_parsed_layers(),
            unresolved: unresolved.into_inner(),
        })
    }
}

/// How a host handled a language named by an injection query.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InjectionResolution {
    /// The host has no override, so the runtime should use its catalog store.
    Fallback,
    /// The host loaded the language under this internal id.
    Loaded(String),
    /// The host owns this resolution but could not load the language.
    Unresolved,
}

/// What a highlight pass should do beyond producing events.
#[derive(Debug)]
pub struct HighlightOptions {
    /// Colourize matching bracket pairs in the root language.
    pub rainbow_brackets: bool,
    /// Descend into languages injected inside the document.
    pub injections: bool,
    /// Keep the tree parsed for each layer, for tools that inspect them.
    pub layers: bool,
}

impl Default for HighlightOptions {
    fn default() -> Self {
        Self {
            rainbow_brackets: false,
            injections: true,
            layers: false,
        }
    }
}

/// The result of a highlight pass. `layers` is empty unless
/// [`HighlightOptions::layers`] asked for them.
pub struct HighlightOutput {
    pub events: Vec<HighlightEvent<'static>>,
    pub layers: Vec<crate::tree_sitter_highlight::ParsedLayer>,
    /// Injected languages this walk found but could not load, so a caller that
    /// resolves parsers itself can tell what the store could not reach. Empty
    /// when everything the document named was available.
    pub unresolved: Vec<String>,
}

static COMPILE_CACHE_DIR: RwLock<Option<std::path::PathBuf>> = RwLock::new(None);

/// Keep compiled parser modules under `dir`, alongside the parsers themselves.
///
/// The engine is process-global and built once, so the last value set before the
/// first [`Runtime`] is the one that takes effect and later calls do nothing.
/// Callers that resolve a data directory of their own should pass it here,
/// otherwise the cache falls back to [`store::resolve_data_dir`] and a
/// caller-supplied directory would hold the parsers while their compiled forms
/// went somewhere unrelated.
pub fn set_compile_cache_dir(dir: std::path::PathBuf) {
    *COMPILE_CACHE_DIR
        .write()
        .expect("compile cache lock poisoned") = Some(dir);
}

fn cached_engine() -> Result<Engine, wasmtime::Error> {
    let root = crate::store::resolve_data_dir(
        COMPILE_CACHE_DIR
            .read()
            .expect("compile cache lock poisoned")
            .clone(),
    );
    engine_with_compile_cache(root)
}

fn engine_with_compile_cache(root: std::path::PathBuf) -> Result<Engine, wasmtime::Error> {
    let mut config = Config::new();
    let mut cache_config = CacheConfig::new();
    cache_config.with_directory(root.join("compiled"));

    if let Ok(cache) = Cache::new(cache_config) {
        config.cache(Some(cache));
    }

    Engine::new(&config)
}

fn rainbow_ranges(
    parser: &mut Parser,
    language: &LoadedLanguage,
    source: &str,
) -> Result<Vec<RainbowRange>, RuntimeError> {
    let query = language.brackets.get_or_init(|| {
        crate::brackets::compile(&language.highlight.language, &language.brackets_source)
    });
    let Some(query) = query else {
        return Ok(Vec::new());
    };

    parser
        .set_language(&language.highlight.language)
        .map_err(|error| RuntimeError::TreeSitter(error.to_string()))?;
    let Some(tree) = parser.parse(source.as_bytes(), None) else {
        return Ok(Vec::new());
    };

    let pairs = bracket_pairs(query, tree.root_node(), source.as_bytes());
    Ok(colorize_bracket_pairs(pairs))
}

fn apply_rainbow_brackets(
    events: Vec<HighlightEvent<'static>>,
    ranges: Vec<RainbowRange>,
    language: &str,
) -> Vec<HighlightEvent<'static>> {
    if ranges.is_empty() {
        return events;
    }

    let mut output = Vec::new();
    let mut range_index = 0usize;
    for event in events {
        let HighlightEvent::Source { start, end } = event else {
            output.push(event);
            continue;
        };

        let mut source_cursor = start;
        while range_index < ranges.len() && ranges[range_index].end <= start {
            range_index += 1;
        }

        let mut next_index = range_index;
        while let Some(range) = ranges.get(next_index) {
            if range.start >= end {
                break;
            }
            if range.start < start || range.end > end {
                next_index += 1;
                continue;
            }

            if source_cursor < range.start {
                output.push(HighlightEvent::Source {
                    start: source_cursor,
                    end: range.start,
                });
            }
            output.push(HighlightEvent::Start {
                scope_index: range.scope_index,
                language: language.to_string(),
            });
            output.push(HighlightEvent::Source {
                start: range.start,
                end: range.end,
            });
            output.push(HighlightEvent::End);
            source_cursor = range.end;
            next_index += 1;
        }

        if source_cursor < end {
            output.push(HighlightEvent::Source {
                start: source_cursor,
                end,
            });
        }
    }
    output
}

// A table-driven test's branches are its coverage; splitting one to satisfy the
// cognitive complexity ceiling would hide what it asserts.
#[allow(clippy::cognitive_complexity)]
#[cfg(test)]
// Every `.wasm`/`.tmp` suffix compared below is one this module built itself.
#[allow(clippy::case_sensitive_file_extension_comparisons)]
mod tests {
    use super::*;
    use crate::store::StoreConfig;
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    const JSON_WASM: &[u8] = include_bytes!("../../../fixtures/test-parsers/tree-sitter-json.wasm");
    const INVALID_TREE_SITTER_WASM: &[u8] = &[
        0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x05, 0x01, 0x60, 0x00, 0x01, 0x7f,
        0x03, 0x02, 0x01, 0x00, 0x07, 0x16, 0x01, 0x12, 0x74, 0x72, 0x65, 0x65, 0x5f, 0x73, 0x69,
        0x74, 0x74, 0x65, 0x72, 0x5f, 0x62, 0x72, 0x6f, 0x6b, 0x65, 0x6e, 0x00, 0x00, 0x0a, 0x06,
        0x01, 0x04, 0x00, 0x41, 0x00, 0x0b,
    ];

    fn package(
        package_name: &str,
        language: &str,
        grammar_name: &str,
        wasm: &[u8],
    ) -> crate::LanguagePackage {
        crate::LanguagePackage {
            package_name: package_name.into(),
            version: crate::lowest_compatible_package_version(),
            definition_hash: "test".into(),
            parser: crate::ParserMetadata {
                name: format!("tree-sitter-{grammar_name}"),
                grammar_name: grammar_name.into(),
                upstream_version: None,
                revision: None,
                sha256: crate::sha256_hex(wasm),
                size: u64::try_from(wasm.len()).expect("parser size fits in u64"),
            },
            languages: std::collections::BTreeMap::from([(
                language.into(),
                crate::PackagedLanguage {
                    highlights: "(string) @string".into(),
                    ..crate::PackagedLanguage::default()
                },
            )]),
        }
    }

    fn json_package(wasm: &[u8]) -> crate::LanguagePackage {
        package("@lumis-sh/wasm-json", "json", "json", wasm)
    }

    fn cache_package(store: &LanguageStore, package: &crate::LanguagePackage, wasm: &[u8]) {
        store.cache_package(package).unwrap();
        crate::write_atomic(&store.parser_path(package).unwrap(), wasm).unwrap();
    }

    fn install_json(runtime: &Runtime, injections: &str) {
        let language = tree_sitter::Language::new(tree_sitter_json::LANGUAGE);
        let mut highlight =
            HighlightConfiguration::new(language, "json", "(string) @string", injections, "")
                .unwrap();
        highlight.configure(&HIGHLIGHT_NAMES);
        Arc::make_mut(&mut runtime.catalog.write().unwrap().languages).insert(
            "json".into(),
            Arc::new(LoadedLanguage {
                highlight,
                brackets_source: String::new(),
                brackets: OnceLock::new(),
            }),
        );
    }

    #[test]
    fn a_parser_module_survives_query_failure_for_a_corrected_retry() {
        let runtime = Runtime::with_worker_limit(1).unwrap();
        let spec = LanguageSpec {
            id: "retry-json".into(),
            aliases: Vec::new(),
            grammar_name: "json".into(),
            wasm: JSON_WASM.to_vec(),
            highlights: "(definitely_not_a_json_node) @string".into(),
            injections: String::new(),
            locals: String::new(),
            brackets: String::new(),
        };

        assert!(matches!(
            runtime.load_language(spec.clone()),
            Err(RuntimeError::Query { .. })
        ));
        let mut second_invalid = spec.clone();
        second_invalid.id = "retry-json-second".into();
        assert!(matches!(
            runtime.load_language(second_invalid),
            Err(RuntimeError::Query { .. })
        ));

        let mut corrected = spec;
        corrected.id = "retry-json-corrected".into();
        corrected.highlights = "(number) @number".into();
        runtime.load_language(corrected).unwrap();
        assert_eq!(
            runtime.module_load_attempt_count(),
            1,
            "query retries over identical parser bytes must not grow the Wasm store"
        );

        let wrong_grammar = LanguageSpec {
            id: "retry-json-wrong-grammar".into(),
            aliases: Vec::new(),
            grammar_name: "not_json".into(),
            wasm: JSON_WASM.to_vec(),
            highlights: "(number) @number".into(),
            injections: String::new(),
            locals: String::new(),
            brackets: String::new(),
        };
        let error = runtime.load_language(wrong_grammar.clone()).unwrap_err();
        assert!(matches!(
            error,
            RuntimeError::Parser { language, message }
                if language == "retry-json-wrong-grammar"
                    && message == "invalid parser grammar: expected 'not_json', got 'json'"
        ));
        let mut repeated_wrong_grammar = wrong_grammar;
        repeated_wrong_grammar.id = "retry-json-wrong-grammar-again".into();
        assert!(matches!(
            runtime.load_language(repeated_wrong_grammar),
            Err(RuntimeError::Parser { language, message })
                if language == "retry-json-wrong-grammar-again"
                    && message == "invalid parser grammar: expected 'not_json', got 'json'"
        ));
        assert_eq!(
            runtime.module_load_attempt_count(),
            1,
            "a grammar mismatch must be rejected before the Wasm store grows"
        );
    }

    #[test]
    fn a_failed_parser_module_load_is_cached_by_content() {
        let runtime = Runtime::with_worker_limit(1).unwrap();
        let spec = LanguageSpec {
            id: "broken-first".into(),
            aliases: Vec::new(),
            grammar_name: "broken".into(),
            wasm: INVALID_TREE_SITTER_WASM.to_vec(),
            highlights: "(_) @variable".into(),
            injections: String::new(),
            locals: String::new(),
            brackets: String::new(),
        };

        let first_error = runtime.load_language(spec.clone()).unwrap_err();
        let mut repeated = spec;
        repeated.id = "broken-second".into();
        let second_error = runtime.load_language(repeated).unwrap_err();
        assert!(matches!(
            (&first_error, &second_error),
            (
                RuntimeError::Parser {
                    language: first_language,
                    message: first_message,
                },
                RuntimeError::Parser {
                    language: second_language,
                    message: second_message,
                },
            ) if first_language == "broken-first"
                && second_language == "broken-second"
                && first_message == second_message
        ));
        assert_eq!(runtime.module_load_attempt_count(), 1);
    }

    #[test]
    fn precompile_validates_loadability_and_continues_after_a_failure() {
        let dir = tempfile::tempdir().unwrap();
        let store = LanguageStore::new(
            StoreConfig {
                cache_dir: dir.path().to_path_buf(),
            },
            Box::new(crate::NoNetwork),
        );
        let invalid = package(
            "@lumis-sh/wasm-json",
            "json",
            "broken",
            INVALID_TREE_SITTER_WASM,
        );
        let valid = package("@lumis-sh/wasm-diff", "diff", "json", JSON_WASM);
        cache_package(&store, &invalid, INVALID_TREE_SITTER_WASM);
        cache_package(&store, &valid, JSON_WASM);

        let runtime = Runtime::with_worker_limit(1).unwrap().with_store(store);
        assert!(wasmtime::Module::new(runtime.workers.engine(), INVALID_TREE_SITTER_WASM).is_ok());

        let names = vec!["json".to_string(), "diff".to_string()];
        let results = runtime.precompile_languages(&names, 1);
        assert!(matches!(
            &results[0],
            Err(RuntimeError::Parser { language, .. }) if language == "json"
        ));
        assert!(results[1].is_ok(), "later parsers must still be attempted");
    }

    #[test]
    fn precompile_validates_queries() {
        let dir = tempfile::tempdir().unwrap();
        let store = LanguageStore::new(
            StoreConfig {
                cache_dir: dir.path().to_path_buf(),
            },
            Box::new(crate::NoNetwork),
        );
        let mut invalid = json_package(JSON_WASM);
        invalid.languages.get_mut("json").unwrap().highlights =
            "(definitely_not_a_json_node) @string".into();
        cache_package(&store, &invalid, JSON_WASM);

        let runtime = Runtime::with_worker_limit(1).unwrap().with_store(store);
        let results = runtime.precompile_languages(&["json".to_string()], 1);
        assert!(matches!(
            &results[0],
            Err(RuntimeError::Query { language, .. }) if language == "json"
        ));
    }

    #[test]
    fn precompile_requires_a_cached_parser_even_when_the_language_is_loaded() {
        let dir = tempfile::tempdir().unwrap();
        let store = LanguageStore::new(
            StoreConfig {
                cache_dir: dir.path().to_path_buf(),
            },
            Box::new(crate::NoNetwork),
        );
        let runtime = Runtime::with_worker_limit(1).unwrap().with_store(store);
        install_json(&runtime, "");

        let results = runtime.precompile_languages(&["json".to_string()], 1);
        assert!(matches!(
            &results[0],
            Err(RuntimeError::LanguageNotCached(language)) if language == "json"
        ));
    }

    #[test]
    fn worker_limit_allows_two_independent_leases() {
        let runtime = Arc::new(Runtime::with_worker_limit(2).unwrap());
        let (ready_tx, ready_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        let release_rx = Arc::new(Mutex::new(release_rx));
        let mut threads = Vec::new();

        for _ in 0..2 {
            let runtime = Arc::clone(&runtime);
            let ready_tx = ready_tx.clone();
            let release_rx = Arc::clone(&release_rx);
            threads.push(thread::spawn(move || {
                let _lease = runtime.workers.lease().unwrap();
                ready_tx.send(()).unwrap();
                release_rx.lock().unwrap().recv().unwrap();
            }));
        }

        ready_rx.recv_timeout(Duration::from_secs(5)).unwrap();
        ready_rx.recv_timeout(Duration::from_secs(5)).unwrap();
        release_tx.send(()).unwrap();
        release_tx.send(()).unwrap();
        for thread in threads {
            thread.join().unwrap();
        }
    }

    #[test]
    fn highlights_safely_from_multiple_threads() {
        let runtime = Arc::new(Runtime::with_worker_limit(2).unwrap());
        install_json(&runtime, "");
        let mut threads = Vec::new();

        for _ in 0..8 {
            let runtime = Arc::clone(&runtime);
            threads.push(thread::spawn(move || {
                let events = runtime
                    .highlight(r#"{"language":"json","value":42}"#, "json", false)
                    .unwrap();
                assert!(!events.is_empty(), "highlighting produced no events");
            }));
        }

        for thread in threads {
            thread.join().unwrap();
        }
    }

    /// The walk loads what it finds, so one call is enough however deep the
    /// injections go, and a language it cannot fetch costs that block its
    /// highlighting rather than failing the whole document.
    #[test]
    fn an_injected_language_is_loaded_during_the_walk() {
        let dir = tempfile::tempdir().unwrap();
        let parsers = dir.path().join("parsers");
        std::fs::create_dir_all(&parsers).unwrap();

        let store = LanguageStore::new(
            StoreConfig {
                cache_dir: dir.path().to_path_buf(),
            },
            Box::new(crate::store::NoNetwork),
        );
        let runtime = Runtime::with_worker_limit(1).unwrap().with_store(store);
        runtime.declare_language("missing", &[]);
        install_json(
            &runtime,
            r#"((string) @injection.content
               (#set! injection.language "missing"))"#,
        );

        // Nothing is fetchable, so this must still succeed rather than fail the
        // document; the injected block simply stays unhighlighted.
        let events = runtime.highlight(r#""embedded""#, "json", false).unwrap();
        assert!(!events.is_empty(), "highlighting produced no events");
    }

    #[test]
    fn an_unloaded_injected_language_is_left_unhighlighted() {
        let runtime = Runtime::with_worker_limit(1).unwrap();
        runtime.declare_language("missing", &[]);
        install_json(
            &runtime,
            r#"((string) @injection.content
               (#set! injection.language "missing"))"#,
        );

        let events = runtime.highlight(r#""embedded""#, "json", false).unwrap();
        assert!(
            events
                .iter()
                .all(|event| !matches!(event, HighlightEvent::Start { language, .. } if language == "missing")),
            "the unloaded language must not appear: {events:?}"
        );
        assert!(
            !events.is_empty(),
            "the rest of the document still highlights"
        );
    }

    /// html asks for "module" and "importmap" before a later pattern injects
    /// javascript or json into the same block, so a name the catalog does not
    /// know is not evidence that anything failed to highlight.
    #[test]
    fn only_catalog_languages_are_reported_unresolved() {
        let dir = tempfile::tempdir().unwrap();
        let store = LanguageStore::new(
            StoreConfig {
                cache_dir: dir.path().to_path_buf(),
            },
            Box::new(crate::store::NoNetwork),
        );
        let runtime = Runtime::with_worker_limit(1).unwrap().with_store(store);
        install_json(
            &runtime,
            r"(pair
                 key: (string (string_content) @injection.language)
                 value: (string (string_content) @injection.content))",
        );

        let output = runtime
            .highlight_with(
                r#"{"module":"a","importmap":"b","rust":"c"}"#,
                "json",
                &HighlightOptions {
                    injections: true,
                    ..HighlightOptions::default()
                },
            )
            .unwrap();

        assert!(
            crate::catalog::find("rust").is_some()
                && crate::catalog::find("module").is_none()
                && crate::catalog::find("importmap").is_none(),
            "the test needs one catalog language and two names that are not"
        );
        assert_eq!(
            output.unresolved,
            vec!["rust".to_string()],
            "a catalog language that could not be fetched is worth reporting; \
             a name that is not a language at all is not"
        );
    }

    /// A Markdown fence names its own language, so the injected name is
    /// attacker-controlled. Failing to load one must not leave anything behind.
    #[test]
    fn unknown_injected_languages_leave_no_load_gate() {
        let dir = tempfile::tempdir().unwrap();
        let store = LanguageStore::new(
            StoreConfig {
                cache_dir: dir.path().to_path_buf(),
            },
            Box::new(crate::store::NoNetwork),
        );
        let runtime = Runtime::with_worker_limit(1).unwrap().with_store(store);
        install_json(
            &runtime,
            r"(pair
                 key: (string (string_content) @injection.language)
                 value: (string (string_content) @injection.content))",
        );

        let document = (0..200)
            .map(|index| format!(r#""no-such-language-{index}":"body""#))
            .collect::<Vec<_>>()
            .join(",");
        let events = runtime
            .highlight(&format!("{{{document}}}"), "json", false)
            .unwrap();
        assert!(!events.is_empty(), "the document still highlights");
        assert_eq!(
            runtime.load_gate_count(),
            0,
            "200 unknown injected names must not allocate 200 gates"
        );

        for index in 0..200 {
            assert!(runtime
                .load_named_language(&format!("no-such-language-{index}"))
                .is_err());
        }
        assert_eq!(runtime.load_gate_count(), 0, "the public path leaks too");

        // The other direction: a catalog language still takes a gate, so this
        // test fails if the deduplication it protects is removed outright.
        assert!(runtime.load_named_language("rust").is_err());
        assert_eq!(runtime.load_gate_count(), 1);
        assert!(runtime.load_named_language("rs").is_err());
        assert_eq!(
            runtime.load_gate_count(),
            1,
            "an alias shares the canonical id's gate"
        );
    }

    /// Ten requests naming the same uncached language must not become ten
    /// downloads of it.
    #[test]
    fn concurrent_loads_of_one_language_fetch_it_once() {
        struct CountingFetcher {
            wasm: Vec<u8>,
            parser_requests: Arc<Mutex<usize>>,
        }

        impl crate::store::Fetcher for CountingFetcher {
            fn get(&self, url: &str) -> Result<Vec<u8>, String> {
                if url.ends_with(".wasm") {
                    *self.parser_requests.lock().unwrap() += 1;
                    // Wide enough that an ungated peer would start its own.
                    thread::sleep(Duration::from_millis(50));
                    return Ok(self.wasm.clone());
                }
                Ok(serde_json::to_vec(&json_package(&self.wasm)).unwrap())
            }
        }

        let dir = tempfile::tempdir().unwrap();
        let parser_requests = Arc::new(Mutex::new(0));
        let store = LanguageStore::new(
            StoreConfig {
                cache_dir: dir.path().to_path_buf(),
            },
            Box::new(CountingFetcher {
                wasm: JSON_WASM.to_vec(),
                parser_requests: Arc::clone(&parser_requests),
            }),
        );
        let runtime = Arc::new(Runtime::with_worker_limit(4).unwrap().with_store(store));

        let threads: Vec<_> = (0..8)
            .map(|_| {
                let runtime = Arc::clone(&runtime);
                thread::spawn(move || runtime.load_named_language("json").unwrap())
            })
            .collect();
        for thread in threads {
            thread.join().unwrap();
        }

        assert_eq!(*parser_requests.lock().unwrap(), 1);
    }

    /// Without a store there is nowhere to load from, so the caller is told
    /// rather than handed an unhighlighted document.
    #[test]
    fn the_root_language_must_be_loaded() {
        let runtime = Runtime::with_worker_limit(1).unwrap();
        runtime.declare_language("json", &[]);
        let error = runtime.highlight("{}", "json", false).unwrap_err();
        assert!(matches!(
            error,
            RuntimeError::LanguageNotLoaded(language) if language == "json"
        ));
    }

    #[test]
    fn ignores_undeclared_query_specific_injections() {
        let runtime = Runtime::with_worker_limit(1).unwrap();
        install_json(
            &runtime,
            r#"((string) @injection.content
               (#set! injection.language "printf"))"#,
        );

        let events = runtime.highlight(r#""%s""#, "json", false).unwrap();
        assert!(!events.is_empty(), "highlighting produced no events");
    }

    #[test]
    fn reuses_a_worker_for_wasm_highlighting() {
        let runtime = Runtime::with_worker_limit(1).unwrap();
        runtime
            .load_language(LanguageSpec {
                id: "diff".into(),
                aliases: Vec::new(),
                grammar_name: "diff".into(),
                wasm: include_bytes!("../../../fixtures/test-parsers/tree-sitter-diff.wasm")
                    .to_vec(),
                highlights: "(addition) @diff.plus\n(deletion) @diff.minus".into(),
                injections: String::new(),
                locals: String::new(),
                brackets: String::new(),
            })
            .unwrap();

        for _ in 0..2 {
            let events = runtime
                .highlight("+ added\n- removed", "diff", false)
                .unwrap();
            assert!(!events.is_empty(), "highlighting produced no events");
        }
    }

    #[test]
    fn reuses_elixir_wasm_highlighting() {
        let runtime = Runtime::with_worker_limit(1).unwrap();
        runtime
            .load_language(LanguageSpec {
                id: "elixir".into(),
                aliases: Vec::new(),
                grammar_name: "elixir".into(),
                wasm: include_bytes!("../../../fixtures/test-parsers/tree-sitter-elixir.wasm")
                    .to_vec(),
                highlights: "(identifier) @variable\n(alias) @module".into(),
                injections: String::new(),
                locals: String::new(),
                brackets: String::new(),
            })
            .unwrap();

        for source in [
            "defmodule Test do\n  @lang :elixir\nend",
            "defmodule Test do\nend",
        ] {
            let events = runtime.highlight(source, "elixir", false).unwrap();
            assert!(!events.is_empty(), "highlighting produced no events");
        }
    }
}

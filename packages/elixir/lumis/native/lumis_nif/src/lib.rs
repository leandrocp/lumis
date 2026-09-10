use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::io::Write;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

mod elixir;

use anyhow::{anyhow, Context, Result};
use elixir::{ExCssOptions, ExFormatterOption, ExStyle, ExTheme};
use lumis_core::annotations::{compose_annotations, Annotation, AnnotationRange, Position};
use lumis_core::events::HighlightEvent;
use lumis_core::formatter::Formatter;
use lumis_core::languages::Language;
use lumis_core::{languages, themes};
use lumis_wasm_runtime::{catalog, store, Runtime, RuntimeError};
use parking_lot::RwLock;
use rustler::{Encoder, Env, Error, NifMap, NifResult, NifStruct, Term};

/// Lazy per-theme cache to eliminate repeated allocations.
/// Themes are converted and cached on first access, amortizing the cost.
static THEME_CACHE: std::sync::LazyLock<RwLock<HashMap<String, ExTheme>>> =
    std::sync::LazyLock::new(|| RwLock::new(HashMap::new()));

// `LazyLock::get`, which `configure_store` needs, is newer than the MSRV.
#[allow(clippy::non_std_lazy_statics)]
static EXECUTOR: Lazy<Result<WasmExecutor>> = Lazy::new(WasmExecutor::new);
static CACHE_BATCH: std::sync::LazyLock<parking_lot::Mutex<()>> =
    std::sync::LazyLock::new(|| parking_lot::Mutex::new(()));
static PRECOMPILE_BATCH: std::sync::LazyLock<parking_lot::Mutex<()>> =
    std::sync::LazyLock::new(|| parking_lot::Mutex::new(()));

enum WasmJob {
    LoadNamed {
        name: String,
        reply: mpsc::SyncSender<Result<(), RuntimeError>>,
    },
    Highlight {
        source: String,
        language: String,
        rainbow_brackets: bool,
        reply: mpsc::SyncSender<Result<Vec<HighlightEvent<'static>>, RuntimeError>>,
    },
}

/// Why a load failed, at the granularity Elixir matches on.
///
/// A caller decides between "I typed the name wrong" and "it could not be
/// obtained"; the detail behind the second is not something a `case` can act on.
enum LoadFailure {
    UnknownLanguage,
    Parser,
}

impl Encoder for LoadFailure {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        match self {
            Self::UnknownLanguage => unknown_language().encode(env),
            Self::Parser => failed_to_load_parser().encode(env),
        }
    }
}

/// Directories the store reads and writes, as `Lumis.Application` configured
/// them. Elixir cannot set an OS environment variable the emulator's NIFs can
/// see, so `config :lumis` arrives here instead.
#[derive(Default)]
struct StorePaths {
    data_dir: Option<std::path::PathBuf>,
}

static STORE_PATHS: std::sync::LazyLock<RwLock<StorePaths>> =
    std::sync::LazyLock::new(|| RwLock::new(StorePaths::default()));

/// The same resolve, verify and cache path the CLI uses, pointed at the
/// directories Lumis persists under.
fn language_store(cache_dir: Option<std::path::PathBuf>) -> store::LanguageStore {
    let paths = STORE_PATHS.read();
    let cache_dir = store::resolve_data_dir(cache_dir.or_else(|| paths.data_dir.clone()));
    store::LanguageStore::new(
        store::StoreConfig { cache_dir },
        Box::new(store::HttpFetcher),
    )
}

struct WasmExecutor {
    runtime: Arc<Runtime>,
    sender: mpsc::SyncSender<WasmJob>,
}

impl WasmExecutor {
    fn new() -> Result<Self> {
        // Sized to the machine, not capped. These threads exist for their 8 MiB
        // stacks: nested injections recurse per layer and overflow the BEAM
        // dirty-scheduler default, which crashes the VM rather than erroring.
        let workers = thread::available_parallelism().map_or(1, usize::from);
        let runtime = thread::Builder::new()
            .name("lumis-wasm-init".into())
            .stack_size(8 * 1024 * 1024)
            .spawn(move || -> Result<Runtime, RuntimeError> {
                let runtime = Runtime::with_worker_limit(workers)?.with_store(language_store(None));
                for language in catalog::LANGUAGES {
                    runtime.declare_language(language.id, language.aliases);
                }
                Ok(runtime)
            })
            .context("could not spawn the Lumis WASM runtime initializer")?
            .join()
            .map_err(|_| anyhow!("Lumis WASM runtime initialization panicked"))??;
        let runtime = Arc::new(runtime);
        let (sender, receiver) = mpsc::sync_channel::<WasmJob>(workers * 2);
        let receiver = Arc::new(Mutex::new(receiver));

        for index in 0..workers {
            let runtime = Arc::clone(&runtime);
            let receiver = Arc::clone(&receiver);
            thread::Builder::new()
                .name(format!("lumis-wasm-{index}"))
                .stack_size(8 * 1024 * 1024)
                .spawn(move || loop {
                    let Ok(job) = receiver.lock().expect("executor lock poisoned").recv() else {
                        return;
                    };
                    match job {
                        WasmJob::LoadNamed { name, reply } => {
                            let _ = reply.send(runtime.load_named_language(&name));
                        }
                        WasmJob::Highlight {
                            source,
                            language,
                            rainbow_brackets,
                            reply,
                        } => {
                            let _ =
                                reply.send(runtime.highlight(&source, &language, rainbow_brackets));
                        }
                    }
                })
                .with_context(|| format!("could not spawn Lumis WASM worker {index}"))?;
        }

        Ok(Self { runtime, sender })
    }

    /// Resolving a language does a TLS handshake, which needs far more stack
    /// than a BEAM dirty scheduler has; run it on the executor's own threads.
    fn load_named_language(&self, name: &str) -> Result<(), LoadFailure> {
        let (reply, result) = mpsc::sync_channel(1);
        self.sender
            .send(WasmJob::LoadNamed {
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

    fn highlight(
        &self,
        source: &str,
        language: &str,
        rainbow_brackets: bool,
    ) -> Result<Vec<HighlightEvent<'static>>, RuntimeError> {
        let (reply, result) = mpsc::sync_channel(1);
        self.sender
            .send(WasmJob::Highlight {
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

rustler::atoms! {
    ok,
    error,
    event_start = "start",
    event_source = "source",
    event_end = "end",
    annotation_start,
    annotation_end,
    language_not_loaded,
    unknown_language,
    failed_to_load_parser,
}

rustler::init!("Elixir.Lumis.Native");

#[derive(Debug, NifMap)]
pub struct ExOptions<'a> {
    pub language: Option<&'a str>,
    pub formatter: ExFormatterOption,
    pub annotations: Vec<Term<'a>>,
    pub rainbow_brackets: bool,
}

#[derive(Clone, Debug, NifStruct)]
#[module = "Lumis.Annotation"]
pub struct ExResolvedAnnotation<'a> {
    pub range: (usize, usize),
    pub data: Term<'a>,
}

#[derive(Debug, NifMap)]
pub struct ExEventOptions<'a> {
    pub language: Option<&'a str>,
    pub annotations: Vec<Term<'a>>,
    pub rainbow_brackets: bool,
}

#[derive(Debug, NifMap)]
struct ExStartEvent {
    scope: String,
    language: String,
}

#[derive(Debug, NifMap)]
struct ExSourceEvent {
    start: usize,
    end: usize,
}

enum CollectedEvent<'a> {
    Start { scope: String, language: String },
    Source { start: usize, end: usize },
    End,
    AnnotationStart(ExResolvedAnnotation<'a>),
    AnnotationEnd,
}

impl<'a> CollectedEvent<'a> {
    fn encode(self, env: Env<'a>) -> Term<'a> {
        match self {
            Self::Start { scope, language } => {
                (event_start(), ExStartEvent { scope, language }).encode(env)
            }
            Self::Source { start, end } => {
                (event_source(), ExSourceEvent { start, end }).encode(env)
            }
            Self::End => event_end().encode(env),
            Self::AnnotationStart(annotation) => (annotation_start(), annotation).encode(env),
            Self::AnnotationEnd => annotation_end().encode(env),
        }
    }
}

struct EventFormatter<'a> {
    language: Language,
    events: Mutex<Vec<CollectedEvent<'a>>>,
}

impl<'a> EventFormatter<'a> {
    fn new(language: Language) -> Self {
        Self {
            language,
            events: Mutex::new(Vec::new()),
        }
    }

    fn into_events(self) -> Vec<CollectedEvent<'a>> {
        self.events.into_inner().expect("event lock poisoned")
    }
}

impl<'a> Formatter<Term<'a>> for EventFormatter<'a> {
    fn language(&self) -> Language {
        self.language
    }

    fn render(
        &self,
        _source: &str,
        events: &[HighlightEvent<'_, Term<'a>>],
        _output: &mut dyn Write,
    ) -> std::io::Result<()> {
        let mut output = self.events.lock().expect("event lock poisoned");

        for event in events {
            let event = match event {
                HighlightEvent::Start {
                    scope_index,
                    language,
                } => CollectedEvent::Start {
                    scope: lumis_core::highlights::HIGHLIGHT_NAMES[*scope_index].to_owned(),
                    language: language.clone(),
                },
                HighlightEvent::Source { start, end } => CollectedEvent::Source {
                    start: *start,
                    end: *end,
                },
                HighlightEvent::End => CollectedEvent::End,
                HighlightEvent::AnnotationStart { annotation } => {
                    CollectedEvent::AnnotationStart(ExResolvedAnnotation {
                        range: (annotation.range().start, annotation.range().end),
                        data: *annotation.data(),
                    })
                }
                HighlightEvent::AnnotationEnd => CollectedEvent::AnnotationEnd,
                // A kind this build predates: drop it rather than crossing the
                // NIF boundary with a shape Elixir has no clause for.
                _ => continue,
            };
            output.push(event);
        }

        Ok(())
    }
}

#[derive(Clone, Debug, NifMap)]
pub struct ExLanguagePackageRef<'a> {
    pub id: &'a str,
    pub aliases: Vec<&'a str>,
    pub package_name: &'a str,
}

#[derive(Clone, Debug, NifMap)]
pub struct ExLanguageInfo<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub aliases: Vec<&'a str>,
    pub extensions: Vec<&'a str>,
    pub globs: Vec<&'a str>,
    pub emacs_modes: Vec<&'a str>,
    pub shebangs: Vec<&'a str>,
}

impl From<languages::LanguageInfo> for ExLanguageInfo<'static> {
    fn from(language: languages::LanguageInfo) -> Self {
        Self {
            id: language.id,
            name: language.name,
            aliases: language.aliases.to_vec(),
            extensions: language.extensions,
            globs: language.globs.to_vec(),
            emacs_modes: language.emacs_modes.to_vec(),
            shebangs: language.shebangs.to_vec(),
        }
    }
}

#[derive(Clone, Debug, NifMap)]
pub struct ExThemeInfo<'a> {
    pub name: &'a str,
    pub appearance: &'a str,
}

impl From<&'static themes::Theme> for ExThemeInfo<'static> {
    fn from(theme: &'static themes::Theme) -> Self {
        Self {
            name: theme.name.as_str(),
            appearance: match theme.appearance {
                themes::Appearance::Light => "light",
                themes::Appearance::Dark => "dark",
            },
        }
    }
}

impl From<&catalog::LanguagePackageRef> for ExLanguagePackageRef<'static> {
    fn from(language: &catalog::LanguagePackageRef) -> Self {
        Self {
            id: language.id,
            aliases: language.aliases.to_vec(),
            package_name: language.package_name,
        }
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
pub(crate) fn highlight<'a>(
    env: Env<'a>,
    source: &'a str,
    options: ExOptions<'a>,
) -> NifResult<Term<'a>> {
    let language = languages::Language::guess(options.language, source);
    let annotations = decode_annotations(options.annotations)?;
    let formatter = match options.formatter.into_formatter(language) {
        Ok(formatter) => formatter,
        Err(message) => return Ok((error(), message).encode(env)),
    };

    let events = match syntax_events(env, source, language, options.rainbow_brackets) {
        Ok(events) => events,
        Err(failure) => return Ok(failure),
    };
    let events = match compose_annotations(source, &events, &annotations) {
        Ok(events) => events,
        Err(annotation_error) => return Ok((error(), annotation_error.to_string()).encode(env)),
    };

    let mut output = Vec::new();
    if let Err(render_error) = formatter.render(source, &events, &mut output) {
        return Ok((error(), render_error.to_string()).encode(env));
    }
    let output = String::from_utf8(output)
        .map_err(|error| Error::Term(Box::new(format!("invalid formatter output: {error}"))))?;
    Ok((ok(), output).encode(env))
}

/// Syntax events for `source`, or the error term Elixir should receive.
fn syntax_events<'a>(
    env: Env<'a>,
    source: &str,
    language: Language,
    rainbow_brackets: bool,
) -> Result<Vec<HighlightEvent<'static>>, Term<'a>> {
    if language == languages::Language::PlainText {
        return Ok(vec![HighlightEvent::Source {
            start: 0,
            end: source.len(),
        }]);
    }

    let executor = executor().map_err(|reason| (error(), format!("{reason:#}")).encode(env))?;
    executor
        .highlight(source, language.id_name(), rainbow_brackets)
        .map_err(|runtime_error| match runtime_error {
            RuntimeError::LanguageNotLoaded(language) => {
                (error(), (language_not_loaded(), language)).encode(env)
            }
            runtime_error => (error(), runtime_error.to_string()).encode(env),
        })
}

#[rustler::nif(schedule = "DirtyCpu")]
pub(crate) fn highlight_events<'a>(
    env: Env<'a>,
    source: &'a str,
    options: ExEventOptions<'a>,
) -> NifResult<Term<'a>> {
    let language = Language::guess(options.language, source);
    let annotations = decode_annotations(options.annotations)?;
    let formatter = EventFormatter::new(language);

    let events = match syntax_events(env, source, language, options.rainbow_brackets) {
        Ok(events) => events,
        Err(failure) => return Ok(failure),
    };
    let events = match compose_annotations(source, &events, &annotations) {
        Ok(events) => events,
        Err(annotation_error) => return Ok((error(), annotation_error.to_string()).encode(env)),
    };

    formatter
        .render(source, &events, &mut std::io::sink())
        .map_err(|error| Error::Term(Box::new(error.to_string())))?;

    let events = formatter
        .into_events()
        .into_iter()
        .map(|event| event.encode(env))
        .collect::<Vec<_>>();

    // The resolved language rides along because detection happened here. A
    // formatter needs it to label its output, and asking Elixir to guess again
    // would run detection over the whole source a second time to reach an answer
    // this call already has.
    Ok((ok(), language.id_name(), events).encode(env))
}

/// Reads the tagged tuples `Lumis.annotations_type/1` normalizes to:
/// `{:offset, start, end, data}`, or `{:position, {line, column}, {line,
/// column}, data}`. Elixir has already validated the shape and the ordering.
fn decode_annotations(annotations: Vec<Term<'_>>) -> NifResult<Vec<Annotation<Term<'_>>>> {
    annotations
        .into_iter()
        .map(|term| {
            let parts = rustler::types::tuple::get_tuple(term)?;
            let [kind, start, end, data] = parts.as_slice() else {
                return Err(Error::BadArg);
            };

            let range = match kind.atom_to_string()?.as_str() {
                "offset" => {
                    AnnotationRange::Offset(start.decode::<usize>()?..end.decode::<usize>()?)
                }
                "position" => {
                    AnnotationRange::Position(decode_position(*start)?..decode_position(*end)?)
                }
                _ => return Err(Error::BadArg),
            };

            Annotation::new(range, *data).map_err(|error| Error::Term(Box::new(error.to_string())))
        })
        .collect()
}

fn decode_position(term: Term<'_>) -> NifResult<Position> {
    let (line, column) = term.decode::<(usize, usize)>()?;
    Ok(Position::new(line, column))
}

fn executor() -> Result<&'static WasmExecutor> {
    EXECUTOR.as_ref().map_err(|error| anyhow!("{error:#}"))
}

/// Point the store at `data_dir`, overriding `LUMIS_DATA_DIR`.
///
/// Returns false once the store exists, since the paths are read when it is
/// built. `Lumis.Application` calls this before anything can use it.
#[rustler::nif]
fn configure_store(data_dir: Option<String>) -> bool {
    if Lazy::get(&EXECUTOR).is_some() {
        return false;
    }
    let mut paths = STORE_PATHS.write();
    paths.data_dir = data_dir.map(std::path::PathBuf::from);

    let compile_cache = store::resolve_data_dir(paths.data_dir.clone());
    lumis_wasm_runtime::set_compile_cache_dir(compile_cache);
    true
}

/// Resolve, download, verify and load `name` through the shared store.
///
/// Elixir no longer fetches anything: this is the same path the CLI takes, so
/// both cache the same bytes in the same place under the same names.
#[rustler::nif(schedule = "DirtyCpu")]
fn load_language_by_name<'a>(env: Env<'a>, name: &str) -> Term<'a> {
    let result = executor()
        .map_err(|_| LoadFailure::Parser)
        .and_then(|runtime| runtime.load_named_language(name));
    match result {
        Ok(()) => ok().encode(env),
        Err(failure) => (error(), failure).encode(env),
    }
}

/// Stack for a thread that resolves TLS or runs Cranelift.
///
/// Both want far more than a BEAM dirty scheduler carries, and overrunning one
/// takes the whole emulator down rather than raising. The executor's workers are
/// sized the same way, for the same reason.
const DEEP_STACK: usize = 8 * 1024 * 1024;

/// Run `work` on a thread with a stack the emulator's own do not have.
///
/// The batch entry points below do their work on the calling thread when there
/// is only one item, and `parallel_map` drains from the caller too, so a dirty
/// scheduler must not be the thread that ends up running either.
fn on_deep_stack<Output: Send>(work: impl FnOnce() -> Output + Send) -> Result<Output> {
    std::thread::scope(|scope| {
        thread::Builder::new()
            .name("lumis-batch".into())
            .stack_size(DEEP_STACK)
            .spawn_scoped(scope, work)
            .context("could not spawn the Lumis batch thread")?
            .join()
            .map_err(|_| anyhow!("the Lumis batch thread panicked"))
    })
}

/// Download and cache `names` concurrently for `Lumis.Languages.cache/2`.
///
/// One result per name, in order, so the caller reports every language that
/// could not be obtained instead of stopping at the first. Caching a bundle one
/// language at a time is a hundred sequential round trips to the CDN.
///
/// Downloading needs no Wasmtime runtime, so this goes straight to a store
/// rather than waking the executor.
#[rustler::nif(schedule = "DirtyIo")]
fn cache_languages(env: Env<'_>, names: Vec<String>, force: bool) -> Term<'_> {
    let results = on_deep_stack(|| {
        let _batch = CACHE_BATCH.lock();
        language_store(None)
            .cache_languages(&names, force, lumis_wasm_runtime::DOWNLOAD_CONCURRENCY)
            .into_iter()
            .map(|result| result.map_err(|failure| failure.to_string()))
            .collect::<Vec<_>>()
    });

    match results {
        Ok(results) => results
            .into_iter()
            .map(|result| match result {
                Ok(path) => (ok(), path.display().to_string()).encode(env),
                Err(message) => (error(), message).encode(env),
            })
            .collect::<Vec<_>>()
            .encode(env),
        Err(error) => repeated_failure(env, &format!("{error:#}"), names.len()),
    }
}

/// Compile `names` into the on-disk Wasmtime cache without loading them.
///
/// Downloading is the smaller half of a cold parser; the Cranelift compile is
/// the larger, and this is what puts it in the image. One result per name, in
/// order.
#[rustler::nif(schedule = "DirtyCpu")]
fn precompile_languages(env: Env<'_>, names: Vec<String>) -> Term<'_> {
    let executor = match executor() {
        Ok(executor) => executor,
        Err(error) => return repeated_failure(env, &format!("{error:#}"), names.len()),
    };

    let results = on_deep_stack(|| {
        let _batch = PRECOMPILE_BATCH.lock();
        executor
            .runtime
            .precompile_languages(&names, lumis_wasm_runtime::compile_concurrency())
            .into_iter()
            .map(|result| result.map_err(|failure| failure.to_string()))
            .collect::<Vec<_>>()
    });

    match results {
        Ok(results) => results
            .into_iter()
            .map(|result| match result {
                Ok(()) => ok().encode(env),
                Err(message) => (error(), message).encode(env),
            })
            .collect::<Vec<_>>()
            .encode(env),
        Err(error) => repeated_failure(env, &format!("{error:#}"), names.len()),
    }
}

/// One failure per name, for the cases that fail before any name is attempted.
/// The batch NIFs answer positionally, so the list still has to line up.
fn repeated_failure<'a>(env: Env<'a>, message: &str, count: usize) -> Term<'a> {
    let failure = (error(), message).encode(env);
    vec![failure; count].encode(env)
}

/// Dirty because the source is caller-supplied and unbounded: detection runs
/// regexes over it, so a large document would hold a normal scheduler past the
/// 1 ms budget.
#[rustler::nif(schedule = "DirtyCpu")]
fn guess_language(name: Option<&str>, source: &str) -> &'static str {
    languages::Language::guess(name, source).id_name()
}

#[rustler::nif]
fn has_language(name: &str) -> bool {
    executor().is_ok_and(|executor| executor.runtime.has_language(name))
}

#[rustler::nif]
fn loaded_languages() -> Vec<String> {
    executor()
        .map(|executor| executor.runtime.loaded_languages())
        .unwrap_or_default()
}

#[rustler::nif]
fn language_package_refs() -> Vec<ExLanguagePackageRef<'static>> {
    catalog::LANGUAGES
        .iter()
        .map(ExLanguagePackageRef::from)
        .collect()
}

#[rustler::nif]
fn language_bundles() -> HashMap<&'static str, Vec<&'static str>> {
    catalog::BUNDLES
        .iter()
        .map(|(name, members)| (*name, members.to_vec()))
        .collect()
}

#[rustler::nif]
fn available_languages() -> Vec<ExLanguageInfo<'static>> {
    languages::available_languages()
        .into_iter()
        .map(ExLanguageInfo::from)
        .collect()
}

#[rustler::nif]
fn language_info(name: &str) -> Option<ExLanguageInfo<'static>> {
    name.parse::<languages::Language>()
        .ok()
        .map(|language| ExLanguageInfo::from(language.info()))
}

#[rustler::nif]
fn available_themes() -> Vec<ExThemeInfo<'static>> {
    // Rust's `available_themes` yields whole themes, which carry more than the
    // wire needs; the summary is built here rather than shipping 246 of them.
    let mut summaries: Vec<ExThemeInfo<'static>> =
        themes::available_themes().map(ExThemeInfo::from).collect();
    summaries.sort_unstable_by_key(|theme| theme.name);
    summaries
}

#[rustler::nif]
fn get_theme(name: &str) -> NifResult<ExTheme> {
    // Fast path: check if theme is already cached (read lock)
    {
        let cache = THEME_CACHE.read();
        if let Some(cached_theme) = cache.get(name) {
            return Ok(cached_theme.clone());
        }
    }

    // Slow path: load theme, convert, and cache it (write lock)
    let theme = themes::get(name).map_err(|_e| Error::Atom("error"))?;
    let ex_theme = ExTheme::from(&theme);

    // Cache the converted theme for future calls
    {
        let mut cache = THEME_CACHE.write();
        cache.insert(name.to_string(), ex_theme.clone());
    }

    Ok(ex_theme)
}

#[rustler::nif]
fn build_theme_from_file(path: &str) -> NifResult<ExTheme> {
    themes::from_file(path)
        .map(|theme| ExTheme::from(&theme))
        .map_err(|_e| Error::Atom("error"))
}

#[rustler::nif]
fn build_theme_from_json_string(json_string: &str) -> NifResult<ExTheme> {
    themes::from_json(json_string)
        .map(|theme| ExTheme::from(&theme))
        .map_err(|_e| Error::Atom("error"))
}

#[rustler::nif]
fn theme_css_from_name(name: &str, options: ExCssOptions) -> NifResult<String> {
    let theme = themes::get(name).map_err(|_e| Error::Atom("error"))?;
    Ok(build_theme_css(&theme, options))
}

#[rustler::nif]
fn theme_css_from_theme(theme: ExTheme, options: ExCssOptions) -> String {
    build_theme_css(&theme.into(), options)
}

fn build_theme_css(theme: &themes::Theme, options: ExCssOptions) -> String {
    let mut builder = themes::CssBuilder::new(theme);

    builder
        .enable_italic(options.enable_italic)
        .scope(options.scope)
        .container_selector(options.container_selector);

    builder.container_style(options.container_style);

    builder.build()
}

#[rustler::nif]
fn ansi_hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    lumis_core::formatter::ansi::hex_to_rgb(hex)
}

#[rustler::nif]
fn ansi_rgb_to_ansi(r: u8, g: u8, b: u8, is_background: bool) -> String {
    lumis_core::formatter::ansi::rgb_to_ansi(r, g, b, is_background)
}

#[rustler::nif]
fn ansi_style_to_ansi(style: ExStyle) -> String {
    lumis_core::formatter::ansi::style_to_ansi(&style.into())
}

#[rustler::nif]
fn ansi_paint(text: &str, style: ExStyle) -> String {
    lumis_core::formatter::ansi::paint(text, &style.into())
}

#[rustler::nif]
fn ansi_reset() -> &'static str {
    lumis_core::formatter::ansi::ANSI_RESET
}

/// Every scope's style for one theme and language, resolved the way the
/// built-in terminal formatter resolves it.
///
/// The per-token counterpart of the rest of these, split for the same reason as
/// [`html_span_attrs`]. `Theme::get_style` walks a scope up to its parent and
/// consults the rainbow-bracket fallbacks, so an Elixir formatter that looks a
/// scope up in `theme.highlights` itself gets a different answer than
/// `:terminal` does. There are only ever 293 answers, so all of them come back
/// at once and Elixir never resolves anything.
#[rustler::nif]
fn ansi_styles(theme: Option<ExTheme>, language: &str) -> HashMap<&'static str, ExStyle> {
    let Some(theme) = theme.map(themes::Theme::from) else {
        return HashMap::new();
    };
    let language = Language::guess(Some(language), "");

    lumis_core::highlights::HIGHLIGHT_NAMES
        .iter()
        .filter_map(|scope| {
            let specialized = format!("{scope}.{}", language.id_name());
            let style = theme
                .get_style(&specialized)
                .or_else(|| theme.get_style(scope))?;
            Some((*scope, ExStyle::from(style)))
        })
        .collect()
}

/// `lumis_core::formatter::html`, reachable from Elixir.
///
/// A formatter written in Elixir needs the same pieces the built-in HTML
/// formatters are assembled from. These hand them over rather than let every
/// formatter grow its own copy, which is how `examples/annotations.exs` shipped
/// an `escape` missing `'` and a `scope_to_class` that could not express the
/// `l-text` fallback at all.
///
/// The split is by call frequency, not by taste. What a formatter calls once per
/// document crosses the boundary as a call. The two things it calls once per
/// token cross once per document as a table instead, because 5000 NIF calls to
/// look up a constant is not what reusing the Rust core should cost.
#[rustler::nif]
fn html_escape(text: &str) -> String {
    lumis_core::formatter::html::escape(text)
}

#[rustler::nif]
fn html_escape_braces(text: &str) -> String {
    lumis_core::formatter::html::escape_braces(text)
}

/// Every highlight scope and the class `:html_linked` gives it.
///
/// `scope_to_class` is a lookup into two generated 293-row tables, so Elixir
/// cannot spell it without copying them. The whole table crosses once instead.
#[rustler::nif]
fn html_classes() -> HashMap<&'static str, String> {
    lumis_core::highlights::HIGHLIGHT_NAMES
        .iter()
        .map(|scope| (*scope, lumis_core::formatter::html::scope_to_class(scope)))
        .collect()
}

/// Every scope's `<span>` attributes for one theme, language and option set.
///
/// The per-token counterpart of [`html_classes`]. `span_inline_attrs` resolves a
/// scope against a theme, and sending the theme across the boundary once per
/// token to do that would cost more than the highlighting did. There are only
/// ever 293 answers, so all of them come back at once.
#[rustler::nif]
fn html_span_attrs(
    theme: Option<ExTheme>,
    language: &str,
    italic: bool,
    include_highlights: bool,
) -> HashMap<&'static str, String> {
    let theme = theme.map(themes::Theme::from);
    let language = Language::guess(Some(language), "");

    lumis_core::highlights::HIGHLIGHT_NAMES
        .iter()
        .map(|scope| {
            let attrs = lumis_core::formatter::html::span_inline_attrs(
                Some(language),
                scope,
                theme.as_ref(),
                italic,
                include_highlights,
            );
            (*scope, attrs)
        })
        .collect()
}

#[rustler::nif]
fn html_open_pre_tag(pre_class: Option<String>, theme: Option<ExTheme>) -> NifResult<String> {
    let theme = theme.map(themes::Theme::from);
    let mut output = Vec::new();
    lumis_core::formatter::html::open_pre_tag(&mut output, pre_class.as_deref(), theme.as_ref())
        .map_err(|error| Error::Term(Box::new(error.to_string())))?;
    html_utf8(output)
}

#[rustler::nif]
fn html_open_code_tag(language: &str) -> NifResult<String> {
    let mut output = Vec::new();
    lumis_core::formatter::html::open_code_tag(&mut output, &Language::guess(Some(language), ""))
        .map_err(|error| Error::Term(Box::new(error.to_string())))?;
    html_utf8(output)
}

#[rustler::nif]
fn html_closing_tags() -> NifResult<String> {
    let mut output = Vec::new();
    lumis_core::formatter::html::closing_tags(&mut output)
        .map_err(|error| Error::Term(Box::new(error.to_string())))?;
    html_utf8(output)
}

#[rustler::nif]
fn html_wrap_line(
    line_number: usize,
    content: &str,
    class_suffix: Option<String>,
    style: Option<String>,
) -> String {
    lumis_core::formatter::html::wrap_line(
        line_number,
        content,
        class_suffix.as_deref(),
        style.as_deref(),
    )
}

fn html_utf8(output: Vec<u8>) -> NifResult<String> {
    String::from_utf8(output)
        .map_err(|error| Error::Term(Box::new(format!("invalid HTML helper output: {error}"))))
}

#[cfg(test)]
mod tests {
    use super::HighlightEvent;
    use lumis_core::formatter::{Formatter, HtmlInlineBuilder};
    use lumis_core::languages::Language;
    use lumis_wasm_runtime::{
        sha256_hex, LanguagePackage, PackagedLanguage, ParserMetadata, Runtime,
    };
    use std::collections::BTreeMap;

    #[test]
    fn test_formatter_works_with_precomputed_events() {
        let source = "@test :test";
        let lang = Language::guess(Some("elixir"), source);
        let formatter = HtmlInlineBuilder::new().language(lang).build().unwrap();
        let events: [HighlightEvent<'_>; 1] = [HighlightEvent::Source {
            start: 0,
            end: source.len(),
        }];
        let mut output = Vec::new();
        formatter.render(source, &events, &mut output).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(!result.is_empty(), "Output should not be empty");

        assert!(
            result.contains("<pre"),
            "Output should contain opening <pre> tag"
        );

        assert!(result.contains("<code"), "Output should contain <code> tag");

        assert!(
            result.contains("test"),
            "Output should contain 'test' keyword"
        );
    }

    #[test]
    fn test_elixir_wasm_with_generated_queries() {
        let wasm =
            include_bytes!("../../../../../../fixtures/test-parsers/tree-sitter-elixir.wasm")
                .to_vec();
        let package = LanguagePackage {
            package_name: "@lumis-sh/wasm-elixir".into(),
            version: "test".into(),
            definition_hash: "test".into(),
            parser: ParserMetadata {
                name: "tree-sitter-elixir".into(),
                grammar_name: "elixir".into(),
                upstream_version: None,
                revision: None,
                sha256: sha256_hex(&wasm),
                size: u64::try_from(wasm.len()).expect("parser size fits in u64"),
            },
            languages: BTreeMap::from([(
                "elixir".into(),
                PackagedLanguage {
                    aliases: Vec::new(),
                    highlights: include_str!(
                        "../../../../../../queries/processed/elixir/highlights.scm"
                    )
                    .into(),
                    injections: include_str!(
                        "../../../../../../queries/processed/elixir/injections.scm"
                    )
                    .into(),
                    locals: String::new(),
                    brackets: include_str!(
                        "../../../../../../queries/processed/default/brackets.scm"
                    )
                    .into(),
                },
            )]),
        };
        let runtime = Runtime::with_worker_limit(1).unwrap();
        runtime
            .load_language(package.language_spec("elixir", wasm).unwrap())
            .unwrap();

        let events = runtime
            .highlight("defmodule Test do\nend", "elixir", false)
            .unwrap();
        assert!(
            !events.is_empty(),
            "highlighting an Elixir module produced no events"
        );
    }
}

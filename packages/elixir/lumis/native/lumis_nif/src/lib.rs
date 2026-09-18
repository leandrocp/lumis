use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::io::Write;
use std::sync::Mutex;
use std::thread;

use anyhow::{anyhow, Context, Result};
use lumis_core::annotations::{compose_annotations, Annotation, AnnotationRange, Position};
use lumis_core::elixir::{
    line_specs_contain, ExCssOptions, ExFormatterOption, ExLineSpec, ExStyle, ExTextDecoration,
    ExTheme,
};
use lumis_core::events::HighlightEvent;
use lumis_core::formatter::Formatter;
use lumis_core::languages::Language;
use lumis_core::{languages, themes};
use lumis_wasm_runtime::{catalog, store, Executor, LoadFailure, RuntimeError};
use parking_lot::RwLock;
use rustler::{Encoder, Env, Error, NifMap, NifResult, NifStruct, Resource, ResourceArc, Term};

/// Lazy per-theme cache to eliminate repeated allocations.
/// Themes are converted and cached on first access, amortizing the cost.
static THEME_CACHE: std::sync::LazyLock<RwLock<HashMap<String, ExTheme>>> =
    std::sync::LazyLock::new(|| RwLock::new(HashMap::new()));

// `LazyLock::get`, which `configure_store` needs, is newer than the MSRV.
#[allow(clippy::non_std_lazy_statics)]
static EXECUTOR: Lazy<Result<Executor>> = Lazy::new(|| {
    Executor::new(language_store(None)).context("could not start the Lumis WASM executor")
});
static CACHE_BATCH: std::sync::LazyLock<parking_lot::Mutex<()>> =
    std::sync::LazyLock::new(|| parking_lot::Mutex::new(()));
static PRECOMPILE_BATCH: std::sync::LazyLock<parking_lot::Mutex<()>> =
    std::sync::LazyLock::new(|| parking_lot::Mutex::new(()));

/// `LoadFailure` belongs to the runtime crate, so the atom mapping lives at the
/// boundary rather than as an orphan `Encoder`.
fn load_failure_atom(failure: LoadFailure) -> rustler::Atom {
    match failure {
        LoadFailure::UnknownLanguage => unknown_language(),
        LoadFailure::Parser => failed_to_load_parser(),
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

fn executor() -> Result<&'static Executor> {
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

/// The directory the store actually resolved to.
///
/// `Lumis.Application` decides this, and a second library embedding Lumis has
/// to reach the same store or the same parser is downloaded and compiled twice.
/// One resolve, readable by anyone who needs it.
#[rustler::nif]
fn data_dir() -> String {
    store::resolve_data_dir(STORE_PATHS.read().data_dir.clone())
        .to_string_lossy()
        .into_owned()
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
        Err(failure) => (error(), load_failure_atom(failure)).encode(env),
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
    let count = names.len();
    let executor = match executor() {
        Ok(executor) => executor,
        Err(error) => return repeated_failure(env, &format!("{error:#}"), count),
    };

    let results = on_deep_stack(|| {
        let _batch = PRECOMPILE_BATCH.lock();
        executor
            .precompile_languages(names, lumis_wasm_runtime::compile_concurrency())
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
        Err(error) => repeated_failure(env, &format!("{error:#}"), count),
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
    executor().is_ok_and(|executor| executor.has_language(name))
}

#[rustler::nif]
fn loaded_languages() -> Vec<String> {
    executor()
        .map(Executor::loaded_languages)
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

#[rustler::nif]
fn html_escape_attr(value: &str) -> String {
    lumis_core::formatter::html::escape_attr(value)
}

#[rustler::nif]
fn html_sanitize_theme_name(name: &str) -> String {
    lumis_core::formatter::html::sanitize_theme_name(name)
}

#[rustler::nif]
fn html_text_decoration(text_decoration: ExTextDecoration) -> &'static str {
    lumis_core::formatter::html::text_decoration(&text_decoration.into())
}

#[rustler::nif]
fn html_style_to_css(style: ExStyle, italic: bool, separator: &str) -> String {
    themes::Style::from(style).css(italic, separator)
}

#[rustler::nif]
fn html_close_pre_tag() -> NifResult<String> {
    let mut output = Vec::new();
    lumis_core::formatter::html::close_pre_tag(&mut output)
        .map_err(|error| Error::Term(Box::new(error.to_string())))?;
    html_utf8(output)
}

#[rustler::nif]
fn html_close_code_tag() -> NifResult<String> {
    let mut output = Vec::new();
    lumis_core::formatter::html::close_code_tag(&mut output)
        .map_err(|error| Error::Term(Box::new(error.to_string())))?;
    html_utf8(output)
}

/// Every scope's multi-theme `<span>` attributes, for one theme set and language.
///
/// The multi-theme counterpart of [`html_span_attrs`], split for the same
/// reason: sending the whole theme set across the boundary once per token to
/// resolve one scope would cost more than the highlighting did.
#[rustler::nif]
fn html_multi_themes_span_attrs(
    themes_map: HashMap<String, ExTheme>,
    default_theme: Option<String>,
    css_variable_prefix: &str,
    language: &str,
    italic: bool,
    include_highlights: bool,
) -> HashMap<&'static str, String> {
    let themes_map: HashMap<String, themes::Theme> = themes_map
        .into_iter()
        .map(|(name, theme)| (name, themes::Theme::from(theme)))
        .collect();
    let language = Language::guess(Some(language), "");

    lumis_core::highlights::HIGHLIGHT_NAMES
        .iter()
        .map(|scope| {
            let attrs = lumis_core::formatter::html::span_multi_themes_attrs(
                scope,
                Some(language),
                &themes_map,
                default_theme.as_deref(),
                css_variable_prefix,
                italic,
                include_highlights,
            );
            (*scope, attrs)
        })
        .collect()
}

#[rustler::nif]
fn html_open_multi_themes_pre_tag(
    pre_class: Option<String>,
    themes_map: HashMap<String, ExTheme>,
    default_theme: Option<String>,
    css_variable_prefix: &str,
) -> NifResult<String> {
    let themes_map: HashMap<String, themes::Theme> = themes_map
        .into_iter()
        .map(|(name, theme)| (name, themes::Theme::from(theme)))
        .collect();

    let mut output = Vec::new();
    lumis_core::formatter::html::open_multi_themes_pre_tag(
        &mut output,
        pre_class.as_deref(),
        &themes_map,
        default_theme.as_deref(),
        css_variable_prefix,
    )
    .map_err(|error| Error::Term(Box::new(error.to_string())))?;
    html_utf8(output)
}

#[rustler::nif]
fn html_line_is_highlighted(lines: Vec<ExLineSpec>, line_number: usize) -> bool {
    line_specs_contain(&lines, line_number)
}

#[rustler::nif]
fn html_highlight_line_class(
    lines: Vec<ExLineSpec>,
    line_number: usize,
    class: Option<String>,
    default_class: Option<String>,
) -> Option<String> {
    if line_specs_contain(&lines, line_number) {
        class.or(default_class)
    } else {
        None
    }
}

/// The event stream rendered into HTML lines, with spans reopened across newlines.
///
/// The one helper an Elixir formatter cannot assemble from the others, because
/// closing and reopening the open spans at every newline is the part that is
/// easy to get wrong. `attrs` is a table from [`html_span_attrs`] or
/// [`html_multi_themes_span_attrs`], so the whole render costs one call rather
/// than one per token.
#[rustler::nif(schedule = "DirtyCpu")]
fn html_render_lines_from_events(
    source: &str,
    events: Vec<Term<'_>>,
    attrs: HashMap<String, String>,
) -> Vec<String> {
    let mut scopes: Vec<String> = Vec::new();
    let mut decoded: Vec<HighlightEvent<'static>> = Vec::with_capacity(events.len());

    for event in events {
        // Decoded by hand rather than through a tagged enum so that an event
        // kind this build predates, or one carrying caller data, is skipped
        // instead of failing the whole render.
        if let Ok(atom) = event.decode::<rustler::Atom>() {
            if atom == event_end() {
                decoded.push(HighlightEvent::End);
            }
            continue;
        }

        let Ok((tag, payload)) = event.decode::<(rustler::Atom, Term<'_>)>() else {
            continue;
        };

        if tag == event_start() {
            let Ok(start) = payload.decode::<ExStartEvent>() else {
                continue;
            };
            let scope_index = scopes
                .iter()
                .position(|scope| *scope == start.scope)
                .unwrap_or_else(|| {
                    scopes.push(start.scope);
                    scopes.len() - 1
                });
            decoded.push(HighlightEvent::Start {
                scope_index,
                language: start.language,
            });
        } else if tag == event_source() {
            if let Ok(source_event) = payload.decode::<ExSourceEvent>() {
                decoded.push(HighlightEvent::Source {
                    start: source_event.start,
                    end: source_event.end,
                });
            }
        }
    }

    lumis_core::formatter::html::render_lines_from_events(source, &decoded, |scope_index, _| {
        scopes
            .get(scope_index)
            .and_then(|scope| attrs.get(scope))
            .cloned()
            .unwrap_or_default()
    })
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

// ---------------------------------------------------------------------------
// MDEx bridge
//
// `mdex_native` highlights code fences with Lumis but must not link a second
// copy of the engine. It reaches this one through `enif_dynamic_resource_call`,
// which carries only plain C types: neither NIF can see the other's Rust.
//
// Events are pushed to a sink the caller owns, so nothing allocated here is
// freed there. `HighlightCall` and `EventC` are spelled identically on both
// sides. Their V1 layouts are frozen from the first Lumis release that ships
// this resource: after that, a different layout gets a different resource and
// function name rather than reusing `MDExBridgeV1`. Until then, any change to
// either struct also bumps `BRIDGE_ABI`, so a stale local build fails loudly
// instead of misreading events.
// ---------------------------------------------------------------------------

/// Identifies a valid V1 call; it does not negotiate a different struct layout.
const BRIDGE_ABI: u32 = 1;

const EVENT_START: u8 = 0;
const EVENT_SOURCE: u8 = 1;
const EVENT_END: u8 = 2;

const STATUS_OK: i32 = 0;
const STATUS_LANGUAGE_NOT_LOADED: i32 = 1;
const STATUS_ERROR: i32 = 2;
const STATUS_ABI_MISMATCH: i32 = 3;
const STATUS_PANIC: i32 = 4;

#[repr(C)]
pub struct EventC {
    pub kind: u8,
    /// Borrowed for the duration of the sink call.
    pub scope: *const u8,
    pub scope_len: usize,
    pub start: usize,
    pub end: usize,
    /// Borrowed for the duration of the sink call.
    pub language: *const u8,
    pub language_len: usize,
}

#[repr(C)]
pub struct HighlightCall {
    pub abi: u32,
    pub source: *const u8,
    pub source_len: usize,
    pub language: *const u8,
    pub language_len: usize,
    /// `0` or `1`. A `u8` rather than `bool` so that no byte the caller could
    /// write is an invalid value on this side of the boundary.
    pub rainbow_brackets: u8,
    pub sink: Option<unsafe extern "C" fn(*mut std::ffi::c_void, EventC)>,
    pub sink_ctx: *mut std::ffi::c_void,
    pub status: i32,
    /// Borrowed until this thread's next bridge call. Copy it before then.
    pub error: *const u8,
    pub error_len: usize,
}

thread_local! {
    /// Keeps the reason alive after `dyncall` returns, since the caller reads
    /// it once `enif_dynamic_resource_call` has handed control back.
    static LAST_ERROR: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) };
}

pub struct HighlightBridge;

#[rustler::resource_impl(name = "MDExBridgeV1")]
impl Resource for HighlightBridge {
    const IMPLEMENTS_DYNCALL: bool = true;

    /// # Safety
    /// `call_data` must point at the V1 `HighlightCall` layout this build was
    /// compiled against. Other layouts must use a differently named dynamic
    /// resource.
    unsafe fn dyncall<'a>(&'a self, _env: Env<'a>, call_data: *mut std::ffi::c_void) {
        // Unwinding across `extern "C"` is undefined behaviour, and rustler's
        // dyncall shim does not catch for us.
        let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            bridge_highlight(call_data.cast::<HighlightCall>());
        }));

        if caught.is_err() {
            let call = &mut *call_data.cast::<HighlightCall>();
            call.status = STATUS_PANIC;
            set_error(call, "Lumis panicked while highlighting");
        }
    }
}

unsafe fn bridge_highlight(call: *mut HighlightCall) {
    let call = &mut *call;

    if call.abi != BRIDGE_ABI {
        call.status = STATUS_ABI_MISMATCH;
        set_error(call, "mdex bridge ABI mismatch");
        return;
    }

    let Some(sink) = call.sink else {
        call.status = STATUS_ERROR;
        set_error(call, "mdex bridge called without a sink");
        return;
    };

    let Some(source) = borrowed_str(call.source, call.source_len) else {
        call.status = STATUS_ERROR;
        set_error(call, "mdex bridge source is not UTF-8");
        return;
    };
    let Some(language) = borrowed_str(call.language, call.language_len) else {
        call.status = STATUS_ERROR;
        set_error(call, "mdex bridge language is not UTF-8");
        return;
    };
    let executor = match executor() {
        Ok(executor) => executor,
        Err(reason) => {
            call.status = STATUS_ERROR;
            set_error(call, &format!("{reason:#}"));
            return;
        }
    };

    match executor.highlight(source, language, call.rainbow_brackets != 0) {
        Ok(events) => {
            // Variants a code fence has no use for are skipped, not sent as a
            // close: an unmatched `End` would unbalance the consumer's scopes.
            for event in events.iter().filter_map(event_to_c) {
                sink(call.sink_ctx, event);
            }
            call.status = STATUS_OK;
            call.error = std::ptr::null();
            call.error_len = 0;
        }
        Err(RuntimeError::LanguageNotLoaded(language)) => {
            call.status = STATUS_LANGUAGE_NOT_LOADED;
            set_error(call, &language);
        }
        Err(runtime_error) => {
            call.status = STATUS_ERROR;
            set_error(call, &runtime_error.to_string());
        }
    }
}

/// `None` for the variants a raw highlight never produces (annotations and
/// decorations belong to formatters); the caller skips those.
fn event_to_c(event: &HighlightEvent<'_>) -> Option<EventC> {
    let mut out = EventC {
        kind: EVENT_END,
        scope: std::ptr::null(),
        scope_len: 0,
        start: 0,
        end: 0,
        language: std::ptr::null(),
        language_len: 0,
    };

    match event {
        HighlightEvent::Start {
            scope_index,
            language,
        } => {
            // An index this table does not have is sent as a null name, which
            // the consumer treats as unknown. It is not sent as `""`, so the
            // two cases stay distinguishable on the other side.
            if let Some(scope) = lumis_core::highlights::HIGHLIGHT_NAMES.get(*scope_index) {
                out.scope = scope.as_ptr();
                out.scope_len = scope.len();
            }
            out.kind = EVENT_START;
            out.language = language.as_ptr();
            out.language_len = language.len();
        }
        HighlightEvent::Source { start, end } => {
            out.kind = EVENT_SOURCE;
            out.start = *start;
            out.end = *end;
        }
        HighlightEvent::End => {}
        _ => return None,
    }

    Some(out)
}

unsafe fn borrowed_str<'a>(ptr: *const u8, len: usize) -> Option<&'a str> {
    if ptr.is_null() {
        return None;
    }
    std::str::from_utf8(std::slice::from_raw_parts(ptr, len)).ok()
}

/// Runs on the panic path too, so it must not be able to panic itself: a
/// borrow that is somehow already held leaves the reason unset rather than
/// unwinding out of `dyncall`.
fn set_error(call: &mut HighlightCall, reason: &str) {
    call.error = std::ptr::null();
    call.error_len = 0;

    LAST_ERROR.with(|last| {
        if let Ok(mut last) = last.try_borrow_mut() {
            last.clear();
            last.push_str(reason);
            call.error = last.as_ptr();
            call.error_len = last.len();
        }
    });
}

/// Hands `mdex_native` the resource it calls back through.
#[rustler::nif]
fn mdex_bridge_v1() -> ResourceArc<HighlightBridge> {
    ResourceArc::new(HighlightBridge)
}

#[cfg(test)]
mod bridge_tests {
    use super::{borrowed_str, event_to_c, EventC, HighlightCall, HighlightEvent, BRIDGE_ABI};
    use std::mem::{offset_of, size_of};

    #[test]
    fn start_events_cross_the_bridge_by_scope_name() {
        let scope_index = lumis_core::highlights::HIGHLIGHT_NAMES
            .binary_search(&"function")
            .unwrap();
        let event = HighlightEvent::Start {
            scope_index,
            language: "elixir".to_string(),
        };

        let event = event_to_c(&event).expect("a Start event crosses");
        let scope = unsafe { borrowed_str(event.scope, event.scope_len) };
        let language = unsafe { borrowed_str(event.language, event.language_len) };

        assert_eq!(scope, Some("function"));
        assert_eq!(language, Some("elixir"));
    }

    #[test]
    fn an_index_outside_this_table_crosses_as_a_null_name() {
        let event = HighlightEvent::Start {
            scope_index: usize::MAX,
            language: "elixir".to_string(),
        };

        let event = event_to_c(&event).expect("a Start event crosses");

        assert!(event.scope.is_null());
        assert_eq!(event.scope_len, 0);
    }

    #[test]
    fn formatter_side_variants_do_not_cross() {
        assert!(event_to_c(&HighlightEvent::AnnotationEnd).is_none());
    }

    #[test]
    fn v1_layout_is_frozen() {
        assert_eq!(BRIDGE_ABI, 1);

        let event_layout = [
            size_of::<EventC>(),
            offset_of!(EventC, kind),
            offset_of!(EventC, scope),
            offset_of!(EventC, scope_len),
            offset_of!(EventC, start),
            offset_of!(EventC, end),
            offset_of!(EventC, language),
            offset_of!(EventC, language_len),
        ];
        let call_layout = [
            size_of::<HighlightCall>(),
            offset_of!(HighlightCall, abi),
            offset_of!(HighlightCall, source),
            offset_of!(HighlightCall, source_len),
            offset_of!(HighlightCall, language),
            offset_of!(HighlightCall, language_len),
            offset_of!(HighlightCall, rainbow_brackets),
            offset_of!(HighlightCall, sink),
            offset_of!(HighlightCall, sink_ctx),
            offset_of!(HighlightCall, status),
            offset_of!(HighlightCall, error),
            offset_of!(HighlightCall, error_len),
        ];

        #[cfg(target_pointer_width = "64")]
        {
            assert_eq!(event_layout, [56, 0, 8, 16, 24, 32, 40, 48]);
            assert_eq!(call_layout, [88, 0, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80]);
        }

        #[cfg(target_pointer_width = "32")]
        {
            assert_eq!(event_layout, [28, 0, 4, 8, 12, 16, 20, 24]);
            assert_eq!(call_layout, [44, 0, 4, 8, 12, 16, 20, 24, 28, 32, 36, 40]);
        }
    }
}

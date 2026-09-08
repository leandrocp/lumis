use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::thread;

use anyhow::{anyhow, Context, Result};
use lumis_core::elixir::{ExCssOptions, ExFormatterOption, ExTheme};
use lumis_core::events::HighlightEvent;
use lumis_core::{languages, themes};
use lumis_wasm_runtime::{catalog, store, Executor, LoadFailure, RuntimeError};
use parking_lot::RwLock;
use rustler::{Encoder, Env, Error, NifMap, NifResult, Term};

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
    language_not_loaded,
    unknown_language,
    failed_to_load_parser,
}

rustler::init!("Elixir.Lumis.Native");

#[derive(Debug, NifMap)]
pub struct ExOptions<'a> {
    pub language: Option<&'a str>,
    pub formatter: ExFormatterOption,
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
    options: ExOptions<'_>,
) -> NifResult<Term<'a>> {
    let language = languages::Language::guess(options.language, source);
    let (formatter, rainbow_brackets) = match options.formatter.into_formatter(language) {
        Ok(formatter) => formatter,
        Err(message) => return Ok((error(), message).encode(env)),
    };

    let events = if language == languages::Language::PlainText {
        vec![HighlightEvent::Source {
            start: 0,
            end: source.len(),
        }]
    } else {
        let executor = match executor() {
            Ok(executor) => executor,
            Err(reason) => return Ok((error(), format!("{reason:#}")).encode(env)),
        };
        match executor.highlight(source, language.id_name(), rainbow_brackets) {
            Ok(events) => events,
            Err(RuntimeError::LanguageNotLoaded(language)) => {
                return Ok((error(), (language_not_loaded(), language)).encode(env));
            }
            Err(runtime_error) => {
                return Ok((error(), runtime_error.to_string()).encode(env));
            }
        }
    };

    let mut output = Vec::new();
    if let Err(render_error) = formatter.render(source, &events, &mut output) {
        return Ok((error(), render_error.to_string()).encode(env));
    }
    let output = String::from_utf8(output)
        .map_err(|error| Error::Term(Box::new(format!("invalid formatter output: {error}"))))?;
    Ok((ok(), output).encode(env))
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
        let events = [HighlightEvent::Source {
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

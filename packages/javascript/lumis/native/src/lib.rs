use base64::Engine as _;
use lumis_core::events::HighlightEvent;
use lumis_core::formatter::bbcode::BBCodeScoped;
use lumis_core::formatter::html_inline::{
    HighlightLines as InlineHighlightLines, HighlightLinesStyle as InlineHighlightLinesStyle,
    HtmlInline,
};
use lumis_core::formatter::html_linked::{HighlightLines as LinkedHighlightLines, HtmlLinked};
use lumis_core::formatter::terminal::{Background as TerminalBackground, Terminal};
use lumis_core::formatter::{Formatter as _, HtmlElement};
use lumis_core::languages::Language;
use lumis_core::themes::{Appearance, Style, Theme};
use lumis_wasm_runtime::{
    catalog, store, Fetcher as _, HighlightOptions, InjectionResolution, Runtime,
};
use napi::bindgen_prelude::{AsyncTask, Buffer, FnArgs, Function};
use napi::{Env, Error, Result, Status, Task};
use napi_derive::napi;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::ops::RangeInclusive;
use std::path::PathBuf;
use std::sync::{Arc, LazyLock, Mutex};

fn native_error(error: impl std::fmt::Display) -> Error {
    Error::new(Status::GenericFailure, error.to_string())
}

#[derive(Deserialize)]
#[serde(untagged)]
enum LineSpec {
    Single(usize),
    Range([usize; 2]),
}

impl LineSpec {
    fn into_range(self) -> RangeInclusive<usize> {
        match self {
            Self::Single(line) => line..=line,
            Self::Range([start, end]) => start..=end,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsHighlightLines {
    lines: Vec<LineSpec>,
    style: Option<String>,
    class: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsHtmlElement {
    open_tag: String,
    close_tag: String,
}

impl From<JsHtmlElement> for HtmlElement {
    fn from(value: JsHtmlElement) -> Self {
        Self {
            open_tag: value.open_tag,
            close_tag: value.close_tag,
        }
    }
}

#[derive(Deserialize)]
struct JsTheme {
    name: String,
    appearance: Appearance,
    #[serde(default)]
    revision: String,
    highlights: BTreeMap<String, Style>,
}

impl From<JsTheme> for Theme {
    fn from(value: JsTheme) -> Self {
        Self {
            name: value.name,
            appearance: value.appearance,
            revision: value.revision,
            highlights: value.highlights,
        }
    }
}

#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct HtmlInlineOptions {
    theme: Option<JsTheme>,
    pre_class: Option<String>,
    italic: bool,
    include_highlights: bool,
    highlight_lines: Option<JsHighlightLines>,
    header: Option<JsHtmlElement>,
}

#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct HtmlLinkedOptions {
    pre_class: Option<String>,
    highlight_lines: Option<JsHighlightLines>,
    header: Option<JsHtmlElement>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct TerminalOptions {
    theme: Option<JsTheme>,
}

#[napi(object)]
pub struct NativeFormatter {
    pub rainbow_brackets: Option<bool>,
    pub kind: String,
    pub options: serde_json::Value,
}

/// A rendered document plus the injected languages the walk could not load.
#[napi(object)]
pub struct NativeFormatted {
    pub output: String,
    pub unresolved: Vec<String>,
}

/// Highlight events plus the injected languages the walk could not load.
#[napi(object)]
pub struct NativeHighlight {
    pub events: Buffer,
    pub unresolved: Vec<String>,
}

/// A resolved language package, flattened for the addon boundary.
#[napi(object)]
pub struct NativeLanguageSpec {
    pub id: String,
    pub aliases: Vec<String>,
    /// Read from the parser's exports when omitted.
    pub grammar_name: Option<String>,
    pub highlights: String,
    pub injections: Option<String>,
    pub locals: Option<String>,
    pub brackets: Option<String>,
}

struct FormatRequest {
    runtime: Arc<Runtime>,
    source: String,
    runtime_language: String,
    display_language: String,
    internal_ids: HashMap<String, String>,
    public_ids: HashMap<String, String>,
    formatter: NativeFormatter,
}
const FORGIVING_BASE64: base64::engine::general_purpose::GeneralPurpose =
    base64::engine::general_purpose::GeneralPurpose::new(
        &base64::alphabet::STANDARD,
        base64::engine::general_purpose::GeneralPurposeConfig::new()
            .with_decode_allow_trailing_bits(true)
            .with_decode_padding_mode(base64::engine::DecodePaddingMode::Indifferent),
    );

type PackageResolverFunction<'env> = Function<'env, String, Option<String>>;
type WasmResolverFunction<'env> = Function<'env, FnArgs<(String, String)>, Option<String>>;

static RESOLVER_CALLBACKS: LazyLock<Mutex<HashMap<usize, usize>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

struct ResolverCallbackGuard {
    env: usize,
}

impl ResolverCallbackGuard {
    fn enter(env: &Env) -> Self {
        let env = env.raw() as usize;
        let mut callbacks = RESOLVER_CALLBACKS
            .lock()
            .expect("resolver callback lock poisoned");
        *callbacks.entry(env).or_default() += 1;
        Self { env }
    }
}

impl Drop for ResolverCallbackGuard {
    fn drop(&mut self) {
        let mut callbacks = RESOLVER_CALLBACKS
            .lock()
            .expect("resolver callback lock poisoned");
        let depth = callbacks
            .get_mut(&self.env)
            .expect("resolver callback guard is not registered");
        *depth -= 1;
        if *depth == 0 {
            callbacks.remove(&self.env);
        }
    }
}

fn reject_reentrant_highlight(env: &Env) -> Result<()> {
    if RESOLVER_CALLBACKS
        .lock()
        .expect("resolver callback lock poisoned")
        .contains_key(&(env.raw() as usize))
    {
        return Err(native_error(
            "native highlighting cannot be called from a language resolver callback",
        ));
    }
    Ok(())
}

/// Where the store looks, when the caller names it rather than the environment.
/// Read once, when the runtime is built.
static STORE_PATHS: Mutex<StorePaths> = Mutex::new(StorePaths {
    data_dir: None,
    consumed: false,
});

struct StorePaths {
    data_dir: Option<PathBuf>,
    /// Set when the runtime read them, which it does exactly once.
    consumed: bool,
}

static SHARED_RUNTIME: LazyLock<std::result::Result<Arc<Runtime>, String>> =
    LazyLock::new(|| build_runtime().map(Arc::new));
static SHARED_DEFINITIONS: LazyLock<Mutex<HashMap<String, Arc<lumis_wasm_runtime::LanguageSpec>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn build_runtime() -> std::result::Result<Runtime, String> {
    let workers = std::thread::available_parallelism().map_or(1, usize::from);
    lumis_wasm_runtime::runtime_with_catalog(language_store(None), workers)
        .map_err(|error| error.to_string())
}

fn shared_runtime() -> Result<&'static Arc<Runtime>> {
    SHARED_RUNTIME.as_ref().map_err(native_error)
}

/// Point the store at an explicit directory, overriding `LUMIS_DATA_DIR`.
///
/// The runtime reads these once, when it is first used, so this returns `false`
/// if that has already happened. Node sets a directory through the environment
/// at any time; the addon cannot, and silently ignoring the difference is how a
/// caller ends up writing to a directory it did not choose.
#[napi(js_name = "configureStore")]
pub fn configure_store(data_dir: Option<String>) -> bool {
    let mut paths = STORE_PATHS.lock().expect("store path lock poisoned");
    if paths.consumed {
        return false;
    }
    paths.data_dir = data_dir.map(PathBuf::from);

    let compile_cache = store::resolve_data_dir(paths.data_dir.clone());
    lumis_wasm_runtime::set_compile_cache_dir(compile_cache);
    true
}

/// The directory the store uses when nothing names one, so Node can defer to the
/// same `etcetera` resolution the addon itself runs instead of porting it.
#[napi(js_name = "defaultDataDir")]
pub fn default_data_dir_js() -> String {
    store::default_data_dir().to_string_lossy().into_owned()
}

/// Compile and validate cached parsers on Node's worker pool, writing their
/// Wasmtime modules into the selected data directory.
#[napi(js_name = "precompileLanguages")]
pub fn precompile_languages(
    names: Vec<String>,
    directory: Option<String>,
) -> AsyncTask<PrecompileLanguagesTask> {
    AsyncTask::new(PrecompileLanguagesTask { names, directory })
}

/// The same resolve, verify and cache path the CLI and the Elixir NIF use.
fn language_store(cache_dir: Option<PathBuf>) -> store::LanguageStore {
    let mut configured = STORE_PATHS.lock().expect("store path lock poisoned");
    configured.consumed = true;
    let cache_dir = store::resolve_data_dir(cache_dir.or_else(|| configured.data_dir.clone()));
    store::LanguageStore::new(
        store::StoreConfig { cache_dir },
        Box::new(store::HttpFetcher),
    )
}

fn resolver_scheme(source: &str) -> Option<(&str, &str)> {
    let (scheme, rest) = source.split_once(':')?;
    let mut bytes = scheme.bytes();
    if !bytes.next()?.is_ascii_alphabetic()
        || !bytes.all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'+' | b'-' | b'.'))
    {
        return None;
    }
    Some((scheme, rest))
}

fn before_fragment(source: &str) -> &str {
    source.split_once('#').map_or(source, |(value, _)| value)
}

fn before_query_or_fragment(source: &str) -> &str {
    let query = source.find('?').unwrap_or(source.len());
    let fragment = source.find('#').unwrap_or(source.len());
    &source[..query.min(fragment)]
}

fn has_malformed_percent_escape(source: &str) -> bool {
    let bytes = source.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] != b'%' {
            index += 1;
            continue;
        }
        if bytes
            .get(index + 1..index + 3)
            .is_none_or(|escape| !escape.iter().all(u8::is_ascii_hexdigit))
        {
            return true;
        }
        index += 3;
    }
    false
}

fn is_infra_ascii_whitespace(byte: u8) -> bool {
    matches!(byte, b'\t' | b'\n' | 0x0c | b'\r' | b' ')
}

fn file_url_host_and_path(file: &str) -> (&str, String) {
    let (host, path) = if let Some(authority) = file.strip_prefix("//") {
        let (host, path) = authority.split_once('/').unwrap_or((authority, ""));
        (host, format!("/{path}"))
    } else {
        ("", file.to_string())
    };
    let bytes = host.as_bytes();
    if bytes.len() == 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' {
        ("", format!("/{host}{path}"))
    } else {
        (host, path)
    }
}

fn read_resolved_source(source: &str) -> std::result::Result<Vec<u8>, String> {
    if std::path::Path::new(source).is_file() {
        return std::fs::read(source).map_err(|error| error.to_string());
    }

    let Some((scheme, rest)) = resolver_scheme(source) else {
        return std::fs::read(source).map_err(|error| error.to_string());
    };

    if scheme.eq_ignore_ascii_case("data") {
        let data = before_fragment(rest);
        let (metadata, body) = data
            .split_once(',')
            .ok_or_else(|| "data URL has no comma separator".to_string())?;
        let decoded = percent_encoding::percent_decode_str(body).collect::<Vec<_>>();
        let is_base64 = metadata.rsplit_once(';').is_some_and(|(_, marker)| {
            marker
                .trim_matches(|character| matches!(character, '\t' | '\n' | '\r' | ' '))
                .eq_ignore_ascii_case("base64")
        });
        if is_base64 {
            let decoded = decoded
                .into_iter()
                .filter(|byte| !is_infra_ascii_whitespace(*byte))
                .collect::<Vec<_>>();
            return FORGIVING_BASE64
                .decode(decoded)
                .map_err(|error| error.to_string());
        }
        return Ok(decoded);
    }

    if scheme.eq_ignore_ascii_case("http") || scheme.eq_ignore_ascii_case("https") {
        return store::HttpFetcher.get(before_fragment(source));
    }

    if scheme.eq_ignore_ascii_case("file") {
        let file = before_query_or_fragment(rest);
        if has_malformed_percent_escape(file) {
            return Err("file URL contains a malformed percent escape".to_string());
        }
        let (host, path) = file_url_host_and_path(file);
        #[cfg(not(windows))]
        if !host.is_empty() && !host.eq_ignore_ascii_case("localhost") {
            return Err(format!("file URL names non-local host {host:?}"));
        }
        let lower = path.to_ascii_lowercase();
        if lower.contains("%2f") || (cfg!(windows) && lower.contains("%5c")) {
            return Err("file URL contains an encoded path separator".to_string());
        }
        let path = percent_encoding::percent_decode_str(&path)
            .decode_utf8()
            .map_err(|error| error.to_string())?;
        #[cfg(windows)]
        if !host.is_empty() && !host.eq_ignore_ascii_case("localhost") {
            return std::fs::read(format!(r"\\{host}\{}", path.trim_start_matches('/')))
                .map_err(|error| error.to_string());
        }
        #[cfg(windows)]
        let path = path.strip_prefix('/').unwrap_or(path.as_ref());
        #[cfg(windows)]
        return std::fs::read(path).map_err(|error| error.to_string());
        #[cfg(not(windows))]
        return std::fs::read(path.as_ref()).map_err(|error| error.to_string());
    }

    Err(format!("unsupported resolver URL scheme: {scheme}"))
}

const SOURCE_EVENT: u8 = 0;
const START_EVENT: u8 = 1;
const END_EVENT: u8 = 2;

fn inline_highlight_lines(value: JsHighlightLines) -> InlineHighlightLines {
    InlineHighlightLines {
        lines: value.lines.into_iter().map(LineSpec::into_range).collect(),
        style: Some(match value.style.as_deref() {
            None | Some("theme") => InlineHighlightLinesStyle::Theme,
            Some(style) => InlineHighlightLinesStyle::Style(style.to_string()),
        }),
        class: value.class,
    }
}

fn render_formatter(
    request: FormatRequest,
) -> std::result::Result<(String, Vec<String>), Box<dyn std::error::Error + Send + Sync>> {
    let FormatRequest {
        runtime,
        source,
        runtime_language,
        display_language,
        internal_ids,
        public_ids,
        formatter,
    } = request;
    let (mut events, unresolved) = highlight_events(
        &runtime,
        &source,
        &runtime_language,
        formatter.rainbow_brackets.unwrap_or(false),
        &internal_ids,
    )?;
    publicize_event_languages(&mut events, &public_ids);
    let output = render_events(&source, &display_language, formatter, &events)?;
    Ok((output, unresolved))
}

fn publicize_event_languages(events: &mut [HighlightEvent], ids: &HashMap<String, String>) {
    for event in events {
        if let HighlightEvent::Start { language, .. } = event {
            if let Some(public) = ids.get(language) {
                language.clone_from(public);
            }
        }
    }
}

fn render_events(
    source: &str,
    display_language: &str,
    formatter: NativeFormatter,
    events: &[HighlightEvent],
) -> std::result::Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let language = if display_language == "plaintext" {
        Language::PlainText
    } else {
        display_language.parse()?
    };
    let mut output = Vec::new();

    match formatter.kind.as_str() {
        "html-inline" => {
            let options: HtmlInlineOptions = serde_json::from_value(formatter.options)?;
            HtmlInline::new(
                language,
                options.theme.map(Theme::from),
                options.pre_class,
                options.italic,
                options.include_highlights,
                options.highlight_lines.map(inline_highlight_lines),
                options.header.map(HtmlElement::from),
            )
            .render(source, events, &mut output)?;
        }
        "html-linked" => {
            let options: HtmlLinkedOptions = serde_json::from_value(formatter.options)?;
            HtmlLinked::new(
                language,
                options.pre_class,
                options.highlight_lines.map(|lines| LinkedHighlightLines {
                    lines: lines.lines.into_iter().map(LineSpec::into_range).collect(),
                    class: lines.class.unwrap_or_else(|| "l-highlighted".to_string()),
                }),
                options.header.map(HtmlElement::from),
            )
            .render(source, events, &mut output)?;
        }
        "bbcode-scoped" => {
            BBCodeScoped::new(language).render(source, events, &mut output)?;
        }
        "terminal" => {
            let options: TerminalOptions = serde_json::from_value(formatter.options)?;
            Terminal::new(
                language,
                options.theme.map(Theme::from),
                TerminalBackground::Inherit,
                None,
            )
            .render(source, events, &mut output)?;
        }
        _ => {
            return Err(format!("unsupported native formatter '{}'", formatter.kind).into());
        }
    }

    Ok(String::from_utf8(output)?)
}

/// Encode the event protocol documented and decoded in
/// `src/core/native-event-codec.ts`.
fn encode_events(events: &[HighlightEvent]) -> Result<Buffer> {
    let mut output = Vec::with_capacity(events.len() * 9);
    for event in events {
        match event {
            HighlightEvent::Source { start, end } => {
                let start = u32::try_from(*start).map_err(native_error)?;
                let end = u32::try_from(*end).map_err(native_error)?;
                output.push(SOURCE_EVENT);
                output.extend_from_slice(&start.to_le_bytes());
                output.extend_from_slice(&end.to_le_bytes());
            }
            HighlightEvent::Start {
                scope_index,
                language,
            } => {
                let scope_index = u16::try_from(*scope_index).map_err(native_error)?;
                let language_len = u16::try_from(language.len()).map_err(native_error)?;
                output.push(START_EVENT);
                output.extend_from_slice(&scope_index.to_le_bytes());
                output.extend_from_slice(&language_len.to_le_bytes());
                output.extend_from_slice(language.as_bytes());
            }
            HighlightEvent::End => output.push(END_EVENT),
        }
    }
    Ok(output.into())
}

pub struct FormatTask {
    request: Option<FormatRequest>,
}

impl Task for FormatTask {
    type Output = (String, Vec<String>);
    type JsValue = NativeFormatted;

    fn compute(&mut self) -> Result<Self::Output> {
        render_formatter(self.request.take().expect("format task already consumed"))
            .map_err(native_error)
    }

    fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
        Ok(NativeFormatted {
            output: output.0,
            unresolved: output.1,
        })
    }
}

pub struct PrecompileLanguagesTask {
    names: Vec<String>,
    directory: Option<String>,
}

impl Task for PrecompileLanguagesTask {
    type Output = bool;
    type JsValue = bool;

    fn compute(&mut self) -> Result<Self::Output> {
        let data_dir = store::resolve_data_dir(self.directory.clone().map(PathBuf::from));
        let language_store = store::LanguageStore::new(
            store::StoreConfig {
                cache_dir: data_dir.clone(),
            },
            Box::new(store::NoNetwork),
        );
        let runtime = Runtime::with_compile_cache_dir(1, data_dir)
            .map_err(native_error)?
            .with_store(language_store);
        let failures = self
            .names
            .iter()
            .zip(
                runtime
                    .precompile_languages(&self.names, lumis_wasm_runtime::compile_concurrency()),
            )
            .filter_map(|(name, result)| result.err().map(|error| format!("{name} ({error})")))
            .collect::<Vec<_>>();

        if failures.is_empty() {
            Ok(true)
        } else {
            Err(native_error(format!(
                "could not compile: {}",
                failures.join(", ")
            )))
        }
    }

    fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
        Ok(output)
    }
}

/// Resolve, download, verify and load `language`, then highlight in one pass.
///
/// Languages injected inside the document are loaded during the same walk, so
/// this is the whole of what highlighting needs.
fn highlight_events(
    runtime: &Runtime,
    source: &str,
    language: &str,
    rainbow_brackets: bool,
    internal_ids: &HashMap<String, String>,
) -> std::result::Result<(Vec<HighlightEvent>, Vec<String>), Box<dyn std::error::Error + Send + Sync>>
{
    if language == "plaintext" {
        return Ok((
            vec![HighlightEvent::Source {
                start: 0,
                end: source.len(),
            }],
            Vec::new(),
        ));
    }

    let output = runtime.highlight_with_resolver(
        source,
        language,
        &HighlightOptions {
            rainbow_brackets,
            ..HighlightOptions::default()
        },
        |injected| {
            internal_ids
                .get(&injected.to_ascii_lowercase())
                .cloned()
                .map_or(InjectionResolution::Fallback, InjectionResolution::Loaded)
        },
    )?;
    Ok((output.events, output.unresolved))
}

/// Highlighting over the shared Wasmtime runtime, promoted to an isolated
/// runtime when this instance accepts caller-owned language behavior.
#[napi]
pub struct NativeRuntime {
    runtime: Mutex<RuntimeState>,
    resolved_ids: Mutex<ResolvedIds>,
    replay: Mutex<ReplayState>,
}

enum RuntimeState {
    Uninitialized,
    Shared(Arc<Runtime>),
    Private(Arc<Runtime>),
}

#[derive(Default)]
struct ResolvedIds {
    internal_by_name: HashMap<String, String>,
    public_by_internal: HashMap<String, String>,
}

#[derive(Default)]
struct ReplayState {
    named: HashSet<String>,
    definitions: HashMap<String, Arc<lumis_wasm_runtime::LanguageSpec>>,
}

fn append_identity_field(identity: &mut Vec<u8>, value: &[u8]) {
    identity.extend_from_slice(&(value.len() as u64).to_le_bytes());
    identity.extend_from_slice(value);
}

fn definition_id(resolved: &str, spec: &lumis_wasm_runtime::LanguageSpec) -> String {
    let mut identity = Vec::with_capacity(
        resolved.len()
            + spec.grammar_name.len()
            + spec.wasm.len()
            + spec.highlights.len()
            + spec.injections.len()
            + spec.locals.len()
            + spec.brackets.len()
            + 7 * std::mem::size_of::<u64>(),
    );
    for field in [
        resolved.as_bytes(),
        spec.grammar_name.as_bytes(),
        spec.wasm.as_slice(),
        spec.highlights.as_bytes(),
        spec.injections.as_bytes(),
        spec.locals.as_bytes(),
        spec.brackets.as_bytes(),
    ] {
        append_identity_field(&mut identity, field);
    }
    format!(
        "{resolved}\u{1}{}",
        lumis_wasm_runtime::sha256_hex(&identity)
    )
}

impl NativeRuntime {
    fn runtime_from_state(state: &mut RuntimeState) -> Result<Arc<Runtime>> {
        match state {
            RuntimeState::Uninitialized => {
                let runtime = Arc::clone(shared_runtime()?);
                *state = RuntimeState::Shared(Arc::clone(&runtime));
                Ok(runtime)
            }
            RuntimeState::Shared(runtime) | RuntimeState::Private(runtime) => {
                Ok(Arc::clone(runtime))
            }
        }
    }

    fn current_runtime(&self) -> Result<Arc<Runtime>> {
        Self::runtime_from_state(&mut self.runtime.lock().expect("native runtime lock poisoned"))
    }

    fn private_runtime(&self) -> Result<Arc<Runtime>> {
        let mut state = self.runtime.lock().expect("native runtime lock poisoned");
        if let RuntimeState::Private(runtime) = &*state {
            return Ok(Arc::clone(runtime));
        }

        let private = build_runtime().map_err(native_error)?;
        let mut replay = self.replay.lock().expect("runtime replay lock poisoned");
        for id in &replay.named {
            private.load_named_language(id).map_err(native_error)?;
        }
        for spec in replay.definitions.values() {
            private
                .load_language(spec.as_ref().clone())
                .map_err(native_error)?;
        }
        replay.named.clear();
        replay.definitions.clear();
        let private = Arc::new(private);
        *state = RuntimeState::Private(Arc::clone(&private));
        Ok(private)
    }

    fn runtime_for_resolvers(&self, has_resolvers: bool) -> Result<Arc<Runtime>> {
        if has_resolvers {
            self.private_runtime()
        } else {
            self.current_runtime()
        }
    }

    fn mapped_id(&self, id: &str) -> Option<String> {
        self.resolved_ids
            .lock()
            .expect("resolved language lock poisoned")
            .internal_by_name
            .get(&id.to_ascii_lowercase())
            .cloned()
    }

    fn remember_language(&self, requested: &str, resolved: &str, aliases: &[String], id: &str) {
        let mut ids = self
            .resolved_ids
            .lock()
            .expect("resolved language lock poisoned");
        for name in std::iter::once(requested)
            .chain(std::iter::once(resolved))
            .chain(aliases.iter().map(String::as_str))
        {
            ids.internal_by_name
                .insert(name.to_ascii_lowercase(), id.to_string());
        }
        ids.public_by_internal
            .insert(id.to_string(), resolved.to_string());
    }

    fn public_id(&self, internal: &str) -> String {
        self.resolved_ids
            .lock()
            .expect("resolved language lock poisoned")
            .public_by_internal
            .get(internal)
            .cloned()
            .unwrap_or_else(|| internal.to_string())
    }

    fn public_ids(&self) -> HashMap<String, String> {
        self.resolved_ids
            .lock()
            .expect("resolved language lock poisoned")
            .public_by_internal
            .clone()
    }

    fn internal_ids(&self) -> HashMap<String, String> {
        self.resolved_ids
            .lock()
            .expect("resolved language lock poisoned")
            .internal_by_name
            .clone()
    }

    fn publicize_events(&self, events: &mut [HighlightEvent]) {
        publicize_event_languages(events, &self.public_ids());
    }

    fn load_definition(
        &self,
        runtime: &Runtime,
        requested: &str,
        resolved: &str,
        aliases: &[String],
        mut spec: lumis_wasm_runtime::LanguageSpec,
        replay_on_promotion: bool,
    ) -> Result<String> {
        let id = definition_id(resolved, &spec);
        spec.id.clone_from(&id);
        spec.aliases.clear();
        let replay = replay_on_promotion.then(|| spec.clone());
        runtime.load_language(spec).map_err(native_error)?;
        if let Some(spec) = replay {
            let mut shared = SHARED_DEFINITIONS
                .lock()
                .expect("shared definition lock poisoned");
            let spec = Arc::clone(shared.entry(id.clone()).or_insert_with(|| Arc::new(spec)));
            self.replay
                .lock()
                .expect("runtime replay lock poisoned")
                .definitions
                .insert(id.clone(), spec);
        }
        self.remember_language(requested, resolved, aliases, &id);
        Ok(id)
    }

    fn load_package(
        &self,
        id: &str,
        expected_package_name: &str,
        package_json: &str,
        wasm: &[u8],
        isolate: bool,
    ) -> Result<String> {
        let package =
            lumis_wasm_runtime::LanguagePackage::from_json(package_json).map_err(native_error)?;
        if package.package_name != expected_package_name {
            return Err(native_error(format!(
                "resolver returned {} for {expected_package_name}",
                package.package_name
            )));
        }
        package.verify_wasm(wasm).map_err(native_error)?;
        let (resolved, definition) = package.language(id).ok_or_else(|| {
            native_error(format!("{} does not define {id:?}", package.package_name))
        })?;
        let spec = lumis_wasm_runtime::LanguageSpec {
            id: String::new(),
            aliases: Vec::new(),
            grammar_name: package.parser.grammar_name.clone(),
            wasm: wasm.to_vec(),
            highlights: definition.highlights.clone(),
            injections: definition.injections.clone(),
            locals: definition.locals.clone(),
            brackets: definition.brackets.clone(),
        };
        if isolate {
            let runtime = self.private_runtime()?;
            self.load_definition(&runtime, id, resolved, &definition.aliases, spec, false)
        } else {
            let mut state = self.runtime.lock().expect("native runtime lock poisoned");
            let runtime = Self::runtime_from_state(&mut state)?;
            let replay_on_promotion = matches!(&*state, RuntimeState::Shared(_));
            self.load_definition(
                &runtime,
                id,
                resolved,
                &definition.aliases,
                spec,
                replay_on_promotion,
            )
        }
    }

    fn resolve_injected(
        &self,
        runtime: &Runtime,
        env: Env,
        injected: &str,
        package_resolver: Option<&PackageResolverFunction<'_>>,
        wasm_resolver: Option<&WasmResolverFunction<'_>>,
    ) -> InjectionResolution {
        if let Some(id) = self.mapped_id(injected) {
            return InjectionResolution::Loaded(id);
        }

        let Some(location) = catalog::find(injected) else {
            return InjectionResolution::Fallback;
        };
        let Some(package_resolver) = package_resolver else {
            return InjectionResolution::Fallback;
        };
        let package_source = {
            let _guard = ResolverCallbackGuard::enter(&env);
            match package_resolver.call(location.package_name.to_string()) {
                Ok(Some(source)) => source,
                Ok(None) => return InjectionResolution::Fallback,
                Err(_) => return InjectionResolution::Unresolved,
            }
        };

        let loaded = (|| -> Result<String> {
            let package_json = read_resolved_source(&package_source).map_err(native_error)?;
            let package_json = std::str::from_utf8(&package_json).map_err(native_error)?;
            let package = lumis_wasm_runtime::LanguagePackage::from_json(package_json)
                .map_err(native_error)?;
            if package.package_name != location.package_name {
                return Err(native_error(format!(
                    "resolver returned {} for {}",
                    package.package_name, location.package_name
                )));
            }
            let (resolved, definition) = package.language(injected).ok_or_else(|| {
                native_error(format!(
                    "{} does not define {injected:?}",
                    package.package_name
                ))
            })?;
            let wasm_ref = serde_json::to_string(&serde_json::json!({
                "packageName": &package.package_name,
                "name": &package.parser.name,
                "version": &package.version,
                "sha256": &package.parser.sha256,
                "size": package.parser.size,
            }))
            .map_err(native_error)?;
            let wasm_resolver =
                wasm_resolver.ok_or_else(|| native_error("WASM resolver is not configured"))?;
            let wasm_source = {
                let _guard = ResolverCallbackGuard::enter(&env);
                wasm_resolver
                    .call(FnArgs::from((resolved.to_string(), wasm_ref)))?
                    .ok_or_else(|| native_error("WASM resolver did not return a source"))?
            };
            let wasm = read_resolved_source(&wasm_source).map_err(native_error)?;
            package.verify_wasm(&wasm).map_err(native_error)?;

            self.load_definition(
                runtime,
                injected,
                resolved,
                &definition.aliases,
                lumis_wasm_runtime::LanguageSpec {
                    id: String::new(),
                    aliases: Vec::new(),
                    grammar_name: package.parser.grammar_name.clone(),
                    wasm,
                    highlights: definition.highlights.clone(),
                    injections: definition.injections.clone(),
                    locals: definition.locals.clone(),
                    brackets: definition.brackets.clone(),
                },
                false,
            )
        })();

        match loaded {
            Ok(id) => InjectionResolution::Loaded(id),
            Err(_) => InjectionResolution::Unresolved,
        }
    }
}

impl Default for NativeRuntime {
    fn default() -> Self {
        Self {
            runtime: Mutex::new(RuntimeState::Uninitialized),
            resolved_ids: Mutex::new(ResolvedIds::default()),
            replay: Mutex::new(ReplayState::default()),
        }
    }
}

#[napi]
impl NativeRuntime {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Load a caller-selected language package in this instance's isolated
    /// runtime, verifying its parser against the declared size and digest.
    #[napi(js_name = "loadLanguagePackage")]
    pub fn load_language_package(
        &self,
        id: String,
        expected_package_name: String,
        package_json: String,
        wasm: Buffer,
    ) -> Result<String> {
        self.load_package(&id, &expected_package_name, &package_json, &wasm, true)
    }

    /// Load a validated package installed at the canonical package name into
    /// the shared runtime. Once this instance is isolated, it stays isolated.
    #[napi(js_name = "loadInstalledLanguagePackage")]
    pub fn load_installed_language_package(
        &self,
        id: String,
        expected_package_name: String,
        package_json: String,
        wasm: Buffer,
    ) -> Result<String> {
        self.load_package(&id, &expected_package_name, &package_json, &wasm, false)
    }

    /// Load a language the caller defined entirely itself: its own parser bytes
    /// and its own queries, with no package metadata behind them.
    #[napi(js_name = "loadLanguageDefinition")]
    pub fn load_language_definition(
        &self,
        spec: NativeLanguageSpec,
        wasm: Buffer,
    ) -> Result<String> {
        let grammar_name = match spec.grammar_name {
            Some(name) => name,
            None => lumis_wasm_runtime::grammar_name(wasm.as_ref()).map_err(native_error)?,
        };
        let id = spec.id;
        let runtime = self.private_runtime()?;
        self.load_definition(
            &runtime,
            &id,
            &id,
            &spec.aliases,
            lumis_wasm_runtime::LanguageSpec {
                id: String::new(),
                aliases: Vec::new(),
                grammar_name,
                wasm: wasm.to_vec(),
                highlights: spec.highlights,
                injections: spec.injections.unwrap_or_default(),
                locals: spec.locals.unwrap_or_default(),
                brackets: spec.brackets.unwrap_or_default(),
            },
            false,
        )
    }

    /// Download, verify and load `id` ahead of a highlight that would do it
    /// anyway, so the cost does not land on a request.
    #[napi(js_name = "loadLanguage")]
    pub fn load_language(&self, id: String) -> Result<()> {
        let mut state = self.runtime.lock().expect("native runtime lock poisoned");
        let runtime = Self::runtime_from_state(&mut state)?;
        runtime.load_named_language(&id).map_err(native_error)?;
        if matches!(&*state, RuntimeState::Shared(_)) {
            self.replay
                .lock()
                .expect("runtime replay lock poisoned")
                .named
                .insert(id);
        }
        Ok(())
    }

    #[napi(js_name = "hasLanguage")]
    pub fn has_language(&self, id: String) -> Result<bool> {
        Ok(self
            .current_runtime()?
            .has_language(self.mapped_id(&id).as_deref().unwrap_or(&id)))
    }

    /// Download and cache `id` without loading it, for build-time prefetching.
    /// Returns the path its parser was written to.
    #[napi(js_name = "cacheLanguage")]
    pub fn cache_language(
        &self,
        id: String,
        directory: Option<String>,
        force: Option<bool>,
    ) -> Result<String> {
        let runtime = self.current_runtime()?;
        let owned;
        let store = match directory {
            Some(directory) => {
                owned = language_store(Some(PathBuf::from(directory)));
                &owned
            }
            None => runtime
                .store()
                .ok_or_else(|| native_error("this runtime has no language store"))?,
        };
        store
            .cache_language(&id, force.unwrap_or(false))
            .map(|path| path.display().to_string())
            .map_err(native_error)
    }

    /// The complete nested event stream as one compact binary value, plus any
    /// injected language the walk found and could not load.
    ///
    /// A configured JavaScript resolver is called from the walk before the
    /// shared catalog store. Failure leaves only that injected block plain.
    #[napi(js_name = "highlightEvents")]
    pub fn highlight_events(
        &self,
        env: Env,
        source: String,
        language: String,
        rainbow_brackets: Option<bool>,
        package_resolver: Option<PackageResolverFunction<'_>>,
        wasm_resolver: Option<WasmResolverFunction<'_>>,
    ) -> Result<NativeHighlight> {
        reject_reentrant_highlight(&env)?;
        let runtime =
            self.runtime_for_resolvers(package_resolver.is_some() || wasm_resolver.is_some())?;
        let mut output = runtime
            .highlight_with_resolver(
                &source,
                &language,
                &HighlightOptions {
                    rainbow_brackets: rainbow_brackets.unwrap_or(false),
                    ..HighlightOptions::default()
                },
                |injected| {
                    self.resolve_injected(
                        &runtime,
                        env,
                        injected,
                        package_resolver.as_ref(),
                        wasm_resolver.as_ref(),
                    )
                },
            )
            .map_err(native_error)?;
        self.publicize_events(&mut output.events);
        Ok(NativeHighlight {
            events: encode_events(&output.events)?,
            unresolved: output.unresolved,
        })
    }

    /// Parse and render built-in formatters entirely in Rust, returning one string.
    #[napi]
    pub fn format(
        &self,
        env: Env,
        source: String,
        language: String,
        formatter: NativeFormatter,
        package_resolver: Option<PackageResolverFunction<'_>>,
        wasm_resolver: Option<WasmResolverFunction<'_>>,
    ) -> Result<NativeFormatted> {
        reject_reentrant_highlight(&env)?;
        if language == "plaintext" {
            let (output, unresolved) = render_formatter(FormatRequest {
                runtime: self.current_runtime()?,
                source,
                runtime_language: language.clone(),
                display_language: language,
                internal_ids: HashMap::new(),
                public_ids: HashMap::new(),
                formatter,
            })
            .map_err(native_error)?;
            return Ok(NativeFormatted { output, unresolved });
        }

        let runtime =
            self.runtime_for_resolvers(package_resolver.is_some() || wasm_resolver.is_some())?;
        let mut highlighted = runtime
            .highlight_with_resolver(
                &source,
                &language,
                &HighlightOptions {
                    rainbow_brackets: formatter.rainbow_brackets.unwrap_or(false),
                    ..HighlightOptions::default()
                },
                |injected| {
                    self.resolve_injected(
                        &runtime,
                        env,
                        injected,
                        package_resolver.as_ref(),
                        wasm_resolver.as_ref(),
                    )
                },
            )
            .map_err(native_error)?;
        self.publicize_events(&mut highlighted.events);
        let output = render_events(
            &source,
            &self.public_id(&language),
            formatter,
            &highlighted.events,
        )
        .map_err(native_error)?;
        Ok(NativeFormatted {
            output,
            unresolved: highlighted.unresolved,
        })
    }

    /// Run async API formatting on Node's worker pool.
    #[napi(js_name = "formatAsync")]
    pub fn format_async(
        &self,
        source: String,
        language: String,
        formatter: NativeFormatter,
    ) -> Result<AsyncTask<FormatTask>> {
        let display_language = self.public_id(&language);
        Ok(AsyncTask::new(FormatTask {
            request: Some(FormatRequest {
                runtime: self.current_runtime()?,
                source,
                runtime_language: language,
                display_language,
                internal_ids: self.internal_ids(),
                public_ids: self.public_ids(),
                formatter,
            }),
        }))
    }
}

#[napi]
pub fn runtime_kind() -> &'static str {
    "native"
}

#[cfg(test)]
mod tests {
    use super::{file_url_host_and_path, has_malformed_percent_escape, read_resolved_source};

    #[test]
    fn drive_letter_file_authority_is_a_local_path() {
        assert_eq!(
            file_url_host_and_path("//C:/dir/parser.wasm"),
            ("", "/C:/dir/parser.wasm".to_string())
        );
    }

    #[test]
    fn malformed_percent_escapes_are_detected() {
        assert!(has_malformed_percent_escape("/tmp/%ZZ/parser.wasm"));
        assert!(has_malformed_percent_escape("/tmp/parser%"));
        assert!(!has_malformed_percent_escape("/tmp/%25/parser.wasm"));
    }

    #[test]
    fn data_urls_use_forgiving_base64_and_only_a_terminal_marker() {
        assert_eq!(
            read_resolved_source("data:text/plain;base64,SGVsbG8").unwrap(),
            b"Hello"
        );
        assert_eq!(
            read_resolved_source("data:text/plain;base64;parameter,Hello%20world").unwrap(),
            b"Hello world"
        );
        assert!(read_resolved_source("data:text/plain;base64,SGV%0BbG8=").is_err());
    }
}

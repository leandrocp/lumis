//! Rust's half of the cross-runtime formatter helper check.
//!
//! `fixtures/formatter-helpers.json` lists the helper capabilities every runtime
//! must offer a custom formatter. Rust cannot reflect on a module at run time, so
//! the check has five parts:
//!
//! - `every_manifest_helper_is_callable` calls every helper by name. A helper
//!   added to the manifest that Rust lacks fails to **compile**.
//! - `manifest_matches_the_helpers_exercised_here` reads the manifest and
//!   requires it to name exactly the helpers exercised above.
//! - `every_helper_matches_the_shared_output_contract` feeds each helper the
//!   manifest's shared inputs and compares its string result.
//! - `every_public_helper_is_accounted_for` parses the helper modules and fails
//!   on a `pub fn` that is in neither the canonical set, `runtime_only`, nor
//!   `deprecated`. This is the one that catches drift: a helper added to Rust
//!   and nowhere else has to be classified before it can ship.
//! - `deprecated_helpers_carry_the_attribute` requires every manifest deprecation
//!   to be a real `#[deprecated]`, so the JSON cannot claim one that is not there.
//!
//! Gated on `lang-rust` because some helpers take a `Language`, and the catalog
//! is feature-gated.
#![cfg(feature = "lang-rust")]
#![allow(deprecated)]

use lumis::events::HighlightEvent;
use lumis::highlights::HIGHLIGHT_NAMES;
use lumis::themes::{Style, TextDecoration, Theme, UnderlineStyle};
use lumis::{ansi, html, languages::Language, themes};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};
use std::str::FromStr as _;
use syn::{Item, Meta};

#[derive(Debug, Deserialize)]
struct Manifest {
    contract: Contract,
    modules: BTreeMap<String, ModuleEntry>,
    runtime_only: BTreeMap<String, serde_json::Value>,
    deprecated: BTreeMap<String, serde_json::Value>,
    waived: BTreeMap<String, serde_json::Value>,
}

impl Manifest {
    /// The helpers `module` deprecates for `runtime`. `$comment` keys, whose
    /// values are arrays of prose rather than entries, are skipped.
    fn deprecated_names(&self, module: &str, runtime: &str) -> BTreeSet<String> {
        self.deprecated
            .get(module)
            .and_then(serde_json::Value::as_object)
            .map(|entries| {
                entries
                    .iter()
                    .filter(|(name, _)| !name.starts_with('$'))
                    .filter(|(_, entry)| {
                        entry["runtimes"]
                            .as_array()
                            .expect("a deprecation lists its runtimes")
                            .iter()
                            .any(|value| value == runtime)
                    })
                    .map(|(name, _)| name.clone())
                    .collect()
            })
            .unwrap_or_default()
    }
}

#[derive(Debug, Deserialize)]
struct ModuleEntry {
    helpers: Vec<HelperEntry>,
}

#[derive(Debug, Deserialize)]
struct HelperEntry {
    name: String,
    expected: String,
    #[serde(default)]
    spelling: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct Contract {
    themes: BTreeMap<String, serde_json::Value>,
    html: HtmlContract,
    style: StyleContract,
    ansi: AnsiContract,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HtmlContract {
    escape_text: String,
    braced_text: String,
    text: String,
    scope: String,
    linked_scope: String,
    language: String,
    theme: String,
    themes: BTreeMap<String, String>,
    theme_name: String,
    pre_class: String,
    line: LineContract,
    lines: Vec<LineContractRange>,
    selected_line: usize,
    highlight_class: String,
    default_highlight_class: String,
    source: String,
    events: Vec<ContractEvent>,
    line_ending_cases: Vec<LineEndingCase>,
}

#[derive(Debug, Deserialize)]
struct LineEndingCase {
    source: String,
    expected: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct LineContract {
    number: usize,
    content: String,
    class: String,
    style: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum LineContractRange {
    Number(usize),
    Range([usize; 2]),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ContractEvent {
    Start { scope: String, language: String },
    Source { start: usize, end: usize },
    End,
}

#[derive(Debug, Deserialize)]
struct StyleContract {
    fg: Option<String>,
    bg: Option<String>,
    bold: bool,
    italic: bool,
    underline: String,
    strikethrough: bool,
}

impl StyleContract {
    fn style(&self) -> Style {
        let underline = match self.underline.as_str() {
            "none" => UnderlineStyle::None,
            "solid" => UnderlineStyle::Solid,
            "wavy" => UnderlineStyle::Wavy,
            "double" => UnderlineStyle::Double,
            "dotted" => UnderlineStyle::Dotted,
            "dashed" => UnderlineStyle::Dashed,
            other => panic!("unknown contract underline style {other:?}"),
        };

        Style {
            fg: self.fg.clone(),
            bg: self.bg.clone(),
            bold: self.bold,
            italic: self.italic,
            text_decoration: TextDecoration {
                underline,
                strikethrough: self.strikethrough,
            },
        }
    }
}

#[derive(Debug, Deserialize)]
struct AnsiContract {
    hex: String,
    rgb: [u8; 3],
    background: bool,
    text: String,
}

impl HelperEntry {
    /// The name Rust spells this capability, which is the canonical one unless
    /// the manifest overrides it. An override containing `::` names a home
    /// outside the helper module.
    fn rust_name(&self) -> &str {
        self.spelling
            .get("rust")
            .map_or(self.name.as_str(), String::as_str)
    }
}

fn manifest() -> Manifest {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/formatter-helpers.json")
        .canonicalize()
        .expect("formatter-helpers.json is reachable");
    serde_json::from_str(&fs::read_to_string(path).expect("read formatter-helpers.json"))
        .expect("parse formatter-helpers.json")
}

fn fixture_theme(contract: &Contract, name: &str) -> Theme {
    let source = contract
        .themes
        .get(name)
        .unwrap_or_else(|| panic!("{name:?} is not a fixture theme"));
    themes::from_json(&source.to_string()).expect("fixture theme is valid")
}

fn fixture_themes(contract: &Contract) -> HashMap<String, Theme> {
    contract
        .html
        .themes
        .iter()
        .map(|(name, theme)| (name.clone(), fixture_theme(contract, theme)))
        .collect()
}

#[test]
fn render_lines_preserves_the_shared_line_ending_contract() {
    let input = manifest().contract.html;

    for case in input.line_ending_cases {
        let events: [HighlightEvent<'_, ()>; 1] = [HighlightEvent::Source {
            start: 0,
            end: case.source.len(),
        }];

        assert_eq!(
            html::render_lines_from_events(&case.source, &events, |_, _| String::new()),
            case.expected,
            "source {:?}",
            case.source
        );
    }
}

fn language(name: &str) -> Language {
    Language::from_str(name).unwrap_or_else(|_| panic!("{name:?} is not a language"))
}

fn scope_index(scope: &str) -> usize {
    HIGHLIGHT_NAMES
        .iter()
        .position(|&candidate| candidate == scope)
        .unwrap_or_else(|| panic!("{scope:?} is not a highlight scope"))
}

fn line_ranges(input: &HtmlContract) -> Vec<RangeInclusive<usize>> {
    input
        .lines
        .iter()
        .map(|line| match line {
            LineContractRange::Number(number) => *number..=*number,
            LineContractRange::Range([start, end]) => *start..=*end,
        })
        .collect()
}

fn highlight_events(input: &HtmlContract) -> Vec<HighlightEvent<'_, ()>> {
    input
        .events
        .iter()
        .map(|event| match event {
            ContractEvent::Start { scope, language } => HighlightEvent::Start {
                scope_index: scope_index(scope),
                language: language.clone(),
            },
            ContractEvent::Source { start, end } => HighlightEvent::Source {
                start: *start,
                end: *end,
            },
            ContractEvent::End => HighlightEvent::End,
        })
        .collect()
}

fn written(write: impl FnOnce(&mut Vec<u8>) -> std::io::Result<()>) -> String {
    let mut output = Vec::new();
    write(&mut output).expect("writing to a Vec cannot fail");
    String::from_utf8(output).expect("formatter helpers emit UTF-8")
}

fn open_tag(name: &str, attrs: &html::HtmlAttrs) -> String {
    written(|output| html::open_tag(output, name, attrs))
}

/// Every helper in the manifest, called once. Adding a helper to the manifest
/// without adding it here fails `manifest_matches_the_helpers_exercised_here`;
/// adding it here without a Rust function fails to compile.
fn exercised_helpers(
    contract: &Contract,
) -> BTreeMap<&'static str, BTreeMap<&'static str, String>> {
    let input = &contract.html;
    let theme = fixture_theme(contract, &input.theme);
    let themes = fixture_themes(contract);
    let language = language(&input.language);
    let style = contract.style.style();
    let lines = line_ranges(input);
    let events = highlight_events(input);
    let class_suffix = format!(" {}", input.line.class);
    let inline_attrs =
        html::span_inline_attrs(Some(language), &input.scope, Some(&theme), false, false);
    let multi_theme_attrs = html::span_multi_themes_attrs(
        &input.scope,
        Some(language),
        &themes,
        None,
        "--lumis",
        false,
        false,
    );
    let pre_attrs = html::pre_attrs(Some(&input.pre_class), Some(&theme), &[]);
    let multi_themes_pre_attrs =
        html::multi_themes_pre_attrs(Some(&input.pre_class), &themes, None, "--lumis", &[]);
    let code_attrs = html::code_attrs(&language, &[]);
    let [red, green, blue] = contract.ansi.rgb;

    [
        (
            "html",
            BTreeMap::from([
                ("escape", html::escape(&input.escape_text)),
                ("escape_attr", html::escape_attr(&input.escape_text)),
                ("escape_braces", html::escape_braces(&input.braced_text)),
                ("scope_to_class", html::scope_to_class(&input.linked_scope)),
                ("themes::Style::css", Style::css(&style, true, " ")),
                (
                    "text_decoration",
                    html::text_decoration(&style.text_decoration).to_string(),
                ),
                (
                    "sanitize_theme_name",
                    html::sanitize_theme_name(&input.theme_name),
                ),
                (
                    "open_span",
                    html::open_span(&html::span_linked_attrs(&input.scope)),
                ),
                ("span_inline_attrs", html::open_span(&inline_attrs)),
                (
                    "span_inline",
                    html::span_inline(
                        &input.text,
                        Some(language),
                        &input.scope,
                        Some(&theme),
                        false,
                        false,
                    ),
                ),
                (
                    "span_linked_attrs",
                    html::span_linked_attrs(&input.linked_scope),
                ),
                (
                    "span_linked",
                    html::span_linked(&input.text, &input.linked_scope),
                ),
                (
                    "span_multi_themes_attrs",
                    html::open_span(&multi_theme_attrs),
                ),
                (
                    "span_multi_themes",
                    html::span_multi_themes(
                        &input.text,
                        &input.scope,
                        Some(language),
                        &themes,
                        None,
                        "--lumis",
                        false,
                        false,
                    ),
                ),
                (
                    "open_tag",
                    open_tag(
                        "pre",
                        &vec![
                            (
                                "class".to_string(),
                                format!("lumis {}", input.pre_class).into(),
                            ),
                            ("hidden".to_string(), true.into()),
                        ],
                    ),
                ),
                (
                    "is_valid_attr_name",
                    html::is_valid_attr_name("x onclick=alert(1)").to_string(),
                ),
                ("pre_attrs", open_tag("pre", &pre_attrs)),
                (
                    "multi_themes_pre_attrs",
                    open_tag("pre", &multi_themes_pre_attrs),
                ),
                ("code_attrs", open_tag("code", &code_attrs)),
                (
                    "open_pre_tag",
                    written(|output| {
                        html::open_pre_tag(output, Some(&input.pre_class), Some(&theme))
                    }),
                ),
                (
                    "open_multi_themes_pre_tag",
                    written(|output| {
                        html::open_multi_themes_pre_tag(
                            output,
                            Some(&input.pre_class),
                            &themes,
                            None,
                            "--lumis",
                        )
                    }),
                ),
                (
                    "open_code_tag",
                    written(|output| html::open_code_tag(output, &language)),
                ),
                (
                    "close_pre_tag",
                    written(|output| html::close_pre_tag(output)),
                ),
                (
                    "close_code_tag",
                    written(|output| html::close_code_tag(output)),
                ),
                ("closing_tags", written(|output| html::closing_tags(output))),
                (
                    "wrap_line",
                    html::wrap_line(
                        input.line.number,
                        &input.line.content,
                        Some(&class_suffix),
                        Some(&input.line.style),
                    ),
                ),
                (
                    "line_is_highlighted",
                    html::line_is_highlighted(&lines, input.selected_line).to_string(),
                ),
                (
                    "highlight_line_class",
                    html::highlight_line_class(
                        &lines,
                        input.selected_line,
                        Some(&input.highlight_class),
                        Some(&input.default_highlight_class),
                    )
                    .unwrap_or_default()
                    .to_string(),
                ),
                (
                    "render_lines_from_events",
                    serde_json::to_string(&html::render_lines_from_events(
                        &input.source,
                        &events,
                        |scope_index, _language| {
                            html::span_linked_attrs(HIGHLIGHT_NAMES[scope_index])
                        },
                    ))
                    .expect("line output serializes"),
                ),
            ]),
        ),
        (
            "ansi",
            BTreeMap::from([
                (
                    "hex_to_rgb",
                    ansi::hex_to_rgb(&contract.ansi.hex)
                        .map(|(red, green, blue)| format!("{red},{green},{blue}"))
                        .unwrap_or_default(),
                ),
                (
                    "rgb_to_ansi",
                    ansi::rgb_to_ansi(red, green, blue, contract.ansi.background),
                ),
                ("style_to_ansi", ansi::style_to_ansi(&style)),
                ("paint", ansi::paint(&contract.ansi.text, &style)),
                ("ANSI_RESET", ansi::ANSI_RESET.to_string()),
            ]),
        ),
    ]
    .into()
}

/// Every `pub fn` in a helper module, and whether it carries `#[deprecated]`.
///
/// `lumis::html` and `lumis::ansi` are the published surface, and
/// `lumis_core::formatter::{html,ansi}` is what they wrap; a helper in either is
/// public, so both are read.
fn public_helpers(module: &str) -> BTreeMap<String, bool> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let sources = [
        manifest_dir.join(format!("src/formatter/{module}.rs")),
        manifest_dir.join(format!("../lumis-core/src/formatter/{module}.rs")),
    ];

    let mut helpers = BTreeMap::new();
    for path in &sources {
        collect_public_helpers(path, &mut helpers);
    }

    assert!(
        helpers.len() > 5,
        "{module}: source scan found almost nothing: {} helpers",
        helpers.len()
    );

    helpers
}

fn collect_public_helpers(path: &Path, helpers: &mut BTreeMap<String, bool>) {
    let source =
        fs::read_to_string(path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    let syntax = syn::parse_file(&source)
        .unwrap_or_else(|error| panic!("parse {}: {error}", path.display()));

    for item in &syntax.items {
        let (name, attrs, visibility) = match item {
            Item::Fn(item) => (item.sig.ident.to_string(), &item.attrs, &item.vis),
            Item::Const(item) => (item.ident.to_string(), &item.attrs, &item.vis),
            _ => continue,
        };

        if !matches!(visibility, syn::Visibility::Public(_)) {
            continue;
        }

        let deprecated = attrs.iter().any(|attr| match &attr.meta {
            Meta::Path(path) | Meta::List(syn::MetaList { path, .. }) => {
                path.is_ident("deprecated")
            }
            Meta::NameValue(_) => false,
        });

        // A helper wrapped by `lumis` and defined in `lumis-core` is one helper;
        // deprecating it in either place deprecates it.
        helpers
            .entry(name)
            .and_modify(|already| *already |= deprecated)
            .or_insert(deprecated);
    }
}

#[test]
fn every_manifest_helper_is_callable() {
    // The helpers run here; reaching this line means they all exist and build.
    let manifest = manifest();
    let exercised = exercised_helpers(&manifest.contract);
    assert_eq!(exercised.len(), 2, "both helper modules are covered");
}

#[test]
fn manifest_matches_the_helpers_exercised_here() {
    let manifest = manifest();
    let exercised = exercised_helpers(&manifest.contract);

    assert_eq!(
        manifest.modules.keys().collect::<Vec<_>>(),
        exercised.keys().collect::<Vec<_>>(),
        "manifest and Rust disagree about which helper modules exist"
    );

    for (module, entry) in &manifest.modules {
        let expected: BTreeSet<&str> = entry.helpers.iter().map(HelperEntry::rust_name).collect();
        let actual: BTreeSet<&str> = exercised[module.as_str()].keys().copied().collect();
        assert_eq!(
            &expected, &actual,
            "{module}: manifest helpers and the Rust functions exercised here disagree"
        );
    }
}

#[test]
fn every_helper_matches_the_shared_output_contract() {
    let manifest = manifest();
    let outputs = exercised_helpers(&manifest.contract);

    for (module, entry) in &manifest.modules {
        for helper in &entry.helpers {
            assert_eq!(
                outputs[module.as_str()][helper.rust_name()],
                helper.expected,
                "{}.{}",
                module,
                helper.rust_name()
            );
        }
    }
}

#[test]
fn every_public_helper_is_accounted_for() {
    let manifest = manifest();

    for (module, entry) in &manifest.modules {
        let canonical: BTreeSet<String> = entry
            .helpers
            .iter()
            .map(|helper| {
                // A spelling naming a home outside the module is checked by the
                // call above, not by the scan.
                helper
                    .rust_name()
                    .rsplit("::")
                    .next()
                    .expect("non-empty name")
                    .to_string()
            })
            .collect();

        let runtime_only = names_under(&manifest.runtime_only, "rust", module);
        let deprecated = manifest.deprecated_names(module, "rust");

        let public = public_helpers(module);
        let unaccounted: Vec<&String> = public
            .keys()
            .filter(|name| {
                !canonical.contains(*name)
                    && !runtime_only.contains(*name)
                    && !deprecated.contains(*name)
            })
            .collect();

        assert!(
            unaccounted.is_empty(),
            "{module}: public helpers in neither the manifest's helper set, runtime_only, \
             nor deprecated: {unaccounted:?}. Add them to fixtures/formatter-helpers.json \
             and to the other runtimes, or classify them."
        );
    }
}

#[test]
fn deprecated_helpers_carry_the_attribute() {
    let manifest = manifest();

    for module in manifest.modules.keys() {
        let public = public_helpers(module);

        for name in manifest.deprecated_names(module, "rust") {
            let deprecated = public.get(&name).unwrap_or_else(|| {
                panic!("{module}: the manifest deprecates {name}, which Rust does not export")
            });

            assert!(
                *deprecated,
                "{module}: the manifest deprecates {name}, but Rust does not mark it #[deprecated]"
            );
        }
    }
}

#[test]
fn every_runtime_only_helper_still_exists() {
    let manifest = manifest();

    for module in manifest.modules.keys() {
        let public = public_helpers(module);

        for name in names_under(&manifest.runtime_only, "rust", module) {
            assert!(
                public.contains_key(&name),
                "{module}: runtime_only lists {name}, which Rust no longer exports; \
                 drop the entry"
            );
        }
    }
}

#[test]
fn no_waiver_outlives_its_reason() {
    let manifest = manifest();
    let waivers: Vec<&String> = manifest
        .waived
        .keys()
        .filter(|key| !key.starts_with('$'))
        .collect();

    assert!(
        waivers.is_empty(),
        "every runtime offers every capability; drop these waivers: {waivers:?}"
    );
}

fn names_under(
    section: &BTreeMap<String, serde_json::Value>,
    runtime: &str,
    module: &str,
) -> BTreeSet<String> {
    section
        .get(runtime)
        .and_then(|runtime| runtime.get(module))
        .and_then(serde_json::Value::as_object)
        .map(|names| {
            names
                .keys()
                .filter(|name| !name.starts_with('$'))
                .cloned()
                .collect()
        })
        .unwrap_or_default()
}

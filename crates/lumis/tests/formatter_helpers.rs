//! Rust's half of the cross-runtime formatter helper check.
//!
//! `fixtures/formatter-helpers.json` lists the helper capabilities every runtime
//! must offer a custom formatter. Rust cannot reflect on a module at run time, so
//! the check has four parts:
//!
//! - `every_manifest_helper_is_callable` calls every helper by name. A helper
//!   added to the manifest that Rust lacks fails to **compile**.
//! - `manifest_matches_the_helpers_exercised_here` reads the manifest and
//!   requires it to name exactly the helpers exercised above.
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
use lumis::themes::{Style, TextDecoration, Theme};
use lumis::{ansi, html, languages::Language, themes};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use syn::{Item, Meta};

#[derive(Debug, Deserialize)]
struct Manifest {
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
    #[serde(default)]
    spelling: BTreeMap<String, String>,
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

fn theme() -> Theme {
    themes::get("dracula").expect("dracula is built in")
}

fn theme_map() -> HashMap<String, Theme> {
    let mut themes = HashMap::new();
    themes.insert("light".to_string(), themes::get("github_light").unwrap());
    themes.insert("dark".to_string(), theme());
    themes
}

fn keyword_index() -> usize {
    HIGHLIGHT_NAMES
        .iter()
        .position(|&scope| scope == "keyword")
        .expect("keyword is a highlight scope")
}

/// Every helper in the manifest, called once. Adding a helper to the manifest
/// without adding it here fails `manifest_matches_the_helpers_exercised_here`;
/// adding it here without a Rust function fails to compile.
fn exercised_helpers() -> BTreeMap<&'static str, BTreeSet<&'static str>> {
    let theme = theme();
    let themes = theme_map();
    let style = theme.get_style("keyword").expect("dracula styles keyword");
    let mut output = Vec::new();

    html::escape("<b>");
    html::escape_attr(r#"x"><script>"#);
    html::escape_braces("{}");
    html::scope_to_class("keyword");
    Style::css(style, false, " ");
    html::text_decoration(&TextDecoration::default());
    html::sanitize_theme_name("catppuccin mocha");
    html::open_span(&html::span_linked_attrs("keyword"));
    html::span_inline_attrs(Some(Language::Rust), "keyword", Some(&theme), false, false);
    html::span_inline(
        "fn",
        Some(Language::Rust),
        "keyword",
        Some(&theme),
        false,
        false,
    );
    html::span_linked_attrs("keyword");
    html::span_linked("fn", "keyword");
    html::span_multi_themes_attrs(
        "keyword",
        None,
        &themes,
        Some("light"),
        "--lumis",
        false,
        false,
    );
    html::span_multi_themes(
        "fn",
        "keyword",
        None,
        &themes,
        Some("light"),
        "--lumis",
        false,
        false,
    );
    html::open_pre_tag(&mut output, Some("code"), Some(&theme)).unwrap();
    html::open_multi_themes_pre_tag(&mut output, Some("code"), &themes, Some("light"), "--lumis")
        .unwrap();
    html::open_code_tag(&mut output, &Language::Rust).unwrap();
    html::close_pre_tag(&mut output).unwrap();
    html::close_code_tag(&mut output).unwrap();
    html::closing_tags(&mut output).unwrap();
    html::wrap_line(1, "code", None, None);
    let highlighted = [1..=1, 3..=5];
    html::line_is_highlighted(&highlighted, 1);
    html::highlight_line_class(&highlighted, 1, None, Some("l-highlighted"));

    let events: [HighlightEvent<'_>; 3] = [
        HighlightEvent::Start {
            scope_index: keyword_index(),
            language: "rust".to_string(),
        },
        HighlightEvent::Source { start: 0, end: 3 },
        HighlightEvent::End,
    ];
    html::render_lines_from_events("a\nb", &events, |scope_index, _language| {
        html::span_linked_attrs(HIGHLIGHT_NAMES[scope_index])
    });

    ansi::hex_to_rgb("#ff79c6");
    ansi::rgb_to_ansi(255, 121, 198, false);
    ansi::style_to_ansi(style);
    ansi::paint("fn", style);
    let _ = ansi::ANSI_RESET;

    [
        (
            "html",
            BTreeSet::from([
                "escape",
                "escape_attr",
                "escape_braces",
                "scope_to_class",
                "themes::Style::css",
                "text_decoration",
                "sanitize_theme_name",
                "open_span",
                "span_inline_attrs",
                "span_inline",
                "span_linked_attrs",
                "span_linked",
                "span_multi_themes_attrs",
                "span_multi_themes",
                "open_pre_tag",
                "open_multi_themes_pre_tag",
                "open_code_tag",
                "close_pre_tag",
                "close_code_tag",
                "closing_tags",
                "wrap_line",
                "line_is_highlighted",
                "highlight_line_class",
                "render_lines_from_events",
            ]),
        ),
        (
            "ansi",
            BTreeSet::from([
                "hex_to_rgb",
                "rgb_to_ansi",
                "style_to_ansi",
                "paint",
                "ANSI_RESET",
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
    let exercised = exercised_helpers();
    assert_eq!(exercised.len(), 2, "both helper modules are covered");
}

#[test]
fn manifest_matches_the_helpers_exercised_here() {
    let manifest = manifest();
    let exercised = exercised_helpers();

    assert_eq!(
        manifest.modules.keys().collect::<Vec<_>>(),
        exercised.keys().collect::<Vec<_>>(),
        "manifest and Rust disagree about which helper modules exist"
    );

    for (module, entry) in &manifest.modules {
        let expected: BTreeSet<&str> = entry.helpers.iter().map(HelperEntry::rust_name).collect();
        assert_eq!(
            &expected,
            &exercised[module.as_str()],
            "{module}: manifest helpers and the Rust functions exercised here disagree"
        );
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

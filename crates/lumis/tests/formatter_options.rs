//! Rust's half of the cross-runtime formatter option check.
//!
//! `fixtures/formatter-options.json` lists the options every covered runtime must
//! accept. Rust cannot reflect on builder setters at runtime, so the check has
//! three parts:
//!
//! - `every_manifest_option_has_a_builder_setter` calls every setter by name.
//!   An option added to the manifest that Rust lacks fails to **compile**.
//! - `manifest_matches_the_setters_exercised_here` reads the manifest and
//!   requires it to name exactly the options exercised above.
//! - `manifest_matches_formatter_fields` parses the formatter structs and
//!   catches a new builder field even if nobody updates either manual list.
//!
//! Together they pin the manifest and the builders to each other in both
//! directions.
//!
//! Gated on `lang-rust` because the builders take a `Language`, and the catalog
//! is feature-gated. The option surface does not vary per language, so one
//! compiled-in language is enough to exercise every setter.
#![cfg(feature = "lang-rust")]

use lumis::formatters::html_inline::{HighlightLines, HighlightLinesStyle};
use lumis::formatters::HtmlElement;
use lumis::{
    languages::Language, themes, BBCodeScopedBuilder, HtmlInlineBuilder, HtmlLinkedBuilder,
    HtmlMultiThemesBuilder, TerminalBackground, TerminalBuilder,
};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::PathBuf;
use syn::{Fields, Item};

#[derive(Debug, Deserialize)]
struct Manifest {
    formatters: BTreeMap<String, FormatterEntry>,
    waived: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct FormatterEntry {
    options: Vec<OptionEntry>,
}

#[derive(Debug, Deserialize)]
struct OptionEntry {
    name: String,
}

fn manifest() -> Manifest {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/formatter-options.json")
        .canonicalize()
        .expect("formatter-options.json is reachable");
    serde_json::from_str(&fs::read_to_string(path).expect("read formatter-options.json"))
        .expect("parse formatter-options.json")
}

fn theme() -> themes::Theme {
    themes::get("dracula").expect("dracula is built in")
}

fn highlight_lines() -> HighlightLines {
    HighlightLines {
        lines: vec![1..=1, 3..=4],
        style: Some(HighlightLinesStyle::Theme),
        class: Some("active".to_string()),
    }
}

fn header() -> HtmlElement {
    HtmlElement {
        open_tag: "<figure>".to_string(),
        close_tag: "</figure>".to_string(),
    }
}

/// Named fields of every struct declared under `src/formatter`, by struct name.
///
/// Discovered by scanning the directory rather than by listing files, so moving
/// a formatter between modules does not also mean editing this test.
fn formatter_struct_fields() -> BTreeMap<String, BTreeSet<String>> {
    let formatter_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/formatter");
    let mut structs = BTreeMap::new();

    let entries = fs::read_dir(&formatter_dir)
        .unwrap_or_else(|error| panic!("read {}: {error}", formatter_dir.display()));

    for entry in entries {
        let path = entry.expect("read formatter dir entry").path();
        if path.extension().is_none_or(|extension| extension != "rs") {
            continue;
        }

        let source = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        let syntax = syn::parse_file(&source)
            .unwrap_or_else(|error| panic!("parse {}: {error}", path.display()));

        for item in &syntax.items {
            let Item::Struct(item) = item else { continue };
            let Fields::Named(fields) = &item.fields else {
                continue;
            };

            structs.insert(
                item.ident.to_string(),
                fields
                    .named
                    .iter()
                    .map(|field| field.ident.as_ref().expect("named field").to_string())
                    .collect(),
            );
        }
    }

    assert!(
        structs.len() > 5,
        "formatter directory scan found almost nothing: {} structs",
        structs.len()
    );

    structs
}

/// The struct behind each manifest formatter. `BBCodeScoped` does not
/// `snake_case` to `bbcode_scoped`, so the pairing is written out.
fn formatter_fields() -> BTreeMap<&'static str, BTreeSet<String>> {
    let structs = formatter_struct_fields();

    [
        ("html_inline", "HtmlInline"),
        ("html_linked", "HtmlLinked"),
        ("html_multi_themes", "HtmlMultiThemes"),
        ("terminal", "Terminal"),
        ("bbcode_scoped", "BBCodeScoped"),
    ]
    .into_iter()
    .map(|(formatter, struct_name)| {
        let fields = structs
            .get(struct_name)
            .unwrap_or_else(|| panic!("no struct {struct_name} under src/formatter"))
            .clone();

        (formatter, fields)
    })
    .collect()
}

/// Every option in the manifest, named once per formatter. Adding an option to
/// the manifest without adding it here fails
/// `manifest_matches_the_setters_exercised_here`; adding it here without a Rust
/// setter fails to compile.
fn exercised_options() -> BTreeMap<&'static str, BTreeSet<&'static str>> {
    let mut exercised: BTreeMap<&'static str, BTreeSet<&'static str>> = BTreeMap::new();

    HtmlInlineBuilder::new()
        .language(Language::Rust)
        .theme(Some(theme()))
        .pre_class(Some("code".to_string()))
        .italic(true)
        .include_highlights(true)
        .highlight_lines(Some(highlight_lines()))
        .header(Some(header()))
        .build()
        .expect("html_inline builds");
    exercised.insert(
        "html_inline",
        [
            "language",
            "theme",
            "pre_class",
            "italic",
            "include_highlights",
            "highlight_lines",
            "header",
        ]
        .into(),
    );

    HtmlLinkedBuilder::new()
        .language(Language::Rust)
        .pre_class(Some("code".to_string()))
        .highlight_lines(Some(lumis::formatters::html_linked::HighlightLines {
            lines: vec![1..=1, 3..=4],
            class: "active".to_string(),
        }))
        .header(Some(header()))
        .build()
        .expect("html_linked builds");
    exercised.insert(
        "html_linked",
        ["language", "pre_class", "highlight_lines", "header"].into(),
    );

    let mut themes_map = HashMap::new();
    themes_map.insert("light".to_string(), theme());
    HtmlMultiThemesBuilder::new()
        .language(Language::Rust)
        .themes(themes_map)
        .default_theme("light")
        .css_variable_prefix("--lumis")
        .pre_class(Some("code".to_string()))
        .italic(true)
        .include_highlights(true)
        .highlight_lines(Some(highlight_lines()))
        .header(Some(header()))
        .build()
        .expect("html_multi_themes builds");
    exercised.insert(
        "html_multi_themes",
        [
            "language",
            "themes",
            "default_theme",
            "css_variable_prefix",
            "pre_class",
            "italic",
            "include_highlights",
            "highlight_lines",
            "header",
        ]
        .into(),
    );

    TerminalBuilder::new()
        .language(Language::Rust)
        .theme(Some(theme()))
        .background(TerminalBackground::Theme)
        .width(Some(120))
        .build()
        .expect("terminal builds");
    exercised.insert(
        "terminal",
        ["language", "theme", "background", "width"].into(),
    );

    BBCodeScopedBuilder::new()
        .language(Language::Rust)
        .build()
        .expect("bbcode_scoped builds");
    exercised.insert("bbcode_scoped", ["language"].into());

    exercised
}

#[test]
fn every_manifest_option_has_a_builder_setter() {
    // The setters run here; reaching this line means they all exist and build.
    let exercised = exercised_options();
    assert_eq!(exercised.len(), 5, "all five formatters are covered");
}

#[test]
fn manifest_matches_the_setters_exercised_here() {
    let manifest = manifest();
    let exercised = exercised_options();

    assert_eq!(
        manifest.formatters.keys().collect::<Vec<_>>(),
        exercised.keys().collect::<Vec<_>>(),
        "manifest and Rust disagree about which formatters exist"
    );

    for (formatter, entry) in &manifest.formatters {
        let expected: BTreeSet<&str> = entry.options.iter().map(|o| o.name.as_str()).collect();
        let actual = &exercised[formatter.as_str()];

        assert_eq!(
            &expected, actual,
            "{formatter}: manifest options and Rust builder setters disagree"
        );
    }
}

#[test]
fn manifest_matches_formatter_fields() {
    let manifest = manifest();
    let fields = formatter_fields();

    assert_eq!(
        manifest.formatters.keys().collect::<Vec<_>>(),
        fields.keys().collect::<Vec<_>>(),
        "manifest and Rust disagree about which formatters exist"
    );

    for (formatter, entry) in &manifest.formatters {
        let expected: BTreeSet<String> = entry.options.iter().map(|o| o.name.clone()).collect();
        assert_eq!(
            expected,
            fields[formatter.as_str()],
            "{formatter}: manifest options and formatter fields disagree"
        );
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
        "every covered runtime offers every option; drop these waivers: {waivers:?}"
    );
}

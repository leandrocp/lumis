//! Rust's half of the HTML attribute escaping parity check.
//!
//! `fixtures/html-attr-escaping.json` holds one expected tag per case. This
//! asserts Rust produces it;
//! `packages/javascript/lumis/test/html-attr-escaping.test.ts` asserts the
//! TypeScript port produces the same. Rust is the reference, so a difference is
//! a bug in the port rather than something to record.

use lumis_core::formatter::html;
use lumis_core::themes::Theme;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct Manifest {
    themes: HashMap<String, serde_json::Value>,
    cases: Vec<Case>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Case {
    name: String,
    helper: Helper,
    expected: String,
    #[serde(default)]
    text: String,
    #[serde(default)]
    scope: String,
    #[serde(default)]
    theme: Option<String>,
    #[serde(default)]
    include_highlights: bool,
    #[serde(default)]
    themes: HashMap<String, String>,
    #[serde(default)]
    default_theme: Option<String>,
    #[serde(default)]
    pre_class: Option<String>,
    #[serde(default)]
    line_class: Option<String>,
    #[serde(default)]
    line_style: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
enum Helper {
    PreTag,
    SpanInline,
    SpanMultiThemes,
    Line,
}

fn manifest() -> Manifest {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/html-attr-escaping.json");
    serde_json::from_str(&fs::read_to_string(&path).expect("read html-attr-escaping.json"))
        .expect("parse html-attr-escaping.json")
}

fn theme(manifest: &Manifest, key: &str) -> Theme {
    let source = manifest
        .themes
        .get(key)
        .unwrap_or_else(|| panic!("`{key}` is not a fixture theme"));
    lumis_core::themes::from_json(&source.to_string()).expect("fixture themes are well-formed")
}

fn render(manifest: &Manifest, case: &Case) -> String {
    match case.helper {
        Helper::PreTag => {
            let mut output = Vec::new();
            html::open_pre_tag(
                &mut output,
                case.pre_class.as_deref(),
                case.theme.as_ref().map(|key| theme(manifest, key)).as_ref(),
            )
            .expect("write to a Vec cannot fail");
            String::from_utf8(output).expect("valid UTF-8")
        }
        Helper::SpanInline => html::span_inline(
            &case.text,
            None,
            &case.scope,
            case.theme.as_ref().map(|key| theme(manifest, key)).as_ref(),
            false,
            case.include_highlights,
        ),
        Helper::SpanMultiThemes => {
            let themes: HashMap<String, Theme> = case
                .themes
                .iter()
                .map(|(name, key)| (name.clone(), theme(manifest, key)))
                .collect();

            html::span_multi_themes(
                &case.text,
                &case.scope,
                None,
                &themes,
                case.default_theme.as_deref(),
                "--lumis",
                false,
                case.include_highlights,
            )
        }
        // The tag only: Rust takes the trailing newline in `content` and the
        // TypeScript port appends it, so the two agree up to the `>`.
        Helper::Line => html::wrap_line(
            1,
            "",
            case.line_class
                .as_ref()
                .map(|class| format!(" {class}"))
                .as_deref(),
            case.line_style.as_deref(),
        ),
    }
}

#[test]
fn covers_both_vectors_that_reach_an_attribute() {
    let manifest = manifest();
    let names: Vec<&str> = manifest
        .cases
        .iter()
        .map(|case| case.name.as_str())
        .collect();

    for required in [
        "pre/caller-class-closes-the-attribute",
        "pre/theme-colour-closes-the-attribute",
        "span/theme-colour-closes-the-attribute",
        "multi-themes/theme-colour-closes-the-attribute",
        "line/caller-class-closes-the-attribute",
        "line/caller-style-closes-the-attribute",
    ] {
        assert!(
            names.contains(&required),
            "the corpus lost its `{required}` case"
        );
    }
}

#[test]
fn escapes_every_attribute_value() {
    let manifest = manifest();

    for case in &manifest.cases {
        let rendered = render(&manifest, case);

        match case.helper {
            Helper::Line => assert!(
                rendered.starts_with(&case.expected),
                "{}: expected the tag {:?}, got {rendered:?}",
                case.name,
                case.expected
            ),
            _ => assert_eq!(rendered, case.expected, "{}", case.name),
        }
    }
}

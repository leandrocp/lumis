//! Rust's half of the ANSI helper parity check.
//!
//! `fixtures/ansi-parity.json` holds one expected value per case. This asserts
//! Rust produces it; `packages/javascript/lumis/test/ansi-parity.test.ts`
//! asserts the TypeScript port produces the same. Rust is the reference, so a
//! difference is a bug in the port rather than something to record.

use lumis_core::formatter::ansi;
use lumis_core::themes::Style;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct Manifest {
    cases: Vec<Case>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Case {
    name: String,
    helper: Helper,
    expected: Expected,
    #[serde(default)]
    hex: String,
    #[serde(default)]
    text: String,
    /// The JSON shape a theme already carries, so neither side reshapes it.
    #[serde(default)]
    style: Style,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
enum Helper {
    HexToRgb,
    StyleToAnsi,
    Paint,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Expected {
    /// `hexToRgb`, where `null` means the string names no color.
    Rgb(Option<[u8; 3]>),
    /// `styleToAnsi` and `paint`, which both answer a string.
    Text(String),
}

fn manifest() -> Manifest {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/ansi-parity.json");
    serde_json::from_str(&fs::read_to_string(&path).expect("read ansi-parity.json"))
        .expect("parse ansi-parity.json")
}

fn expected_rgb(case: &Case) -> Option<(u8, u8, u8)> {
    match &case.expected {
        Expected::Rgb(rgb) => rgb.map(|[r, g, b]| (r, g, b)),
        Expected::Text(text) => panic!("{}: expected an RGB triple, not {text:?}", case.name),
    }
}

fn expected_text(case: &Case) -> &str {
    match &case.expected {
        Expected::Text(text) => text,
        Expected::Rgb(rgb) => panic!("{}: expected a string, not {rgb:?}", case.name),
    }
}

#[test]
fn covers_the_input_a_valid_prefix_used_to_slip_through() {
    let manifest = manifest();
    let names: Vec<&str> = manifest
        .cases
        .iter()
        .map(|case| case.name.as_str())
        .collect();

    for required in [
        "hex/trailing-non-hex-digit",
        "hex/last-component-half-valid",
        "hex/trailing-space",
        "hex/leading-space",
        "hex/signed-component",
        "hex/non-ascii-in-the-last-component",
        "hex/repeated-hash-is-trimmed",
        "paint/an-empty-background-still-takes-the-per-line-branch",
    ] {
        assert!(
            names.contains(&required),
            "the corpus lost its `{required}` case"
        );
    }
}

#[test]
fn answers_what_the_port_has_to_answer() {
    let manifest = manifest();

    for case in &manifest.cases {
        match case.helper {
            Helper::HexToRgb => assert_eq!(
                ansi::hex_to_rgb(&case.hex),
                expected_rgb(case),
                "{}",
                case.name
            ),
            Helper::StyleToAnsi => assert_eq!(
                ansi::style_to_ansi(&case.style),
                expected_text(case),
                "{}",
                case.name
            ),
            Helper::Paint => assert_eq!(
                ansi::paint(&case.text, &case.style),
                expected_text(case),
                "{}",
                case.name
            ),
        }
    }
}

//! The CLI's half of the cross-runtime formatter option check.
//!
//! `fixtures/formatter-options.json` lists the options every covered runtime must
//! accept. Visible flags are checked through `lumis highlight --help`; hidden
//! compatibility flags are exercised directly.
//!
//! Options are `snake_case` in the manifest and kebab-case behind `--` here.
//! An option that needs more than one flag records that in `spelling.cli`.

use assert_cmd::cargo::cargo_bin_cmd;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

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
    #[serde(default)]
    spelling: Option<Spelling>,
}

#[derive(Debug, Deserialize)]
struct Spelling {
    #[serde(default)]
    cli: Option<Vec<String>>,
}

fn manifest() -> Manifest {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/formatter-options.json")
        .canonicalize()
        .expect("formatter-options.json is reachable");
    serde_json::from_str(&fs::read_to_string(path).expect("read formatter-options.json"))
        .expect("parse formatter-options.json")
}

fn highlight_help() -> String {
    let output = cargo_bin_cmd!("lumis")
        .args(["highlight", "--help"])
        .output()
        .expect("lumis highlight --help runs");
    assert!(output.status.success(), "lumis highlight --help failed");
    String::from_utf8(output.stdout).expect("help output is UTF-8")
}

/// The flags an option is expected to appear as.
fn expected_flags(option: &OptionEntry) -> Vec<String> {
    if let Some(cli) = option.spelling.as_ref().and_then(|s| s.cli.as_ref()) {
        return cli.iter().map(|flag| format!("--{flag}")).collect();
    }

    vec![format!("--{}", option.name.replace('_', "-"))]
}

#[test]
fn highlight_accepts_every_option_in_the_manifest() {
    let manifest = manifest();
    let help = highlight_help();
    let mut missing: BTreeSet<String> = BTreeSet::new();

    for entry in manifest.formatters.values() {
        for option in &entry.options {
            for flag in expected_flags(option) {
                // `themes` is `--themes`, and `--theme` is a prefix of it, so
                // match on a word boundary rather than a bare substring.
                let present = help
                    .split_whitespace()
                    .any(|token| token.trim_end_matches(&[',', '<'][..]) == flag);
                if flag == "--italic" {
                    assert!(!present, "deprecated --italic must stay hidden from help");
                    cargo_bin_cmd!("lumis")
                        .args(["highlight", &flag, "--help"])
                        .assert()
                        .success();
                } else if !present {
                    missing.insert(flag);
                }
            }
        }
    }

    assert!(
        missing.is_empty(),
        "lumis highlight is missing these manifest options: {missing:?}"
    );
}

#[test]
fn no_waiver_outlives_its_reason() {
    let waivers: Vec<String> = manifest()
        .waived
        .keys()
        .filter(|key| !key.starts_with('$'))
        .cloned()
        .collect();

    assert!(
        waivers.is_empty(),
        "every covered runtime offers every option; drop these waivers: {waivers:?}"
    );
}

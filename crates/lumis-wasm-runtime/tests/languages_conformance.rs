//! The Rust half of the shared language-name conformance suite.
//!
//! `fixtures/conformance-languages/cases.json` states what loading and the
//! CLI's download must agree on in every runtime. This asserts the Rust
//! implementation against it;
//! `packages/javascript/lumis/test/languages-conformance.test.ts` and
//! `packages/elixir/lumis/test/languages_conformance_test.exs` assert theirs
//! against the same file.

use std::path::PathBuf;

use lumis_wasm_runtime::catalog;
use serde_json::Value;

fn cases() -> Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/conformance-languages/cases.json");
    let bytes = std::fs::read(&path).unwrap_or_else(|error| {
        panic!("could not read {}: {error}", path.display());
    });
    serde_json::from_slice(&bytes).expect("cases.json is not valid JSON")
}

fn strings(value: &Value) -> Vec<String> {
    value
        .as_array()
        .expect("expected an array")
        .iter()
        .map(|entry| entry.as_str().expect("expected a string").to_string())
        .collect()
}

fn expand(names: &[String]) -> Result<Vec<String>, String> {
    catalog::expand_bundles(names.iter().map(String::as_str))
        .map_err(|unknown| unknown.name().to_string())
}

#[test]
fn every_spelling_of_a_bundle_expands_the_same_way() {
    let cases = cases();
    let groups = cases["spellings"]["groups"]
        .as_array()
        .expect("spellings.groups");
    assert!(groups.len() >= 5, "the catalog has five bundles to cover");

    for group in groups {
        let spellings = strings(group);
        let expected = expand(&spellings[..1]).expect("the first spelling names a bundle");
        assert!(!expected.is_empty(), "{} expanded to nothing", spellings[0]);

        for spelling in &spellings[1..] {
            let actual = expand(std::slice::from_ref(spelling)).unwrap_or_else(|unknown| {
                panic!("{spelling} was rejected as unknown bundle {unknown}");
            });
            assert_eq!(
                actual, expected,
                "{spelling} disagrees with {}",
                spellings[0]
            );
        }
    }
}

#[test]
fn a_name_that_is_not_a_bundle_survives_expansion() {
    let cases = cases();

    for name in strings(&cases["passthrough"]["names"]) {
        let expanded =
            expand(std::slice::from_ref(&name)).expect("a plain name is never an unknown bundle");
        assert_eq!(expanded, vec![name]);
    }
}

#[test]
fn a_bundle_member_named_twice_is_kept_once() {
    let cases = cases();
    let bundle = cases["deduplication"]["bundle"]
        .as_str()
        .expect("deduplication.bundle")
        .to_string();
    let member = cases["deduplication"]["alsoNamed"]
        .as_str()
        .expect("deduplication.alsoNamed")
        .to_string();

    let alone = expand(std::slice::from_ref(&bundle)).expect("a bundle");
    assert!(alone.contains(&member), "{bundle} should contain {member}");

    let with_repeat = expand(&[bundle, member.clone(), member.clone()]).expect("a bundle");

    assert_eq!(
        with_repeat.iter().filter(|name| **name == member).count(),
        1
    );
    assert_eq!(with_repeat.len(), alone.len());
}

#[test]
fn expansion_keeps_the_order_it_was_given() {
    let cases = cases();
    let input = strings(&cases["ordering"]["input"]);
    let first = cases["ordering"]["firstIs"].as_str().expect("firstIs");
    let last = cases["ordering"]["lastIs"].as_str().expect("lastIs");

    let expanded = expand(&input).expect("a bundle and two languages");

    assert_eq!(expanded.first().map(String::as_str), Some(first));
    assert_eq!(expanded.last().map(String::as_str), Some(last));
}

#[test]
fn an_unknown_bundle_is_an_error_rather_than_a_language() {
    let cases = cases();

    for name in strings(&cases["unknownBundles"]["names"]) {
        let rejected = expand(std::slice::from_ref(&name));
        assert_eq!(
            rejected,
            Err(name.clone()),
            "{name} should be rejected as an unknown bundle, naming itself"
        );
    }
}

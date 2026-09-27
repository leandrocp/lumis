use assert_cmd::cargo::cargo_bin_cmd;
use serde_json::{json, Value};
use std::collections::BTreeSet;

fn manifest() -> Value {
    serde_json::from_str(include_str!("../../../fixtures/theme-css-options.json")).unwrap()
}

#[test]
fn manifest_matches_build_css_flags() {
    let help = cargo_bin_cmd!("lumis")
        .args(["themes", "build-css", "--help"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let help = String::from_utf8(help).unwrap();
    let actual: BTreeSet<_> = help
        .lines()
        .filter_map(|line| {
            let line = line.trim_start();
            if line.starts_with('-') {
                line.split_whitespace().find(|word| word.starts_with("--"))
            } else {
                None
            }
        })
        .filter(|flag| {
            !["--help", "--version", "--verbose", "--data-dir", "--config"].contains(flag)
        })
        .collect();
    let manifest = manifest();
    let expected: BTreeSet<_> = manifest["options"]
        .as_array()
        .unwrap()
        .iter()
        .map(|option| format!("--{}", option["cli"].as_str().unwrap()))
        .collect();
    assert_eq!(actual, expected.iter().map(String::as_str).collect());
}

#[test]
fn shared_css_cases() {
    let manifest = manifest();
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir(dir.path().join("themes")).unwrap();
    std::fs::write(
        dir.path().join("themes/css-contract.json"),
        manifest["theme"].to_string(),
    )
    .unwrap();
    let cases = manifest["cases"].as_array().unwrap();
    assert!(cases.len() >= 6);
    for case in cases {
        let mut cmd = cargo_bin_cmd!("lumis");
        cmd.arg("--data-dir")
            .arg(dir.path())
            .args(["themes", "build-css", "css-contract"]);
        for (name, value) in case["options"].as_object().unwrap() {
            let option = manifest["options"]
                .as_array()
                .unwrap()
                .iter()
                .find(|option| option["name"] == *name)
                .unwrap();
            let flag = format!("--{}", option["cli"].as_str().unwrap());
            match value {
                Value::Bool(false) => {
                    cmd.arg(flag);
                }
                Value::Bool(true) => {}
                Value::String(value) => {
                    cmd.arg(flag).arg(value);
                }
                Value::Array(pairs) => {
                    for pair in pairs {
                        cmd.arg(&flag).arg(format!(
                            "{}={}",
                            pair[0].as_str().unwrap(),
                            pair[1].as_str().unwrap()
                        ));
                    }
                }
                _ => panic!("unhandled CSS option: {name}"),
            }
        }
        cmd.assert()
            .success()
            .stdout(case["css"].as_str().unwrap().to_string())
            .stderr("");
    }
}

#[test]
fn built_in_theme_and_errors() {
    let dir = tempfile::tempdir().unwrap();
    let theme = lumis_core::themes::get("github_light").unwrap();
    cargo_bin_cmd!("lumis")
        .arg("--data-dir")
        .arg(dir.path())
        .args(["themes", "build-css", "github_light"])
        .assert()
        .success()
        .stdout(lumis_core::themes::CssBuilder::new(&theme).build())
        .stderr("");
    cargo_bin_cmd!("lumis")
        .arg("--data-dir")
        .arg(dir.path())
        .args(["themes", "build-css", "missing-theme"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicates::str::contains("unknown theme: missing-theme"));
    for invalid in ["padding", "=red", "padding="] {
        cargo_bin_cmd!("lumis")
            .args([
                "themes",
                "build-css",
                "github_light",
                "--container-style",
                invalid,
            ])
            .assert()
            .failure()
            .stdout("")
            .stderr(predicates::str::contains("property=value"));
    }
    let defaults: serde_json::Map<String, Value> = manifest()["options"]
        .as_array()
        .unwrap()
        .iter()
        .map(|option| {
            (
                option["name"].as_str().unwrap().to_string(),
                option["default"].clone(),
            )
        })
        .collect();
    assert_eq!(
        Value::Object(defaults),
        json!({"layout":true,"enable_italic":true,"scope":"","container_selector":".lumis","container_style":[]})
    );
}

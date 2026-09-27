use assert_cmd::cargo::cargo_bin_cmd;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

mod common;

fn cmd() -> assert_cmd::Command {
    let mut command = cargo_bin_cmd!("lumis");
    command.env(
        "LUMIS_CONFIG",
        common::source_fixtures_dir().join("missing-config.toml"),
    );
    command.env("LUMIS_DATA_DIR", common::data_dir());
    command
}

fn fixtures_dir() -> PathBuf {
    common::data_dir()
}

fn conformance_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("fixtures")
        .join("conformance")
}

#[derive(Debug, Deserialize)]
struct FixtureMetadata {
    language: String,
    theme: String,
    #[serde(default, rename = "rainbowBrackets")]
    rainbow_brackets: bool,
    /// `--structure` for the three HTML formatters; absent means a block.
    #[serde(default)]
    structure: Option<String>,
    #[serde(default, rename = "htmlMultiThemes")]
    html_multi_themes: Option<HtmlMultiThemesFixture>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HtmlMultiThemesFixture {
    themes: BTreeMap<String, String>,
    #[serde(default)]
    default_theme: Option<String>,
    #[serde(default)]
    highlight_lines: Vec<usize>,
}

struct Fixture {
    metadata: FixtureMetadata,
    source: String,
    html_inline: String,
    html_linked: String,
    html_multi_themes: String,
    terminal: String,
    bbcode: String,
}

fn load_fixture(name: &str) -> Fixture {
    let dir = conformance_dir().join(name);
    let metadata = serde_json::from_str::<FixtureMetadata>(
        &fs::read_to_string(dir.join("fixture.json")).expect("failed to read fixture metadata"),
    )
    .expect("failed to parse fixture metadata");

    Fixture {
        metadata,
        source: fs::read_to_string(dir.join("source.txt")).expect("failed to read source"),
        html_inline: fs::read_to_string(dir.join("html-inline.html"))
            .expect("failed to read html-inline"),
        html_linked: fs::read_to_string(dir.join("html-linked.html"))
            .expect("failed to read html-linked"),
        html_multi_themes: fs::read_to_string(dir.join("html-multi-themes.html"))
            .expect("failed to read html-multi-themes"),
        terminal: fs::read_to_string(dir.join("terminal.txt")).expect("failed to read terminal"),
        bbcode: fs::read_to_string(dir.join("bbcode.txt")).expect("failed to read bbcode"),
    }
}

fn run_highlight_source(fixture: &Fixture, formatter: &str, extra_args: &[&str]) -> String {
    let mut command = cmd();
    command
        .arg("--data-dir")
        .arg(fixtures_dir())
        .arg("highlight")
        .arg("-l")
        .arg(&fixture.metadata.language)
        .arg("-f")
        .arg(formatter)
        .args(extra_args);

    if fixture.metadata.rainbow_brackets {
        command.arg("--rainbow-brackets");
    }

    let output = command
        .write_stdin(fixture.source.as_str())
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn run_html(fixture: &Fixture, formatter: &str, extra_args: &[&str]) -> String {
    let mut args = extra_args.to_vec();
    if let Some(structure) = &fixture.metadata.structure {
        args.extend(["--structure", structure]);
    }

    run_highlight_source(fixture, formatter, &args)
}

fn assert_text_eq(output: &str, expected: &str) {
    assert_eq!(output.replace("\r\n", "\n"), expected.replace("\r\n", "\n"));
}

fn check_html_inline(fixture: &Fixture) {
    let output = run_html(fixture, "html-inline", &["-t", &fixture.metadata.theme]);
    assert_text_eq(&output, &fixture.html_inline);
}

fn check_html_linked(fixture: &Fixture) {
    let output = run_html(fixture, "html-linked", &[]);
    assert_text_eq(&output, &fixture.html_linked);
}

fn check_html_multi_themes(fixture: &Fixture) {
    let mut args = Vec::new();

    if let Some(config) = &fixture.metadata.html_multi_themes {
        // Reversed on purpose: the formatter sorts theme names itself, so
        // output must not depend on the order they were given in.
        for (name, theme) in config.themes.iter().rev() {
            args.push("--themes".to_string());
            args.push(format!("{name}:{theme}"));
        }
        if let Some(default_theme) = &config.default_theme {
            args.push("--default-theme".to_string());
            args.push(default_theme.clone());
        }

        if !config.highlight_lines.is_empty() {
            args.push("--highlight-lines".to_string());
            args.push(
                config
                    .highlight_lines
                    .iter()
                    .map(usize::to_string)
                    .collect::<Vec<_>>()
                    .join(","),
            );
        }
    } else {
        args.extend([
            "--themes".to_string(),
            format!("main:{}", fixture.metadata.theme),
            "--default-theme".to_string(),
            "main".to_string(),
        ]);
    }

    let arg_refs = args.iter().map(String::as_str).collect::<Vec<_>>();
    let output = run_html(fixture, "html-multi-themes", &arg_refs);
    assert_text_eq(&output, &fixture.html_multi_themes);
}

fn check_terminal(fixture: &Fixture) {
    let output = run_highlight_source(fixture, "terminal", &["-t", &fixture.metadata.theme]);
    assert_text_eq(&output, &fixture.terminal);
}

fn check_bbcode(fixture: &Fixture) {
    let output = run_highlight_source(fixture, "bbcode-scoped", &[]);
    assert_text_eq(&output, &fixture.bbcode);
}

include!(concat!(env!("OUT_DIR"), "/conformance_tests.rs"));

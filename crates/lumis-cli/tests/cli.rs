use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

mod common;

fn cmd() -> assert_cmd::Command {
    let mut command = cargo_bin_cmd!("lumis");
    command.env(
        "LUMIS_CONFIG",
        source_fixtures_dir().join("missing-config.toml"),
    );
    command.env("LUMIS_DATA_DIR", common::data_dir());
    command
}

fn fixtures_dir() -> PathBuf {
    common::language_fixtures_dir()
}

fn source_fixtures_dir() -> PathBuf {
    common::source_fixtures_dir()
}

fn write_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, content).unwrap();
}

#[cfg(unix)]
fn install_fake_nvim(bin_dir: &Path) {
    let script = r##"#!/usr/bin/env bash
set -euo pipefail

capture_dir="${LUMIS_FAKE_NVIM_CAPTURE_DIR:?missing capture dir}"
appearance="${LUMIS_FAKE_NVIM_APPEARANCE:-dark}"
colorscheme="${@: -1}"

cp init.lua "$capture_dir/init.lua"
cp themes.lua "$capture_dir/themes.lua"
cp extract_theme.lua "$capture_dir/extract_theme.lua"
printf '%s\n' "$@" > "$capture_dir/argv.txt"

cat > "$colorscheme.json" <<EOF
{"name":"$colorscheme","appearance":"$appearance","revision":"fake-revision","highlights":{"normal":{"fg":"#ffffff","bg":"#000000"}}}
EOF
"##;

    let path = bin_dir.join("nvim");
    write_file(&path, script);

    #[allow(clippy::useless_conversion)]
    {
        use std::os::unix::fs::PermissionsExt;

        let mut perms = fs::metadata(&path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&path, perms).unwrap();
    }
}

#[cfg(windows)]
fn install_fake_nvim(bin_dir: &Path) {
    let source = r##"
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let capture_dir = PathBuf::from(env::var("LUMIS_FAKE_NVIM_CAPTURE_DIR").expect("missing capture dir"));
    let appearance = env::var("LUMIS_FAKE_NVIM_APPEARANCE").unwrap_or_else(|_| "dark".to_string());
    let args: Vec<String> = env::args().skip(1).collect();
    let colorscheme = args.last().expect("missing colorscheme arg");

    fs::copy("init.lua", capture_dir.join("init.lua")).unwrap();
    fs::copy("themes.lua", capture_dir.join("themes.lua")).unwrap();
    fs::copy("extract_theme.lua", capture_dir.join("extract_theme.lua")).unwrap();
    fs::write(capture_dir.join("argv.txt"), args.join("\n") + "\n").unwrap();

    let json = format!(
        "{{\"name\":\"{}\",\"appearance\":\"{}\",\"revision\":\"fake-revision\",\"highlights\":{{\"normal\":{{\"fg\":\"#ffffff\",\"bg\":\"#000000\"}}}}}}",
        colorscheme, appearance
    );
    fs::write(format!("{}.json", colorscheme), json).unwrap();
}
"##;

    let source_path = bin_dir.join("fake_nvim.rs");
    let exe_path = bin_dir.join("nvim.exe");

    write_file(&source_path, source);

    let status = std::process::Command::new("rustc")
        .arg(&source_path)
        .arg("-O")
        .arg("-o")
        .arg(&exe_path)
        .status()
        .unwrap();

    assert!(status.success(), "failed to build fake nvim.exe");
}

fn fake_nvim_path(bin_dir: &Path) -> OsString {
    let current_path = std::env::var_os("PATH").unwrap_or_default();
    let mut paths = vec![bin_dir.to_path_buf()];
    paths.extend(std::env::split_paths(&current_path));
    std::env::join_paths(paths).unwrap()
}

#[test]
fn version_flag() {
    cmd()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn short_version_flag() {
    cmd()
        .arg("-V")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn help_flag() {
    cmd()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("highlight"));
}

#[test]
fn list_languages() {
    cmd()
        .args(["languages", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rust"))
        .stdout(predicate::str::contains("javascript"))
        .stdout(predicate::str::contains("elixir"));
}

#[test]
fn list_themes() {
    cmd()
        .args(["themes", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("dracula"));
}

#[test]
fn list_themes_uses_data_dir_from_env() {
    let tmp = tempfile::tempdir().unwrap();
    write_file(
        &tmp.path().join("themes/custom.json"),
        r#"{"name":"custom","appearance":"dark","revision":"test","highlights":{}}"#,
    );

    cmd()
        .env("LUMIS_DATA_DIR", tmp.path())
        .args(["themes", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("custom (file)"));
}

/// The format was unpinned, so nothing would have noticed it changing. One
/// language and one theme are enough to hold the shape of each.
#[test]
fn list_languages_prints_an_id_then_its_globs() {
    let output = cmd().args(["languages", "list"]).assert().success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();

    let elixir = stdout
        .lines()
        .position(|line| line == "elixir")
        .expect("elixir is listed on a line of its own");
    assert_eq!(stdout.lines().nth(elixir + 1).unwrap(), "  *.ex  *.exs");
}

#[test]
fn list_themes_prints_one_sorted_name_per_line() {
    let output = cmd().args(["themes", "list"]).assert().success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    let names: Vec<&str> = stdout.lines().collect();

    assert!(
        names.len() > 200,
        "expected the full corpus, got {}",
        names.len()
    );
    assert!(names.contains(&"dracula"));

    let mut sorted = names.clone();
    sorted.sort_unstable();
    assert_eq!(names, sorted);
}

#[test]
fn languages_show_prints_what_the_catalog_knows() {
    cmd()
        .args(["languages", "show", "elixir"])
        .assert()
        .success()
        .stdout(predicate::str::contains("elixir: Elixir"))
        .stdout(predicate::str::contains("extensions: *.ex, *.exs"))
        .stdout(predicate::str::contains("emacs modes: elixir"));
}

#[test]
fn languages_show_resolves_an_alias() {
    cmd()
        .args(["languages", "show", "js"])
        .assert()
        .success()
        .stdout(predicate::str::contains("javascript: JavaScript"))
        .stdout(predicate::str::contains("aliases: js, jsx"));
}

#[test]
fn languages_show_rejects_an_unknown_language() {
    cmd()
        .args(["languages", "show", "not-a-language"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("unknown language: not-a-language"));
}

#[test]
fn themes_show_prints_appearance_and_colors() {
    cmd()
        .args(["themes", "show", "dracula"])
        .assert()
        .success()
        .stdout(predicate::str::contains("dracula: dark"))
        .stdout(predicate::str::contains("background: #282a36"));
}

#[test]
fn themes_show_finds_a_theme_from_the_data_dir() {
    let tmp = tempfile::tempdir().unwrap();
    write_file(
        &tmp.path().join("themes/custom.json"),
        r#"{"name":"custom","appearance":"dark","revision":"test","highlights":{}}"#,
    );

    cmd()
        .env("LUMIS_DATA_DIR", tmp.path())
        .args(["themes", "show", "custom"])
        .assert()
        .success()
        .stdout(predicate::str::contains("custom: dark"));
}

#[test]
fn themes_show_rejects_an_unknown_theme() {
    cmd()
        .args(["themes", "show", "not-a-theme"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("unknown theme: not-a-theme"));
}

#[test]
fn highlight_nonexistent_file() {
    cmd()
        .args(["highlight", "nonexistent.rs"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("No such file"));
}

#[test]
fn dump_tree_from_stdin() {
    cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args(["dump", "tree", "-l", "javascript"])
        .write_stdin("const answer = 42;\n")
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            "[program] language: javascript, range: 0:0-1:0",
        ))
        .stdout(predicate::str::contains("[lexical_declaration]"))
        .stdout(predicate::str::contains("text:").not());
}

#[test]
fn dump_tree_autodetects_language_from_path() {
    let tmp = tempfile::tempdir().unwrap();
    let source_path = tmp.path().join("example.js");
    write_file(&source_path, "const answer = 42;\n");

    cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .arg("dump")
        .arg("tree")
        .arg(source_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "[program] language: javascript, range: 0:0-1:0",
        ));
}

#[test]
fn dump_tree_supports_canonical_sexp_format() {
    cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args(["dump", "tree", "--format", "sexp", "-l", "javascript"])
        .write_stdin("const answer = 42;\n")
        .assert()
        .success()
        .stdout(predicate::str::starts_with("(program\n"))
        .stdout(predicate::str::contains("(lexical_declaration"));
}

#[test]
fn dump_tree_text_uses_bounded_previews_by_default() {
    let long_text = "a".repeat(100);
    let source = format!("// {long_text}\n");
    let output = cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args(["dump", "tree", "--text", "-l", "javascript"])
        .write_stdin(source)
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("..."));
    assert!(!stdout.contains(&long_text));
}

#[test]
fn dump_tree_text_accepts_custom_limit_and_full_value() {
    let source = "// abcdefghijklmnopqrstuvwxyz\n";

    cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args(["dump", "tree", "--text=10", "-l", "javascript"])
        .write_stdin(source)
        .assert()
        .success()
        .stdout(predicate::str::contains("text: \"// a...yz\\n\""));

    cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args(["dump", "tree", "--text=full", "-l", "javascript"])
        .write_stdin(source)
        .assert()
        .success()
        .stdout(predicate::str::contains(format!("text: {source:?}")));
}

#[test]
fn dump_tree_text_does_not_consume_the_path() {
    let tmp = tempfile::tempdir().unwrap();
    let source_path = tmp.path().join("example.js");
    write_file(&source_path, "const answer = 42;\n");

    cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args(["dump", "tree", "--text"])
        .arg(source_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("text: \"const answer = 42;\\n\""));
}

#[test]
fn dump_tree_sexp_format_labels_injected_trees() {
    cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args([
            "dump",
            "tree",
            "--format",
            "sexp",
            "--injections",
            "-l",
            "markdown",
        ])
        .write_stdin("```javascript\nconst answer = 42;\n```\n")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "language: markdown, depth: 0, range: 0:0-3:0",
        ))
        .stdout(predicate::str::contains("(document\n"))
        .stdout(predicate::str::contains(
            "language: javascript, depth: 1, range: 1:0-2:0",
        ))
        .stdout(predicate::str::contains("(program\n"));
}

#[test]
fn dump_tree_injections_and_injected_highlights_are_opt_in() {
    let source = "```javascript\nconst answer = 42;\n```\n";

    cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args(["dump", "tree", "--highlights", "-l", "markdown"])
        .write_stdin(source)
        .assert()
        .success()
        .stdout(predicate::str::contains("language: javascript").not());

    cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args([
            "dump",
            "tree",
            "--injections",
            "--highlights",
            "-l",
            "markdown",
        ])
        .write_stdin(source)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "[program] language: javascript, range: 1:0-2:0",
        ))
        .stdout(predicate::str::contains("@keyword language: javascript"));
}

#[test]
fn dump_tree_rejects_annotations_in_sexp_format() {
    cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args([
            "dump",
            "tree",
            "--format",
            "sexp",
            "--text",
            "-l",
            "javascript",
        ])
        .write_stdin("const answer = 42;\n")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "--format sexp cannot be combined with --text or --highlights",
        ));
}

#[test]
fn dump_events_as_json() {
    cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args(["dump", "events", "-l", "javascript"])
        .write_stdin("const answer = 42;\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"type\": \"start\""))
        .stdout(predicate::str::contains("\"scope\": \"keyword\""))
        .stdout(predicate::str::contains("\"type\": \"source\""));
}

#[test]
fn dump_tree_prints_exact_highlight_ranges_and_text() {
    cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args(["dump", "tree", "--text", "--highlights", "-l", "javascript"])
        .write_stdin("const answer = 42;\n")
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            "[program] language: javascript, range: 0:0-1:0",
        ))
        .stdout(predicate::str::contains("[lexical_declaration]"))
        .stdout(predicate::str::contains(
            "@keyword language: javascript, range: 0:0-0:5, text: \"const\"",
        ))
        .stdout(predicate::str::contains(
            "@punctuation.delimiter language: javascript, range: 0:17-0:18, text: \";\"",
        ))
        .stdout(predicate::str::contains(
            "[identifier] field: name, language: javascript, range: 0:6-0:12, text: \"answer\"",
        ))
        .stdout(predicate::str::contains(
            "@variable language: javascript, range: 0:6-0:12, text: \"answer\"",
        ));
}

#[test]
fn dump_tree_interleaves_highlights_and_children_in_source_order() {
    let output = cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args(["dump", "tree", "--text", "--highlights", "-l", "javascript"])
        .write_stdin("const answer = 42;\n")
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let ordered = [
        "@keyword language: javascript, range: 0:0-0:5, text: \"const\"",
        "[identifier] field: name",
        "@operator language: javascript, range: 0:13-0:14, text: \"=\"",
        "[number] field: value",
        "@punctuation.delimiter language: javascript, range: 0:17-0:18, text: \";\"",
    ]
    .map(|text| stdout.find(text).unwrap());
    assert!(ordered.windows(2).all(|pair| pair[0] < pair[1]));

    let output = cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args(["dump", "tree", "--text", "--highlights", "-l", "html"])
        .write_stdin("<div>text</div>\n")
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let ordered = [
        "@tag.delimiter language: html, range: 0:0-0:1, text: \"<\"",
        "[tag_name]",
        "@tag.delimiter language: html, range: 0:4-0:5, text: \">\"",
        "[text]",
        "@tag.delimiter language: html, range: 0:9-0:11, text: \"</\"",
    ]
    .map(|text| stdout.find(text).unwrap());
    assert!(ordered.windows(2).all(|pair| pair[0] < pair[1]));
}

#[test]
fn dump_tree_reports_exact_injected_highlight_ranges() {
    cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args([
            "dump",
            "tree",
            "--injections",
            "--text",
            "--highlights",
            "-l",
            "markdown",
        ])
        .write_stdin("```javascript\nconst answer = 42;\n```\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("[code_fence_content]"))
        .stdout(predicate::str::contains(
            "[program] language: javascript, range: 1:0-2:0",
        ))
        .stdout(predicate::str::contains(
            "@keyword language: javascript, range: 1:0-1:5, text: \"const\"",
        ))
        .stdout(predicate::str::contains(
            "@variable language: javascript, range: 1:6-1:12, text: \"answer\"",
        ));
}

#[test]
fn dump_tree_reports_exact_highlights_across_languages() {
    let cases = [
        (
            "python",
            "def greet(name):\n    return name\n",
            vec![
                "@keyword language: python, range: 0:0-0:3, text: \"def\"",
                "@function language: python, range: 0:4-0:9, text: \"greet\"",
            ],
        ),
        (
            "html",
            "<div class=\"x\">hello</div>\n",
            vec![
                "@tag.delimiter language: html, range: 0:0-0:1, text: \"<\"",
                "@tag.delimiter language: html, range: 0:14-0:15, text: \">\"",
                "@string language: html, range: 0:11-0:14, text: \"\\\"x\\\"\"",
            ],
        ),
        (
            "json",
            "{\"x\": true}\n",
            vec![
                "@punctuation.bracket language: json, range: 0:0-0:1, text: \"{\"",
                "@punctuation.bracket language: json, range: 0:10-0:11, text: \"}\"",
                "@boolean language: json, range: 0:6-0:10, text: \"true\"",
            ],
        ),
        (
            "css",
            "body { color: red; }\n",
            vec![
                "@punctuation.bracket language: css, range: 0:5-0:6, text: \"{\"",
                "@punctuation.bracket language: css, range: 0:19-0:20, text: \"}\"",
                "@property language: css, range: 0:7-0:12, text: \"color\"",
            ],
        ),
        (
            "diff",
            "--- a/x\n+++ b/x\n-old\n+new\n",
            vec![
                "@diff.minus language: diff, range: 0:0-0:7, text: \"--- a/x\"",
                "@diff.plus language: diff, range: 1:0-1:7, text: \"+++ b/x\"",
                "@punctuation.special language: diff, range: 2:0-2:1, text: \"-\"",
            ],
        ),
    ];

    for (language, source, expected) in cases {
        let output = cmd()
            .arg("--data-dir")
            .arg(fixtures_dir())
            .args(["dump", "tree", "--text", "--highlights", "-l", language])
            .write_stdin(source)
            .output()
            .unwrap();
        assert!(output.status.success(), "tree dump failed for {language}");
        let stdout = String::from_utf8(output.stdout).unwrap();
        for capture in expected {
            assert!(
                stdout.contains(capture),
                "missing {capture:?} in {language} output:\n{stdout}"
            );
        }
    }
}

const DIFF_SNIPPET: &str = "\
--- a/foo.txt
+++ b/foo.txt
@@ -1,3 +1,3 @@
 context
-old line
+new line
 context
";

#[test]
fn highlight_source_diff_terminal() {
    cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args(["highlight", "-l", "diff"])
        .write_stdin(DIFF_SNIPPET)
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn highlight_source_terminal_with_custom_background() {
    cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args(["highlight", "-l", "diff", "-b", "#282a36"])
        .write_stdin(DIFF_SNIPPET)
        .assert()
        .success()
        .stdout(predicate::str::contains("\u{1b}[48;2;40;42;54m"));
}

#[test]
fn highlight_uses_theme_from_config() {
    let tmp = tempfile::tempdir().unwrap();
    let config_path = tmp.path().join("config.toml");
    write_file(&config_path, "[highlight]\ntheme = \"dracula\"\n");

    cmd()
        .env("LUMIS_CONFIG", &config_path)
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args([
            "highlight",
            "-l",
            "diff",
            "--background",
            "theme",
            "--width",
            "20",
        ])
        .write_stdin("abc\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("\u{1b}[48;2;40;42;54m"));
}

#[test]
fn highlight_verbose_reports_resolved_theme() {
    cmd()
        .arg("--verbose")
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args([
            "highlight",
            "-l",
            "diff",
            "--theme",
            "dracula",
            "--background",
            "theme",
            "--width",
            "20",
        ])
        .write_stdin("abc\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("\u{1b}[48;2;40;42;54m"))
        .stderr(predicate::str::contains("--\nlanguage: diff"))
        .stderr(predicate::str::contains("language: diff"))
        .stderr(predicate::str::contains("theme: dracula"))
        .stderr(predicate::str::contains("theme: dracula\n--\n\n"));
}

#[test]
fn highlight_theme_flag_overrides_config() {
    let tmp = tempfile::tempdir().unwrap();
    let config_path = tmp.path().join("config.toml");
    write_file(&config_path, "[highlight]\ntheme = \"github_light\"\n");

    cmd()
        .env("LUMIS_CONFIG", &config_path)
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args([
            "highlight",
            "-l",
            "diff",
            "--theme",
            "dracula",
            "--background",
            "theme",
            "--width",
            "20",
        ])
        .write_stdin("abc\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("\u{1b}[48;2;40;42;54m"));
}

#[test]
fn highlight_theme_flag_skips_configured_auto_theme() {
    let tmp = tempfile::tempdir().unwrap();
    let config_path = tmp.path().join("config.toml");
    write_file(&config_path, "[highlight]\ntheme = \"auto\"\n");

    cmd()
        .env("LUMIS_CONFIG", &config_path)
        .arg("--data-dir")
        .arg(fixtures_dir())
        .arg("--verbose")
        .args([
            "highlight",
            "-l",
            "diff",
            "--theme",
            "dracula",
            "--background",
            "theme",
            "--width",
            "20",
        ])
        .write_stdin("abc\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("\u{1b}[48;2;40;42;54m"))
        .stderr(predicate::str::contains("theme: dracula"))
        .stderr(predicate::str::contains("theme: auto unavailable").not());
}

#[test]
fn config_flag_overrides_config_environment_variable() {
    let tmp = tempfile::tempdir().unwrap();
    let env_config_path = tmp.path().join("env-config.toml");
    let flag_config_path = tmp.path().join("flag-config.toml");
    write_file(&env_config_path, "[highlight]\ntheme = \"github_light\"\n");
    write_file(&flag_config_path, "[highlight]\ntheme = \"dracula\"\n");

    cmd()
        .env("LUMIS_CONFIG", &env_config_path)
        .args(["--config", flag_config_path.to_str().unwrap()])
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args([
            "highlight",
            "-l",
            "diff",
            "--background",
            "theme",
            "--width",
            "20",
        ])
        .write_stdin("abc\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("\u{1b}[48;2;40;42;54m"));
}

#[test]
fn highlight_auto_renders_without_theme_when_detection_is_unavailable() {
    let tmp = tempfile::tempdir().unwrap();
    let config_path = tmp.path().join("config.toml");
    write_file(&config_path, "[highlight]\ntheme = \"auto\"\n");

    cmd()
        .env("LUMIS_CONFIG", &config_path)
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args(["highlight", "-l", "diff"])
        .write_stdin(DIFF_SNIPPET)
        .assert()
        .success()
        .stdout(predicate::str::contains("\u{1b}[").not());
}

#[test]
fn highlight_verbose_reports_unavailable_auto_theme() {
    let tmp = tempfile::tempdir().unwrap();
    let config_path = tmp.path().join("config.toml");
    write_file(&config_path, "[highlight]\ntheme = \"auto\"\n");

    cmd()
        .env("LUMIS_CONFIG", &config_path)
        .arg("--data-dir")
        .arg(fixtures_dir())
        .arg("--verbose")
        .args(["highlight", "-l", "diff"])
        .write_stdin(DIFF_SNIPPET)
        .assert()
        .success()
        .stdout(predicate::str::contains("\u{1b}[").not())
        .stderr(predicate::str::contains("--\nlanguage: diff"))
        .stderr(predicate::str::contains("language: diff"))
        .stderr(predicate::str::contains("theme: auto unavailable"))
        .stderr(predicate::str::contains("theme: auto unavailable\n--\n\n"));
}

#[test]
fn highlight_reports_invalid_config() {
    let tmp = tempfile::tempdir().unwrap();
    let config_path = tmp.path().join("config.toml");
    write_file(&config_path, "[highlight\ntheme = \"dracula\"\n");

    cmd()
        .env("LUMIS_CONFIG", &config_path)
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args(["highlight", "-l", "diff"])
        .write_stdin(DIFF_SNIPPET)
        .assert()
        .failure()
        .stderr(predicate::str::contains("failed to parse config file"));
}

#[test]
fn highlight_source_terminal_with_theme_background_and_width() {
    cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args([
            "highlight",
            "-l",
            "diff",
            "--theme",
            "dracula",
            "--background",
            "theme",
            "-w",
            "20",
        ])
        .write_stdin("abc\n")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "\u{1b}[0m\u{1b}[48;2;40;42;54mabc\u{1b}[0m\u{1b}[0m\u{1b}[48;2;40;42;54m                 \u{1b}[0m\n",
        ));
}

#[test]
fn highlight_source_diff_html_inline() {
    cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args([
            "highlight",
            "-l",
            "diff",
            "-f",
            "html-inline",
            "-t",
            "dracula",
        ])
        .write_stdin(DIFF_SNIPPET)
        .assert()
        .success()
        .stdout(predicate::str::contains("<pre"));
}

#[test]
fn highlight_source_html_inline_routes_parity_options() {
    cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args([
            "highlight",
            "-l",
            "diff",
            "-f",
            "html-inline",
            "-t",
            "dracula",
            "--pre-class",
            "custom",
            "--italic",
            "--include-highlights",
            "--header-open",
            "<figure>",
            "--header-close",
            "</figure>",
            "-H",
            "1",
            "--highlight-lines-class",
            "selected",
            "--highlight-lines-style",
            "none",
        ])
        .write_stdin(DIFF_SNIPPET)
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            "<figure><pre class=\"lumis custom\"",
        ))
        .stdout(predicate::str::contains(
            "<div class=\"l-line selected\" data-line=\"1\"><span",
        ))
        .stdout(predicate::str::contains("data-highlight=\""))
        .stdout(predicate::str::contains("font-style: italic;"))
        .stdout(predicate::str::ends_with("</code></pre></figure>"));
}

#[test]
fn highlight_source_diff_html_linked() {
    cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args([
            "highlight",
            "-l",
            "diff",
            "-f",
            "html-linked",
            "--pre-class",
            "custom",
            "--header-open",
            "<figure>",
            "--header-close",
            "</figure>",
            "-H",
            "1",
            "--highlight-lines-class",
            "selected",
        ])
        .write_stdin(DIFF_SNIPPET)
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            "<figure><pre class=\"lumis custom\"",
        ))
        .stdout(predicate::str::contains(
            "<div class=\"l-line selected\" data-line=\"1\">",
        ))
        .stdout(predicate::str::ends_with("</code></pre></figure>"));
}

#[test]
fn highlight_source_diff_bbcode_scoped() {
    let source = "@@ -1 +1 @@\n-[url=x]\n+[url=y]\n";

    cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args(["highlight", "-l", "diff", "-f", "bbcode-scoped"])
        .write_stdin(source)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "[attribute-diff]@@ -1 +1 @@[/attribute-diff]",
        ))
        .stdout(predicate::str::contains("&#91;url=x&#93;"))
        .stdout(predicate::str::contains("&#91;url=y&#93;"));
}

#[test]
fn highlight_file_path_autodetects_language() {
    let tmp = tempfile::tempdir().unwrap();
    let source = tmp.path().join("sample.json");
    write_file(&source, r#"{"key": true}"#);

    cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args(["highlight", source.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn highlight_source_diff_html_multi_themes_with_all_options() {
    cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args([
            "highlight",
            "-l",
            "diff",
            "-f",
            "html-multi-themes",
            "--themes",
            "main:dracula",
            "--themes",
            "alt:github_dark",
            "--default-theme",
            "main",
            "--css-variable-prefix=--demo",
            "--pre-class",
            "custom",
            "--italic",
            "--include-highlights",
            "--header-open",
            "<figure>",
            "--header-close",
            "</figure>",
            "-H",
            "2",
            "--highlight-lines-class",
            "selected",
            "--highlight-lines-style",
            "none",
        ])
        .write_stdin(DIFF_SNIPPET)
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            "<figure><pre class=\"lumis lumis-themes custom ",
        ))
        .stdout(predicate::str::contains("--demo-alt"))
        .stdout(predicate::str::contains(
            "<div class=\"l-line selected\" data-line=\"2\"><span",
        ))
        .stdout(predicate::str::contains("data-highlight=\""))
        .stdout(predicate::str::contains("font-style:"))
        .stdout(predicate::str::ends_with("</code></pre></figure>"));
}

#[test]
fn highlight_requires_themes_for_html_multi_themes() {
    cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args(["highlight", "-l", "diff", "-f", "html-multi-themes"])
        .write_stdin(DIFF_SNIPPET)
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "--formatter html-multi-themes requires --themes",
        ));
}

#[test]
fn highlight_rejects_invalid_highlight_line_ranges() {
    cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args(["highlight", "-l", "diff", "-f", "html-inline", "-H", "3-1"])
        .write_stdin(DIFF_SNIPPET)
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Start line (3) must be less than or equal to end line (1)",
        ));
}

/// The formatter check runs before the value is parsed, so a flag the formatter
/// ignores is reported as inapplicable rather than as malformed.
#[test]
fn an_inapplicable_flag_is_reported_before_its_value_is_parsed() {
    cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args([
            "highlight",
            "-f",
            "html-inline",
            "-l",
            "diff",
            "-w",
            "not-a-width",
        ])
        .write_stdin(DIFF_SNIPPET)
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "`--width` is not accepted by the `html-inline` formatter",
        ));
}

#[test]
fn themes_generate_prints_json_to_stdout() {
    let tmp = tempfile::tempdir().unwrap();
    let bin_dir = tmp.path().join("bin");
    let capture_dir = tmp.path().join("capture");

    fs::create_dir_all(&bin_dir).unwrap();
    fs::create_dir_all(&capture_dir).unwrap();
    install_fake_nvim(&bin_dir);

    cmd()
        .env("PATH", fake_nvim_path(&bin_dir))
        .env("LUMIS_FAKE_NVIM_CAPTURE_DIR", &capture_dir)
        .args([
            "themes",
            "generate",
            "-u",
            "https://github.com/folke/tokyonight.nvim",
            "-c",
            "tokyonight-night",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""name":"tokyonight-night""#));

    assert!(capture_dir.join("extract_theme.lua").exists());
    assert!(capture_dir.join("init.lua").exists());
    assert!(capture_dir.join("themes.lua").exists());

    let themes_lua = fs::read_to_string(capture_dir.join("themes.lua")).unwrap();
    assert!(themes_lua.contains("https://github.com/folke/tokyonight.nvim"));
    assert!(themes_lua.contains("vim.o.background = \"dark\""));
    assert!(!themes_lua.contains("vim.g.test_setup = true"));

    let extract_theme = fs::read_to_string(capture_dir.join("extract_theme.lua")).unwrap();
    assert!(extract_theme.contains("extract_theme.lua requires a theme name"));
}

#[test]
fn themes_generate_supports_output_setup_and_appearance_options() {
    let tmp = tempfile::tempdir().unwrap();
    let bin_dir = tmp.path().join("bin");
    let capture_dir = tmp.path().join("capture");
    let output_path = tmp.path().join("tokyonight.json");

    fs::create_dir_all(&bin_dir).unwrap();
    fs::create_dir_all(&capture_dir).unwrap();
    install_fake_nvim(&bin_dir);

    cmd()
        .env("PATH", fake_nvim_path(&bin_dir))
        .env("LUMIS_FAKE_NVIM_CAPTURE_DIR", &capture_dir)
        .env("LUMIS_FAKE_NVIM_APPEARANCE", "light")
        .args([
            "themes",
            "generate",
            "-u",
            "https://github.com/folke/tokyonight.nvim",
            "-c",
            "tokyonight-day",
            "-s",
            "vim.g.test_setup = true",
            "-o",
            output_path.to_str().unwrap(),
            "-a",
            "light",
        ])
        .assert()
        .success()
        .stderr(predicate::str::contains("Theme saved to"));

    let generated = fs::read_to_string(&output_path).unwrap();
    assert!(generated.contains(r#""appearance":"light""#));

    let themes_lua = fs::read_to_string(capture_dir.join("themes.lua")).unwrap();
    assert!(themes_lua.contains("vim.o.background = \"light\""));
    assert!(themes_lua.contains("vim.g.test_setup = true"));
}

// -- languages cache --

/// clap prints each env-backed option's current value, so `$LUMIS_DATA_DIR` and
/// `$LUMIS_CONFIG` land in this output. A checkout under a path containing
/// "fetch" or "update" would fail the assertions below on the paths rather than
/// on the wording, so both are cleared.
#[test]
fn languages_help_uses_cache_terminology() {
    cmd()
        .env("LUMIS_DATA_DIR", "")
        .env("LUMIS_CONFIG", "")
        .args(["languages", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("cache"))
        .stdout(predicate::str::contains("fetch").not())
        .stdout(predicate::str::contains("update").not());
}

#[test]
fn cache_languages_no_args_fails() {
    cmd()
        .args(["languages", "cache"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "specify language names, a bundle such as bundle-web, or --all",
        ));
}

#[test]
fn cache_languages_rejects_an_unknown_bundle() {
    cmd()
        .args(["languages", "cache", "bundle-nope"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("unknown bundle 'bundle-nope'"));
}

#[test]
fn cache_languages_rejects_languages_with_all() {
    cmd()
        .args(["languages", "cache", "rust", "--all"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "pass language names or --all, not both",
        ));
}

#[test]
fn cache_languages_already_cached() {
    // The fixtures dir already has the diff package cached — silent without -v.
    cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args(["languages", "cache", "diff"])
        .assert()
        .success()
        .stdout("")
        .stderr("");
}

#[test]
fn cache_languages_already_cached_verbose() {
    cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .arg("-v")
        .args(["languages", "cache", "diff"])
        .assert()
        .success()
        .stderr(predicate::str::contains("--> diff"))
        .stderr(predicate::str::contains("downloaded to "))
        .stderr(predicate::str::contains("tree-sitter-diff-"))
        .stderr(predicate::str::is_match("cached in [0-9]+\\.[0-9]{3}s").unwrap())
        .stderr(predicate::str::is_match("compiled in [0-9]+\\.[0-9]{3}s").unwrap());
}

#[test]
fn cache_languages_skips_plaintext_aliases() {
    let tmp = tempfile::tempdir().unwrap();

    cmd()
        .arg("--data-dir")
        .arg(tmp.path())
        .arg("-v")
        .args(["languages", "cache", "plaintext", "text", "txt", "plain"])
        .assert()
        .success()
        .stdout("")
        .stderr("");

    assert!(fs::read_dir(tmp.path().join("parsers"))
        .unwrap()
        .next()
        .is_none());
}

/// Downloading is the smaller half of a cold parser; the Wasmtime compile is the
/// larger, and a prepared directory is meant to carry both. Host cache APIs do
/// the same, so they cannot prepare different things.
#[test]
fn cache_languages_compiles_what_it_downloads() {
    // Building a runtime at all compiles Tree-sitter's own module, so a
    // non-empty cache proves nothing. Each additional parser adds an entry
    // beside it, and only compiling produces that.
    fn cached_modules(store: &std::path::Path) -> usize {
        let Ok(namespaces) = fs::read_dir(store.join("compiled").join("modules")) else {
            return 0;
        };
        namespaces
            .filter_map(Result::ok)
            .filter_map(|namespace| fs::read_dir(namespace.path()).ok())
            .flat_map(|entries| entries.filter_map(Result::ok))
            .filter(|entry| entry.path().extension().is_none())
            .count()
    }

    let one = seeded_store();
    cmd()
        .arg("--data-dir")
        .arg(one.path())
        .args(["languages", "cache", "json"])
        .assert()
        .success();

    let two = seeded_store();
    cmd()
        .arg("--data-dir")
        .arg(two.path())
        .args(["languages", "cache", "json", "css"])
        .assert()
        .success();

    assert!(
        cached_modules(two.path()) > cached_modules(one.path()),
        "caching must compile each parser, not just download it: {} vs {}",
        cached_modules(two.path()),
        cached_modules(one.path())
    );
}

/// A store directory holding the committed fixtures, so `languages cache` can be
/// exercised without a download. Caching into an empty directory needs the
/// network by construction; there is no second directory to copy from.
fn seeded_store() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let parsers = dir.path().join("parsers");
    fs::create_dir_all(&parsers).unwrap();
    for entry in fs::read_dir(common::data_dir().join("parsers")).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            fs::copy(entry.path(), parsers.join(entry.file_name())).unwrap();
        }
    }
    dir
}

#[test]
fn cache_languages_to_temp_dir() {
    let tmp = seeded_store();
    cmd()
        .arg("--data-dir")
        .arg(tmp.path())
        .args(["languages", "cache", "json"])
        .assert()
        .success();

    // Verify the WASM file was cached
    assert!(fs::read_dir(tmp.path().join("parsers"))
        .unwrap()
        .any(|entry| entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with("tree-sitter-json-")));
}

#[test]
fn cache_languages_to_temp_dir_verbose() {
    let tmp = seeded_store();
    cmd()
        .arg("-v")
        .arg("--data-dir")
        .arg(tmp.path())
        .args(["languages", "cache", "json"])
        .assert()
        .success()
        .stderr(predicate::str::contains("tree-sitter-json-"));

    assert!(fs::read_dir(tmp.path().join("parsers"))
        .unwrap()
        .any(|entry| entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with("tree-sitter-json-")));
}

#[test]
fn cache_languages_reuses_an_existing_file() {
    let tmp = seeded_store();

    // Cache once.
    cmd()
        .arg("--data-dir")
        .arg(tmp.path())
        .args(["languages", "cache", "json"])
        .assert()
        .success();

    // Then reuse it without a network request.
    cmd()
        .arg("-v")
        .arg("--data-dir")
        .arg(tmp.path())
        .args(["languages", "cache", "json"])
        .assert()
        .success()
        .stderr(predicate::str::contains("tree-sitter-json-"));
}

#[test]
fn cache_languages_then_highlight() {
    let tmp = seeded_store();

    // Cache the parser first.
    cmd()
        .arg("--data-dir")
        .arg(tmp.path())
        .args(["languages", "cache", "json"])
        .assert()
        .success();

    // Now highlight with it
    cmd()
        .arg("--data-dir")
        .arg(tmp.path())
        .args(["highlight", "-l", "json"])
        .write_stdin(r#"{"key": "value"}"#)
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

/// Highlighting an HTML document loads JavaScript for its `<script>` block
/// during the same pass, from a data directory that has never held either
/// parser. Before this, the block stayed plain until `languages cache javascript`
/// had been run.
/// One pass: nothing names javascript, so the walk had to discover the `<script>`
/// injection and load it mid-walk. Whether the parser was already on disk is a
/// separate question — every catalog package is published, so "present" and
/// "obtainable" cannot be told apart without a fetcher that refuses.
#[test]
fn highlighting_loads_an_injected_language_the_caller_never_named() {
    cmd()
        .args(["dump", "events", "-l", "html"])
        .write_stdin("<script>const answer = 42</script>")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"language\": \"javascript\""));
}

/// A document is worth more than the one block Lumis cannot highlight, so a
/// block naming a language it cannot load leaves that block plain and the rest
/// standing.
///
/// Named so no catalog lookup can succeed, which also keeps the test off the
/// network; `lumis-wasm-runtime` covers the known-but-unfetchable case against
/// a fetcher that refuses everything.
#[test]
fn an_unloadable_injected_language_does_not_fail_the_document() {
    cmd()
        .args(["dump", "events", "-l", "markdown"])
        .write_stdin("# Title\n\n```notalanguage\nx = 1\n```\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"language\": \"markdown\""));
}

#[test]
fn short_h_prints_help_rather_than_asking_for_line_numbers() {
    cmd()
        .args(["highlight", "-h"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Highlight source code"));
}

#[test]
fn highlight_help_groups_options_by_the_formatters_that_accept_them() {
    cmd()
        .args(["highlight", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Options for --formatter terminal:",
        ))
        .stdout(predicate::str::contains(
            "Options for --formatter html-inline, html-linked, html-multi-themes:",
        ))
        .stdout(predicate::str::contains(
            "Options for --formatter html-inline, html-multi-themes:",
        ))
        .stdout(predicate::str::contains(
            "Options for --formatter html-multi-themes:",
        ));
}

#[test]
fn highlight_rejects_an_option_the_chosen_formatter_ignores() {
    cmd()
        .args(["highlight", "-l", "diff", "--pre-class", "custom"])
        .write_stdin(DIFF_SNIPPET)
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "`--pre-class` is not accepted by the `terminal` formatter",
        ))
        .stderr(predicate::str::contains(
            "HTML options apply to: html-inline, html-linked, html-multi-themes",
        ))
        .stderr(predicate::str::contains("lumis formatters show terminal"));
}

#[test]
fn highlight_names_every_rejected_flag_in_one_group_in_one_error() {
    cmd()
        .args([
            "highlight",
            "-l",
            "diff",
            "-f",
            "html-inline",
            "-b",
            "theme",
            "-w",
            "120",
        ])
        .write_stdin(DIFF_SNIPPET)
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "`--background`, `--width` are not accepted by the `html-inline` formatter",
        ));
}

#[test]
fn highlight_names_rejected_flags_from_every_group_in_one_error() {
    cmd()
        .args([
            "highlight",
            "-l",
            "diff",
            "-f",
            "html-linked",
            "-t",
            "dracula",
            "--italic",
        ])
        .write_stdin(DIFF_SNIPPET)
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "`--theme`, `--italic` are not accepted by the `html-linked` formatter",
        ))
        .stderr(predicate::str::contains(
            "`--theme` applies to: html-inline, terminal",
        ))
        .stderr(predicate::str::contains(
            "inline-style options apply to: html-inline, html-multi-themes",
        ));
}

#[test]
fn highlight_rejects_theme_for_a_formatter_that_cannot_color() {
    cmd()
        .args([
            "highlight",
            "-l",
            "diff",
            "-f",
            "html-linked",
            "-t",
            "dracula",
        ])
        .write_stdin(DIFF_SNIPPET)
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "`--theme` is not accepted by the `html-linked` formatter",
        ))
        .stderr(predicate::str::contains(
            "`--theme` applies to: html-inline, terminal",
        ));
}

/// `html_linked::HighlightLines` has no `style` field, so the flag is rejected
/// even though `--highlight-lines` itself is accepted.
#[test]
fn highlight_rejects_highlight_lines_style_for_html_linked() {
    cmd()
        .args([
            "highlight",
            "-l",
            "diff",
            "-f",
            "html-linked",
            "-H",
            "1",
            "--highlight-lines-style",
            "none",
        ])
        .write_stdin(DIFF_SNIPPET)
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "`--highlight-lines-style` is not accepted by the `html-linked` formatter",
        ));
}

/// A theme in the config file applies to every command, so it must not turn
/// `-f html-linked` into an error the user cannot see the cause of.
#[test]
fn a_config_theme_does_not_trip_the_formatter_check() {
    let tmp = tempfile::tempdir().unwrap();
    let config = tmp.path().join("config.toml");
    write_file(&config, "[highlight]\ntheme = \"dracula\"\n");

    cmd()
        .env("LUMIS_CONFIG", &config)
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args(["highlight", "-l", "diff", "-f", "html-linked"])
        .write_stdin(DIFF_SNIPPET)
        .assert()
        .success();
}

#[test]
fn css_variable_prefix_accepts_a_leading_dash_value() {
    cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args([
            "highlight",
            "-l",
            "diff",
            "-f",
            "html-multi-themes",
            "--themes",
            "main:dracula",
            "--css-variable-prefix=--shiki",
        ])
        .write_stdin(DIFF_SNIPPET)
        .assert()
        .success()
        .stdout(predicate::str::contains("--shiki-"));
}

#[test]
fn formatters_list_names_every_formatter() {
    cmd()
        .args(["formatters", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("html-inline"))
        .stdout(predicate::str::contains("html-linked"))
        .stdout(predicate::str::contains("html-multi-themes"))
        .stdout(predicate::str::contains("terminal"))
        .stdout(predicate::str::contains("bbcode-scoped"));
}

#[test]
fn formatters_show_lists_only_what_the_formatter_accepts() {
    cmd()
        .args(["formatters", "show", "terminal"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--background"))
        .stdout(predicate::str::contains("--width"))
        .stdout(predicate::str::contains("--theme"))
        .stdout(predicate::str::contains("--highlight-lines"))
        .stdout(predicate::str::contains("--highlight-lines-background"))
        .stdout(predicate::str::contains("--formatter").not())
        .stdout(predicate::str::contains("--pre-class").not())
        .stdout(predicate::str::contains("--highlight-lines-class").not())
        .stdout(predicate::str::contains("--italic").not());
}

/// Every formatter takes `--highlight-lines` now, so the flag that used to be
/// HTML-only has to reach the two that had no line structure of their own.
#[test]
fn terminal_paints_a_highlighted_line() {
    cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args([
            "highlight",
            "-f",
            "terminal",
            "-l",
            "diff",
            "-H",
            "1",
            "--highlight-lines-background",
            "#ff0000",
        ])
        .write_stdin(DIFF_SNIPPET)
        .assert()
        .success()
        .stdout(predicate::str::contains("\u{1b}[48;2;255;0;0m"));
}

#[test]
fn bbcode_wraps_a_highlighted_line() {
    cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args(["highlight", "-f", "bbcode-scoped", "-l", "diff", "-H", "1"])
        .write_stdin(DIFF_SNIPPET)
        .assert()
        .success()
        .stdout(predicate::str::contains("[highlighted]"))
        .stdout(predicate::str::contains("[/highlighted]"));
}

#[test]
fn bbcode_without_highlight_lines_has_no_line_structure() {
    cmd()
        .arg("--data-dir")
        .arg(fixtures_dir())
        .args(["highlight", "-f", "bbcode-scoped", "-l", "diff"])
        .write_stdin(DIFF_SNIPPET)
        .assert()
        .success()
        .stdout(predicate::str::contains("[highlighted]").not());
}

#[test]
fn formatters_show_rejects_an_unknown_formatter() {
    cmd()
        .args(["formatters", "show", "nonsense"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("nonsense"));
}

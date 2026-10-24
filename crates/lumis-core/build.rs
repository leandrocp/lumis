// Generates language metadata into OUT_DIR:
//
// 1. `languages_data.rs` (included by `src/languages.rs`)
//
//    Reads `languages.toml` and emits a `define_languages!` macro invocation
//    that declares the `Language` enum with all parser variants, detection
//    metadata (globs, aliases, emacs modes, shebangs), and feature gates.
//
//    Variants are split into two groups:
//      - `always`: PlainText, Diff (no feature gate)
//      - `gated`: everything else, behind `lang-*` features
//
// To inspect the generated output, look in:
//   target/debug/build/lumis-core-<hash>/out/

use serde::Deserialize;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Deserialize)]
struct LanguagesToml {
    parsers: BTreeMap<String, ParserEntry>,
    #[serde(flatten)]
    _rest: toml::Value,
}

#[derive(Deserialize)]
struct ParserEntry {
    globs: Option<Vec<String>>,
    aliases: Option<Vec<String>>,
    variant: Option<String>,
    display_name: Option<String>,
    wasm_name: Option<String>,
    emacs: Option<Vec<String>>,
    shebang: Option<Vec<String>>,
    feature: Option<String>,
    #[serde(flatten)]
    _rest: toml::Value,
}

fn manifest_dir() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
}

fn titlecase(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().to_string() + chars.as_str(),
    }
}

fn key_to_feature(key: &str) -> String {
    format!("lang-{}", key.replace('_', "-"))
}

fn format_str_list(list: &[String]) -> String {
    if list.is_empty() {
        return "[]".to_string();
    }
    let items: Vec<String> = list.iter().map(|s| format!("\"{s}\"")).collect();
    format!("[{}]", items.join(", "))
}

fn format_entry(key: &str, entry: &ParserEntry) -> String {
    let variant = entry
        .variant
        .as_deref()
        .unwrap_or(&titlecase(key))
        .to_string();
    let display_name = entry
        .display_name
        .as_deref()
        .unwrap_or(&variant)
        .to_string();

    let mut from_str = vec![key.to_string()];
    if let Some(aliases) = &entry.aliases {
        from_str.extend(aliases.iter().cloned());
    }

    let globs = entry.globs.as_deref().unwrap_or(&[]);
    let emacs = entry.emacs.as_deref().unwrap_or(&[]);
    let shebang = entry.shebang.as_deref().unwrap_or(&[]);
    let wasm_name = entry.wasm_name.as_deref().unwrap_or(key);
    let parser = wasm_name.strip_prefix("tree-sitter-").unwrap_or(wasm_name);

    format!(
        r#"{variant} {{
            id: "{key}",
            name: "{display_name}",
            parser: Some("{parser}"),
            from_str: {from_str},
            globs: {globs},
            emacs: {emacs},
            shebang: {shebang}
        }}"#,
        variant = variant,
        key = key,
        display_name = display_name,
        from_str = format_str_list(&from_str),
        globs = format_str_list(globs),
        emacs = format_str_list(emacs),
        shebang = format_str_list(shebang),
    )
}

fn languages() {
    // In the workspace, languages.toml lives at the repo root (../../).
    // When published, it's included at the crate root via Cargo.toml `include`.
    let workspace_path = manifest_dir().join("../../languages.toml");
    let crate_path = manifest_dir().join("languages.toml");
    let languages_toml_path = if workspace_path.exists() {
        workspace_path
    } else {
        crate_path
    };

    println!("cargo:rerun-if-changed={}", languages_toml_path.display());

    let content = fs::read_to_string(&languages_toml_path).expect("failed to read languages.toml");
    let toml: LanguagesToml = toml::from_str(&content).expect("failed to parse languages.toml");

    let mut always_entries = Vec::new();
    let mut gated_entries = Vec::new();

    // PlainText is always available and has no parser entry in languages.toml.
    always_entries.push(
        r#"        PlainText {
            id: "plaintext",
            name: "Plain Text",
            parser: None,
            from_str: ["plaintext", "text", "txt", "plain"],
            globs: [],
            emacs: ["fundamental", "text"],
            shebang: []
        }"#
        .to_string(),
    );

    for (key, entry) in &toml.parsers {
        let body = format_entry(key, entry);

        if key == "diff" {
            always_entries.push(format!("        {body}"));
        } else {
            let feat = entry.feature.clone().unwrap_or_else(|| key_to_feature(key));
            gated_entries.push(format!("        [\"{feat}\"] {body}"));
        }
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = out_dir.join("languages_data.rs");
    let mut f = fs::File::create(&out_path).expect("failed to create languages_data.rs");

    writeln!(
        f,
        "// This file is auto-generated from languages.toml by build.rs"
    )
    .unwrap();
    writeln!(f, "// Do not edit.").unwrap();
    writeln!(f).unwrap();
    writeln!(f, "define_languages! {{").unwrap();
    writeln!(f, "    always {{").unwrap();
    writeln!(f, "{}", always_entries.join(",\n")).unwrap();
    writeln!(f, "    }}").unwrap();
    writeln!(f, "    gated {{").unwrap();
    writeln!(f, "{}", gated_entries.join(",\n")).unwrap();
    writeln!(f, "    }}").unwrap();
    writeln!(f, "}}").unwrap();
}

fn main() {
    languages();
}

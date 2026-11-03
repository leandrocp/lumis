// Generates two outputs into OUT_DIR:
//
// 1. Compiled C/C++ tree-sitter parsers (via `vendored_parsers()`)
//
//    Reads `languages.toml` to find parsers without a `crate` field — these
//    are vendored under `vendored_parsers/`. Each parser is gated behind its
//    `lang-*` feature flag. Builds run in parallel using rayon. Scanner files
//    (scanner.c / scanner.cc) are auto-detected from the filesystem.
//
// 2. `queries_constants.rs` (via `queries()`, included by `src/queries.rs`)
//
//    Reads processed .scm files from `queries/processed/<lang>/` and
//    emits one `&str` constant per language per query type:
//      pub const RUST_HIGHLIGHTS: &str = "...";
//      pub const RUST_INJECTIONS: &str = "...";
//      pub const RUST_LOCALS: &str = "...";
//
//    Also converts `#lua-match?` predicates to `#match?` with Lua-to-Rust
//    regex conversion (this step only applies to the native Rust crate,
//    not the CLI which uses web-tree-sitter).
//
//    Feature gates are derived from `languages.toml` using each parser's
//    `query_name` (or key) and `feature` (or `lang-{key}`) fields.
//
// Theme data is provided by lumis-core (re-exported via src/themes.rs).
//
// To inspect the generated output, look in:
//   target/debug/build/lumis-<hash>/out/

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use rayon::prelude::*;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::env;
use std::fmt::Write as _;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use xz2::read::XzDecoder;

// ── languages.toml types ───────────────────────────────────────────────────

#[derive(Deserialize)]
struct LanguagesToml {
    parsers: BTreeMap<String, ParserEntry>,
    #[serde(flatten)]
    _rest: toml::Value,
}

#[derive(Deserialize)]
struct ParserEntry {
    #[serde(rename = "crate")]
    crate_name: Option<String>,
    location: Option<String>,
    query_name: Option<String>,
    feature: Option<String>,
    #[serde(flatten)]
    _rest: toml::Value,
}

fn load_languages_toml() -> LanguagesToml {
    let path = resolve_path("languages.toml");
    println!("cargo:rerun-if-changed={}", path.display());
    let content = fs::read_to_string(&path).expect("failed to read languages.toml");
    toml::from_str(&content).expect("failed to parse languages.toml")
}

/// Derive the feature flag for a parser entry.
fn feature_for(key: &str, entry: &ParserEntry) -> String {
    entry
        .feature
        .clone()
        .unwrap_or_else(|| format!("lang-{}", key.replace('_', "-")))
}

/// Check whether a cargo feature is enabled via `CARGO_FEATURE_*` env vars.
fn is_feature_enabled(feature: &str) -> bool {
    let env_key = format!("CARGO_FEATURE_{}", feature.to_uppercase().replace('-', "_"));
    env::var(env_key).is_ok()
}

// ── main ───────────────────────────────────────────────────────────────────

fn manifest_dir() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
}

fn workspace_root() -> PathBuf {
    manifest_dir()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

/// Resolve a path that lives at the workspace root during development
/// but at the crate root when published (via symlinks in Cargo include).
fn resolve_path(relative_to_root: &str) -> PathBuf {
    let workspace = workspace_root().join(relative_to_root);
    if workspace.exists() {
        return workspace;
    }
    let crate_local = manifest_dir().join(relative_to_root);
    if crate_local.exists() {
        return crate_local;
    }
    workspace
}

fn main() {
    let toml = load_languages_toml();
    vendored_parsers(&toml);
    queries(&toml);
    gen_conformance_tests();
}

// ── vendored parsers ───────────────────────────────────────────────────────

struct TreeSitterParser {
    name: String,
    parser_dir: PathBuf,
    src_dir: PathBuf,
    parser_file: PathBuf,
    extra_files: Vec<PathBuf>,
}

impl TreeSitterParser {
    fn build(&self) {
        let mut c_files = vec![self.parser_file.clone()];
        let mut cpp_files = vec![];

        for file in &self.extra_files {
            if file.extension().and_then(|ext| ext.to_str()) == Some("c") {
                c_files.push(file.clone());
            } else {
                cpp_files.push(file.clone());
            }
        }

        if !cpp_files.is_empty() {
            let mut cpp_build = cc::Build::new();
            cpp_build
                .include(&self.src_dir)
                .cpp(true)
                .std("c++14")
                .flag_if_supported("-Wno-implicit-fallthrough")
                .flag_if_supported("-Wno-unused-parameter")
                .flag_if_supported("-Wno-ignored-qualifiers")
                .link_lib_modifier("+whole-archive");

            for file in &cpp_files {
                cpp_build.file(file);
            }

            cpp_build.compile(&format!("{}-cpp", self.name));
        }

        let mut build = cc::Build::new();
        // Keep the original source directory on the include path even when a
        // compressed scanner is built from OUT_DIR. Nested grammars can use
        // relative includes outside src/, such as XML's ../../common/scanner.h.
        build.include(&self.src_dir).warnings(false);

        // Parsers generated with tree-sitter CLI < 0.26.4 ship an array.h
        // whose macros violate strict aliasing; gcc -O2 miscompiles them on
        // aarch64-linux into heap corruption (tree-sitter/tree-sitter@ed6e42c,
        // tree-sitter/tree-sitter-haskell#144).
        build.flag_if_supported("-fno-strict-aliasing");

        // MSVC reads source and encodes string literals in the system code
        // page, so a non-ASCII token name such as Bend's `"\u03bb"` becomes
        // `?` and queries naming it fail to compile.
        if build.get_compiler().is_like_msvc() {
            build.flag("/utf-8");
        }

        // Prefix TAG_TYPES_BY_TAG_NAME to avoid symbol conflicts between
        // parsers that share the same tag.h definitions (angular, astro, vue).
        let tag_h = self.src_dir.join("tag.h");
        if tag_h.exists() {
            if let Ok(content) = fs::read_to_string(&tag_h) {
                if content.contains("TAG_TYPES_BY_TAG_NAME") {
                    build.flag(format!(
                        "-DTAG_TYPES_BY_TAG_NAME={}_TAG_TYPES_BY_TAG_NAME",
                        self.name.replace('-', "_"),
                    ));
                }
            }
        }

        for file in &c_files {
            build.file(file);
        }

        build.link_lib_modifier("+whole-archive");
        build.compile(&self.name);
    }
}

/// Find the vendored directory name for a parser.
///
/// Uses `query_name` when present, otherwise defaults to `tree-sitter-{key}`.
fn find_vendored_dir(vendored_root: &Path, key: &str, entry: &ParserEntry) -> Option<String> {
    if let Some(query_name) = &entry.query_name {
        let query_dir = format!("tree-sitter-{query_name}");
        if vendored_root.join(&query_dir).exists() {
            return Some(query_dir);
        }
    }

    let default_name = format!("tree-sitter-{key}");
    if vendored_root.join(&default_name).exists() {
        return Some(default_name);
    }

    None
}

/// Determine the src directory within a vendored parser.
///
/// If `location` is set and `{parser_dir}/{location}/src/parser.c` or
/// `{parser_dir}/{location}/src/parser.c.xz` exists,
/// uses the nested path (e.g. tree-sitter-csv/csv/src). Otherwise uses
/// `{parser_dir}/src`.
fn resolve_src_dir(parser_dir: &Path, entry: &ParserEntry) -> PathBuf {
    if let Some(loc) = &entry.location {
        let nested = parser_dir.join(loc).join("src");
        if nested.join("parser.c").exists() || nested.join("parser.c.xz").exists() {
            return nested;
        }
    }
    parser_dir.join("src")
}

fn ensure_source_file(
    out_dir: &Path,
    parser_name: &str,
    src_dir: &Path,
    key: &str,
    file_name: &str,
) -> PathBuf {
    let source = src_dir.join(file_name);
    if source.exists() {
        return source;
    }

    let compressed = src_dir.join(format!("{file_name}.xz"));
    assert!(
        compressed.exists(),
        "missing parser source '{file_name}' for vendored parser '{key}' at {}",
        src_dir.display()
    );

    let unpacked_dir = out_dir.join("vendored_parsers").join(parser_name);
    let unpacked_source = unpacked_dir.join(file_name);

    fs::create_dir_all(&unpacked_dir).expect("failed to create decompressed parser directory");

    let input = File::open(&compressed).unwrap_or_else(|err| {
        panic!(
            "failed to open compressed parser source '{file_name}' for '{key}' at {}: {err}",
            compressed.display()
        )
    });
    let mut decoder = XzDecoder::new(input);
    let mut decoded = Vec::new();
    decoder.read_to_end(&mut decoded).unwrap_or_else(|err| {
        panic!(
            "failed to decompress parser source '{file_name}' for '{key}' at {}: {err}",
            compressed.display()
        )
    });
    fs::write(&unpacked_source, decoded).unwrap_or_else(|err| {
        panic!(
            "failed to write decompressed parser source '{file_name}' for '{key}' at {}: {err}",
            unpacked_source.display()
        )
    });

    unpacked_source
}

// https://github.com/Wilfred/difftastic/blob/8953c55cf854ceac2ccb6ece004d6a94a5bfa122/build.rs
// TODO: remove vendored parsers in favor of crates as soon as they implement LanguageFn
fn vendored_parsers(toml: &LanguagesToml) {
    let vendored_root = manifest_dir().join("vendored_parsers");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let parsers: Vec<TreeSitterParser> = toml
        .parsers
        .iter()
        .filter(|(_, entry)| entry.crate_name.is_none())
        // `plaintext` has no grammar and borrows diff's, so diff is compiled
        // whatever the feature set. `lumis-core`'s build script keeps diff out of
        // its gated block for the same reason.
        .filter(|(key, entry)| {
            key.as_str() == "diff" || is_feature_enabled(&feature_for(key, entry))
        })
        .filter_map(|(key, entry)| {
            let dir_name = find_vendored_dir(&vendored_root, key, entry)?;
            let parser_dir = vendored_root.join(&dir_name);
            let src_dir = resolve_src_dir(&parser_dir, entry);
            let parser_file = ensure_source_file(&out_dir, &dir_name, &src_dir, key, "parser.c");

            assert!(
                parser_file.exists(),
                "missing parser source for vendored parser '{key}' at {}",
                src_dir.display()
            );

            // Auto-detect scanner files
            let mut extra_files = Vec::new();
            for name in ["scanner.c", "scanner.cc"] {
                if src_dir.join(name).exists() || src_dir.join(format!("{name}.xz")).exists() {
                    extra_files.push(ensure_source_file(&out_dir, &dir_name, &src_dir, key, name));
                }
            }

            Some(TreeSitterParser {
                name: dir_name,
                parser_dir,
                src_dir,
                parser_file,
                extra_files,
            })
        })
        .collect();

    for parser in &parsers {
        // Watch the whole parser tree because scanners can include shared files
        // outside src/ (for example tree-sitter-xml/common/scanner.h).
        println!("cargo:rerun-if-changed={}", parser.parser_dir.display());
    }

    parsers.par_iter().for_each(TreeSitterParser::build);
}

// ── queries ────────────────────────────────────────────────────────────────

fn read_query_file(path: &Path) -> String {
    if !path.exists() {
        return String::new();
    }

    fs::read_to_string(path).expect("failed to read query file")
}

fn read_query_file_with_default(path: &Path, default_path: &Path) -> String {
    if path.exists() {
        read_query_file(path)
    } else {
        read_query_file(default_path)
    }
}

fn require_highlights_query(path: &Path, language: &str) {
    assert!(
        path.exists(),
        "missing processed highlights query for language '{language}' at {}. Run `mise run langs-preprocess-queries` first.",
        path.display()
    );

    let content = fs::read_to_string(path).unwrap_or_else(|err| {
        panic!(
            "failed to read processed highlights query for language '{language}' at {}: {err}",
            path.display()
        )
    });

    assert!(
        !content.trim().is_empty(),
        "empty processed highlights query for language '{language}' at {}. Run `mise run langs-preprocess-queries` again.",
        path.display()
    );
}

/// Build a map from query directory name → list of feature flags.
///
/// Uses `query_name` (or the parser key) as the map key, and `feature`
/// (or `lang-{key}`) as the feature flag. Multiple parsers may share the
/// same query name (e.g. ejs and erb both use "`embedded_template`").
fn build_query_feature_map(toml: &LanguagesToml) -> BTreeMap<String, Vec<String>> {
    let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for (key, entry) in &toml.parsers {
        let query_name = entry.query_name.as_deref().unwrap_or(key).to_string();
        let feat = feature_for(key, entry);
        map.entry(query_name).or_default().push(feat);
    }

    map
}

fn queries(toml: &LanguagesToml) {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let dest_path = out_dir.join("queries_constants.rs");

    let queries_path = resolve_path("queries/processed");
    let mut generated_code = TokenStream::new();

    println!("cargo:rerun-if-changed={}", queries_path.display());

    let entries = fs::read_dir(&queries_path).unwrap_or_else(|_| {
        panic!(
            "failed to read queries/processed directory at {}. Run `mise run langs-preprocess-queries` first.",
            queries_path.display()
        )
    });

    let query_feature_map = build_query_feature_map(toml);
    let mut bracket_query_arms = Vec::new();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let language = path.file_name().unwrap().to_str().unwrap();

        // Determine whether to generate constants for this query directory.
        // "diff" is always enabled (used as plaintext fallback).
        let should_generate = if language == "diff" {
            true
        } else if let Some(features) = query_feature_map.get(language) {
            features.iter().any(|f| is_feature_enabled(f))
        } else {
            false
        };

        if !should_generate {
            continue;
        }

        let lang_upper = language.to_uppercase();
        let queries = ["highlights", "injections", "locals", "brackets"];

        require_highlights_query(&path.join("highlights.scm"), language);

        for query in queries {
            let file_path = path.join(format!("{query}.scm"));
            let const_name = format_ident!("{}_{}", lang_upper, query.to_uppercase());
            let processed_content = if query == "brackets" && language != "default" {
                read_query_file_with_default(&file_path, &queries_path.join("default/brackets.scm"))
            } else {
                read_query_file(&file_path)
            };

            generated_code.extend(quote! {
                #[doc(hidden)]
                pub const #const_name: &str = #processed_content;
            });

            generated_code.extend(quote! {});
        }

        let brackets_const = format_ident!("{}_BRACKETS", lang_upper);
        bracket_query_arms.push(quote! { #language => #brackets_const, });

        generated_code.extend(quote! {});
    }

    generated_code.extend(quote! {
        #[doc(hidden)]
        pub fn bracket_query_for_language(language: &str) -> &'static str {
            match language {
                #(#bracket_query_arms)*
                _ => "",
            }
        }
    });

    let mut output_file = File::create(&dest_path).unwrap();

    write!(
        output_file,
        "{}",
        prettyplease::unparse(&syn::parse2::<syn::File>(generated_code).unwrap())
    )
    .unwrap();
}

// ── conformance tests ──────────────────────────────────────────────────────

fn gen_conformance_tests() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let conformance_dir = resolve_path("fixtures/conformance");

    println!("cargo:rerun-if-changed={}", conformance_dir.display());

    let mut code = String::new();

    if let Ok(entries) = fs::read_dir(&conformance_dir) {
        let mut names: Vec<String> = entries
            .filter_map(|e| {
                let e = e.ok()?;
                if e.file_type().ok()?.is_dir() {
                    Some(e.file_name().to_string_lossy().to_string())
                } else {
                    None
                }
            })
            .filter(|n| !n.starts_with('.'))
            .collect();
        names.sort();

        for name in &names {
            let ident = name.replace('-', "_");
            let _ = write!(
                code,
                r#"
mod {ident} {{
    use super::*;
    fn fixture() -> Fixture {{ load_fixture("{name}") }}
    #[test] #[ignore = "conformance"] fn conformance_events() {{ check_events(&fixture()); }}
    #[test] #[ignore = "conformance"] fn conformance_html_inline() {{ check_html_inline(&fixture()); }}
    #[test] #[ignore = "conformance"] fn conformance_html_linked() {{ check_html_linked(&fixture()); }}
    #[test] #[ignore = "conformance"] fn conformance_html_multi_themes() {{ check_html_multi_themes(&fixture()); }}
    #[test] #[ignore = "conformance"] fn conformance_terminal() {{ check_terminal(&fixture()); }}
    #[test] #[ignore = "conformance"] fn conformance_bbcode_scoped() {{ check_bbcode(&fixture()); }}
}}
"#
            );
        }
    }

    fs::write(out_dir.join("conformance_tests.rs"), code).unwrap();
}

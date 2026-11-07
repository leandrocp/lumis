//! Every parser loads through Wasmtime and compiles and runs its processed
//! queries on its `samples/` file.
//!
//! The JavaScript half, `packages/javascript/lumis/test/query-compile.test.ts`,
//! checks the same parsers under web-tree-sitter. The two loaders disagree:
//! tree-sitter 0.27's `WasmStore` copies only part of some grammars' supertype
//! tables (#1738), so csharp, haskell and perl compile in the browser while the
//! Elixir NIF, the native addon and the CLI reject their queries. Nothing
//! noticed, because every query check ran under web-tree-sitter. Tree-sitter
//! 0.27.2 includes the fix; those three fail here with the earlier runtime.
//!
//! The fixture test runs with the rest of the suite. The built-parser one needs
//! every parser built first, so it is ignored here: `mise run test-queries` and
//! `.github/workflows/queries.yml` build them and run it, and `mise run
//! wasm-check <parser>` runs it for one, as a release does before publishing.

#![cfg(feature = "wasm")]

use lumis_wasm_runtime::{grammar_name, HighlightOptions, LanguageSpec, Runtime};
use std::path::{Path, PathBuf};

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()))
}

struct Parser {
    id: String,
    wasm_name: String,
    query_name: String,
    aliases: Vec<String>,
}

fn parsers() -> Vec<Parser> {
    let manifest: toml::Table = toml::from_str(&read(&workspace_root().join("languages.toml")))
        .expect("languages.toml parses");
    let entries = manifest["parsers"]
        .as_table()
        .expect("languages.toml has [parsers]");
    entries
        .iter()
        .map(|(id, entry)| {
            let field = |key: &str| entry.get(key).and_then(toml::Value::as_str);
            Parser {
                id: id.clone(),
                wasm_name: field("wasm_name")
                    .map_or_else(|| format!("tree-sitter-{id}"), str::to_string),
                query_name: field("query_name").unwrap_or(id).to_string(),
                aliases: entry
                    .get("aliases")
                    .and_then(toml::Value::as_array)
                    .map(|aliases| {
                        aliases
                            .iter()
                            .filter_map(toml::Value::as_str)
                            .map(str::to_string)
                            .collect()
                    })
                    .unwrap_or_default(),
            }
        })
        .collect()
}

/// The website keys samples by the filename before the first dot. A few predate
/// their language id, so an alias and the id without underscores are tried too.
fn sample_path(parser: &Parser) -> Option<PathBuf> {
    let samples = workspace_root().join("samples");
    let files: Vec<String> = std::fs::read_dir(&samples)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", samples.display()))
        .map(|entry| entry.expect("readable dir entry").file_name())
        .map(|name| name.to_string_lossy().into_owned())
        .filter(|name| name != "README.md" && name != "LICENSE.md")
        .collect();
    let stems = std::iter::once(parser.id.clone())
        .chain(parser.aliases.iter().cloned())
        .chain(std::iter::once(parser.id.replace('_', "")));
    for stem in stems {
        if let Some(file) = files
            .iter()
            .find(|name| name.split('.').next() == Some(stem.as_str()))
        {
            return Some(samples.join(file));
        }
    }
    None
}

/// Load `wasm` as `parser` into a runtime of its own, the way a language
/// package loads in the NIF, the addon and the CLI, then highlight its sample
/// so every query's predicates run against a real tree.
fn check(parser: &Parser, wasm: &Path) -> Result<(), String> {
    let wasm = std::fs::read(wasm).map_err(|error| error.to_string())?;
    let grammar_name = grammar_name(&wasm).map_err(|error| error.to_string())?;
    let queries = workspace_root()
        .join("queries/processed")
        .join(&parser.query_name);
    // Every other query is optional, but a language without highlights loads,
    // highlights its sample and passes having compiled nothing.
    if !queries.join("highlights.scm").exists() {
        return Err(format!(
            "no highlights query in queries/processed/{}",
            parser.query_name
        ));
    }
    let query = |kind: &str| {
        let path = queries.join(format!("{kind}.scm"));
        if path.exists() {
            read(&path)
        } else {
            String::new()
        }
    };

    // One per language, so no grammar shares a store with another and each
    // store is freed before the next language loads.
    let runtime = Runtime::with_worker_limit(1).map_err(|error| error.to_string())?;
    runtime
        .load_language(LanguageSpec {
            id: parser.id.clone(),
            aliases: parser.aliases.clone(),
            grammar_name,
            wasm,
            highlights: query("highlights"),
            injections: query("injections"),
            locals: query("locals"),
            brackets: query("brackets"),
        })
        .map_err(|error| error.to_string())?;

    let sample =
        sample_path(parser).ok_or_else(|| format!("no samples/ file for {}", parser.id))?;
    runtime
        .highlight_with(
            &read(&sample),
            &parser.id,
            &HighlightOptions {
                rainbow_brackets: true,
                ..HighlightOptions::default()
            },
        )
        .map_err(|error| error.to_string())?;
    Ok(())
}

/// Check each of `selected` against the parser `wasm` finds for it, and fail
/// once all have run if any did not pass.
fn check_all<'a>(
    selected: impl IntoIterator<Item = &'a Parser>,
    wasm: impl Fn(&Parser) -> Option<PathBuf>,
) {
    let mut checked = 0;
    let mut failures = Vec::new();
    for parser in selected {
        checked += 1;
        let result = match wasm(parser) {
            Some(path) => check(parser, &path),
            None => Err(format!(
                "no parser built from languages.toml; run mise run wasm-build {}",
                parser.wasm_name
            )),
        };
        match result {
            Ok(()) => eprintln!("ok: {}", parser.id),
            Err(message) => {
                eprintln!("FAILED: {}: {message}", parser.id);
                failures.push(format!("{}: {message}", parser.id));
            }
        }
    }
    // Never report success while checking nothing.
    assert!(checked > 0, "no languages selected");
    assert!(
        failures.is_empty(),
        "{} of {checked} languages failed in Wasmtime:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

/// The parsers in `fixtures/test-parsers`, which every runtime's suite loads.
#[test]
fn fixture_parsers_compile_their_queries_in_wasmtime() {
    let fixtures = workspace_root().join("fixtures/test-parsers");
    let fixture = |parser: &Parser| {
        let path = fixtures.join(format!("{}.wasm", parser.wasm_name));
        path.exists().then_some(path)
    };
    let parsers = parsers();
    check_all(
        parsers.iter().filter(|parser| fixture(parser).is_some()),
        fixture,
    );
}

/// Every parser `languages.toml` pins, or those `LUMIS_QUERY_LANGUAGES` names,
/// as `mise run wasm-build` built it.
#[test]
#[ignore = "needs built parsers: mise run test-queries"]
fn built_parsers_compile_their_queries_in_wasmtime() {
    let parsers = parsers();
    assert!(parsers.len() > 100, "languages.toml lists every parser");

    let only: Vec<String> = std::env::var("LUMIS_QUERY_LANGUAGES")
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .map(str::to_string)
        .collect();
    let unknown: Vec<&String> = only
        .iter()
        .filter(|id| !parsers.iter().any(|parser| &parser.id == *id))
        .collect();
    assert!(unknown.is_empty(), "unknown languages: {unknown:?}");

    let root = workspace_root();
    check_all(
        parsers
            .iter()
            .filter(|parser| only.is_empty() || only.contains(&parser.id)),
        |parser| {
            let path = root
                .join("tmp/wasm/build")
                .join(format!("{}.wasm", parser.wasm_name));
            path.exists().then_some(path)
        },
    );
}

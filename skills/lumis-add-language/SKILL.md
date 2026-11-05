---
name: lumis-add-language
description: Add a Tree-sitter grammar to the Lumis repository. Use when contributing a new language to leandrocp/lumis, covering the languages.toml entry, Rust wiring, highlight queries, a sample file, the regenerated LANGUAGES.md, and the pull request.
license: MIT
---

# Add a language to Lumis

`languages.toml` is the source of truth for every language. The Rust `Language` enum, language detection, the runtime catalogs, the JavaScript language modules, the WASM packages and both language tables are generated from it. Edit the source inputs and run the generators; do not hand-edit generated files.

Work from a clone of `leandrocp/lumis` on a topic branch, after `mise run setup`. `CONTRIBUTING.md` ("Adding a new language" and "languages.toml") is the reference; this skill follows it.

## 1. Check the grammar

- It is a Tree-sitter grammar in a public Git repository. Pin a full commit SHA, not a branch.
- Note whether a crate is published on crates.io for that revision. A crate is optional; without one Lumis vendors the C sources.
- Find highlight queries. Most languages use nvim-treesitter's (`[queries.default]` in `languages.toml`). Check `runtime/queries/<name>/` there first. If nvim-treesitter has none, use the grammar repository's own `queries/` directory.
- If the grammar has an external scanner, plan to load the built parser, not just compile it (step 7).

## 2. Add the parser entry

Add `[parsers.<name>]` to `languages.toml`, in alphabetical order:

```toml
[parsers.lean]
git = "https://github.com/Julian/tree-sitter-lean.git"
rev = "full_commit_sha"
version = "x.y.z"
aliases = ["lean4"]
globs = ["*.lean"]
```

`git`, `rev` and `version` are required. `version` is the crate version when `crate` is set, and always the version of the `@lumis-sh/wasm-<name>` package. The optional fields are documented under "Parser entry fields" in `CONTRIBUTING.md`: `crate`, `location` for multi-grammar repositories, `generate`, `aliases`, `globs`, `variant`, `display_name`, `emacs`, `shebang`, `query_name`, `wasm_name` and `feature`.

Give the entry the names a reader will type: the id, common aliases, and every file extension. Those become the published language table.

If the queries come from the grammar repository rather than nvim-treesitter, add a query source. Its key is the parser's `query_name` when the entry sets one, and the parser id otherwise:

```toml
[queries.lean]
git = "https://github.com/Julian/tree-sitter-lean.git"
rev = "full_commit_sha"
path = "queries"
```

If the language belongs in an existing bundle, add its id to the matching `[bundles.*]` array, one id per line. `full` is capped by `max_memory_mb`; a large parser goes in its `exclude` list.

## 3. Wire up Rust

In `crates/lumis/Cargo.toml`:

- With a crates.io package, add the optional dependency and the feature:

  ```toml
  tree-sitter-{lang} = { version = "x.y.z", optional = true }
  lang-{name} = ["dep:tree-sitter-{lang}", "lumis-core/lang-{name}"]
  ```

- Without one, add an empty feature: `lang-{name} = ["lumis-core/lang-{name}"]`.
- Add `lang-{name}` to `all-languages` in both `crates/lumis/Cargo.toml` and `crates/lumis-core/Cargo.toml`.

For a vendored parser (no crate, or an unreleased fix), add its compilation to `vendored_parsers()` in `crates/lumis/build.rs` and declare it in `crates/lumis/src/languages.rs`:

```rust
unsafe extern "C" {
    // ...existing declarations...
    #[cfg(feature = "lang-{name}")]
    fn tree_sitter_{name}() -> *const ();
}
```

Add the two lines to the existing `unsafe extern "C"` block, in alphabetical order; do not open a second block.

Then, in `crates/lumis/src/languages.rs`, add a `{NAME}_CONFIG` static `LazyLock<HighlightConfiguration>`, return it from the `config()` match under `#[cfg(feature = "lang-{name}")]`, and add a `test_{name}_config_loads` row to the config test table. Copy the shape of a neighbouring language.

## 4. Fetch the parser and queries

```sh
mise run langs-fetch-vendored-parsers {name}
cargo run --manifest-path crates/dev/Cargo.toml --no-default-features -- cargo-update-dep {name}
mise run cargo-update-features
mise run langs-sync-lockfiles
mise run langs-gen-catalog
mise run langs-fetch-queries {query_name}
mise run langs-preprocess-queries {query_name}
pnpm --filter @lumis-sh/lumis run build:generate
```

`{query_name}` is the entry's `query_name` if it sets one, otherwise the parser id. `langs-sync-lockfiles` re-resolves both `Cargo.lock` files so `--locked` builds keep working after a manifest change.

This writes `queries/upstream/{query_name}/`, the preprocessed `queries/processed/{query_name}/` that every runtime reads, `crates/lumis-wasm-runtime/src/catalog.rs`, the bundle feature lists, and the generated JavaScript modules.

Never edit `queries/upstream/` or `queries/processed/`. When a fetched query is wrong for the pinned grammar:

- `queries/override/{query_name}/{query}.scm` replaces the upstream query entirely.
- `queries/append/{query_name}/{query}.scm` adds patterns after it.

Say in a comment what upstream does and why it is wrong, then run `mise run langs-preprocess-queries {query_name}` again.

Bracket matching and rainbow brackets read `queries/brackets/default/brackets.scm`. If that default does not fit the grammar, add `queries/brackets/{query_name}/brackets.scm` with `@open`/`@close` pairs, and tag string or template delimiters with `(#set! rainbow.exclude)`. Use Neovim's query semantics: `#lua-match?` takes a Lua pattern, `#match?` a regex.

## 5. Add a sample and lookup cases

- Add `samples/{name}.{ext}` with representative code that exercises the grammar: comments, strings, numbers, definitions, calls and any injected languages. Tests, CI query checks and the website read it.
- Add the id, aliases and an extension to `fixtures/language-lookup.json`, which pins language lookup across runtimes.

## 6. Generate the language tables

```sh
mise run docs-gen-languages-md
```

This regenerates `LANGUAGES.md` and `docs/content/reference/languages.md`. Commit both. Nothing checks them for drift, so a stale table ships unnoticed. Size and memory columns stay empty until the parser is published and `mise run wasm-sizes {name}` measures it; that is expected for a new language.

If the language is excluded from `bundle-full`, add it to the list in `docs/content/languages.mdx` too.

## 7. Build and load the parser

```sh
mise run wasm-build {name}
mise run wasm-check {name}
```

The parser is not published yet, so stage it and point the CLI at the staged store to see the sample highlighted:

```sh
export LUMIS_DATA_DIR=$PWD/tmp/wasm/local
mise run wasm-stage {name}
cargo run -p lumis-cli -- highlight samples/{name}.{ext}
```

`wasm-check` loads the parser, parses the sample and runs every query under both web-tree-sitter and Wasmtime. A grammar that compiles but fails to load (`bad export type for 'tree_sitter_{name}_external_scanner_create'`) is usually pinned at a stale revision; check the revision before suspecting the toolchain.

## 8. Check and open the pull request

These are the checks CI runs for a language change, from `CONTRIBUTING.md`, `mise.toml` and the Rust, JavaScript, conformance and queries workflows:

- [ ] `cargo test --all-features` passes, including the new `test_{name}_config_loads`.
- [ ] `cargo clippy --all-features -- -D warnings` is clean.
- [ ] `mise run langs-check-catalog` passes: the runtime catalogs match `languages.toml` and `bundle-full` stays under its memory cap.
- [ ] `mise run test-queries` passes: every processed query compiles against the pinned grammar and the regexes are portable. At minimum, `mise run wasm-check {name}`.
- [ ] `mise run test-conformance` passes.
- [ ] `mise run fmt` leaves nothing to commit and `mise run lint` passes.
- [ ] Every generated file is committed: `languages.toml`, both Cargo manifests, `crates/lumis-wasm-runtime/src/catalog.rs`, both `Cargo.lock` files, `packages/javascript/lumis/bundles/*.ts` when a bundle changed, `queries/upstream/{query_name}/`, `queries/processed/{query_name}/`, any vendored parser sources, `samples/{name}.{ext}`, `fixtures/language-lookup.json`, `LANGUAGES.md` and `docs/content/reference/languages.md`.
- [ ] No changelog edit and no version bump. Changelogs are generated from Conventional Commits at release time, so the commit subject is the changelog entry: `feat(languages): add <Language> support`. Parser packages are published by the `wasm-release` workflow after merge.

Open the pull request with the grammar URL and revision, where the queries come from, any override or append patch and why, the bundle the language joined or was excluded from, and the commands you ran.

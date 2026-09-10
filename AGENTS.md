# AGENTS.md

## Project overview

Lumis is a multi-runtime syntax highlighter built around the same core workflow across Rust, CLI, Elixir, JavaScript, browser, and Java:

1. choose a language
2. choose a theme
3. choose a formatter
4. render highlighted output

This repository is a monorepo. Rust crates, JavaScript packages, Elixir bindings, generated assets, examples, the website, and the docs site all move together.

## Read this first

Before making changes, agents must read:

- `CONTRIBUTING.md`
- `RELEASE.md`
- `ARCHITECTURE.md`

Those files define the build pipeline, release model, generated artifacts, and how the repository is wired together. Do not guess when they already answer the question.

## Operating rules

### Keep the API aligned across runtimes

Lumis should present one mental model everywhere. Public APIs across languages should stay aligned in naming, argument shape, defaults, return behavior, formatter flow, and feature coverage.

The Rust implementation is the source of truth. When a cross-runtime API decision is unclear, follow Rust first and bring other runtimes into line with it instead of inventing runtime-specific behavior.

### A shared surface needs a checked-in manifest, or it drifts silently

"Keep the API aligned" is a rule nothing enforces. Two files do enforce it, each for one surface, and each read by a test in every runtime that has to offer it:

- `fixtures/formatter-options.json` — what the built-in formatters **accept**.
- `fixtures/formatter-helpers.json` — what a **custom** formatter can be built from.

The second exists because the first was the only one. Options were pinned, helpers were not, and the three helper modules reached 46 names between them with 11 in all three ([#1381](https://github.com/leandrocp/lumis/issues/1381)): JavaScript had grown 20 helpers Rust never got, and a multi-theme formatter could not be written in Elixir at all. Nothing failed, because a helper another runtime lacks raises no error where it does exist.

- **Add to the manifest first, then watch the runtimes go red.** That is the order that makes the gap visible instead of leaving it for an audit.
- **Every runtime's test checks both directions.** A capability in the manifest that the runtime lacks fails, and a public helper the manifest does not account for fails too. Only the second one catches a helper added to one runtime and nowhere else, which is how this drifted.
- **A capability is a thing a formatter can do, not a signature.** Runtimes keep their own argument shapes: Elixir returns a whole scope table where Rust takes a theme, because the NIF boundary is not free; JavaScript takes an attribute object where Rust takes a rendered string. Aligning the spelling of those would make the API worse.
- **`runtime_only` is for a signature the boundary dictates, and it carries the reason.** Elixir's tables and JavaScript's `encodeSource`/`decodeSourceSlice` are there; generic tag plumbing exported by accident is not, that is `deprecated`.
- **A new surface with the same shape gets the same treatment.** If a third module lands that every runtime must offer, it goes in a manifest before it goes in a second runtime.

### Reuse the Rust core instead of reimplementing it

Shared behavior belongs in one Rust crate that every runtime consumes. A second implementation of the same logic in another language, or in another Rust crate, is a divergence that will drift.

- Before writing logic in JavaScript, Elixir, the CLI, or a build script, check whether `crates/lumis-core`, `crates/lumis`, `crates/lumis-wasm-runtime`, or `crates/lumis-build` already owns it, and extend that crate instead.
- Prefer moving work to generation time in Rust over duplicating it per runtime. Query preprocessing is the model: `crates/lumis-build` converts Lua patterns once, and every runtime then reads byte-identical `.scm` files.
- When a runtime genuinely cannot call into Rust, such as the browser, the Rust crate still defines the behavior, and the port must be covered by a test that pins both against the same input.
- Two copies of the same algorithm require a test that pins them to each other, and deleting one copy is better still. `definitionHash` was computed in both `crates/dev` and a Python release script until the script was ported into `crates/dev`; "the CI job has no Rust toolchain" is a workflow line to add, not a reason to reimplement.

### Use the standard library and established packages

Do not hand-roll infrastructure that a standard library or maintained crate/package already provides. This applies especially to version requirements, URL handling, HTTP behavior, archive formats, hashing, locking, serialization, and protocol parsing.

- Check the standard library and existing workspace dependencies first.
- If they do not cover the requirement, add a focused, established dependency instead of growing a local partial implementation.
- Custom code is justified only when available libraries cannot express the required behavior. Document that gap and test the compatibility boundary.
- Delegate package discovery and version-range resolution to the package registry and CDN. Lumis validates the resolved result and caches it; it does not implement another package manager.

### Keep environment variables few, and resolve each one once

An environment variable is global mutable state that no signature declares and no type checks. Prefer a function argument, a `mise` task `env`, a config file, or a CLI flag with clap's `env()` attribute, which puts the variable in `--help` for free. `LUMIS_DATA_DIR` and `LUMIS_CONFIG` are declared that way in `crates/lumis-cli/src/main.rs` and are the pattern to copy.

Before adding one, the bar is:

- **It cannot be a parameter.** A value the caller already has does not become an environment variable to avoid threading it through.
- **One place decides what it means.** Reading it in a few places to honor an override is fine; computing a default from it in a few places is not. Resolve it once, then pass the result inward. Five sites have five chances to disagree, and the fifth will.
- **A row in `CONTRIBUTING.md`.** Ten of thirteen `LUMIS_*` variables were written down nowhere, so the only way to learn one existed was to grep for it.
- **A name that is not already taken.** `LUMIS_BUILD` and `LUMIS_USE_LEGACY_ARTIFACTS` keep `rustler_precompiled`'s spelling rather than inventing a local one.

Test-only knobs belong in the test harness or the `mise` task, not in library code. `benchmarks/mise.toml` is the model: `[env]` for what the whole suite shares, per-task `env` for the rest, and nothing outside `benchmarks/` reads a `BENCH_*` variable.

### Data and config directories resolve identically on every runtime

The CLI, the Node addon, the Elixir NIF and the browser must agree on where Lumis reads and writes, or the same machine grows two stores and downloads every parser twice.

- **One function owns the data directory.** `lumis_wasm_runtime::store::default_data_dir` calls `etcetera` once; `resolve_data_dir` layers an explicit argument and then `LUMIS_DATA_DIR` over it. Every Rust caller goes through those, and `store.rs` holds the only `env::var_os("LUMIS_DATA_DIR")` in the tree. JavaScript reads the variable in three places, but only ever to honor an override that is already set; none of them computes a default.
- **An empty value counts as unset.** `""` is falsy in JavaScript but a valid `PathBuf` in Rust that resolves to the current directory, so an unfiltered `var_os` would scatter `parsers/` and `compiled/` wherever the process started while JavaScript quietly used the platform default. `named_directory` filters it on the Rust side. Any new path variable needs the same treatment.
- **Config resolves through the same strategy.** Today only the CLI has a config file, and `crates/lumis-cli/src/config.rs` uses `etcetera`'s `config_dir()`. A second runtime that grows one resolves it the same way instead of re-deriving `XDG_CONFIG_HOME` and `%APPDATA%` by hand.
- **`choose_base_strategy` is the convention:** XDG everywhere except Windows, where it is `%APPDATA%`. It is `Xdg` on macOS as well. `etcetera` reserves `Apple` for `choose_native_strategy`, which Lumis does not use, and that distinction is the whole of the bug below.
- **A runtime that cannot call Rust asks one that can.** Node resolves the default through the addon's `defaultDataDir()` rather than computing its own, via `loadAddon`, which answers even when the process has selected the Wasm runtime.
- **Any remaining port needs a parity test**, per the rule above about reimplementation. `packages/javascript/lumis/test/data-dir-parity.test.ts` pins the no-addon fallback against the addon across four `XDG_DATA_HOME` states.

No JavaScript library matches `etcetera`, so reaching for one does not solve this. `env-paths` uses the Apple convention on macOS, `%LOCALAPPDATA%` on Windows, and appends a `-nodejs` suffix; adopting it would relabel the divergence rather than remove it. The library does the work once, in Rust, and everything else asks.

This is written down because it shipped ([#1264](https://github.com/leandrocp/lumis/pull/1264)). `default_data_dir` existed five times, the TypeScript copy picked Apple on macOS and `%LOCALAPPDATA%` on Windows, and the native and Wasm runtimes of the same npm package therefore used different stores. One developer machine held both, 2 parsers in one and 45 in the other. Nothing failed. Each runtime worked correctly against its own directory, and the only symptom was a download that had already happened once. A divergence whose whole cost is duplicated work raises no error, so a test has to be what catches it.

### Highlighting loads what a document needs, in one pass

A parser is a WebAssembly module fetched from a registry and executed in the
host process. Highlighting resolves, downloads, verifies and loads whatever a
document turns out to name, including languages injected inside it, and caches
them for every later request. Nothing has to be declared up front.

Three rules hold that together, and a change that breaks any of them is wrong:

- **One pass.** An injected language is loaded during the walk that discovered
  it, not by highlighting the document twice or by scanning it first. That is
  why `Runtime::highlight` takes a callback that can load: the walk descends
  into the language it just fetched and finds whatever *that* contains, however
  deep the nesting goes.
- **A failure costs one block, not the document.** A thousand-line Markdown file
  with one fenced block in an unpublished language still highlights; that block
  stays plain. Only the root language failing is an error.
- **One implementation.** `LanguageStore` resolves and caches; `Runtime` loads
  and highlights. The CLI, the Elixir NIF and the Node addon all call them, so
  none of them can drift.

Browsers are the exception, and only because loading is asynchronous there:
`web-tree-sitter` cannot fetch a parser inside a synchronous walk, so an
injected language has to be loaded before the document mentioning it. Node uses
the native addon precisely so it does not inherit that limit. Do not "fix" the
browser by making the other runtimes match it.

`ARCHITECTURE.md` has the full reasoning, including why pre-loading is an
optimization rather than a requirement. A cold parser costs a download and then
a Wasmtime compile, and the compile is the larger half: seven parsers already on
disk take about 8 s to compile and 1.3 s once `compiled/` is warm. `load` exists
to move both off the first request, not to gate anything.

### Route work through `mise`

`mise.toml` is the control plane for this repository. It defines tasks without pinning runtime versions, so local commands use the developer's installed Rust, JavaScript, Elixir, and other toolchains.

- Prefer `mise run` tasks over raw tool invocations.
- If a repeated workflow does not have a task yet, add or update `mise.toml` instead of documenting an ad hoc command sequence.
- Use the existing top-level entry points whenever possible: `mise run setup`, `mise run fmt`, `mise run lint`, `mise run test`, `mise run test-conformance`, `mise run docs`, `mise run dev`, and `mise run docs-site`.

### Theme extraction and Neovim plugins

Theme extraction uses Neovim's built-in plugin manager, `vim.pack`.

- Read the upstream docs before changing this flow: <https://neovim.io/doc/user/pack/#_plugin-manager>
- `vim.pack` manages plugins under `stdpath("data") .. "/site/pack/core/opt"`. In this repo, `themes/init.lua` sets `XDG_DATA_HOME` to the repo-local `nvim/data`, so extraction must not touch a user's normal Neovim install.
- Do not manually mutate `vim.pack` managed plugin repositories with raw `git` commands unless there is no supported `vim.pack` API for the operation.
- `vim.pack.add()` installs missing plugins and makes them available in the current session, but it does not update an existing plugin's revision.
- Use `vim.pack.update()` when theme generation needs fresh upstream plugin revisions. Use non-interactive options in automation.
- Keep plugin directory names explicit and collision-safe. The name field controls the managed directory name, and case-only repo differences can collide on macOS filesystems.

### Treat structural changes as design work

Changes to repository layout, package boundaries, generated artifact flow, release mechanics, shared API shape, or build orchestration are structural changes.

Those changes require review. They also require checking whether `CONTRIBUTING.md`, `RELEASE.md`, and `ARCHITECTURE.md` still describe reality. If they do not, update them in the same change.

### Rust rules are stricter, not looser

Rust is the reference implementation and must follow the Rust API Guidelines:

- <https://rust-lang.github.io/api-guidelines/checklist.html>

Prefer guideline-compliant naming, error behavior, builder patterns, documentation, and trait usage. Do not introduce a Rust API shortcut that other runtimes cannot sensibly mirror.

### Parse TypeScript boundaries into domain types

`unknown` belongs at genuinely untrusted boundaries, not throughout the implementation. Do not widen a known structure to `Record<string, unknown>` or use assertions to make parsed JSON, TOML, dynamic imports, native data, or other external values look typed. Validate the input and construct a named or discriminated domain value before passing it inward. When another API already defines the shape, derive it with utilities such as `Parameters`, `ReturnType`, or `Pick` instead of recreating it as a generic property bag.

### Tree-sitter is the driver

Lumis is a Tree-sitter-based syntax highlighter. Language behavior must respect Tree-sitter's design, mechanics, and query model rather than bypassing them with ad hoc text scanning.

- Use Tree-sitter parsers, syntax trees, captures, predicates, and query metadata as the source of truth for language-aware behavior.
- Treat `.scm` query files as executable language behavior, not just generated assets to carry around.
- For cross-runtime features, define the behavior in Rust first, then align JavaScript, Elixir, browser, Java, and CLI behavior to the Rust implementation.
- Do not replace language-specific query semantics with generic string scanning unless it is an explicit fallback for plaintext or unavailable parser/query data.

### Neovim is the reference for query semantics

Lumis queries are fetched from <https://github.com/nvim-treesitter/nvim-treesitter>, so those `.scm` files mean whatever Neovim's highlighting engine says they mean. Neovim, not Tree-sitter's documentation and not a plausible reading of the pattern, decides the intended behavior.

- Before changing predicate handling, query preprocessing, or capture resolution, read the upstream implementation. Clone it locally: `git clone --depth 1 --filter=blob:none --sparse https://github.com/neovim/neovim.git "$(mktemp -d)/nvim"`, then `git sparse-checkout set runtime/lua/vim/treesitter`.
- The predicate handlers live in `runtime/lua/vim/treesitter/query.lua`. Two of them are easy to conflate:
  - `#lua-match?` is `string.find(node_text, pattern)`, so its argument is a **Lua 5.1 pattern**, matched unanchored unless it starts with `^`.
  - `#match?` (aliased as `#vim-match?`) is `vim.regex('\v' .. pattern)`, a **Vim very-magic regex**. Lumis instead evaluates `#match?` with Tree-sitter's own implementation: the `regex` crate in Rust and `RegExp` in `web-tree-sitter`.
- A Lua pattern is not a regex. `-` is the lazy `*` only after a pattern item, `^` anchors only at the start, `$` only at the end, `.` matches newlines, and `%d`/`%s`/`%w` are ASCII in the C locale. `crates/lumis-build` owns that translation; extend it rather than hand-editing generated queries.
- Any translated pattern must be valid **and equivalent** in both the `regex` crate and JavaScript `RegExp` with no flags. The two disagree on nested character classes, `\d`/`\w`/`\s` width, and inline `(?i)` groups. `crates/lumis-build/tests/processed_queries.rs` and `packages/javascript/lumis/test/query-patterns.test.ts` enforce this over the whole corpus and must not be allowed to skip languages.
- Settle a question about pattern semantics by running it, not by reasoning about it: `nvim --headless -c 'lua print(string.find(subject, pattern))' -c q`.
- The converter stays faithful to Neovim even when upstream is wrong. Fix an authoring mistake in `queries/override/` or `queries/append/` instead, where it is visible in review, and say in a comment what upstream does and why it is wrong.

### A test that cannot fail is worse than no test

A test that skips silently reports the same green as a test that verified something. That is how the defects in `REVIEW.md` §1 shipped: the only per-language query check `return`ed early for 77 of 115 languages.

- Never `return` or `continue` out of a test body to handle a missing prerequisite. Fail, or record the gap in a checked-in file that the test enforces.
- A gap that genuinely cannot be closed yet gets an explicit waiver that can only shrink. The test must fail on an undeclared gap **and** on a waiver entry that is no longer needed. `packages/javascript/lumis/test/unverified-parsers.json` is the pattern.
- Assert corpus size. `expect(patterns.length).toBeGreaterThan(200)` is what catches a discovery bug that silently finds nothing.
- Prove a new guard fails: inject the defect it is meant to catch, watch it go red, then revert. A guard that has never failed has not been tested.
- Do not let published artifacts gate correctness checks. Build what you need from the pinned source instead, as `mise run test-queries` does, otherwise coverage silently tracks the release cycle.

### What the workspace never resolves, it never checks

The root `Cargo.toml` points `path` at every lumis crate and patches the rest through `[patch.crates-io]`. Nearly every build in this repository therefore compiles against the working tree, and the `version` requirement sitting beside each `path` is inert until the crate reaches crates.io. Nothing local can be green or red about it. The lone exception is `crates/autumnus`, which sits in its own workspace and depends on `lumis` through the registry.

That is not a gap in the test suite, it is a gap in what a test *could* observe, so it needs a check that reads the manifests rather than builds them. `mise run check-crate-deps` requires each requirement to equal the depended-on crate's version in this repository, and runs in `mise run lint`, in Rust CI, and again before `cargo publish`. `mise run release-prepare` calls it with `--fix`, so releases stay in step without a checklist item.

Two things this cost, both worth recognising elsewhere:

- **A too-loose requirement disables the tooling that would maintain it.** `cargo set-version -p lumis-core 2.4.0` rewrites dependents whose requirement the new version falls outside. With `lumis-core = "2"` there was nothing to rewrite, so it stayed `"2"` across three minor releases until `lumis` called an API that `2.0.0` did not have ([#1118](https://github.com/leandrocp/lumis/issues/1118)). Spell requirements in full.
- **A fresh resolve is not a reproduction.** The failure needed an existing `Cargo.lock`; a new project picked the newest match and worked. When a bug report says "fails on Windows" and the code has no target-specific path, reproduce the reporter's *resolution*, not their OS.

### A vendored file records every local change in its own header

`crates/lumis-wasm-runtime/src/tree_sitter_highlight.rs` is upstream
tree-sitter's `highlight.rs` with Lumis deltas applied in place. Its header
carries a "Current local deltas" list, and that list is the only thing that makes
the next upstream sync a review rather than an archaeology exercise: a diff
against upstream shows *what* changed, never *why*, and cannot distinguish a
deliberate Lumis behaviour from a merge accident.

- **Editing a vendored file without adding to its header is incomplete work.**
  Not a follow-up, not a separate commit. Reviewers cannot tell an intentional
  delta from a mistake, and neither can whoever re-syncs the file.
- **Say why upstream cannot serve, not what the code does.** "`@injection.filename`
  resolves an injected language from a path, as Neovim's
  `LanguageTree:_get_injection` does. Upstream has no equivalent" is the useful
  entry. "Added a match arm" is not.
- **A change inside an existing delta extends that entry rather than adding one.**
  The line-boundary fix to `#offset!` belongs under the `#offset!` bullet,
  because someone re-syncing needs the whole of that feature in one place.
- **Note any new dependency the file gains.** Referencing `lumis_core` from a
  vendored file is a coupling upstream does not have, so the header says so.
- **Prefer shrinking the delta to growing it.** Reuse a shape the file already
  has rather than introducing a parallel one; the filename capture reuses the
  content capture's `#offset!` handling for exactly this reason.

This applies to any file carrying a "Vendored from" header, not just this one.

### Comments are a last resort

Do not narrate code. A comment is justified only when the code is genuinely hard to follow, or when it does something a reader would not expect and would otherwise "fix". Everything else should be carried by naming and structure.

Worth keeping, because the behaviour is surprising:

```rust
// Windows refuses to replace a file another handle still has open.
```

```elixir
# Rustler encodes `Result<(), String>` as `{:ok, {}}`, not `:ok`.
```

Not worth keeping, because the code already says it:

```elixir
# Read from the copy vendored in this repository, so the demo runs offline.
@source_path Path.expand("../../../benchmarks/webgpu_compute_reduce.html", __DIR__)
```

Rationale that is about the change rather than the code belongs in the commit message. `mise.toml` takes no comments at all.

### Formatting and linting are enforced everywhere

Every file the repo authors is formatted and linted, including generated ones. `mise run fmt` formats every language, `mise run lint` checks them. Run those before pushing; CI only checks.

There is no per-package path list. `fmt-js` is `oxfmt "**/*.{ts,tsx,mjs,cjs,js,jsx}"` from the repo root and `lint-js` is `oxlint --deny-warnings --report-unused-disable-directives .` beside it, so a new package, script, example or test directory is covered the day it is added, whether or not the file has been staged yet.

**Generated files are not an exception.** A generator writes its output and then formats it, so regenerating and format-checking agree. If you add a generator, format what it emits.

`.oxfmtrc.json` holds the only exclusions, and each needs a reason to be there:

- vendored parsers and vendored site assets — not ours to restyle
- `dist/` — bundler output, which the next build would rewrite anyway
- `samples/` and `fixtures/` — corpora whose exact bytes are the test
- `benchmarks/shadcn_sidebar.tsx` — a vendored third-party file the showcase pins by SHA-256, so
  formatting it would break the pin. The other vendored showcase documents are not JavaScript and
  never matched the glob
- `packages/javascript/themes/themes/` — 246 modules holding one long `JSON.stringify` line each; formatting them costs 28k lines for no readability, since nobody reads generated theme data
- `lumis/langs/`, `lumis/src/generated/`, `lumis/src/tree-sitter-wasm.ts` — build output that is never committed. oxfmt reads only the `.gitignore` in its working directory, so paths ignored by a nested one have to be named here.

This replaced a per-package `fmt:check` that named `src/` only. Every `test/`, `scripts/` and `examples/` directory in the repo had therefore never been formatted, and nobody could tell, because the check was green. When adding a formatter or a linter, make the default "everything" and subtract; never name a directory and hope the list is maintained.

`oxlint` runs with `--deny-warnings`, so a warning fails the build. Silence a genuinely intentional one at the line with `// oxlint-disable-next-line <rule> -- <why>`; do not let it sit in the output. `--report-unused-disable-directives` runs too, so a silence that stops being needed fails rather than lingering.

**Every linter runs at its strict setting.** clippy at `clippy::pedantic`, oxlint with `correctness`, `suspicious`, `perf` and `pedantic` as errors, credo with `--strict`, selene with warnings fatal. `CONTRIBUTING.md` has the table and where each configuration lives.

Every runtime implemented here also caps cyclomatic complexity at 20: oxlint's classic `eslint/complexity` rule for JavaScript, Credo's `CyclomaticComplexity` check for Elixir, and pinned Lizard for every Rust source file. The sole Rust whitelist entry is the exact upstream-vendored Tree-sitter iterator; do not broaden it or add authored code to it.

A lint the repository has decided against is a waiver, not a line-level silence: it goes in `[workspace.lints.clippy]` or the `rules` block of `.oxlintrc.json`, with the reason and the number of sites it fired on. Those lists shrink; an addition needs the same justification the existing entries carry. Prefer configuring a rule over disabling it — `eqeqeq` keeps `== null`, `max-depth` is set to the deepest block the tree actually has, `prefer-nullish-coalescing` skips `if` statements whose guard is truthiness rather than nullishness.

**An autofix is a proposed edit, not a result.** Read what `--fix` wrote before keeping it. In this repository `unicorn/prefer-code-point` rewrote surrogate-pair validation that has to work in UTF-16 code units, `unicorn/no-useless-undefined` dropped arguments that were required, `oxc/no-map-spread` proposed an in-place `Object.assign`, and `prefer-nullish-coalescing` turned an `if (!x)` guard into `??=`, which stops treating `""` as absent. Build and test after a sweep; two of those four only surfaced as a `tsc` error, and the other two would not have surfaced at all.

### Validate workflow changes with `actionlint` before pushing

Run `mise run lint-workflows` after **any** edit under `.github/workflows/`. It is also part of `mise run lint`, and CI runs it, but CI finding it costs a full round trip.

This matters most for bulk edits. Rewriting action inputs with `sed` or a regex is exactly how a `with:` key gets separated from the block it belongs to:

```yaml
- uses: jdx/mise-action@v4
    working_directory: benchmarks   # `with:` removed -> invalid YAML
```

That file no longer parses, and the failure surfaces as an unrelated-looking lint error rather than at the step you edited. Two rules follow:

- A workflow edit is not done until `actionlint` passes.
- When changing an action's inputs, check **every** call site rather than the one that prompted the change. Inputs differ per job — `install: true` may be droppable in one place while `install: false` and `working_directory` next door are load-bearing.

Version pins in workflows deserve the same suspicion. `jdx/mise-action`'s `version:` input pins **mise itself**, not the action, and dropping it can break tool installs that the pinned mise handled.

### Parser WASM is compiled by Tree-sitter's WASI SDK, not by Emscripten

`tree-sitter build --wasm` used `emcc` through CLI 0.25. Since 0.26 it downloads its own `wasi-sdk` into `~/.cache/tree-sitter/wasi-sdk` and compiles with the clang inside it, so Emscripten is not on the parser build path at all and having `emcc` on `PATH` changes nothing about the artifact. The whole invocation, from `Loader::compile_parser_to_wasm`, is:

```sh
clang --target=wasm32-unknown-wasi -fPIC -shared -Os \
  -Wl,--export=tree_sitter_<lang> -Wl,--allow-undefined -Wl,--no-entry \
  -nostdlib -fno-exceptions -fvisibility=hidden -I . parser.c scanner.c
```

Those flags are hardcoded. `TREE_SITTER_WASI_SDK_PATH` selects a different SDK directory and is the only knob; there is no `CFLAGS` escape hatch, and the SDK version is pinned per CLI release (`crates/loader/wasi-sdk-version`), so bumping `tree-sitter` in `mise.toml` can silently change the compiler.

Emscripten did not stop mattering, it stopped mattering *here*, and upstream draws the line precisely ([maxbrunsfeld on #4393](https://github.com/tree-sitter/tree-sitter/pull/4393#issuecomment-2831035549)): it is still required to build the Tree-sitter **web binding**, the JavaScript-to-WASM glue published as `web-tree-sitter`, and it is not required to compile **parsers**.

Lumis sits on the far side of that line. It depends on `web-tree-sitter` from npm, embeds that package's prebuilt `web-tree-sitter.wasm` verbatim through `scripts/build-runtime-wasm.ts`, and edits the shipped bundle as text in `scripts/patch-web-tree-sitter-bundle.mjs`. Neither compiles anything. So no part of this repository installs or pins Emscripten: `LUMIS_EMSDK_VERSION`, the `setup-emsdk` steps in `wasm-release.yml` and `queries.yml`, the `emcc` check in `mise run setup`, and the `EMSDK_PYTHON`/`EMCC_DEBUG` plumbing in `crates/dev` were all removed once the compiler moved, having installed a toolchain nothing invoked.

That changes the day Lumis builds `web-tree-sitter` from source instead of patching the published bundle. Until then, if you reach for `emcc` to explain a WASM problem, first check that anything still calls it.

One failure in particular used to be blamed on Emscripten and is worth knowing on its own terms:

```
bad export type for 'tree_sitter_<lang>_external_scanner_create': undefined
```

A parser that triggers it **compiles cleanly** and only fails when something loads it, and only when the grammar has an external scanner. So a green build proves nothing — load the parser.

It was reported upstream as an Emscripten bug (<https://github.com/tree-sitter/tree-sitter/issues/5037>), and it is not one. Measured on `tree-sitter-hcl` back when Emscripten was still installed here, holding the toolchain fixed and varying only the grammar revision:

| Grammar revision | emsdk 4.0.15 | emsdk 6.0.5 |
| --- | --- | --- |
| `636dbe70`, pinned in `languages.toml` | fails to load | fails to load |
| `64ad6278`, what the published package ships | loads | loads |

The revision moves the result and the toolchain does not, so the cause is a **stale grammar revision**. When a scanner-bearing language fails to load, check whether `languages.toml` has fallen behind the revision the published package was built from before suspecting the compiler.

## Documentation is part of the change

Docs, specs, and examples are not cleanup work for later. They are part of the feature.

- Keep README files, package docs, examples, and generated references consistent with the shipped behavior.
- If a change affects public API, behavior, configuration, or generated outputs, update the relevant docs and examples in the same PR.
- If writing needs polish, use the available `humanizer` skill before finishing.

Prefer concrete explanations over marketing language. Show the real API. Keep examples runnable.

One command per line in a shell block. `cd packages/elixir/lumis && LUMIS_BUILD=1 iex -S mix`
reads as one step but is two, and a reader who wants only the second has to take
the line apart. Better still, check whether the `cd` is needed at all: `node
packages/javascript/lumis/test.mjs` runs from the repo root, because Node
resolves imports from the file rather than the working directory.

### Public docs describe the runtime, not what is under it

Rust being the reference implementation is a rule for contributors, not a fact
users need. Someone reading `Lumis.Formatter.ANSI` cannot act on "this calls into
Rust", cannot see the crate, and would not do anything differently if the answer
were Zig. Every sentence of it is bloat in a place with a low budget for it.

- **A public doc says what the function does and what to do with it.** Not what
  it delegates to, not which crate owns the logic, not that a table crosses a
  boundary once instead of per token. `styles/1` returns the whole table because
  a reader is told to build it once and read it per token, which is a fact about
  their loop; whether that is Rust, a `:persistent_term` or a literal is not.
- **Name the behaviour, not the implementation, when explaining a difference.**
  "`Map.get(theme.highlights, scope)` misses the fallbacks `:terminal` applies"
  is actionable. "Scope resolution belongs in Rust" is trivia that happens to
  sit where the actionable sentence should have been.
- **The Rust origin belongs where a contributor reads it**: this file, the
  binding crate's own doc comments, a commit message. `html_span_attrs` in
  `lumis_nif` explains the call-frequency split at length, correctly — that is a
  contributor's file.
- **Cross-runtime pointers are not this.** "The counterpart of `Language::guess`
  in Rust and `guessLanguage()` in JavaScript" helps someone moving between the
  packages Lumis publishes, and stays.

This applies to every published surface: package docs, READMEs, `usage-rules.md`,
and `docs/content/`. It came out of an ANSI helper module whose moduledoc opened
by explaining that its functions call Rust, before saying what any of them did.

### READMEs stay small, detail lives in the docs site

READMEs are entry points, not manuals. Keep them small, direct, and targeted: the minimal usage to get started, then a link to the relevant page under `docs/content/`.

- Put detailed guides, option references, multi-runtime examples, and edge cases in `docs/content/`, not in READMEs.
- When a feature spans runtimes, document it once in `docs/content/` using runtime `Tabs` (`<Tabs groupId="runtime" ...>` with JavaScript, Rust, Elixir, Java, CLI), instead of repeating it in each package README.
- A README mention of a new capability should be one or two lines plus a link to the docs page (use the published `https://lumis.sh/docs/<slug>` URL).
- Internal docs cross-links use slug form, e.g. `/themes/css-files#build-custom-css`.

## Website and docs site

This repo ships two documentation surfaces:

- `website/` for the main site and demos
- `docs/` for the Docusaurus docs app published under `/docs`

Code changes that affect user-facing behavior often need updates in one or both places. Do not treat the website and docs site as downstream afterthoughts.

## Generated artifacts and shared data

Large parts of the repository are generated from shared inputs such as `languages.toml`, `highlights.toml`, themes, queries, and conformance fixtures.

- Edit the source inputs, not generated outputs, unless the generated file is the intended source.
- For query changes, treat `queries/upstream/` as fetched source material, `queries/override/` as full replacements, and `queries/append/` as additive local patches.
- Regenerate checked-in artifacts with the documented `mise run` workflows.
- When generated or vendored files are added, removed, moved, or reclassified, update `.gitattributes` in the same change. For content-only updates, verify that its existing patterns still cover every affected path.
- Keep Rust, JavaScript, Elixir, docs, fixtures, and generated metadata in sync.

## Verification

Match verification to the scope of the change, but do not stop at a partial check when a broader shared workflow is affected.

Common entry points:

- `mise run fmt`
- `mise run lint`
- `mise run test`
- `mise run test-conformance`
- `mise run docs`

For language, theme, query, docs-generation, or bundle changes, run the relevant regeneration commands described in `CONTRIBUTING.md` and commit the resulting tracked files.

## Release discipline

Releases are prepared by `release-prepare.yml` on every push to `main`, which opens or updates one pull request per package that needs one, and published when a maintainer merges one: `release-tag.yml` turns the merge commit into the package tag the publish workflows run from. `mise run release-prepare` does the same preparation locally.

- Do not hand-edit versions or changelog release sections in normal feature work.
- Follow `RELEASE.md` for release-specific tasks.
- Write the release commit subject as `chore(release): <package> <version>` and nothing else. The `(#1234)` suffix a squash merge appends is the one addition `release-tag.yml` tolerates. That subject is the whole interface: CI skips on the prefix, and `release-tag.yml` reads the package and version out of it to decide what to publish. A different spelling runs every workflow against a change that was already green, and ships nothing.
- Adding a releasable package is a row in `mise run release-packages`. Nothing else holds a per-package list, so a second copy of that table is a bug waiting to drift.
- If a change affects packaging, tags, publish behavior, or release metadata expectations, treat it as a release-sensitive change and review the release docs before touching anything.

## Commit messages

- Use Conventional Commits for git commit messages.
- Prefer a clear type and scope when they help, for example `fix(rust): ...` or `docs(website): ...`.
- Keep the subject concise and descriptive.

## Practical default

When in doubt:

1. read the three core docs
2. make the smallest correct change
3. keep Rust as the reference
4. run the work through `mise`
5. update docs, examples, website, and docs site before calling the work done

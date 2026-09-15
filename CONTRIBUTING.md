# Contributing

See `ARCHITECTURE.md` for how the crates, packages, website, and build pipeline fit together.

## Table of contents

- [Getting started](#getting-started)
- [Testing](#testing)
- [Linting](#linting)
- [Releases](#releases)
- [Benchmarks](#benchmarks)
- [Environment variables](#environment-variables)
- [Configuration files](#configuration-files)
  - [highlights.toml](#highlightstoml)
  - [languages.toml](#languagestoml)
- [Languages and parsers](#languages-and-parsers)
  - [Adding a new language](#adding-a-new-language)
  - [Updating parsers](#updating-parsers)
  - [Updating queries](#updating-queries)
  - [Building WASMs](#building-wasms)
  - [Running a runtime locally](#running-a-runtime-locally)
  - [Parsers in CI](#parsers-in-ci)
- [Themes](#themes)
  - [Adding a new theme](#adding-a-new-theme)
  - [Updating themes](#updating-themes)

## Getting started

This project uses [mise](https://mise.jdx.dev/) as its task runner. The root configuration intentionally does not pin tool versions; tasks use the Rust, JavaScript, Elixir, and other toolchains already available in your environment. After cloning the repo, run:

```sh
mise run setup
```

This installs the Rust, JS, and Elixir dependencies and checks the required tools.

## Testing

Run the standard Rust, Elixir, and JavaScript test suites with:

```sh
mise run test
```

That builds the Node addon first and runs the whole `@lumis-sh/lumis` suite twice,
once per runtime, because Node highlights through the addon by default and falls
back to `web-tree-sitter` where none is built. It also stages parsers into
`target/test-parsers` and points the dependent packages at them, so nothing in
the suite depends on what is currently published to npm.

Cross-runtime output compatibility and browser support have dedicated suites:

```sh
mise run test-conformance
mise run test-conformance-browser
```

`test-conformance` includes the browser suite. Run an individual conformance task to check one runtime in isolation:

```sh
mise run test-conformance-rust
mise run test-conformance-cli
mise run test-conformance-javascript-native
mise run test-conformance-javascript-wasm
mise run test-conformance-browser
mise run test-conformance-elixir
```

Node has two of them because it has two runtimes: the Wasmtime addon it uses by
default, and `web-tree-sitter` where no addon is built. Both must produce the
same bytes as Rust.

### The formatter option manifest

Conformance pins what the formatters **output**. `fixtures/formatter-options.json`
pins what they **accept**, which conformance cannot see: a Rust builder can grow
a field and ship while Elixir, JavaScript and the CLI silently lack it. That is
how `rainbow_brackets` reached four runtimes and no options table, and how
JavaScript's `terminal` went without `background` and `width`.

Each implementation in this repository has a test that reads the manifest:

| Runtime | Test | How it checks |
| --- | --- | --- |
| Rust | `crates/lumis/tests/formatter_options.rs` | calls every setter by name, so a missing one fails to compile |
| CLI | `crates/lumis-cli/tests/formatter_options.rs` | reads `lumis highlight --help`, so it sees the flags clap parses |
| Elixir | `packages/elixir/lumis/test/formatter_options_test.exs` | reads the keys off the NimbleOptions schema |
| JavaScript | `packages/javascript/lumis/test/formatter-options.test.ts` | `Required<Options>` literals, so a missing field fails type-aware lint |

These are the four implementations in this repository. Java is part of the
same parity target, but its implementation lives in `lumis4j`; D26 in
`API_DRIFT.md` tracks making that repository consume this contract too.

Adding a formatter option means adding it to the manifest first and watching
four runtimes go red. `waived` is the escape hatch for something a runtime
genuinely cannot offer; the tests fail on a waiver that is no longer needed, so
it can only shrink. It is currently empty.

The browser task installs the required Chromium, Firefox, and WebKit builds before running. CI runs these six tasks as independent parallel jobs.

## Linting

```sh
mise run lint
```

Every linter runs at its strict setting, and every runtime has one. What that
means per language, and where the configuration lives:

| Runtime | Linter | Strict setting | Configuration |
| --- | --- | --- | --- |
| Rust | clippy, lizard | clippy's `pedantic` group plus `rust_2018_idioms` and `unreachable_pub`, all denied; cognitive complexity at most 10; classic cyclomatic complexity at most 20 | `[workspace.lints]` in the root `Cargo.toml`, `clippy.toml`, `.lizard-whitelist` |
| JavaScript, TypeScript | oxlint | the `correctness`, `suspicious`, `perf` and `pedantic` categories, as errors; classic cyclomatic complexity at most 9 | `.oxlintrc.json` |
| Elixir | credo | `mix credo --strict`; cyclomatic complexity at most 9 | `packages/elixir/lumis/.credo.exs` |
| Lua | selene | every lint selene ships, warnings included | `selene.toml`, `neovim.yml` |
| GitHub Actions | actionlint | its default, which is already strict | `.github/actionlint.yaml` |

Every runtime implemented in this repository enforces a complexity ceiling with
its language-specific analyzer, at the number the other projects in this family
hold: **9**, classic McCabe, for Elixir and for JavaScript.

Rust uses clippy's cognitive complexity at **10** rather than a McCabe ceiling of
9, because `?` makes every fallible call a cyclomatic branch and would score a
flat decoder the same as a deeply nested function. The lizard check still applies
classic cyclomatic complexity at 20 on top, so each metric covers what the other
is blind to. It reads every `*.rs` file, including crates outside the workspace;
its only whitelist entry is an exact function in the upstream-vendored
Tree-sitter highlighter, and the file that function lives in carries the matching
`#[allow(clippy::cognitive_complexity)]` for the same reason.

Two rules hold this together, and both come from what the checks used to miss:

- **Every file the repo authors is linted.** `mise run lint-js` runs oxlint from
  the repo root over everything, the way `mise run fmt-js` formats. Directory
  lists go stale in one direction only: `website/` carried an empty
  `.oxlintrc.jsonc` that silently exempted all 28 of its files from every rule,
  and the per-package scripts named `src/` and `test/`, so the CLI package, the
  docs site, the benchmark scripts and every `examples/` directory had never been
  linted at all. Subtract from "everything" in `.oxlintrc.json`'s
  `ignorePatterns`, and give each entry a reason.
- **A silenced lint says why.** Line-level `// oxlint-disable-next-line <rule> --
  <why>` and `#[allow(...)]` with a comment above it. `--report-unused-disable-directives`
  runs in both the root and type-aware package passes, so a silence that stops
  being needed fails the build rather than sitting there.

A lint the repository has decided not to adopt is a waiver, not a silence: it
sits in one place, with the reason and the number of sites it fired on when it
was written down. `[workspace.lints.clippy]` and the `rules` block in
`.oxlintrc.json` hold all of them. That list is allowed to shrink; adding to it
needs the same justification the existing entries carry.

TypeScript's type-aware rules cannot run from the repo root, because they need
each package's `tsconfig.json` and its built dependencies. The per-package
`pnpm run lint` scripts add them on top of the root sweep, and CI runs both:
`lint.yml` for the repo-wide pass, `javascript.yml` for the type-aware one.

## Releases

Releases are prepared by `release-prepare.yml`, one pull request per package, and published when one of those pull requests is merged.

- Every push to `main` opens or updates one `chore(release): <package> <version>` pull request per package that `mise run release-plan` says needs one, on branch `release/<package>`.
- Merging one is what ships it: `release-tag.yml` reads the merge commit and pushes `<package>/v<version>`, which starts the publish workflow. Merge one at a time, in publish order, and let each publish finish first.
- Run `mise run release-plan` to see the same list locally, and `mise run release-needed` for the commits behind it.
- Prepare a release by hand with `mise run release-prepare <package> <version>`, then commit it to `main` with that same subject; the tagging is automatic either way.
- `mise run release-prepare` bumps the target package version file and prepends the next changelog entry, and also rewrites dependent manifests and lockfiles. Review and commit everything it touches.
- If dependent manifests must move together, update them separately in the same release commit. `npm-lumis` is the exception: it also bumps the native crate, the `@lumis-sh/lumis-native` selector and all platform packages, because the release workflow publishes them under the same version first.
- A `version` requirement on a lumis crate must equal that crate's version in this repository. `mise run release-prepare` keeps them in step, rewriting dependent manifests beyond the package being released, so review and commit every file it touches. `mise run check-crate-deps` reports drift and `--fix` repairs it. Apart from `crates/autumnus`, no build here resolves those requirements, so that check is the only thing that can catch them. See [Crate version requirements](RELEASE.md#crate-version-requirements).
- The package tag, such as `cargo-lumis-cli/v0.2.0`, is pushed by `release-tag.yml` rather than by hand, and pushing it triggers the publish workflows.
- A successful `cargo-lumis-cli` publish opens `align-npm-cli`, which updates only the npm package's `binaryVersion`. Merging it lets the normal release workflow prepare npm's own version and changelog.

Do not hand-edit release versions or changelog sections when `mise run release-prepare` can generate them.

## Benchmarks

The suite under [`benchmarks/`](benchmarks/) runs four shared scenarios across Lumis Rust, Elixir, JavaScript Wasm, and CLI, plus syntect, Shiki, and `bat`. It uses Criterion for Rust and syntect, Mitata for JavaScript and Shiki, Benchee for Elixir, and Hyperfine for the CLIs. It also measures comparable package and release artifact sizes. mise installs the pinned toolchain and coordinates the runs.

Run the complete suite with:

```sh
mise trust benchmarks/mise.toml
mise install -C benchmarks
mise run -C benchmarks run
```

The root `mise run bench` task runs the same suite. Scenarios and current results are listed in [`benchmarks/README.md`](benchmarks/README.md).

## Environment variables

`mise.toml` sets everything a task needs, so a normal `mise run` invocation needs
none of these. They exist to override that default from the outside.

Most of them matter only while working on the repository. The exceptions are
`LUMIS_DATA_DIR`, which every runtime reads, and the tool-specific settings a
user can hit without cloning anything: `LUMIS_CONFIG` for the CLI, and
`LUMIS_BUILD`, `LUMIS_USE_LEGACY_ARTIFACTS` and `LUMIS_ARTIFACT_SOURCE` for the
Hex package.

`LUMIS_DATA_DIR` names the directory Lumis persists parsers, themes, and
compiled modules under. An empty value counts as unset, in every runtime.

Unset, the CLI and both native addons call
`lumis_wasm_runtime::store::default_data_dir`, which uses [`etcetera`]'s base
strategy: `$XDG_DATA_HOME` or `~/.local/share` everywhere except Windows, where
it is `%APPDATA%`. Node asks the addon for that same value through
`defaultDataDir()`. A platform with no addon falls back to the
`platformDataDir()` port in `src/runtime/node-cache.ts`, and
`test/data-dir-parity.test.ts` pins that port against the addon so the two cannot
drift.

| Variable | Read by | Purpose |
| --- | --- | --- |
| `LUMIS_DATA_DIR` | every runtime | Directory for parsers, themes, and compiled modules |
| `LUMIS_CONFIG` | CLI | Config file path, same as `--config` |
| `LUMIS_BUILD` | Elixir | Build the NIF from source instead of downloading a precompiled one |
| `LUMIS_USE_LEGACY_ARTIFACTS` | Elixir | Select the legacy-CPU NIF variant |
| `LUMIS_ARTIFACT_SOURCE` | Elixir | `github` or `cloudflare`; where to download the precompiled NIF from |
| `LUMIS_TEST_RUNTIME` | JavaScript tests | `native` or `wasm`; fails loudly if the requested runtime is unavailable |
| `LUMIS_QUERY_LANGUAGES` | `test:queries` | Comma-separated languages, so CI can shard parser builds |
| `LUMIS_QUERY_BATCH_LIMIT` | `test:queries` | Maximum languages per batch, asserted rather than assumed |
| `LUMIS_QUERY_COVERAGE` | `test:queries` | `complete` requires full coverage and forbids waivers |
| `LUMIS_QUERY_PARSERS` | `test:queries` | `published` judges only what npm ships |
| `LUMIS_WASM_REBUILD` | `crates/dev` | `1` rebuilds a parser already present in `tmp/wasm/build` |
| `LUMIS_FAKE_NVIM_CAPTURE_DIR` | CLI tests | Where the fake `nvim` records the arguments it was given |
| `LUMIS_FAKE_NVIM_APPEARANCE` | CLI tests | Appearance the fake `nvim` reports, `dark` by default |

`LUMIS_BUILD` and `LUMIS_USE_LEGACY_ARTIFACTS` are
[`rustler_precompiled`](https://hexdocs.pm/rustler_precompiled/) conventions,
which is why they are not named `LUMIS_*_NIF_*`.

The benchmark suite has its own `BENCH_*` set, declared in
[`benchmarks/mise.toml`](benchmarks/mise.toml) rather than here, since nothing
outside `benchmarks/` reads them.

[`etcetera`]: https://docs.rs/etcetera/

## Configuration files

These files drive code generation, builds, detection metadata, and theme extraction.

| File | Purpose |
| ------ | --------- |
| [`highlights.toml`](highlights.toml) | Tree-sitter highlight scope names |
| [`languages.toml`](languages.toml) | Parser metadata, query sources, language bundles, feature flags |
| [`themes/themes.lua`](themes/themes.lua) | Theme definitions from Neovim colorscheme sources |

### highlights.toml

`highlights.toml` defines the recognized Tree-sitter highlight scope names. `crates/lumis-core/src/highlights.rs` and `packages/javascript/lumis/src/highlights.ts` are generated from it, so do not edit those files by hand.

After changing `highlights.toml`, regenerate:

```sh
mise run langs-gen-highlights
```

This updates:

- `highlights.rs`: `HIGHLIGHT_NAMES` and `CLASSES` arrays (`CLASSES` replaces `.` with `-`)
- `highlights.ts`: `HIGHLIGHT_NAMES` array

### languages.toml

All language metadata lives in `languages.toml`. It is consumed by:

- `crates/dev`: fetches vendored parsers, preprocesses queries, builds WASMs, and generates the Rust package catalog and docs
- `crates/lumis/build.rs`: reads `queries/processed/` to embed query constants, including Lua-to-Rust regex conversion
- `crates/lumis-cli/build.rs`: reads `queries/processed/` to embed query constants
- `crates/lumis-core/build.rs`: generates the `Language` enum and Rust detection metadata
- `packages/javascript/lumis/scripts/build-langs.ts`: generates `packages/javascript/lumis/langs/*.ts`, bundles, and JS detection and loader metadata
- `packages/javascript/scripts/build-wasm-bundles.ts`: generates `packages/javascript/wasm-bundle-*` preset packages from bundle definitions
- CI workflows: build WASMs and update parser and query revisions

Bundle definitions also live here under `[bundles.*]`.

For Rust crates, bundle support is implemented as Cargo features such as `lang-bundle-web` and `lang-bundle-system`, which enable the related `lang-*` features transitively. The `lang-bundle-*` feature lists in `crates/lumis/Cargo.toml` and `crates/lumis-core/Cargo.toml` are generated from `languages.toml` by `mise run cargo-update-features`.

- Keep one parser ID per line in bundle arrays for cleaner diffs.
- After changing parser or bundle entries, sync the generated Rust and JavaScript outputs:

```sh
mise run cargo-update-features
mise run langs-gen-catalog
pnpm --filter @lumis-sh/lumis run build:generate
pnpm --dir packages/javascript run build:wasm-bundles
```

This updates files such as:

- `crates/lumis-wasm-runtime/src/catalog.rs`
- `packages/javascript/lumis/langs/*.ts`
- `packages/javascript/lumis/bundles/*.ts`
- `packages/javascript/lumis/src/generated/*`
- `packages/javascript/wasm-bundle-*/`

Parser/query upgrades do not change runtime catalogs. Changes to language IDs,
aliases, package assignments, bundles, or the Tree-sitter series do; CI runs
`mise run langs-check-catalog` and rejects stale generated data.

#### Parser entry fields

```toml
[parsers.bash]
git = "https://github.com/tree-sitter/tree-sitter-bash.git"
rev = "a06c2e4415e9bc0346c6b86d401879ffb44058f7"
crate = "tree-sitter-bash"        # optional -- Rust crate name on crates.io
version = "0.25.1"                # required -- crate version (if crate) and WASM release version
location = "bash"                 # optional -- subdirectory in multi-grammar repos
generate = true                   # optional -- run `tree-sitter generate` before build
aliases = ["sh", "zsh"]           # optional -- alternative names (extra from_str matches)
wasm_name = "tree-sitter-bash"    # optional -- override WASM filename (default: tree-sitter-{name})
query_name = "bash"               # optional -- override query directory name (default: parser name)
# Language detection fields (used by lumis-core/build.rs):
globs = ["*.sh", "*.bash"]        # optional -- file glob patterns (default: [])
variant = "Bash"                  # optional -- Rust enum variant (default: titlecase of key)
display_name = "Bash"             # optional -- human-readable name (default: same as variant)
emacs = ["sh"]                    # optional -- Emacs mode names for detection
shebang = ["bash", "sh"]          # optional -- shebang interpreter names
feature = "lang-ocaml"            # optional -- override feature name (default: lang-{key with _ -> -})
```

`git`, `rev`, and `version` are always required. Every parser is identified by its git repo and pinned commit.

`git` + `rev` and `crate` are not alternatives:

- `git` + `rev`: always required for WASM builds, vendored parser fetches, and the source-of-truth version pin
- `crate`: optional and Rust-only. When present, Rust uses the crates.io package instead of vendored source. The dependency must also be declared in `crates/lumis/Cargo.toml` as an optional dependency for new languages, and existing entries are kept in sync from `languages.toml` by `cargo run --manifest-path crates/dev/Cargo.toml --no-default-features -- cargo-update-dep`.

`version` is used for two things:

1. If `crate` is set, it is the crate version synced into `crates/lumis/Cargo.toml`.
2. It always drives the npm package version for `@lumis-sh/wasm-{name}`.

#### Vendoring unreleased parser fixes

Lumis can deliberately vendor a grammar that also has a crates.io package when
an unreleased source fix is required. Its C sources are then compiled by
`crates/lumis/build.rs`, including Lumis's compiler flags, instead of by the
external crate's build script.

Forks inherit upstream tags, so the newest release tag can point behind the
recorded fork revision. `upgrade-parsers` checks Git ancestry and never replaces
the recorded revision with one of its ancestors. This protects unreleased fork
fixes without separate configuration while still allowing a later descendant
release to advance normally. When the fix ships in an upstream crate release,
return the parser entry to the upstream repository and crate.

#### Query entry fields

```toml
[queries.default]
git = "https://github.com/nvim-treesitter/nvim-treesitter.git"
rev = "3edb01f912867603c2aef9079f208f0244c0885b"
path = "runtime/queries"
```

Most languages use the `default` query source from `nvim-treesitter`. Add an explicit entry only when queries come from a different repo:

```toml
[queries.iex]
git = "https://github.com/elixir-lang/tree-sitter-iex.git"
rev = "39f20bb51f502e32058684e893c0c0b00bb2332c"
path = "queries"
```

## Languages and parsers

### Adding a new language

#### 1. Add a parser entry to `languages.toml`

```toml
[parsers.{name}]
git = "https://github.com/.../tree-sitter-{lang}.git"
rev = "full_commit_sha"
version = "x.y.z"
# Add crate if a crates.io package exists
# Add aliases if the language has common alternative names
```

`crates/lumis-core/build.rs` generates the `Language` enum variant and Rust detection metadata from this entry. The JS build generates its detection tables from the same `languages.toml` data.

#### 2. Add Rust wiring

In `crates/lumis/Cargo.toml`:

- If a crates.io package exists, add an optional dependency:

  ```toml
  tree-sitter-{lang} = { version = "x.y.z", optional = true }
  ```

  and a feature flag:

  ```toml
  lang-{name} = ["dep:tree-sitter-{lang}", "lumis-core/lang-{name}"]
  ```

- If no crate exists, add an empty feature flag:

  ```toml
  lang-{name} = ["lumis-core/lang-{name}"]
  ```

- Add `lang-{name}` to `all-languages` in both `crates/lumis/Cargo.toml` and `crates/lumis-core/Cargo.toml`.
- If the language belongs in an existing bundle, add it to the matching bundle in `languages.toml`, then run `mise run cargo-update-features` to regenerate the `lang-bundle-*` feature lists in both Cargo manifests.

#### 3. Fetch parser and queries

```sh
mise run langs-fetch-vendored-parsers {name}
mise run cargo-update-dep {name}
mise run cargo-update-features
mise run langs-gen-catalog
mise run langs-fetch-queries {name}
```

#### 4. Wire up vendored parsers

Do this when there is no crates.io package or when Lumis intentionally vendors
an unreleased source fix as described in
[Vendoring unreleased parser fixes](#vendoring-unreleased-parser-fixes):

- Add compilation in `crates/lumis/build.rs` inside `vendored_parsers()`.
- Add an `unsafe extern "C"` declaration in `crates/lumis/src/languages.rs`:

  ```rust
  #[cfg(feature = "lang-{name}")]
  fn tree_sitter_{name}() -> *const ();
  ```

#### 5. Wire up highlight config

In `crates/lumis/src/languages.rs`:

- Add a `{LANG}_CONFIG` static `LazyLock<HighlightConfiguration>`.
- Return it from the `config()` match with `#[cfg(feature = "lang-{name}")]`.

#### 6. Add a sample file

Add `samples/{name}.{ext}` with representative code. It is used by tests and `website/` demos.

#### 7. Generate docs

```sh
mise run docs-gen-languages-md
```

This updates `LANGUAGES.md`.

#### 8. Verify

```sh
cargo test --all-features
cargo clippy --all-features -- -D warnings
mise run test-conformance
```

### Updating parsers

```sh
mise run langs-upgrade-parsers {name}
mise run langs-fetch-vendored-parsers {name}
```

For parser and query upgrades, prefer the `Upgrade Languages` GitHub workflow or run the upgrade recipes below before `mise run langs-update {name}`. Do not use Dependabot to bump `tree-sitter-*` Rust parser crates independently. Those versions are tied to the pinned parser and query state in `languages.toml`.

`mise run langs-upgrade-parsers {name}` updates `languages.toml`, syncs any crate-backed Rust parser versions into `crates/lumis/Cargo.toml`, and refreshes Rust bundle features from `languages.toml`.

### Updating queries

```sh
mise run langs-upgrade-queries {name}
mise run langs-fetch-queries {name}
mise run langs-preprocess-queries
```

Omit `{name}` to upgrade all queries at once.

`mise run langs-update {name}` is the end-to-end command for a coordinated parser update. It fetches parsers first, syncs crate-backed Rust parser dependencies and Rust bundle features, then fetches and preprocesses queries, and regenerates `LANGUAGES.md`.

Raw query sources live in `queries/upstream/`. First-class local bracket queries live in `queries/brackets/`. Preprocessed tracked outputs live in `queries/processed/` and should be committed whenever upstream queries, bracket queries, replacements, or append patches change.

#### Bracket queries

Upstream grammars and nvim-treesitter rarely ship a `brackets.scm`, so Lumis keeps its own as first-class local queries:

- `queries/brackets/default/brackets.scm`: shared default, used by any language without its own bracket query
- `queries/brackets/{name}/brackets.scm`: a language-specific query for when the default doesn't fit the grammar

Bracket matching and rainbow brackets both read these queries. A query captures `@open`/`@close` pairs; a pattern tagged with `(#set! rainbow.exclude)` still matches but is skipped during rainbow coloring, so string and template delimiters stay uncolored.

#### Query replacements and append patches

When a fetched upstream query needs to change, use one of these directories:

- `queries/override/{name}/{query}.scm`: replace the upstream query entirely when it's incompatible with the pinned parser
- `queries/append/{name}/{query}.scm`: add local patterns after the upstream query, or after the replacement query when both exist

### Building WASMs

WASMs are built in CI by the `wasm-release` workflow, but you can build them locally:

```sh
mise run wasm-build
mise run wasm-build {name}
```

The compiler comes from `tree-sitter-cli`, which brings its own WASI SDK and downloads it on first use; Emscripten is not involved. You also need `git`, to fetch each grammar at the revision `languages.toml` pins, and `npm`, for the grammars whose parser is generated from `grammar.js`.

### Running a runtime locally

Parsers download on demand, so a checkout needs no setup. You only need
`LUMIS_DATA_DIR` for a parser you built yourself, which is not published and
would 404 on the CDN:

```sh
export LUMIS_DATA_DIR=$PWD/tmp/wasm/local
```

```sh
mise run wasm-build elixir
mise run wasm-stage elixir
```

Staging more languages adds to the same directory, and a parser there is used
even when it is only reached as an injected language.

```sh
cargo run -p lumis-cli -- highlight packages/elixir/lumis/mix.exs
cargo run -p lumis-cli -- highlight -l elixir <<< 'def foo, do: "hello"'
```

```sh
# Elixir
cd packages/elixir/lumis
LUMIS_BUILD=1 iex -S mix
iex> Lumis.highlight!("def foo, do: \"hello\"", formatter: {:html_inline, language: "elixir", theme: "dracula"})
```

Node resolves `@lumis-sh/lumis` by its own name only from inside the package, so
the script has to live there:

```js
// packages/javascript/lumis/test.mjs
import { createHighlighter } from "@lumis-sh/lumis";
import { htmlInline } from "@lumis-sh/lumis/formatters";
import elixir from "@lumis-sh/lumis/langs/elixir";
import dracula from "@lumis-sh/themes/dracula";

const highlighter = await createHighlighter({ languages: [elixir] });
const html = highlighter.highlight(
  'def foo, do: "hello"',
  htmlInline({ language: elixir, theme: dracula }),
);
console.log(html);
```

```sh
pnpm --filter @lumis-sh/lumis run build
node packages/javascript/lumis/test.mjs
```

A browser cannot read `LUMIS_DATA_DIR`, so serve the staged directory instead
and point the resolvers at it. The dev server serves the package root, so a
symlink puts the parsers on the same origin as the page:

```sh
mkdir -p packages/javascript/lumis/.tmp
ln -sfn ../../../../tmp/wasm/local/parsers packages/javascript/lumis/.tmp/parsers
pnpm --filter @lumis-sh/lumis run dev
```

Then open `/test.html` on the dev server.

```html
<!-- packages/javascript/lumis/test.html -->
<div id="out"></div>
<script type="module">
  import { createHighlighter } from "@lumis-sh/lumis";
  import { htmlInline } from "@lumis-sh/lumis/formatters";
  import elixir from "@lumis-sh/lumis/langs/elixir";
  import dracula from "@lumis-sh/themes/dracula";

  const base = "/.tmp/parsers";
  const highlighter = await createHighlighter({
    languages: [elixir],
    languagePackageResolver: (packageName) =>
      `${base}/${packageName.replace("@lumis-sh/wasm-", "")}.lumis.json`,
    wasmResolver: (_language, wasm) => `${base}/${wasm.name}-${wasm.version}-${wasm.sha256}.wasm`,
  });

  document.getElementById("out").innerHTML = highlighter.highlight(
    'def foo, do: "hello"',
    htmlInline({ language: elixir, theme: dracula }),
  );
</script>
```

Those URLs are the layout `mise run wasm-stage` writes: the package file is named
after the package minus its scope, and the parser is content-addressed. Loading is
asynchronous in a browser, so an injected language has to be named in `languages:`
rather than discovered mid-document.

After a change under `crates/`, rebuild the native artifact or the runtime keeps
the old one and fails as if your change broke it:

```sh
mix compile --force                              # Elixir NIF
pnpm --filter @lumis-sh/lumis run build:native   # Node addon
```

#### WASM distribution

Each grammar is published as a self-contained `@lumis-sh/wasm-{name}` language
package. It contains the parser WASM, matching processed queries, aliases,
grammar metadata, byte length, and SHA-256. During staging, `crates/dev`
generates `tmp/wasm/publish/{name}/lumis.json`; it is published with the
package rather than checked in.

Runtime catalogs generated from `languages.toml` contain stable IDs, aliases,
and package names. They also contain one compatible npm range derived from the
Tree-sitter series in `mise.toml`; they do not pin every package independently.
JavaScript, CLI, and Elixir ask the CDN to resolve that range, validate and cache
the exact `lumis.json` returned, then fetch the exact versioned parser named by
that metadata and verify its bytes. Parser or query updates within the supported
series therefore publish only the affected language package, without a runtime
release.

A compatible cached manifest is an exact lock and normal highlighting never
revalidates it. Use the runtime's forced cache command to resolve the range
again. Changing the Tree-sitter minor series changes the compatibility range
and does require runtime releases.

CLI, Node and Elixir caches persist on disk under `LUMIS_DATA_DIR`, in the same
layout, so one prepared directory serves all three; browsers use CacheStorage
with an IndexedDB fallback. Local and benchmark execution should provide both
`lumis.json` and its matching parser so queries and parser bytes remain
atomic.

#### Parsers in CI

CI builds parsers from `languages.toml` rather than taking them from npm, so a
revision bump is validated before it is published rather than after.

- **Queries CI** builds 110 of the 113 parsers across 12 shards and compiles
  every processed query for all 115 language definitions against the grammar its
  language actually pins. Each shard runs in fresh batches of four selected
  languages; the global `cannotCompile` check may load PHP as a fifth grammar,
  so no process retains more than five. `llvm`, `vim` and `zsh` exceed a runner's
  memory and are committed under `fixtures/parsers/` with their measured peak
  RSS; a parser that cannot be built does not fail its shard, but falls back to
  that copy and then to the published package.
- **Conformance CI** builds the seventeen parsers the committed fixtures supply,
  stages them with `wasm-stage`, and points `LUMIS_DATA_DIR` at the result, so
  the CLI, Elixir and Node native suites render from parsers built in that run.
- **JavaScript CI** runs the direct-addon store tests in their own process against
  a seeded copy of that store, then runs the remaining native-selected tests
  against an empty one before running the full Wasm-selected suite. The split is
  intentional: a parser already in the store takes precedence over configured
  JavaScript resolvers, so one process cannot honestly prove both paths.

That parser set comes from the fixture filenames, not from the languages named
in the fixtures' expected events. A document can attempt a language that never
appears in its output — every Lua comment injects the `comment` parser — and
leaving it out sends the runtime to the network mid-suite.

Two files record what these checks cannot cover, and both may only shrink:

- `fixtures/parsers/` holds a committed build for a grammar CI cannot compile.
  `tree-sitter-vim` needs 18.3 GB of memory against a runner's 16 GB.
- `unverified-parsers.json` has two lists: `languages`, which npm has fallen
  behind on, and `cannotCompile`, for a language a built parser still cannot
  check. `llvm` has no queries upstream; `php` traps while parsing its own
  sample at the pinned revision, published package included. A test fails when
  either entry starts working.

The `wasm-release` workflow publishes packages automatically. It detects which parsers still need publishing with `mise run wasm-publish-needed`, which compares each published package's `definitionHash` against the one `crates/dev` computes from `languages.toml` and the processed queries, then builds and publishes the rest in parallel.

## Themes

Themes are extracted from Neovim colorscheme plugins. Each theme is a JSON file in `themes/`. Theme definitions live in [`themes/themes.lua`](themes/themes.lua), which follows the `vim.pack` convention so Neovim can install them automatically.

### Adding a new theme

1. Add the theme definition to `themes/themes.lua`:

```lua
{
    url = "https://github.com/author/theme-repo",
    name = "theme_name",
    config = function()
        vim.o.background = "dark"
        vim.cmd([[colorscheme theme-name]])
    end,
}
```

1. Generate the JSON file and sync package outputs:

```sh
mise run themes-gen theme_name
```

1. Regenerate CSS and sync package outputs:

```sh
mise run css-gen
```

1. Regenerate docs:

```sh
mise run docs-gen-themes-md
```

`build.rs` picks it up on the next build.

### Updating themes

```sh
mise run themes-gen
mise run themes-gen theme_name
```

After regenerating, refresh CSS outputs:

```sh
mise run css-gen
```

Theme generation keeps the previous revision when an upstream commit produces
identical theme data, so revision-only changes do not create update PRs.

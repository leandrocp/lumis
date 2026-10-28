# Architecture

<!-- markdownlint-disable MD013 -->

```text
+------------------+ +---------------+ +------------------+
| languages.toml   | | themes/*.json | | queries/**/*.scm |
| parser metadata  | | theme data    | | highlight rules  |
+--------+---------+ +-------+-------+ +---------+--------+
         |                   |                   |
         v                   v                   v
+--------------------------------------------------------------+
| mise.toml + benchmarks/mise.toml + crates/dev                |
| setup / lint / test / docs / codegen / packaging / benchmark |
+-------+-------------------+-------------------+--------------+
        |                   |                   |
        v                   v                   v
+--------------------+ +--------------+ +--------------------+
| processed queries  | | css/*.css    | | tree-sitter-*.wasm |
| generated queries  | | linked HTML  | | parser binaries    |
+---------+----------+ +------+-------+ +----------+---------+
          |                   |                    |
          |                   |                    |
          v                   |                    v
+-------------------------+   |     +---------------------------+
| generated runtime data  |   |     | language packages         |
| JS language handles +   |   |     | parser + queries +        |
| shared Rust catalog     |   |     | integrity metadata        |
+------------+------------+   |     +---------------------------+
+------------+------------+   |                    |
             |                |                    |
             +----------------+--------------------+
                                  |
                                  v
+--------------------------------------------------------------+
| lumis-core (Rust crate)                                      |
| language detection + theme/style logic + formatter behavior  |
+-----------------------------+--------------------------------+
                              |
                              v
+--------------------------+   +---------------------------------------------------+
| lumis (Rust crate)       |   | lumis-wasm-runtime                                |
| native parser features   |   | resolve + verify + cache + one-pass highlighting  |
+--------------------------+   +----+-------------------+----------------------+---+
                                    |                   |                      |
                                    v                   v                      v
                            +--------------+   +----------------+   +--------------------+
                            | lumis-cli    |   | elixir/lumis   |   | lumis-js-native    |
                            | Rust binary  |   | Rustler NIF    |   | Node napi addon    |
                            +------+-------+   +--------+-------+   +---------+----------+
                                   |                    |                     |
                                   |                    |                     v
                                   |                    |          +------------------------+
                                   |                    |          | javascript/lumis (npm) |
                                   |                    |          | addon on Node,         |
                                   |                    |          | web-tree-sitter in the |
                                   |                    |          | browser                |
                                   |                    |          +-----------+------------+
                                   |                    |                      |
                                   +--------------------+----------------------+
                                                            |
                                                            v
                                                 +---------------------------+
                                                 | website / docs / examples|
                                                 +---------------------------+
```

## JavaScript integrations

`@lumis-sh/rehype-lumis`, `@lumis-sh/markdown-it-lumis`, `@lumis-sh/react`, and
`@lumis-sh/vite` require `@lumis-sh/lumis` as a peer dependency and keep it as a
development dependency for builds and tests. The application supplies the Lumis
installation its integrations use. Built-in formatter identity lives in a
module-local registry; a second copy cannot recognize the application's
formatters when rehype or markdown-it adds code-block attributes. Vite uses
rehype for that step.

The runnable examples remain standalone npm projects, outside the pnpm
workspace. They declare their parser packages explicitly. `test-examples`
uses npm to install isolated copies against packed workspace libraries and the
local native addon, then builds or renders each project. Example npm lockfiles
are untracked; CI checks fresh resolutions, not a separate pnpm installation path.
The same runner creates or refreshes the checked-in HTML through `examples-regen`.
Npm release preparation updates example dependency ranges to the released version.

## HTML line contract

HTML formatters render content-only `span.l-line` elements, separated by LF text
nodes. A final source newline terminates its line instead of creating an extra
line element. The per-line HTML helpers follow the same content-only contract;
custom formatters join wrapped lines explicitly. Terminal and BBCode keep their
source terminators; every formatter reads the same line-decoration event stream.

`lumis-core` owns HTML rendering and the generated theme layout. The JavaScript
port is pinned by `fixtures/formatter-helpers.json` and the conformance corpus;
Elixir uses the same helpers through its NIF. Theme CSS, inline styles and the
browser layout tests move with the markup. `fixtures/theme-css-options.json`
pins CSS builder options and output across Rust, JavaScript, Elixir, and the CLI.
Generated CSS includes layout by default; callers can disable it while prebuilt
stylesheets keep it. The layout rules are documented in
[CSS theme files](docs/content/themes/css-files.mdx#line-layout) and the line
helpers in [custom formatters](docs/content/formatters/custom.mdx).

## Automatic terminal theme choice

Automatic terminal theme choice lives in `lumis-core::themes::closest`.
The CLI gathers background, foreground and ANSI accent colors in one terminal
exchange and passes those observations to the core. The selection rule is
pinned by `fixtures/theme-choice-cases.json`; it does not depend on terminal I/O.
Elixir calls the same function through its NIF. JavaScript exposes a synchronous
`closest` through `@lumis-sh/lumis/themes`, using the normalized theme catalog
and a port checked against the same cases and monochrome exclusions. That
entry point has no native-addon dependency and works in browsers.

## Elixir options

`lumis_core::elixir`, behind the optional `rustler` feature, owns the BEAM
types shared by `lumis_nif` and `mdex_native`. Lumis keeps its existing
NimbleOptions validation, `rust_options!/1` conversion and derived NIF
decoders. Its wire format and formatter helper APIs do not change.

`ExLumisOptions::from_term` also reads options as callers write them, such as
`[formatter: {:html_inline, theme: "dracula", highlight_lines: %{lines: [2..4]}}]`.
mdex_native uses this entry point without installing `:lumis`. It accepts
keyword lists, maps, deprecated options and the encoded fields older MDEx
versions pass. Resolved themes and line-range structs decode directly; no
Elixir reverse conversion or second theme lookup is needed.

The raw decoder permits repeated keys and nil optional fields. Invalid
options raise `ArgumentError` naming the option and suggesting close matches.
Error messages summarize containers and large binaries without traversing or
copying them. mdex_native's integration fixtures compare raw and converted
options through rendering, including modified custom themes.

## Website rendering

`website/` builds static HTML for the home, comparison, and showcase pages.
Vite loads `src/entry-server.ts` during the build and inserts the existing
section renderers' output into each HTML entry. Browser entry points attach
interactions to that markup. Content changes therefore reach both plain HTTP
clients and browsers from the same source.

## Performance benchmark lane

`benchmarks/` is an intentionally non-published comparison layer over the public Rust, JavaScript, and CLI surfaces.

```text
benchmarks/fixtures + deterministic generator
                    |
                    v
       identical small and large Rust inputs
       +----------+----------+----------+
       |          |          |          |
       v          v          v          v
 Rust/Criterion JS/Mitata Elixir/Benchee CLI/Hyperfine
 Lumis/syntect  Lumis/Shiki     Lumis     Lumis/bat
       |          |          |          |
       +----------+----------+----------+
                         |
                         v
 target/benchmarks/runs/current/ raw reports + package sizes + metadata
                         |
                         v
                 benchmarks/README.md
```

The Rust benchmark package is its own Cargo workspace so syntect and Criterion do not enter normal production workspace builds. The private JavaScript benchmark package joins the pnpm workspace so it can consume locally built Lumis packages while keeping Shiki and Mitata out of published manifests.

Fixture generation, optimized artifact builds, timed execution, memory sampling, cache preparation, and reporting are separate mise tasks. `benchmarks/mise.toml` pins the benchmark toolchain, models task dependencies and incremental sources/outputs, and owns shared paths. Root `mise run bench-*` tasks delegate to that benchmark configuration. The root `mise.toml` pins only the Tree-sitter CLI series, which `dev wasm-needed` reads so the published `lumis.treeSitter` metadata and the CLI that built a parser cannot drift apart; `mise.lock` locks it for every platform CI runs on. Language runtimes stay unpinned at the root so local commands use the developer's own toolchains. Timed families execute serially. Parser assets are prepared and verified before timing so network variability never enters benchmark results.

CI uses `jdx/mise-action` with the same benchmark config. Every pull request runs one observational, non-gating stable suite covering Rust, JavaScript, CLI, Elixir, package size, and memory. It appends available benchmark tables to the GitHub Actions job summary and retains raw report artifacts for seven days.

## Dynamic language packages

`languages.toml` is the repository source of truth. Generated runtime catalogs
contain stable language IDs, aliases, and npm package names plus one
Tree-sitter-compatible npm range for the whole catalog. `crates/dev` derives
that range from the Tree-sitter series in `mise.toml` and generates the
checked-in Rust catalog data, which `lumis-wasm-runtime` expands through a
declarative macro. JavaScript generates the same range from the same source.
There is no independently maintained version per language.

Each `@lumis-sh/wasm-*` release is the independently versioned, atomic unit
containing:

- one parser WASM
- every Lumis language backed by that parser
- the matching highlight, injection, locals, and bracket queries
- parser SHA-256, grammar name, and package version

During staging, `crates/dev` serializes those inputs into `lumis.json`. The
manifest is published inside the language package and is not checked in.

What those packages cost runs the other way. `parser-sizes.json` is checked in
and measured *from* the published packages by `mise run wasm-sizes`: the
tarball, the unpacked `.wasm`, and the memory the parser reserves in the shared
Tree-sitter WASM store, read from its `dylink.0` section. The language catalog
is generated in a checkout with no network and no built WASM, so the numbers
cannot be measured where they are rendered.

Changing a parser or one of its queries publishes only that language package.
It does not require a JavaScript, CLI, Rust, or Elixir runtime release unless
the language-package format or supported Tree-sitter ABI series changes.

Node, Elixir and browsers never ask a CDN: they read the manifest and parser
from the package a project installed or an application imported, and check it
against the compatible range. The CLI asks npm CDNs to resolve the range,
validates the exact version returned in the package metadata, then loads the
exact parser that metadata identifies:

```text
stable language catalog + compatible range
         |
         v
store metadata (exact lock) -> CDN range resolution
         |
         v
store parser (verified) -> exact-version CDN parser
         |
         +-> persistent Wasmtime compiled cache
```

### One rule, three declarations

Every runtime has a set of languages it can use, and renders anything outside it
plain. That rule is the same everywhere; what differs is how the set is
declared, and a runtime gets exactly one of these:

| Runtime | Declares its set in | How parsers arrive |
| --- | --- | --- |
| Rust `lumis` crate | `Cargo.toml` features | linked statically: 65 crates.io parsers, 47 vendored sources |
| JavaScript on Node | `package.json` | `@lumis-sh/wasm-*` in `node_modules` |
| JavaScript in a browser | the packages it imports | `@lumis-sh/wasm-*` in the bundle, each exporting its language |
| Elixir, any future FFI binding | `mix.exs` dependencies | `lumis_wasm_*` in each dependency's `priv/parsers` |
| The CLI | nothing — it declares no set | fetched on demand into its own store |

The Rust crate depends on `lumis-wasm-runtime` with `default-features = false`,
so it has no wasmtime and no HTTP client: a language that was not compiled in
does not exist, and there is nothing to fetch. That is the boundary talking, the
same way the NIF boundary shapes Elixir's formatter signatures. It is not a
divergence to paper over.

Elixir declares in `mix.exs` and pins in `mix.lock`, the files it already has. A
parser package is an ordinary OTP application whose `priv/parsers` holds the
manifest and the WASM it describes, so a release carries it like any
dependency's assets and nothing is fetched at runtime. `config :lumis,
:parser_dirs` names directories directly for a project that vendors them
instead.

The CLI is the one row with no declaration, and that is deliberate rather than a
gap: see below. A JavaScript caller can still fetch at run time by configuring a
resolver, which is a declaration of its own.

Nobody gets two declarations. A JavaScript project that installs its parsers
does not also write a lock; `package.json` already is one.

**The CLI is its own runtime, and declares nothing.** It is a viewer and a store
filler: `lumis highlight` and `lumis dump` resolve freely, and `lumis languages
download` fills the store. Reading a project's dependencies would make a
highlighter behave differently depending on the directory it ran in, which is
worse than one that does not.

### Highlighting loads what a document needs, in one pass

Highlighting a document resolves, downloads, verifies and loads whatever it
turns out to name. A Markdown file with a fenced Rust block highlights that
block, without Rust having been mentioned anywhere in the calling code. Nothing
is configured, nothing is enumerated.

The three runtimes used to disagree here — the CLI and JavaScript skipped an
unloaded injection while Elixir fetched it and re-highlighted — so the same
input produced different output per runtime. What removed the disagreement was
making one implementation serve all three, not making the rule stricter.

**One pass, not two.** `Runtime::highlight` hands Tree-sitter a callback for
injected languages, and that callback loads. So the walk descends into the
language it just fetched and finds whatever *that* language injects, however
deep the nesting goes. The alternatives were both worse: highlighting twice
throws away the first pass, and a separate discovery pass has to re-run the
injections query per nesting level, which measured at 32% of a full pass and
still cannot see past the layers it has already loaded.

Configured JavaScript language-package and WASM resolvers are part of that Node
path too. A language first named by an injection is resolved and loaded before
the walk descends into it. The native walk can synchronously read local paths
and `file:`, `data:`, `http:`, and `https:` URLs returned by those resolvers.
JavaScript-owned resources such as `blob:` URLs have to be resolved by
`loadLanguage` before the walk because Rust cannot access JavaScript's blob
registry.

Ordinary Node highlighters share one `Runtime`, so loading Markdown forty times
does not create forty Tree-sitter Wasm stores. A highlighter is promoted to its
own runtime only when it accepts caller-selected parser bytes, a different
package, or resolver callbacks. Definitions use a content-derived internal id,
and catalog roots or canonical installed packages loaded before promotion are
replayed into the private runtime. The Wasmtime engine and persistent caches
remain shared.

That private store is the lifetime boundary. Tree-sitter exposes no operation
that reclaims one language from a Wasm store, and loading another module advances
the store's memory allocation monotonically. Removing an id from Lumis's catalog
therefore does not free its parser. Dropping the whole private runtime does, so
caller-resolved definitions are isolated between highlighters and reclaimed when
their highlighter and any in-flight formatting task are gone.

Queries are not part of that surface. A parser and the queries written against it
are released together in one package, so a caller names a package and, at most,
where its parser bytes come from. Supplying queries directly is a planned
cross-runtime capability; it existed in JavaScript alone and was removed rather
than shipped half-designed.

Parser modules are retained before query compilation. Rust keys both successful
and failed store loads by the declared grammar and the SHA-256 of the actual
bytes; `web-tree-sitter` retains the instantiated language, or the failure, by
byte digest within its JavaScript realm. An invalid query can therefore be
corrected without loading the parser again, while repeating the same parser-load
failure cannot grow Tree-sitter's store. Fetch, resolver and integrity failures
happen before this cache and remain retryable. Package-backed loads also inspect
the module's single `tree_sitter_*` function export and require it to match
`parser.grammarName`, so native and browser runtimes interpret the same package
metadata identically.

**A failure costs one block.** A thousand-line document with ten languages must
not fail because one fenced block names something unpublished. An injected
language that cannot be fetched leaves its content plain and the walk carries
on. Only the root language failing is an error the caller sees.

Preloading follows the same rule: naming a list or a bundle loads everything in
it and reports which names failed, rather than stopping at the first. One
unpublished parser must not cost the rest.

**A cold parser is expensive; a warm one is not.** It costs a download and then
a Wasmtime compile, and the compile is the larger half: seven parsers already on
disk take about 8 s to compile against 1.3 s once `compiled/` holds their
compiled forms. Registering an already-compiled parser is 3-15 ms, and about
0.3 ms after. That is why loading in the request path is fine once the store is
warm, and why `load` still exists: to move both costs off a user's first
request, not to gate anything.

Once a parser is warm, most of what remains is compiling its queries, and Tree-sitter
spends nearly all of that analysing the grammar's parse table. That work is
independent per language, so `Runtime::load_languages` and
`Runtime::load_named_languages` add parser modules to the shared store one at a
time, which a `WasmStore` requires, then compile the queries of the whole batch in
parallel outside the store's lock. Each host loads through these entry points when
it already holds a list: `createHighlighter`'s initial languages in Node, through an
async addon task off the JavaScript thread, and `Lumis.Languages.load/1` with a
list in Elixir. A language an injection names still loads by itself, during the
walk that found it.

In Elixir it is cheaper still, because loading is global to the VM. One
`Runtime` lives in the NIF, so the first process to need a language pays, and
every process after it does not.

Browsers are the exception, and only half of loading is the problem. Resolving
a package and fetching a parser are asynchronous, so a walk cannot do them.
Instantiating a parser whose bytes are already in memory is synchronous: a
`WebAssembly.Module` handed to `Language.loadSync` is ready in a few
milliseconds. So loading a `web-tree-sitter` language is split in two. The
asynchronous half resolves the package, reads the parser and instantiates it;
the synchronous half compiles the queries and creates a parser for one runtime.
A language whose first half some runtime on the page already finished, from its
catalog package's own parser and with no caller resolver involved, is
registered for the whole realm, and a walk in any other runtime without a
resolver of its own runs the second half when an injection names it. As in
Rust, that time is given back to the render's budget. A runtime with a
resolver never borrows, because its caller said where parsers come from.

That leaves one real limit: a language nothing on the page has loaded still has
to be loaded before the document mentioning it. And Chromium refuses a
synchronous compile or instantiation over 8 MiB on the main thread (Firefox and
WebKit do not, but there is no way to ask), so a parser above that size, such as
F#, Nim or SystemVerilog, always takes `Language.load`. Node runs the native
addon specifically so it inherits none of this, and falls back to
`web-tree-sitter` only where no addon is built.

The npm build bundles the browser entry separately from the Node entry.
`tsup.browser.config.ts` resolves `process` and `globalThis.process` to
`undefined`, removing Tree-sitter's Node branches before a CDN can inject a
Node-compatible process polyfill. The Node bundle keeps those branches for its
Wasm fallback. Both builds finish before the bundle patch and declaration check
run.

The resolver itself follows the same ownership boundary. The CLI and Elixir call
`lumis-wasm-runtime::LanguageStore`, so compatible version checks, exact
manifest caching, integrity verification, and refresh semantics are one Rust
implementation. The Node addon loads only the `@lumis-sh/wasm-*` packages the
project installed: JavaScript resolves each one's `lumis.json` through Node's
module resolution, which Rust cannot follow, and the addon parses and validates
them with the same `LanguagePackage` code. Lumis checks parser bytes against the
manifest's SHA-256 only where it downloads them, through the CLI or a resolver,
and when it reads them back from its own cache; npm and Hex already verify what
they install. The browser cannot call Rust, so its small
TypeScript adapter checks an imported package's manifest against the same
generated range with npm's `semver` package. The
portable Node fallback uses that browser implementation. Cross-runtime package
fixtures pin both implementations to the same manifest contract.

Node discovers packages from imported languages and bundles, then from the
working directory. A language's file URL locates its package; a generated
bundle carries its module URL under `Symbol.for("@lumis-sh/package-url")` so
its language entries remain enumerable on their own. Discovery follows each
package's parser dependencies, including pnpm's separate dependency locations.
New roots extend the process-wide installed set and refresh the addon before
highlighting. Already discovered package versions keep their identity.

Everything a runtime persists lives under one directory, named by
`LUMIS_DATA_DIR`: `parsers/` for language packages and parser WASM, `themes/`
for the CLI's custom themes, `compiled/` for Wasmtime's module cache. Only the
CLI reads `parsers/`, whether the files were downloaded or staged there by a
build step; Elixir and Node load the parser packages a project installed, and
share `compiled/` with the CLI. Browsers keep parsers a configured resolver fetched in CacheStorage instead, having no filesystem. Parser
cache keys contain the parser name, package version, and digest, so upgrades do
not overwrite older verified assets. A compatible package already in the
directory is an exact lock and is never revalidated during highlighting; a
request therefore never waits on the network for something already on disk.
`lumis languages download --force` explicitly resolves the range again, fetches
and verifies the exact parser it names, then replaces the cached manifest. A
failed refresh therefore leaves the previous manifest and parser usable. Staging the directory makes deployments reproducible, while a new
or explicitly refreshed cache adopts new compatible language releases without a
runtime release.

The range is the Tree-sitter ABI boundary, currently `0.26`, not a Lumis release
train. npm and the CDN already implement SemVer selection, redirects, mirrors,
and package publication; Lumis supplies the range and validates the returned
manifest rather than recreating those package-manager responsibilities. Moving
to a new Tree-sitter minor series changes the range and legitimately requires a
runtime release. Publishing `@lumis-sh/wasm-rust@0.26.x` does not.

### Preparing the persistent store

One verb in every library runtime — **load**, which compiles a language, keeps
it in this runtime, and leaves its compiled module on disk. A second verb,
**download**, exists only on the CLI, which is the one host with no package
manager behind it: it fills a directory some other process will read.

The library runtimes used to offer downloading too, as `Lumis.Languages.download/2`
and `downloadLanguages()`. Hex and npm now own delivery — a parser is an ordinary
dependency, `mix deps.get` and `npm install` fetch it, and highlighting reads it
off the code path — so those functions described a delivery model that no longer
existed, and were the last thing in either runtime that reached the network.
Removing them is what makes "no network at boot or at render" a property rather
than a default.

Adding and removing a language is not a third verb: that is a dependency edit,
and every runtime already has tooling for it — `cargo add`, `npm install`,
`mix deps.get`. Lumis does not reimplement any of them.

That is also how a future Go or PHP binding would work without reimplementing
anything: point the store at directories holding parsers, laid out the way
`priv/parsers` is, and the declaration is whatever that ecosystem's manifest
says.

An application loads at startup without waiting for it. Elixir runs the load
under a `:temporary` child of Lumis's supervisor; JavaScript leaves the promise
unawaited. Neither can delay a boot or fail one, which matters because warm-up
is an optimization and highlighting loads on demand regardless:

```elixir
Lumis.Languages.async_load(["rust", "javascript"])
```

```javascript
import { load } from "@lumis-sh/lumis/languages"

load(["rust", "javascript"]).catch(report)
```

For a separate build or operations step, the CLI is the one command:

```sh
lumis languages download rust javascript
```

It writes a self-sufficient directory — parser bytes plus the `lumis.json` that
names them, and the compiled modules beside them — so pointing `LUMIS_DATA_DIR`
at it is all a deployment needs. It goes through `LanguageStore::cache_languages`
and `Runtime::precompile_languages`, which every other runtime then reads through
`LanguageStore`.

Downloading and loading differ in what they keep, and that is the whole reason
both exist. `Runtime::precompile_languages` validates each parser and its queries
in a disposable Tree-sitter store and then drops it, because the process that
will use the directory is not this one; retaining the compiled modules instead
would cost about 7.5 MB per language. A startup warm-up loads into the runtime
and keeps it, so no request reloads what the warm-up already paid for. Preparing
a directory and warming a VM are therefore complementary rather than
alternatives: a release fills the directory, and the application that reads it
still loads from disk into memory.

Validation writes Wasmtime's compiled module under `compiled/` and catches link
and external scanner failures that raw compilation cannot. Each worker drops its
store after validating one parser, and the compilation limit bounds simultaneous
stores to four. A full catalog therefore never accumulates in Tree-sitter's
shared 128 MiB address space, and peak memory stays bounded on high-core
builders.

**One prepared directory serves every runtime, including the compile.** Wasmtime
keys its module cache on the compiler and its version, so a release build of the
CLI, the Elixir NIF and the Node addon all read and write the same
`compiled/modules/` entries. Preparing a directory with `lumis languages download`
and pointing Elixir at it costs 128 ms to load a parser against 294 ms with
`compiled/` removed. What keeps that true is a single wasmtime version across
the three; `mise run elixir-nif-lock-check` enforces it, because the Elixir NIF
is the only one with a lockfile of its own.

Debug builds deliberately do not share. Wasmtime adds the executable's mtime to
the cache path when `debug_assertions` is on, so a rebuilt binary cannot read
entries its predecessor wrote. Measuring cache sharing with `cargo run` and
`LUMIS_BUILD=1 mix` therefore shows two namespaces and no reuse; that is the
debug profile, not a divergence between runtimes.

### Public language and theme modules

JavaScript's `/languages` and `/themes` entry points use the same short operation
names as Rust's `languages` and `themes` modules and Elixir's `Lumis.Languages`
and `Lumis.Theme`. The root keeps highlighting and deprecated compatibility
aliases. Language modules share the root runtime; importing the root does not
pull in the full theme catalog. `fixtures/api.json` records the canonical names,
runtime-specific operations, and aliases. The external Java binding retains its
separate enum API until that repository adopts these operations.

`Theme::map_styles`, `Lumis.Theme.map_styles/2`, and JavaScript's `mapStyles`
transform existing theme entries before formatting. Rust defines the contract;
callbacks execute in their caller's runtime, since an Elixir or browser callback
cannot be passed to the Rust closure. `fixtures/theme-map-styles.json` pins the
same transformation in all three runtimes, including exact scope matching,
complete style replacement, and unchanged metadata. No renderer applies a second
transformation or changes its existing style-resolution rules.

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
- parser size, SHA-256, grammar name, and package version

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

Dynamic runtimes ask npm CDNs to resolve the compatible range, validate the
exact version returned in the package metadata, then load the exact parser that
metadata identifies:

```text
stable language catalog + compatible range
         |
         v
installed/local language package -> persistent exact metadata cache -> CDN range resolution
         |
         v
installed/local parser -> persistent verified parser cache -> exact-version CDN parser
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
| JavaScript with installed packages | `package.json` | `@lumis-sh/wasm-*` in `node_modules` |
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
gap: see below.

Nobody gets two declarations. A JavaScript project that installs its parsers
does not also write a lock; `package.json` already is one.

One divergence remains: a JavaScript project that installs *no* parser still
falls back to the CDN, where an Elixir project that depends on none highlights
nothing. Elixir took the stricter rule first because it had no declaration at
all before; closing the JavaScript side is its own change.

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
bytes; `web-tree-sitter` retains the corresponding `Language.load` promise by
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

In Elixir it is cheaper still, because loading is global to the VM. One
`Runtime` lives in the NIF, so the first process to need a language pays, and
every process after it does not.

Browsers are the exception. `web-tree-sitter` loads asynchronously, so a parser
cannot be fetched inside a synchronous walk; an injected language has to be
loaded before the document mentioning it. Node runs the native addon
specifically so it does not inherit that limit, and falls back to
`web-tree-sitter` only where no addon is built.

The resolver itself follows the same ownership boundary. CLI, Elixir, and the
default Node addon all call `lumis-wasm-runtime::LanguageStore`, so compatible
version checks, exact manifest caching, integrity verification, and refresh
semantics are one Rust implementation. The browser cannot call synchronous Rust
from its asynchronous fetch path, so its small TypeScript adapter consumes the
same generated range and uses npm's `semver` package for the same check. The
portable Node fallback uses that browser implementation. Cross-runtime package
fixtures pin both implementations to the same manifest contract.

Everything a runtime persists lives under one directory, named by
`LUMIS_DATA_DIR`: `parsers/` for language packages and parser WASM, `themes/`
for the CLI's custom themes, `compiled/` for Wasmtime's module cache. The CLI,
Elixir and Node write the same filenames into `parsers/`, so one prepared
directory serves all three, whether the files were downloaded or staged there
by a build step. Browsers use CacheStorage instead, having no filesystem. Parser
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
loadLanguages(["rust", "javascript"]).catch(report)
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

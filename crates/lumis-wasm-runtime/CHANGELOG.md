## [0.3.0](https://github.com/leandrocp/lumis/compare/cargo-lumis-wasm-runtime/v0.2.1...cargo-lumis-wasm-runtime/v0.3.0) (2026-09-24)


### Bug Fixes

- BREAKING: make public error enums non-exhaustive - [#1395](https://github.com/leandrocp/lumis/pull/1395)
- bound the query match limit so capture iteration stays linear, and make it configurable - [#1425](https://github.com/leandrocp/lumis/pull/1425) by @ericmj
- slow HTML highlighting and missing colors inside nested elements - [#1435](https://github.com/leandrocp/lumis/pull/1435)
- BREAKING: return an exception from Lumis.highlight/2 instead of a string - [#1499](https://github.com/leandrocp/lumis/pull/1499)
- BREAKING: report a full Wasm store as its own parser failure - [#1507](https://github.com/leandrocp/lumis/pull/1507)
- drop systemverilog from bundle-full so the bundle can be loaded - [#1509](https://github.com/leandrocp/lumis/pull/1509)


### Features

- BREAKING: introduce annotations - [#1100](https://github.com/leandrocp/lumis/pull/1100)
- expose rainbow brackets as decorations - [#1439](https://github.com/leandrocp/lumis/pull/1439)
- add lumis-lock.toml to pin parser versions - [#1477](https://github.com/leandrocp/lumis/pull/1477)
- manage and enforce lumis-lock.toml with mix lumis.* - [#1479](https://github.com/leandrocp/lumis/pull/1479)
- bound a render with a time limit - [#1478](https://github.com/leandrocp/lumis/pull/1478)
- rename the cache verb to download - [#1481](https://github.com/leandrocp/lumis/pull/1481)
- BREAKING: parsers from Hex deps - [#1486](https://github.com/leandrocp/lumis/pull/1486)
- BREAKING: remove the runtime parser download from the Elixir and Node libraries - [#1510](https://github.com/leandrocp/lumis/pull/1510)


### Performance

- coalesce same-scope highlight spans - [#1459](https://github.com/leandrocp/lumis/pull/1459)

## [0.2.1](https://github.com/leandrocp/lumis/compare/cargo-lumis-wasm-runtime/v0.2.0...cargo-lumis-wasm-runtime/v0.2.1) (2026-09-03)


### Code Refactoring

- keep one copy of each test parser - [#1288](https://github.com/leandrocp/lumis/pull/1288)


### Features

- highlight code inside diff hunks - [#1284](https://github.com/leandrocp/lumis/pull/1284)

## [0.2.0](https://github.com/leandrocp/lumis/compare/cargo-lumis-wasm-runtime/v0.1.0...cargo-lumis-wasm-runtime/v0.2.0) (2026-08-13)

### Bug Fixes

- pin lumis crate requirements to the workspace version - [#1140](https://github.com/leandrocp/lumis/pull/1140)
- resolve compatible WASM package versions - [#1263](https://github.com/leandrocp/lumis/pull/1263)
- resolve one data directory across every runtime - [#1264](https://github.com/leandrocp/lumis/pull/1264)
- align the public API across Rust, the CLI, Elixir and JavaScript - [#1266](https://github.com/leandrocp/lumis/pull/1266)

### Features

- unify dynamic WASM language loading - [#1099](https://github.com/leandrocp/lumis/pull/1099)
- update lang scala - [#1221](https://github.com/leandrocp/lumis/pull/1221)
- BREAKING: one vocabulary for the language and theme catalog, in every runtime - [#1269](https://github.com/leandrocp/lumis/pull/1269)
- warm-up parsers - [#1273](https://github.com/leandrocp/lumis/pull/1273)

### Performance

- prepare large parser bundles concurrently - [#1271](https://github.com/leandrocp/lumis/pull/1271)

## [0.1.0](https://github.com/leandrocp/lumis/tree/cargo-lumis-wasm-runtime/v0.1.0) (2026-07-23)


### Features

- Lumis JavaScript native (Rust bindings) - [#1083](https://github.com/leandrocp/lumis/pull/1083)

<!-- Releases are prepended by `mise run release-prepare`. -->

## [0.6.3](https://github.com/leandrocp/lumis/compare/npm-cli/v0.6.2...npm-cli/v0.6.3) (2026-09-24)


### Bug Fixes

- queue npm CLI after Cargo publish - [#1353](https://github.com/leandrocp/lumis/pull/1353)
- update npm CLI binary to 0.6.0 - [#1530](https://github.com/leandrocp/lumis/pull/1530)


### Documentation

- migrate documentation to Fumadocs - [#1445](https://github.com/leandrocp/lumis/pull/1445)

## [0.6.2](https://github.com/leandrocp/lumis/compare/npm-cli/v0.6.1...npm-cli/v0.6.2) (2026-09-03)


### Code Refactoring

- keep one copy of each test parser - [#1288](https://github.com/leandrocp/lumis/pull/1288)


### Features

- highlight code inside diff hunks - [#1284](https://github.com/leandrocp/lumis/pull/1284)

## [0.6.1](https://github.com/leandrocp/lumis/compare/npm-cli/v0.6.0...npm-cli/v0.6.1) (2026-08-13)


### Bug Fixes

- resolve the npm binary through npm - [#1282](https://github.com/leandrocp/lumis/pull/1282)

## [0.6.0](https://github.com/leandrocp/lumis/compare/npm-cli/v0.5.0...npm-cli/v0.6.0) (2026-08-13)

### Bug Fixes

- pin lumis crate requirements to the workspace version - [#1140](https://github.com/leandrocp/lumis/pull/1140)
- highlight lines in light-dark() mode - [#1260](https://github.com/leandrocp/lumis/pull/1260)
- resolve compatible WASM package versions - [#1263](https://github.com/leandrocp/lumis/pull/1263)
- resolve one data directory across every runtime - [#1264](https://github.com/leandrocp/lumis/pull/1264)
- align the public API across Rust, the CLI, Elixir and JavaScript - [#1266](https://github.com/leandrocp/lumis/pull/1266)

### Documentation

- add release migration steps - [#1272](https://github.com/leandrocp/lumis/pull/1272)

### Features

- unify dynamic WASM language loading - [#1099](https://github.com/leandrocp/lumis/pull/1099)
- update lang scala - [#1221](https://github.com/leandrocp/lumis/pull/1221)
- BREAKING: scope highlight options to the formatters that accept them - [#1267](https://github.com/leandrocp/lumis/pull/1267)
- BREAKING: one vocabulary for the language and theme catalog, in every runtime - [#1269](https://github.com/leandrocp/lumis/pull/1269)
- warm-up parsers - [#1273](https://github.com/leandrocp/lumis/pull/1273)

### Performance

- prepare large parser bundles concurrently - [#1271](https://github.com/leandrocp/lumis/pull/1271)

### Breaking changes

**Updated commands**

1. Replace `lumis parsers fetch <names>` with `lumis languages cache <names>`.
2. Replace `lumis parsers update <names>` with `lumis languages cache --force <names>`.
3. Use `-H` for highlighted lines and `-h` for help.
4. Use `-V` for version and `-v` for verbose output.
5. Remove flags rejected by `lumis formatters show <name>`.

## [0.5.0](https://github.com/leandrocp/lumis/compare/npm-cli/v0.4.1...npm-cli/v0.5.0) (2026-07-23)


### Bug Fixes

- prevent query capture list pool overflow on large inputs - [#1028](https://github.com/leandrocp/lumis/pull/1028) by @ericmj


### Features

- add mdx language reusing the markdown parser - [#1012](https://github.com/leandrocp/lumis/pull/1012) by @benswift
- add dump commands - [#1087](https://github.com/leandrocp/lumis/pull/1087)
- Lumis JavaScript native (Rust bindings) - [#1083](https://github.com/leandrocp/lumis/pull/1083)

## [0.4.1](https://github.com/leandrocp/lumis/compare/npm-cli/v0.4.0...npm-cli/v0.4.1) (2026-07-11)

### Features

- config file [#998](https://github.com/leandrocp/lumis/pull/998)
- auto theme [#998](https://github.com/leandrocp/lumis/pull/998)

## [0.4.0](https://github.com/leandrocp/lumis/tree/npm-cli/v0.4.0) (2026-07-10)


### Features

- introduce npm cli - [#997](https://github.com/leandrocp/lumis/pull/997)

# Changelog

All notable changes to this package will be documented in this file.

## [0.6.0](https://github.com/leandrocp/lumis/compare/cargo-lumis-cli/v0.5.1...cargo-lumis-cli/v0.6.0) (2026-09-11)


### Bug Fixes

- reject a hex colour that is only partly hex - [#1394](https://github.com/leandrocp/lumis/pull/1394)
- BREAKING: make public error enums non-exhaustive - [#1395](https://github.com/leandrocp/lumis/pull/1395)


### Features

- BREAKING: introduce annotations - [#1100](https://github.com/leandrocp/lumis/pull/1100)

## [0.5.1](https://github.com/leandrocp/lumis/compare/cargo-lumis-cli/v0.5.0...cargo-lumis-cli/v0.5.1) (2026-09-03)


### Code Refactoring

- keep one copy of each test parser - [#1288](https://github.com/leandrocp/lumis/pull/1288)


### Features

- highlight code inside diff hunks - [#1284](https://github.com/leandrocp/lumis/pull/1284)

## [0.5.0](https://github.com/leandrocp/lumis/compare/cargo-lumis-cli/v0.4.2...cargo-lumis-cli/v0.5.0) (2026-08-13)

### Bug Fixes

- pin lumis crate requirements to the workspace version - [#1140](https://github.com/leandrocp/lumis/pull/1140)
- highlight lines in light-dark() mode - [#1260](https://github.com/leandrocp/lumis/pull/1260)
- resolve compatible WASM package versions - [#1263](https://github.com/leandrocp/lumis/pull/1263)
- resolve one data directory across every runtime - [#1264](https://github.com/leandrocp/lumis/pull/1264)
- align the public API across Rust, the CLI, Elixir and JavaScript - [#1266](https://github.com/leandrocp/lumis/pull/1266)

### Features

- unify dynamic WASM language loading - [#1099](https://github.com/leandrocp/lumis/pull/1099)
- update lang scala - [#1221](https://github.com/leandrocp/lumis/pull/1221)
- BREAKING: scope highlight options to the formatters that accept them - [#1267](https://github.com/leandrocp/lumis/pull/1267)
- BREAKING: one vocabulary for the language and theme catalog, in every runtime - [#1269](https://github.com/leandrocp/lumis/pull/1269)
- warm-up parsers - [#1273](https://github.com/leandrocp/lumis/pull/1273)

### Performance

- prepare large parser bundles concurrently - [#1271](https://github.com/leandrocp/lumis/pull/1271)

### Breaking changes

1. Replace `lumis parsers fetch <names>` with `lumis languages cache <names>`.
2. Replace `lumis parsers update <names>` with `lumis languages cache --force <names>`.
3. Use `-H` for highlighted lines and `-h` for help.
4. Use `-V` for version and `-v` for verbose output.
5. Remove flags rejected by `lumis formatters show <name>`.

## [0.4.2](https://github.com/leandrocp/lumis/compare/cargo-lumis-cli/v0.4.1...cargo-lumis-cli/v0.4.2) (2026-07-23)


### Bug Fixes

- prevent query capture list pool overflow on large inputs - [#1028](https://github.com/leandrocp/lumis/pull/1028) by @ericmj


### Features

- add mdx language reusing the markdown parser - [#1012](https://github.com/leandrocp/lumis/pull/1012) by @benswift
- add dump commands - [#1087](https://github.com/leandrocp/lumis/pull/1087)
- Lumis JavaScript native (Rust bindings) - [#1083](https://github.com/leandrocp/lumis/pull/1083)

## [0.4.1](https://github.com/leandrocp/lumis/compare/cargo-lumis-cli/v0.4.0...cargo-lumis-cli/v0.4.1) (2026-07-11)

### Features

- config file [#998](https://github.com/leandrocp/lumis/pull/998)
- auto theme [#998](https://github.com/leandrocp/lumis/pull/998)

## [0.4.0](https://github.com/leandrocp/lumis/compare/cargo-lumis-cli/v0.3.0...cargo-lumis-cli/v0.4.0) (2026-06-12)

### ⚠ BREAKING CHANGES

- adopt the `l-` prefix on token class [#952](https://github.com/leandrocp/lumis/pull/952)

### Features

- add toon language - [#734](https://github.com/leandrocp/lumis/pull/734)
- rainbow brackets - [#949](https://github.com/leandrocp/lumis/pull/949)

## [0.3.0](https://github.com/leandrocp/lumis/compare/cargo-lumis-cli/v0.2.0...cargo-lumis-cli/v0.3.0) (2026-04-22)

### Bug Fixes

- remove stale gleam query node name causing `QueryError` - [#718](https://github.com/leandrocp/lumis/pull/718) by @stefanobaghino

### Features

- terminal: add bg color and width options - [#713](https://github.com/leandrocp/lumis/pull/713) by @Gazler
- terminal: add background theme option - [#723](https://github.com/leandrocp/lumis/pull/723)

## [0.2.0](https://github.com/leandrocp/lumis/compare/cargo-lumis-cli/v0.1.4...cargo-lumis-cli/v0.2.0) (2026-04-09)

### Features

- add -V as short for --verbose (#704)

### Bug Fixes

- BREAKING: restore -v version (#704)
- bundle theme extractor to fix `themes generate` (#704)

## [0.1.4](https://github.com/leandrocp/lumis/compare/rust-cli@v0.1.3...rust-cli@v0.1.4) (2026-04-06)

### Features

* align `lang` -&gt; `language` ([#591](https://github.com/leandrocp/lumis/issues/591)) ([ec9614d](https://github.com/leandrocp/lumis/commit/ec9614d7e6631ec9c5146e758a4b6e849446e876))

## [0.1.3](https://github.com/leandrocp/lumis/compare/rust-cli@v0.1.2...rust-cli@v0.1.3) (2026-04-04)

### Bug Fixes

* stop pinning lumis-core workspace deps ([ef2f002](https://github.com/leandrocp/lumis/commit/ef2f002eb60cd408872c5ce5f4dd7792bb00c557))

## [0.1.2](https://github.com/leandrocp/lumis/compare/rust-cli@v0.1.1...rust-cli@v0.1.2) (2026-04-03)

### Bug Fixes

* loosen lumis-core dep ([61a0e45](https://github.com/leandrocp/lumis/commit/61a0e45bc877f3a2f4652660e8c38a282b4d4552))
* query overrides and injections ([#551](https://github.com/leandrocp/lumis/issues/551)) ([2a356fb](https://github.com/leandrocp/lumis/commit/2a356fbd8f0841c7e0fdb5835330f98621c199ce))


## [0.1.1](https://github.com/leandrocp/lumis/compare/rust-cli@v0.1.0...rust-cli@v0.1.1) (2026-04-02)

### Bug Fixes

* cli seg fault ([#544](https://github.com/leandrocp/lumis/issues/544)) ([7948c64](https://github.com/leandrocp/lumis/commit/7948c64295580e5e9c8be9e041716a837da71e43))

## [0.1.0](https://github.com/leandrocp/lumis/compare/rust-cli@v0.0.3...rust-cli@v0.1.0) (2026-04-02)

### ⚠ BREAKING CHANGES

* bump tree-sitter 0.26 ([#521](https://github.com/leandrocp/lumis/issues/521))

### Miscellaneous Chores

* bump tree-sitter 0.26 ([#521](https://github.com/leandrocp/lumis/issues/521)) ([ad9a567](https://github.com/leandrocp/lumis/commit/ad9a5679e94032e153eb7d997f2c1577479ec812))

## [0.0.3](https://github.com/leandrocp/lumis/compare/rust-cli@v0.0.2...rust-cli@v0.0.3) (2026-03-31)

### Features

* add BBCode formatter ([@andrea](https://github.com/andrea)TP) ([#471](https://github.com/leandrocp/lumis/issues/471)) ([8fdce4d](https://github.com/leandrocp/lumis/commit/8fdce4dbff7fcd6cd943d049c9f46588a77e1aa5))
* add BBCode formatter ([@andrea](https://github.com/andrea)TP) ([#471](https://github.com/leandrocp/lumis/issues/471)) ([8fdce4d](https://github.com/leandrocp/lumis/commit/8fdce4dbff7fcd6cd943d049c9f46588a77e1aa5))
* add bbcode_scoped formatter to Elixir and Javascript ([#478](https://github.com/leandrocp/lumis/issues/478)) ([1a34357](https://github.com/leandrocp/lumis/commit/1a343574b0239b5100d6b170124b586b0aabecfe))

## [0.0.2](https://github.com/leandrocp/lumis/compare/rust-cli@v0.0.1...rust-cli@v0.0.2) (2026-03-24)

### Bug Fixes

* **cli:** gen themes lua execution ([#452](https://github.com/leandrocp/lumis/issues/452)) ([d55d7a1](https://github.com/leandrocp/lumis/commit/d55d7a141e500411b88c5d4cf12ac9d17ce33f2d))
* include unnamed children in range ([#430](https://github.com/leandrocp/lumis/issues/430)) ([782bfaa](https://github.com/leandrocp/lumis/commit/782bfaaf4ac86fd287541fcd1307157060846ffc))
* markdown inline injections ([#454](https://github.com/leandrocp/lumis/issues/454)) ([0ec8563](https://github.com/leandrocp/lumis/commit/0ec856302aa7a41dbf78b81eabdd6220db953fba))

## 0.0.1 - 2026-03-23

Initial release.

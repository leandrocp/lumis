## [0.14.0](https://github.com/leandrocp/lumis/compare/cargo-lumis/v0.13.1...cargo-lumis/v0.14.0) (2026-09-11)


### Bug Fixes

- escape attribute values in the Rust formatters - [#1385](https://github.com/leandrocp/lumis/pull/1385)
- reject a hex colour that is only partly hex - [#1394](https://github.com/leandrocp/lumis/pull/1394)
- BREAKING: make public error enums non-exhaustive - [#1395](https://github.com/leandrocp/lumis/pull/1395)


### Code Refactoring

- BREAKING: re-export lumis-core's formatters from lumis - [#1387](https://github.com/leandrocp/lumis/pull/1387)


### Features

- update generated themes - [#1358](https://github.com/leandrocp/lumis/pull/1358)
- update lang perl - [#1357](https://github.com/leandrocp/lumis/pull/1357)
- add Token themes - [#1365](https://github.com/leandrocp/lumis/pull/1365)
- update lang protobuf - [#1356](https://github.com/leandrocp/lumis/pull/1356)
- BREAKING: introduce annotations - [#1100](https://github.com/leandrocp/lumis/pull/1100)
- ANSI formatter helpers - [#1388](https://github.com/leandrocp/lumis/pull/1388)
- export the same helpers across runtimes - [#1396](https://github.com/leandrocp/lumis/pull/1396)

## [0.13.1](https://github.com/leandrocp/lumis/compare/cargo-lumis/v0.13.0...cargo-lumis/v0.13.1) (2026-09-03)


### Features

- highlight code inside diff hunks - [#1284](https://github.com/leandrocp/lumis/pull/1284)
- update lang diff - [#1290](https://github.com/leandrocp/lumis/pull/1290)
- update generated themes - [#1293](https://github.com/leandrocp/lumis/pull/1293)
- update lang graphql - [#1292](https://github.com/leandrocp/lumis/pull/1292)
- name the highlight options in every runtime - [#1289](https://github.com/leandrocp/lumis/pull/1289)
- update lang racket - [#1306](https://github.com/leandrocp/lumis/pull/1306)
- update generated themes - [#1307](https://github.com/leandrocp/lumis/pull/1307)
- opt into rainbow brackets from the flat token iterator - [#1333](https://github.com/leandrocp/lumis/pull/1333)
- update lang protobuf - [#1345](https://github.com/leandrocp/lumis/pull/1345)
- update lang matlab - [#1344](https://github.com/leandrocp/lumis/pull/1344)
- update generated themes - [#1343](https://github.com/leandrocp/lumis/pull/1343)
- update lang graphql - [#1342](https://github.com/leandrocp/lumis/pull/1342)

## [0.13.0](https://github.com/leandrocp/lumis/compare/cargo-lumis/v0.12.1...cargo-lumis/v0.13.0) (2026-08-13)

### Bug Fixes

- vendor strict-aliasing-unsafe parsers - [#1117](https://github.com/leandrocp/lumis/pull/1117)
- BREAKING: align the default theme on none across runtimes - [#1142](https://github.com/leandrocp/lumis/pull/1142)
- pin lumis crate requirements to the workspace version - [#1140](https://github.com/leandrocp/lumis/pull/1140)
- render attribute-less spans consistently - [#1259](https://github.com/leandrocp/lumis/pull/1259)
- highlight lines in light-dark() mode - [#1260](https://github.com/leandrocp/lumis/pull/1260)
- align the public API across Rust, the CLI, Elixir and JavaScript - [#1266](https://github.com/leandrocp/lumis/pull/1266)

### Documentation

- add release migration steps - [#1272](https://github.com/leandrocp/lumis/pull/1272)

### Features

- update lang fsharp - [#1101](https://github.com/leandrocp/lumis/pull/1101)
- update lang erlang - [#1105](https://github.com/leandrocp/lumis/pull/1105)
- update generated themes - [#1122](https://github.com/leandrocp/lumis/pull/1122)
- unify dynamic WASM language loading - [#1099](https://github.com/leandrocp/lumis/pull/1099)
- update lang wat - [#1125](https://github.com/leandrocp/lumis/pull/1125)
- update lang terraform - [#1124](https://github.com/leandrocp/lumis/pull/1124)
- update lang fsharp - [#1121](https://github.com/leandrocp/lumis/pull/1121)
- update lang csv - [#1120](https://github.com/leandrocp/lumis/pull/1120)
- update lang jinja_inline - [#1119](https://github.com/leandrocp/lumis/pull/1119)
- update lang scala - [#1221](https://github.com/leandrocp/lumis/pull/1221)

### Breaking changes

Why: catalog shapes now match.

1. `available_languages()` returns `Vec<LanguageInfo>` instead of `HashMap`.

## [0.12.1](https://github.com/leandrocp/lumis/compare/cargo-lumis/v0.12.0...cargo-lumis/v0.12.1) (2026-07-23)


### Bug Fixes

- prevent query capture list pool overflow on large inputs - [#1028](https://github.com/leandrocp/lumis/pull/1028) by @ericmj
- haskell heap corruption on aarch64-linux - [#1086](https://github.com/leandrocp/lumis/pull/1086) by @ericmj


### Features

- add mdx language reusing the markdown parser - [#1012](https://github.com/leandrocp/lumis/pull/1012) by @benswift
- upgrade lang qmljs - [#1008](https://github.com/leandrocp/lumis/pull/1008)
- upgrade lang perl - [#1006](https://github.com/leandrocp/lumis/pull/1006)
- upgrade lang elm - [#1005](https://github.com/leandrocp/lumis/pull/1005)
- upgrade lang latex - [#1004](https://github.com/leandrocp/lumis/pull/1004)
- upgrade lang dart - [#1003](https://github.com/leandrocp/lumis/pull/1003)
- add mdx to web-extra - [#1025](https://github.com/leandrocp/lumis/pull/1025)
- upgrade lang cmake - [#1002](https://github.com/leandrocp/lumis/pull/1002)
- update lang systemverilog - [#1082](https://github.com/leandrocp/lumis/pull/1082)
- update lang clojure - [#1080](https://github.com/leandrocp/lumis/pull/1080)
- Lumis JavaScript native (Rust bindings) - [#1083](https://github.com/leandrocp/lumis/pull/1083)

## [0.12.0](https://github.com/leandrocp/lumis/compare/cargo-lumis/v0.11.0...cargo-lumis/v0.12.0) (2026-07-08)


### Features

- upgrade lang nushell - [#990](https://github.com/leandrocp/lumis/pull/990)
- upgrade lang perl - [#988](https://github.com/leandrocp/lumis/pull/988)
- upgrade lang fsharp - [#984](https://github.com/leandrocp/lumis/pull/984)
- update generated themes - [#992](https://github.com/leandrocp/lumis/pull/992)
- update jinja and r langs - [#993](https://github.com/leandrocp/lumis/pull/993)
- expose open_multi_themes_pre_tag - [#994](https://github.com/leandrocp/lumis/pull/994)

## [0.11.0](https://github.com/leandrocp/lumis/compare/cargo-lumis/v0.10.0...cargo-lumis/v0.11.0) (2026-07-03)


### Bug Fixes

- preserve curly braces - [#982](https://github.com/leandrocp/lumis/pull/982)
- themes issues - [#977](https://github.com/leandrocp/lumis/pull/977) by @Fosox


### Features

- upgrade perl

## [0.10.0](https://github.com/leandrocp/lumis/compare/cargo-lumis/v0.9.0...cargo-lumis/v0.10.0) (2026-06-12)

### ⚠ BREAKING CHANGES

- adopt the `l-` prefix on token class [#952](https://github.com/leandrocp/lumis/pull/952)

### Bug Fixes

- use vim.pack to update themes - [#751](https://github.com/leandrocp/lumis/pull/751)
- fix build for glimmer and fortran

### Features

- add toon language - [#734](https://github.com/leandrocp/lumis/pull/734)
- rainbow brackets - [#949](https://github.com/leandrocp/lumis/pull/949)
- css stylesheet builder - [#951](https://github.com/leandrocp/lumis/pull/951)

## [0.9.0](https://github.com/leandrocp/lumis/compare/cargo-lumis/v0.8.0...cargo-lumis/v0.9.0) (2026-04-22)

### Bug Fixes

- remove stale gleam query node name causing `QueryError` - [#718](https://github.com/leandrocp/lumis/pull/718) by @stefanobaghino

### Features

- terminal: add bg color and width options - [#713](https://github.com/leandrocp/lumis/pull/713) by @Gazler
- terminal: add background theme option - [#723](https://github.com/leandrocp/lumis/pull/723)

## [0.8.0](https://github.com/leandrocp/lumis/compare/cargo-lumis/v0.7.0...cargo-lumis/v0.8.0) (2026-04-18)

### Bug Fixes

- queries override

### Code Refactoring

- BREAKING: deprecate ANSI iterator helpers in favor of `paint` - [#706](https://github.com/leandrocp/lumis/pull/706)

### Documentation

- move runnable examples into package dirs - [#709](https://github.com/leandrocp/lumis/pull/709)

## [0.7.0](https://github.com/leandrocp/lumis/compare/rust@v0.6.0...rust@v0.7.0) (2026-04-06)


### ⚠ BREAKING CHANGES

* rename bundle features `bundle-{name}` -> `lang-bundle-{name}` ([#679](https://github.com/leandrocp/lumis/issues/679))
* rename `formatter` -> `formatters` ([#678](https://github.com/leandrocp/lumis/issues/678))

### Features

* align `lang` -&gt; `language` ([#591](https://github.com/leandrocp/lumis/issues/591)) ([ec9614d](https://github.com/leandrocp/lumis/commit/ec9614d7e6631ec9c5146e758a4b6e849446e876))


### Bug Fixes

* rename `formatter` -&gt; `formatters` ([#678](https://github.com/leandrocp/lumis/issues/678)) ([138ba6d](https://github.com/leandrocp/lumis/commit/138ba6d8a968078d2009afe49943f38df7c119be))
* rename bundle features `bundle-{name}` -&gt; `lang-bundle-{name}` ([#679](https://github.com/leandrocp/lumis/issues/679)) ([ae7ce75](https://github.com/leandrocp/lumis/commit/ae7ce752a72dbfbf5177103d9d72efbd6ff3af9f))

## [0.6.0](https://github.com/leandrocp/lumis/compare/rust@v0.5.1...rust@v0.6.0) (2026-04-04)


### ⚠ BREAKING CHANGES

* improve formatter internals ([#570](https://github.com/leandrocp/lumis/issues/570))

### Code Refactoring

* improve formatter internals ([#570](https://github.com/leandrocp/lumis/issues/570)) ([c811832](https://github.com/leandrocp/lumis/commit/c811832c808c50f0ab6603c70d5403813e06476d))

## [0.5.1](https://github.com/leandrocp/lumis/compare/rust@v0.5.0...rust@v0.5.1) (2026-04-03)


### Features

* language bundles ([#556](https://github.com/leandrocp/lumis/issues/556)) ([6e98734](https://github.com/leandrocp/lumis/commit/6e98734c20d8a86384f3e8b8e2b694c40bbb0f12))


### Bug Fixes

* loosen lumis-core dep ([61a0e45](https://github.com/leandrocp/lumis/commit/61a0e45bc877f3a2f4652660e8c38a282b4d4552))
* query overrides and injections ([#551](https://github.com/leandrocp/lumis/issues/551)) ([2a356fb](https://github.com/leandrocp/lumis/commit/2a356fbd8f0841c7e0fdb5835330f98621c199ce))

## [0.5.0](https://github.com/leandrocp/lumis/compare/rust@v0.4.0...rust@v0.5.0) (2026-04-02)


### ⚠ BREAKING CHANGES

* bump tree-sitter 0.26 ([#521](https://github.com/leandrocp/lumis/issues/521))
* pass Language to highlight_iter callbacks ([#509](https://github.com/leandrocp/lumis/issues/509))

### Features

* add 135 themes ([#483](https://github.com/leandrocp/lumis/issues/483)) ([26b9d84](https://github.com/leandrocp/lumis/commit/26b9d84f096e5d9dd5064cf816f794fd578f5adf))
* add 42 new languages ([#467](https://github.com/leandrocp/lumis/issues/467)) ([9bacc46](https://github.com/leandrocp/lumis/commit/9bacc46552597f0b7ee5c3b8adde3bfc321a2097))
* add BBCode formatter ([@andrea](https://github.com/andrea)TP) ([#471](https://github.com/leandrocp/lumis/issues/471)) ([8fdce4d](https://github.com/leandrocp/lumis/commit/8fdce4dbff7fcd6cd943d049c9f46588a77e1aa5))
* add BBCode formatter ([@andrea](https://github.com/andrea)TP) ([#471](https://github.com/leandrocp/lumis/issues/471)) ([8fdce4d](https://github.com/leandrocp/lumis/commit/8fdce4dbff7fcd6cd943d049c9f46588a77e1aa5))
* add bbcode_scoped formatter to Elixir and Javascript ([#478](https://github.com/leandrocp/lumis/issues/478)) ([1a34357](https://github.com/leandrocp/lumis/commit/1a343574b0239b5100d6b170124b586b0aabecfe))
* add zsh and llvm languages ([#510](https://github.com/leandrocp/lumis/issues/510)) ([fe8bb2c](https://github.com/leandrocp/lumis/commit/fe8bb2ce76634b75ac5624166216112aba56915a))
* pass Language to highlight_iter callbacks ([#509](https://github.com/leandrocp/lumis/issues/509)) ([5cd345b](https://github.com/leandrocp/lumis/commit/5cd345baefe410a6966b8d855e87a440a1640c14))


### Bug Fixes

* add jinja_inline and remove disabled wasm parsers ([#508](https://github.com/leandrocp/lumis/issues/508)) ([4cb4527](https://github.com/leandrocp/lumis/commit/4cb45279783fd0eba25016c219a3a1feb8b09ac1))
* align parsers with nvim-treesitter query expectations ([#472](https://github.com/leandrocp/lumis/issues/472)) ([297cbfd](https://github.com/leandrocp/lumis/commit/297cbfda5b13694bfbc863093f98557481b91445))
* depend on lumis-core &gt;=0.0.1 ([61af49a](https://github.com/leandrocp/lumis/commit/61af49ae9a910c6abe248291eee3d9cee3e017e2))
* include unnamed children in range ([#430](https://github.com/leandrocp/lumis/issues/430)) ([782bfaa](https://github.com/leandrocp/lumis/commit/782bfaaf4ac86fd287541fcd1307157060846ffc))
* reduce package size ([#527](https://github.com/leandrocp/lumis/issues/527)) ([50844a5](https://github.com/leandrocp/lumis/commit/50844a5ab412a4f12c35c59af8eb3011e2496576))


### Miscellaneous Chores

* bump tree-sitter 0.26 ([#521](https://github.com/leandrocp/lumis/issues/521)) ([ad9a567](https://github.com/leandrocp/lumis/commit/ad9a5679e94032e153eb7d997f2c1577479ec812))

## [0.4.0](https://github.com/leandrocp/lumis/compare/rust@v0.3.1...rust@v0.4.0) (2026-04-02)


### ⚠ BREAKING CHANGES

* bump tree-sitter 0.26 ([#521](https://github.com/leandrocp/lumis/issues/521))

### Bug Fixes

* reduce package size ([#527](https://github.com/leandrocp/lumis/issues/527)) ([50844a5](https://github.com/leandrocp/lumis/commit/50844a5ab412a4f12c35c59af8eb3011e2496576))


### Miscellaneous Chores

* bump tree-sitter 0.26 ([#521](https://github.com/leandrocp/lumis/issues/521)) ([ad9a567](https://github.com/leandrocp/lumis/commit/ad9a5679e94032e153eb7d997f2c1577479ec812))

## [0.3.1](https://github.com/leandrocp/lumis/compare/rust@v0.3.0...rust@v0.3.1) (2026-03-31)


### Bug Fixes

* depend on lumis-core &gt;=0.0.1 ([61af49a](https://github.com/leandrocp/lumis/commit/61af49ae9a910c6abe248291eee3d9cee3e017e2))

## [0.3.0](https://github.com/leandrocp/lumis/compare/rust@v0.2.2...rust@v0.3.0) (2026-03-30)


### ⚠ BREAKING CHANGES

* pass Language to highlight_iter callbacks ([#509](https://github.com/leandrocp/lumis/issues/509))

### Features

* add 135 themes ([#483](https://github.com/leandrocp/lumis/issues/483)) ([26b9d84](https://github.com/leandrocp/lumis/commit/26b9d84f096e5d9dd5064cf816f794fd578f5adf))
* add zsh and llvm languages ([#510](https://github.com/leandrocp/lumis/issues/510)) ([fe8bb2c](https://github.com/leandrocp/lumis/commit/fe8bb2ce76634b75ac5624166216112aba56915a))
* pass Language to highlight_iter callbacks ([#509](https://github.com/leandrocp/lumis/issues/509)) ([5cd345b](https://github.com/leandrocp/lumis/commit/5cd345baefe410a6966b8d855e87a440a1640c14))


### Bug Fixes

* add jinja_inline and remove disabled wasm parsers ([#508](https://github.com/leandrocp/lumis/issues/508)) ([4cb4527](https://github.com/leandrocp/lumis/commit/4cb45279783fd0eba25016c219a3a1feb8b09ac1))

## [0.2.2](https://github.com/leandrocp/lumis/compare/rust@v0.2.1...rust@v0.2.2) (2026-03-27)


### Features

* add 42 new languages ([#467](https://github.com/leandrocp/lumis/issues/467)) ([9bacc46](https://github.com/leandrocp/lumis/commit/9bacc46552597f0b7ee5c3b8adde3bfc321a2097))
* add BBCode formatter to Rust and CLI ([@andreaTP](https://github.com/andreaTP)) ([#471](https://github.com/leandrocp/lumis/issues/471)) ([8fdce4d](https://github.com/leandrocp/lumis/commit/8fdce4dbff7fcd6cd943d049c9f46588a77e1aa5))
* add BBCode formatter to Elixir and Javascript ([#478](https://github.com/leandrocp/lumis/issues/478)) ([1a34357](https://github.com/leandrocp/lumis/commit/1a343574b0239b5100d6b170124b586b0aabecfe))


### Bug Fixes

* align parsers with nvim-treesitter query expectations ([#472](https://github.com/leandrocp/lumis/issues/472)) ([297cbfd](https://github.com/leandrocp/lumis/commit/297cbfda5b13694bfbc863093f98557481b91445))

## [0.2.1](https://github.com/leandrocp/lumis/compare/rust@v0.2.0...rust@v0.2.1) (2026-03-24)


### Bug Fixes

* include unnamed children in range ([#430](https://github.com/leandrocp/lumis/issues/430)) ([782bfaa](https://github.com/leandrocp/lumis/commit/782bfaaf4ac86fd287541fcd1307157060846ffc))

## 0.2.0 - 2026-03-23

### Breaking Changes

- **`highlight_iter` callback signature changed**: the callback now receives `(text, language, range, scope, style)` instead of `(text, range, scope, style)`. The new `language` parameter reports the active language (useful for language injections, e.g. JavaScript inside HTML `<script>` tags).
- **`span_inline` / `span_inline_attrs` parameter order changed**: `language` now comes before `scope` — `span_inline(text, language, scope, ...)` instead of `span_inline(text, scope, language, ...)`.

### Migration

Update `highlight_iter` callbacks:

```rust
// Before
highlight_iter(source, language, theme, |text, range, scope, style| { ... })

// After
highlight_iter(source, language, theme, |text, _language, range, scope, style| { ... })
```

Update `span_inline` calls:

```rust
// Before
span_inline(text, scope, Some(language), theme, false, false)

// After
span_inline(text, Some(language), scope, theme, false, false)
```

## 0.1.3 - 2026-02-20

### Changed
- Add language wat (@andreaTP)

## 0.1.2 - 2026-02-18

### Changed
- Add language nushell (started by @c4lliope)
- Rename CSS class from `athl` to `lumis` for consistency with the project name
- Rename CSS class from `athl-themes` to `lumis-themes` for multi-theme formatter
- Change default CSS variable prefix from `--athl` to `--lumis`

## 0.1.1 - 2026-01-27

### Removed
- Remove `elixir-nif` feature. The Elixir/Rustler bridge code is now maintained in the Elixir package itself.

## 0.1.0 - 2026-01-23

First release of `lumis`, a renamed and restructured version of `autumnus`.

### Migration from autumnus

Update your `Cargo.toml`:

```toml
# Before
[dependencies]
autumnus = "0.8"

# After
[dependencies]
lumis = "0.1"
```

Update your imports:

```rust
// Before
use autumnus::*;

// After
use lumis::*;
```

The API remains the same as `autumnus` v0.8.0 - only the crate and binary names have changed.

A deprecated `autumnus` v0.9.0 crate re-exports all types from `lumis` with deprecation warnings to facilitate migration.

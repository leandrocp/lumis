## [3.0.0](https://github.com/leandrocp/lumis/compare/cargo-lumis-core/v2.5.0...cargo-lumis-core/v3.0.0) (2026-09-11)


### Bug Fixes

- escape attribute values in the Rust formatters - [#1385](https://github.com/leandrocp/lumis/pull/1385)
- reject a hex colour that is only partly hex - [#1394](https://github.com/leandrocp/lumis/pull/1394)
- BREAKING: make public error enums non-exhaustive - [#1395](https://github.com/leandrocp/lumis/pull/1395)


### Code Refactoring

- BREAKING: re-export lumis-core's formatters from lumis - [#1387](https://github.com/leandrocp/lumis/pull/1387)


### Features

- update generated themes - [#1358](https://github.com/leandrocp/lumis/pull/1358)
- add Token themes - [#1365](https://github.com/leandrocp/lumis/pull/1365)
- BREAKING: introduce annotations - [#1100](https://github.com/leandrocp/lumis/pull/1100)
- HTML formatter helpers - [#1377](https://github.com/leandrocp/lumis/pull/1377)
- ANSI formatter helpers - [#1388](https://github.com/leandrocp/lumis/pull/1388)
- export the same helpers across runtimes - [#1396](https://github.com/leandrocp/lumis/pull/1396)

## [2.5.0](https://github.com/leandrocp/lumis/compare/cargo-lumis-core/v2.4.0...cargo-lumis-core/v2.5.0) (2026-09-03)


### Features

- highlight code inside diff hunks - [#1284](https://github.com/leandrocp/lumis/pull/1284)
- update generated themes - [#1293](https://github.com/leandrocp/lumis/pull/1293)
- update generated themes - [#1307](https://github.com/leandrocp/lumis/pull/1307)
- update generated themes - [#1343](https://github.com/leandrocp/lumis/pull/1343)

## [2.4.0](https://github.com/leandrocp/lumis/compare/cargo-lumis-core/v2.3.0...cargo-lumis-core/v2.4.0) (2026-08-13)

### Bug Fixes

- render attribute-less spans consistently - [#1259](https://github.com/leandrocp/lumis/pull/1259)
- highlight lines in light-dark() mode - [#1260](https://github.com/leandrocp/lumis/pull/1260)
- align the public API across Rust, the CLI, Elixir and JavaScript - [#1266](https://github.com/leandrocp/lumis/pull/1266)

### Features

- update generated themes - [#1122](https://github.com/leandrocp/lumis/pull/1122)

## [2.3.0](https://github.com/leandrocp/lumis/compare/cargo-lumis-core/v2.2.0...cargo-lumis-core/v2.3.0) (2026-07-23)


### Features

- add mdx language reusing the markdown parser - [#1012](https://github.com/leandrocp/lumis/pull/1012) by @benswift
- add mdx to web-extra - [#1025](https://github.com/leandrocp/lumis/pull/1025)

## [2.2.0](https://github.com/leandrocp/lumis/compare/cargo-lumis-core/v2.1.0...cargo-lumis-core/v2.2.0) (2026-07-08)


### Features

- update generated themes - [#992](https://github.com/leandrocp/lumis/pull/992)
- expose open_multi_themes_pre_tag - [#994](https://github.com/leandrocp/lumis/pull/994)

## [2.1.0](https://github.com/leandrocp/lumis/compare/cargo-lumis-core/v2.0.0...cargo-lumis-core/v2.1.0) (2026-07-03)


### Bug Fixes

- preserve curly braces - [#982](https://github.com/leandrocp/lumis/pull/982)
- themes issues - [#977](https://github.com/leandrocp/lumis/pull/977) by @Fosox

## [2.0.0](https://github.com/leandrocp/lumis/compare/cargo-lumis-core/v1.1.0...cargo-lumis-core/v2.0.0) (2026-06-12)

### ⚠ BREAKING CHANGES

- adopt the `l-` prefix on token class - [#952](https://github.com/leandrocp/lumis/pull/952)

### Bug Fixes

- use vim.pack to update themes - [#751](https://github.com/leandrocp/lumis/pull/751)

### Features

- add toon language - [#734](https://github.com/leandrocp/lumis/pull/734)
- rainbow brackets - [#949](https://github.com/leandrocp/lumis/pull/949)
- css stylesheet builder - [#951](https://github.com/leandrocp/lumis/pull/951)

## [1.1.0](https://github.com/leandrocp/lumis/compare/cargo-lumis-core/v1.0.0...cargo-lumis-core/v1.1.0) (2026-04-22)

### Features

- terminal: add bg color and width options - [#713](https://github.com/leandrocp/lumis/pull/713) by @Gazler
- terminal: add background theme option - [#723](https://github.com/leandrocp/lumis/pull/723)

## [1.0.0](https://github.com/leandrocp/lumis/compare/cargo-lumis-core/v0.1.0...cargo-lumis-core/v1.0.0) (2026-04-18)


### Code Refactoring

- BREAKING: deprecate ANSI iterator helpers in favor of `paint` - [#706](https://github.com/leandrocp/lumis/pull/706)

## [0.1.0](https://github.com/leandrocp/lumis/compare/rust-core@v0.0.5...rust-core@v0.1.0) (2026-04-06)


### ⚠ BREAKING CHANGES

* rename bundle features `bundle-{name}` -> `lang-bundle-{name}` ([#679](https://github.com/leandrocp/lumis/issues/679))

### Features

* align `lang` -&gt; `language` ([#591](https://github.com/leandrocp/lumis/issues/591)) ([ec9614d](https://github.com/leandrocp/lumis/commit/ec9614d7e6631ec9c5146e758a4b6e849446e876))


### Bug Fixes

* rename bundle features `bundle-{name}` -&gt; `lang-bundle-{name}` ([#679](https://github.com/leandrocp/lumis/issues/679)) ([ae7ce75](https://github.com/leandrocp/lumis/commit/ae7ce752a72dbfbf5177103d9d72efbd6ff3af9f))

## [0.0.5](https://github.com/leandrocp/lumis/compare/rust-core@v0.0.4...rust-core@v0.0.5) (2026-04-03)


### Maintenance

* fix release


## [0.0.4](https://github.com/leandrocp/lumis/compare/rust-core@v0.0.3...rust-core@v0.0.4) (2026-04-03)


### Features

* language bundles ([#556](https://github.com/leandrocp/lumis/issues/556)) ([6e98734](https://github.com/leandrocp/lumis/commit/6e98734c20d8a86384f3e8b8e2b694c40bbb0f12))

## [0.0.3](https://github.com/leandrocp/lumis/compare/rust-core@v0.0.2...rust-core@v0.0.3) (2026-03-30)


### Features

* add 135 themes ([#483](https://github.com/leandrocp/lumis/issues/483)) ([26b9d84](https://github.com/leandrocp/lumis/commit/26b9d84f096e5d9dd5064cf816f794fd578f5adf))
* add zsh and llvm languages ([#510](https://github.com/leandrocp/lumis/issues/510)) ([fe8bb2c](https://github.com/leandrocp/lumis/commit/fe8bb2ce76634b75ac5624166216112aba56915a))


### Bug Fixes

* add jinja_inline and remove disabled wasm parsers ([#508](https://github.com/leandrocp/lumis/issues/508)) ([4cb4527](https://github.com/leandrocp/lumis/commit/4cb45279783fd0eba25016c219a3a1feb8b09ac1))

## [0.0.2](https://github.com/leandrocp/lumis/compare/rust-core@v0.0.1...rust-core@v0.0.2) (2026-03-27)


### Features

* add 42 new languages ([#467](https://github.com/leandrocp/lumis/issues/467)) ([9bacc46](https://github.com/leandrocp/lumis/commit/9bacc46552597f0b7ee5c3b8adde3bfc321a2097))
* add BBCode formatter to Rust and CLI ([@andreaTP](https://github.com/andreaTP)) ([#471](https://github.com/leandrocp/lumis/issues/471)) ([8fdce4d](https://github.com/leandrocp/lumis/commit/8fdce4dbff7fcd6cd943d049c9f46588a77e1aa5))
* add BBCode formatter to Elixir and Javascript ([#478](https://github.com/leandrocp/lumis/issues/478)) ([1a34357](https://github.com/leandrocp/lumis/commit/1a343574b0239b5100d6b170124b586b0aabecfe))

## 0.0.1 - 2026-23-03

Initial release. Shared core logic for Lumis packages.

## [0.8.0](https://github.com/leandrocp/lumis/compare/hex-lumis/v0.7.0...hex-lumis/v0.8.0) (2026-09-03)


### Code Refactoring

- keep one copy of each test parser - [#1288](https://github.com/leandrocp/lumis/pull/1288)


### Documentation

- showcase page - [#1141](https://github.com/leandrocp/lumis/pull/1141)
- document Nix builds - [#1314](https://github.com/leandrocp/lumis/pull/1314)


### Features

- highlight code inside diff hunks - [#1284](https://github.com/leandrocp/lumis/pull/1284)
- update generated themes - [#1293](https://github.com/leandrocp/lumis/pull/1293)
- name the highlight options in every runtime - [#1289](https://github.com/leandrocp/lumis/pull/1289)
- update generated themes - [#1307](https://github.com/leandrocp/lumis/pull/1307)
- BREAKING: require Elixir 1.15 - [#1338](https://github.com/leandrocp/lumis/pull/1338)
- update generated themes - [#1343](https://github.com/leandrocp/lumis/pull/1343)

## [0.7.0](https://github.com/leandrocp/lumis/compare/hex-lumis/v0.6.3...hex-lumis/v0.7.0) (2026-08-13)

### Bug Fixes

- BREAKING: align the default theme on none across runtimes - [#1142](https://github.com/leandrocp/lumis/pull/1142)
- pin lumis crate requirements to the workspace version - [#1140](https://github.com/leandrocp/lumis/pull/1140)
- highlight lines in light-dark() mode - [#1260](https://github.com/leandrocp/lumis/pull/1260)
- resolve compatible WASM package versions - [#1263](https://github.com/leandrocp/lumis/pull/1263)
- resolve one data directory across every runtime - [#1264](https://github.com/leandrocp/lumis/pull/1264)
- align the public API across Rust, the CLI, Elixir and JavaScript - [#1266](https://github.com/leandrocp/lumis/pull/1266)

### Documentation

- add deployment guide - [#1268](https://github.com/leandrocp/lumis/pull/1268)
- add release migration steps - [#1272](https://github.com/leandrocp/lumis/pull/1272)

### Features

- update generated themes - [#1122](https://github.com/leandrocp/lumis/pull/1122)
- unify dynamic WASM language loading - [#1099](https://github.com/leandrocp/lumis/pull/1099)
- BREAKING: one vocabulary for the language and theme catalog, in every runtime - [#1269](https://github.com/leandrocp/lumis/pull/1269)
- mirror precompiled NIFs to Cloudflare R2 - [#1270](https://github.com/leandrocp/lumis/pull/1270)
- warm-up parsers - [#1273](https://github.com/leandrocp/lumis/pull/1273)

### Performance

- prepare large parser bundles concurrently - [#1271](https://github.com/leandrocp/lumis/pull/1271)

### Breaking changes

**Load languages on-demand**

No changes are required but it's recommended to warm-up parsers to avoid slow first requests.

See https://lumis.hexdocs.pm/deployment.html

**Removed default `onedark` theme**

Add `theme: "onedark"` to `:html_inline` and `:terminal` to keep old colors.

**Align Theme and Language APIs**

1. Replace `Lumis.available_languages()["elixir"]` with `Lumis.Languages.get("elixir")`.
2. Get theme names with `Enum.map(Lumis.available_themes(), & &1.name)`.

## [0.6.3](https://github.com/leandrocp/lumis/compare/hex-lumis/v0.6.2...hex-lumis/v0.6.3) (2026-07-23)


### Bug Fixes

- prevent query capture list pool overflow on large inputs - [#1028](https://github.com/leandrocp/lumis/pull/1028) by @ericmj
- keep Elixir NIF lockfile portable - [#1029](https://github.com/leandrocp/lumis/pull/1029)
- haskell heap corruption on aarch64-linux - [#1086](https://github.com/leandrocp/lumis/pull/1086) by @ericmj

## [0.6.2](https://github.com/leandrocp/lumis/compare/hex-lumis/v0.6.1...hex-lumis/v0.6.2) (2026-07-08)


### Features

- update generated themes - [#992](https://github.com/leandrocp/lumis/pull/992)
- update jinja and r langs - [#993](https://github.com/leandrocp/lumis/pull/993)

## [0.6.1](https://github.com/leandrocp/lumis/compare/hex-lumis/v0.6.0...hex-lumis/v0.6.1) (2026-07-03)


### Bug Fixes

- preserve curly braces - [#982](https://github.com/leandrocp/lumis/pull/982)
- themes issues - [#977](https://github.com/leandrocp/lumis/pull/977) by @Fosox

## [0.6.0](https://github.com/leandrocp/lumis/compare/hex-lumis/v0.5.0...hex-lumis/v0.6.0) (2026-06-12)

### ⚠ BREAKING CHANGES

- adopt the `l-` prefix on token class [#952](https://github.com/leandrocp/lumis/pull/952)

### Bug Fixes

- use vim.pack to update themes - [#751](https://github.com/leandrocp/lumis/pull/751)
- fix build for glimmer and fortran

### Features

- rainbow brackets - [#949](https://github.com/leandrocp/lumis/pull/949)
- css stylesheet builder - [#951](https://github.com/leandrocp/lumis/pull/951)

## [0.5.0](https://github.com/leandrocp/lumis/compare/hex-lumis/v0.4.0...hex-lumis/v0.5.0) (2026-04-22)

### Bug Fixes

- remove stale gleam query node name causing `QueryError` - [#718](https://github.com/leandrocp/lumis/pull/718) by @stefanobaghino

### Features

- terminal: add bg color and width options - [#713](https://github.com/leandrocp/lumis/pull/713) by @Gazler
- terminal: add background theme option - [#723](https://github.com/leandrocp/lumis/pull/723)

## [0.4.0](https://github.com/leandrocp/lumis/compare/hex-lumis/v0.3.0...hex-lumis/v0.4.0) (2026-04-18)

### Bug Fixes

- BREAKING: rename `formatter` -> `formatters` - [#678](https://github.com/leandrocp/lumis/pull/678)

### Documentation

- elixir examples
- move runnable examples into package dirs - [#709](https://github.com/leandrocp/lumis/pull/709)

### Features

- align `lang` -> `language` names - [#591](https://github.com/leandrocp/lumis/pull/591)

## [0.3.0](https://github.com/leandrocp/lumis/compare/elixir@v0.2.0...elixir@v0.3.0) (2026-04-05)

### Features

* align formatters to accept `:language`  ([#572](https://github.com/leandrocp/lumis/issues/572)) ([0c6e569](https://github.com/leandrocp/lumis/commit/0c6e569fa4918a2962a61cea47715e6f0b762ec2))

### Code Refactoring

* improve formatter internals ([#570](https://github.com/leandrocp/lumis/issues/570)) ([c811832](https://github.com/leandrocp/lumis/commit/c811832c808c50f0ab6603c70d5403813e06476d))


### Code Refactoring

* improve formatter internals ([#570](https://github.com/leandrocp/lumis/issues/570)) ([c811832](https://github.com/leandrocp/lumis/commit/c811832c808c50f0ab6603c70d5403813e06476d))

## [0.2.0](https://github.com/leandrocp/lumis/compare/elixir@v0.1.2...elixir@v0.2.0) (2026-04-02)


### ⚠ BREAKING CHANGES

* bump tree-sitter 0.26 ([#521](https://github.com/leandrocp/lumis/issues/521))

### Features

* add 135 themes ([#483](https://github.com/leandrocp/lumis/issues/483)) ([26b9d84](https://github.com/leandrocp/lumis/commit/26b9d84f096e5d9dd5064cf816f794fd578f5adf))
* add 42 new languages ([#467](https://github.com/leandrocp/lumis/issues/467)) ([9bacc46](https://github.com/leandrocp/lumis/commit/9bacc46552597f0b7ee5c3b8adde3bfc321a2097))
* add bbcode_scoped formatter to Elixir and Javascript ([#478](https://github.com/leandrocp/lumis/issues/478)) ([1a34357](https://github.com/leandrocp/lumis/commit/1a343574b0239b5100d6b170124b586b0aabecfe))


### Bug Fixes

* include unnamed children in range ([#430](https://github.com/leandrocp/lumis/issues/430)) ([782bfaa](https://github.com/leandrocp/lumis/commit/782bfaaf4ac86fd287541fcd1307157060846ffc))
* isolate elixir nif from rust release workspace ([#528](https://github.com/leandrocp/lumis/issues/528)) ([da1c6d1](https://github.com/leandrocp/lumis/commit/da1c6d127850ebc33697a502577e0b6c968e9825))


### Miscellaneous Chores

* bump tree-sitter 0.26 ([#521](https://github.com/leandrocp/lumis/issues/521)) ([ad9a567](https://github.com/leandrocp/lumis/commit/ad9a5679e94032e153eb7d997f2c1577479ec812))

## 0.1.2 - 2026-03-04

### Fixed
- Use published `lumis` crate

## 0.1.1 - 2026-02-19

### Changed
- Add language nushell (started by @c4lliope)
- Rename CSS class from `athl` to `lumis` for consistency with the project name
- Rename CSS class from `athl-themes` to `lumis-themes` for multi-theme formatter
- Change default CSS variable prefix from `--athl` to `--lumis`

## 0.1.0 - 2026-01-27

First release of `lumis`, a renamed version of the `autumn` package.

### Migration from autumn

Update your `mix.exs`:

```elixir
# Before
{:autumn, "~> 0.6"}

# After
{:lumis, "~> 0.1"}
```

Update your imports:

```elixir
# Before
alias Autumn
alias Autumn.Theme

# After
alias Lumis
alias Lumis.Theme
```

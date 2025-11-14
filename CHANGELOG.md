# Changelog

## Unreleased

**Important:** This release introduces several breaking changes. Please refer to the migration guide below for details on updating your code.

### Migration Guide

#### Formatter API Changes

**Before:**
```rust
use autumnus::{highlight, Options, FormatterOption, themes};

let code = "fn main() {}";
let options = Options {
    language: Some("rust"),
    formatter: FormatterOption::HtmlInline {
        theme: themes::get("dracula").ok(),
        pre_class: Some("code-block"),
        italic: false,
        include_highlights: false,
        highlight_lines: None,
        header: None,
    },
};
let html = highlight(code, options);
```

**After (direct initialization):**
```rust
use autumnus::{highlight, Options, HtmlInlineBuilder, languages::Language, themes};

let code = "fn main() {}";
let lang = Language::guess(Some("rust"), code);
let formatter = HtmlInlineBuilder::new()
    .lang(lang)
    .theme(themes::get("dracula").ok())
    .pre_class(Some("code-block"))
    .build()
    .unwrap();

let options = Options {
    source: code,
    language: Some("rust"),
    formatter: Box::new(formatter),
};
let html = highlight(options);
```

**After (builder pattern):**
```rust
use autumnus::{highlight, OptionsBuilder, HtmlInlineBuilder, languages::Language, themes};

let code = "fn main() {}";
let lang = Language::guess(Some("rust"), code);
let formatter = HtmlInlineBuilder::new()
    .lang(lang)
    .theme(themes::get("dracula").ok())
    .pre_class(Some("code-block"))
    .build()
    .unwrap();

let options = OptionsBuilder::new()
    .source(code)
    .language("rust")
    .formatter(Box::new(formatter))
    .build()
    .unwrap();

let html = highlight(options);
```

**Key Changes:**
1. Formatters are now configuration-only - no `.source()` method on builders
2. Source code is passed via `Options.source` field instead
3. `Options` can be created using direct initialization, `Options::new()`, or `OptionsBuilder`
4. This allows formatters to be reusable across multiple source strings

#### Function Signature Changes

- `highlight(source, options)` → `highlight(options)` where `options.source` contains the code
- `write_highlight(output, source, options)` → `write_highlight(output, options)` where `options.source` contains the code
- `Formatter::format(&self, output)` → `Formatter::format(&self, source, output)` for custom formatters

#### Theme and Language Changes (from previous releases)

- Remove `&` when storing themes: `let theme = themes::get("dracula")?;` (no longer returns a reference)
- Update `Language::guess()` calls to use `Option<&str>` for first parameter

### Changed
- **BREAKING**: Removed `HtmlFormatter` trait - use helper functions in `html` module instead (`html::open_pre_tag()`, `html::open_code_tag()`, `html::closing_tags()`)
- **BREAKING**: Removed `FormatterOption` enum - use builder pattern instead (`HtmlInlineBuilder`, `HtmlLinkedBuilder`, `TerminalBuilder`)
- **BREAKING**: Changed `highlight()` signature from `(source: &str, options: Options)` to `(options: Options)` - source now passed via `Options.source` field
- **BREAKING**: Changed `write_highlight()` signature from `(output: &mut dyn Write, source: &str, options: Options)` to `(output: &mut dyn Write, options: Options)` - source now passed via `Options.source` field
- **BREAKING**: Added required `source: &str` field to `Options` struct
- **BREAKING**: `Options.formatter` changed from `FormatterOption` to `Box<dyn Formatter>`
- **BREAKING**: Renamed `Options.lang_or_file` field to `Options.language` for clearer semantics
- **BREAKING**: Removed `.source()` method from formatter builders - formatters are now configuration-only objects
- **BREAKING**: Changed `Formatter::format()` signature to take `source: &str` parameter - custom formatters must update trait implementation
- **BREAKING**: Changed `Options::new()` signature from `new(source, language)` to `new(source, language, formatter)` - all three parameters now required
- **BREAKING**: Changed `Language::guess()` signature from `guess(&str, &str)` to `guess(Option<&str>, &str)`
  - `None` now explicitly means auto-detect from content
  - Empty string (`""`) defaults to `Language::PlainText`
  - Eliminates lossy `.unwrap_or("")` conversion
- **BREAKING**: `themes::get()` now returns owned `Theme` instead of `&'static Theme`
- **BREAKING**: Removed `'a` lifetime parameter from formatters where only used for theme

### Added
- `OptionsBuilder` for fluent API to build `Options` with builder pattern
- `Default` implementation for `Options` (empty source, no language hint, HTML inline formatter)
- New `highlight` module with ergonomic API for building custom formatters:
  - `Highlighter` - Stateful highlighter for repeated operations
  - `HighlightIterator` - Lazy iterator for streaming access with position information
  - `highlight_iter()` - Convenient function to create iterators
- Helper functions in `html` module for HTML generation:
  - `html::open_pre_tag()` - Generate opening `<pre>` tag with optional class and theme styles
  - `html::open_code_tag()` - Generate opening `<code>` tag with language class
  - `html::closing_tags()` - Generate closing `</code></pre>` tags
- Builder pattern for formatters: `HtmlInlineBuilder`, `HtmlLinkedBuilder`, `TerminalBuilder`
- Custom formatters can now be implemented via the `Formatter` trait
- Implemented `FromStr` trait for `Language` enabling `.parse()` method
- Added `LanguageParseError` type for parse failures
- Language parsing now supports language names, file extensions, and file paths via `FromStr`
- Implemented `FromStr` trait for `Theme` enabling `.parse()` method
- Added `ThemeParseError` type for parse failures
- Added 7 comprehensive examples: `basic_highlighting`, `custom_theme`, `terminal_colors`, `custom_formatter`, `iterator_api`, `line_by_line`, `html_with_classes`

## [0.7.8] - 2025-11-13

### Changed
- Update languages: angular, powershell, latex
- Update themes: onedark_cool, onedark_darker, onedark_deep, onedark_light, onedark_warmer

## [0.7.7] - 2025-10-27

### Changed
- Update Python queries from upstream nvim-treesitter (@DolceTriade)
- Update tree-sitter-vue parser
- Update tree-sitter-angular parser
- Update tree-sitter-perl parser
- Update tree-sitter-dart parser

## [0.7.6] - 2025-10-22

### Fixed
- High CPU usage in Nix language queries (@DolceTriade)
- Fix default trait

### Changed
- Update themes: catppuccin_frappe, catppuccin_latte, catppuccin_macchiato, catppuccin_mocha, matte_black, modus_operandi
- Update CSS files
- Update samples

## [0.7.5] - 2025-10-10

### Added
- Add Typst language (@mylanconnolly)

### Changed
- Update rustler to 0.37.0
- Update tree-sitter-erlang to 0.15.0
- Update tree-sitter-sequel to 0.3.11
- Update tree-sitter-yaml to 0.7.2

## [0.7.4] - 2025-09-30

### Changed
- Relax tree-sitter requirement to v0.25
- Updated languages: latex, markdown, powershell, c_sharp, fsharp, go, make, ocaml, proto, python, scala, zig, css, proto

## [0.7.3] - 2025-08-20

### Added
- Add common Elixir sigils injections

### Changed
- Update tree-sitter-php to v0.24.2
- Update PHP queries

## [0.7.2] - 2025-08-20

### Added
- Add `matte_black` theme from [matteblack.nvim](https://github.com/tahayvr/matteblack.nvim)

## [0.7.1] - 2025-08-09

### Added
- Add `--color` option to `autumn dump-tree-sitter` command for colored AST output

### Changed
- Sync vendored parsers with nvim-treesitter repo
- Add language markdown-inline
- Update parsers: angular, latex, llvm, markdown, perl, vim
- Update queries: c, csharp, ecma, fsharp, javascript, php, powershell, swift, tsx
- Improve `autumn dump-tree-sitter` output to display field names and match Neovim's `:InspectTree` format while preserving raw text tokens

## [0.7.0] - 2025-07-26

### Added
- Add `--highlight-lines` option to autumn CLI for highlighting specific line ranges

### Changed
- **Breaking** Change HTML line containers from `<span>` elements to `<div>` elements in both HTML inline and linked formatters
- **Breaking** Remove transition, display, and width fields from theme's `Style` struct
- **Breaking** Revert to use `CursorLine` highlight group to highlight lines in HTML formatters

## [0.6.0] - 2025-07-23

### Added
- Add `class` field to `HighlightLines` in HTML inline formatter for custom CSS classes on highlighted lines
- Add `highlighted` style to all themes with CSS properties to properly style highlighted lines
- Add `display`, `width`, and `transition` fields to theme styles for extended styling capabilities
- Add language `caddy`
- Add language `fish`

### Changed
- Map Neovim's `Visual` highlight group to `highlighted` style in theme extraction
- Update all theme JSON files to include `highlighted` style derived from Visual highlight with CSS properties
- Update all CSS files to include `.highlighted` class for HTML linked formatter
- Update elixir-nif `ExStyle` struct to include `display`, `width`, and `transition` fields
- **Breaking** Change `HighlightLines.style` field from `HighlightLinesStyle` to `Option<HighlightLinesStyle>` allowing users to define either style or class for highlighted lines
- **Breaking** Rename feature flag `elixir` to `elixir-nif` for clarity
- **Breaking** Remove `visual` and `cursorline` theme style in favor of `highlighted` for clarity

### Fixed
- Fix missing style fields in elixir-nif module for proper theme style conversion

## [0.5.0] - 2025-07-07

### Changed
- **Breaking** Change formatter builders to use the mutable pattern
- **Breaking** Builders `theme` and `pre_class` arguments changed to `Option`
- **Breaking** Builder `build()` method now returns a `Result` requiring `.unwrap()` or proper error handling
- **Breaking** Line highlighting now uses "visual" theme style instead of "cursorline" for consistency

## [0.4.0] - 2025-06-19

### Changed
- **Breaking** Require Rust 1.86 or later
- Update to Rust edition 2021
- Update parsers: angular, c, cmake, comment, hcl, liquid, llvm, ocaml, perl, vim, vue, yaml
- Update queries: cmake, elm, fsharp, html, latex, php, vue
- Update themes: flexoki, modus operandi, moonfly, nightfly
- Add Elixir ~V live_svelte injection
- Deprecate `with_*` methods in favor of builder pattern

### Added
- Add formatter builders: HtmlInlineBuilder, HtmlLinkedBuilder, and TerminalBuilder
- Add `header` option to HTML formatters for wrapping code blocks with custom HTML elements
- Add individual language feature flags (`lang-*`) for selective compilation to reduce build times
- Add `all-languages` convenience feature flag to enable all language support
- Add `highlight_lines` option to HTML formatters for highlighting specific lines
- Add `header` option to HTML formatters for wrapping html tags around code blocks
- Add cursorline highlight in themes
- Add languages: assembly, dart
- Add themes: horizon, horizon_dark, horizon_light, iceberg, molokai, moonlight, nordfox, papercolor_dark, papercolor_light, srcery, zenburn
- Add query overwrite system for customizing syntax highlighting

### Fixed
- Fix unsafe extern declarations

## [0.3.2] - 2025-05-21

### Changed
- Update CSS files

### Added
- Add neovim light and dark themes (@mhanberg)

## [0.3.1] - 2025-05-02

### Changed
- Update dependencies
- Update parsers
- Update queries

## [0.3.0] - 2025-04-18

### Changed
- Improve API structure and organization
- **Breaking:** Change `new` function API for formatters

### Added
- Add `nix` language (@kivikakk)
- Add `write_highlight` to write highlighted code into a Write
- Add `elixir` module and feature flag to expose Rustler related code

## [0.2.0] - 2025-04-08

### Changed
- Expose `open_pre_tag`, `open_code_tag`, and `closing_tags` in `HtmlFormatter` trait
- **Breaking:** Move `theme` field from `Options` to `FormatterOption` enum variants

## [0.1.10] - 2025-04-07

### Changed
- Expose Formatters functions for external use

### Removed
- **Breaking:** Remove unused `italic` option from `Terminal` formatter

## [0.1.9] - 2025-03-14

### Changed
- Update tree-sitter-erlang to v0.13.0
- Allow empty themes - change option `theme` to `Option`

### Removed
- Remove /utf-8 flag for msvc

## [0.1.8] - 2025-03-13

### Changed
- Use same parser version/revision as nvim-treesitter
- Update themes
- Update samples

### Added
- Add Vue language support
- Add HCL language support

### Fixed
- Fix scope constants based on nvim-treesitter CONTRIBUTING.md
- Fix highlights

## [0.1.7] - 2025-03-09

### Changed
- Make language optional and move to `Options`
- Rename `lang_or_path` to `lang_or_file`
- Rename option `include_highlight` to `include_highlights`
- Change types `&str` to `String` in `Options`

### Removed
- Remove options `italic` and `include_highlights` from `HtmlLinked`

### Fixed
- Fix uppercase language name guessing

## [0.1.6] - 2025-03-08

### Fixed
- Fix theme colors and CSS

## [0.1.5] - 2025-03-07

### Changed
- Move formatter-specific options (pre_class, italic, include_highlight) from `Options` to their respective formatter structs (`HtmlInline`, `HtmlLinked`, `Terminal`)

### Added
- Add `languages::available_languages()` to get a map of all supported languages with their details
- Add `themes::available_themes()` to get a list of all available themes

## [0.1.4] - 2025-03-06

### Fixed
- Fix theme path building relative to CARGO_MANIFEST_DIR
- Fix documentation: exclude dev binary from docs
- Fix documentation: remove unnecessary empty default features

## [0.1.3] - 2025-03-05

### Fixed
- Fix docs: generate link to def

## [0.1.2] - 2025-03-05

### Fixed
- Fix doc_auto_cfg

## [0.1.1] - 2025-03-05

### Fixed
- Fix docs config

## [0.1.0] - 2025-03-05

### Added
- Add initial release with core functionality

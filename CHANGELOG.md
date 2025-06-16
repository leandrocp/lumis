# Changelog

## Unreleased

### Added
- Add `header` option to HTML formatters for wrapping code blocks with custom HTML elements
- Add shared `HtmlElement` struct for configuring HTML wrapper elements
- Add `highlight_lines` and `header` fields to Elixir bindings for HTML formatters

### Improvements
- Updated parsers: angular, c, cmake, comment, hcl, liquid, llvm, ocaml, perl, vim, vue, yaml
- Updated queries: cmake, elm, fsharp, html, latex, php, vue
- Updated themes: flexoki, modus operandi, moonfly, nightfly

## [0.3.2] - 2025-05-21

### Improvements
- Add neovim light and dark themes - @mhanberg
- Update CSS files

## [0.3.1] - 2025-05-02

### Improvements
- Update dependencies
- Update parsers
- Update queries

## [0.3.0] - 2025-04-18

### Added
- Added `nix` language - @kivikakk
- Added `write_highlight` to write highlighted code into a Write
- Added `elixir` module and feature flag to expose Rustler related code

### Changed
- Improved API structure and organization

### Breaking Changes
- [Formatters] Changed `new` function API

## [0.2.0] - 2025-04-08

### Changed
- Exposed `open_pre_tag`, `open_code_tag`, and `closing_tags` in `HtmlFormatter` trait

### Breaking Changes
- Moved `theme` field from `Options` to `FormatterOption` enum variants

## [0.1.10] - 2025-04-07

### Changed
- Exposed Formatters functions for external use

### Breaking Changes
- Remove option `italic` from `Terminal` formatter (was not used)

## [0.1.9] - 2025-03-14

### Changed
- Updated tree-sitter-erlang to v0.13.0
- Allow empty themes - changed option `theme` to `Option`
- Removed /utf-8 flag for msvc

## [0.1.8] - 2025-03-13

### Added
- Vue
- HCL

### Fixed
- Scope constants based on nvim-treesitter CONTRIBUTING.md
- Highlights

### Changed
- Use same parser version/revision as nvim-treesitter
- Updated themes
- Updated samples

## [0.1.7] - 2025-03-09

### Fixed
- Guess uppercase language names

### Changed
- Make language optional and move to `Options`
- Rename `lang_or_path` to `lang_or_file`
- Rename option `include_highlight` to `include_highlights`
- Change types `&str` to `String` in `Options`
- Remove options `italic` and `include_highlights` from `HtmlLinked`

## [0.1.6] - 2025-03-08

### Fixed
- Some theme colors and CSS

## [0.1.5] - 2025-03-07

### Added
- `languages::available_languages()`: Get a map of all supported languages with their details
- `themes::available_themes()`: Get a list of all available themes

### Changed
- Moved formatter-specific options (pre_class, italic, include_highlight) from `Options` to their respective formatter structs (`HtmlInline`, `HtmlLinked`, `Terminal`)

## [0.1.4] - 2025-03-06

### Fixed
- Build theme path relative to CARGO_MANIFEST_DIR
- Documentation: exclude dev binary from docs
- Documentation: remove unnecessary empty default features

## [0.1.3] - 2025-03-05

### Fixed
- Docs: generate link to def

## [0.1.2] - 2025-03-05

### Fixed
- doc_auto_cfg

## [0.1.1] - 2025-03-05

### Fixed
- Docs config

## [0.1.0] - 2025-03-05

### Added
- Initial release with core functionality

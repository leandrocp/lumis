# General Guidelines
- Do not add code comments or explanations in the codebase unless explicitly requested
- Run `cargo test -- --nocapture {test_name}` after changes to ensure all tests pass; Fix any failing tests
- Run `cargo test` (the whole test suite) only when a lot of files change
- Run `cargo doc` after doc changes; Fix any warnings or errors in the documentation
- Run `cargo clippy -- -D warnings` eventually to check for linting issues and fix any warnings and errors
- Include changes in `CHANGELOG.md` following the Common Changelog format (https://common-changelog.org)

## Commands
- `just` is used for common development tasks; use it when a request involves running such commands
- `cargo run --bin autumn` is the CLI tool for Autumnus
- Use `--help` to learn more about `autumn` bin commands, for eg: `cargo run --bin autumn highlight --help`

## Non-standard Directories
- `vendored_parsers/`: Tree-sitter parser and grammar for additional languages not included in `Cargo.toml`
- `queries/`: Tree-sitter query files for syntax highlighting with inheritance and overwriting support
- `themes/`: Neovim theme definitions as JSON files
- `css/`: Generated CSS files for HTML linked formatter
- `samples/`: Generated HTML samples for some language/theme combinations

## Tree-sitter Integration
- Uses both crate-based parsers (in `Cargo.toml`) and vendored parsers (in `vendored_parsers/`)
- Vendored parsers are necessary for languages not yet available as crates or needing custom modifications
- Query files support inheritance (`;inherits: language1,language2`) and override system

## Theme System
- Themes are extracted from Neovim colorschemes using Lua scripts in `themes/`
- Each theme is a JSON file defining colors for syntax highlighting scopes
- CSS generation creates stylesheets for HTML linked formatter
- Themes are lazily loaded as static constants

## Adding New Languages from crates.io
1. Search parser in https://crates.io
2. Add Tree-sitter parser dependency to `Cargo.toml`
3. Gate language features with `#[cfg(feature = "lang-{name}")]` in the codebase
4. Follow "Updating language.rs to add a new language"
5. Follow "Adding New Queries"

## Adding New vendorized Languages
1. To vendor you must include the repo into `update-parses` in `justfile` and run `just update-parser <repo-name>`, eg: `just update-parsers tree-sitter-dart`
2. Add language in function `vendored_parsers` in `build.rs`
3. Run `cargo build`
4. Add language in `extern "C"` block in `src/languages.rs`
5. Follow "Updating language.rs to add a new language"
5. Follow "Adding New Queries"

## Updating language.rs to add a new language
- Fetch https://github.com/Wilfred/difftastic/blob/master/src/parse/guess_language.rs to learn the language detection logic
- Update `src/languages.rs` to include the new language:
  - Add the new language in `pub enum Language`
  - Add the new language in `pub fn guess`
  - Add the new language in `pub fn language_globs`
  - Add the new language in `pub fn name`
  - Add the new language in `pub fn config`
  - Add the static language config as `<LANGUAGE>_CONFIG`

## Adding New Queries
- Copy query files from https://github.com/nvim-treesitter/nvim-treesitter/tree/master/queries into `queries/<language>/` directory (copy only highlights.scm, injections.scm, and locals.scm)
- Add language in function `queries` in `build.rs`

## Adding New Themes
- Add theme definition in `themes/<theme-name>.json`
- Run `just gen-css` to generate CSS file
- Theme becomes automatically available through the theme system

## Important Notes
- Features: `elixir` (for Rustler NIF), `dev` (for development tools)
- Overwrites in `overwrites/` directory can modify or extend query files
- CSS files are generated and should not be manually edited

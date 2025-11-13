# General Guidelines
- Do not add code comments or explanations in the codebase unless explicitly requested
- Run `cargo test -- --nocapture {test_name}` after changes to ensure all tests pass; Fix any failing tests
- Run `cargo test --all-features` (the whole test suite) only when a lot of files change
- Run `cargo doc` after doc changes; Fix any warnings or errors in the documentation
- Run `cargo clippy --all-features -- -D warnings` eventually to check for linting issues and fix any warnings and errors

## Changelog
- Follow the https://common-changelog.org format
- Create a "## Unreleased" section if it doesn't exist yet
- Add new entries into the "## Unreleased" section
- Precise and concise descriptions but always complete
- Review existing entries for accuracy and clarity
- Fetch commits from latest release tag to HEAD

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
6. Include language in the list of languages in `lib.rs` under `//! ## Languages available`

## Adding New vendorized Languages
1. To vendor you must include the repo into `update-parses` in `justfile` and run `just update-parser <repo-name>`, eg: `just update-parsers tree-sitter-dart`
2. Add language in function `vendored_parsers` in `build.rs`
3. Run `cargo build`
4. Add language in `extern "C"` block in `src/languages.rs`
5. Follow "Updating language.rs to add a new language"
6. Follow "Adding New Queries"
7. Include language in the list of languages in `lib.rs` under `//! ## Languages available`

## Updating language.rs to add a new language
- Fetch https://github.com/Wilfred/difftastic/blob/master/src/parse/guess_language.rs to learn the language detection logic
- Update `src/languages.rs` to include the new language:
  - Add the new language in `pub enum Language`
  - Add the new language in `impl FromStr for Language` (for parsing language names/extensions)
  - Add the new language in `pub fn language_globs` and fetch file extension from difftastic repo or from the vendored parser repo file package.json or tree-sitter.json
  - Add the new language in `pub fn name`
  - Add the new language in `pub fn config`
  - Add the static language config as `<LANGUAGE>_CONFIG`
  - Add the new language into feature `all-languages` in `Cargo.toml`
  - Write a test `test_{lang_name}_config_loads`

## Update Language documentation
- Add the new language in section `//! ## Languages available` in `src/lib.rs`

## Adding New Queries
- Copy query files from https://github.com/nvim-treesitter/nvim-treesitter/tree/main/runtime/queries into `queries/<language>/` directory (copy only highlights.scm, injections.scm, and locals.scm)
- Add language in function `queries` in `build.rs`

## Adding New Themes
1. Search for the Neovim plugin that provides the theme, for eg: `dracula` is provided by https://github.com/Mofiqul/dracula.nvim
2. Fetch the theme repo README to understand how to install and configure it. Note: look for the Lazy package manager installation instructions
3. Add each variation as a separate theme into `themes/themes.lua`, for eg: the colorscheme `dracula-soft` is added as theme `dracula_soft`
4. Run `just gen-themes <theme-name>` to generate the theme JSON file
5. Run `just gen-css` to generate CSS file
6. Add the new theme in section `//! ## Themes available` in `src/lib.rs`
7. Update `CHANGELOG.md` with the new theme details (follow `## Changelog` guidelines)

## Adding New Samples
1. Fetch example from https://github.com/adambard/learnxinyminutes-docs located at https://raw.githubusercontent.com/adambard/learnxinyminutes-docs/refs/heads/master/<language>.md
2. Extract example from code block and save it in `samples/<language>.<ext>`, for eg Elixir sample starts with ```elixir in https://raw.githubusercontent.com/adambard/learnxinyminutes-docs/refs/heads/master/elixir.md and is saved as `samples/elixir.exs`.
  Notes:
    - Concat multiple code blocks into a single file if necessary.
    - If the repo Learn X in Y Minutes does not have a sample for the language, create a comprehensive sample file demoing the language features.
3. Add the language in the list of languages in `samples/index.html` in `const languages = [...]`
4. Run `echo y | just gen-samples` to generate the HTML samples

## Important Notes
- Features: `elixir-nif` (for Rustler NIF), `dev` (for development tools)
- Overwrites in `overwrites/` directory can modify or extend query files
- CSS files are generated and should not be manually edited

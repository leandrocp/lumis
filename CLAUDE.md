# General Guidelines
- Do not add code comments or explanations in the codebase unless explicitly requested
- Run `cargo test -- --nocapture {test_name}` after changes to ensure all tests pass; Fix any failing tests
- Run `cargo test` (the whole test suite) only when a lot of files change
- Run `cargo doc` after doc changes; Fix any warnings or errors in the documentation
- Run `cargo clippy -- -D warnings` eventually to check for linting issues and fix any warnings
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

## Adding New Languages
1. Add Tree-sitter parser dependency to `Cargo.toml` or vendor in `vendored_parsers/`
2. Add queries in `queries/<language>/` directory (highlights.scm, injections.scm, locals.scm)
3. Update language detection in `src/languages.rs`

## Adding New Themes
1. Add theme definition in `themes/<theme-name>.json`
2. Run `just gen-css` to generate CSS file
3. Theme becomes automatically available through the theme system

## Important Notes
- Features: `elixir` (for Rustler NIF), `dev` (for development tools)
- Overwrites in `overwrites/` directory can modify or extend query files
- CSS files are generated and should not be manually edited

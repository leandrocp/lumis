## Architecture
- **Main API**: `lib/lumis.ex` - Primary public interface with `highlight/2` and `highlight!/2` functions
- **Rust NIF**: `native/lumis_nif/` - Rust NIF wrapper using rustler that interfaces with the lumis Rust crate
- **Core Engine**: Uses the `lumis` crate which leverages Tree-sitter for parsing 70+ languages with 120+ Neovim themes
- **Theme System**: `lib/lumis/theme.ex` - Theme management and style definitions
- **CSS Assets**: `priv/static/css/` - Pre-generated CSS files for the html_linked formatter
- **Precompiled Binaries**: Uses `rustler_precompiled` for distributing precompiled NIFs across multiple platforms

The library supports four output formatters:
- `:html_inline` - HTML with inline styles (default)
- `:html_linked` - HTML with CSS classes (requires linking CSS from `priv/static/css/`)
- `:html_multi_themes` - HTML with CSS custom properties for multiple themes (light/dark mode support)
- `:terminal` - ANSI escape codes for terminal output

Lines are wrapped in `<div class="line" data-line="N">` elements

### Supporting Modules
- `lib/lumis/native.ex` - RustlerPrecompiled configuration and NIF stubs
- `lib/lumis/errors.ex` - Exception definitions (Lumis.HighlightError)
- `lib/lumis/nif_structs.ex` - Internal structs for NIF communication (HtmlElement, HtmlInlineHighlightLines, HtmlLinkedHighlightLines)
- `lib/lumis/comptime_utils.ex` - Compile-time CPU capability detection for optimized binary selection

### Rust Implementation
- Located in `native/lumis_nif/`
- Minimal NIF wrapper that delegates to the `lumis` crate
- Cargo.toml specifies `lumis` with features: `all-languages`, `nif_version_2_15`, `elixir-nif`
- All NIFs are scheduled on DirtyCPU for non-blocking operation
- Precompiled for multiple platforms (see `lib/lumis/native.ex` for target list)

## Elixir
- Read and think about these guidelines before making a plan: https://hexdocs.pm/elixir/code-anti-patterns.html, https://hexdocs.pm/elixir/design-anti-patterns.html, https://hexdocs.pm/elixir/process-anti-patterns.html, and https://hexdocs.pm/elixir/macro-anti-patterns.html
- Run `mix compile --all-warnings` to make sure changes are correct and fix any warnings
- Run `mix format` after making changes, but only after checking compilation warnings
- Try to remove dead code on `warning: variable "x" is unused`
- Prefer running specific tests with `mix test {file:line}` over the whole suite like `mix test`
- Prefer changing existing files or modules related to the topic in the context instead of creating new files or modules
- Place private functions near to where they are used and avoid placing them at the end of the module
- Do not add code comments unless I explicitly ask you to,
- Do not remove existing code comments unless I explicitly ask you to
- Write concise `@moduledoc` and `@doc` for modules and functions

## Rust
- From the root dir, run `mix list.rust` and `test.rust` to check changes or the regular `cargo` commands if working directly in the `native/lumis_nif/` dir

## Changelog
- Follow the https://common-changelog.org format
- Create a "## Unreleased" section if it doesn't exist yet
- Add new entries into the "## Unreleased" section
- Short and concise descriptions
- Review existing entries for accuracy and clarity
- Fetch commits from latest release tag to HEAD

## Important Notes
- Options are defined using nimble_options so fetch https://hexdocs.pm/nimble_options/NimbleOptions.html before making changes

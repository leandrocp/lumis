# lumis-cli

CLI for [Lumis](https://github.com/leandrocp/lumis), a syntax highlighter powered by Tree-sitter and Neovim themes.

The installed binary is `lumis`.

## Features

- **110+ Tree-sitter languages** - Fast, accurate, and updated syntax parsing
- **250+ built-in Neovim themes** - Updated and curated themes from the Neovim community
- **Built-in formatters** - HTML (inline/linked), Terminal (ANSI), Multi-theme (light/dark), BBCode
- **Custom formatters** - Build your own output
- **Language auto-detection** - File extension, shebang, and emacs-mode support
- **Line highlighting** - Mark and style individual lines, with custom HTML wrappers
- **Streaming-friendly** - Handles incomplete code
- **Load parsers on demand** - Verified and cached, including injected languages

## Install

```sh
curl -LsSf https://lumis.sh/install.sh | sh
```

Other installation methods, including PowerShell, Homebrew, `cargo-binstall`,
npm, and Cargo, are listed in the [installation guide](https://docs.lumis.sh/cli/install).

From npm or crates.io:

```sh
npx @lumis-sh/cli --help
npm install -g @lumis-sh/cli
cargo install lumis-cli
```

## Commands

```text
lumis highlight          Highlight a file or stdin
lumis dump tree          Print configurable Tree-sitter syntax trees
lumis dump events        Print raw highlight events as JSON
lumis languages list     List supported language ids and file patterns
lumis languages show     Print what the catalog knows about one language
lumis languages cache    Download and compile parsers so later runs skip both
lumis themes list        List built-in themes and custom themes in the data dir
lumis themes show        Print one theme's appearance and colors
lumis themes generate    Extract a theme JSON file from a Neovim colorscheme repo
```

## Usage

```sh
lumis highlight src/main.rs
lumis highlight -t dracula -f html-inline src/main.rs -o out.html
lumis highlight -l python <<< 'x = 1'
```

Terminal output is the default. The language comes from the filename; `-l` is
only needed when reading stdin.

Parsers download on demand into the data directory (`LUMIS_DATA_DIR`, otherwise
the platform default) and are verified before use. `lumis languages cache` fetches
and compiles them ahead of time, and every Lumis runtime reads that same
directory, so a cache prepared here also starts Elixir and Node warm.

## Documentation

- [CLI commands](https://docs.lumis.sh/cli/commands) — every command, flag and default
- [CLI highlighting](https://docs.lumis.sh/usage/cli-highlight) and [behavior](https://docs.lumis.sh/usage/cli-behavior)
- [Themes](https://docs.lumis.sh/usage/themes) — the theme list and custom themes
- [Caching parsers](https://docs.lumis.sh/recipes/cache-parsers-cli)

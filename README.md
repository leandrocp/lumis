# Lumis

[![Crates.io](https://img.shields.io/crates/v/lumis)](https://crates.io/crates/lumis)
[![docs.rs](https://img.shields.io/docsrs/lumis)](https://docs.rs/lumis)

Lumis is a syntax highlighter powered by Tree-sitter and Neovim themes. It provides beautiful and accurate syntax highlighting for over 70 programming languages with support for over 100 themes.

## Features

- 🎨 Over 100 themes including popular ones like:
  - Dracula, Catppuccin, Tokyo Night, Gruvbox
  - GitHub themes (light/dark)
  - Solarized variants
  - Nord, OneDark, and many more
- 🌳 Tree-sitter powered syntax highlighting with [specialized scoping](https://neovim.io/doc/user/treesitter.html#treesitter-highlight)
- 📝 Support for 70+ programming languages
- 🎯 Multiple output formats:
  - HTML with inline styles
  - HTML with linked stylesheets
  - HTML with multiple themes (light/dark or custom)
  - Terminal output with ANSI colors
- 🔍 Automatic language detection from file extensions
- 🚀 Zero configuration needed to get started
- 🖥️ Command-line interface included

## Installation

### As a Library

Add Lumis to your `Cargo.toml`:

```toml
[dependencies]
lumis = "0.2"
```

#### Selective Language Support

By default, Lumis includes support for all languages, which can result in longer compilation times. You can reduce compilation time and binary size by enabling only the languages you need:

```toml
[dependencies]
lumis = { version = "0.2", default-features = false, features = ["lang-rust", "lang-javascript", "lang-python"] }
```

Available language features:
- `lang-angular` - Angular templates
- `lang-asm` - Assembly
- `lang-astro` - Astro framework
- `lang-bash` - Bash/Shell scripts
- `lang-c` - C programming language
- `lang-caddy` - Caddy
- `lang-clojure` - Clojure
- `lang-cmake` - CMake build files
- `lang-comment` - Comment highlighting
- `lang-commonlisp` - Common Lisp
- `lang-cpp` - C++
- `lang-csharp` - C#
- `lang-css` - CSS stylesheets
- `lang-csv` - CSV files
- `lang-dart` - Dart
- `lang-diff` - Diff/patch files
- `lang-dockerfile` - Docker files
- `lang-eex` - Elixir EEx templates
- `lang-ejs` - EJS templates
- `lang-elixir` - Elixir
- `lang-elm` - Elm
- `lang-erb` - ERB templates
- `lang-erlang` - Erlang
- `lang-fish` - Fish
- `lang-fsharp` - F#
- `lang-gleam` - Gleam
- `lang-glimmer` - Glimmer/Handlebars
- `lang-go` - Go
- `lang-graphql` - GraphQL
- `lang-haskell` - Haskell
- `lang-hcl` - HCL/Terraform
- `lang-heex` - Phoenix HEEx templates
- `lang-html` - HTML
- `lang-iex` - Elixir IEx
- `lang-java` - Java
- `lang-javascript` - JavaScript
- `lang-json` - JSON
- `lang-kotlin` - Kotlin
- `lang-latex` - LaTeX
- `lang-liquid` - Liquid templates
- `lang-llvm` - LLVM IR
- `lang-lua` - Lua
- `lang-make` - Makefiles
- `lang-markdown` - Markdown
- `lang-markdown-inline` - Inline Markdown
- `lang-nix` - Nix
- `lang-objc` - Objective-C
- `lang-ocaml` - OCaml
- `lang-ocamlinterface` - OCaml Interface
- `lang-perl` - Perl
- `lang-php` - PHP
- `lang-plaintext` - Plain Text
- `lang-powershell` - PowerShell
- `lang-protobuf` - Protocol Buffers
- `lang-python` - Python
- `lang-r` - R
- `lang-regex` - Regular expressions
- `lang-ruby` - Ruby
- `lang-rust` - Rust
- `lang-scala` - Scala
- `lang-scss` - SCSS
- `lang-sql` - SQL
- `lang-surface` - Phoenix Surface
- `lang-svelte` - Svelte
- `lang-swift` - Swift
- `lang-toml` - TOML
- `lang-tsx` - TypeScript JSX
- `lang-typescript` - TypeScript
- `lang-typst` - Typst
- `lang-vim` - Vim script
- `lang-vue` - Vue.js
- `lang-xml` - XML
- `lang-yaml` - YAML
- `lang-zig` - Zig

Or use the convenience feature to enable all languages:

```toml
[dependencies]
lumis = { version = "0.2", features = ["all-languages"] }
```

### As a CLI Tool

Install the `lumis` command-line tool:

```sh
cargo install lumis
```

#### Faster CLI Installation with Selective Languages

For faster compilation, you can install the CLI with only the languages you need:

```sh
# Install with only specific languages
cargo install lumis --no-default-features --features "lang-rust,lang-python,lang-javascript"

# Install with web development languages
cargo install lumis --no-default-features --features "lang-html,lang-css,lang-javascript,lang-typescript,lang-json"

# Install with all languages (same as default)
cargo install lumis --features "all-languages"
```

This can significantly reduce compilation time, especially on slower machines or CI environments.

## Usage

### Library Usage

#### Basic Example

```rust
use lumis::{highlight, Options};

let code = r#"
    function greet(name) {
        console.log(`Hello ${name}!`);
    }
"#;

let html = highlight("javascript", code, Options::default());
```

#### Using a Specific Theme

```rust
use lumis::{highlight, Options, themes::Theme};

let code = "SELECT * FROM users WHERE active = true;";

// Parse theme from string
let theme: Theme = "dracula".parse().expect("Theme not found");
// Or: let theme = themes::get("dracula").expect("Theme not found");

let html = highlight(
    "sql",
    code,
    Options {
        theme,
        ..Options::default()
    }
);
```

#### Language Detection from File Path

```rust
use lumis::{highlight, Options};

let code = r#"
    defmodule MyApp do
      def hello, do: :world
    end
"#;
// Language will be automatically detected as Elixir from the .ex extension
let html = highlight("app.ex", code, Options::default());
```

#### Terminal Output with ANSI Colors

```rust
use lumis::{highlight, Options, FormatterOption};

let code = "puts 'Hello from Ruby!'";
let ansi = highlight(
    "ruby",
    code,
    Options {
        formatter: FormatterOption::Terminal,
        ..Options::default()
    }
);
```

#### HTML with Linked Stylesheets

```rust
use lumis::{highlight, Options, FormatterOption};

let code = "console.log('Hello!')";
let html = highlight(
    "javascript",
    code,
    Options {
        formatter: FormatterOption::HtmlLinked,
        ..Options::default()
    }
);
```

When using `FormatterOption::HtmlLinked`, include the corresponding CSS file for your chosen theme:

```html
<link rel="stylesheet" href="css/dracula.css" />
```

### Command-Line Usage

The `lumis` command-line tool provides several commands for syntax highlighting and code analysis:

#### List Available Languages

```sh
lumis list-languages
```

Lists all supported programming languages and their associated file patterns.

#### List Available Themes

```sh
lumis list-themes
```

Lists all available syntax highlighting themes.

#### Highlight a File

```sh
lumis highlight <path> [options]
```

Highlights the contents of a file with syntax highlighting.

Options:
- `-f, --formatter <formatter>`: Output format (default: terminal)
  - `terminal`: ANSI colored output for terminal
  - `html-inline`: HTML output with inline styles
  - `html-linked`: HTML output with linked stylesheet
- `-t, --theme <theme>`: Theme name (default: catppuccin_frappe)

Example:
```sh
lumis highlight src/main.rs --formatter html-inline --theme github_dark
```

#### Highlight Source Code

```sh
lumis highlight-source <source> [options]
```

Highlights a string of source code.

Options:
- `-l, --language <language>`: Programming language for the source code
- `-f, --formatter <formatter>`: Output format (default: terminal)
- `-t, --theme <theme>`: Theme name (default: catppuccin_frappe)

Example:
```sh
lumis highlight-source "println!(\"Hello World!\");" -l rust
```

#### Dump Tree-sitter AST

```sh
lumis dump-tree-sitter <path>
```

Dumps the Tree-sitter AST (Abstract Syntax Tree) for a given file. This is useful for debugging or understanding how Tree-sitter parses your code.

#### Generate Theme

```sh
lumis gen-theme --url <git-url> --colorscheme <name> [options]
```

Generates a theme JSON file from any Git repository containing a Neovim theme.

**Note**: Requires `nvim` to be installed and available in `$PATH`.

Required options:
- `--url <git-url>`: Git repository URL (e.g., https://github.com/catppuccin/nvim)
- `--colorscheme <name>`: Colorscheme name to activate (e.g., catppuccin-mocha)

Optional:
- `--setup <lua-code>`: Custom Lua setup code to run before activating the colorscheme
- `-o, --output <path>`: Output file path (prints to stdout if not specified)
- `--appearance <light|dark>`: Theme appearance (defaults to dark)

Examples:

```sh
# Basic usage - output to stdout
lumis gen-theme --url https://github.com/catppuccin/nvim --colorscheme catppuccin-mocha

# Save to file
lumis gen-theme \
  --url https://github.com/folke/tokyonight.nvim \
  --colorscheme tokyonight \
  -o tokyonight.json

# With custom setup code
lumis gen-theme \
  --url https://github.com/ellisonleao/gruvbox.nvim \
  --colorscheme gruvbox \
  --setup "require('gruvbox').setup({ contrast = 'hard' })" \
  -o gruvbox-hard.json

# Specify light appearance
lumis gen-theme \
  --url https://github.com/projekt0n/github-nvim-theme \
  --colorscheme github_light \
  --appearance light \
  -o github-light.json
```

See [themes/README.md](themes/README.md) for more details on theme generation.

## Supported Languages

Check the [documentation](https://docs.rs/lumis/latest/lumis/#languages-available) for a complete list of supported languages and file extensions.

## Available Themes

Lumis includes over 100 themes, such as:

- Dracula and Dracula Soft
- Catppuccin (Mocha, Macchiato, Frappe, Latte)
- GitHub themes (Light/Dark, High Contrast, Colorblind)
- Gruvbox (Light/Dark variants)
- Nord
- OneDark variants
- Rose Pine
- Solarized variants
- Tokyo Night variants
- And many more!

Check the [documentation](https://docs.rs/lumis/latest/lumis/#themes-available) for a complete list of supported themes.

## Migration from autumnus

If you were using the `autumnus` crate, simply update your dependencies:

```toml
# Before
[dependencies]
autumnus = "0.8"

# After
[dependencies]
lumis = "0.2"
```

And update your imports:

```rust
// Before
use autumnus::*;

// After
use lumis::*;
```

The API remains the same.

## Contributing

Contributions are welcome! Feel free to:

- Report bugs
- Suggest new features
- Add new themes
- Add support for new languages
- Improve documentation

## Acknowledgements

Lumis would not be possible without these projects:

- [inkjet](https://github.com/Colonial-Dev/inkjet)
- [difftastic](https://github.com/Wilfred/difftastic)
- [Learn X in Y minutes](https://github.com/adambard/learnxinyminutes-docs)

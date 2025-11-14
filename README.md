# Autumnus

[![Crates.io](https://img.shields.io/crates/v/autumnus)](https://crates.io/crates/autumnus)
[![docs.rs](https://img.shields.io/docsrs/autumnus)](https://docs.rs/autumnus)

Autumnus is a syntax highlighter powered by Tree-sitter and Neovim themes. It provides beautiful and accurate syntax highlighting for over 50 programming languages with support for over 100 themes.

## Features

- 🎨 Over 100 themes including popular ones like:
  - Dracula, Catppuccin, Tokyo Night, Gruvbox
  - GitHub themes (light/dark)
  - Solarized variants
  - Nord, OneDark, and many more
- 🌳 Tree-sitter powered syntax highlighting for accurate parsing
- 📝 Support for 50+ programming languages
- 🎯 Multiple output formats:
  - HTML with inline styles
  - HTML with linked stylesheets
  - Terminal output with ANSI colors
- 🔍 Automatic language detection from file extensions
- 🚀 Zero configuration needed to get started
- 🖥️ Command-line interface included

## Installation

### As a Library

Add Autumnus to your `Cargo.toml`:

```toml
[dependencies]
autumnus = "0.3"
```

#### Selective Language Support

By default, Autumnus includes support for all languages, which can result in longer compilation times. You can reduce compilation time and binary size by enabling only the languages you need:

```toml
[dependencies]
autumnus = { version = "0.3", default-features = false, features = ["lang-rust", "lang-javascript", "lang-python"] }
```

Available language features:
- `lang-angular` - Angular templates
- `lang-asm` - Assembly
- `lang-astro` - Astro framework
- `lang-bash` - Bash/Shell scripts
- `lang-c` - C programming language
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
- `lang-perl` - Perl
- `lang-php` - PHP
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
- `lang-vim` - Vim script
- `lang-vue` - Vue.js
- `lang-xml` - XML
- `lang-yaml` - YAML
- `lang-zig` - Zig

Or use the convenience feature to enable all languages:

```toml
[dependencies]
autumnus = { version = "0.3", features = ["all-languages"] }
```

### As a CLI Tool

Install the `autumn` command-line tool:

```sh
cargo install autumnus
```

#### Faster CLI Installation with Selective Languages

For faster compilation, you can install the CLI with only the languages you need:

```sh
# Install with only specific languages
cargo install autumnus --no-default-features --features "lang-rust,lang-python,lang-javascript"

# Install with web development languages
cargo install autumnus --no-default-features --features "lang-html,lang-css,lang-javascript,lang-typescript,lang-json"

# Install with all languages (same as default)
cargo install autumnus --features "all-languages"
```

This can significantly reduce compilation time, especially on slower machines or CI environments.

Note: While the package name is `autumnus`, the installed binary is named `autumn`. This means you use `cargo install autumnus` to install it, but run it as `autumn` in your terminal.

## Usage

### Library Usage

#### Basic Example

```rust
use autumnus::{highlight, Options};

let code = r#"
    function greet(name) {
        console.log(`Hello ${name}!`);
    }
"#;

let html = highlight("javascript", code, Options::default());
```

#### Using a Specific Theme

```rust
use autumnus::{highlight, Options, themes::Theme};

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
use autumnus::{highlight, Options};

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
use autumnus::{highlight, Options, FormatterOption};

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
use autumnus::{highlight, Options, FormatterOption};

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

The `autumn` command-line tool provides several commands for syntax highlighting and code analysis:

#### List Available Languages

```sh
autumn list-languages
```

Lists all supported programming languages and their associated file patterns.

#### List Available Themes

```sh
autumn list-themes
```

Lists all available syntax highlighting themes.

#### Highlight a File

```sh
autumn highlight <path> [options]
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
autumn highlight src/main.rs --formatter html-inline --theme github_dark
```

#### Highlight Source Code

```sh
autumn highlight-source <source> [options]
```

Highlights a string of source code.

Options:
- `-l, --language <language>`: Programming language for the source code
- `-f, --formatter <formatter>`: Output format (default: terminal)
- `-t, --theme <theme>`: Theme name (default: catppuccin_frappe)

Example:
```sh
autumn highlight-source "println!(\"Hello World!\");" -l rust
```

#### Dump Tree-sitter AST

```sh
autumn dump-tree-sitter <path>
```

Dumps the Tree-sitter AST (Abstract Syntax Tree) for a given file. This is useful for debugging or understanding how Tree-sitter parses your code.

#### Generate Theme

```sh
autumn gen-theme --url <git-url> --colorscheme <name> [options]
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
autumn gen-theme --url https://github.com/catppuccin/nvim --colorscheme catppuccin-mocha

# Save to file
autumn gen-theme \
  --url https://github.com/folke/tokyonight.nvim \
  --colorscheme tokyonight \
  -o tokyonight.json

# With custom setup code
autumn gen-theme \
  --url https://github.com/ellisonleao/gruvbox.nvim \
  --colorscheme gruvbox \
  --setup "require('gruvbox').setup({ contrast = 'hard' })" \
  -o gruvbox-hard.json

# Specify light appearance
autumn gen-theme \
  --url https://github.com/projekt0n/github-nvim-theme \
  --colorscheme github_light \
  --appearance light \
  -o github-light.json
```

See [themes/README.md](themes/README.md) for more details on theme generation.

## Supported Languages

Check the [documentation](https://docs.rs/autumnus/latest/autumnus/#languages-available) for a complete list of supported languages and file extensions.

## Available Themes

Autumnus includes over 100 themes, such as:

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

Check the [documentation](https://docs.rs/autumnus/latest/autumnus/#themes-available) for a complete list of supported themes.

## Contributing

Contributions are welcome! Feel free to:

- Report bugs
- Suggest new features
- Add new themes
- Add support for new languages
- Improve documentation

## Acknowledgements

Autumnus would not be possible without these projects:

- [inkjet](https://github.com/Colonial-Dev/inkjet)
- [difftastic](https://github.com/Wilfred/difftastic)
- [Learn X in Y minutes](https://github.com/adambard/learnxinyminutes-docs)

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
autumnus = "0.1"
```

### As a CLI Tool

Install the `autumn` command-line tool:

```sh
cargo install autumnus
```

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
use autumnus::{highlight, Options, themes};

let code = "SELECT * FROM users WHERE active = true;";
let html = highlight(
    "sql",
    code,
    Options {
        theme: themes::get("dracula").expect("Theme not found"),
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

## Supported Languages

Autumnus supports a wide range of programming languages, including but not limited to:

- Angular
- Astro
- Bash
- C/C++
- C#
- Clojure
- CSS/SCSS
- Dockerfile
- Elixir/EEx/HEEx
- Elm
- Erlang
- F#
- Go
- GraphQL
- Haskell
- HTML
- Java
- JavaScript/TypeScript
- JSON
- Kotlin
- LaTeX
- Lua
- Markdown
- OCaml
- PHP
- Python
- Ruby
- Rust
- SQL
- Swift
- TOML
- XML
- YAML
- And many more!

Check the [documentation](https://docs.rs/autumnus) for a complete list of supported languages and file extensions.

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

Visit the [documentation](https://docs.rs/autumnus) for a complete list of available themes.

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

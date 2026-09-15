# Lumis

<p align="center">
  Syntax Highlighter powered by Tree-sitter and Neovim themes.
</p>

<p align="center">
  <a href="https://lumis.sh">https://lumis.sh</a>
</p>

<p align="center">
  <a href="https://crates.io/crates/lumis"><img src="https://img.shields.io/crates/v/lumis" alt="Crates.io"></a>
  <a href="https://docs.rs/lumis"><img src="https://img.shields.io/docsrs/lumis" alt="docs.rs"></a>
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/license-MIT-blue" alt="License"></a>
</p>

## Features

- **110+ Tree-sitter languages** - Fast, accurate, and updated syntax parsing
- **250+ built-in Neovim themes** - Updated and curated themes from the Neovim community
- **Built-in formatters** - HTML (inline/linked), Terminal (ANSI), Multi-theme (light/dark), BBCode
- **Custom formatters** - Build your own output
- **Language auto-detection** - File extension, shebang, and emacs-mode support
- **Line highlighting** - Mark and style individual lines, with custom HTML wrappers
- **Streaming-friendly** - Handles incomplete code
- **Feature-gated languages** - Compile in every language, a bundle, or just the ones you name

## Installation

```toml
[dependencies]
lumis = "0.1"
```

All languages are on by default. To cut compile time and binary size, take only
what you need:

```toml
lumis = { version = "0.1", default-features = false, features = ["lang-rust", "lang-bundle-web"] }
```

Feature names are `lang-<name>` and `lang-bundle-<web|web-extra|system|backend|full>`;
the full list is in [Cargo.toml](https://github.com/leandrocp/lumis/blob/main/crates/lumis/Cargo.toml).

## Usage

```rust
use lumis::{highlight, HtmlInlineBuilder, languages::Language, themes};

let code = r#"fn main() { println!("Hello, world!"); }"#;
let theme = themes::get("dracula").unwrap();

let formatter = HtmlInlineBuilder::new()
    .language(Language::Rust)
    .theme(Some(theme))
    .build()
    .unwrap();

let html = highlight(code, formatter);
```

The language is optional — Lumis detects it from the source, a file extension,
or a shebang. `write_highlight()` streams to any `Write` instead of allocating a
`String`.

Formatters decide the output: `HtmlInline`, `HtmlLinked`, `HtmlMultiThemes`,
`Terminal`, or your own via the `Formatter` trait.

## Documentation

- [Highlighting](https://lumis.sh/docs/usage/highlight) and [Rust advanced usage](https://lumis.sh/docs/usage/rust)
- [Formatters](https://lumis.sh/docs/formatters) — every formatter and its options
- [Custom formatters](https://lumis.sh/docs/formatters/custom)
- [Annotations](https://lumis.sh/docs/formatters/annotations) — compose your own ranges into the event stream
- [Themes](https://lumis.sh/docs/themes) and [CSS theme files](https://lumis.sh/docs/themes/css-files)
- [Languages](https://lumis.sh/docs/reference/languages) — the full list and how detection works
- [Line highlighting](https://lumis.sh/docs/recipes/line-highlighting)
- [Recipes](https://lumis.sh/docs/recipes)

API reference: [docs.rs/lumis](https://docs.rs/lumis).

The CLI is a separate crate: `cargo install lumis-cli`.

## Acknowledgements

* [Inkjet](https://crates.io/crates/inkjet) for the implementation up to v0.2 and for the inspiration

## License

MIT

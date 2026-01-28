<h1 align="center">Lumis</h1>

<p align="center">
  <strong>Syntax highlighting powered by Tree-sitter and Neovim themes</strong>
</p>

<p align="center">
  <a href="https://lumis.sh">lumis.sh</a>
</p>

<p align="center">
  <a href="https://crates.io/crates/lumis"><img src="https://img.shields.io/crates/v/lumis" alt="Crates.io"></a>
  <a href=""><img src="https://img.shields.io/hexpm/v/lumis" alt="Hex.pm"></a>
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/license-MIT-blue" alt="License"></a>
</p>

---

**70+ languages. 120+ themes. One API.**

Lumis gives you accurate, beautiful syntax highlighting using the same Tree-sitter parsers and Neovim colorschemes developers love. Use it from the CLI, Rust, or Elixir.

<table>
<tr>
<td><img src="assets/ruby.png" alt="Ruby with Catppuccin Frappe theme"></td>
<td><img src="assets/sql.png" alt="SQL with GitHub Light theme"></td>
</tr>
</table>

## Quick Start

### CLI

```sh
cargo install lumis

lumis highlight src/index.js --theme dracula
```

### [Rust](https://crates.io/crates/lumis)

```rust
use lumis::{highlight, HtmlInlineBuilder, languages::Language, themes};

let code = "print('Hello')";
let theme = themes::get("dracula").unwrap();

let formatter = HtmlInlineBuilder::new()
    .lang(Language::Python)
    .theme(Some(theme))
    .build()
    .unwrap();

let html = highlight(code, formatter);
```

### [Elixir](https://hex.pm/packages/lumis)

```elixir
Lumis.highlight!("setTimeout(fun, 5000)", language: "js", formatter: {:html_inline, theme: "dracula"})
```

## Features

- **Tree-sitter parsing** - Fast and accurate syntax parsing for 70+ languages
- **120+ Neovim themes** - Updated and curated 100+ themes from the Neovim community
- **Multiple outputs** - HTML (inline/linked), Terminal (ANSI), multi-theme (light/dark), and custom formatters
- **Language auto-detection** - File extension and shebang support
- **Streaming-friendly** - Handles incomplete code gracefully
- **Zero config** - Works out of the box

## Documentation

| Platform | Install | Docs |
|----------|---------|------|
| **CLI** | `cargo install lumis` | [Usage](#cli-usage) |
| **Rust** | `cargo add lumis` | [crates/lumis](crates/lumis/README.md) &bull; [docs.rs](https://docs.rs/lumis) |
| **Elixir** | `{:lumis, "~> 0.1"}` | [packages/elixir](packages/elixir/lumis/README.md) &bull; [hexdocs](https://hexdocs.pm/lumis) |

## Contributing

Contributions welcome! Feel free to open issues or PRs for bugs, features, new themes, or language support.

## License

MIT

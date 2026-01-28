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

- 70+ languages with Tree-sitter parsing
- 120+ Neovim themes
- HTML output with inline styles or CSS classes
- Multi-theme support for light/dark mode
- Terminal output with ANSI colors
- Language auto-detection via file extensions and shebangs
- Line highlighting with custom styling
- Custom HTML wrappers for code blocks
- Streaming-friendly - handles incomplete code gracefully
- Custom formatters via the `Formatter` trait

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
lumis = "0.1"
```

Or install the CLI:

```sh
cargo install lumis
```

## Quick Start

```rust
use lumis::{highlight, HtmlInlineBuilder, languages::Language, themes};

let code = r#"fn main() { println!("Hello, world!"); }"#;
let theme = themes::get("dracula").unwrap();

let formatter = HtmlInlineBuilder::new()
    .lang(Language::Rust)
    .theme(Some(theme))
    .build()
    .unwrap();

let html = highlight(code, formatter);
```

For streaming to files or other writers, use `write_highlight()`:

```rust
use lumis::{write_highlight, HtmlInlineBuilder, languages::Language};
use std::fs::File;

let code = "fn main() { }";
let formatter = HtmlInlineBuilder::new()
    .lang(Language::Rust)
    .build()
    .unwrap();

let mut file = File::create("output.html").unwrap();
write_highlight(&mut file, code, formatter).unwrap();
```

## Formatters

Lumis provides four built-in formatters:

| Formatter | Output | Use When |
|-----------|--------|----------|
| `HtmlInlineBuilder` | HTML with inline styles | Standalone HTML, emails, no external CSS |
| `HtmlMultiThemesBuilder` | HTML with CSS variables | Light/dark mode, theme switching |
| `HtmlLinkedBuilder` | HTML with CSS classes | Multiple code blocks, custom styling |
| `TerminalBuilder` | ANSI escape codes | CLI tools, terminal output |

### HTML Inline

Generates HTML with inline styles for each token:

```rust
use lumis::{highlight, HtmlInlineBuilder, languages::Language, themes};

let code = "puts 'Hello from Ruby!'";
let theme = themes::get("catppuccin_mocha").unwrap();

let formatter = HtmlInlineBuilder::new()
    .lang(Language::Ruby)
    .theme(Some(theme))
    .pre_class(Some("code-block".to_string()))  // Custom CSS class for <pre>
    .italic(true)                                // Enable italic styles
    .include_highlights(true)                    // Add data-highlight attributes
    .build()
    .unwrap();

let html = highlight(code, formatter);
```

### HTML Linked

Generates HTML with CSS classes for styling (requires external CSS):

```rust
use lumis::{highlight, HtmlLinkedBuilder, languages::Language};

let code = "<div>Hello World</div>";

let formatter = HtmlLinkedBuilder::new()
    .lang(Language::HTML)
    .pre_class(Some("my-code".to_string()))
    .build()
    .unwrap();

let html = highlight(code, formatter);
```

CSS theme files are available in the `css/` directory of the crate.

### HTML Multi-Themes

Generates HTML with CSS custom properties for multiple themes, enabling light/dark mode support:

```rust
use lumis::{highlight, HtmlMultiThemesBuilder, languages::Language, themes};
use std::collections::HashMap;

let code = "const x = 42;";

let mut themes_map = HashMap::new();
themes_map.insert("light".to_string(), themes::get("github_light").unwrap());
themes_map.insert("dark".to_string(), themes::get("github_dark").unwrap());

let formatter = HtmlMultiThemesBuilder::new()
    .lang(Language::JavaScript)
    .themes(themes_map)
    .default_theme("light")  // or "light-dark()" for CSS function
    .build()
    .unwrap();

let html = highlight(code, formatter);
```

Use CSS media queries for automatic theme switching:

```css
@media (prefers-color-scheme: dark) {
  .lumis,
  .lumis span {
    color: var(--lumis-dark) !important;
    background-color: var(--lumis-dark-bg) !important;
  }
}
```

### Terminal

Generates ANSI escape codes for terminal output:

```rust
use lumis::{highlight, TerminalBuilder, languages::Language, themes};

let code = "puts 'Hello from Ruby!'";
let theme = themes::get("dracula").unwrap();

let formatter = TerminalBuilder::new()
    .lang(Language::Ruby)
    .theme(Some(theme))
    .build()
    .unwrap();

let ansi = highlight(code, formatter);
println!("{}", ansi);
```

## Line Highlighting

Highlight specific lines with custom styling:

```rust
use lumis::{highlight, HtmlInlineBuilder, languages::Language, themes};
use lumis::formatter::html_inline::{HighlightLines, HighlightLinesStyle};

let code = "line 1\nline 2\nline 3\nline 4";
let theme = themes::get("catppuccin_mocha").unwrap();

let highlight_lines = HighlightLines {
    lines: vec![1..=1, 3..=4],  // Highlight lines 1, 3, and 4
    style: Some(HighlightLinesStyle::Theme),
    class: None,
};

let formatter = HtmlInlineBuilder::new()
    .lang(Language::PlainText)
    .theme(Some(theme))
    .highlight_lines(Some(highlight_lines))
    .build()
    .unwrap();

let html = highlight(code, formatter);
```

## Custom HTML Wrappers

Wrap the formatted output with custom HTML elements:

```rust
use lumis::{highlight, HtmlInlineBuilder, languages::Language, formatter::HtmlElement};

let code = "fn main() { }";

let formatter = HtmlInlineBuilder::new()
    .lang(Language::Rust)
    .header(Some(HtmlElement {
        open_tag: "<div class=\"code-wrapper\">".to_string(),
        close_tag: "</div>".to_string(),
    }))
    .build()
    .unwrap();

let html = highlight(code, formatter);
// Output: <div class="code-wrapper"><pre class="lumis">...</pre></div>
```

## Themes

120+ themes from popular Neovim colorschemes:

```rust
use lumis::themes;

// Get a theme by name
let theme = themes::get("dracula").unwrap();

// Parse from string
let theme: themes::Theme = "catppuccin_mocha".parse().unwrap();

// List all available themes
for theme in themes::available_themes() {
    println!("{} ({})", theme.name, theme.appearance);
}

// Filter by appearance
use lumis::themes::Appearance;

let dark_themes: Vec<_> = themes::available_themes()
    .filter(|t| t.appearance == Appearance::Dark)
    .collect();
```

### Custom Themes

Load themes from JSON files or strings:

```rust
use lumis::themes;

// From file
let theme = themes::from_file("my_theme.json").unwrap();

// From JSON string
let json = r#"{
    "name": "my_theme",
    "appearance": "dark",
    "revision": "v1.0.0",
    "highlights": {
        "keyword": { "fg": "#ff79c6", "bold": true },
        "string": { "fg": "#f1fa8c" },
        "comment": { "fg": "#6272a4", "italic": true }
    }
}"#;
let theme = themes::from_json(json).unwrap();
```

## Custom Formatters

Implement the `Formatter` trait to create custom output formats:

```rust
use lumis::{
    formatter::Formatter,
    highlight::highlight_iter,
    languages::Language,
    themes,
};
use std::io::{self, Write};

struct CsvFormatter {
    language: Language,
    theme: Option<themes::Theme>,
}

impl Formatter for CsvFormatter {
    fn format(&self, source: &str, output: &mut dyn Write) -> io::Result<()> {
        writeln!(output, "text,start,end,scope,fg")?;

        highlight_iter(source, self.language, self.theme.clone(), |text, range, scope, style| {
            let fg = style.fg.as_deref().unwrap_or("none");
            let escaped = text.replace('"', "\"\"");
            writeln!(output, "\"{}\",{},{},{},{}", escaped, range.start, range.end, scope, fg)
        })
        .map_err(io::Error::other)
    }
}
```

## CLI Usage

```sh
# Highlight a file
lumis highlight src/main.rs --theme dracula

# Highlight with a specific language
lumis highlight code.txt --language rust --theme github_dark

# Output to terminal (default)
lumis highlight src/main.rs --theme catppuccin_mocha

# List available themes
lumis themes

# List available languages
lumis languages
```

## Language Feature Flags

By default, Lumis includes all languages. To reduce compile time and binary size, enable only the languages you need:

```toml
[dependencies]
lumis = { version = "0.1", default-features = false, features = ["lang-rust", "lang-javascript"] }
```

Available features:
- `all-languages` - Enable all languages (default)
- `lang-rust`, `lang-javascript`, `lang-typescript`, `lang-python`, etc.

See the full list of language features in [Cargo.toml](https://github.com/leandrocp/lumis/blob/main/crates/lumis/Cargo.toml).

## Supported Languages

| Language | File Extensions |
|----------|-----------------|
| Angular | *.angular, component.html |
| Assembly | *.s, *.asm |
| Astro | *.astro |
| Bash | *.bash, *.sh, *.zsh, and more |
| C | *.c |
| C++ | *.cpp, *.cc, *.h, *.hpp |
| C# | *.cs |
| CSS | *.css |
| Clojure | *.clj, *.cljs, *.cljc |
| Dart | *.dart |
| Dockerfile | Dockerfile, *.dockerfile |
| Elixir | *.ex, *.exs |
| Erlang | *.erl, *.hrl |
| F# | *.fs, *.fsx |
| Gleam | *.gleam |
| Go | *.go |
| GraphQL | *.graphql |
| HTML | *.html, *.htm |
| Haskell | *.hs |
| HCL | *.hcl, *.tf |
| Java | *.java |
| JavaScript | *.js, *.mjs, *.cjs |
| JSON | *.json |
| Kotlin | *.kt |
| LaTeX | *.tex |
| Lua | *.lua |
| Markdown | *.md |
| Nix | *.nix |
| OCaml | *.ml, *.mli |
| PHP | *.php |
| Python | *.py |
| Ruby | *.rb |
| Rust | *.rs |
| SQL | *.sql |
| Scala | *.scala |
| Swift | *.swift |
| TOML | *.toml |
| TypeScript | *.ts |
| TSX | *.tsx |
| Vue | *.vue |
| YAML | *.yaml, *.yml |
| Zig | *.zig |
| ...and more | See docs for full list |

## Available Themes

Themes are sourced from popular Neovim colorschemes:

- **Catppuccin**: catppuccin_frappe, catppuccin_latte, catppuccin_macchiato, catppuccin_mocha
- **Dracula**: dracula, dracula_soft
- **GitHub**: github_dark, github_light, github_dark_dimmed, and more
- **Gruvbox**: gruvbox_dark, gruvbox_light, and variants
- **Tokyo Night**: tokyonight_day, tokyonight_moon, tokyonight_night, tokyonight_storm
- **One Dark**: onedark, onedark_cool, onedark_darker, onelight
- **Rose Pine**: rosepine_dark, rosepine_dawn, rosepine_moon
- **Nord**: nord, nordic, nordfox
- **Material**: material_darker, material_oceanic, material_palenight
- **Kanagawa**: kanagawa_wave, kanagawa_dragon, kanagawa_lotus
- ...and 100+ more

Use `themes::available_themes()` or the CLI `lumis themes` for the complete list.

## Acknowledgements

- [Inkjet](https://crates.io/crates/inkjet) for the Rust implementation in early versions
- The Neovim community for the beautiful colorschemes

## License

MIT

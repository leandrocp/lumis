# Lumis

<!-- MDOC -->

<p align="center">
  Syntax highlighter powered by Tree-sitter and Neovim themes.
</p>

<p align="center">
  <a href="https://lumis.sh">https://lumis.sh</a>
</p>

<div align="center">
  <a href="https://hex.pm/packages/lumis">
    <img alt="Hex Version" src="https://img.shields.io/hexpm/v/lumis">
  </a>

  <a href="https://hexdocs.pm/lumis">
    <img alt="Hex Docs" src="http://img.shields.io/badge/hex.pm-docs-green.svg?style=flat">
  </a>

  <a href="https://opensource.org/licenses/MIT">
    <img alt="MIT" src="https://img.shields.io/hexpm/l/lumis">
  </a>
</div>

## Features

- 🌳 70+ languages with tree-sitter parsing
- 🎨 120+ Neovim themes
- 📝 HTML output with inline styles or CSS classes
- 🖥️ Terminal output with ANSI colors
- 🔍 Language auto-detection
- 🎯 Customizable formatting options
- ✨ Line highlighting with custom styling
- 🎁 Custom HTML wrappers for code blocks

## Installation

```elixir
def deps do
  [
    {:lumis, "~> 0.1"}
  ]
end
```

## Usage

### Basic Usage (HTML Inline)

```elixir
iex> Lumis.highlight!("Atom.to_string(:elixir)", language: "elixir")
~s|<pre class="athl" style="color: #abb2bf; background-color: #282c34;"><code class="language-elixir" translate="no" tabindex="0"><div class="line" data-line="1"><span style="color: #e5c07b;">Atom</span><span style="color: #56b6c2;">.</span><span style="color: #61afef;">to_string</span><span style="color: #c678dd;">(</span><span style="color: #e06c75;">:elixir</span><span style="color: #c678dd;">)</span>
</span></code></pre>|
```

See the HTML Linked and Terminal formatters below for more options.

### Language Auto-detection

```elixir
iex> Lumis.highlight!("#!/usr/bin/env bash\nID=1")
~s|<pre class="athl" style="color: #abb2bf; background-color: #282c34;"><code class="language-bash" translate="no" tabindex="0"><div class="line" data-line="1"><span style="color: #c678dd;">#!/usr/bin/env bash</span>
</div><div class="line" data-line="2"><span style="color: #d19a66;">ID</span><span style="color: #56b6c2;">=</span><span style="color: #d19a66;">1</span>
</span></code></pre>|
```

### Themes

Themes are sourced from popular Neovim colorschemes.

Use `Lumis.available_themes/0` to list all available themes. You can specify a theme by name in the formatter options, or use `Lumis.Theme.get/1` to get a specific theme struct if you need to inspect or manipulate its styles.

```elixir
# Using theme name in formatter options
iex> Lumis.highlight!("setTimeout(fun, 5000);", language: "js", formatter: {:html_inline, theme: "github_light"})
~s|<pre class="athl" style="color: #1f2328; background-color: #ffffff;"><code class="language-javascript" translate="no" tabindex="0"><div class="line" data-line="1"><span style="color: #6639ba;">setTimeout</span><span style="color: #1f2328;">(</span><span style="color: #1f2328;">fun</span><span style="color: #1f2328;">,</span> <span style="color: #0550ae;">5000</span><span style="color: #1f2328;">)</span><span style="color: #1f2328;">;</span>
</span></code></pre>|

# Using theme struct
iex> theme = Lumis.Theme.get("github_light")
iex> Lumis.highlight!("setTimeout(fun, 5000);", language: "js", formatter: {:html_inline, theme: theme})
```

#### Bring Your Own Theme

You can also load custom themes from JSON files or strings:

```elixir
# Load from JSON file
{:ok, theme} = Lumis.Theme.from_file("/path/to/your/theme.json")
Lumis.highlight!("your code", theme: theme)

# Load from JSON string
theme_json = ~s({"name": "my_theme", "appearance": "dark", "highlights": {"comment": {"fg": "#808080"}}})
{:ok, theme} = Lumis.Theme.from_json(theme_json)
Lumis.highlight!("your code", theme: theme)
```

## Incomplete or Malformed code

It's also capable of handling incomplete or malformed code, useful for streaming like in a ChatGPT interface:

```elixir
iex> Lumis.highlight!("const header = document.getEl", language: "js")
~s|<pre class="athl" style="color: #abb2bf; background-color: #282c34;"><code class="language-javascript" translate="no" tabindex="0"><div class="line" data-line="1"><span style="color: #c678dd;">const</span> <span style="color: #abb2bf;">header</span> <span style="color: #abb2bf;">=</span> <span style="color: #e86671;">document</span><span style="color: #848b98;">.</span><span style="color: #56b6c2;">getEl</span>
</span></code></pre>|
```

## Formatters

Lumis supports four output formatters:

All HTML formatters wrap each line in a `<div class="line">` element with a `data-line` attribute containing the line number, making it easy to add line numbers or implement line-based features in your application.

See the [Livebook examples](https://github.com/leandrocp/lumis/tree/main/examples) and [t:formatter/0](https://hexdocs.pm/lumis/Lumis.html#t:formatter/0) for more.

### HTML Inline (Default)

Generates HTML with inline styles for each token:

```elixir
iex> Lumis.highlight!("Atom.to_string(:elixir)", language: "elixir", formatter: :html_inline)
# or with options
iex> Lumis.highlight!("Atom.to_string(:elixir)", language: "elixir", formatter: {:html_inline, pre_class: "my-code", italic: true, include_highlights: true})
```

Options:
- `:pre_class` - CSS class for the `<pre>` tag
- `:italic` - enable italic styles
- `:include_highlights` - include highlight scope names in `data-highlight` attributes
- `:highlight_lines` - highlight specific lines with custom styling
- `:header` - wrap the highlighted code with custom HTML elements

### HTML Linked

Generates HTML with CSS classes for styling:

```elixir
iex> Lumis.highlight!("Atom.to_string(:elixir)", language: "elixir", formatter: :html_linked)
# or with options
iex> Lumis.highlight!("Atom.to_string(:elixir)", language: "elixir", formatter: {:html_linked, pre_class: "my-code"})
```

Options:
- `:pre_class` - CSS class for the `<pre>` tag
- `:highlight_lines` - highlight specific lines with custom CSS class
- `:header` - wrap the highlighted code with custom HTML elements

To use linked styles, you need to include one of the [available CSS themes](https://github.com/leandrocp/lumis/tree/main/priv/static/css) in your app.

For Phoenix apps, add this to your `endpoint.ex`:

```elixir
plug Plug.Static,
  at: "/themes",
  from: {:lumis, "priv/static/css/"},
  only: ["dracula.css"] # choose any theme you want
```

Then add the stylesheet to your template:

```html
<link phx-track-static rel="stylesheet" href={~p"/themes/dracula.css"} />
```

### HTML Multi-Themes

Generates HTML with CSS custom properties (variables) for multiple themes, enabling light/dark mode support. Inspired by [Shiki Dual Themes](https://shiki.style/guide/dual-themes).

```elixir
# Basic dual theme with CSS variables
iex> Lumis.highlight!("Atom.to_string(:elixir)",
  language: "elixir",
  formatter: {:html_multi_themes,
    themes: [light: "github_light", dark: "github_dark"]
  }
)

# With light-dark() function for automatic theme switching
iex> Lumis.highlight!("Atom.to_string(:elixir)",
  language: "elixir",
  formatter: {:html_multi_themes,
    themes: [light: "github_light", dark: "github_dark"],
    default_theme: "light-dark()"
  }
)
```

The generated HTML includes CSS custom properties like `--athl-light`, `--athl-dark`, `--athl-{theme}-bg`, and font styling variables (`-font-style`, `-font-weight`, `-text-decoration`) that can be used with CSS media queries or JavaScript for theme switching:

```css
/* Automatic light/dark mode based on system preference */
@media (prefers-color-scheme: dark) {
  .athl,
  .athl span {
    color: var(--athl-dark) !important;
    background-color: var(--athl-dark-bg) !important;
    font-style: var(--athl-dark-font-style) !important;
    font-weight: var(--athl-dark-font-weight) !important;
    text-decoration: var(--athl-dark-text-decoration) !important;
  }
}

/* Manual control with class-based switching */
html.dark .athl,
html.dark .athl span {
  color: var(--athl-dark) !important;
  background-color: var(--athl-dark-bg) !important;
  font-style: var(--athl-dark-font-style) !important;
  font-weight: var(--athl-dark-font-weight) !important;
  text-decoration: var(--athl-dark-text-decoration) !important;
}
```

Options:
- `:themes` (required) - keyword list mapping theme identifiers to theme names, e.g., `[light: "github_light", dark: "github_dark"]`
- `:default_theme` - controls inline color rendering: theme identifier for inline colors, `"light-dark()"` for CSS function, or `nil` for CSS variables only
- `:css_variable_prefix` - custom CSS variable prefix (default: `"--athl"`)
- `:pre_class` - CSS class for the `<pre>` tag
- `:italic` - enable italic styles
- `:include_highlights` - include highlight scope names in `data-highlight` attributes
- `:highlight_lines` - highlight specific lines with custom styling
- `:header` - wrap the highlighted code with custom HTML elements

### Terminal

Generates ANSI escape codes for terminal output:

```elixir
iex> Lumis.highlight!("Atom.to_string(:elixir)", language: "elixir", formatter: :terminal)
# or with options
iex> Lumis.highlight!("Atom.to_string(:elixir)", language: "elixir", formatter: {:terminal, theme: "github_light"})
```

Options:
- `:theme` - theme to apply styles

## Samples

Visit https://lumis.sh to check out some examples.

## Acknowledgements

* [Makeup](https://hex.pm/packages/makeup) for setting up the baseline and for the inspiration
* [Inkjet](https://crates.io/crates/inkjet) for the Rust implementation up to v0.2 and for the inspiration

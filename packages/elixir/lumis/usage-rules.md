# Lumis - Usage Rules for AI Agents

Lumis is a syntax highlighter for Elixir that uses Tree-sitter parsers and Neovim themes. It provides fast, accurate syntax highlighting with support for 70+ languages and 120+ themes, outputting to HTML (inline or linked) or terminal (ANSI codes).

## Core Concepts

### What Lumis Does
- Highlights source code using Tree-sitter parsers
- Supports 70+ programming languages with auto-detection
- Provides 120+ Neovim themes (light and dark)
- Handles incomplete/malformed code gracefully (useful for streaming scenarios)
- Outputs HTML with inline styles, HTML with CSS classes, or ANSI terminal codes

### What Lumis Does NOT Do
- Does not parse or execute code
- Does not validate code syntax (it highlights even invalid code)
- Does not provide language-specific transformations
- Does not format or prettify code

## Basic Usage

### Simple Highlighting

Always use the 2-arity form with options:

```elixir
# Good - with language specified
Lumis.highlight!("defmodule MyApp do", language: "elixir")

# Good - language auto-detection
Lumis.highlight!("#!/usr/bin/env bash\necho 'hello'")

# Avoid - deprecated 3-arity form
Lumis.highlight!("elixir", "defmodule MyApp do", [])
```

### Return Values

- `highlight/2` returns `{:ok, html_string}` or `{:error, reason}`
- `highlight!/2` returns `html_string` or raises an exception

```elixir
# Pattern match on success/error
case Lumis.highlight(source, language: "elixir") do
  {:ok, html} -> html
  {:error, error} -> handle_error(error)
end

# Or use the bang version when you expect success
html = Lumis.highlight!(source, language: "elixir")
```

## Language Specification

### Specifying Languages

You can specify languages in multiple ways:

```elixir
# By language name
Lumis.highlight!(code, language: "elixir")
Lumis.highlight!(code, language: "javascript")
Lumis.highlight!(code, language: "rust")

# By file extension
Lumis.highlight!(code, language: ".ex")
Lumis.highlight!(code, language: ".js")

# By filename
Lumis.highlight!(code, language: "app.ex")
Lumis.highlight!(code, language: "lib/my_module.ex")

# Auto-detection (omit language option)
Lumis.highlight!(code)
```

### Discovering Available Languages

```elixir
# Get all available languages
languages = Lumis.available_languages()
# Returns: %{"elixir" => {"Elixir", ["*.ex", "*.exs"]}, ...}

# Check if a language is supported
Map.has_key?(Lumis.available_languages(), "elixir")
```

## Formatters

Lumis supports four formatters. The formatter option controls the output format.

### HTML Inline (Default)

Generates HTML with inline styles. Best for email, isolated components, or when you don't want external CSS.

```elixir
# Using default formatter
Lumis.highlight!(code, language: "elixir")

# Explicitly specify with options
Lumis.highlight!(code,
  language: "elixir",
  formatter: {:html_inline, [
    theme: "github_light",
    pre_class: "my-code",
    italic: true,
    include_highlights: false
  ]}
)
```

Available options for `:html_inline`:
- `:theme` - Theme name (string) or `Lumis.Theme` struct
- `:pre_class` - CSS class to add to the `<pre>` tag
- `:italic` - Enable italic styles (default: `false`)
- `:include_highlights` - Add `data-highlight` attributes for debugging (default: `false`)
- `:highlight_lines` - Highlight specific lines (see Line Highlighting section)
- `:header` - Wrap with custom HTML tags (see Custom Wrappers section)

### HTML Linked

Generates HTML with CSS classes. Requires linking a CSS file from `priv/static/css/`.

```elixir
Lumis.highlight!(code,
  language: "elixir",
  formatter: {:html_linked, [
    pre_class: "my-code"
  ]}
)
```

**Important**: You must include the CSS file in your application:

For Phoenix apps, add to `endpoint.ex`:
```elixir
plug Plug.Static,
  at: "/themes",
  from: {:lumis, "priv/static/css/"},
  only: ["onedark.css"]  # or any other theme
```

Then in your template:
```heex
<link rel="stylesheet" href={~p"/themes/onedark.css"} />
```

Available options for `:html_linked`:
- `:pre_class` - CSS class to add to the `<pre>` tag
- `:highlight_lines` - Highlight specific lines with CSS class
- `:header` - Wrap with custom HTML tags

### Terminal

Generates ANSI escape codes for terminal output.

```elixir
Lumis.highlight!(code,
  language: "elixir",
  formatter: :terminal
)

# With theme
Lumis.highlight!(code,
  language: "elixir",
  formatter: {:terminal, theme: "github_light"}
)
```

Available options for `:terminal`:
- `:theme` - Theme name (string) or `Lumis.Theme` struct

### HTML Multi-Themes

Generates HTML with CSS custom properties (variables) for multiple themes, enabling light/dark mode support. Inspired by [Shiki Dual Themes](https://shiki.style/guide/dual-themes).

```elixir
# Basic dual theme with CSS variables
Lumis.highlight!(code,
  language: "elixir",
  formatter: {:html_multi_themes,
    themes: [light: "github_light", dark: "github_dark"]
  }
)

# With light-dark() function for automatic theme switching
Lumis.highlight!(code,
  language: "elixir",
  formatter: {:html_multi_themes,
    themes: [light: "github_light", dark: "github_dark"],
    default_theme: "light-dark()"
  }
)

# With inline colors for default theme
Lumis.highlight!(code,
  language: "elixir",
  formatter: {:html_multi_themes,
    themes: [light: "github_light", dark: "github_dark"],
    default_theme: "light"
  }
)

# Multiple themes with custom prefix
Lumis.highlight!(code,
  language: "elixir",
  formatter: {:html_multi_themes,
    themes: [light: "github_light", dark: "github_dark", dim: "catppuccin_frappe"],
    css_variable_prefix: "--code"
  }
)
```

**How it works:**
- Generates CSS custom properties like `--athl-light-fg`, `--athl-dark-fg`, etc.
- Theme identifiers (from the keyword list keys) become CSS class names
- Use CSS media queries or JavaScript to switch between themes
- The `default_theme` option controls inline color rendering:
  - `nil` (default): Only CSS variables, no inline colors
  - A theme identifier (e.g., `"light"`): Renders inline colors for that theme plus CSS variables for all themes
  - `"light-dark()"`: Uses CSS light-dark() function for automatic theme switching

**CSS Integration Examples:**

```css
/* Automatic light/dark mode based on system preference */
@media (prefers-color-scheme: light) {
  .athl-themes {
    color: var(--athl-light);
    background-color: var(--athl-light-bg);
  }
}

@media (prefers-color-scheme: dark) {
  .athl-themes {
    color: var(--athl-dark);
    background-color: var(--athl-dark-bg);
  }
}

/* Manual control with data attributes */
[data-theme="light"] .athl-themes {
  color: var(--athl-light);
  background-color: var(--athl-light-bg);
}

[data-theme="dark"] .athl-themes {
  color: var(--athl-dark);
  background-color: var(--athl-dark-bg);
}
```

Available options for `:html_multi_themes`:
- `:themes` (required) - Keyword list mapping theme identifiers to theme names or structs, e.g., `[light: "github_light", dark: "github_dark"]`
- `:default_theme` - Controls inline color rendering: theme identifier, `"light-dark()"`, or `nil` (default: `nil`)
- `:css_variable_prefix` - Custom CSS variable prefix (default: `"--athl"`)
- `:pre_class` - CSS class to add to the `<pre>` tag
- `:italic` - Enable italic styles (default: `false`)
- `:include_highlights` - Add `data-highlight` attributes for debugging (default: `false`)
- `:highlight_lines` - Highlight specific lines (same options as `:html_inline`)
- `:header` - Wrap with custom HTML tags (same options as other formatters)

## Themes

### Using Themes

```elixir
# List all available themes
themes = Lumis.available_themes()
# Returns: ["onedark", "github_light", "dracula", ...]

# Use a theme by name
Lumis.highlight!(code,
  language: "elixir",
  formatter: {:html_inline, theme: "dracula"}
)

# Get a theme struct
theme = Lumis.Theme.get("github_light")
Lumis.highlight!(code, language: "elixir", formatter: {:html_inline, theme: theme})
```

### Custom Themes

You can load custom themes from JSON:

```elixir
# From a JSON file
{:ok, theme} = Lumis.Theme.from_file("/path/to/theme.json")
Lumis.highlight!(code, formatter: {:html_inline, theme: theme})

# From a JSON string
json = ~s({"name": "my_theme", "appearance": "dark", "highlights": {...}})
{:ok, theme} = Lumis.Theme.from_json(json)
Lumis.highlight!(code, formatter: {:html_inline, theme: theme})
```

## Advanced Features

### Line Highlighting

Highlight specific lines with custom styling or CSS classes.

#### HTML Inline Line Highlighting

```elixir
# Use theme's highlighted style (default)
Lumis.highlight!(code,
  language: "elixir",
  formatter: {:html_inline,
    highlight_lines: %{lines: [2, 3, 4]}
  }
)

# Explicit theme style
Lumis.highlight!(code,
  language: "elixir",
  formatter: {:html_inline,
    highlight_lines: %{lines: [1, 5..10], style: :theme}
  }
)

# Custom inline style
Lumis.highlight!(code,
  language: "elixir",
  formatter: {:html_inline,
    highlight_lines: %{
      lines: [2..4, 7],
      style: "background-color: #fff3cd; border-left: 3px solid #ffc107;"
    }
  }
)

# CSS class only (no inline style)
Lumis.highlight!(code,
  language: "elixir",
  formatter: {:html_inline,
    highlight_lines: %{
      lines: [1, 2, 3],
      style: nil,
      class: "highlighted-line"
    }
  }
)
```

The `:lines` option accepts:
- Single integers: `[1, 3, 5]`
- Ranges: `[2..10]`
- Mix of both: `[1, 3..5, 8, 10..15]`

#### HTML Linked Line Highlighting

```elixir
# Use default "highlighted" class from theme CSS
Lumis.highlight!(code,
  language: "elixir",
  formatter: {:html_linked,
    highlight_lines: %{lines: [2..4, 6]}
  }
)

# Use custom CSS class
Lumis.highlight!(code,
  language: "elixir",
  formatter: {:html_linked,
    highlight_lines: %{lines: [1, 2, 3], class: "error-line"}
  }
)
```

### Custom HTML Wrappers

Wrap the highlighted code with custom HTML elements:

```elixir
header = %{
  open_tag: "<figure><figcaption>app.ex</figcaption>",
  close_tag: "</figure>"
}

Lumis.highlight!(code,
  language: "elixir",
  formatter: {:html_inline, header: header}
)

# Output:
# <figure>
#   <figcaption>app.ex</figcaption>
#   <pre>...</pre>
# </figure>
```

Both `:open_tag` and `:close_tag` are required when using `:header`.

## HTML Output Structure

Lumis generates semantic HTML with line wrappers:

```html
<pre class="athl" style="color: #abb2bf; background-color: #282c34;">
  <code class="language-elixir" translate="no" tabindex="0">
    <div class="line" data-line="1">
      <span style="color: #c678dd;">defmodule</span>
      <span style="color: #e5c07b;">MyApp</span>
    </div>
    <div class="line" data-line="2">
      ...
    </div>
  </code>
</pre>
```

Key points:
- Each line is wrapped in `<div class="line" data-line="N">`
- The `data-line` attribute contains the line number (1-indexed)
- The `<code>` tag has `translate="no"` to prevent browser translation
- The `<code>` tag has `tabindex="0"` for keyboard accessibility

## Common Patterns

### Highlighting Code Blocks in Phoenix

```elixir
# In your LiveView or template
def render(assigns) do
  ~H"""
  <div class="code-container">
    <%= raw Lumis.highlight!(@code, language: @language) %>
  </div>
  """
end
```

**Important**: Always use `raw/1` to prevent double-escaping HTML.

### Streaming Code (ChatGPT-style)

Lumis handles incomplete code gracefully:

```elixir
# This works even though the code is incomplete
Lumis.highlight!("defmodule MyApp do\n  def hel", language: "elixir")
```

This is useful for streaming scenarios where code is being typed or generated incrementally.

### Dynamic Theme Selection

```elixir
def highlight_with_user_theme(code, language, user_preferences) do
  theme = user_preferences.dark_mode? && "github_dark" || "github_light"

  Lumis.highlight!(code,
    language: language,
    formatter: {:html_inline, theme: theme}
  )
end
```

### Validating Options

```elixir
# Validate options before using them
options = [
  language: "elixir",
  formatter: {:html_inline, theme: "onedark"}
]

validated = Lumis.validate_options!(options)
# Use validated options...
```

### Getting Default Options

```elixir
# Get the default options used by Lumis
defaults = Lumis.default_options()
# Returns: [language: nil, formatter: {:html_inline, [...]}]
```

## Best Practices

### DO: Specify Language When Known

Always specify the language when you know it:

```elixir
# Good
Lumis.highlight!(code, language: "elixir")

# Less ideal - auto-detection is slower
Lumis.highlight!(code)
```

### DO: Use Pattern Matching for Error Handling

```elixir
# Good
case Lumis.highlight(code, language: "unknown") do
  {:ok, html} -> html
  {:error, _} -> fallback_html(code)
end

# Or use with/1
with {:ok, html} <- Lumis.highlight(code, language: lang) do
  html
end
```

### DO: Cache Highlighted Output

Syntax highlighting is CPU-intensive. Cache the output when possible:

```elixir
# In Phoenix LiveView
def mount(_params, _session, socket) do
  code = get_code()
  highlighted = Lumis.highlight!(code, language: "elixir")

  {:ok, assign(socket, highlighted: highlighted)}
end
```

### DO: Use html_linked for Large Applications

For applications with many code blocks, use `:html_linked` to reduce HTML size:

```elixir
# Smaller HTML output
Lumis.highlight!(code,
  language: "elixir",
  formatter: :html_linked
)
```

### DON'T: Double-escape HTML

```elixir
# Bad - will show escaped HTML entities
~H"""
<div><%= Lumis.highlight!(code, language: "elixir") %></div>
"""

# Good - use raw/1
~H"""
<div><%= raw Lumis.highlight!(code, language: "elixir") %></div>
"""
```

### DON'T: Use Deprecated Options

```elixir
# Bad - deprecated
Lumis.highlight!(code, theme: "onedark", inline_style: true)

# Good - use formatter option
Lumis.highlight!(code, formatter: {:html_inline, theme: "onedark"})
```

### DON'T: Mix Formatters and Deprecated Options

```elixir
# Bad - confusing and error-prone
Lumis.highlight!(code,
  formatter: :html_inline,
  theme: "onedark"  # deprecated
)

# Good - everything in formatter options
Lumis.highlight!(code,
  formatter: {:html_inline, theme: "onedark"}
)
```

## Common Mistakes to Avoid

### Mistake: Not Using `raw/1` in Phoenix

```elixir
# Wrong - HTML will be escaped
~H"""<div><%= @highlighted_code %></div>"""

# Correct
~H"""<div><%= raw @highlighted_code %></div>"""
```

### Mistake: Forgetting CSS for html_linked

```elixir
# This will output HTML without colors
Lumis.highlight!(code, formatter: :html_linked)
```

Remember to include the CSS file in your application.

### Mistake: Invalid Line Numbers

```elixir
# Wrong - line numbers are 1-indexed
highlight_lines: %{lines: [0, 1, 2]}

# Correct - start from 1
highlight_lines: %{lines: [1, 2, 3]}
```

### Mistake: Incomplete Header Option

```elixir
# Wrong - missing close_tag
header: %{open_tag: "<div>"}

# Correct - both tags required
header: %{open_tag: "<div>", close_tag: "</div>"}
```

### Mistake: Using String for Lines

```elixir
# Wrong - lines must be integers or ranges
highlight_lines: %{lines: ["1", "2"]}

# Correct
highlight_lines: %{lines: [1, 2]}
```

## Function Quick Reference

### Main Functions

```elixir
# Highlight with error handling
{:ok, html} = Lumis.highlight(source, opts)

# Highlight and raise on error
html = Lumis.highlight!(source, opts)

# Get all available languages
%{} = Lumis.available_languages()

# Get all available themes
[] = Lumis.available_themes()

# Validate options
opts = Lumis.validate_options!(opts)

# Get default options
opts = Lumis.default_options()
```

### Theme Functions

```elixir
# Get a theme by name
%Lumis.Theme{} = Lumis.Theme.get("github_light")
%Lumis.Theme{} = Lumis.Theme.get("unknown", default_theme)

# Load theme from file
{:ok, theme} = Lumis.Theme.from_file(path)

# Load theme from JSON string
{:ok, theme} = Lumis.Theme.from_json(json_string)
```

## Options Reference

```elixir
[
  # Language specification (optional - auto-detected if omitted)
  language: "elixir" | ".ex" | "app.ex" | nil,

  # Formatter specification
  formatter:
    :html_inline |
    {:html_inline, [
      theme: "onedark" | %Lumis.Theme{},
      pre_class: "my-class",
      italic: false,
      include_highlights: false,
      highlight_lines: %{
        lines: [1, 2..5],
        style: :theme | "custom-css" | nil,
        class: "custom-class"
      },
      header: %{
        open_tag: "<div>",
        close_tag: "</div>"
      }
    ]} |
    :html_linked |
    {:html_linked, [
      pre_class: "my-class",
      highlight_lines: %{
        lines: [1, 2..5],
        class: "highlighted"
      },
      header: %{
        open_tag: "<div>",
        close_tag: "</div>"
      }
    ]} |
    :terminal |
    {:terminal, [
      theme: "onedark" | %Lumis.Theme{}
    ]} |
    :html_multi_themes |
    {:html_multi_themes, [
      themes: [light: "github_light", dark: "github_dark"],  # required
      default_theme: "light" | "light-dark()" | nil,
      css_variable_prefix: "--custom",
      pre_class: "my-class",
      italic: false,
      include_highlights: false,
      highlight_lines: %{
        lines: [1, 2..5],
        style: :theme | "custom-css" | nil,
        class: "custom-class"
      },
      header: %{
        open_tag: "<div>",
        close_tag: "</div>"
      }
    ]}
]
```

## Summary

Lumis is a fast, reliable syntax highlighter for Elixir. Key points to remember:

1. **Always specify language when known** for better performance
2. **Use `raw/1` in Phoenix templates** to prevent HTML escaping
3. **Cache highlighted output** when possible
4. **Use `:html_linked` for large applications** to reduce HTML size
5. **Use `:html_multi_themes` for light/dark mode** support with CSS custom properties
6. **Include CSS files** when using `:html_linked` formatter
7. **Handles incomplete code** gracefully for streaming scenarios
8. **70+ languages** with auto-detection support
9. **117 Neovim themes** available
10. **Line numbers** are 1-indexed in the `data-line` attribute
11. **Validate options** with `validate_options!/1` when needed

For more information, see the [HexDocs](https://hexdocs.pm/lumis) or the [GitHub repository](https://github.com/leandrocp/lumis).

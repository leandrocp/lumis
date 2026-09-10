# Lumis - Usage Rules for AI Agents

Lumis is a syntax highlighter for Elixir that uses Tree-sitter parsers and Neovim themes. It provides fast, accurate syntax highlighting with support for 110+ languages and 250+ built-in themes, outputting to HTML (inline or linked) or terminal (ANSI codes).

## Core Concepts

### What Lumis Does
- Highlights source code using Tree-sitter parsers
- Supports 110+ programming languages with auto-detection
- Provides 250+ built-in Neovim themes (light and dark)
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
Lumis.highlight!("defmodule MyApp do", formatter: {:html_inline, language: "elixir"})

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
case Lumis.highlight(source, formatter: {:html_inline, language: "elixir"}) do
  {:ok, html} -> html
  {:error, error} -> handle_error(error)
end

# Or use the bang version when you expect success
html = Lumis.highlight!(source, formatter: {:html_inline, language: "elixir"})
```

## Language Specification

### Specifying Languages

You can specify languages in multiple ways:

```elixir
# By language name
Lumis.highlight!(code, formatter: {:html_inline, language: "elixir"})
Lumis.highlight!(code, formatter: {:html_inline, language: "javascript"})
Lumis.highlight!(code, formatter: {:html_inline, language: "rust"})

# By file extension
Lumis.highlight!(code, formatter: {:html_inline, language: ".ex"})
Lumis.highlight!(code, formatter: {:html_inline, language: ".js"})

# By filename
Lumis.highlight!(code, formatter: {:html_inline, language: "app.ex"})
Lumis.highlight!(code, formatter: {:html_inline, language: "lib/my_module.ex"})

# Auto-detection (omit language option)
Lumis.highlight!(code)
```

### Discovering Available Languages

```elixir
# Get all available languages, sorted by id
languages = Lumis.available_languages()
# Returns: [%{id: "elixir", name: "Elixir", aliases: [], extensions: ["*.ex", "*.exs"],
#            globs: ["*.ex", "*.exs"], emacs_modes: ["elixir"], shebangs: ["elixir"]}, ...]

# Check if a language is supported
Enum.any?(Lumis.available_languages(), &(&1.id == "elixir"))

# Look one up by id or alias, without scanning the catalog
Lumis.Languages.get("js")
# Returns: %{id: "javascript", name: "JavaScript", ...}
Lumis.Languages.get("not-a-language")
# Returns: nil

# Or resolve a name, path or source the way highlighting does
"elixir" = Lumis.Languages.guess("lib/app.ex")
```

### Parser Loading

Highlighting loads what a document needs, including languages injected inside
it, so nothing has to be declared up front. Loading is global to the VM, so the
cost is paid once.

Load ahead of time to keep the first download off a user's request. A cold
parser costs a download *and* a Wasmtime compile:

```elixir
:ok = Lumis.Languages.load(["markdown", "elixir", "json"])
```

From an application's `start/2`, use the async variant instead. It returns
immediately, so a slow or unreachable CDN cannot delay or fail the boot, and it
is not matched on:

```elixir
Lumis.Languages.async_load(["markdown", "elixir", "json"])
```

The store is checked before the CDN, and is also where Wasmtime persists
compiled modules. It resolves in that order: `config :lumis, data_dir:` wins,
`LUMIS_DATA_DIR` is the environment fallback, and Lumis's own `priv/lumis` is
the zero-configuration default. A cold cache resolves the runtime's compatible
package range; the exact package stored in that directory is then served
without revalidating it.

`Lumis.Languages.cache/2` and `lumis languages cache` fill that directory
without loading anything, for a build or operations step that runs before the
VM that serves. Inside a running application prefer `async_load/1`, which keeps
what it loads rather than compiling and discarding it.

## Formatters

Lumis supports five formatters. The formatter option controls the output format.

### HTML Inline (Default)

Generates HTML with inline styles. Best for email, isolated components, or when you don't want external CSS.

```elixir
# Using default formatter
Lumis.highlight!(code, formatter: {:html_inline, language: "elixir"})

# Explicitly specify with options
Lumis.highlight!(code,
  formatter: {:html_inline, [
    language: "elixir",
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
  formatter: {:html_linked, [
    language: "elixir",
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

#### Building scoped CSS

`Lumis.Theme.build_css!/2` builds a stylesheet from a theme name or `Lumis.Theme` struct, for when the bundled CSS files are not enough.

```elixir
css =
  Lumis.Theme.build_css!("github_dark",
    scope: ~s(html[data-theme="dark"]),
    container_selector: ".lumis",
    container_style: [
      {"background-color", "var(--code-background)"},
      {"border-radius", "0.375rem"}
    ]
  )
```

Options: `:scope`, `:container_selector`, `:container_style`, and `:enable_italic`. Use `:container_style` for declarations on the container selector, such as `padding`, `border-radius`, or a replacement `background-color`.

### HTML Multi-Themes

Generates HTML with CSS custom properties (variables) for multiple themes, enabling light/dark mode support. Inspired by [Shiki Dual Themes](https://shiki.style/guide/dual-themes).

```elixir
# Basic dual theme with CSS variables
Lumis.highlight!(code,
  formatter: {:html_multi_themes,
    language: "elixir",
    themes: [light: "github_light", dark: "github_dark"]
  }
)

# With light-dark() function for automatic theme switching
Lumis.highlight!(code,
  formatter: {:html_multi_themes,
    language: "elixir",
    themes: [light: "github_light", dark: "github_dark"],
    default_theme: "light-dark()"
  }
)

# With inline colors for default theme
Lumis.highlight!(code,
  formatter: {:html_multi_themes,
    language: "elixir",
    themes: [light: "github_light", dark: "github_dark"],
    default_theme: "light"
  }
)

# Multiple themes with custom prefix
Lumis.highlight!(code,
  formatter: {:html_multi_themes,
    language: "elixir",
    themes: [light: "github_light", dark: "github_dark", dim: "catppuccin_frappe"],
    css_variable_prefix: "--code"
  }
)
```

**How it works:**
- Generates CSS custom properties like `--lumis-light-fg`, `--lumis-dark-fg`, etc.
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
  .lumis-themes {
    color: var(--lumis-light);
    background-color: var(--lumis-light-bg);
  }
}

@media (prefers-color-scheme: dark) {
  .lumis-themes {
    color: var(--lumis-dark);
    background-color: var(--lumis-dark-bg);
  }
}

/* Manual control with data attributes */
[data-theme="light"] .lumis-themes {
  color: var(--lumis-light);
  background-color: var(--lumis-light-bg);
}

[data-theme="dark"] .lumis-themes {
  color: var(--lumis-dark);
  background-color: var(--lumis-dark-bg);
}
```

Available options for `:html_multi_themes`:
- `:themes` (required) - Keyword list mapping theme identifiers to theme names or structs, e.g., `[light: "github_light", dark: "github_dark"]`
- `:default_theme` - Controls inline color rendering: theme identifier, `"light-dark()"`, or `nil` (default: `nil`)
- `:css_variable_prefix` - Custom CSS variable prefix (default: `"--lumis"`)
- `:pre_class` - CSS class to add to the `<pre>` tag
- `:italic` - Enable italic styles (default: `false`)
- `:include_highlights` - Add `data-highlight` attributes for debugging (default: `false`)
- `:highlight_lines` - Highlight specific lines (same options as `:html_inline`)
- `:header` - Wrap with custom HTML tags (same options as other formatters)

### Terminal

Generates ANSI escape codes for terminal output.

```elixir
Lumis.highlight!(code,
  formatter: {:terminal, language: "elixir"}
)

# With theme
Lumis.highlight!(code,
  formatter: {:terminal, language: "elixir", theme: "github_light"}
)
```

Available options for `:terminal`:
- `:theme` - Theme name (string) or `Lumis.Theme` struct

### BBCode Scoped

Generates nested BBCode tags using highlight scope names from the Rust implementation. Literal `[` and `]` are escaped as `&#91;` and `&#93;` so forum software does not treat source text as markup. It does not emit standard forum-style BBCode like `[b]`, `[color]`, or `[code]`.

```elixir
Lumis.highlight!(code,
  formatter: {:bbcode_scoped, language: "elixir"}
)
```

Available options for `:bbcode_scoped`:
- none

## Themes

### Using Themes

```elixir
# List all available themes, sorted by name
themes = Lumis.available_themes()
# Returns: [%{name: "dracula", appearance: "dark"}, ...]

# Or just the names
names = Enum.map(Lumis.available_themes(), & &1.name)
# Returns: ["adwaita_dark", "adwaita_light", ...]

# Use a theme by name
Lumis.highlight!(code,
  formatter: {:html_inline, theme: "dracula"}
)

# Get a theme struct
theme = Lumis.Theme.get("github_light")
Lumis.highlight!(code, formatter: {:html_inline, language: "elixir", theme: theme})
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
  formatter: {:html_inline,
    language: "elixir",
    highlight_lines: %{lines: [2, 3, 4]}
  }
)

# Explicit theme style
Lumis.highlight!(code,
  formatter: {:html_inline,
    language: "elixir",
    highlight_lines: %{lines: [1, 5..10], style: :theme}
  }
)

# Custom inline style
Lumis.highlight!(code,
  formatter: {:html_inline,
    language: "elixir",
    highlight_lines: %{
      lines: [2..4, 7],
      style: "background-color: #fff3cd; border-left: 3px solid #ffc107;"
    }
  }
)

# CSS class only (no inline style)
Lumis.highlight!(code,
  formatter: {:html_inline,
    language: "elixir",
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
# Use default "l-highlighted" class from theme CSS
Lumis.highlight!(code,
  formatter: {:html_linked,
    language: "elixir",
    highlight_lines: %{lines: [2..4, 6]}
  }
)

# Use custom CSS class
Lumis.highlight!(code,
  formatter: {:html_linked,
    language: "elixir",
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
  formatter: {:html_inline, language: "elixir", header: header}
)

# Output:
# <figure>
#   <figcaption>app.ex</figcaption>
#   <pre>...</pre>
# </figure>
```

Both `:open_tag` and `:close_tag` are required when using `:header`.

### Custom Formatters

When no built-in formatter emits what you need, implement `Lumis.Formatter` and
pass the module. Lumis highlights the source and hands `render/3` the event
stream, with the resolved language in `options`.

```elixir
defmodule MyFormatter do
  @behaviour Lumis.Formatter

  alias Lumis.Formatter.HTML

  @impl true
  def render(source, events, options) do
    language = Keyword.fetch!(options, :language)
    theme = Keyword.get(options, :theme)
    attrs = HTML.span_attrs(theme: theme, language: language)

    body =
      Enum.map(events, fn
        {:start, %{scope: scope}} -> HTML.open_span(attrs, scope)
        :end -> "</span>"
        {:source, %{start: start, end: stop}} ->
          HTML.escape(binary_part(source, start, stop - start))

        # Always keep this clause. Lumis adds event kinds as it grows, and
        # without it a newer Lumis raises FunctionClauseError instead of
        # rendering.
        _event -> []
      end)

    [HTML.open_pre_tag(theme: theme), HTML.open_code_tag(language), body, HTML.closing_tags()]
  end
end

Lumis.highlight!(code, formatter: {MyFormatter, language: "elixir", theme: "github_light"})
```

`render/3` returns iodata; Lumis flattens it once at the end.

Do not hand-roll HTML escaping or scope-to-class mapping. `Lumis.Formatter.HTML`
gives the built-in formatters' pieces:

- `escape/1`, `escape_braces/1`
- `scope_to_class/1`, `span_linked_attrs/1`, `span_linked/2`
- `span_attrs/1`, `open_span/2`, `span_inline_attrs/2`, `span_inline/3`
- `open_pre_tag/1`, `open_code_tag/1`, `closing_tags/0`, `wrap_line/3`

`span_attrs/1` and `classes/0` return whole tables because resolving a scope is
a per-token operation. Build the table once outside the loop and read it inside.

Do not hand-roll ANSI color or text-decoration escape sequences either.
`Lumis.Formatter.ANSI` gives `:terminal`'s pieces:

- `hex_to_rgb/1`, `rgb_to_ansi/4`
- `style_to_ansi/1`, `paint/2`, `reset/0`
- `styles/1`, `style_for/2`

Do not resolve a scope with `Map.get(theme.highlights, scope)`. That misses the
fallbacks `:terminal` applies — `tag.delimiter` is painted by `tag` in a theme
that styles only `tag` — so it paints differently. Use `styles/1` and read it
with `style_for/2`, and build one table per language you meet: a theme can style
a scope per language, and an injected block carries its own language on its
`:start` event.

## HTML Output Structure

Lumis generates semantic HTML with line wrappers:

```html
<pre class="lumis" style="color: #abb2bf; background-color: #282c34;">
  <code class="language-elixir" translate="no" tabindex="0">
    <div class="l-line" data-line="1">
      <span style="color: #c678dd;">defmodule</span>
      <span style="color: #e5c07b;">MyApp</span>
    </div>
    <div class="l-line" data-line="2">
      ...
    </div>
  </code>
</pre>
```

Key points:
- Each line is wrapped in `<div class="l-line" data-line="N">`
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
    <%= raw Lumis.highlight!(@code, formatter: {:html_inline, language: @language}) %>
  </div>
  """
end
```

**Important**: Always use `raw/1` to prevent double-escaping HTML.

### Streaming Code (ChatGPT-style)

Lumis handles incomplete code gracefully:

```elixir
# This works even though the code is incomplete
Lumis.highlight!("defmodule MyApp do\n  def hel", formatter: {:html_inline, language: "elixir"})
```

This is useful for streaming scenarios where code is being typed or generated incrementally.

### Dynamic Theme Selection

```elixir
def highlight_with_user_theme(code, language, user_preferences) do
  theme = user_preferences.dark_mode? && "github_dark" || "github_light"

  Lumis.highlight!(code,
    formatter: {:html_inline, language: language, theme: theme}
  )
end
```

### Validating Options

```elixir
# Validate options before using them
options = [
  formatter: {:html_inline, language: "elixir", theme: "onedark"}
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
Lumis.highlight!(code, formatter: {:html_inline, language: "elixir"})

# Less ideal - auto-detection is slower
Lumis.highlight!(code)
```

### DO: Use Pattern Matching for Error Handling

```elixir
# Good
case Lumis.highlight(code, formatter: {:html_inline, language: "unknown"}) do
  {:ok, html} -> html
  {:error, _} -> fallback_html(code)
end

# Or use with/1
with {:ok, html} <- Lumis.highlight(code, formatter: {:html_inline, language: language}) do
  html
end
```

### DO: Cache Highlighted Output

Syntax highlighting is CPU-intensive. Cache the output when possible:

```elixir
# In Phoenix LiveView
def mount(_params, _session, socket) do
  code = get_code()
  highlighted = Lumis.highlight!(code, formatter: {:html_inline, language: "elixir"})

  {:ok, assign(socket, highlighted: highlighted)}
end
```

### DO: Use html_linked for Large Applications

For applications with many code blocks, use `:html_linked` to reduce HTML size:

```elixir
# Smaller HTML output
Lumis.highlight!(code,
  formatter: {:html_linked, language: "elixir"}
)
```

### DON'T: Double-escape HTML

```elixir
# Bad - will show escaped HTML entities
~H"""
<div><%= Lumis.highlight!(code, formatter: {:html_inline, language: "elixir"}) %></div>
"""

# Good - use raw/1
~H"""
<div><%= raw Lumis.highlight!(code, formatter: {:html_inline, language: "elixir"}) %></div>
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

# Render the event stream yourself
html = Lumis.highlight!(source, formatter: {MyFormatter, language: "elixir"})

# Get all available languages
[%{id: _} | _] = Lumis.available_languages()

# Get one language by id or alias
%{id: "javascript"} = Lumis.Languages.get("js")

# Get the ids loaded into this VM
[_ | _] = Lumis.loaded_languages()

# Get all available themes
[%{name: _, appearance: _} | _] = Lumis.available_themes()

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
  # Formatter specification
  formatter:
    :html_inline |
    {:html_inline, [
      language: "elixir" | ".ex" | "app.ex" | nil,
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
      language: "elixir" | ".ex" | "app.ex" | nil,
      pre_class: "my-class",
      highlight_lines: %{
        lines: [1, 2..5],
        class: "l-highlighted"
      },
      header: %{
        open_tag: "<div>",
        close_tag: "</div>"
      }
    ]} |
    :terminal |
    {:terminal, [
      language: "elixir" | ".ex" | "app.ex" | nil,
      theme: "onedark" | %Lumis.Theme{}
    ]} |
    :html_multi_themes |
    {:html_multi_themes, [
      language: "elixir" | ".ex" | "app.ex" | nil,
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
    ]} |
    :bbcode_scoped |
    {:bbcode_scoped, [
      language: "elixir" | ".ex" | "app.ex" | nil
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
8. **110+ languages** with auto-detection support
9. **250+ built-in Neovim themes** available
10. **Line numbers** are 1-indexed in the `data-line` attribute
11. **Validate options** with `validate_options!/1` when needed
12. **Cache parser WASMs** at build time so the first request does not download

For more information, see the [HexDocs](https://hexdocs.pm/lumis) or the [GitHub repository](https://github.com/leandrocp/lumis).

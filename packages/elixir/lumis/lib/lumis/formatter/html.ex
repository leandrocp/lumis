defmodule Lumis.Formatter.HTML do
  @moduledoc """
  The HTML pieces the built-in formatters are assembled from.

  A module implementing `Lumis.Formatter` gets an event stream and has to turn
  it into markup; these are the parts of that job worth not writing again.

  Every function here calls into the same code the built-in `:html_inline` and
  `:html_linked` formatters use, so output built with them is styled by the same
  theme stylesheets and escapes the same characters.

  ## Example

      defmodule MyFormatter do
        @behaviour Lumis.Formatter

        alias Lumis.Formatter.HTML

        @impl true
        def render(source, events, options) do
          language = Keyword.fetch!(options, :language)
          attrs = HTML.span_attrs(theme: Keyword.get(options, :theme), language: language)

          body =
            Enum.map(events, fn
              {:start, %{scope: scope}} -> HTML.open_span(attrs, scope)
              :end -> "</span>"
              {:source, %{start: start, end: stop}} ->
                HTML.escape(binary_part(source, start, stop - start))

              # Lumis adds event kinds over time. Render the ones you know.
              _event -> []
            end)

          [
            HTML.open_pre_tag(theme: Keyword.get(options, :theme)),
            HTML.open_code_tag(language),
            body,
            HTML.closing_tags()
          ]
        end
      end
  """

  alias Lumis.LineSpec
  alias Lumis.Native
  alias Lumis.Theme
  alias Lumis.Theme.Style
  alias Lumis.Theme.TextDecoration

  @default_class "l-text"
  @default_css_variable_prefix "--lumis"

  @doc """
  Escapes `&`, `<`, `>`, `"` and `'`.

      iex> Lumis.Formatter.HTML.escape(~s|a < b && c|)
      "a &lt; b &amp;&amp; c"

  """
  @spec escape(String.t()) :: String.t()
  def escape(text) when is_binary(text), do: Native.html_escape(text)

  @doc """
  Escapes a value going inside a double-quoted attribute.

  The helpers here already escape every attribute they build; this is for a
  formatter that assembles its own tags. The escape set is `escape/1`'s, which is
  safe for CSS in an attribute because the HTML parser decodes the entity before
  the CSS parser sees it.

      iex> Lumis.Formatter.HTML.escape_attr(~s|x"><script>|)
      "x&quot;&gt;&lt;script&gt;"

      iex> Lumis.Formatter.HTML.escape_attr("font-family: 'Fira Code'")
      "font-family: &#39;Fira Code&#39;"

  """
  @spec escape_attr(String.t()) :: String.t()
  def escape_attr(value) when is_binary(value), do: Native.html_escape_attr(value)

  @doc """
  Escapes `{` and `}`, which a templating language such as HEEx would otherwise read.

      iex> Lumis.Formatter.HTML.escape_braces("%{a: 1}")
      "%&lbrace;a: 1&rbrace;"

  """
  @spec escape_braces(String.t()) :: String.t()
  def escape_braces(text) when is_binary(text), do: Native.html_escape_braces(text)

  @doc """
  Every highlight scope mapped to the CSS class `:html_linked` gives it.

  Read once and keep it: `scope_to_class/1` looks a scope up in this, and calling
  it per token is the whole reason the table exists.
  """
  @spec classes() :: %{String.t() => String.t()}
  def classes do
    case :persistent_term.get({__MODULE__, :classes}, nil) do
      nil ->
        classes = Native.html_classes()
        :persistent_term.put({__MODULE__, :classes}, classes)
        classes

      classes ->
        classes
    end
  end

  @doc """
  The CSS class for a scope, or `"l-text"` for one Lumis does not know.

      iex> Lumis.Formatter.HTML.scope_to_class("keyword.function")
      "l-keyword-function"

      iex> Lumis.Formatter.HTML.scope_to_class("not.a.scope")
      "l-text"

  """
  @spec scope_to_class(String.t() | nil) :: String.t()
  def scope_to_class(nil), do: @default_class
  def scope_to_class(scope) when is_binary(scope), do: Map.get(classes(), scope, @default_class)

  @doc """
  The `class` attribute for a scope, for output styled by a theme stylesheet.

      iex> Lumis.Formatter.HTML.span_linked_attrs("keyword")
      ~s|class="l-keyword"|

  """
  @spec span_linked_attrs(String.t() | nil) :: String.t()
  def span_linked_attrs(scope), do: ~s|class="#{scope_to_class(scope)}"|

  @doc """
  A `<span>` carrying a scope's CSS class, with `text` escaped.

      iex> Lumis.Formatter.HTML.span_linked("defmodule", "keyword.function")
      ~s|<span class="l-keyword-function">defmodule</span>|

  """
  @spec span_linked(String.t(), String.t() | nil) :: String.t()
  def span_linked(text, scope) do
    "<span #{span_linked_attrs(scope)}>#{escape(text)}</span>"
  end

  @doc """
  Every scope's `<span>` attributes for one theme and language.

  The inline-style counterpart of `classes/0`, and the same advice applies: build
  it once per document and read it per token with `open_span/2` or
  `span_inline_attrs/2`.

  ## Options

    * `:theme` (`t:Lumis.Theme.t/0` or a theme name) — resolves each scope to its
      style. Without one every scope's attributes are `""`, which is a `<span>`
      with nothing on it.
    * `:language` — the language whose specialized scopes to prefer, e.g.
      `comment.elixir` over `comment`. Defaults to `"plaintext"`.
    * `:italic` (default `false`) — emit `font-style: italic` for a style that
      asks for it
    * `:include_highlights` (default `false`) — add `data-highlight="<scope>"`

  """
  @spec span_attrs(keyword()) :: %{String.t() => String.t()}
  def span_attrs(options \\ []) when is_list(options) do
    Native.html_span_attrs(
      resolve_theme(Keyword.get(options, :theme)),
      Keyword.get(options, :language) || "plaintext",
      Keyword.get(options, :italic, false),
      Keyword.get(options, :include_highlights, false)
    )
  end

  @doc """
  A scope's `<span>` attributes, read out of a `span_attrs/1` table.

  Returns `""` for a scope the theme styles in no way, which is a `<span>` with
  no attributes rather than one with an empty `style`.
  """
  @spec span_inline_attrs(%{String.t() => String.t()}, String.t() | nil) :: String.t()
  def span_inline_attrs(_attrs, nil), do: ""
  def span_inline_attrs(attrs, scope), do: Map.get(attrs, scope, "")

  @doc """
  An opening `<span>` for a scope, with its attributes from a `span_attrs/1` table.

  A scope the theme does not style opens a bare `<span>`, so every `<span>` still
  pairs with the `</span>` an `:end` event writes.
  """
  @spec open_span(%{String.t() => String.t()}, String.t() | nil) :: String.t()
  def open_span(attrs, scope) do
    case span_inline_attrs(attrs, scope) do
      "" -> "<span>"
      attrs -> "<span #{attrs}>"
    end
  end

  @doc """
  A `<span>` with a theme's colors written inline, with `text` escaped.

  Convenient for a one-off; in a loop, hoist `span_attrs/1` out and use
  `open_span/2` so the table is not rebuilt per token.
  """
  @spec span_inline(String.t(), %{String.t() => String.t()}, String.t() | nil) :: String.t()
  def span_inline(text, attrs, scope) do
    "#{open_span(attrs, scope)}#{escape(text)}</span>"
  end

  @doc """
  Every scope's `<span>` attributes for a set of themes, as CSS custom properties.

  The multi-theme counterpart of `span_attrs/1`, and the same advice applies:
  build it once per document and read it per token with `open_span/2`. The result
  is a table `open_span/2` and `span_multi_themes/3` both read.

  Each theme's style becomes `--<prefix>-<theme>` custom properties, so one
  document can be restyled by CSS alone. `:default_theme` names the one written
  inline; the rest stay variables.

  ## Options

    * `:themes` (required) — the same `[light: "github_light", dark: dark_theme]`
      keyword list `:html_multi_themes` takes, or a map. The names become the CSS
      variable suffixes and the `<pre>` classes `open_multi_themes_pre_tag/1`
      writes.
    * `:default_theme` — the theme written inline. `"light-dark()"` writes the
      themes named `light` and `dark` into CSS `light-dark()` calls and emits no
      variables. Without one every theme is a variable and nothing is inline.
    * `:css_variable_prefix` (default `"--lumis"`) — the custom property prefix
    * `:language` — the language whose specialized scopes to prefer, e.g.
      `comment.elixir` over `comment`. Defaults to `"plaintext"`.
    * `:italic` (default `false`) — emit `font-style: italic` for a style that
      asks for it
    * `:include_highlights` (default `false`) — add `data-highlight="<scope>"`

  """
  @spec span_multi_themes_attrs(keyword()) :: %{String.t() => String.t()}
  def span_multi_themes_attrs(options \\ []) when is_list(options) do
    Native.html_multi_themes_span_attrs(
      resolve_themes(Keyword.get(options, :themes, %{})),
      Keyword.get(options, :default_theme),
      css_variable_prefix(options),
      Keyword.get(options, :language) || "plaintext",
      Keyword.get(options, :italic, false),
      Keyword.get(options, :include_highlights, false)
    )
  end

  @doc """
  A `<span>` carrying every theme's style as CSS custom properties, with `text` escaped.

  `attrs` is a `span_multi_themes_attrs/1` table. The inline counterpart is
  `span_inline/3`, and the two differ only in which table they read.
  """
  @spec span_multi_themes(String.t(), %{String.t() => String.t()}, String.t() | nil) :: String.t()
  def span_multi_themes(text, attrs, scope) do
    "#{open_span(attrs, scope)}#{escape(text)}</span>"
  end

  @doc """
  The opening `<pre>` tag, carrying a theme's own colors when one is given.

  ## Options

    * `:theme` (`t:Lumis.Theme.t/0` or a theme name) — writes the theme's
      `normal` colors into a `style` attribute, the way `:html_inline` does
    * `:class` — appended to the `lumis` class every Lumis block carries

  ## Example

      iex> Lumis.Formatter.HTML.open_pre_tag(class: "my-block")
      ~s|<pre class="lumis my-block">|

  """
  @spec open_pre_tag(keyword()) :: String.t()
  def open_pre_tag(options \\ []) when is_list(options) do
    Native.html_open_pre_tag(
      Keyword.get(options, :class),
      resolve_theme(Keyword.get(options, :theme))
    )
  end

  @doc """
  The opening `<pre>` tag for a multi-theme block.

  Carries `lumis`, `lumis-themes` and one class per theme name, so a stylesheet
  can select the active theme, and the same `normal` colors
  `span_multi_themes_attrs/1` writes per scope.

  ## Options

    * `:themes` (required) — the same keyword list or map `span_multi_themes_attrs/1` takes
    * `:default_theme` — the theme written inline; `"light-dark()"` writes both
      the `light` and `dark` themes into CSS `light-dark()` calls
    * `:css_variable_prefix` (default `"--lumis"`) — the custom property prefix
    * `:class` — appended to the classes above

  ## Example

      iex> Lumis.Formatter.HTML.open_multi_themes_pre_tag(themes: [light: "github_light"], default_theme: "light")
      ~s|<pre class="lumis lumis-themes light" style="color:#1f2328; background-color:#ffffff;">|

  """
  @spec open_multi_themes_pre_tag(keyword()) :: String.t()
  def open_multi_themes_pre_tag(options \\ []) when is_list(options) do
    Native.html_open_multi_themes_pre_tag(
      Keyword.get(options, :class),
      resolve_themes(Keyword.get(options, :themes, %{})),
      Keyword.get(options, :default_theme),
      css_variable_prefix(options)
    )
  end

  defp css_variable_prefix(options) do
    case Keyword.get(options, :css_variable_prefix) do
      nil -> @default_css_variable_prefix
      prefix -> prefix
    end
  end

  @doc """
  The opening `<code>` tag for a language.

      iex> Lumis.Formatter.HTML.open_code_tag("elixir")
      ~s|<code class="language-elixir" translate="no" tabindex="0">|

  A formatter reads its language from the `:language` option `render/3` receives,
  which Lumis has already resolved by detection when the caller named none.
  """
  @spec open_code_tag(String.t() | nil) :: String.t()
  def open_code_tag(language), do: Native.html_open_code_tag(language || "plaintext")

  @doc """
  Both closing tags, in the order `open_code_tag/1` and `open_pre_tag/1` opened them.

      iex> Lumis.Formatter.HTML.closing_tags()
      "</code></pre>"

  """
  @spec closing_tags() :: String.t()
  def closing_tags, do: Native.html_closing_tags()

  @doc """
  The closing `</pre>` tag.

      iex> Lumis.Formatter.HTML.close_pre_tag()
      "</pre>"

  """
  @spec close_pre_tag() :: String.t()
  def close_pre_tag, do: Native.html_close_pre_tag()

  @doc """
  The closing `</code>` tag.

      iex> Lumis.Formatter.HTML.close_code_tag()
      "</code>"

  """
  @spec close_code_tag() :: String.t()
  def close_code_tag, do: Native.html_close_code_tag()

  @doc """
  A theme name with everything but letters, digits, `-` and `_` replaced by `-`.

  The form a theme name takes inside the CSS custom properties
  `span_multi_themes_attrs/1` writes.

      iex> Lumis.Formatter.HTML.sanitize_theme_name("Catppuccin Mocha")
      "Catppuccin-Mocha"

  """
  @spec sanitize_theme_name(String.t()) :: String.t()
  def sanitize_theme_name(name) when is_binary(name), do: Native.html_sanitize_theme_name(name)

  @doc """
  The CSS `text-decoration` value for a `Lumis.Theme.TextDecoration`.

      iex> Lumis.Formatter.HTML.text_decoration(%Lumis.Theme.TextDecoration{underline: :wavy, strikethrough: true})
      "underline wavy line-through"

      iex> Lumis.Formatter.HTML.text_decoration(%Lumis.Theme.TextDecoration{})
      "none"

  """
  @spec text_decoration(TextDecoration.t()) :: String.t()
  def text_decoration(%TextDecoration{} = text_decoration),
    do: Native.html_text_decoration(text_decoration)

  @doc """
  A `Lumis.Theme.Style` as inline CSS declarations.

  The same declarations `span_attrs/1` puts in a `style` attribute, for a
  formatter that styles something other than a `<span>`.

  ## Options

    * `:italic` (default `false`) — emit `font-style: italic` for a style that
      asks for it
    * `:separator` (default `" "`) — what goes between declarations

  ## Example

      iex> Lumis.Formatter.HTML.style_to_css(%Lumis.Theme.Style{fg: "#ff79c6", bold: true})
      "color: #ff79c6; font-weight: bold;"

  """
  @spec style_to_css(Style.t(), keyword()) :: String.t()
  def style_to_css(%Style{} = style, options \\ []) when is_list(options) do
    Native.html_style_to_css(
      style,
      Keyword.get(options, :italic, false),
      Keyword.get(options, :separator, " ")
    )
  end

  @doc """
  Wraps one rendered line in the `<div>` the built-in HTML formatters emit.

  Lines are 1-based, and `data-line` is what a "highlight these lines" feature
  and anchor links both key off.

  `content` goes in verbatim. A line from `render_lines_from_events/3` carries no
  trailing newline; the built-in formatters put one inside the `<div>` so the
  block still copies as lines, which is `wrap_line(n, [line, "\\n"])`.

  ## Options

    * `:class_suffix` — appended to `l-line` verbatim, so pass a leading space,
      e.g. `" l-highlighted"`
    * `:style` — a `style` attribute for the line

  ## Example

      iex> Lumis.Formatter.HTML.wrap_line(2, "code")
      ~s|<div class="l-line" data-line="2">code</div>|

  """
  @spec wrap_line(pos_integer(), iodata(), keyword()) :: String.t()
  def wrap_line(line_number, content, options \\ []) when is_list(options) do
    Native.html_wrap_line(
      line_number,
      IO.iodata_to_binary(content),
      Keyword.get(options, :class_suffix),
      Keyword.get(options, :style)
    )
  end

  @doc """
  Whether a line falls inside a list of line numbers and ranges.

  Lines are 1-based, matching the `data-line` `wrap_line/3` writes and the
  `:highlight_lines` option the built-in formatters take.

  A range's step counts: `1..9//2` is five lines, not nine. Stepped ranges stay
  compact across the native boundary, regardless of their declared span.

      iex> Lumis.Formatter.HTML.line_is_highlighted([1, 3..5], 4)
      true

      iex> Lumis.Formatter.HTML.line_is_highlighted([1, 3..5], 2)
      false

      iex> Lumis.Formatter.HTML.line_is_highlighted([1..9//2], 4)
      false

  """
  @spec line_is_highlighted([pos_integer() | Range.t()], pos_integer()) :: boolean()
  def line_is_highlighted(lines, line_number) when is_list(lines) do
    Native.html_line_is_highlighted(LineSpec.encode!(lines), line_number)
  end

  @doc """
  The CSS class a highlighted line carries, or `nil` when the line is not highlighted.

  Reads `lines` the way `line_is_highlighted/2` does, including range steps and
  direction.

  ## Options

    * `:class` — the class to use, taking precedence over `:default_class`
    * `:default_class` — what to fall back to, e.g. `"l-highlighted"`

  ## Example

      iex> Lumis.Formatter.HTML.highlight_line_class([1, 3..5], 4, default_class: "l-highlighted")
      "l-highlighted"

      iex> Lumis.Formatter.HTML.highlight_line_class([1, 3..5], 2, default_class: "l-highlighted")
      nil

  """
  @spec highlight_line_class([pos_integer() | Range.t()], pos_integer(), keyword()) ::
          String.t() | nil
  def highlight_line_class(lines, line_number, options \\ [])
      when is_list(lines) and is_list(options) do
    Native.html_highlight_line_class(
      LineSpec.encode!(lines),
      line_number,
      Keyword.get(options, :class),
      Keyword.get(options, :default_class)
    )
  end

  @doc """
  The event stream rendered into HTML lines, one string per line.

  A `<span>` that crosses a newline is closed at the end of one line and reopened
  at the start of the next, so every line's tags nest on their own and can be
  wrapped with `wrap_line/3` independently. That closing and reopening is the
  part worth not writing again.

  `attrs` maps a scope to the attributes its `<span>` carries, so the whole render
  costs one call rather than one per token: a `span_attrs/1` or
  `span_multi_themes_attrs/1` table, or for class-based output,
  `Map.new(classes(), fn {scope, _} -> {scope, span_linked_attrs(scope)} end)`.
  A scope the table does not carry opens a bare `<span>`.

  Event kinds this build does not render — annotations, and anything a newer
  Lumis adds — are skipped rather than raising.

  ## Example

      iex> events = [{:start, %{scope: "keyword", language: "elixir"}}, {:source, %{start: 0, end: 3}}, :end]
      iex> Lumis.Formatter.HTML.render_lines_from_events("a\\nb", events, %{"keyword" => ~s|class="l-keyword"|})
      [~s|<span class="l-keyword">a</span>|, ~s|<span class="l-keyword">b</span>|]

  """
  @spec render_lines_from_events(String.t(), [Lumis.Formatter.event(term())], %{
          String.t() => String.t()
        }) :: [String.t()]
  def render_lines_from_events(source, events, attrs)
      when is_binary(source) and is_list(events) and is_map(attrs) do
    Native.html_render_lines_from_events(source, events, attrs)
  end

  defp resolve_theme(nil), do: nil
  defp resolve_theme(%Theme{} = theme), do: theme
  defp resolve_theme(name) when is_binary(name), do: Theme.get(name)

  # Accepts the `themes: [light: "github_light"]` keyword list `:html_multi_themes`
  # takes, and a map. A name no theme resolves to drops out, the way a `:theme`
  # that resolves to nothing leaves `open_pre_tag/1` unstyled.
  defp resolve_themes(themes) do
    themes
    |> Enum.flat_map(fn {name, theme} ->
      case resolve_theme(theme) do
        nil -> []
        theme -> [{to_string(name), theme}]
      end
    end)
    |> Map.new()
  end
end

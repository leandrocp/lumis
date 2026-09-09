defmodule Lumis.Formatter.HTML do
  @moduledoc """
  The HTML pieces the built-in formatters are assembled from.

  The Elixir counterpart of `lumis::formatters::html` in Rust and
  `@lumis-sh/lumis/formatters/html` in JavaScript. A module implementing
  `Lumis.Formatter` gets an event stream and has to turn it into markup; these
  are the parts of that job worth not writing again.

  Every function here calls into the same Rust that the built-in `:html_inline`
  and `:html_linked` formatters call, so output built with them is styled by the
  same theme stylesheets and escapes the same characters.

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

  ## Where the work happens

  Everything below is Rust. What a formatter calls once per document crosses the
  NIF boundary as a call; the two things it calls once per *token* — resolving a
  scope to a class, and resolving a scope to a theme's inline style — cross once
  per document as a table, because a NIF call per token to look up a constant
  costs more than the highlighting did. `classes/0` and `span_attrs/1` return
  those tables; `scope_to_class/1` and `open_span/2` read them.
  """

  alias Lumis.Native
  alias Lumis.Theme

  @default_class "l-text"

  @doc """
  Escapes `&`, `<`, `>`, `"` and `'`.

      iex> Lumis.Formatter.HTML.escape(~s|a < b && c|)
      "a &lt; b &amp;&amp; c"

  """
  @spec escape(String.t()) :: String.t()
  def escape(text) when is_binary(text), do: Native.html_escape(text)

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
  Wraps one rendered line in the `<div>` the built-in HTML formatters emit.

  Lines are 1-based, and `data-line` is what a "highlight these lines" feature
  and anchor links both key off.

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

  defp resolve_theme(nil), do: nil
  defp resolve_theme(%Theme{} = theme), do: theme
  defp resolve_theme(name) when is_binary(name), do: Theme.get(name)
end

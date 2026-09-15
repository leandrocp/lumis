defmodule Lumis.Formatter.ANSI do
  @moduledoc """
  The ANSI pieces the built-in `:terminal` formatter is assembled from.

  A module implementing `Lumis.Formatter` gets an event stream and has to turn
  it into output; these helpers apply the same colors, text decorations, and
  reset behavior as the built-in formatter.

  ## Example

      defmodule MyTerminalFormatter do
        @behaviour Lumis.Formatter

        alias Lumis.Formatter.ANSI

        @impl true
        def render(source, events, options) do
          theme = Keyword.get(options, :theme)

          {output, _scopes, _tables} =
            Enum.reduce(events, {[], [], %{}}, fn
              {:start, %{scope: scope, language: language}}, {output, scopes, tables} ->
                {output, [{scope, language} | scopes], tables}

              :end, {output, [_scope | scopes], tables} ->
                {output, scopes, tables}

              {:source, %{start: start, end: stop}}, {output, scopes, tables} ->
                text = binary_part(source, start, stop - start)

                case scopes do
                  [] ->
                    {[text | output], scopes, tables}

                  [{scope, language} | _rest] ->
                    # One table per language, built once and read per token.
                    tables =
                      Map.put_new_lazy(tables, language, fn ->
                        ANSI.styles(theme: theme, language: language)
                      end)

                    painted = ANSI.paint(text, ANSI.style_for(tables[language], scope))
                    {[painted | output], scopes, tables}
                end

              # Lumis adds event kinds as it grows. Render the ones you know and
              # skip the rest, or a newer Lumis raises FunctionClauseError here.
              _event, state ->
                state
            end)

          Enum.reverse(output)
        end
      end

      Lumis.highlight!("defmodule App do\\nend",
        formatter: {MyTerminalFormatter, language: "elixir", theme: "dracula"}
      )
  """

  alias Lumis.Native
  alias Lumis.Theme
  alias Lumis.Theme.Style

  @doc """
  Converts a six-digit hex color to an RGB tuple.

  A leading `#` is optional. Returns `nil` when the color is not six hexadecimal
  digits.

      iex> Lumis.Formatter.ANSI.hex_to_rgb("#ff79c6")
      {255, 121, 198}

      iex> Lumis.Formatter.ANSI.hex_to_rgb("fff")
      nil

  """
  @spec hex_to_rgb(String.t()) :: {0..255, 0..255, 0..255} | nil
  def hex_to_rgb(hex) when is_binary(hex), do: Native.ansi_hex_to_rgb(hex)

  @doc """
  Builds a 24-bit ANSI color escape sequence from RGB components.

  Set `is_background` to `true` for a background color and `false` for a
  foreground color.
  """
  @spec rgb_to_ansi(0..255, 0..255, 0..255, boolean()) :: String.t()
  def rgb_to_ansi(r, g, b, is_background)
      when r in 0..255 and g in 0..255 and b in 0..255 and is_boolean(is_background) do
    Native.ansi_rgb_to_ansi(r, g, b, is_background)
  end

  @doc """
  Converts a `Lumis.Theme.Style` to ANSI escape sequences.

  Covers foreground and background colors, bold, italic, strikethrough, and
  every underline style supported by `Lumis.Theme.TextDecoration`.
  """
  @spec style_to_ansi(Style.t()) :: String.t()
  def style_to_ansi(%Style{} = style), do: Native.ansi_style_to_ansi(style)

  @doc """
  Every scope's style for one theme and language.

  Resolving a scope is a per-token operation, so this is the whole table: build
  it once outside the loop and read it inside with `style_for/2`. A scope the
  theme styles in no way is absent from it.

  A scope resolves the way `:terminal` resolves it, which is not a lookup in
  `theme.highlights` — `tag.delimiter` falls back to `tag`, and a theme can
  style a scope per language. An injected block carries its own language on its
  `:start` event, so a document with injections needs one table per language.

  ## Options

    * `:theme` (`t:Lumis.Theme.t/0` or a theme name) — the theme to resolve
      against. Without one the table is empty, which paints nothing.
    * `:language` — the language whose specialized scopes to prefer, e.g.
      `comment.elixir` over `comment`. Defaults to `"plaintext"`.

  """
  @spec styles(keyword()) :: %{String.t() => Style.t()}
  def styles(options \\ []) when is_list(options) do
    Native.ansi_styles(
      resolve_theme(Keyword.get(options, :theme)),
      Keyword.get(options, :language) || "plaintext"
    )
  end

  @doc """
  A scope's style, read out of a `styles/1` table.

  Returns `nil` for a scope the theme styles in no way, which `paint/2` renders
  unchanged.
  """
  @spec style_for(%{String.t() => Style.t()}, String.t() | nil) :: Style.t() | nil
  def style_for(_styles, nil), do: nil
  def style_for(styles, scope) when is_binary(scope), do: Map.get(styles, scope)

  @doc """
  Paints `text` with a `Lumis.Theme.Style`.

  The result uses the same reset and newline handling as the built-in
  `:terminal` formatter. Text with an empty style, or with the `nil` that
  `style_for/2` returns for an unstyled scope, is returned unchanged.
  """
  @spec paint(String.t(), Style.t() | nil) :: String.t()
  def paint(text, nil) when is_binary(text), do: text
  def paint(text, %Style{} = style) when is_binary(text), do: Native.ansi_paint(text, style)

  @doc "Returns the ANSI sequence that clears all formatting."
  @spec reset() :: String.t()
  def reset, do: Native.ansi_reset()

  defp resolve_theme(nil), do: nil
  defp resolve_theme(%Theme{} = theme), do: theme
  defp resolve_theme(name) when is_binary(name), do: Theme.get(name)
end

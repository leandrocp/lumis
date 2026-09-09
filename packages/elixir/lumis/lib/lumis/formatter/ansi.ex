defmodule Lumis.Formatter.ANSI do
  @moduledoc """
  The ANSI pieces the built-in `:terminal` formatter is assembled from.

  A module implementing `Lumis.Formatter` gets an event stream and has to turn
  it into output; these helpers apply the same colors, text decorations, and
  reset behavior as the built-in formatter.
  """

  alias Lumis.Native
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
  Paints `text` with a `Lumis.Theme.Style`.

  The result uses the same reset and newline handling as the built-in
  `:terminal` formatter. Text with an empty style is returned unchanged.
  """
  @spec paint(String.t(), Style.t()) :: String.t()
  def paint(text, %Style{} = style) when is_binary(text), do: Native.ansi_paint(text, style)

  @doc "Returns the ANSI sequence that clears all formatting."
  @spec reset() :: String.t()
  def reset, do: Native.ansi_reset()
end

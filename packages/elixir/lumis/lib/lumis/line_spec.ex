defmodule Lumis.LineSpec do
  @moduledoc false

  # The one place a line number or `Range` becomes something Rust can read.
  #
  # Elixir's `Range` carries a direction and a step, so both cross the NIF
  # boundary with the two endpoints. Rust keeps that arithmetic progression
  # compact and clips it to the rendered document; no range is expanded into
  # one term per selected line.

  @typedoc false
  @type t :: pos_integer() | Range.t()

  @typedoc false
  @type encoded ::
          {:single, integer()}
          | {:range, %{start: integer(), end: integer(), step: integer()}}

  @doc false
  @spec encode([t()]) :: {:ok, [encoded()]}
  def encode(lines) when is_list(lines), do: {:ok, Enum.map(lines, &encode_one/1)}

  @doc false
  @spec encode!([t()]) :: [encoded()]
  def encode!(lines) when is_list(lines), do: Enum.map(lines, &encode_one/1)

  defp encode_one(%Range{first: first, last: last, step: step}),
    do: {:range, %{start: first, end: last, step: step}}

  defp encode_one(line) when is_integer(line), do: {:single, line}
end

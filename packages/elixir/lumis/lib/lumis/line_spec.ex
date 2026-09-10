defmodule Lumis.LineSpec do
  @moduledoc false

  # The one place a line number or `Range` becomes something Rust can read.
  #
  # `{:range, ...}` arrives as a `RangeInclusive<usize>`, which has no step and
  # cannot run backwards, so only a plain ascending range survives as one.
  # `1..5//2` sent as `1..=5` highlights the two lines the step excluded, and
  # `5..3` sent as `5..=3` matches nothing at all, so both expand into single
  # lines instead. A line range is short, so expanding it costs nothing.

  @type t :: pos_integer() | Range.t()

  @spec encode([t()]) :: [
          {:single, pos_integer()} | {:range, %{start: integer(), end: integer()}}
        ]
  def encode(lines) when is_list(lines), do: Enum.flat_map(lines, &encode_one/1)

  defp encode_one(%Range{first: first, last: last, step: 1}),
    do: [{:range, %{start: first, end: last}}]

  defp encode_one(%Range{} = range), do: Enum.map(range, &{:single, &1})
  defp encode_one(line) when is_integer(line), do: [{:single, line}]
end

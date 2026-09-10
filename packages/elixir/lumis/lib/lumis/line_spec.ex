defmodule Lumis.LineSpec do
  @moduledoc false

  # The one place a line number or `Range` becomes something Rust can read.
  #
  # A line range crosses as a `RangeInclusive<usize>`, which has no step and
  # cannot run backwards. Elixir's `Range` has both, and JavaScript's `[number,
  # number]` has neither, so the step is the one thing the shared representation
  # cannot carry.
  #
  # A step of 1 or -1 describes a contiguous run either way round, so both
  # normalize to one ascending range and cost nothing. Anything else has to
  # expand into single lines, and that is the case with a ceiling: `1..1_000_000
  # //2` is 500,000 tuples and 19MB, so `1..1_000_000_000//2` would be 19GB.
  # `Range.size/1` is arithmetic, so the size is known before anything is built.

  @typedoc false
  @type t :: pos_integer() | Range.t()

  @typedoc false
  @type encoded :: {:single, integer()} | {:range, %{start: integer(), end: integer()}}

  # Each spec measures ~40 bytes, so this is a few MB at worst. A stepped range
  # covering more lines than this is not a document anyone is highlighting.
  @max_expanded 100_000

  @doc false
  @spec encode([t()]) :: {:ok, [encoded()]} | {:error, String.t()}
  def encode(lines) when is_list(lines) do
    lines
    |> Enum.reduce_while({:ok, []}, fn line, {:ok, acc} ->
      case encode_one(line) do
        {:ok, specs} -> {:cont, {:ok, [specs | acc]}}
        {:error, message} -> {:halt, {:error, message}}
      end
    end)
    |> case do
      {:ok, acc} -> {:ok, acc |> Enum.reverse() |> Enum.concat()}
      {:error, message} -> {:error, message}
    end
  end

  @doc false
  @spec encode!([t()]) :: [encoded()]
  def encode!(lines) when is_list(lines) do
    case encode(lines) do
      {:ok, specs} -> specs
      {:error, message} -> raise ArgumentError, message
    end
  end

  # A contiguous run, either way round. `5..3//-1` covers the same lines as
  # `3..5`, and an empty range stays empty because Rust's `5..=3` matches
  # nothing either.
  defp encode_one(%Range{first: first, last: last, step: 1}),
    do: {:ok, [{:range, %{start: first, end: last}}]}

  defp encode_one(%Range{first: first, last: last, step: -1}),
    do: {:ok, [{:range, %{start: last, end: first}}]}

  defp encode_one(%Range{} = range) do
    size = Range.size(range)

    if size > @max_expanded do
      {:error,
       "a line range with a step of #{range.step} has to be expanded into single lines, " <>
         "and #{inspect(range)} covers #{size} of them, over the #{@max_expanded} limit. " <>
         "Use a range with a step of 1, which has no limit."}
    else
      {:ok, Enum.map(range, &{:single, &1})}
    end
  end

  defp encode_one(line) when is_integer(line), do: {:ok, [{:single, line}]}
end

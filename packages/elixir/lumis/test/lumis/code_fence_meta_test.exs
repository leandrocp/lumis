defmodule Lumis.CodeFenceMetaTest do
  @moduledoc """
  Elixir's half of the code fence meta parity check.

  `fixtures/code-fence-meta.json` holds one expected result per case.
  `crates/lumis-core/src/code_fence.rs` asserts Rust produces it; this asserts
  `Lumis.parse_code_fence_meta/1` returns the same, in Elixir's shapes.
  """
  use ExUnit.Case, async: true

  doctest Lumis, only: [parse_code_fence_meta: 1]

  @fixture_path Path.expand("../../../../../fixtures/code-fence-meta.json", __DIR__)
  @external_resource @fixture_path

  test "matches the shared fixture" do
    cases = @fixture_path |> File.read!() |> Jason.decode!() |> Map.fetch!("cases")
    assert length(cases) > 40

    for %{"name" => name, "meta" => meta, "expected" => expected} <- cases do
      assert Lumis.parse_code_fence_meta(meta) == expected(expected), name
    end
  end

  defp expected(fields) do
    %{
      theme: fields["theme"],
      pre_class: fields["preClass"],
      highlight_lines: lines(fields["highlightLines"]),
      highlight_lines_style: style(fields["highlightLinesStyle"]),
      highlight_lines_class: fields["highlightLinesClass"],
      include_highlights: fields["includeHighlights"],
      unrecognized:
        for attribute <- Map.get(fields, "unrecognized", []) do
          %{name: attribute["name"], value: attribute["value"]}
        end
    }
  end

  defp lines(nil), do: nil
  defp lines(pairs), do: for([first, last] <- pairs, do: first..last//1)

  defp style("theme"), do: :theme
  defp style(css), do: css
end

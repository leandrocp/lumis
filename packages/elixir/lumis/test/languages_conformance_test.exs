defmodule Lumis.LanguagesConformanceTest do
  @moduledoc """
  The Elixir half of the shared language-name conformance suite.

  `fixtures/conformance-languages/cases.json` states what loading and the CLI's
  download must agree on in every runtime. This asserts the Elixir implementation
  against it; `crates/lumis-wasm-runtime/tests/languages_conformance.rs` and
  `packages/javascript/lumis/test/languages-conformance.test.ts` assert theirs
  against the same file.
  """
  use ExUnit.Case, async: true

  @cases "../../../../fixtures/conformance-languages/cases.json"
         |> Path.expand(__DIR__)
         |> File.read!()
         |> Jason.decode!()

  defp expand(names), do: Lumis.Languages.expand_bundles(names)

  test "expands every spelling of a bundle the same way" do
    groups = get_in(@cases, ["spellings", "groups"])
    assert length(groups) >= 5, "the catalog has five bundles to cover"

    for [first | rest] <- groups do
      assert {:ok, expected} = expand([first])
      refute expected == [], "#{first} expanded to nothing"

      for spelling <- rest do
        assert {:ok, ^expected} = expand([spelling]), "#{spelling} disagrees with #{first}"
      end
    end
  end

  test "lets a name that is not a bundle survive expansion" do
    for name <- get_in(@cases, ["passthrough", "names"]) do
      assert {:ok, [^name]} = expand([name])
    end
  end

  test "keeps a bundle member named twice only once" do
    bundle = get_in(@cases, ["deduplication", "bundle"])
    member = get_in(@cases, ["deduplication", "alsoNamed"])

    assert {:ok, alone} = expand([bundle])
    assert member in alone

    assert {:ok, with_repeat} = expand([bundle, member, member])

    assert Enum.count(with_repeat, &(&1 == member)) == 1
    assert length(with_repeat) == length(alone)
  end

  test "keeps the order it was given" do
    input = get_in(@cases, ["ordering", "input"])
    first = get_in(@cases, ["ordering", "firstIs"])
    last = get_in(@cases, ["ordering", "lastIs"])

    assert {:ok, expanded} = expand(input)

    assert List.first(expanded) == first
    assert List.last(expanded) == last
  end

  test "rejects an unknown bundle rather than treating it as a language" do
    for name <- get_in(@cases, ["unknownBundles", "names"]) do
      assert {:error, {:unknown_bundle, ^name}} = expand([name]),
             "#{name} should be rejected as an unknown bundle, naming itself"
    end
  end
end

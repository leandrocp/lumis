defmodule Lumis.HighlightEventsTest do
  use ExUnit.Case, async: true

  alias Lumis.Decoration.RainbowBracket

  test "source events cover the source exactly, in order, inside balanced scopes" do
    sources = [
      "",
      "x = 1",
      "defmodule A do\n  @doc \"\"\"\n  héllo ☕\n  \"\"\"\n  def a, do: ~r/[a-z]+/\nend\n",
      "a = 1\r\nb = 2\r\n"
    ]

    for source <- sources do
      events = Lumis.highlight_events!(source, "elixir")

      covered =
        Enum.reduce(events, 0, fn
          {:source, %{start: start, end: stop}}, position ->
            assert start == position
            stop

          _event, position ->
            position
        end)

      assert covered == byte_size(source)
      assert balanced?(events)
    end
  end

  test "annotations arrive at their resolved byte range with their data untouched" do
    source = "π = 1\ncafé = 2"
    data = %{id: 7, change: {:added, [1, 2]}}

    events =
      Lumis.highlight_events!(source, "elixir",
        annotations: [[position: {{1, 0}, {1, 5}}, data: data]]
      )

    assert [%{range: {start, stop}, data: ^data}] =
             for({:annotation_start, annotation} <- events, do: annotation)

    assert binary_part(source, start, stop - start) == "café"
    assert Enum.count(events, &(&1 == :annotation_end)) == 1
  end

  test "rainbow brackets arrive as decorations whose depth does not wrap" do
    source = "[[[[[[[0]]]]]]]"

    decorated = Lumis.highlight_events!(source, "elixir", rainbow_brackets: true)
    depths = for {:decoration_start, %RainbowBracket{depth: depth}} <- decorated, do: depth

    assert depths |> Enum.uniq() |> Enum.sort() == Enum.to_list(0..6)

    refute Enum.any?(
             Lumis.highlight_events!(source, "elixir"),
             &match?({:decoration_start, _}, &1)
           )
  end

  test "a render over its time budget comes back as one plain source event" do
    source = String.duplicate("value = [1, %{a: :b}] |> Enum.map(&(&1 * 2))\n", 20_000)

    assert Lumis.highlight_events!(source, "elixir", budget: [time_limit: 1]) ==
             [{:source, %{start: 0, end: byte_size(source)}}]
  end

  test "nil detects the language from the source" do
    source = "#!/usr/bin/env bash\necho 1"

    assert Lumis.highlight_events!(source, nil) == Lumis.highlight_events!(source, "bash")
  end

  test "a language whose parser is not installed comes back as one plain source event" do
    assert {:error, :not_installed} = Lumis.Languages.load("go")

    source = "package main"

    assert Lumis.highlight_events!(source, "go") == [
             {:source, %{start: 0, end: byte_size(source)}}
           ]
  end

  test "an annotation that cannot be placed is an error" do
    options = [annotations: [[offset: {1, 2}, data: nil]]]

    assert {:error, %Lumis.RenderError{reason: :annotation}} =
             Lumis.highlight_events("π", "elixir", options)

    assert_raise Lumis.HighlightError, fn -> Lumis.highlight_events!("π", "elixir", options) end
  end

  test "invalid arguments raise" do
    assert_raise NimbleOptions.ValidationError, fn ->
      Lumis.highlight_events("x", "elixir", budget: [match_limit: 0])
    end

    assert_raise FunctionClauseError, fn -> Lumis.highlight_events("x", :elixir) end
  end

  defp balanced?(events) do
    depth =
      Enum.reduce_while(events, 0, fn
        {:start, _}, depth -> {:cont, depth + 1}
        :end, 0 -> {:halt, :unbalanced}
        :end, depth -> {:cont, depth - 1}
        _event, depth -> {:cont, depth}
      end)

    depth == 0
  end
end

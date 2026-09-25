defmodule Lumis.HighlightEventsTest do
  use ExUnit.Case, async: true

  alias Lumis.Formatter.HTML

  defmodule EventsFormatter do
    @behaviour Lumis.Formatter

    @impl true
    def render(_source, events, _options), do: :erlang.term_to_binary(events)
  end

  test "returns the events a custom formatter receives" do
    source = "items = [[1], {:ok, \"two\"}]"
    options = [annotations: [[offset: {0, 5}, data: %{id: 1}]], rainbow_brackets: true]

    {:ok, rendered} =
      Lumis.highlight(source, [formatter: {EventsFormatter, language: "elixir"}] ++ options)

    assert Lumis.highlight_events(source, "elixir", options) ==
             {:ok, :erlang.binary_to_term(rendered)}
  end

  test "detects the language when none is given" do
    events = Lumis.highlight_events!("#!/usr/bin/env bash\necho 1", nil)

    assert {:start, %{language: "bash"}} = Enum.find(events, &match?({:start, _}, &1))
  end

  test "renders one HTML fragment per source line" do
    source = "s = \"\"\"\none\n\"\"\"\nx = 1"
    attrs = Map.new(HTML.classes(), fn {scope, class} -> {scope, ~s|class="#{class}"|} end)

    lines =
      HTML.render_lines_from_events(source, Lumis.highlight_events!(source, "elixir"), attrs)

    assert length(lines) == 4
    assert Enum.at(lines, 1) =~ ~s(<span class="l-string">one</span>)
  end

  test "a language whose parser is not installed comes back as plain source" do
    {events, log} =
      ExUnit.CaptureLog.with_log(fn -> Lumis.highlight_events!("package main", "go") end)

    assert events == [{:source, %{start: 0, end: 12}}]
    assert log =~ "no parser for \"go\""
  end

  test "an annotation that cannot be composed is an error" do
    options = [annotations: [[offset: {1, 2}, data: %{}]]]

    assert {:error, %Lumis.RenderError{reason: :annotation}} =
             Lumis.highlight_events("π", "elixir", options)

    assert_raise Lumis.HighlightError, ~r/not a UTF-8 character boundary/, fn ->
      Lumis.highlight_events!("π", "elixir", options)
    end
  end

  test "invalid options raise" do
    assert_raise NimbleOptions.ValidationError, fn ->
      Lumis.highlight_events("x", "elixir", budget: [match_limit: 0])
    end
  end
end

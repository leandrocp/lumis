defmodule Lumis.AnnotationsTest do
  use ExUnit.Case, async: false

  import ExUnit.CaptureIO

  alias Lumis.Annotation
  alias Lumis.Position
  alias Lumis.Range.Offset
  alias Lumis.Range.Position, as: PositionRange

  defmodule TestFormatter do
    @behaviour Lumis.Formatter

    @impl true
    def render(source, events, _options) do
      Enum.map(events, fn
        {:start, %{scope: "punctuation.bracket.rainbow." <> _level}} ->
          send(self(), :saw_rainbow_bracket)
          []

        {:source, %{start: start, end: end_offset}} ->
          binary_part(source, start, end_offset - start)

        {:annotation_start, %Annotation{range: range, data: %{id: id}}} ->
          send(self(), {:resolved_range, id, range})
          ["<annotation:", Integer.to_string(id), ">"]

        :annotation_end ->
          "</annotation>"

        _syntax_event ->
          []
      end)
    end
  end

  test "annotations and their Elixir data reach a custom formatter" do
    source = "(price + tax)"

    annotations = [
      Annotation.new(
        Offset.new(1, byte_size(source) - 1),
        %{id: 7}
      )
    ]

    assert {:ok, output} =
             Lumis.highlight(source,
               formatter: {TestFormatter, language: "elixir"},
               annotations: annotations,
               rainbow_brackets: true
             )

    assert output == "(<annotation:7>price + tax</annotation>)"
    assert_received {:resolved_range, 7, %Offset{start: 1, end: 12}}
    assert_received :saw_rainbow_bracket
  end

  test "invalid UTF-8 byte boundaries are rejected against the source" do
    annotations = [
      Annotation.new(
        Offset.new(1, 2),
        %{}
      )
    ]

    assert_raise Lumis.HighlightError, ~r/not a UTF-8 character boundary/, fn ->
      Lumis.highlight!("π",
        formatter: {TestFormatter, language: "elixir"},
        annotations: annotations
      )
    end
  end

  test "position ranges use zero-based UTF-8 byte columns" do
    source = "π\ncafé"

    annotations = [
      Annotation.new(
        PositionRange.new(
          Position.new(1, 0),
          Position.new(1, 5)
        ),
        %{id: 8}
      )
    ]

    assert {:ok, output} =
             Lumis.highlight(source,
               formatter: {TestFormatter, language: "elixir"},
               annotations: annotations
             )

    assert output == "π\n<annotation:8>café</annotation>"
    assert_received {:resolved_range, 8, %Offset{start: 3, end: 8}}
  end

  test "position columns must be UTF-8 byte boundaries" do
    annotations = [
      Annotation.new(
        PositionRange.new(
          Position.new(1, 0),
          Position.new(1, 4)
        ),
        %{}
      )
    ]

    assert_raise Lumis.HighlightError, ~r/not a UTF-8 character boundary/, fn ->
      Lumis.highlight!("π\ncafé",
        formatter: {TestFormatter, language: "elixir"},
        annotations: annotations
      )
    end
  end

  test "range constructors reject reversed ranges" do
    assert_raise ArgumentError, ~r/offset range start must not be after its end/, fn ->
      Offset.new(6, 4)
    end

    assert_raise ArgumentError, ~r/position range start must not be after its end/, fn ->
      PositionRange.new(Position.new(2, 0), Position.new(1, 0))
    end
  end

  test "range constructors accept an empty range as a point" do
    assert %Offset{start: 4, end: 4} = Offset.new(4, 4)

    position = Position.new(1, 4)

    assert %PositionRange{start: ^position, end: ^position} =
             PositionRange.new(position, position)
  end

  test "a point annotation on a blank line reaches the formatter" do
    source = "a\n\nb"

    annotations = [Annotation.new(Offset.new(2, 2), %{id: 3})]

    assert {:ok, output} =
             Lumis.highlight(source,
               formatter: {TestFormatter, language: "elixir"},
               annotations: annotations
             )

    assert output == "a\n<annotation:3></annotation>\nb"
    assert_received {:resolved_range, 3, %Offset{start: 2, end: 2}}
  end

  test "the diff viewer example renders the complete annotation flow" do
    example = Path.expand("../examples/diff_viewer.exs", __DIR__)
    output = capture_io(fn -> Code.require_file(example) end)

    assert output =~ "calculator.ex · before"
    assert output =~ "calculator.ex · after"
    assert output =~ ~s(data-marker="-")
    assert output =~ ~s(data-marker="+")
    assert output =~ ~s(data-marker="~")
    assert output =~ "diff-span-removed"
    assert output =~ "diff-span-added"
    assert output =~ ~s(data-label="New service fee")
    assert output =~ ~s(<span class="l-)
  end
end

defmodule Lumis.AnnotationsTest do
  use ExUnit.Case, async: false

  import ExUnit.CaptureIO

  alias Lumis.Annotation

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
      [offset: {1, byte_size(source) - 1}, data: %{id: 7}]
    ]

    assert {:ok, output} =
             Lumis.highlight(source,
               formatter: {TestFormatter, language: "elixir"},
               annotations: annotations,
               rainbow_brackets: true
             )

    assert output == "(<annotation:7>price + tax</annotation>)"
    assert_received {:resolved_range, 7, {1, 12}}
    assert_received :saw_rainbow_bracket
  end

  test "invalid UTF-8 byte boundaries are rejected against the source" do
    annotations = [
      [offset: {1, 2}, data: %{}]
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
      [position: {{1, 0}, {1, 5}}, data: %{id: 8}]
    ]

    assert {:ok, output} =
             Lumis.highlight(source,
               formatter: {TestFormatter, language: "elixir"},
               annotations: annotations
             )

    assert output == "π\n<annotation:8>café</annotation>"
    assert_received {:resolved_range, 8, {3, 8}}
  end

  test "position columns must be UTF-8 byte boundaries" do
    annotations = [
      [position: {{1, 0}, {1, 4}}, data: %{}]
    ]

    assert_raise Lumis.HighlightError, ~r/not a UTF-8 character boundary/, fn ->
      Lumis.highlight!("π\ncafé",
        formatter: {TestFormatter, language: "elixir"},
        annotations: annotations
      )
    end
  end

  defp highlight_with(annotations) do
    Lumis.highlight("a\n\nb", formatter: :html_inline, annotations: annotations)
  end

  test "rejects a range that runs backwards" do
    assert_raise NimbleOptions.ValidationError,
                 ~r/annotation 0 offset range start must not be after its end: 6\.\.4/,
                 fn -> highlight_with([[offset: {6, 4}]]) end

    assert_raise NimbleOptions.ValidationError,
                 ~r/annotation 0 position range start must not be after its end/,
                 fn ->
                   highlight_with([
                     [position: {{2, 0}, {1, 0}}]
                   ])
                 end
  end

  test "accepts an empty range as a point" do
    assert {:ok, _html} = highlight_with([[offset: {1, 1}]])

    assert {:ok, _html} =
             highlight_with([[position: {{0, 1}, {0, 1}}]])
  end

  test "requires exactly one of :offset and :position" do
    assert_raise NimbleOptions.ValidationError,
                 ~r/annotation 0 needs an :offset or a :position range/,
                 fn -> highlight_with([[data: %{}]]) end

    assert_raise NimbleOptions.ValidationError,
                 ~r/annotation 0 sets both :offset and :position, which is ambiguous/,
                 fn ->
                   highlight_with([
                     [offset: {0, 1}, position: {{0, 0}, {0, 1}}]
                   ])
                 end
  end

  test "rejects an annotation that is not a keyword list" do
    assert_raise NimbleOptions.ValidationError,
                 ~r/annotation 0 must be a keyword list with :offset or :position/,
                 fn -> highlight_with([%{offset: {0, 1}}]) end
  end

  test "rejects an unknown key rather than ignoring it" do
    assert_raise NimbleOptions.ValidationError,
                 ~r/annotation 0: unknown options \[:colour\]/,
                 fn ->
                   highlight_with([[offset: {0, 1}, colour: :red]])
                 end
  end

  test "rejects a malformed position" do
    assert_raise NimbleOptions.ValidationError,
                 ~r/annotation 0: invalid tuple in :position option/,
                 fn ->
                   highlight_with([[position: {{0}, {0, 1}}]])
                 end
  end

  test "data defaults to nil when omitted" do
    defmodule DataFormatter do
      @behaviour Lumis.Formatter

      @impl true
      def render(_source, events, _options) do
        for {:annotation_start, annotation} <- events, do: inspect(annotation.data)
      end
    end

    assert {:ok, "nil"} =
             Lumis.highlight("ab",
               formatter: {DataFormatter, language: "elixir"},
               annotations: [[offset: {0, 1}]]
             )
  end

  test "a point annotation on a blank line reaches the formatter" do
    source = "a\n\nb"

    annotations = [[offset: {2, 2}, data: %{id: 3}]]

    assert {:ok, output} =
             Lumis.highlight(source,
               formatter: {TestFormatter, language: "elixir"},
               annotations: annotations
             )

    assert output == "a\n<annotation:3></annotation>\nb"
    assert_received {:resolved_range, 3, {2, 2}}
  end

  test "the example renders every annotation shape" do
    example = Path.expand("../examples/annotations.exs", __DIR__)
    output = capture_io(fn -> Code.require_file(example) end)

    # A position range crossing a line boundary.
    assert output =~ ~s(<span class="line-changed">)
    # An offset range starting mid-token splits the `variable` scope.
    assert output =~
             ~s(<span class="l-variable">p</span><mark class="edit"><span class="l-variable">rice</span>)

    # Byte offsets over a multibyte literal.
    assert output =~ "&quot;☕ café&quot;"
    # A point renders as an empty element.
    assert output =~ ~s(<i data-note="why the gap?"></i>)
    # The overlapping annotation is closed and reopened, so it starts twice.
    assert length(String.split(output, ~s(<mark class="right">))) - 1 == 2
  end
end

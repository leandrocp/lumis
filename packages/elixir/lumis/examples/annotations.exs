# Every shape an annotation can take, over one small source.
#
# Lumis does not compute the ranges. A diff library, a search index or a
# compiler supplies them; Lumis places them correctly relative to the syntax and
# hands them to this formatter.
#
# Run with `mix run examples/annotations.exs`.

defmodule MarkFormatter do
  @moduledoc false
  @behaviour Lumis.Formatter

  @impl true
  def render(source, events, _options) do
    # `:annotation_end` carries no payload, so keep a stack of what was opened.
    {output, []} =
      Enum.map_reduce(events, [], fn
        {:start, %{scope: scope}}, open ->
          {~s(<span class="#{scope_class(scope)}">), open}

        :end, open ->
          {"</span>", open}

        {:annotation_start, %{data: %{type: :line, kind: kind}}}, open ->
          {~s(<span class="line-#{kind}">), ["span" | open]}

        {:annotation_start, %{data: %{type: :span, name: name}}}, open ->
          {~s(<mark class="#{name}">), ["mark" | open]}

        # A point opens and closes with nothing between it.
        {:annotation_start, %{data: %{type: :note, label: label}}}, open ->
          {~s(<i data-note="#{escape(label)}">), ["i" | open]}

        :annotation_end, [tag | open] ->
          {"</#{tag}>", open}

        {:source, %{start: start, end: stop}}, open ->
          {escape(binary_part(source, start, stop - start)), open}
      end)

    output
  end

  defp scope_class(scope), do: "l-" <> String.replace(scope, ".", "-")

  # Lumis has no Elixir HTML helpers yet — see leandrocp/lumis#1359.
  defp escape(text) do
    text
    |> String.replace("&", "&amp;")
    |> String.replace("<", "&lt;")
    |> String.replace(">", "&gt;")
    |> String.replace("\"", "&quot;")
    |> String.replace("'", "&#39;")
  end
end

defmodule LumisAnnotationsExample do
  @moduledoc false

  @source ~s(let total = price + tax;\nlet label = "☕ café";\n\nlet net = total - fee;)

  def render_example do
    annotations = [
      # A position range, and one that crosses a line boundary. Zero-based
      # line, UTF-8 byte column, which is what a diff reports.
      [position: lines(0, 1), data: %{type: :line, kind: :changed}],
      # An offset range starting mid-token. Lumis closes and reopens the
      # `variable` scope around it, so `price` renders as `p` + `rice`.
      [offset: find("rice"), data: %{type: :span, name: :edit}],
      # Offsets are UTF-8 bytes, not characters. `☕` is 3 and `é` is 2, and
      # `find` counts them, so the whole literal is covered.
      [offset: find(~s("☕ café")), data: %{type: :span, name: :text}],
      # An empty range is a point. Line 2 is blank, so there is nothing to
      # cover, and a review comment still has somewhere to land.
      [
        position: {[line: 2, column: 0], [line: 2, column: 0]},
        data: %{type: :note, label: "why the gap?"}
      ],
      [position: lines(3, 3), data: %{type: :line, kind: :added}],
      # Two that overlap without either containing the other. Lumis closes
      # `left` and reopens `right` after it, so `right` opens twice.
      [offset: find("total - "), data: %{type: :span, name: :left}],
      [offset: find("- fee"), data: %{type: :span, name: :right}]
    ]

    {:ok, html} =
      Lumis.highlight(@source,
        formatter: {MarkFormatter, language: "javascript"},
        annotations: annotations
      )

    html
  end

  # Byte range of the first occurrence of `text`. A diff or search library would
  # report these; nothing here is hand-counted.
  defp find(text) do
    {start, length} = :binary.match(@source, text)
    {start, start + length}
  end

  # Lines `first` through `last`, as zero-based lines and UTF-8 byte columns.
  defp lines(first, last) do
    width =
      @source
      |> String.split("\n")
      |> Enum.at(last, "")
      |> byte_size()

    {[line: first, column: 0], [line: last, column: width]}
  end
end

LumisAnnotationsExample.render_example() |> IO.puts()

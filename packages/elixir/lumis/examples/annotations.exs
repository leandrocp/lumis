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

  alias Lumis.Formatter.HTML

  @impl true
  def render(source, events, _options) do
    # `:annotation_end` carries no payload, so keep a stack of what was opened.
    {output, []} =
      Enum.map_reduce(events, [], fn
        {:start, %{scope: scope}}, open ->
          {"<span #{HTML.span_linked_attrs(scope)}>", open}

        :end, open ->
          {"</span>", open}

        {:annotation_start, %{data: %{type: :line, kind: kind}}}, open ->
          {~s(<span class="line-#{kind}">), ["span" | open]}

        {:annotation_start, %{data: %{type: :span, name: name}}}, open ->
          {~s(<mark class="#{name}">), ["mark" | open]}

        # A point opens and closes with nothing between it.
        {:annotation_start, %{data: %{type: :note, label: label}}}, open ->
          {~s(<i data-note="#{HTML.escape(label)}">), ["i" | open]}

        :annotation_end, [tag | open] ->
          {"</#{tag}>", open}

        {:source, %{start: start, end: stop}}, open ->
          {HTML.escape(binary_part(source, start, stop - start)), open}

        # Lumis adds event kinds over time. Render the ones you know and skip
        # the rest, or a newer Lumis raises FunctionClauseError here.
        _event, open ->
          {[], open}
      end)

    output
  end
end

defmodule LumisAnnotationsExample do
  @moduledoc false

  @source ~s(let total = price + tax;\nlet label = "☕ café";\n\nlet net = total - fee;)

  def render_example do
    annotations = [
      # Zero-based line and UTF-8 byte column. This one crosses a line.
      [
        position: {{0, 0}, {1, 24}},
        data: %{type: :line, kind: :changed}
      ],
      # `rice`, inside `price`. Starting mid-token makes Lumis close and reopen
      # the `variable` scope, so it renders as `p` + `rice`.
      [offset: {13, 17}, data: %{type: :span, name: :edit}],
      # `"☕ café"`. Offsets are UTF-8 bytes, so `☕` costs 3 and `é` costs 2.
      [offset: {37, 48}, data: %{type: :span, name: :text}],
      # An empty range is a point. Line 2 is blank, so there is nothing to
      # cover, and a review comment still has somewhere to land.
      [
        position: {{2, 0}, {2, 0}},
        data: %{type: :note, label: "why the gap?"}
      ],
      [
        position: {{3, 0}, {3, 22}},
        data: %{type: :line, kind: :added}
      ],
      # `total - ` and `- fee` overlap without either containing the other, so
      # Lumis closes `left` and reopens `right` after it.
      [offset: {61, 69}, data: %{type: :span, name: :left}],
      [offset: {67, 72}, data: %{type: :span, name: :right}]
    ]

    {:ok, html} =
      Lumis.highlight(@source,
        formatter: {MarkFormatter, language: "javascript"},
        annotations: annotations
      )

    html
  end

end

LumisAnnotationsExample.render_example() |> IO.puts()

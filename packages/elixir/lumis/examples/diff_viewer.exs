defmodule DiffHtmlFormatter do
  @moduledoc false
  @behaviour Lumis.Formatter

  @line_markers %{context: " ", added: "+", removed: "-", changed: "~"}

  @impl true
  def render(source, events, _options) do
    {body, []} =
      Enum.map_reduce(events, [], fn
        {:start, %{scope: scope}}, annotation_closings ->
          {~s(<span class="#{scope_class(scope)}">), annotation_closings}

        :end, annotation_closings ->
          {"</span>", annotation_closings}

        {:source, %{start: start, end: end_offset}}, annotation_closings ->
          text = binary_part(source, start, end_offset - start)
          {escape(text), annotation_closings}

        {:annotation_start, %{data: %{type: :line} = line}}, annotation_closings ->
          opening =
            ~s(<span class="diff-line diff-line-#{line.kind}" data-line="#{line.number}" data-marker="#{@line_markers[line.kind]}">)

          {opening, ["</span>" | annotation_closings]}

        {:annotation_start, %{data: %{type: :span} = span}}, annotation_closings ->
          {~s(<mark class="diff-span diff-span-#{span.kind}">), ["</mark>" | annotation_closings]}

        {:annotation_start, %{data: %{type: :annotation} = annotation}},
        annotation_closings ->
          opening =
            ~s(<span class="diff-annotation" data-label="#{escape(annotation.label)}">)

          {opening, ["</span>" | annotation_closings]}

        :annotation_end, [closing | annotation_closings] ->
          {closing, annotation_closings}
      end)

    [
      ~s(<pre class="diff-code"><code data-lang="elixir">),
      body,
      "</code></pre>"
    ]
  end

  # Rust and JavaScript formatters call `lumis::formatters::html` and
  # `@lumis-sh/lumis/formatters/html` for these two. Elixir has no equivalent
  # yet, so they are hand-rolled here and have to track the Rust versions by
  # hand — see leandrocp/lumis#1359.
  defp scope_class(scope), do: "l-" <> String.replace(scope, ".", "-")

  defp escape(text) do
    text
    |> String.replace("&", "&amp;")
    |> String.replace("<", "&lt;")
    |> String.replace(">", "&gt;")
    |> String.replace("\"", "&quot;")
    |> String.replace("'", "&#39;")
  end
end

defmodule LumisDiffViewerExample do
  @moduledoc false

  def render_example do
    old_source = """
    def calculate(price, tax) do
      price + tax
    end
    """

    new_source = """
    def calculate(price, tax, fee) do
      subtotal = price + tax
      subtotal + fee
    end
    """

    # A diff library would determine these ranges. Lumis only consumes them.
    old_annotations = [
      annotation(old_source, "def calculate(price, tax) do", %{
        type: :line,
        number: 1,
        kind: :changed
      }),
      annotation(old_source, "  price + tax", %{
        type: :line,
        number: 2,
        kind: :removed
      }),
      annotation(old_source, "end", %{
        type: :line,
        number: 3,
        kind: :context
      }),
      annotation(old_source, "tax", %{type: :span, kind: :removed}),
      annotation(old_source, "price + tax", %{
        type: :span,
        kind: :removed
      })
    ]

    new_annotations = [
      annotation(new_source, "def calculate(price, tax, fee) do", %{
        type: :line,
        number: 1,
        kind: :changed
      }),
      annotation(new_source, "  subtotal = price + tax", %{
        type: :line,
        number: 2,
        kind: :added
      }),
      annotation(new_source, "  subtotal + fee", %{
        type: :line,
        number: 3,
        kind: :added
      }),
      annotation(new_source, "end", %{
        type: :line,
        number: 4,
        kind: :context
      }),
      annotation(new_source, "fee", %{type: :span, kind: :added}),
      annotation(new_source, "  subtotal = price + tax", %{
        type: :span,
        kind: :added
      }),
      annotation(new_source, "  subtotal + fee", %{
        type: :span,
        kind: :added
      }),
      annotation(
        new_source,
        "fee",
        %{type: :annotation, label: "New service fee"},
        :last
      )
    ]

    old_html = render_pane(old_source, old_annotations)
    new_html = render_pane(new_source, new_annotations)

    [page_start(), old_html, page_middle(), new_html, page_end()]
  end

  defp render_pane(source, annotations) do
    {:ok, html} =
      Lumis.highlight(source,
        formatter: {DiffHtmlFormatter, language: "elixir"},
        annotations: annotations
      )

    html
  end

  defp annotation(source, text, data, occurrence \\ :first) do
    matches = :binary.matches(source, text)

    {start, length} =
      case occurrence do
        :first -> List.first(matches)
        :last -> List.last(matches)
      end

    [offset: {start, start + length}, data: data]
  end

  defp page_start do
    """
    <!doctype html>
    <html lang="en">
    <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Lumis Elixir annotation diff viewer</title>
    <style>
    :root { color-scheme: dark; font-family: ui-sans-serif, system-ui, sans-serif; }
    body { margin: 0; background: #0d1117; color: #e6edf3; }
    main { padding: 2rem; }
    h1 { margin: 0 0 .35rem; font-size: 1.35rem; }
    .subtitle { margin: 0 0 1.25rem; color: #8b949e; }
    .diff-viewer { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); border: 1px solid #30363d; border-radius: 10px; overflow: hidden; }
    .diff-pane + .diff-pane { border-left: 1px solid #30363d; }
    .diff-pane h2 { margin: 0; padding: .7rem 1rem; background: #161b22; border-bottom: 1px solid #30363d; font-size: .85rem; font-weight: 600; }
    .diff-code { margin: 0; padding: 0; overflow-x: auto; background: #0d1117; }
    .diff-code code { display: block; min-width: max-content; padding: .5rem 0; }
    .diff-line { display: inline-block; min-width: 100%; min-height: 1.45em; }
    .diff-line::before { display: inline-block; width: 4.5rem; margin-right: .75rem; padding-right: .7rem; color: #6e7681; text-align: right; content: attr(data-line) " " attr(data-marker); user-select: none; }
    .diff-line-added { background: rgba(46, 160, 67, .18); }
    .diff-line-removed { background: rgba(248, 81, 73, .18); }
    .diff-line-changed { background: rgba(187, 128, 9, .18); }
    .diff-line-added::before { color: #56d364; background: rgba(46, 160, 67, .16); }
    .diff-line-removed::before { color: #ff7b72; background: rgba(248, 81, 73, .16); }
    .diff-line-changed::before { color: #e3b341; background: rgba(187, 128, 9, .16); }
    .diff-span { border-radius: 3px; color: inherit; }
    .diff-span-added { background: rgba(46, 160, 67, .42); }
    .diff-span-removed { background: rgba(248, 81, 73, .42); }
    .diff-annotation { position: relative; border-bottom: 2px dotted #d2a8ff; }
    .diff-annotation::after { position: absolute; z-index: 1; left: 0; bottom: 1.5rem; width: max-content; max-width: 12rem; padding: .3rem .45rem; border: 1px solid #8957e5; border-radius: 5px; background: #2d1b4e; color: #d2a8ff; font: 11px/1.2 ui-sans-serif, system-ui, sans-serif; content: "● " attr(data-label); }
    .l-keyword, .l-keyword-function { color: #ff7b72; }
    .l-function, .l-function-method { color: #d2a8ff; }
    .l-variable, .l-variable-parameter { color: #ffa657; }
    .l-operator, .l-punctuation-bracket, .l-punctuation-delimiter { color: #8b949e; }
    @media (max-width: 850px) {
      main { padding: 1rem; }
      .diff-viewer { grid-template-columns: 1fr; }
      .diff-pane + .diff-pane { border-left: 0; border-top: 1px solid #30363d; }
    }
    </style>
    </head>
    <body>
    <main>
    <h1>Annotation API: Elixir diff viewer</h1>
    <p class="subtitle">Caller-supplied annotations composed with Lumis syntax events</p>
    <div class="diff-viewer">
    <section class="diff-pane">
    <h2>calculator.ex · before</h2>
    """
  end

  defp page_middle do
    """

    </section>
    <section class="diff-pane">
    <h2>calculator.ex · after</h2>
    """
  end

  defp page_end do
    """

    </section>
    </div>
    </main>
    </body>
    </html>
    """
  end
end

IO.write(LumisDiffViewerExample.render_example())

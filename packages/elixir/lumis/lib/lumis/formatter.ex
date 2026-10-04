defmodule Lumis.Formatter do
  @moduledoc """
  Behaviour for custom formatters.

  Lumis syntax-highlights the source and composes caller-provided annotations
  before calling `render/3`. The formatter receives one properly nested,
  sequential event stream.

  Lumis adds event kinds as it grows, so a formatter should match the ones it
  renders and skip the rest:

      _event, acc -> {[], acc}

  Without that clause a newer Lumis raises `FunctionClauseError` rather than
  rendering. The built-in formatters do the same thing with annotations, which
  they cannot render without knowing the caller's data.

  `lines_from_events/2` splits the stream into lines of tokens, for output built
  line by line in any format. `Lumis.Formatter.HTML` and `Lumis.Formatter.ANSI`
  hold the pieces specific to one format.

  ## Options

  `render/3` receives the options the formatter was given, plus `:language`,
  which is always the language highlighting **actually used**. A caller who
  named none gets the detected one rather than `nil`, so a formatter can label
  its output without running detection a second time:

      def render(source, events, options) do
        Lumis.Formatter.HTML.open_code_tag(Keyword.fetch!(options, :language))
      end
  """

  @typedoc "A syntax, caller-provided annotation, or Lumis decoration event."
  @type event(data) ::
          {:start, %{scope: String.t(), language: String.t()}}
          | {:source, %{start: non_neg_integer(), end: non_neg_integer()}}
          | :end
          | {:annotation_start, %{range: {non_neg_integer(), non_neg_integer()}, data: data}}
          | :annotation_end
          | {:decoration_start, Lumis.Decoration.RainbowBracket.t()}
          | :decoration_end

  @typedoc """
  A run of one line's text under one scope, from `lines_from_events/2`.

  `range` is where `text` sits in the source, in bytes. `scope` is the innermost
  open scope, or `""` outside every scope and for a scope Lumis does not name.
  """
  @type token :: %{
          text: String.t(),
          range: {non_neg_integer(), non_neg_integer()},
          scope: String.t(),
          language: String.t()
        }

  @typedoc "One line from `lines_from_events/2`."
  @type line(data) :: %{number: pos_integer(), tokens: [token()], annotations: [data]}

  @doc """
  The event stream split into lines of tokens, with the annotations each line
  touches.

  These are the lines `Lumis.Formatter.HTML.render_lines_from_events/3` renders,
  as data, for output that is not an HTML string: content only, a final newline adds no line, and an
  empty source is one empty line. A scope that crosses a newline gives one token
  on each line, and a token's `range` leaves the terminator out.

  A rainbow bracket reports `punctuation.bracket.rainbow.N`. A rainbow bracket,
  and text outside every scope, report the language of the stream's first scope,
  or `"plaintext"` when it has none.

  Nothing here resolves a style. Build a `Lumis.Formatter.ANSI.styles/1` table
  per language and read each token with `Lumis.Formatter.ANSI.style_for/2`; the
  table carries the parent-scope fallbacks that `Map.get(theme.highlights, scope)`
  misses.

  A line lists the data of every annotation covering any part of it, once each,
  in the order they open. A point annotation lands on the line holding it,
  including a blank one, and a point at the very end of a source that ends in a
  newline lands on the last line.

  ## Example

      iex> events = [{:start, %{scope: "comment", language: "rust"}}, {:source, %{start: 0, end: 4}}, :end]
      iex> Lumis.Formatter.lines_from_events("a\\r\\nb", events)
      [
        %{number: 1, tokens: [%{text: "a", range: {0, 1}, scope: "comment", language: "rust"}], annotations: []},
        %{number: 2, tokens: [%{text: "b", range: {3, 4}, scope: "comment", language: "rust"}], annotations: []}
      ]

  """
  @spec lines_from_events(String.t(), [event(data)]) :: [line(data)]
        when data: term()
  def lines_from_events(source, events) when is_binary(source) and is_list(events) do
    Lumis.Native.formatter_lines_from_events(source, events)
  end

  @doc "Renders a unified event stream for `source`."
  @callback render(
              source :: String.t(),
              events :: [event(term())],
              options :: keyword()
            ) :: iodata()
end

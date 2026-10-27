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

  ## Options

  `render/3` receives the options the formatter was given, plus `:language`,
  which is always the language highlighting **actually used**. A caller who
  named none gets the detected one rather than `nil`, so a formatter can label
  its output without running detection a second time:

      def render(source, events, options) do
        Lumis.Formatter.HTML.open_code_tag(Keyword.fetch!(options, :language))
      end

  It also receives `:budget_exhausted`, which names the limit that bound the
  highlight: `nil` when none did, `:time` when the time limit ran out and the
  events are the whole source as plain text, or `:matches` when the match limit
  dropped some scopes. The built-in HTML formatters put it in
  `data-lumis-budget` on the `<pre>`; a formatter with nowhere to put it can
  ignore it.

  ## Failure

  `render/3` has no error return. An exception it raises propagates out of
  `Lumis.highlight/2` and `Lumis.highlight!/2` unchanged, rather than becoming
  `{:error, %Lumis.RenderError{}}`, so the stack trace still points at the
  formatter.
  """

  @typedoc "A syntax, caller-provided annotation, or Lumis decoration event."
  @type event(data) ::
          {:start, %{scope: String.t(), language: String.t()}}
          | {:source, %{start: non_neg_integer(), end: non_neg_integer()}}
          | :end
          | {:annotation_start, %{range: {non_neg_integer(), non_neg_integer()}, data: data}}
          | :annotation_end
          | {:decoration_start, Lumis.Decoration.RainbowBracket.t()}
          | {:decoration_start, Lumis.Decoration.DiffWord.t()}
          | :decoration_end

  @doc "Renders a unified event stream for `source`."
  @callback render(
              source :: String.t(),
              events :: [event(term())],
              options :: keyword()
            ) :: iodata()
end

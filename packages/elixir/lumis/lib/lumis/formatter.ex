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
  """

  @typedoc "A syntax or caller-provided annotation event."
  @type event(data) ::
          {:start, %{scope: String.t(), language: String.t()}}
          | {:source, %{start: non_neg_integer(), end: non_neg_integer()}}
          | :end
          | {:annotation_start, Lumis.Annotation.t(data)}
          | :annotation_end

  @doc "Renders a unified event stream for `source`."
  @callback render(
              source :: String.t(),
              events :: [event(term())],
              options :: keyword()
            ) :: iodata()
end

defmodule Lumis.Annotation do
  @moduledoc """
  A caller-provided semantic range, as a formatter receives it.

  Annotations are supplied to `Lumis.highlight/2` as keyword lists:

      annotations: [
        [offset: {12, 23}, data: %{change: :added}],
        [position: {[line: 1, column: 10], [line: 1, column: 21]}, data: %{change: :removed}]
      ]

  `:offset` is a half-open range of absolute UTF-8 byte offsets. `:position` is
  a half-open range of zero-based lines and UTF-8 byte columns. Both resolve to
  byte offsets before a formatter sees them, so this struct always carries
  `:start` and `:end` in bytes whichever form built it.

  `:data` stays an Elixir term and is passed through untouched.

      {:annotation_start, %Lumis.Annotation{start: 12, end: 23, data: %{change: :added}}}

  This struct is built by Lumis, not by callers.
  """

  @enforce_keys [:start, :end, :data]
  defstruct [:start, :end, :data]

  @typedoc "A resolved annotation, its range measured in UTF-8 bytes."
  @type t(data) :: %__MODULE__{
          start: non_neg_integer(),
          end: non_neg_integer(),
          data: data
        }

  @type t :: t(term())
end

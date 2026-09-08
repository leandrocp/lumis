defmodule Lumis.Annotation do
  @moduledoc """
  A caller-provided semantic range, as a formatter receives it.

  Annotations are supplied to `Lumis.highlight/2` as keyword lists:

      annotations: [
        [offset: {12, 23}, data: %{change: :added}],
        [position: {{1, 10}, {1, 21}}, data: %{change: :removed}]
      ]

  `:offset` is a half-open range of absolute UTF-8 byte offsets, as
  `{start, end}`. `:position` is a half-open range of zero-based lines and
  UTF-8 byte columns, as `{{line, column}, {line, column}}`. Both resolve to
  byte offsets before a formatter sees them, so `:range` is always
  `{start, end}` in bytes whichever form built it.

  `:data` stays an Elixir term and is passed through untouched.

      {:annotation_start, %Lumis.Annotation{range: {12, 23}, data: %{change: :added}}}

  This struct is built by Lumis, not by callers.
  """

  @enforce_keys [:range, :data]
  defstruct [:range, :data]

  @typedoc "A resolved annotation, its range measured in UTF-8 bytes."
  @type t(data) :: %__MODULE__{
          range: {non_neg_integer(), non_neg_integer()},
          data: data
        }

  @type t :: t(term())
end

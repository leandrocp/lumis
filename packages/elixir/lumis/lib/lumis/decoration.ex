defmodule Lumis.Decoration.RainbowBracket do
  @moduledoc """
  A matched bracket carrying its real zero-based nesting depth.

  Built-in formatters cycle this depth through six theme scopes. Custom
  formatters receive the unwrapped value between `:decoration_start` and
  `:decoration_end` events.
  """

  @enforce_keys [:depth]
  defstruct [:depth]

  @type t :: %__MODULE__{depth: non_neg_integer()}
end

defmodule Lumis.HighlightError do
  defexception [:error]

  @type t() :: %__MODULE__{error: Exception.t()}

  def message(%__MODULE__{error: error}) do
    """
    error highlighting source code

    got:

      #{Exception.message(error)}
    """
  end
end

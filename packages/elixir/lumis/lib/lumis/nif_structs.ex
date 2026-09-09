defmodule Lumis.HTMLElement do
  @moduledoc false
  defstruct open_tag: nil, close_tag: nil
end

defmodule Lumis.HTMLInlineHighlightLines do
  @moduledoc false
  defstruct lines: [], style: :theme, class: nil
end

defmodule Lumis.HTMLLinkedHighlightLines do
  @moduledoc false
  defstruct lines: [], class: "l-highlighted"
end

defmodule Lumis.HtmlElement do
  @moduledoc false
  defstruct open_tag: nil, close_tag: nil
end

defmodule Lumis.HtmlInlineHighlightLines do
  @moduledoc false
  defstruct lines: [], style: :theme, class: nil
end

defmodule Lumis.HtmlLinkedHighlightLines do
  @moduledoc false
  defstruct lines: [], class: "highlighted"
end

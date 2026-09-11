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

defmodule Lumis.TerminalHighlightLines do
  @moduledoc false
  defstruct lines: [], background: nil
end

defmodule Lumis.BBCodeHighlightLines do
  @moduledoc false
  defstruct lines: []
end

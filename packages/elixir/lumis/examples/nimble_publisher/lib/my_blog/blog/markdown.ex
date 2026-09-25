defmodule MyBlog.Blog.Markdown do
  @moduledoc """
  Converts the example blog's Markdown posts to highlighted HTML.
  """

  def convert(_path, body, _attrs, _opts) do
    MDEx.to_html!(body,
      extension: [
        table: true,
        strikethrough: true,
        tasklist: true
      ],
      syntax_highlight: [
        engine: :lumis,
        opts: [formatter: {:html_inline, theme: "github_light"}]
      ]
    )
  end
end

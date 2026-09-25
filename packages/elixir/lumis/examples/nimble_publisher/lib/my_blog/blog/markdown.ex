defmodule MyBlog.Blog.Markdown do
  @moduledoc """
  Converts the example blog's Markdown posts to highlighted HTML.
  """

  def convert(_path, body, _attrs, _opts) do
    # NimblePublisher converts posts while MyBlog.Blog compiles, and `mix
    # compile` starts no applications. This brings up `:mdex_native`, whose
    # `start/2` hands its NIF the `lumis_wasm_*` directories to load parsers
    # from; without it every fence still renders through the formatter but with
    # no parser behind it — Lumis markup around plain, uncolored code.
    {:ok, _} = Application.ensure_all_started(:mdex)

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

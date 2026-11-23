html =
  Lumis.highlight!("defmodule Smoke do\n  def ok, do: :ok\nend\n",
    formatter: {:html_inline, language: "elixir", include_highlights: true}
  )

if not String.contains?(html, ~s(data-highlight="keyword)) do
  raise "Lumis did not highlight Elixir: #{html}"
end

markdown =
  MDExNative.Comrak.markdown_to_html!("```elixir\ndefmodule Smoke do\nend\n```\n",
    syntax_highlight: [
      engine: :lumis,
      opts: [formatter: {:html_inline, include_highlights: true}]
    ]
  )

if not String.contains?(markdown, ~s(data-highlight="keyword)) do
  raise "MDExNative did not highlight Elixir: #{markdown}"
end

IO.puts("smoke ok")

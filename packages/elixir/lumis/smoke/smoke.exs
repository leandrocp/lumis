html =
  Lumis.highlight!("defmodule Smoke do\n  def ok, do: :ok\nend\n",
    formatter: {:html_inline, language: "elixir"}
  )

if not String.contains?(html, "Smoke") or length(Regex.scan(~r/<span\b/, html)) < 3 do
  raise "unexpected Lumis output: #{html}"
end

markdown =
  MDExNative.Comrak.markdown_to_html!("```elixir\n:ok\n```\n",
    syntax_highlight: [engine: :lumis, opts: [formatter: {:html_inline, theme: "github_light"}]]
  )

if not String.contains?(markdown, "<span"), do: raise("unexpected MDExNative output: #{markdown}")

IO.puts("smoke ok")

# Installs this package's precompiled NIF next to mdex_native's Lumis build, as
# an MDEx app does (#2114), and checks both highlight. Release CI runs it with
# RUSTLER_PRECOMPILED_GLOBAL_CACHE_PATH pointing at the NIFs it just built.
Mix.install(
  [
    {:lumis, path: Path.expand("..", __DIR__)},
    {:lumis_wasm_elixir, "~> 0.26.0"},
    {:mdex_native, ">= 0.3.2"}
  ],
  config: [mdex_native: [syntax_highlighter: :lumis]]
)

html =
  Lumis.highlight!("defmodule Smoke do\nend\n",
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

defmodule LumisThemes.MixProject do
  use Mix.Project

  @version "0.1.0"

  def project do
    [
      app: :lumis_themes,
      version: @version,
      elixir: "~> 1.15",
      description: "Permissively licensed Neovim themes for Lumis highlighters.",
      package: [
        licenses: ["MIT", "Apache-2.0", "BSD-3-Clause", "CC0-1.0", "ISC"],
        links: %{"GitHub" => "https://github.com/leandrocp/lumis"},
        files: ~w(lib priv mix.exs)
      ],
      deps: []
    ]
  end
end

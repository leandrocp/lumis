defmodule LumisThemesCopyleft.MixProject do
  use Mix.Project

  @version "0.1.0"

  def project do
    [
      app: :lumis_themes_copyleft,
      version: @version,
      elixir: "~> 1.15",
      description: "Optional copyleft and Vim-licensed Neovim themes for Lumis highlighters.",
      package: [
        licenses: ["GPL-2.0", "GPL-3.0", "LGPL-3.0", "Apache-2.0", "Vim"],
        links: %{"GitHub" => "https://github.com/leandrocp/lumis"},
        files: ~w(priv mix.exs)
      ],
      deps: []
    ]
  end
end

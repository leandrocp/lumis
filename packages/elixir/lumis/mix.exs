defmodule Lumis.MixProject do
  use Mix.Project

  @source_url "https://github.com/leandrocp/lumis"
  @version "0.1.0"

  def project do
    [
      app: :lumis,
      version: @version,
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      build_embedded: Mix.env() == :prod,
      package: package(),
      docs: docs(),
      deps: deps(),
      aliases: aliases(),
      name: "Lumis",
      homepage_url: "https://lumis.sh",
      description: "Syntax highlighter powered by Tree-sitter and Neovim themes."
    ]
  end

  def application do
    [
      extra_applications: [:logger]
    ]
  end

  def cli do
    [
      preferred_envs: [
        docs: :docs,
        "hex.publish": :docs,
        "test.rust": :test,
        "test.all": :test,
        quality: :test
      ]
    ]
  end

  defp package do
    [
      maintainers: ["Leandro Pereira"],
      licenses: ["MIT"],
      links: %{
        Changelog: "https://hexdocs.pm/lumis/changelog.html",
        GitHub: @source_url,
        Site: "https://lumis.sh"
      },
      files: ~w[
        lib
        native/lumis_nif/src
        native/lumis_nif/.cargo
        native/lumis_nif/Cargo.*
        native/lumis_nif/Cross.toml
        priv/static/css
        examples
        checksum-*.exs
        mix.exs
        README.md
        LICENSE.md
        CHANGELOG.md
        usage-rules.md
      ]
    ]
  end

  defp docs do
    [
      main: "Lumis",
      source_ref: "v#{@version}",
      source_url: @source_url,
      source_url_pattern: "#{@source_url}/blob/v#{@version}/packages/elixir/lumis/%{path}#L%{line}",
      extras: [
        "CHANGELOG.md",
        "examples/light_dark_manual.livemd",
        "examples/light_dark_vars.livemd",
        "examples/light_dark_function.livemd"
      ],
      skip_undefined_reference_warnings_on: ["CHANGELOG.md"]
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.29", optional: true},
      {:rustler_precompiled, "~> 0.6"},
      {:nimble_options, "~> 1.0"},
      {:ex_doc, ">= 0.0.0", only: :docs},
      {:makeup_elixir, ">= 0.0.0", only: :docs},
      {:makeup_eex, ">= 0.0.0", only: :docs},
      {:makeup_syntect, ">= 0.0.0", only: :docs}
    ]
  end

  defp aliases do
    [
      setup: ["deps.get", "compile"],
      quality: ["format.all", "lint.rust", "test.all"],
      "gen.checksum": "rustler_precompiled.download Lumis.Native --all --print",
      "format.all": ["format.rust", "format"],
      "test.all": ["test.rust", "test"],
      "format.rust": ["cmd cargo fmt --manifest-path=native/lumis_nif/Cargo.toml --all"],
      "lint.rust": [
        "cmd cargo clippy --manifest-path=native/lumis_nif/Cargo.toml -- -Dwarnings"
      ],
      "test.rust": ["cmd cargo test --manifest-path=native/lumis_nif/Cargo.toml"]
    ]
  end
end

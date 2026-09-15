defmodule Lumis.MixProject do
  use Mix.Project

  @source_url "https://github.com/leandrocp/lumis"
  @version "0.8.0"

  def project do
    [
      app: :lumis,
      version: @version,
      elixir: "~> 1.15",
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
      extra_applications: [:logger],
      mod: {Lumis.Application, []}
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
        guides
        checksum-*.exs
        mix.exs
        README.md
        LICENSE
        CHANGELOG.md
        usage-rules.md
      ]
    ]
  end

  defp docs do
    [
      main: "Lumis",
      source_ref: "hex-lumis/v#{@version}",
      source_url: @source_url,
      source_url_pattern:
        "#{@source_url}/blob/hex-lumis/v#{@version}/packages/elixir/lumis/%{path}#L%{line}",
      extras: [
        "guides/deployment.md",
        "CHANGELOG.md",
        "examples/bbcode_scoped.livemd",
        "examples/html_linked_scoped_css.livemd",
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
      {:rustler_precompiled, "~> 0.8"},
      {:nimble_options, "~> 1.0"},
      {:ex_doc, ">= 0.0.0", only: :docs},
      {:makeup_elixir, ">= 0.0.0", only: :docs},
      {:makeup_eex, ">= 0.0.0", only: :docs},
      {:makeup_syntect, ">= 0.0.0", only: :docs},
      {:credo, "~> 1.7", only: [:dev, :test], runtime: false}
    ]
  end

  defp aliases do
    [
      setup: ["deps.get", "compile"],
      # The suite stages committed fixtures because it has no published parsers
      # to fetch. Once it can download them, the tests can use the registry path.
      test: ["stage.test_parsers", "test"],
      "stage.test_parsers": [
        "cmd --cd ../../.. cargo run -q --manifest-path crates/dev/Cargo.toml -- " <>
          "stage-test-parsers target/test-parsers"
      ],
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

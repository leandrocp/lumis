defmodule LumisSmoke.MixProject do
  use Mix.Project

  def project do
    [app: :lumis_smoke, version: "0.1.0", elixir: "~> 1.15", deps: deps()]
  end

  def application, do: [extra_applications: [:logger]]

  defp deps do
    [
      {:lumis, path: ".."},
      {:lumis_wasm_elixir, "~> 0.26.0"},
      # Another NIF in the same VM, as in an MDEx app: glibc shares one static
      # TLS reserve between them (#2114).
      {:mdex_native, ">= 0.3.2"}
    ]
  end
end

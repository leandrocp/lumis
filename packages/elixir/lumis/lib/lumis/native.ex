# https://github.com/elixir-explorer/explorer/blob/d11216282bbdb0dcaef2519c2bfefda46c2981e0/lib/explorer/polars_backend/native.ex

defmodule Lumis.Native do
  @moduledoc false

  mix_config = Mix.Project.config()
  version = mix_config[:version]
  github_url = mix_config[:package][:links][:GitHub]
  mode = if Mix.env() in [:dev, :test], do: :debug, else: :release

  use_legacy =
    Application.compile_env(
      :lumis,
      :use_legacy_artifacts,
      System.get_env("LUMIS_USE_LEGACY_ARTIFACTS") in ["true", "1"]
    )

  variants_for_linux = [
    legacy_cpu: fn ->
      # These are the same from the release workflow.
      # See the meaning in: https://unix.stackexchange.com/a/43540
      needed_caps = ~w[fxsr sse sse2 ssse3 sse4_1 sse4_2 popcnt avx fma]

      use_legacy or
        (is_nil(use_legacy) and
           not Lumis.ComptimeUtils.cpu_with_all_caps?(needed_caps))
    end
  ]

  other_variants = [legacy_cpu: fn -> use_legacy end]

  use RustlerPrecompiled,
    otp_app: :lumis,
    crate: "lumis_nif",
    version: version,
    base_url: "#{github_url}/releases/download/v#{version}",
    targets: ~w(
      aarch64-apple-darwin
      aarch64-unknown-linux-gnu
      aarch64-unknown-linux-musl
      arm-unknown-linux-gnueabihf
      riscv64gc-unknown-linux-gnu
      x86_64-apple-darwin
      x86_64-pc-windows-gnu
      x86_64-pc-windows-msvc
      x86_64-unknown-freebsd
      x86_64-unknown-linux-gnu
      x86_64-unknown-linux-musl
    ),
    variants: %{
      "x86_64-unknown-linux-gnu" => variants_for_linux,
      "x86_64-pc-windows-msvc" => other_variants,
      "x86_64-pc-windows-gnu" => other_variants,
      "x86_64-unknown-freebsd" => other_variants
    },
    # We don't use any features of newer NIF versions, so 2.15 is enough.
    nif_versions: ["2.15"],
    mode: mode,
    force_build: System.get_env("LUMIS_BUILD") in ["1", "true"]

  def available_languages(), do: :erlang.nif_error(:nif_not_loaded)
  def available_themes(), do: :erlang.nif_error(:nif_not_loaded)
  def get_theme(_name), do: :erlang.nif_error(:nif_not_loaded)
  def build_theme_from_file(_path), do: :erlang.nif_error(:nif_not_loaded)
  def build_theme_from_json_string(_json_string), do: :erlang.nif_error(:nif_not_loaded)
  def highlight(_source, _options), do: :erlang.nif_error(:nif_not_loaded)
end

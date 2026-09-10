# https://github.com/elixir-explorer/explorer/blob/d11216282bbdb0dcaef2519c2bfefda46c2981e0/lib/explorer/polars_backend/native.ex

defmodule Lumis.Native do
  @moduledoc false

  mix_config = Mix.Project.config()
  version = mix_config[:version]
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
    base_url: {Lumis.Native.ArtifactURL, :url},
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

  def available_languages, do: :erlang.nif_error(:nif_not_loaded)
  def language_info(_name), do: :erlang.nif_error(:nif_not_loaded)
  def available_themes, do: :erlang.nif_error(:nif_not_loaded)
  def get_theme(_name), do: :erlang.nif_error(:nif_not_loaded)
  def build_theme_from_file(_path), do: :erlang.nif_error(:nif_not_loaded)
  def build_theme_from_json_string(_json_string), do: :erlang.nif_error(:nif_not_loaded)
  def theme_css_from_name(_name, _options), do: :erlang.nif_error(:nif_not_loaded)
  def theme_css_from_theme(_theme, _options), do: :erlang.nif_error(:nif_not_loaded)
  def configure_store(_data_dir), do: :erlang.nif_error(:nif_not_loaded)
  def language_package_refs, do: :erlang.nif_error(:nif_not_loaded)
  def language_bundles, do: :erlang.nif_error(:nif_not_loaded)
  def load_language_by_name(_name), do: :erlang.nif_error(:nif_not_loaded)

  def cache_languages(_names, _force), do: :erlang.nif_error(:nif_not_loaded)
  def precompile_languages(_names), do: :erlang.nif_error(:nif_not_loaded)

  def guess_language(_name, _source), do: :erlang.nif_error(:nif_not_loaded)
  def has_language(_name), do: :erlang.nif_error(:nif_not_loaded)
  def loaded_languages, do: :erlang.nif_error(:nif_not_loaded)
  def highlight(_source, _options), do: :erlang.nif_error(:nif_not_loaded)
  def highlight_events(_source, _options), do: :erlang.nif_error(:nif_not_loaded)

  def ansi_hex_to_rgb(_hex), do: :erlang.nif_error(:nif_not_loaded)
  def ansi_rgb_to_ansi(_r, _g, _b, _is_background), do: :erlang.nif_error(:nif_not_loaded)
  def ansi_style_to_ansi(_style), do: :erlang.nif_error(:nif_not_loaded)
  def ansi_paint(_text, _style), do: :erlang.nif_error(:nif_not_loaded)
  def ansi_reset, do: :erlang.nif_error(:nif_not_loaded)
  def ansi_styles(_theme, _language), do: :erlang.nif_error(:nif_not_loaded)

  def html_escape(_text), do: :erlang.nif_error(:nif_not_loaded)
  def html_escape_braces(_text), do: :erlang.nif_error(:nif_not_loaded)
  def html_classes, do: :erlang.nif_error(:nif_not_loaded)

  def html_span_attrs(_theme, _language, _italic, _include_highlights),
    do: :erlang.nif_error(:nif_not_loaded)

  def html_open_pre_tag(_pre_class, _theme), do: :erlang.nif_error(:nif_not_loaded)
  def html_open_code_tag(_language), do: :erlang.nif_error(:nif_not_loaded)
  def html_closing_tags, do: :erlang.nif_error(:nif_not_loaded)

  def html_wrap_line(_line_number, _content, _class_suffix, _style),
    do: :erlang.nif_error(:nif_not_loaded)
end

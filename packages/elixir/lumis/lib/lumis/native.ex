# https://github.com/elixir-explorer/explorer/blob/d11216282bbdb0dcaef2519c2bfefda46c2981e0/lib/explorer/polars_backend/native.ex

defmodule Lumis.Native do
  @moduledoc false

  mix_config = Mix.Project.config()
  version = mix_config[:version]
  mode = if Mix.env() in [:dev, :test], do: :debug, else: :release
  force_build = System.get_env("LUMIS_BUILD") in ["1", "true"]

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

  workspace_root = Path.expand("../../../../..", __DIR__)
  workspace_crates_path = Path.join(workspace_root, "crates")

  if force_build and File.dir?(workspace_crates_path) do
    # Rustler ignores crates resolved locally through the workspace's [patch.crates-io].
    patched_crate_globs =
      Enum.map(~w[lumis-core lumis-wasm-runtime], fn crate ->
        Path.join([workspace_crates_path, crate, "**/*"])
      end)

    patched_crate_resources =
      patched_crate_globs
      |> Enum.flat_map(&Path.wildcard/1)
      |> Enum.filter(&File.regular?/1)

    # lumis_nif is a workspace member, so the root manifest and lock resolve this
    # build, not the ones beside it that Rustler tracks and a published build uses.
    # `Path.wildcard` skips dot directories, so Rustler misses `.cargo/config.toml`,
    # which carries the CFLAGS and rustflags the artifact is compiled with.
    build_input_resources = [
      Path.join(workspace_root, "Cargo.toml"),
      Path.join(workspace_root, "Cargo.lock"),
      Path.expand("../../native/lumis_nif/.cargo/config.toml", __DIR__)
    ]

    @patched_crate_globs patched_crate_globs
    @patched_crate_resources_hash :erlang.md5(patched_crate_resources)

    for resource <- patched_crate_resources ++ build_input_resources do
      @external_resource resource
    end

    @doc false
    def __mix_recompile__? do
      resources =
        @patched_crate_globs
        |> Enum.flat_map(&Path.wildcard/1)
        |> Enum.filter(&File.regular?/1)

      :erlang.md5(resources) != @patched_crate_resources_hash
    end
  end

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
    force_build: force_build

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
  def html_escape_attr(_value), do: :erlang.nif_error(:nif_not_loaded)
  def html_escape_braces(_text), do: :erlang.nif_error(:nif_not_loaded)
  def html_classes, do: :erlang.nif_error(:nif_not_loaded)
  def html_sanitize_theme_name(_name), do: :erlang.nif_error(:nif_not_loaded)
  def html_text_decoration(_text_decoration), do: :erlang.nif_error(:nif_not_loaded)
  def html_style_to_css(_style, _italic, _separator), do: :erlang.nif_error(:nif_not_loaded)

  def html_span_attrs(_theme, _language, _italic, _include_highlights),
    do: :erlang.nif_error(:nif_not_loaded)

  def html_multi_themes_span_attrs(
        _themes,
        _default_theme,
        _css_variable_prefix,
        _language,
        _italic,
        _include_highlights
      ),
      do: :erlang.nif_error(:nif_not_loaded)

  def html_pre_attrs(_pre_class, _theme, _attrs), do: :erlang.nif_error(:nif_not_loaded)
  def html_open_pre_tag(_pre_class, _theme, _attrs), do: :erlang.nif_error(:nif_not_loaded)

  def html_multi_themes_pre_attrs(
        _pre_class,
        _themes,
        _default_theme,
        _css_variable_prefix,
        _attrs
      ),
      do: :erlang.nif_error(:nif_not_loaded)

  def html_open_multi_themes_pre_tag(
        _pre_class,
        _themes,
        _default_theme,
        _css_variable_prefix,
        _attrs
      ),
      do: :erlang.nif_error(:nif_not_loaded)

  def html_code_attrs(_language, _attrs), do: :erlang.nif_error(:nif_not_loaded)
  def html_open_code_tag(_language, _attrs), do: :erlang.nif_error(:nif_not_loaded)
  def html_open_tag_from_attrs(_name, _attrs), do: :erlang.nif_error(:nif_not_loaded)
  def html_valid_attr_name(_name), do: :erlang.nif_error(:nif_not_loaded)
  def html_close_pre_tag, do: :erlang.nif_error(:nif_not_loaded)
  def html_close_code_tag, do: :erlang.nif_error(:nif_not_loaded)
  def html_closing_tags, do: :erlang.nif_error(:nif_not_loaded)

  def html_wrap_line(_line_number, _content, _class_suffix, _style),
    do: :erlang.nif_error(:nif_not_loaded)

  def html_line_is_highlighted(_lines, _line_number), do: :erlang.nif_error(:nif_not_loaded)

  def html_highlight_line_class(_lines, _line_number, _class, _default_class),
    do: :erlang.nif_error(:nif_not_loaded)

  def html_render_lines_from_events(_source, _events, _attrs),
    do: :erlang.nif_error(:nif_not_loaded)
end

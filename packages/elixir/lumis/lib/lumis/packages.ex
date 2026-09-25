defmodule Lumis.Packages do
  @moduledoc false

  # Parsers this project depends on, found the way any dependency's assets are.
  #
  # Internal. What users touch is their `deps` in `mix.exs`.
  #
  # A parser is an ordinary OTP application whose `priv/parsers` holds a
  # `lumis.json` and the WASM it describes — the same layout the store uses, so
  # nothing has to translate between "installed" and "downloaded". Depending on
  # one is how a project declares it may load that language, which is the rule
  # JavaScript follows for `@lumis-sh/wasm-*` in `package.json` and Rust follows
  # for Cargo features. Each runtime expresses it in the package manager it
  # already has.

  @prefix "lumis_wasm_"

  @doc false
  # `priv/parsers` of every installed parser application.
  #
  # An empty list is a real answer, not a missing one: this project depends on
  # no parsers, so it may load none. Nothing is fetched to make up the
  # difference, which is the same thing depending on no `@lumis-sh/wasm-*`
  # package means in JavaScript and compiling no language features means in
  # Rust.
  @spec installed_dirs() :: [Path.t()]
  def installed_dirs do
    (configured() ++ Enum.flat_map(roots(), &parser_dir/1))
    |> Enum.uniq()
    |> Enum.sort()
  end

  # `config :lumis, :parser_dirs` names directories directly, for a project that
  # vendors parsers rather than depending on them — an air-gapped build that
  # ships the bytes it already has, say. Same layout, same verification; only
  # how the directory got there differs.
  defp configured do
    :lumis
    |> Application.get_env(:parser_dirs, [])
    |> Enum.map(&Path.expand/1)
  end

  @doc false
  # The parser applications this project depends on, by directory name.
  #
  # Read off the code path rather than `Application.loaded_applications/0`. A
  # parser application has no supervision tree and nothing depends on it at the
  # OTP level, so a release never loads it — the bytes are there in `lib/` and
  # the application is invisible. The code path lists it either way.
  #
  # In a release the directory carries the version (`lumis_wasm_elixir-0.26.3`),
  # under Mix it does not; matching the prefix covers both, and the name is only
  # used for reporting.
  @spec applications() :: [String.t()]
  def applications do
    roots() |> Enum.map(&Path.basename/1) |> Enum.sort()
  end

  defp roots do
    for path <- :code.get_path(),
        dir = to_string(path),
        Path.basename(dir) == "ebin",
        root = Path.dirname(dir),
        String.starts_with?(Path.basename(root), @prefix),
        uniq: true,
        do: root
  end

  @doc false
  @spec prefix() :: String.t()
  def prefix, do: @prefix

  @doc false
  # The Hex package supplying the catalog language package with this suffix.
  #
  # The catalog names packages the way npm does, since that is where parsers are
  # published from, and the NIF hands over only the part after
  # `@lumis-sh/wasm-`. Composing the rest here keeps `lumis_wasm_` in one place
  # and keeps the npm spelling in `store::package_suffix`, so neither side has
  # to know how the other names the same bytes.
  @spec hex_name(String.t() | nil) :: String.t() | nil
  def hex_name(nil), do: nil
  def hex_name(suffix), do: @prefix <> String.replace(suffix, "-", "_")

  @doc false
  # The Mix requirement for a parser this build can load, for the dependency
  # `Lumis.ParserError` tells someone to add.
  #
  # `~> 0.26.0`, never `~> 0.26`. The catalog's range is `0.26`, which semver
  # reads as `>= 0.26.0 and < 0.27.0` — the bound the store actually enforces —
  # while Mix reads `~> 0.26` as `>= 0.26.0 and < 1.0.0`. The looser one lets a
  # 0.27 parser install and then be refused at load, which is a runtime error
  # standing in for a resolver one.
  #
  # That bound is not a guess about where a breaking change lands. The series is
  # generated from the `tree-sitter` pin in `mise.toml`, so `0.26` means parsers
  # built against tree-sitter 0.26 and a 0.27 series would be built against an
  # ABI this NIF is not linked to. Refusing is the only safe answer, and the
  # refusal surfaces as `:not_installed`: the package resolved and is sitting in
  # `deps`, so an out-of-range one reads as absent rather than as the wrong
  # version. Getting the requirement right is what keeps that from happening.
  #
  # Built from the lowest accepted version rather than by appending `.0` to the
  # range, so it stays correct if the range stops being a bare `MAJOR.MINOR`.
  @spec requirement() :: String.t()
  def requirement, do: "~> " <> Lumis.Native.lowest_compatible_package_version()

  # A parser application with no `priv/parsers` is not an error worth stopping a
  # boot over: it contributes nothing, and the language it was supposed to carry
  # reports itself when something asks for it.
  defp parser_dir(root) do
    dir = Path.join([root, "priv", "parsers"])
    if File.dir?(dir), do: [dir], else: []
  end
end

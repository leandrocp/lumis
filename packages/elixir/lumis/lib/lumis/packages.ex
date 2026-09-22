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
  # Rust. A language you did not add is a language you do not have.
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

  # A parser application with no `priv/parsers` is not an error worth stopping a
  # boot over: it contributes nothing, and the language it was supposed to carry
  # reports itself when something asks for it.
  defp parser_dir(root) do
    dir = Path.join([root, "priv", "parsers"])
    if File.dir?(dir), do: [dir], else: []
  end
end

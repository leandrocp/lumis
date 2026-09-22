defmodule Lumis.Languages do
  @moduledoc """
  Downloads, caches and loads Tree-sitter parsers.

  A language is a parser WASM plus the queries that drive highlighting, released
  together as a `@lumis-sh/wasm-*` package with its size and SHA-256. Nothing
  here needs calling for normal use: `Lumis.highlight/2` fetches and loads what a
  document turns out to need, including languages injected inside it.

  Reach for it in two situations.

  Load ahead of the first request, so a download does not land on a user. From
  an application's `start/2`, use `async_load/1`, which returns before the
  network work rather than holding up the boot:

      Lumis.Languages.async_load(["elixir", "html"])
      :ok = Lumis.Languages.load(["elixir", "html"])

  Or prepare a directory a *different* process will read, validating parsers and
  persisting their compiled modules without retaining any language in memory:

      {:ok, _paths} = Lumis.Languages.download(["elixir", "html"])

  ## Where parsers come from

  Each is taken from `$LUMIS_DATA_DIR/parsers` if it is there, and the CDN
  otherwise. Bytes are checked against the size and digest their package
  declares before use, and anything that fails is discarded rather than trusted,
  so a corrupted store repairs itself.

  Concurrent requests for the same uncached language wait for the first rather
  than each downloading it, and loading is global to the VM: the process that
  pays for a language pays once, for every process after it.
  """

  require Logger

  alias Lumis.Native

  @plaintext_names ~w(plaintext text txt plain)

  @typedoc "A language set shared with the `@lumis-sh/wasm-bundle-*` packages."
  @type bundle() ::
          :bundle_web | :bundle_web_extra | :bundle_system | :bundle_backend | :bundle_full

  @doc """
  Loads one language, a list of them, or a bundle, verifying each parser first.

  Highlighting loads on demand, so this is an optimization rather than a
  requirement: call it at startup to move the download off the first request.
  Already-loaded languages return immediately.

      :ok = Lumis.Languages.load("elixir")
      :ok = Lumis.Languages.load(["elixir", :html])
      :ok = Lumis.Languages.load(:bundle_web)

  A `:bundle_*` atom names the same set of languages as the `@lumis-sh/wasm-bundle-*`
  package of that name, so every runtime means the same thing by it. `:bundle_full`
  is every language in the catalog, which is rarely what a deployment wants; prefer
  a narrower bundle, or name the languages a document can contain.

  ## Failures

  A list loads every name in it and reports the ones that failed, rather than
  stopping at the first. One unpublished parser in a bundle should not cost the
  rest, the same way one bad block does not cost a document.

      {:error, %{"css" => :failed_to_load_parser}} = Lumis.Languages.load(["elixir", "css"])

    * `:unknown_language` — the name is not in the catalog
    * `:failed_to_load_parser` — it is, but its parser could not be obtained or verified
    * `:unknown_bundle` — no bundle by that name

  """
  @type failure() :: :unknown_language | :failed_to_load_parser
  @spec load(bundle() | String.t() | atom() | [String.t() | atom()]) ::
          :ok | {:error, :unknown_bundle | %{String.t() => failure()}}
  def load(names) when is_list(names) do
    failures =
      Enum.reduce(names, %{}, fn name, failures ->
        case load(name) do
          :ok -> failures
          # A bundle named inside a list reports its own members, not itself.
          {:error, nested} when is_map(nested) -> Map.merge(failures, nested)
          {:error, reason} -> Map.put(failures, to_string(name), reason)
        end
      end)

    if failures == %{}, do: :ok, else: {:error, failures}
  end

  def load(name) when is_atom(name) do
    case bundle_members(name) do
      {:ok, members} -> load(members)
      :error -> {:error, :unknown_bundle}
      :not_a_bundle -> load(Atom.to_string(name))
    end
  end

  def load(name) when is_binary(name) and name in @plaintext_names, do: :ok

  def load(name) when is_binary(name) do
    case bundle_members(name) do
      {:ok, members} -> load(members)
      :error -> {:error, :unknown_bundle}
      :not_a_bundle -> Native.load_language_by_name(name)
    end
  end

  @doc """
  Runs `load/1` in the background and returns immediately.

  Written for `start/2`, where the alternative is holding the whole boot on a
  download. Highlighting loads on demand anyway, so an application that starts
  before its parsers are warm serves correctly the entire time; it only pays for
  a language on the first request that names one the warm-up has not reached.

      def start(_type, _args) do
        Lumis.Languages.async_load(~w(markdown elixir javascript))
        Supervisor.start_link(children(), strategy: :one_for_one, name: MyApp.Supervisor)
      end

  Failures are logged rather than returned, because nothing is waiting on them.
  Nothing this function does can stop an application from booting: the work runs
  under a `:temporary` child of Lumis's own supervisor, so it is never retried
  and never escalates. Use `load/1` when the caller does need the result.

  Returns `{:ok, pid}`; the docs above ignore it deliberately, since matching on
  it is how a warm-up ends up able to break a boot after all.
  """
  @spec async_load(bundle() | String.t() | atom() | [String.t() | atom()]) ::
          DynamicSupervisor.on_start_child()
  def async_load(names) do
    Task.Supervisor.start_child(Lumis.TaskSupervisor, fn ->
      case load(names) do
        :ok ->
          :ok

        {:error, reason} ->
          Logger.warning(
            "Lumis could not warm #{inspect(names)}: #{inspect(reason)}. " <>
              "Highlighting will load these on demand."
          )
      end
    end)
  end

  @bundle_prefixes ["bundle_", "bundle-"]

  @doc false
  # Compares normalized strings rather than `String.to_atom/1` on the caller's
  # name: atoms are never garbage collected, so a name reaching this from a
  # request would grow the atom table without bound.
  def bundle_members(name) do
    case name |> to_string() |> strip_bundle_prefix() do
      nil -> :not_a_bundle
      suffix -> find_bundle(normalize_bundle(suffix))
    end
  end

  defp find_bundle(wanted) do
    Enum.find_value(bundles(), :error, fn {bundle, members} ->
      if bundle_key(bundle) == wanted, do: {:ok, members}
    end)
  end

  defp bundle_key(bundle) do
    bundle |> Atom.to_string() |> strip_bundle_prefix() |> normalize_bundle()
  end

  defp strip_bundle_prefix(string) do
    downcased = String.downcase(string)

    Enum.find_value(@bundle_prefixes, fn prefix ->
      case String.split(downcased, prefix, parts: 2) do
        ["", suffix] -> suffix
        _ -> nil
      end
    end)
  end

  defp normalize_bundle(nil), do: nil
  defp normalize_bundle(suffix), do: suffix |> String.downcase() |> String.replace("-", "_")

  @doc false
  def expand_bundles(names) do
    names
    |> Enum.reduce_while({:ok, []}, fn name, {:ok, acc} ->
      case bundle_members(name) do
        {:ok, members} -> {:cont, {:ok, Enum.reverse(members, acc)}}
        :error -> {:halt, {:error, {:unknown_bundle, to_string(name)}}}
        :not_a_bundle -> {:cont, {:ok, [to_string(name) | acc]}}
      end
    end)
    |> case do
      {:ok, reversed} -> {:ok, reversed |> Enum.reverse() |> Enum.uniq()}
      error -> error
    end
  end

  @doc """
  Resolves a name, path or source to a language id, the way highlighting does.

  `name` can be a language id, an alias, a file name or a path. When it does not
  resolve, `source` is checked for an Emacs mode header, a shebang, an HTML
  doctype or an XML declaration. Falls back to `"plaintext"`.

  The counterpart of `Language::guess` in Rust and `guessLanguage()` in
  JavaScript. `Lumis.highlight/2` already calls this when no language is given;
  reach for it directly to label a snippet before deciding what to do with it.

      "elixir" = Lumis.Languages.guess("lib/app.ex")
      "bash" = Lumis.Languages.guess(nil, "#!/usr/bin/env bash")
      "plaintext" = Lumis.Languages.guess(nil, "")

  """
  @spec guess(String.t() | atom() | nil, String.t()) :: String.t()
  def guess(name, source \\ "")

  def guess(nil, source) when is_binary(source), do: Native.guess_language(nil, source)

  def guess(name, source) when is_binary(source) do
    Native.guess_language(to_string(name), source)
  end

  @doc """
  Looks one language up by id or alias, or returns `default`.

  The same record `Lumis.available_languages/0` returns one of, without scanning
  the catalog, and resolving aliases the way highlighting does: `"js"` finds
  JavaScript.

      iex> Lumis.Languages.get("js").id
      "javascript"

      iex> Lumis.Languages.get("not-a-language")
      nil

  """
  @spec get(String.t() | atom(), any()) :: Lumis.language_info() | any()
  def get(name, default \\ nil)

  def get(name, default) when is_atom(name) and not is_nil(name),
    do: get(Atom.to_string(name), default)

  def get(name, default) when is_binary(name) do
    case Native.language_info(name) do
      nil -> default
      language -> language
    end
  end

  @doc """
  The languages each `:bundle_*` name covers.

  These are the same sets the `@lumis-sh/wasm-bundle-*` packages ship, so naming
  a bundle means the same thing in every runtime.
  """
  @spec bundles() :: %{bundle() => [String.t()]}
  def bundles do
    Map.new(Native.language_bundles(), fn {name, members} ->
      {String.to_atom("bundle_" <> String.replace(name, "-", "_")), members}
    end)
  end

  @doc """
  Downloads, validates and compiles languages without loading them permanently.

  Takes the same names `load/1` does, including `:bundle_*`. Returns the paths
  written. Already verified parser bytes and compiled modules are reused on
  later calls.

  This fills a directory for a process that has not started yet, so it validates
  and compiles without keeping anything: a release step preparing a volume, or a
  task run before the VM that will serve. To warm the VM you are already in, use
  `async_load/1` or `load/1`, which do the same work and keep the result, so no
  request reloads what this would have discarded.

  Names are downloaded and compiled concurrently. Every one is attempted: a
  bundle reports each language it could not prepare rather than stopping at the
  first, the same way `load/1` does.

      {:ok, paths} = Lumis.Languages.download(["elixir", "html"])
      {:error, %{"css" => reason}} = Lumis.Languages.download(["elixir", "css"])

  ## Options

    * `:force` — resolve the compatible package range again and replace a
      verified parser
  """
  @spec download([String.t() | atom()], keyword()) ::
          {:ok, [String.t()]}
          | {:error, String.t() | {:unknown_bundle, String.t()} | %{String.t() => String.t()}}
  def download(names, options \\ []) when is_list(names) and is_list(options) do
    force? = Keyword.get(options, :force, false)

    with {:ok, expanded} <- expand_bundles(names) do
      names = Enum.reject(expanded, &(&1 in @plaintext_names))

      with {:ok, paths} <- do_cache(names, force?),
           {:ok, _compiled} <- do_precompile(names) do
        {:ok, paths}
      end
    end
  end

  @doc """
  Former name of `download/2`. Still works, and warns at compile time.
  """
  @deprecated "Use Lumis.Languages.download/2 instead"
  @spec cache([String.t() | atom()], keyword()) ::
          {:ok, [String.t()]}
          | {:error, String.t() | {:unknown_bundle, String.t()} | %{String.t() => String.t()}}
  def cache(names, options \\ []), do: download(names, options)

  defp do_cache([], _force?), do: {:ok, []}

  defp do_cache(names, force?) do
    names
    |> Native.cache_languages(force?)
    |> collect(names, fn {:ok, path}, _name -> path end)
    |> case do
      # A few languages share one grammar, so this is shorter than `names`.
      {:ok, paths} ->
        {:ok, Enum.uniq(paths)}

      {:error, failures} when length(names) == 1 ->
        {:error, Map.fetch!(failures, hd(names))}

      error ->
        error
    end
  end

  defp do_precompile([]), do: {:ok, []}

  defp do_precompile(names) do
    names
    |> Native.precompile_languages()
    |> collect(names, fn :ok, name -> name end)
  end

  # The batch NIFs answer positionally, one result per name, so a failure is
  # named by zipping rather than by threading the name through the batch.
  # `Enum.zip/1` would quietly drop the tail if the two ever disagreed, and a
  # language silently missing from a prepared image is the bug that costs most.
  defp collect(results, names, success) do
    if length(results) != length(names) do
      raise "expected one result per language, got #{length(results)} for #{length(names)}"
    end

    {values, failures} =
      [names, results]
      |> Enum.zip()
      |> Enum.reduce({[], %{}}, fn {name, result}, {values, failures} ->
        case result do
          {:error, reason} -> {values, Map.put(failures, name, reason)}
          result -> {[success.(result, name) | values], failures}
        end
      end)

    if failures == %{}, do: {:ok, Enum.reverse(values)}, else: {:error, failures}
  end
end

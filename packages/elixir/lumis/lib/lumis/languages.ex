defmodule Lumis.Languages do
  @moduledoc """
  Loads Tree-sitter parsers this project depends on.

  A language is a parser WASM plus the queries that drive highlighting, shipped
  together as a `lumis_wasm_*` dependency with its size and SHA-256. Nothing
  here needs calling for normal use: `Lumis.highlight/2` loads what a document
  turns out to need, including languages injected inside it — as long as the
  project depends on them. One it does not answers `:not_installed`.

  Reach for it to load ahead of the first request, so the compile does not land
  on a user. From an application's `start/2`, use `async_load/1`, which returns
  without holding up the boot:

      Lumis.Languages.async_load(["elixir", "html"])
      :ok = Lumis.Languages.load(["elixir", "html"])

  ## Where parsers come from

  From the `priv/parsers` of each installed `lumis_wasm_*` dependency, or a
  directory named by `config :lumis, :parser_dirs`. A language no dependency
  supplies is not fetched — it answers `:not_installed`.

  Bytes are checked against the size and digest their manifest declares before
  use, and anything that fails is discarded rather than trusted.

  Concurrent requests for the same unloaded language wait for the first rather
  than each compiling it, and loading is global to the VM: the process that pays
  for a language pays once, for every process after it.
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
  requirement: call it at startup to move the compile off the first request.
  Already-loaded languages return immediately.

      :ok = Lumis.Languages.load("elixir")
      :ok = Lumis.Languages.load(["elixir", :html])
      :ok = Lumis.Languages.load(:bundle_web)

  A `:bundle_*` atom names the same set of languages as the `lumis_wasm_bundle_*`
  package of that name, so every runtime means the same thing by it. `:bundle_full`
  is every language in the catalog, which is rarely what a deployment wants; prefer
  a narrower bundle, or name the languages a document can contain.

  Loading only ever reaches parsers this project depends on, because a parser is
  an ordinary dependency:

      # mix.exs
      {:lumis_wasm_elixir, "~> 0.26.0"}

  ## Failures

  A list loads every name in it and reports the ones that failed, rather than
  stopping at the first. One unpublished parser in a bundle should not cost the
  rest, the same way one bad block does not cost a document.

      {:error, %{"css" => :failed_to_load_parser}} = Lumis.Languages.load(["elixir", "css"])

    * `:unknown_language` — the name is not in the catalog
    * `:not_installed` — it is, but this project does not depend on its parser
    * `:failed_to_load_parser` — the parser could not be read or verified
    * `:unknown_bundle` — no bundle by that name
    * `%Lumis.ParserError{reason: :store_full}` — the parser is fine, and this
      process has no room for another one

  The last is the exception `Lumis.highlight/2` answers with, rather than an
  atom, because it is the one failure here that is not about the language named:
  it loads on its own, retrying it cannot help, and `Exception.message/1` says
  what does. See `Lumis.ParserError` for the reason itself.

  A single name answers with the reason itself; a list answers with a map from
  name to reason, so one failure never hides the others.

  """
  @type failure() ::
          :unknown_language
          | :not_installed
          | :failed_to_load_parser
          | Lumis.ParserError.t()
  @spec load(bundle() | String.t() | atom() | [String.t() | atom()]) ::
          :ok
          | {:error, :unknown_bundle | failure()}
          | {:error, %{String.t() => :unknown_bundle | failure()}}
  def load(names) when is_list(names) do
    failures =
      Enum.reduce(names, %{}, fn name, failures ->
        collect_failure(failures, name, load(name))
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
      :not_a_bundle -> describe_load(Native.load_language_by_name(name))
    end
  end

  # Every other reason the NIF gives is already an atom. A full store arrives as
  # the fields of a `Lumis.ParserError`, the same way a failed highlight's do.
  defp describe_load({:error, {:parser, fields}}),
    do: {:error, Lumis.ParserError.from_nif(fields)}

  defp describe_load(other), do: other

  @doc false
  # Files one name's answer into the failures `load/1` has collected so far.
  # Named rather than inlined so the clause order can be tested: reaching the
  # map clause with a struct needs a Wasm store that is actually full, which
  # costs a hundred parser compiles to arrange.
  def collect_failure(failures, _name, :ok), do: failures

  # A bundle named inside a list reports its own members, not itself.
  # `not is_struct` because an exception is a map too: merging a
  # `Lumis.ParserError` would scatter its fields across the result instead of
  # filing it under the language that failed, and spelling the guard this way
  # covers every struct reason added later.
  def collect_failure(failures, _name, {:error, nested})
      when is_map(nested) and not is_struct(nested),
      do: Map.merge(failures, nested)

  def collect_failure(failures, name, {:error, reason}),
    do: Map.put(failures, to_string(name), reason)

  @doc """
  Runs `load/1` in the background and returns immediately.

  Written for `start/2`, where the alternative is holding the whole boot on a
  compile. Highlighting loads on demand anyway, so an application that starts
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
          # Deliberately not "highlighting will load these on demand": a parser
          # this project never added is not going to arrive later, so saying so
          # would send someone looking for a slow first request instead of a
          # missing dependency.
          Logger.warning(
            "Lumis could not warm #{inspect(names)}: #{inspect(reason)}. " <>
              "A :not_installed language needs its parser added to mix.exs; " <>
              "a :store_full one means this VM already holds every parser it " <>
              "can and cannot free any, so it takes a smaller language set at " <>
              "the next boot rather than a shorter warm-up here; " <>
              "anything else is retried when a document asks for it."
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
end

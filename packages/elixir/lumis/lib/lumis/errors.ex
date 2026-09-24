defmodule Lumis.ParserError do
  @moduledoc """
  A language's parser could not be loaded.

  `Lumis.highlight/2` returns this when the root language of a document is
  unavailable. An *injected* language is not an error — that block stays plain
  and the rest of the document still highlights.

  Match on `:reason` rather than on the message:

      case Lumis.highlight(source, formatter: {:html_inline, language: "elixir"}) do
        {:ok, html} ->
          html

        {:error, %Lumis.ParserError{reason: :not_installed, package: package}} ->
          Logger.error("add {:\#{package}, \\"~> 0.26\\"} to mix.exs")
          Plug.HTML.html_escape(source)

        {:error, error} ->
          Logger.error(Exception.message(error))
          Plug.HTML.html_escape(source)
      end

  ## Reasons

    * `:not_installed` — the parser is not a dependency of this project. The
      only reason a deployment normally sees, and the only one adding a
      dependency fixes.
    * `:parser_missing` — it *is* a dependency, but the `.wasm` its manifest
      names is not beside it. Fetching dependencies again is the fix; adding
      the dependency is not, since it is already there.
    * `:unknown_language` — the name is not in the catalog
    * `:not_loaded` — the language is known but no store could supply it
    * `:not_cached` — its bytes are not on disk and nothing fetched them
    * `:incompatible_version` — the installed parser package is outside the
      range this build of Lumis supports; `:detail` names both
    * `:download_failed`, `:unavailable` — it is neither installed nor
      reachable over the network
    * `:invalid_package`, `:invalid_package_name`, `:package_name_mismatch` —
      the package could not be read, or is not the one that was asked for
    * `:invalid_parser`, `:invalid_queries` — the parser WASM or its queries
      were rejected
    * `:io` — a read or a write was refused
    * `:store_unavailable` — this runtime has no language store at all
    * `:store_full` — the parser is fine and this process has no room for
      another one. Every language shares one Tree-sitter Wasm store whose memory
      is capped and never reclaimed, so past a point the next parser does not
      fit, whatever the machine has free. The language named is not at fault: it
      loads on its own, and which languages fail depends only on the order they
      were asked for. Nothing the running VM can do recovers — loading is global
      to it and no parser is ever unloaded — so this is a decision about which
      languages a *boot* loads, and work that needs more than fit has to be
      split across OS processes, the cap being per process and not per
      scheduler.

  The list is open: a Lumis release can add a reason, so a `case` over it needs
  a catch-all clause.

  ## Fields

    * `:language` — the language id that could not be loaded
    * `:package` — the Hex package that supplies it, `nil` when the language is
      not in the catalog
    * `:reason` — one of the atoms above
    * `:detail` — what the Rust core reported, for logs
  """

  @type reason() :: atom()

  @type t() :: %__MODULE__{
          language: String.t(),
          package: String.t() | nil,
          reason: reason(),
          detail: String.t() | nil
        }

  defexception [:language, :package, :reason, :detail]

  @doc false
  # The NIF classifies a failure and hands over its fields; the struct is built
  # here, because the advice a missing parser needs is "add it to mix.exs" in
  # Elixir and "add it to package.json" in Node, off the same `:reason`. Shared
  # by `Lumis.highlight/2` and `Lumis.Languages.load/1`, which reach the same
  # failures by different routes.
  def from_nif(%{package_suffix: suffix} = fields) do
    fields
    |> Map.delete(:package_suffix)
    |> Map.put(:package, Lumis.Packages.hex_name(suffix))
    |> then(&struct!(__MODULE__, &1))
  end

  @impl true
  def message(%__MODULE__{reason: :not_installed} = error) do
    """
    no parser for #{inspect(error.language)}: this project does not depend on one

    add it to mix.exs, then fetch your dependencies again:

        {:#{error.package}, "#{Lumis.Packages.requirement()}"}
    """
  end

  def message(%__MODULE__{reason: :parser_missing} = error) do
    """
    #{error.package} is a dependency of this project, but the #{error.language} parser it ships is missing

    fetch your dependencies again, or reinstall it:

        mix deps.get
    """
  end

  def message(%__MODULE__{reason: :incompatible_version} = error) do
    """
    this build of Lumis does not support the installed #{error.package} parser: #{error.detail}

    update the dependency in mix.exs to #{Lumis.Packages.requirement()}, or \
    pin Lumis to a version that accepts the one you have
    """
  end

  def message(%__MODULE__{reason: :unknown_language} = error) do
    "no language named #{inspect(error.language)}; see `Lumis.available_languages/0`"
  end

  def message(%__MODULE__{reason: :store_full} = error) do
    """
    no room for the #{error.language} parser: this process already holds as many as its Wasm store can

    #{error.language} is not at fault — it loads on its own, and which languages \
    fail depends only on the order they were asked for.

    This VM cannot recover: loading is global to it and a parser is never \
    unloaded, so no retry and no shorter call frees room. Decide the set at boot \
    instead — load only the languages this application can be asked for rather \
    than everything published — and split work that needs more than fit across \
    OS processes.

    Tree-sitter reported: #{error.detail}
    """
  end

  def message(%__MODULE__{reason: :not_loaded} = error) do
    "could not load the #{error.language} parser. Warm it with " <>
      "`Lumis.Languages.async_load([#{inspect(error.language)}])` from your " <>
      "application's start/2 if this host has no network access"
  end

  def message(%__MODULE__{} = error) do
    "could not load the #{error.language} parser: #{error.detail}"
  end
end

defmodule Lumis.RenderError do
  @moduledoc """
  Highlighting failed for a reason that is not a missing parser.

  `Lumis.highlight/2` returns this when the formatter, the annotations, the
  render itself or the WASM runtime failed. Most of these are a caller mistake
  that the options schema could not catch; `:runtime` is environmental.

  ## Reasons

    * `:formatter` — the formatter could not be built, most often an unknown
      theme name
    * `:annotation` — an annotation could not be composed onto the source
    * `:render` — the formatter failed while writing its output
    * `:highlight` — the highlighter itself failed
    * `:invalid_match_limit` — `budget: [match_limit: _]` was outside `1..65536`
    * `:runtime` — the WASM runtime could not be started or is unusable

  The list is open, so a `case` over it needs a catch-all clause.
  """

  @type reason() :: atom()

  @type t() :: %__MODULE__{reason: reason(), detail: String.t() | nil}

  defexception [:reason, :detail]

  @impl true
  def message(%__MODULE__{reason: :formatter, detail: detail}),
    do: "invalid formatter: #{detail}"

  def message(%__MODULE__{reason: :annotation, detail: detail}),
    do: "invalid annotation: #{detail}"

  def message(%__MODULE__{reason: :runtime, detail: detail}),
    do: "the Lumis WASM runtime is unavailable: #{detail}"

  def message(%__MODULE__{detail: detail}), do: detail
end

defmodule Lumis.HighlightError do
  defexception [:error]

  @type t() :: %__MODULE__{error: Exception.t() | term()}

  def message(%__MODULE__{error: error}) do
    detail = if is_exception(error), do: Exception.message(error), else: inspect(error)

    """
    error highlighting source code

    got:

      #{detail}
    """
  end
end

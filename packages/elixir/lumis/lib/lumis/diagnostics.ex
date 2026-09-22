defmodule Lumis.Diagnostics do
  @moduledoc false

  # Reporting a language a document named and highlighting skipped.
  #
  # Internal. What users touch is `config :lumis, :report_unresolved`.
  #
  # Reporting only. Whether a language loads is decided by `lumis-lock.toml` and
  # nothing else; this setting decides whether you hear about the ones that did
  # not. Keeping those apart is deliberate — a switch that could do both would
  # be a second place to look when a block comes back plain.

  require Logger

  @table :lumis_unresolved_reported

  @doc false
  # One line per language for the life of the node, not one per document. A
  # README with twenty fenced blocks in a language nobody installed is one
  # missing parser, and twenty identical lines would bury the next real one.
  def report(unresolved)

  def report([]), do: :ok

  def report(unresolved) when is_list(unresolved) do
    for language <- unresolved, first_time?(language) do
      Logger.warning("""
      Lumis skipped "#{language}", injected inside the document being highlighted, \
      so that block rendered without highlighting.

      Pin it with `mix lumis.add #{language}` if the application should highlight it, \
      or set `config :lumis, report_unresolved: false` to stop reporting this.\
      """)
    end

    :ok
  end

  @doc false
  def report_unresolved? do
    Application.get_env(:lumis, :report_unresolved, true)
  end

  @doc false
  # Created here rather than in a supervisor: reporting has to work for a
  # `Lumis.highlight/2` call in a script or a test that never started the
  # application, and an owner process would take the table down with it.
  def setup do
    :ets.new(@table, [:named_table, :public, :set, read_concurrency: true])
    :ok
  rescue
    ArgumentError -> :ok
  end

  # `insert_new` rather than a lookup and a write: two documents highlighting
  # the same missing language on different schedulers would otherwise both see
  # an empty table and both report.
  defp first_time?(language) do
    setup()
    :ets.insert_new(@table, {language})
  end

  @doc false
  # Test support: forget what has been reported so the next call reports again.
  def reset do
    setup()
    :ets.delete_all_objects(@table)
    :ok
  end
end

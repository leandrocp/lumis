defmodule Mix.Tasks.Lumis.Update do
  @moduledoc """
  Re-resolve locked packages and move `lumis-lock.toml`.

      mix lumis.update rust
      mix lumis.update --all

  The only task that changes a pinned version. Everything is resolved before
  anything is written, so a registry that is down for one package leaves the
  lock untouched rather than moving the rest.
  """
  @shortdoc "Re-resolve locked packages"

  use Mix.Task
  alias Mix.Tasks.Lumis.Helpers

  @impl Mix.Task
  def run(args) do
    {opts, languages} = OptionParser.parse!(args, strict: [all: :boolean])
    all? = Keyword.get(opts, :all, false)

    cond do
      all? and languages != [] ->
        Mix.raise("name a language or pass --all, not both")

      not all? and languages == [] ->
        Mix.raise("name a language to update, or pass --all")

      true ->
        :ok
    end

    Helpers.start!()
    path = Helpers.lock_path!(false)

    changes = Helpers.unwrap!(Lumis.Native.lock_update(path, languages, all?))

    changes
    |> Enum.filter(& &1.moved)
    |> Helpers.report_changes()

    Helpers.sync!(path)
  end
end

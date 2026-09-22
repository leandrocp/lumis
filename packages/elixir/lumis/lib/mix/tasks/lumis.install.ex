defmodule Mix.Tasks.Lumis.Install do
  @moduledoc """
  Cache exactly what `lumis-lock.toml` names.

      mix lumis.install

  Takes no language names — use `mix lumis.add` to record one. Resolving nothing
  is the point: this downloads the pinned versions and fails if the store holds
  a different one, so a stale machine cache cannot quietly win.

  This is the release step, and it also refreshes the copy of the lock the
  application reads at boot.
  """
  @shortdoc "Cache exactly what lumis-lock.toml names"

  use Mix.Task
  alias Mix.Tasks.Lumis.Helpers

  @impl Mix.Task
  def run([name | _]) do
    Mix.raise("""
    mix lumis.install takes no language names

    To record #{name}, run:

        mix lumis.add #{name}
    """)
  end

  def run([]) do
    Helpers.start!()
    path = Helpers.lock_path!(false)

    case Helpers.unwrap!(Lumis.Native.lock_languages(path)) do
      nil ->
        Mix.raise("no #{Lumis.Lock.file_name()} at #{path}")

      [] ->
        Mix.shell().info("#{Path.relative_to_cwd(path)} names no languages")
        Helpers.sync!(path)

      languages ->
        Helpers.sync!(path)
        Helpers.materialize!(languages)
    end
  end
end

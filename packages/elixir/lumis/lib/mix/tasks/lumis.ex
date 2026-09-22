defmodule Mix.Tasks.Lumis do
  @moduledoc """
  Manage the parsers this project may load.

      mix lumis.add rust
      mix lumis.remove rust
      mix lumis.update rust
      mix lumis.update --all
      mix lumis.install

  See `Lumis.Lock` for what `lumis-lock.toml` is and where it lives.
  """
  @shortdoc "Manage lumis-lock.toml"

  use Mix.Task

  @impl Mix.Task
  def run(_args) do
    Mix.raise("""
    mix lumis takes a subcommand:

      mix lumis.add <language>       record languages and cache them
      mix lumis.remove <language>    stop recording languages
      mix lumis.update <language>    re-resolve one, or pass --all
      mix lumis.install              cache exactly what the lock names
    """)
  end
end

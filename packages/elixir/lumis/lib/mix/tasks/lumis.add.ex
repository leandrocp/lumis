defmodule Mix.Tasks.Lumis.Add do
  @moduledoc """
  Record languages in `lumis-lock.toml` and cache them.

      mix lumis.add rust
      mix lumis.add elixir heex
      mix lumis.add bundle-web

  Creates the lock at the repository root when there is none, and prints where.

  A bundle is shorthand: its members are recorded and the bundle name is not, so
  re-running `mix lumis.add bundle-web` is what picks up a language the bundle
  gained. Widening what a project may load stays a visible diff.
  """
  @shortdoc "Record languages in lumis-lock.toml"

  use Mix.Task
  alias Mix.Tasks.Lumis.Helpers

  @impl Mix.Task
  def run(args) do
    languages = Helpers.languages!(args, "lumis.add")
    Helpers.start!()

    path = Helpers.lock_path!(true)
    existed? = File.regular?(path)

    changes = Helpers.unwrap!(Lumis.Native.lock_add(path, languages))

    unless existed?, do: Mix.shell().info("created #{Path.relative_to_cwd(path)}")
    Helpers.report_changes(changes)
    Helpers.sync!(path)

    # Cache after recording, so the parser is on disk before anything asks for
    # it and the entry just written is proven satisfiable rather than left to
    # the next run.
    Helpers.materialize!(languages)
  end
end

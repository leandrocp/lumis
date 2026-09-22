defmodule Mix.Tasks.Lumis.Remove do
  @moduledoc """
  Stop recording languages in `lumis-lock.toml`.

      mix lumis.remove rust

  The package stays while another language still needs it: `ejs` and `erb` are
  both `@lumis-sh/wasm-embedded-template`, so removing one leaves the other
  pinned. Parser bytes already on disk are left alone.
  """
  @shortdoc "Stop recording languages in lumis-lock.toml"

  use Mix.Task
  alias Mix.Tasks.Lumis.Helpers

  @impl Mix.Task
  def run(args) do
    languages = Helpers.languages!(args, "lumis.remove")
    Helpers.start!()

    path = Helpers.lock_path!(false)
    removed = Helpers.unwrap!(Lumis.Native.lock_remove(path, languages))

    Enum.each(languages -- removed, fn name ->
      Mix.shell().info("#{name} is not in #{Lumis.Lock.file_name()}")
    end)

    Enum.each(removed, &Mix.shell().info("removed #{&1}"))
    Helpers.sync!(path)
  end
end

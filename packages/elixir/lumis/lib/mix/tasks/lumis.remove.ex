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
    {removed, missing} = Helpers.unwrap!(Lumis.Native.lock_remove(path, languages))

    # Both lists come back expanded. Subtracting from the raw arguments instead
    # would announce `bundle-web is not in lumis-lock.toml` right after removing
    # every one of its members, since a bundle name is never itself recorded.
    Enum.each(missing, fn name ->
      Mix.shell().info("#{name} is not in #{Lumis.Lock.file_name()}")
    end)

    Enum.each(removed, &Mix.shell().info("removed #{&1}"))
    Helpers.sync!(path)
  end
end

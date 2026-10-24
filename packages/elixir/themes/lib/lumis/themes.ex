defmodule Lumis.Themes do
  @moduledoc false

  @doc false
  @spec installed_files() :: [Path.t()]
  def installed_files do
    [pack_file!(:lumis_themes) | optional_files()]
  end

  @doc false
  @spec ensure_initialized(module()) :: :ok
  def ensure_initialized(native) when is_atom(native) do
    files = installed_files()
    key = {__MODULE__, native, files}

    if not :persistent_term.get(key, false) do
      :ok =
        :global.trans(
          {{__MODULE__, native}, self()},
          fn -> initialize(native, files, key) end,
          [node()]
        )
    end

    :ok
  end

  defp initialize(native, files, key) do
    if not :persistent_term.get(key, false) do
      :ok = native.install_theme_packs(files)
      :persistent_term.put(key, true)
    end

    :ok
  end

  defp optional_files do
    case :code.priv_dir(:lumis_themes_copyleft) do
      {:error, :bad_name} -> []
      dir -> [Path.join(to_string(dir), "lumis-themes.json")]
    end
  end

  defp pack_file!(app) do
    case :code.priv_dir(app) do
      {:error, reason} -> raise "cannot locate #{app} theme pack: #{inspect(reason)}"
      dir -> Path.join(to_string(dir), "lumis-themes.json")
    end
  end
end

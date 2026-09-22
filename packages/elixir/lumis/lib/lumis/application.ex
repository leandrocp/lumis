defmodule Lumis.Application do
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    # A lock that cannot be read stops the boot rather than being skipped:
    # starting without the pins a project checked in would quietly restore the
    # behaviour the lock was added to remove.
    case Lumis.Native.configure_store(
           data_dir(),
           Lumis.Lock.path(),
           Lumis.Packages.installed_dirs()
         ) do
      {:ok, _} -> :ok
      {:error, reason} -> raise "Lumis could not read #{Lumis.Lock.file_name()}: #{reason}"
    end

    opts = [strategy: :one_for_one, name: Lumis.Supervisor]
    Supervisor.start_link([{Task.Supervisor, name: Lumis.TaskSupervisor}], opts)
  end

  @doc false
  # `nil` hands the decision back to the NIF, which reads `LUMIS_DATA_DIR` and
  # then falls back to the user data directory. Elixir only has to answer for
  # the case neither of those covers.
  def data_dir do
    cond do
      path = Application.get_env(:lumis, :data_dir) -> Path.expand(path)
      named_env() -> nil
      true -> priv_dir()
    end
  end

  @doc false
  # The directory Lumis will actually use, which `data_dir/0` deliberately does
  # not answer: it returns `nil` where the NIF decides. `Lumis.Lock` needs the
  # path itself, to find the copy of the lock a release reads.
  #
  # `nil` only when the platform default is what applies, which Elixir cannot
  # compute — the NIF asks `etcetera` for it.
  def resolved_data_dir do
    cond do
      path = Application.get_env(:lumis, :data_dir) -> Path.expand(path)
      path = named_env() -> Path.expand(path)
      true -> priv_dir()
    end
  end

  # An empty value names no directory. `PathBuf::from("")` resolves to the
  # current directory on the Rust side, so an unfiltered value would scatter the
  # store wherever the process started; both sides have to agree it is unset.
  defp named_env do
    case System.get_env("LUMIS_DATA_DIR") do
      nil -> nil
      "" -> nil
      path -> path
    end
  end

  # Keep the zero-configuration cache inside the application. Releases that use
  # a read-only filesystem or need persistence across deployments configure an
  # absolute writable `:data_dir` instead.
  defp priv_dir do
    case :code.priv_dir(:lumis) do
      {:error, _} -> nil
      priv -> Path.join(List.to_string(priv), "lumis")
    end
  end
end

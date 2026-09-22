defmodule Mix.Tasks.Lumis.Helpers do
  @moduledoc false

  # Shared by the `mix lumis.*` tasks. The lock format, bundle expansion and
  # resolution all live in Rust; these tasks parse arguments, print, and keep the
  # data directory's copy in step. Nothing here reimplements what a lock means.

  @spec start!() :: :ok
  def start! do
    Mix.Task.run("app.start")
    :ok
  end

  @spec lock_path!(boolean()) :: Path.t()
  def lock_path!(create?) do
    case Lumis.Lock.project_path(create?) do
      {:ok, path} ->
        path

      {:error, reason} ->
        Mix.raise("""
        #{reason}

        Record a language first:

            mix lumis.add rust
        """)
    end
  end

  @spec languages!([String.t()], String.t()) :: [String.t()]
  def languages!([], task) do
    Mix.raise("mix #{task} takes language names, or a bundle such as bundle-web")
  end

  def languages!(names, _task), do: names

  @spec unwrap!({:ok, term()} | {:error, String.t()}) :: term()
  def unwrap!({:ok, value}), do: value
  def unwrap!({:error, reason}), do: Mix.raise(reason)

  @doc """
  Keep the data directory's copy in step with `path`.

  A release ships `priv/` and not the project root, so the copy is what the
  application reads. Writing the lock without it would leave a project whose
  tests pass and whose release pins nothing.
  """
  @spec sync!(Path.t()) :: :ok
  def sync!(path) do
    case Lumis.Lock.sync(path) do
      :ok -> :ok
      {:error, reason} -> Mix.raise("could not copy #{path} into the data directory: #{reason}")
    end
  end

  @spec report_changes([map()]) :: :ok
  def report_changes([]), do: Mix.shell().info("already up to date")

  def report_changes(changes) do
    Enum.each(changes, fn change ->
      case change do
        %{moved: true, package: package, previous_version: previous, version: version} ->
          Mix.shell().info("#{package} #{previous} -> #{version}")

        %{package: package, version: version} ->
          Mix.shell().info("#{package} #{version}")
      end
    end)
  end

  @doc """
  Download, verify and compile `names`, reporting every failure.
  """
  @spec materialize!([String.t()]) :: :ok
  def materialize!([]), do: :ok

  def materialize!(names) do
    # The store took its lock at boot, before this task edited it. Caching
    # against that copy would refuse the entry just written.
    lock_path!(false) |> Lumis.Native.lock_refresh() |> unwrap!()

    case Lumis.Languages.cache(names) do
      {:ok, _paths} ->
        :ok

      {:error, failures} when is_map(failures) ->
        Enum.each(failures, fn {name, reason} -> Mix.shell().error("#{name}: #{reason}") end)
        Mix.raise("failed to prepare #{map_size(failures)} language(s)")

      {:error, reason} ->
        Mix.raise(to_string(inspect(reason)))
    end
  end
end

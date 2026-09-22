defmodule Lumis.Lock do
  @moduledoc false

  # Where this project's `lumis-lock.toml` lives, and how it reaches a release.
  #
  # Internal. What users touch is `mix lumis.*` and `config :lumis, :lock`; what
  # a lock is and which runtimes need one belongs in the docs site, not here.

  @lock_file "lumis-lock.toml"

  # The lock governing this application, or `nil` when there is none.
  #
  # Resolved in order:
  #
  #   1. `config :lumis, :lock`, an explicit path
  #   2. `lumis-lock.toml` in the data directory, which is what a release reads
  #   3. `lumis-lock.toml` at or above the working directory, for `mix` and tests
  #
  # A release has no project to search upward from, which is why the data
  # directory carries a copy. `mix lumis.add` and `mix lumis.install` write it,
  # so the two never drift apart in normal use.
  @spec path() :: Path.t() | nil
  def path do
    configured() || in_data_dir() || nearest(File.cwd!())
  end

  # The lock a `mix lumis.*` task edits: the one governing the current project.
  #
  # Unlike `path/0` this never answers with the data directory's copy, which is
  # an output rather than a source. `create?` decides whether a project with no
  # lock yet gets a path to create one at, which only `mix lumis.add` wants.
  @spec project_path(boolean()) :: {:ok, Path.t()} | {:error, String.t()}
  def project_path(create? \\ false) do
    cwd = File.cwd!()

    cond do
      found = nearest(cwd) -> {:ok, found}
      not create? -> {:error, "no #{@lock_file} found at or above #{cwd}"}
      true -> {:ok, Path.join(project_root(cwd), @lock_file)}
    end
  end

  # Copy `source` into the data directory, so a release reads what was locked.
  #
  # A release ships `priv/` but not the project root, so without this the
  # application would boot with no lock and load anything — the behaviour the
  # lock exists to remove.
  @spec sync(Path.t()) :: :ok | {:error, File.posix()}
  def sync(source) do
    destination = data_dir_lock()

    cond do
      is_nil(destination) -> :ok
      Path.expand(source) == Path.expand(destination) -> :ok
      true -> copy(source, destination)
    end
  end

  @spec file_name() :: String.t()
  def file_name, do: @lock_file

  defp copy(source, destination) do
    with :ok <- File.mkdir_p(Path.dirname(destination)),
         {:ok, _} <- File.copy(source, destination) do
      :ok
    else
      {:error, reason} -> {:error, reason}
    end
  end

  defp configured do
    case Application.get_env(:lumis, :lock) do
      nil -> nil
      path -> Path.expand(path)
    end
  end

  defp in_data_dir do
    case data_dir_lock() do
      nil -> nil
      path -> if File.regular?(path), do: path
    end
  end

  defp data_dir_lock do
    case Lumis.Application.resolved_data_dir() do
      nil -> nil
      dir -> Path.join(dir, @lock_file)
    end
  end

  # Walk up rather than looking only beside the caller, so a task run inside an
  # umbrella application still finds the repository's one lock.
  defp nearest(dir) do
    candidate = Path.join(dir, @lock_file)
    parent = Path.dirname(dir)

    cond do
      File.regular?(candidate) -> candidate
      parent == dir -> nil
      true -> nearest(parent)
    end
  end

  # The repository root, so `mix lumis.add` in an umbrella child still writes one
  # lock. Deliberately not derived from `mix.exs`: an umbrella has several and
  # nothing says which is right, which is the same reason there is one lock
  # rather than one per application.
  #
  # `exists?` rather than `dir?`: in a git worktree `.git` is a *file* holding a
  # gitlink, so checking for a directory walks straight past the worktree and
  # into whatever repository encloses it.
  defp project_root(dir) do
    parent = Path.dirname(dir)

    cond do
      File.exists?(Path.join(dir, ".git")) -> dir
      parent == dir -> File.cwd!()
      true -> project_root(parent)
    end
  end
end

# https://github.com/elixir-explorer/explorer/blob/d11216282bbdb0dcaef2519c2bfefda46c2981e0/lib/explorer/comptime_utils.ex

defmodule Lumis.ComptimeUtils do
  @moduledoc false

  # Jason comes from optional Rustler and is only called for source builds.
  @compile {:no_warn_undefined, Jason}

  # Only works for Linux targets today, but we don't need more.
  @doc false
  def cpu_with_all_caps?(needed_flags, opts \\ []) do
    opts = Keyword.validate!(opts, cpu_info_file_path: "/proc/cpuinfo", target: nil)

    case File.read(opts[:cpu_info_file_path]) do
      {:ok, contents} ->
        flags =
          contents
          |> String.split("\n")
          |> Stream.filter(&String.starts_with?(&1, "flags"))
          |> Stream.map(fn line ->
            [_, flags] = String.split(line, ": ")
            String.split(flags)
          end)
          |> Stream.uniq()
          |> Enum.to_list()
          |> List.flatten()

        Enum.all?(needed_flags, fn flag -> flag in flags end)

      {:error, _} ->
        # There is no way to say, so we default to false.
        false
    end
  end

  @doc false
  def local_cargo_dependency_resources(crate_path) do
    crate_path = Path.expand(crate_path)
    metadata = cargo_metadata!(crate_path)
    packages = Map.new(metadata["packages"], &{&1["id"], &1})

    dependencies =
      Map.new(metadata["resolve"]["nodes"], fn node ->
        {node["id"], Enum.map(node["deps"], & &1["pkg"])}
      end)

    root_id =
      Enum.find_value(packages, fn {id, package} ->
        if Path.dirname(package["manifest_path"]) == crate_path, do: id
      end) || raise "cargo metadata did not contain the crate at #{crate_path}"

    dependencies
    |> reachable_dependency_ids([root_id], MapSet.new())
    |> MapSet.delete(root_id)
    |> Enum.map(&Map.fetch!(packages, &1))
    |> Enum.filter(&is_nil(&1["source"]))
    |> Enum.flat_map(&cargo_package_resources/1)
    |> Enum.sort()
  end

  defp cargo_metadata!(crate_path) do
    case System.cmd("cargo", ~w[metadata --format-version=1], cd: crate_path) do
      {metadata, 0} -> Jason.decode!(metadata)
      {output, _code} -> raise "calling `cargo metadata` failed.\n" <> output
    end
  end

  defp reachable_dependency_ids(_dependencies, [], visited), do: visited

  defp reachable_dependency_ids(dependencies, [id | rest], visited) do
    if MapSet.member?(visited, id) do
      reachable_dependency_ids(dependencies, rest, visited)
    else
      reachable_dependency_ids(
        dependencies,
        Map.get(dependencies, id, []) ++ rest,
        MapSet.put(visited, id)
      )
    end
  end

  defp cargo_package_resources(package) do
    package_path = Path.dirname(package["manifest_path"])
    target_path = Path.join(package_path, "target/")

    package_path
    |> Path.join("**/*")
    |> Path.wildcard()
    |> Enum.reject(&(String.starts_with?(&1, target_path) or File.dir?(&1)))
  end
end

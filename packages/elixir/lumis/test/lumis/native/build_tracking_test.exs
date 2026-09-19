defmodule Lumis.Native.BuildTrackingTest do
  use ExUnit.Case, async: false

  @repo_root Path.expand("../../../../../..", __DIR__)
  @tracks_workspace_crates System.get_env("LUMIS_BUILD") in ["1", "true"] and
                             File.dir?(Path.join(@repo_root, "crates"))

  test "tracks the patched crates only in workspace source builds" do
    resources =
      Lumis.Native.module_info(:attributes)
      |> Keyword.get_values(:external_resource)
      |> List.flatten()

    tracked_resources = [
      Path.join(@repo_root, "crates/lumis-core/src/highlights.rs"),
      Path.join(@repo_root, "crates/lumis-wasm-runtime/src/runtime.rs")
    ]

    assert Enum.all?(tracked_resources, &(&1 in resources)) == @tracks_workspace_crates
    assert function_exported?(Lumis.Native, :__mix_recompile__?, 0) == @tracks_workspace_crates
    refute Path.join(@repo_root, "crates/lumis-cli/src/main.rs") in resources

    if @tracks_workspace_crates do
      probe_name = "build_tracking_probe_#{System.unique_integer([:positive])}.tmp"

      probe_paths =
        Enum.map(~w[lumis-core lumis-wasm-runtime], fn crate ->
          Path.join([@repo_root, "crates", crate, probe_name])
        end)

      on_exit(fn -> Enum.each(probe_paths, &File.rm/1) end)
      Enum.each(probe_paths, &File.write!(&1, "probe"))

      assert Lumis.Native.__mix_recompile__?()
    end
  end
end

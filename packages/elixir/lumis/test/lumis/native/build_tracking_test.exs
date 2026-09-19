defmodule Lumis.Native.BuildTrackingTest do
  use ExUnit.Case, async: true

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
  end
end

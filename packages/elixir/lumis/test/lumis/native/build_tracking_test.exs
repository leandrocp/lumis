defmodule Lumis.Native.BuildTrackingTest do
  use ExUnit.Case, async: true

  test "tracks locally patched Rust dependencies" do
    resources =
      Lumis.Native.module_info(:attributes)
      |> Keyword.get_values(:external_resource)
      |> List.flatten()

    repo_root = Path.expand("../../../../../..", __DIR__)

    assert Path.join(repo_root, "crates/lumis-core/src/highlights.rs") in resources
    assert Path.join(repo_root, "crates/lumis-wasm-runtime/src/runtime.rs") in resources
    refute Path.join(repo_root, "crates/lumis-cli/src/main.rs") in resources
  end
end

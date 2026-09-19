defmodule Lumis.Native.BuildTrackingTest do
  use ExUnit.Case, async: false

  @repo_root Path.expand("../../../../../..", __DIR__)

  setup do
    resources =
      Lumis.Native.module_info(:attributes)
      |> Keyword.get_values(:external_resource)
      |> List.flatten()

    # `LUMIS_BUILD` is read when `Lumis.Native` compiles, which is often an
    # earlier run with a different value, so reading it here would test the
    # shell rather than the module. Rustler registers the NIF crate's own
    # sources only on a source build, which is the same condition and is
    # recorded on the module itself.
    source_build? =
      "native/lumis_nif/src/lib.rs" in resources and
        File.dir?(Path.join(@repo_root, "crates"))

    %{resources: resources, source_build?: source_build?}
  end

  test "tracks the patched crates only in workspace source builds", ctx do
    tracked_resources = [
      Path.join(@repo_root, "crates/lumis-core/src/highlights.rs"),
      Path.join(@repo_root, "crates/lumis-wasm-runtime/src/runtime.rs")
    ]

    build_inputs = [
      Path.join(@repo_root, "Cargo.toml"),
      Path.join(@repo_root, "Cargo.lock"),
      Path.join(@repo_root, "packages/elixir/lumis/native/lumis_nif/.cargo/config.toml")
    ]

    assert Enum.all?(tracked_resources, &(&1 in ctx.resources)) == ctx.source_build?
    assert Enum.all?(build_inputs, &(&1 in ctx.resources)) == ctx.source_build?
    assert function_exported?(Lumis.Native, :__mix_recompile__?, 0) == ctx.source_build?
    refute Path.join(@repo_root, "crates/lumis-cli/src/main.rs") in ctx.resources

    if ctx.source_build? do
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

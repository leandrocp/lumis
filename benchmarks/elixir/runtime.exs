defmodule Lumis.BenchmarkRuntime do
  @moduledoc """
  Points Lumis at the parsers `mise run prepare:languages` laid out, so nothing
  in a timed run depends on the network.
  """

  def configure(repo_dir) do
    packages = Path.join(repo_dir, "target/benchmarks/language-packages")
    parsers = Path.join(packages, "parsers")

    unless File.dir?(parsers) do
      raise "run `mise run -C benchmarks prepare:languages` first, no parsers at #{packages}"
    end

    # The prepared packages stand in for `lumis_wasm_*` deps: `Mix.install` here
    # depends on none, so `Lumis.Packages.installed_dirs/0` finds nothing and
    # every scenario language would be reported as not installed. Declaring the
    # directory instead keeps a timed run off the network, the way depending on
    # the Hex packages would.
    #
    # One store directory, so the prepared packages are also where compiled
    # modules are written.
    #
    # After Application.start, so the NIF store is configured here rather than
    # from config; it is built lazily on first use, which has not happened yet.
    true = Lumis.Native.configure_store(packages, [parsers])
  end
end

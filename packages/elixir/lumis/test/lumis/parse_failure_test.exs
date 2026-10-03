defmodule Lumis.ParseFailureTest do
  use ExUnit.Case, async: true

  @broken_parser Path.expand("../../../../../fixtures/failing-parsers/php-0.26.4.wasm", __DIR__)

  @tag :tmp_dir
  test "a scanner trap reports :parse and the next document still highlights", %{tmp_dir: dir} do
    store = Application.fetch_env!(:lumis, :data_dir)
    package = store |> Path.join("parsers/php.lumis.json") |> File.read!() |> Jason.decode!()
    wasm = File.read!(@broken_parser)
    digest = :crypto.hash(:sha256, wasm) |> Base.encode16(case: :lower)
    package = put_in(package, ["parser", "sha256"], digest)
    File.write!(Path.join(dir, "php.lumis.json"), Jason.encode!(package))
    File.write!(Path.join(dir, "tree-sitter-php-#{package["version"]}-#{digest}.wasm"), wasm)

    # A runtime keeps its loaded parsers for the VM's lifetime. This process
    # must start with the broken parser, independently of the conformance suite.
    script = """
    Application.put_env(:lumis, :parser_dirs, [#{inspect(dir)}])
    options = [formatter: {:html_linked, language: "php"}]
    {:ok, before} = Lumis.highlight("<?php $a = 1;", options)
    {:error, %Lumis.RenderError{reason: :parse, detail: detail}} =
      Lumis.highlight("<?php\\n$a = <<<END\\nEND;\\n", options)
    true = detail == "parser returned no tree for language 'php'"
    {:ok, ^before} = Lumis.highlight("<?php $a = 1;", options)
    IO.write("parse failure and recovery verified")
    """

    {output, status} =
      System.cmd("mix", ["run", "--no-start", "--no-compile", "-e", script],
        env: [{"MIX_ENV", "test"}],
        stderr_to_stdout: true
      )

    assert status == 0, output
    assert output =~ "parse failure and recovery verified"
  end
end

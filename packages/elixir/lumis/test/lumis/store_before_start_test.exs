defmodule Lumis.StoreBeforeStartTest do
  use ExUnit.Case, async: true

  # The store is built once per VM, and this suite's VM started :lumis before
  # any test ran. `mix compile` rendering a template is the case that matters,
  # and it never starts the application, so the order needs a VM of its own.
  test "highlighting before :lumis starts reaches the declared parsers, then and after" do
    script = """
    highlighted? = fn -> Lumis.highlight!("x = 1", formatter: {:html_linked, language: "elixir"}) =~ "<span" end
    before_start = highlighted?.()
    {:ok, _} = Application.ensure_all_started(:lumis)
    IO.write(inspect({before_start, highlighted?.()}))
    """

    {output, status} =
      System.cmd("mix", ["run", "--no-start", "--no-compile", "-e", script],
        env: [{"MIX_ENV", "test"}],
        stderr_to_stdout: true
      )

    assert status == 0, output
    assert output =~ "{true, true}"
  end
end

defmodule Lumis.StoreBeforeStartTest do
  use ExUnit.Case, async: true

  # The store is built once per VM, and this suite's VM started :lumis before
  # any test ran. `mix compile` rendering a template is the case that matters,
  # and it never starts the application, so the order needs a VM of its own.
  test "highlighting before :lumis starts reaches the declared parsers, then and after" do
    output =
      run_before_start("""
      highlighted? = fn -> Lumis.highlight!("x = 1", formatter: {:html_linked, language: "elixir"}) =~ "<span" end
      before_start = highlighted?.()
      {:ok, _} = Application.ensure_all_started(:lumis)
      IO.write(inspect({before_start, highlighted?.()}))
      """)

    assert output =~ "{true, true}"
  end

  # Plain text configures the store without building it, and
  # `config/runtime.exs` runs after compilation, before :lumis starts. The
  # directories it sets still have to reach the store.
  @tag :tmp_dir
  test "config set after a highlight that built no store applies when :lumis starts", %{
    tmp_dir: tmp_dir
  } do
    data_dir = Path.join(tmp_dir, "data")

    output =
      run_before_start("""
      Lumis.highlight!("hello", formatter: {:html_linked, language: "plaintext"})
      Application.put_env(:lumis, :data_dir, #{inspect(data_dir)})
      {:ok, _} = Application.ensure_all_started(:lumis)
      Lumis.highlight!("x = 1", formatter: {:html_linked, language: "elixir"})
      IO.write("data_dir_used=" <> inspect(File.dir?(#{inspect(data_dir)})))
      """)

    assert output =~ "data_dir_used=true"
  end

  defp run_before_start(script) do
    {output, status} =
      System.cmd("mix", ["run", "--no-start", "--no-compile", "-e", script],
        env: [{"MIX_ENV", "test"}],
        stderr_to_stdout: true
      )

    assert status == 0, output
    output
  end
end

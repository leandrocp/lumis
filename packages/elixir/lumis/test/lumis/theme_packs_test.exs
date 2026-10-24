defmodule Lumis.ThemePacksTest do
  use ExUnit.Case, async: true

  @moduletag timeout: 120_000

  test "default dependency works before applications start, without the optional pack" do
    run_before_start("""
    optional_ebin = :code.lib_dir(:lumis_themes_copyleft, :ebin)
    true = :code.del_path(optional_ebin)
    assert {:error, :bad_name} == :code.priv_dir(:lumis_themes_copyleft)

    [default_file] = Lumis.Themes.installed_files()
    expected_names = default_file |> File.read!() |> Jason.decode!() |> Map.fetch!("themes") |> Enum.map(& &1["name"]) |> Enum.sort()
    assert [%{name: _} | _] = Lumis.Theme.available()
    assert length(expected_names) > 200
    assert Enum.map(Lumis.Theme.available(), & &1.name) == expected_names
    assert %Lumis.Theme{name: "github_light"} = Lumis.Theme.get("github_light")
    assert Lumis.Theme.get("oceanic_next") == nil
    assert Lumis.Theme.build_css!("github_light") =~ ".lumis"
    assert %Lumis.Theme{} = Lumis.Theme.closest(%{background: "#000000"})
    cases = Path.expand("../../../fixtures/theme-choice-cases.json") |> File.read!() |> Jason.decode!()
    assert length(cases) >= 20
    for fixture <- cases do
      theme = Lumis.Theme.closest(%{background: fixture["background"], foreground: fixture["foreground"], ansi: fixture["ansi"]})
      assert (theme && theme.name) == Map.get(fixture, "expected_default", fixture["expected"]), fixture["name"]
    end
    for name <- ~w(darkplus no_clown_fiesta poimandres yorumi) do
      assert Lumis.Theme.get(name) == nil
    end
    refute List.keymember?(Application.started_applications(), :lumis, 0)
    refute List.keymember?(Application.started_applications(), :lumis_themes, 0)
    """)
  end

  for {operation, expression} <- [
        get:
          "assert %Lumis.Theme{name: \"oceanic_next\"} = Lumis.Native.get_theme(\"oceanic_next\")",
        available: "assert length(Lumis.Native.available_themes()) == 252",
        closest: "assert %Lumis.Theme{} = Lumis.Theme.closest(%{background: \"#1b2b34\"})",
        css: "assert Lumis.Theme.build_css!(\"oceanic_next\") =~ \".lumis\"",
        highlight: """
        assert {:ok, html} = Lumis.highlight("hello", formatter: {:html_inline, language: "plaintext", theme: "oceanic_next"})
        assert html =~ "#1b2b34"
        """,
        native_highlight: """
        options = [formatter: {:html_inline, language: "plaintext", theme: "oceanic_next"}] |> Lumis.validate_options!() |> Lumis.rust_options!()
        assert {:ok, html} = Lumis.Native.highlight("hello", options)
        assert html =~ "#1b2b34"
        """,
        compile_time: """
        Code.compile_string(""\"
        defmodule CompileTimeTheme do
          @theme Lumis.Theme.get("oceanic_next")
          def theme, do: @theme
        end
        ""\")
        assert %Lumis.Theme{name: "oceanic_next"} = CompileTimeTheme.theme()
        """
      ] do
    test "#{operation} initializes installed packs on its first pre-start call" do
      run_before_start(unquote(expression))
    end
  end

  test "concurrent first calls and repeated native installation are idempotent" do
    run_before_start("""
    results =
      1..32
      |> Task.async_stream(fn _ -> Lumis.Theme.get("oceanic_next") end, max_concurrency: 16)
      |> Enum.to_list()
    assert length(results) == 32
    assert Enum.all?(results, &match?({:ok, %Lumis.Theme{name: "oceanic_next"}}, &1))
    files = Lumis.Themes.installed_files()
    assert :ok = Lumis.Native.install_theme_packs(files)
    assert :ok = Lumis.Native.install_theme_packs(files)
    assert length(Lumis.Theme.available()) == 252
    {:ok, _} = Application.ensure_all_started(:lumis)
    assert %Lumis.Theme{name: "oceanic_next"} = Lumis.Theme.get("oceanic_next")
    """)
  end

  @tag :tmp_dir
  test "a missing pack file fails without losing the installed themes", %{tmp_dir: tmp_dir} do
    assert {:error, message} =
             Lumis.Native.install_theme_packs([Path.join(tmp_dir, "missing.json")])

    assert message =~ "cannot read theme pack"
    assert %Lumis.Theme{name: "github_light"} = Lumis.Theme.get("github_light")
    assert %Lumis.Theme{name: "oceanic_next"} = Lumis.Theme.get("oceanic_next")
  end

  defp run_before_start(script) do
    {output, status} =
      System.cmd(
        "mix",
        [
          "run",
          "--no-start",
          "--no-compile",
          "-e",
          "import ExUnit.Assertions\nassert Lumis.Native.get_theme_nif(\"oceanic_next\") == :error\n" <>
            script
        ],
        env: [{"MIX_ENV", "test"}, {"ERL_FLAGS", "+S 2:2"}],
        stderr_to_stdout: true
      )

    assert status == 0, output
  end
end

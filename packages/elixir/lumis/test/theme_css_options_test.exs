defmodule Lumis.ThemeCssOptionsTest do
  use ExUnit.Case, async: false

  @manifest_path Path.expand("../../../../fixtures/theme-css-options.json", __DIR__)
  @external_resource @manifest_path

  defp manifest, do: @manifest_path |> File.read!() |> Jason.decode!()

  defp options(values) do
    Enum.map(values, fn
      {"container_style", pairs} -> {:container_style, Enum.map(pairs, &List.to_tuple/1)}
      {name, value} -> {String.to_existing_atom(name), value}
    end)
  end

  test "pins CSS option names and defaults in both directions" do
    source =
      Path.expand("../lib/lumis/theme.ex", __DIR__) |> File.read!() |> Code.string_to_quoted!()

    {_, schema} =
      Macro.prewalk(source, nil, fn
        {:@, _, [{:css_options_schema, _, [schema]}]} = node, _ -> {node, schema}
        node, schema -> {node, schema}
      end)

    {schema, _} = Code.eval_quoted(schema)
    actual = Map.new(schema, fn {key, spec} -> {key, Keyword.fetch!(spec, :default)} end)

    expected =
      manifest()["options"] |> Map.new(&{&1["name"], &1["default"]}) |> options() |> Map.new()

    assert actual == expected
  end

  test "renders the shared CSS cases through the NIF" do
    manifest = manifest()
    {:ok, theme} = manifest["theme"] |> Jason.encode!() |> Lumis.Theme.from_json()
    assert length(manifest["cases"]) >= 6

    for entry <- manifest["cases"] do
      assert Lumis.Theme.build_css!(theme, options(entry["options"])) == entry["css"],
             entry["name"]
    end
  end

  test "enable_italic warns only when supplied and preserves CSS output" do
    {:ok, theme} = manifest()["theme"] |> Jason.encode!() |> Lumis.Theme.from_json()

    assert ExUnit.CaptureIO.capture_io(:stderr, fn ->
             assert Lumis.Theme.build_css!(theme) =~ "font-style: italic;"
           end) == ""

    for enabled <- [false, true] do
      warning =
        ExUnit.CaptureIO.capture_io(:stderr, fn ->
          css = Lumis.Theme.build_css!(theme, enable_italic: enabled)
          assert String.contains?(css, "font-style: italic;") == enabled
        end)

      assert warning =~ "deprecated"
      assert warning =~ "Lumis.Theme.map_styles/2"
    end
  end

  test "accepts layout for a theme name and validates its type" do
    full = Lumis.Theme.build_css!("github_light")
    colors = full |> String.split("@layer lumis") |> hd()
    assert {:ok, ^colors} = Lumis.Theme.build_css("github_light", layout: false)
    assert Lumis.Theme.build_css!("github_light", layout: true) == full

    assert {:error, %NimbleOptions.ValidationError{}} =
             Lumis.Theme.build_css("github_light", layout: "false")

    assert {:error, :not_found} = Lumis.Theme.build_css("missing-theme", layout: false)
  end
end

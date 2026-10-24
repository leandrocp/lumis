defmodule Lumis.ThemeClosestTest do
  use ExUnit.Case, async: true

  alias Lumis.Formatter.ANSI
  alias Lumis.Theme

  @fixtures Path.expand("../../../../fixtures", __DIR__)
  @cases @fixtures |> Path.join("theme-choice-cases.json") |> File.read!() |> Jason.decode!()
  @excluded @fixtures
            |> Path.join("theme-choice-excluded.json")
            |> File.read!()
            |> Jason.decode!()
  @core_scopes ~w(keyword string comment function type constant number variable operator punctuation)

  test "uses the complete shared corpus" do
    assert length(@cases) >= 20
    assert length(Lumis.Theme.available()) > 200
  end

  for fixture <- @cases do
    test fixture["name"] do
      fixture = unquote(Macro.escape(fixture))

      theme =
        Theme.closest(%{
          background: fixture["background"],
          foreground: fixture["foreground"],
          ansi: fixture["ansi"]
        })

      expected =
        case :code.priv_dir(:lumis_themes_copyleft) do
          {:error, :bad_name} -> Map.get(fixture, "expected_default", fixture["expected"])
          _ -> fixture["expected"]
        end

      assert (theme && theme.name) == expected
    end
  end

  test "pins the excluded monochrome themes" do
    excluded =
      Lumis.Theme.available()
      |> Enum.map(&Theme.get(&1.name))
      |> Enum.filter(&(core_color_count(&1) <= 3))
      |> Enum.map(& &1.name)

    assert excluded == @excluded
  end

  test "rejects invalid colors" do
    for hex <- ["#fff", "#gggggg", "ff+79c", "aéaaa", ""] do
      assert_raise ArgumentError, fn -> Theme.closest(%{background: hex}) end
      assert_raise ArgumentError, fn -> Theme.closest(%{foreground: hex}) end
      ansi = List.replace_at(List.duplicate(nil, 16), 15, hex)
      assert_raise ArgumentError, fn -> Theme.closest(%{background: "#000000", ansi: ansi}) end
    end
  end

  test "rejects palettes other than 16 entries" do
    for length <- [0, 15, 17] do
      assert_raise ArgumentError, fn -> Theme.closest(%{ansi: List.duplicate(nil, length)}) end
    end
  end

  defp core_color_count(theme) do
    @core_scopes
    |> Enum.map(fn scope ->
      case theme.highlights[scope] do
        %{fg: color} when is_binary(color) -> color
        _ -> theme.highlights["normal"].fg
      end
    end)
    |> Enum.reject(&is_nil/1)
    |> Enum.map(&ANSI.hex_to_rgb/1)
    |> Enum.uniq()
    |> length()
  end
end

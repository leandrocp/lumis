defmodule Lumis.ThemeTest do
  use ExUnit.Case, async: true

  alias Lumis.Theme

  describe "map_styles/2" do
    test "matches the shared contract without changing the original" do
      fixture =
        Path.expand("../../../../../fixtures/theme-map-styles.json", __DIR__)
        |> File.read!()
        |> Jason.decode!()

      {:ok, original} = Theme.from_json(Jason.encode!(fixture["theme"]))
      {:ok, expected} = Theme.from_json(Jason.encode!(fixture["expected"]))
      assert map_size(original.highlights) == 6

      mapped =
        Theme.map_styles(original, fn scope, style ->
          send(self(), {:mapped, scope})

          case scope do
            "markup.raw.block" -> %{style | italic: false}
            "normal" -> %{style | bg: "#ffffff"}
            "highlighted" -> %Theme.Style{}
            _ -> style
          end
        end)

      assert mapped == expected
      assert original.highlights["markup.raw.block"].italic
      assert Theme.build_css!(mapped) == Theme.build_css!(expected)

      for scope <- Map.keys(original.highlights), do: assert_received({:mapped, ^scope})
      refute_received {:mapped, _}
    end

    test "composes transformations and handles empty themes" do
      theme =
        Theme.get("github_light")
        |> Theme.map_styles(fn _scope, style -> %{style | italic: false} end)

      refute Theme.build_css!(theme) =~ "font-style: italic"

      theme =
        Theme.map_styles(theme, fn scope, style ->
          %{style | italic: scope == "markup.italic"}
        end)

      assert length(String.split(Theme.build_css!(theme), "font-style: italic")) == 2

      assert Theme.map_styles(%Theme{}, fn _, _ -> flunk("empty theme has no styles") end) ==
               %Theme{}
    end
  end

  describe "fetch" do
    test "fetch existing theme" do
      assert %Theme{name: "github_light", appearance: :light, highlights: highlights} =
               Theme.get("github_light")

      assert %Theme.Style{
               fg: "#1f2328",
               text_decoration: %Theme.TextDecoration{}
             } = highlights["variable"]
    end

    test "fetch invalid theme" do
      refute Theme.get("invalid")
    end
  end

  describe "from_json/1" do
    test "loads theme from valid JSON string" do
      json =
        ~s({"name": "test_theme", "appearance": "dark", "revision": "test", "highlights": {"comment": {"fg": "#808080"}}})

      assert {:ok,
              %Theme{
                name: "test_theme",
                appearance: :dark,
                revision: "test",
                highlights: highlights
              }} =
               Theme.from_json(json)

      assert highlights["comment"].fg == "#808080"
    end

    test "loads theme with text_decoration from JSON" do
      json =
        ~s({"name": "test_theme", "appearance": "dark", "revision": "test", "highlights": {"comment": {"fg": "#808080", "undercurl": true, "strikethrough": true}}})

      assert {:ok, %Theme{highlights: highlights}} = Theme.from_json(json)

      assert %Theme.Style{
               text_decoration: %Theme.TextDecoration{underline: :wavy, strikethrough: true}
             } = highlights["comment"]
    end

    test "returns error for invalid JSON" do
      assert {:error, :invalid_json} = Theme.from_json("invalid json")
    end

    test "returns error for empty string" do
      assert {:error, :invalid_json} = Theme.from_json("")
    end
  end

  describe "from_file/1" do
    @tag :tmp_dir
    test "loads theme from valid JSON file", %{tmp_dir: tmp_dir} do
      json_content =
        ~s({"name": "file_theme", "appearance": "light", "revision": "test", "highlights": {"string": {"fg": "#22863a"}}})

      temp_file = Path.join(tmp_dir, "test_theme.json")
      File.write!(temp_file, json_content)

      assert {:ok,
              %Theme{
                name: "file_theme",
                appearance: :light,
                revision: "test",
                highlights: highlights
              }} =
               Theme.from_file(temp_file)

      assert %Theme.Style{fg: "#22863a"} = highlights["string"]
    end

    test "returns error for non-existent file" do
      assert {:error, :invalid_theme_file} = Theme.from_file("/non/existent/file.json")
    end

    @tag :tmp_dir
    test "returns error for invalid JSON file", %{tmp_dir: tmp_dir} do
      temp_file = Path.join(tmp_dir, "invalid_theme.json")
      File.write!(temp_file, "invalid json content")

      assert {:error, :invalid_theme_file} = Theme.from_file(temp_file)
    end
  end
end

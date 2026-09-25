defmodule Lumis.ConformanceTest do
  use ExUnit.Case, async: true
  @moduletag :conformance

  @conformance_dir Path.expand("../../../../fixtures/conformance", __DIR__)

  @fixture_names @conformance_dir
                 |> Path.join("*/fixture.json")
                 |> Path.wildcard()
                 |> Enum.map(&Path.dirname/1)
                 |> Enum.map(&Path.basename/1)
                 |> Enum.sort()

  # The same eleven the JavaScript conformance suite declares. Highlighting would
  # load them anyway; naming them keeps this suite comparing formatter output
  # rather than measuring a first load, and matches what the browser has to do,
  # since it cannot load inside the walk.
  setup_all do
    :ok =
      Lumis.Languages.load(~w(
        json diff elixir html javascript css lua markdown markdown_inline mdx python
      ))

    :ok
  end

  defp load_fixture(name) do
    dir = Path.join(@conformance_dir, name)
    metadata = dir |> Path.join("fixture.json") |> File.read!() |> Jason.decode!()
    metadata = Map.put(metadata, "htmlMultiThemesOptions", metadata["htmlMultiThemes"])

    Map.merge(metadata, %{
      "source" => File.read!(Path.join(dir, "source.txt")),
      "htmlInline" => File.read!(Path.join(dir, "html-inline.html")),
      "htmlLinked" => File.read!(Path.join(dir, "html-linked.html")),
      "htmlMultiThemes" => File.read!(Path.join(dir, "html-multi-themes.html")),
      "bbcode" => File.read!(Path.join(dir, "bbcode.txt")),
      "terminal" => File.read!(Path.join(dir, "terminal.txt"))
    })
  end

  defp rainbow_brackets(fixture), do: fixture["rainbowBrackets"] || false

  defp serialize_event({:start, %{scope: scope, language: language}}),
    do: %{"type" => "start", "scope" => scope, "language" => language}

  defp serialize_event({:source, %{start: start, end: stop}}),
    do: %{"type" => "source", "start" => start, "end" => stop}

  defp serialize_event(:end), do: %{"type" => "end"}

  defp serialize_event({:decoration_start, %Lumis.Decoration.RainbowBracket{depth: depth}}),
    do: %{
      "type" => "decorationStart",
      "decoration" => %{"type" => "rainbowBracket", "depth" => depth}
    }

  defp serialize_event(:decoration_end), do: %{"type" => "decorationEnd"}

  defp conformance_theme(name) do
    path = Path.expand("../../../../fixtures/conformance-themes/#{name}.json", __DIR__)

    case File.read(path) do
      {:ok, json} ->
        {:ok, theme} = Lumis.Theme.from_json(json)
        theme

      {:error, :enoent} ->
        name
    end
  end

  defp html_multi_themes_options(fixture) do
    config =
      fixture["htmlMultiThemesOptions"] ||
        %{
          "themes" => %{"main" => fixture["theme"]},
          "defaultTheme" => "main"
        }

    # Reverse-sorted on purpose: the formatter sorts theme names itself, so
    # output must not depend on the order they were given in.
    themes =
      config["themes"]
      |> Enum.sort(:desc)
      |> Enum.map(fn {name, theme} -> {String.to_atom(name), conformance_theme(theme)} end)

    highlight_lines =
      case config["highlightLines"] do
        lines when is_list(lines) and lines != [] -> %{lines: lines, style: :theme}
        _ -> nil
      end

    [
      language: fixture["language"],
      themes: themes,
      default_theme: config["defaultTheme"],
      highlight_lines: highlight_lines
    ]
  end

  for name <- @fixture_names do
    describe name do
      @tag fixture: name
      test "events" do
        fixture = load_fixture(unquote(name))

        events =
          Lumis.highlight_events!(fixture["source"], fixture["language"],
            rainbow_brackets: rainbow_brackets(fixture)
          )

        assert Enum.map(events, &serialize_event/1) == fixture["events"]
      end

      @tag fixture: name
      test "html_inline" do
        fixture = load_fixture(unquote(name))

        assert Lumis.highlight!(fixture["source"],
                 formatter: {
                   :html_inline,
                   language: fixture["language"], theme: fixture["theme"]
                 },
                 rainbow_brackets: rainbow_brackets(fixture)
               ) == fixture["htmlInline"]
      end

      @tag fixture: name
      test "html_linked" do
        fixture = load_fixture(unquote(name))

        assert Lumis.highlight!(fixture["source"],
                 formatter: {:html_linked, language: fixture["language"]},
                 rainbow_brackets: rainbow_brackets(fixture)
               ) == fixture["htmlLinked"]
      end

      @tag fixture: name
      test "html_multi_themes" do
        fixture = load_fixture(unquote(name))

        assert Lumis.highlight!(fixture["source"],
                 formatter: {
                   :html_multi_themes,
                   html_multi_themes_options(fixture)
                 },
                 rainbow_brackets: rainbow_brackets(fixture)
               ) == fixture["htmlMultiThemes"]
      end

      @tag fixture: name
      test "terminal" do
        fixture = load_fixture(unquote(name))

        assert Lumis.highlight!(fixture["source"],
                 formatter: {
                   :terminal,
                   language: fixture["language"], theme: fixture["theme"]
                 },
                 rainbow_brackets: rainbow_brackets(fixture)
               ) == fixture["terminal"]
      end

      @tag fixture: name
      test "bbcode_scoped" do
        fixture = load_fixture(unquote(name))

        assert Lumis.highlight!(fixture["source"],
                 formatter: {:bbcode_scoped, language: fixture["language"]},
                 rainbow_brackets: rainbow_brackets(fixture)
               ) == fixture["bbcode"]
      end
    end
  end
end

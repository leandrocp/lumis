defmodule Lumis.Formatter.ANSITest do
  use ExUnit.Case, async: true

  doctest Lumis.Formatter.ANSI

  alias Lumis.Formatter.ANSI
  alias Lumis.Theme
  alias Lumis.Theme.Style
  alias Lumis.Theme.TextDecoration

  @source "defmodule App do\n  @doc \"hello\"\n  def run, do: :ok\nend\n"

  defmodule CaptureFormatter do
    @moduledoc false
    @behaviour Lumis.Formatter

    @impl true
    def render(_source, events, options) do
      send(Keyword.fetch!(options, :owner), {:events, events})
      []
    end
  end

  defmodule HelperFormatter do
    @moduledoc false
    @behaviour Lumis.Formatter

    alias Lumis.Formatter.ANSI

    @impl true
    def render(source, events, options) do
      theme = Keyword.fetch!(options, :theme)

      {parts, _scopes, _tables} =
        Enum.reduce(events, {[], [], %{}}, fn
          {:start, %{scope: scope, language: language}}, {parts, scopes, tables} ->
            {parts, [{scope, language} | scopes], tables}

          :end, {parts, [_scope | scopes], tables} ->
            {parts, scopes, tables}

          {:source, %{start: start, end: stop}}, {parts, scopes, tables} ->
            text = binary_part(source, start, stop - start)
            {painted, tables} = paint(text, scopes, theme, tables)
            {[painted | parts], scopes, tables}

          _event, state ->
            state
        end)

      parts |> Enum.reverse() |> IO.iodata_to_binary()
    end

    defp paint(text, [], _theme, tables), do: {text, tables}

    defp paint(text, [{scope, language} | _scopes], theme, tables) do
      tables =
        Map.put_new_lazy(tables, language, fn ->
          ANSI.styles(theme: theme, language: language)
        end)

      {ANSI.paint(text, ANSI.style_for(tables[language], scope)), tables}
    end
  end

  # A theme, a language, and source whose scopes that theme does not style
  # directly, so resolution has to fall back the way `:terminal` falls back.
  @terminal_parity_cases [
    {"elixir", "dracula", @source},
    {"html", "github_light", ~s|<div class="a">t</div>\n|},
    {"markdown", "catppuccin_frappe", "# T\n\n> q\n\n```elixir\ndef run, do: :ok\n```\n"}
  ]

  setup_all do
    for {language, _theme, _source} <- @terminal_parity_cases do
      :ok = Lumis.Languages.load(language)
    end

    :ok
  end

  test "hex_to_rgb/1 and rgb_to_ansi/4 compose into style_to_ansi/1" do
    assert {255, 121, 198} = rgb = ANSI.hex_to_rgb("#ff79c6")
    assert rgb == ANSI.hex_to_rgb("ff79c6")
    assert ANSI.hex_to_rgb("fff") == nil
    assert ANSI.hex_to_rgb("not-a-color") == nil
    assert ANSI.hex_to_rgb("aéaaa") == nil
    # Six digits is the whole string, not a prefix of it.
    assert ANSI.hex_to_rgb("ff79cz") == nil
    assert ANSI.hex_to_rgb("ff+79c") == nil

    {r, g, b} = rgb
    assert ANSI.style_to_ansi(%Style{fg: "#ff79c6"}) == ANSI.rgb_to_ansi(r, g, b, false)
    assert ANSI.style_to_ansi(%Style{bg: "#ff79c6"}) == ANSI.rgb_to_ansi(r, g, b, true)
  end

  test "reset/0 is the reset paint and the terminal formatter emit" do
    style = %Style{bold: true}
    painted = ANSI.paint("text", style)
    scopes = for {:start, %{scope: scope}} <- events(@source, "elixir"), uniq: true, do: scope
    terminal = terminal_output(@source, theme_for(scopes, style), "elixir")

    assert String.starts_with?(painted, ANSI.reset())
    assert String.ends_with?(painted, ANSI.reset())
    assert String.contains?(terminal, ANSI.reset())
  end

  test "paint/2 and style_to_ansi/1 reproduce the built-in terminal formatter" do
    events = events(@source, "elixir")
    scopes = for {:start, %{scope: scope}} <- events, uniq: true, do: scope

    styles = [
      %Style{fg: "#ff79c6"},
      %Style{bg: "#282a36"},
      %Style{bold: true},
      %Style{italic: true},
      %Style{text_decoration: %TextDecoration{underline: :solid}},
      %Style{text_decoration: %TextDecoration{underline: :wavy}},
      %Style{text_decoration: %TextDecoration{underline: :double}},
      %Style{text_decoration: %TextDecoration{underline: :dotted}},
      %Style{text_decoration: %TextDecoration{underline: :dashed}},
      %Style{text_decoration: %TextDecoration{strikethrough: true}},
      %Style{
        fg: "#ff79c6",
        bg: "#282a36",
        bold: true,
        italic: true,
        text_decoration: %TextDecoration{underline: :wavy, strikethrough: true}
      }
    ]

    for style <- styles do
      theme = theme_for(scopes, style)

      helper_output =
        Lumis.highlight!(@source,
          formatter: {HelperFormatter, language: "elixir", theme: theme}
        )

      assert helper_output == terminal_output(@source, theme, "elixir")
      assert ANSI.style_to_ansi(style) != ""
      assert String.contains?(ANSI.paint("token", style), ANSI.style_to_ansi(style))
    end
  end

  test "paint/2 leaves text unchanged for an empty style" do
    assert ANSI.paint("plain text", %Style{}) == "plain text"
    assert ANSI.paint("plain text", nil) == "plain text"
    assert ANSI.style_to_ansi(%Style{}) == ""
  end

  test "styles/1 resolves a scope to its parent, which theme.highlights alone does not" do
    theme = Theme.get("github_light")
    styles = ANSI.styles(theme: theme, language: "html")

    assert Map.get(theme.highlights, "tag.delimiter") == nil
    assert Map.get(theme.highlights, "tag") != nil
    assert ANSI.style_for(styles, "tag.delimiter") == Map.get(theme.highlights, "tag")
  end

  test "styles/1 prefers a language's specialized scope" do
    style = %Style{fg: "#ff79c6"}
    specialized = %Style{fg: "#282a36"}

    theme = %Theme{
      theme_for(["comment"], style)
      | highlights: %{"comment" => style, "comment.elixir" => specialized}
    }

    assert ANSI.style_for(ANSI.styles(theme: theme, language: "elixir"), "comment") == specialized
    assert ANSI.style_for(ANSI.styles(theme: theme, language: "rust"), "comment") == style
  end

  test "styles/1 without a theme paints nothing" do
    styles = ANSI.styles(language: "elixir")

    assert styles == %{}
    assert ANSI.style_for(styles, "keyword") == nil
    assert ANSI.style_for(styles, nil) == nil
    assert ANSI.paint("def", ANSI.style_for(styles, "keyword")) == "def"
  end

  test "styles/1 accepts a theme name" do
    assert ANSI.styles(theme: "dracula", language: "elixir") ==
             ANSI.styles(theme: Theme.get("dracula"), language: "elixir")
  end

  test "a formatter built on styles/1 reproduces :terminal for real themes" do
    for {language, theme_name, source} <- @terminal_parity_cases do
      theme = Theme.get(theme_name)

      helper_output =
        Lumis.highlight!(source, formatter: {HelperFormatter, language: language, theme: theme})

      assert helper_output == terminal_output(source, theme, language),
             "#{language} highlighted with #{theme_name} diverges from the :terminal formatter"
    end
  end

  defp events(source, language) do
    Lumis.highlight!(source,
      formatter: {CaptureFormatter, language: language, owner: self()}
    )

    receive do
      {:events, events} -> events
    after
      1_000 -> flunk("the formatter was never handed an event stream")
    end
  end

  defp theme_for(scopes, style) do
    %Theme{
      name: "ansi-helper-test",
      appearance: :dark,
      revision: "test",
      highlights: Map.new(scopes, &{&1, style})
    }
  end

  defp terminal_output(source, theme, language) do
    Lumis.highlight!(source, formatter: {:terminal, language: language, theme: theme})
  end
end

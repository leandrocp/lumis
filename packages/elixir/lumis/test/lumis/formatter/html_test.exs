defmodule Lumis.Formatter.HTMLTest do
  use ExUnit.Case, async: true

  alias Lumis.Formatter.HTML

  # The only public way to see an event stream is to be handed one, so this
  # captures it the way any formatter would.
  defmodule CaptureFormatter do
    @moduledoc false
    @behaviour Lumis.Formatter

    @impl true
    def render(_source, events, options) do
      send(Keyword.fetch!(options, :owner), {:events, events})
      []
    end
  end

  defp events(source, language) do
    Lumis.highlight!(source, formatter: {CaptureFormatter, language: language, owner: self()})

    receive do
      {:events, events} -> events
    after
      1_000 -> flunk("the formatter was never handed an event stream")
    end
  end

  # These helpers exist so an Elixir formatter stops hand-rolling its own, so
  # every test that could drift compares against what the built-in Rust
  # formatters actually emit rather than against a literal written here.
  @sources [
    {"defmodule App do\n  @doc \"hi & bye, it's fine\"\n  def go, do: :ok\nend\n", "elixir"},
    {"# Title\n\n```elixir\n:ok\n```\n", "markdown"},
    {"<p class=\"x\">a & b < c</p>\n", "html"},
    {"const x = `a ${b} c`\nconst y = '<>'\n", "javascript"},
    {"just words, no parser\n", "plaintext"},
    {"", "elixir"}
  ]

  @themes ["github_light", "dracula"]

  setup_all do
    :ok = Lumis.Languages.load(~w(elixir markdown markdown_inline html javascript))
    :ok
  end

  defp html_linked(source, language) do
    Lumis.highlight!(source, formatter: {:html_linked, language: language})
  end

  defp html_inline(source, language, theme, options) do
    Lumis.highlight!(
      source,
      formatter: {:html_inline, [language: language, theme: theme] ++ options}
    )
  end

  describe "escape/1" do
    test "escapes the five entities lumis_core::formatter::html::escape does" do
      assert HTML.escape(~s|&<>"'|) == "&amp;&lt;&gt;&quot;&#39;"
    end

    test "escapes an ampersand once" do
      assert HTML.escape("&lt;") == "&amp;lt;"
    end

    test "matches the escaping html_linked applies to the same text" do
      # `'` is the one the hand-rolled copy in examples/annotations.exs missed.
      for {source, language} <- @sources, source != "" do
        html = html_linked(source, language)

        for fragment <- ~w(& < > "), String.contains?(source, fragment) do
          assert String.contains?(html, HTML.escape(fragment)),
                 "#{language}: html_linked does not contain #{inspect(HTML.escape(fragment))}"
        end
      end
    end

    test "leaves braces alone" do
      assert HTML.escape("%{a: 1}") == "%{a: 1}"
    end
  end

  describe "escape_braces/1" do
    test "escapes both braces" do
      assert HTML.escape_braces("%{a: 1}") == "%&lbrace;a: 1&rbrace;"
    end
  end

  describe "scope_to_class/1" do
    test "every class html_linked emits is one scope_to_class produces" do
      known = MapSet.new(Map.values(HTML.classes()))

      for {source, language} <- @sources do
        source
        |> html_linked(language)
        |> then(&Regex.scan(~r/<span class="([^"]+)"/, &1))
        |> Enum.each(fn [_, class] ->
          assert MapSet.member?(known, class),
                 "#{language}: html_linked emits #{inspect(class)}, which is in no scope's class"
        end)
      end
    end

    test "resolves the scopes an event stream actually carries" do
      {source, language} = hd(@sources)

      source
      |> events(language)
      |> Enum.each(fn
        {:start, %{scope: scope}} ->
          assert HTML.scope_to_class(scope) != "l-text",
                 "#{scope} fell back to l-text"

        _event ->
          :ok
      end)
    end

    test "an unknown scope falls back to l-text, which a hand-rolled copy cannot express" do
      assert HTML.scope_to_class("not.a.scope") == "l-text"
      assert HTML.scope_to_class(nil) == "l-text"
    end
  end

  describe "tags" do
    test "open_pre_tag/1 appends a class" do
      assert HTML.open_pre_tag() == ~s|<pre class="lumis">|
      assert HTML.open_pre_tag(class: "mine") == ~s|<pre class="lumis mine">|
    end

    test "open_pre_tag/1 with a theme is what html_inline opens with" do
      for theme <- @themes, {source, language} <- @sources do
        assert String.starts_with?(
                 html_inline(source, language, theme, []),
                 HTML.open_pre_tag(theme: theme)
               ),
               "#{theme}/#{language} opened differently"
      end
    end

    test "open_pre_tag/1 without a theme is what html_linked opens with" do
      for {source, language} <- @sources do
        assert String.starts_with?(html_linked(source, language), HTML.open_pre_tag())
      end
    end

    test "open_code_tag/1 is what the built-in formatters open with" do
      for {source, language} <- @sources do
        assert String.contains?(html_linked(source, language), HTML.open_code_tag(language))
      end
    end

    test "closing_tags/0 is what the built-in formatters close with" do
      assert HTML.closing_tags() == "</code></pre>"

      for {source, language} <- @sources do
        assert String.ends_with?(html_linked(source, language), HTML.closing_tags())
      end
    end
  end

  describe "wrap_line/3" do
    test "numbers lines from one" do
      assert HTML.wrap_line(2, "code") == ~s|<div class="l-line" data-line="2">code</div>|
    end

    test "appends a class suffix and a style" do
      assert HTML.wrap_line(1, "x", class_suffix: " l-highlighted", style: "color: red;") ==
               ~s|<div class="l-line l-highlighted" style="color: red;" data-line="1">x</div>|
    end

    test "produces the line wrappers html_linked emits" do
      for {source, language} <- @sources do
        html = html_linked(source, language)

        html
        |> then(&Regex.scan(~r|<div class="l-line" data-line="(\d+)">|, &1))
        |> Enum.each(fn [opening, number] ->
          assert String.starts_with?(HTML.wrap_line(String.to_integer(number), ""), opening),
                 "#{language}: wrap_line does not produce #{inspect(opening)}"
        end)
      end
    end
  end

  describe "span_attrs/1 and open_span/2" do
    test "every span html_inline opens is one open_span produces" do
      for theme <- @themes,
          {source, language} <- @sources,
          italic <- [false, true],
          include_highlights <- [false, true] do
        html =
          html_inline(source, language, theme,
            italic: italic,
            include_highlights: include_highlights
          )

        attrs =
          HTML.span_attrs(
            theme: theme,
            language: language,
            italic: italic,
            include_highlights: include_highlights
          )

        html
        |> then(&Regex.scan(~r|(<span[^>]*>)|, &1))
        |> Enum.map(fn [_, tag] -> tag end)
        |> Enum.uniq()
        |> Enum.each(fn tag ->
          scope = scope_of(tag, attrs)

          assert scope, "#{theme}/#{language}: no scope produces #{inspect(tag)}"
        end)
      end
    end

    test "a scope the theme does not style opens a bare span" do
      attrs = HTML.span_attrs(theme: "github_light", language: "elixir")

      assert HTML.open_span(attrs, nil) == "<span>"
      assert HTML.open_span(attrs, "not.a.scope") == "<span>"
    end

    test "without a theme every scope is unstyled" do
      attrs = HTML.span_attrs(language: "elixir")

      assert HTML.open_span(attrs, "keyword.function") == "<span>"
    end

    test "include_highlights adds the scope html_inline puts in data-highlight" do
      attrs = HTML.span_attrs(theme: "github_light", language: "elixir", include_highlights: true)

      assert HTML.open_span(attrs, "keyword.function") =~ ~s|data-highlight="keyword.function"|
    end

    test "span_inline/3 escapes the text" do
      attrs = HTML.span_attrs(language: "elixir")

      assert HTML.span_inline("a & b", attrs, "keyword") == "<span>a &amp; b</span>"
    end
  end

  describe "span_linked/2" do
    test "is the span html_linked emits for the same scope" do
      for {source, language} <- @sources, source != "" do
        html = html_linked(source, language)

        source
        |> events(language)
        |> Enum.flat_map(fn
          {:start, %{scope: scope}} -> [scope]
          _event -> []
        end)
        |> Enum.uniq()
        |> Enum.each(fn scope ->
          assert String.contains?(html, "<span #{HTML.span_linked_attrs(scope)}>"),
                 "#{language}: html_linked never opens <span #{HTML.span_linked_attrs(scope)}>"
        end)
      end
    end
  end

  # A `<span ...>` from html_inline is reproduced by `open_span/2` for exactly
  # one scope; find it, or report that none matches.
  defp scope_of(tag, attrs) do
    Enum.find_value(Map.keys(attrs), fn scope ->
      if HTML.open_span(attrs, scope) == tag, do: scope
    end) || if tag == "<span>", do: :unstyled
  end
end

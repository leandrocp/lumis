defmodule Lumis.Formatter.HTMLTest do
  use ExUnit.Case, async: true

  alias Lumis.Formatter.HTML

  doctest Lumis.Formatter.HTML

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

  describe "escape_attr/1" do
    test "escapes the same five entities escape/1 does" do
      assert HTML.escape_attr(~s|&<>"'|) == "&amp;&lt;&gt;&quot;&#39;"
    end

    test "closes an injection through a caller-supplied class" do
      assert HTML.open_pre_tag(class: ~s|x"><script>|) ==
               ~s|<pre class="lumis #{HTML.escape_attr(~s|x"><script>|)}">|
    end

    test "leaves quoted CSS readable after the parser decodes it" do
      assert HTML.escape_attr("font-family: 'Fira Code';") == "font-family: &#39;Fira Code&#39;;"
    end
  end

  describe "sanitize_theme_name/1" do
    test "keeps letters, digits, dash and underscore" do
      assert HTML.sanitize_theme_name("catppuccin_mocha-2") == "catppuccin_mocha-2"
    end

    test "replaces everything else with a dash, keeping non-ASCII letters" do
      assert HTML.sanitize_theme_name("Rosé Pine (Dawn)") == "Rosé-Pine--Dawn-"
    end

    test "is the form a theme name takes in the CSS variables span_multi_themes_attrs writes" do
      attrs =
        HTML.span_multi_themes_attrs(themes: [{:"my theme", "github_light"}], language: "elixir")

      assert attrs["keyword"] =~ "--lumis-#{HTML.sanitize_theme_name("my theme")}:"
    end
  end

  describe "text_decoration/1 and style_to_css/2" do
    test "text_decoration/1 covers underline styles and strikethrough" do
      assert HTML.text_decoration(%Lumis.Theme.TextDecoration{}) == "none"

      assert HTML.text_decoration(%Lumis.Theme.TextDecoration{strikethrough: true}) ==
               "line-through"

      assert HTML.text_decoration(%Lumis.Theme.TextDecoration{underline: :dotted}) ==
               "underline dotted"

      assert HTML.text_decoration(%Lumis.Theme.TextDecoration{
               underline: :double,
               strikethrough: true
             }) == "underline double line-through"
    end

    test "style_to_css/2 writes the declarations span_attrs puts in a style attribute" do
      theme = Lumis.Theme.get("dracula")
      attrs = HTML.span_attrs(theme: theme, language: "elixir")
      style = theme.highlights["keyword"]

      assert HTML.open_span(attrs, "keyword") =~ HTML.style_to_css(style)
    end

    test "style_to_css/2 honours :italic and :separator" do
      style = %Lumis.Theme.Style{fg: "#ff79c6", italic: true}

      assert HTML.style_to_css(style) == "color: #ff79c6;"
      assert HTML.style_to_css(style, italic: true) == "color: #ff79c6; font-style: italic;"

      assert HTML.style_to_css(style, italic: true, separator: "\n") ==
               "color: #ff79c6;\nfont-style: italic;"
    end
  end

  describe "close_pre_tag/0 and close_code_tag/0" do
    test "together they are closing_tags/0" do
      assert HTML.close_code_tag() <> HTML.close_pre_tag() == HTML.closing_tags()
    end
  end

  describe "span_multi_themes_attrs/1 and open_multi_themes_pre_tag/1" do
    @themes_option [light: "github_light", dark: "dracula"]

    defp html_multi_themes(source, language, options) do
      Lumis.highlight!(
        source,
        formatter: {:html_multi_themes, [language: language, themes: @themes_option] ++ options}
      )
    end

    test "opens the <pre> html_multi_themes opens" do
      for default_theme <- [nil, "light", "light-dark()"],
          {source, language} <- @sources do
        options = if default_theme, do: [default_theme: default_theme], else: []

        assert String.starts_with?(
                 html_multi_themes(source, language, options),
                 HTML.open_multi_themes_pre_tag([themes: @themes_option] ++ options)
               ),
               "#{inspect(default_theme)}/#{language} opened differently"
      end
    end

    test "every span html_multi_themes opens is one span_multi_themes_attrs produces" do
      for default_theme <- [nil, "light", "light-dark()"],
          {source, language} <- @sources,
          source != "" do
        options = if default_theme, do: [default_theme: default_theme], else: []

        attrs =
          HTML.span_multi_themes_attrs([themes: @themes_option, language: language] ++ options)

        html_multi_themes(source, language, options)
        |> then(&Regex.scan(~r|(<span[^>]*>)|, &1))
        |> Enum.map(fn [_, tag] -> tag end)
        |> Enum.uniq()
        |> Enum.each(fn tag ->
          assert scope_of(tag, attrs),
                 "#{inspect(default_theme)}/#{language}: no scope produces #{inspect(tag)}"
        end)
      end
    end

    test "span_multi_themes/3 escapes the text and opens with the table's attributes" do
      attrs = HTML.span_multi_themes_attrs(themes: @themes_option, language: "elixir")

      assert HTML.span_multi_themes("a & b", attrs, "keyword") ==
               "#{HTML.open_span(attrs, "keyword")}a &amp; b</span>"
    end

    test "an explicit nil CSS variable prefix uses the default" do
      assert HTML.span_multi_themes_attrs(themes: @themes_option, css_variable_prefix: nil) ==
               HTML.span_multi_themes_attrs(themes: @themes_option)

      assert HTML.open_multi_themes_pre_tag(themes: @themes_option, css_variable_prefix: nil) ==
               HTML.open_multi_themes_pre_tag(themes: @themes_option)
    end

    test "no themes leaves every scope unstyled" do
      attrs = HTML.span_multi_themes_attrs(themes: [], language: "elixir")

      assert HTML.open_span(attrs, "keyword") == "<span>"
    end
  end

  describe "line_is_highlighted/2 and highlight_line_class/3" do
    test "reads integers and ranges the way :highlight_lines does" do
      lines = [1, 3..5]

      assert HTML.line_is_highlighted(lines, 1)
      assert HTML.line_is_highlighted(lines, 4)
      assert HTML.line_is_highlighted(lines, 5)
      refute HTML.line_is_highlighted(lines, 2)
      refute HTML.line_is_highlighted(lines, 6)
      refute HTML.line_is_highlighted([], 1)
    end

    test "highlight_line_class/3 prefers :class over :default_class" do
      lines = [1, 3..5]

      assert HTML.highlight_line_class(lines, 4, default_class: "l-highlighted") ==
               "l-highlighted"

      assert HTML.highlight_line_class(lines, 4, class: "active", default_class: "l-highlighted") ==
               "active"

      assert HTML.highlight_line_class(lines, 2, class: "active") == nil
      assert HTML.highlight_line_class(lines, 4, []) == nil
    end

    # The compact native representation has to preserve both pieces of Elixir
    # range semantics. Dropping the step would highlight lines 2 and 4 here;
    # dropping the direction would make the descending range match nothing.
    test "honours a range's step and direction" do
      assert HTML.line_is_highlighted([1..5//2], 1)
      refute HTML.line_is_highlighted([1..5//2], 2)
      assert HTML.line_is_highlighted([1..5//2], 3)
      refute HTML.line_is_highlighted([1..5//2], 4)
      assert HTML.line_is_highlighted([1..5//2], 5)

      assert HTML.line_is_highlighted([5..3//-1], 4)
      refute HTML.line_is_highlighted([5..3//-1], 2)

      assert HTML.line_is_highlighted([10..2//-3], 4)
      refute HTML.line_is_highlighted([10..2//-3], 2), "the last bound need not be selected"

      refute HTML.line_is_highlighted([1..0//1], 1), "an empty range highlights nothing"
      refute HTML.line_is_highlighted([3..5//-1], 4), "so does an empty descending range"
    end

    # Every range crosses as one compact arithmetic progression, regardless of
    # its direction, step, or declared span.
    test "a huge contiguous range costs nothing, whichever way round" do
      assert HTML.line_is_highlighted([1..1_000_000_000], 999_999_999)
      assert HTML.line_is_highlighted([1_000_000_000..1//-1], 999_999_999)
    end

    test "a huge stepped range stays compact" do
      assert {:ok, [{:range, %{start: 1, end: 1_000_000_000, step: 2}}]} =
               Lumis.LineSpec.encode([1..1_000_000_000//2])

      assert HTML.line_is_highlighted([1..1_000_000_000//2], 3)
      refute HTML.line_is_highlighted([1..1_000_000_000//2], 4)

      assert HTML.highlight_line_class([1..1_000_000_000//2], 3, default_class: "hl") == "hl"
    end

    test "the formatter clips a huge stepped range to its rendered lines" do
      html =
        Lumis.highlight!("one\ntwo\nthree",
          formatter:
            {:html_linked,
             language: "plaintext", highlight_lines: %{lines: [1..1_000_000_000//2], class: "hl"}}
        )

      assert html =~ ~s|class="l-line hl" data-line="1"|
      refute html =~ ~s|class="l-line hl" data-line="2"|
      assert html =~ ~s|class="l-line hl" data-line="3"|
    end

    test "picks the lines html_linked marks with its highlight class" do
      source = "one\ntwo\nthree\nfour\nfive\n"

      for lines <- [[1, 3..4], [1..5//2], [5..3//-1]] do
        html =
          Lumis.highlight!(source,
            formatter:
              {:html_linked, language: "plaintext", highlight_lines: %{lines: lines, class: "hl"}}
          )

        for line_number <- 1..5 do
          marked = String.contains?(html, ~s|class="l-line hl" data-line="#{line_number}"|)

          assert marked == HTML.line_is_highlighted(lines, line_number),
                 "#{inspect(lines)} line #{line_number}: html_linked and " <>
                   "line_is_highlighted/2 disagree"
        end
      end
    end
  end

  describe "render_lines_from_events/3" do
    test "renders the lines html_linked wraps, for every source" do
      linked_attrs =
        Map.new(HTML.classes(), fn {scope, _class} -> {scope, HTML.span_linked_attrs(scope)} end)

      for {source, language} <- @sources, source != "" do
        expected =
          source
          |> html_linked(language)
          |> then(&Regex.scan(~r|<div class="l-line" data-line="\d+">(.*?)</div>|s, &1))
          |> Enum.map(fn [_, content] -> String.trim_trailing(content, "\n") end)

        actual =
          HTML.render_lines_from_events(source, events(source, language), linked_attrs)

        assert actual == expected,
               "#{language}: render_lines_from_events differs from html_linked"
      end
    end

    test "reopens a span that crosses a newline" do
      events = [
        {:start, %{scope: "keyword", language: "elixir"}},
        {:source, %{start: 0, end: 3}},
        :end
      ]

      assert HTML.render_lines_from_events("a\nb", events, %{"keyword" => ~s|class="l-keyword"|}) ==
               [~s|<span class="l-keyword">a</span>|, ~s|<span class="l-keyword">b</span>|]
    end

    test "a scope the table does not carry opens a bare span" do
      events = [
        {:start, %{scope: "not.a.scope", language: "elixir"}},
        {:source, %{start: 0, end: 1}},
        :end
      ]

      assert HTML.render_lines_from_events("a", events, %{}) == ["<span>a</span>"]
    end

    test "skips an event kind it has no markup for rather than raising" do
      events = [
        {:annotation_start, %Lumis.Annotation{range: {0, 1}, data: %{anything: true}}},
        {:source, %{start: 0, end: 1}},
        :annotation_end,
        {:something_a_newer_lumis_adds, %{}}
      ]

      assert HTML.render_lines_from_events("a", events, %{}) == ["a"]
    end

    test "escapes the source it renders" do
      events = [{:source, %{start: 0, end: 5}}]

      assert HTML.render_lines_from_events("a & b", events, %{}) == ["a &amp; b"]
    end

    # A formatter can build its own events rather than replaying the ones it was
    # handed, so these offsets are caller data. An offset landing inside a
    # multi-byte character used to panic the NIF.
    test "takes offsets that split a character without blowing up" do
      assert HTML.render_lines_from_events("é", [{:source, %{start: 0, end: 1}}], %{}) == [""]
      assert HTML.render_lines_from_events("éx", [{:source, %{start: 1, end: 3}}], %{}) == ["x"]
      assert HTML.render_lines_from_events("é", [{:source, %{start: 0, end: 2}}], %{}) == ["é"]
    end

    test "takes offsets that are out of range or reversed" do
      assert HTML.render_lines_from_events("ab", [{:source, %{start: 0, end: 99}}], %{}) == ["ab"]
      assert HTML.render_lines_from_events("ab", [{:source, %{start: 99, end: 99}}], %{}) == [""]
      assert HTML.render_lines_from_events("ab", [{:source, %{start: 2, end: 0}}], %{}) == [""]
    end
  end

  # The point of the whole module: a formatter assembled from the helpers is the
  # built-in one, byte for byte. This is the formatter #1381 said could not be
  # written in Elixir at all.
  defmodule MultiThemeFormatter do
    @moduledoc false
    @behaviour Lumis.Formatter

    alias Lumis.Formatter.HTML

    @themes [light: "github_light", dark: "dracula"]

    @impl true
    def render(source, events, options) do
      language = Keyword.fetch!(options, :language)
      default_theme = Keyword.fetch!(options, :default_theme)

      attrs =
        HTML.span_multi_themes_attrs(
          themes: @themes,
          default_theme: default_theme,
          language: language
        )

      body =
        source
        |> HTML.render_lines_from_events(events, attrs)
        |> Enum.with_index(1)
        |> Enum.map(fn {line, number} -> HTML.wrap_line(number, [line, "\n"]) end)

      [
        HTML.open_multi_themes_pre_tag(themes: @themes, default_theme: default_theme),
        HTML.open_code_tag(language),
        body,
        HTML.closing_tags()
      ]
    end
  end

  describe "a formatter built from the helpers" do
    test "reproduces :html_multi_themes byte for byte" do
      for default_theme <- [nil, "light", "light-dark()"],
          {source, language} <- @sources,
          source != "" do
        options = if default_theme, do: [default_theme: default_theme], else: []

        builtin =
          Lumis.highlight!(
            source,
            formatter:
              {:html_multi_themes,
               [language: language, themes: [light: "github_light", dark: "dracula"]] ++ options}
          )

        mine =
          Lumis.highlight!(
            source,
            formatter: {MultiThemeFormatter, language: language, default_theme: default_theme}
          )

        assert mine == builtin,
               "#{inspect(default_theme)}/#{language}: the helper-built formatter differs"
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

defmodule Lumis.LumisTest do
  use ExUnit.Case, async: true
  import ExUnit.CaptureIO

  defp assert_output(source, expected, opts) do
    result = Lumis.highlight!(source, opts)
    # IO.puts(result)
    assert String.trim(result) == String.trim(expected)
  end

  defp assert_contains(source, expected, opts) do
    result =
      source
      |> Lumis.highlight!(opts)
      |> String.trim()

    # IO.puts(result)

    assert String.contains?(result, expected)
  end

  describe "deprecated still works" do
    test "highlight" do
      capture_io(:stderr, fn ->
        assert {:ok, hl} = Lumis.highlight("elixir", ":test", theme: "onedark")

        assert hl =~
                 ~s|<pre class="lumis" style="color: #abb2bf; background-color: #282c34;"><code class="language-elixir"|

        assert {:ok, hl} = Lumis.highlight("elixir", ":test", theme: "dracula")

        assert hl =~
                 ~s|<pre class="lumis" style="color: #f8f8f2; background-color: #282a36;"><code class="language-elixir"|
      end)
    end

    test "top-level language option warns" do
      warning =
        capture_io(:stderr, fn ->
          assert {:ok, _highlighted} = Lumis.highlight(":test", language: "elixir")
        end)

      assert warning =~ ":language option is deprecated"
    end

    test "deprecated highlight/3 does not double-warn about :language" do
      warning =
        capture_io(:stderr, fn ->
          assert {:ok, _highlighted} = Lumis.highlight("elixir", ":test", [])
        end)

      refute warning =~ ":language option is deprecated"
    end

    test "highlight!" do
      capture_io(:stderr, fn ->
        assert Lumis.highlight!("elixir", ":test", theme: "onedark") =~
                 ~s|<pre class="lumis" style="color: #abb2bf; background-color: #282c34;"><code class="language-elixir"|

        assert Lumis.highlight!("elixir", ":test", theme: "dracula") =~
                 ~s|<pre class="lumis" style="color: #f8f8f2; background-color: #282a36;"><code class="language-elixir"|
      end)
    end

    test "theme option" do
      capture_io(:stderr, fn ->
        assert {:ok, highlighted} =
                 Lumis.highlight(":test", language: "elixir", theme: "github_light")

        assert highlighted =~
                 ~s|<pre class="lumis" style="color: #1f2328; background-color: #ffffff;">|
      end)
    end

    test "inline_style option" do
      capture_io(:stderr, fn ->
        assert {:ok,
                "<pre class=\"lumis\" style=\"color: #abb2bf; background-color: #282c34;\"><code class=\"language-elixir\" translate=\"no\" tabindex=\"0\"><div class=\"l-line\" data-line=\"1\"><span style=\"color: #e06c75;\">:test</span>\n</div></code></pre>"} =
                 Lumis.highlight(":test",
                   language: "elixir",
                   theme: "onedark",
                   inline_style: true
                 )
      end)
    end

    test "pre_class option" do
      capture_io(:stderr, fn ->
        assert {:ok, highlighted} =
                 Lumis.highlight(":test", language: "elixir", pre_class: "deprecated")

        assert highlighted =~ ~s|<pre class="lumis deprecated"|
      end)
    end
  end

  test "available_languages" do
    available_languages = Lumis.available_languages()
    by_id = Map.new(available_languages, &{&1.id, &1})

    assert length(available_languages) >= 77
    assert available_languages == Enum.sort_by(available_languages, & &1.id)

    assert by_id["elixir"] == %{
             id: "elixir",
             name: "Elixir",
             aliases: [],
             extensions: ["*.ex", "*.exs"],
             globs: ["*.ex", "*.exs"],
             emacs_modes: ["elixir"],
             shebangs: ["elixir"]
           }

    assert by_id["comment"].name == "Comment"
    assert by_id["comment"].globs == []
    assert by_id["markdown_inline"].name == "Markdown Inline"

    assert by_id["plaintext"] == %{
             id: "plaintext",
             name: "Plain Text",
             aliases: ["text", "txt", "plain"],
             extensions: [],
             globs: [],
             emacs_modes: ["fundamental", "text"],
             shebangs: []
           }

    # A language's own id is not repeated in its aliases.
    assert by_id["asm"].aliases == ["assembly"]

    # `extensions` is the subset of `globs` that name a bare extension.
    assert "PKGBUILD" in by_id["bash"].globs
    refute "PKGBUILD" in by_id["bash"].extensions
  end

  test "available_themes" do
    expected_count =
      __DIR__
      |> Path.join("../../../../themes")
      |> File.ls!()
      |> Enum.count(&String.ends_with?(&1, ".json"))

    themes = Lumis.available_themes()

    assert length(themes) == expected_count
    assert themes == Enum.sort_by(themes, & &1.name)
    assert %{name: "github_light", appearance: "light"} in themes
    assert %{name: "dracula", appearance: "dark"} in themes
  end

  describe "Theme.build_css" do
    test "builds default linked CSS from a theme name" do
      assert {:ok, css} = Lumis.Theme.build_css("github_light")

      assert css =~ ".lumis {"
      assert css =~ ".l-keyword {"
      assert css =~ "color: #1f2328;"
    end

    test "builds scoped CSS with container style overrides" do
      css =
        Lumis.Theme.build_css!("github_dark",
          scope: ~s(html[data-theme="dark"]),
          container_selector: ".lumis",
          container_style: [
            {"background-color", "var(--code-background)"},
            {"border-radius", "0.375rem"},
            {"padding", "1rem"}
          ]
        )

      assert css =~ ~s(html[data-theme="dark"] .lumis {)
      assert css =~ ~s|background-color: var(--code-background);|
      assert css =~ ~s(border-radius: 0.375rem;)
      assert css =~ ~s(html[data-theme="dark"] .l-keyword {)
    end

    test "returns error for unknown theme names" do
      assert Lumis.Theme.build_css("unknown_theme") == {:error, :not_found}
    end

    test "returns error for invalid options" do
      assert {:error, %NimbleOptions.ValidationError{}} =
               Lumis.Theme.build_css("github_light", scope: 1)
    end
  end

  test "default_options/0" do
    assert [formatter: {:html_inline, formatter_opts}] =
             Lumis.default_options()

    assert Keyword.equal?(
             [
               language: nil,
               header: nil,
               italic: false,
               theme: nil,
               pre_class: nil,
               include_highlights: false,
               rainbow_brackets: false,
               highlight_lines: nil
             ],
             formatter_opts
           )
  end

  describe "formatter_type: :html_inline" do
    test "default opts" do
      assert {:ok, {:html_inline, formatter_opts}} = Lumis.formatter_type(:html_inline)

      assert Keyword.equal?(
               [
                 language: nil,
                 italic: false,
                 theme: nil,
                 pre_class: nil,
                 include_highlights: false,
                 rainbow_brackets: false,
                 highlight_lines: nil,
                 header: nil
               ],
               formatter_opts
             )
    end
  end

  describe "formatter_type: :html_linked" do
    test "default opts" do
      assert {:ok, {:html_linked, formatter_opts}} = Lumis.formatter_type(:html_linked)

      assert Keyword.equal?(
               [
                 language: nil,
                 pre_class: nil,
                 rainbow_brackets: false,
                 highlight_lines: nil,
                 header: nil
               ],
               formatter_opts
             )
    end
  end

  describe "formatter_type: :terminal" do
    test "default opts" do
      assert {:ok, {:terminal, formatter_opts}} = Lumis.formatter_type(:terminal)

      assert Keyword.equal?(
               [
                 language: nil,
                 theme: nil,
                 background: nil,
                 width: nil,
                 rainbow_brackets: false
               ],
               formatter_opts
             )
    end
  end

  describe "formatter_type: :bbcode_scoped" do
    test "default opts" do
      assert Lumis.formatter_type(:bbcode_scoped) ==
               {:ok, {:bbcode_scoped, [language: nil, rainbow_brackets: false]}}
    end
  end

  describe "formatter language option" do
    test "html_inline with language" do
      assert {:ok, result} =
               Lumis.highlight(":test", formatter: {:html_inline, language: "elixir"})

      assert result =~ ~s|class="language-elixir"|
    end

    test "html_linked with language" do
      assert {:ok, result} =
               Lumis.highlight(":test", formatter: {:html_linked, language: "elixir"})

      assert result =~ ~s|class="language-elixir"|
    end

    test "terminal with language" do
      assert {:ok, result} =
               Lumis.highlight(":test",
                 formatter: {:terminal, language: "elixir", theme: "onedark"}
               )

      assert result =~ "\e[0m"
    end

    test "bbcode_scoped with language" do
      assert {:ok, result} =
               Lumis.highlight(":test", formatter: {:bbcode_scoped, language: "elixir"})

      assert result =~ "[string-special-symbol-elixir]"
    end

    test "html_multi_themes with language" do
      assert {:ok, result} =
               Lumis.highlight(":test",
                 formatter: {:html_multi_themes, language: "elixir", themes: [main: "onedark"]}
               )

      assert result =~ ~s|class="language-elixir"|
    end

    test "no deprecation warning when using formatter language" do
      warning =
        capture_io(:stderr, fn ->
          assert {:ok, _} =
                   Lumis.highlight(":test", formatter: {:html_inline, language: "elixir"})
        end)

      # Not `== ""`: test files are compiled while async tests already run, so
      # unrelated compiler diagnostics land on stderr depending on the seed.
      refute warning =~ ":language option is deprecated"
    end

    test "formatter_type preserves language value" do
      assert {:ok, {:html_inline, opts}} =
               Lumis.formatter_type({:html_inline, [language: "rust"]})

      assert Keyword.get(opts, :language) == "rust"
    end

    test "formatter_type preserves language for terminal" do
      assert {:ok, {:terminal, opts}} =
               Lumis.formatter_type({:terminal, [language: "rust"]})

      assert Keyword.get(opts, :language) == "rust"
    end

    test "formatter_type preserves background for terminal" do
      assert {:ok, {:terminal, opts}} =
               Lumis.formatter_type({:terminal, [background: "#282a36"]})

      assert Keyword.get(opts, :background) == "#282a36"
    end

    test "formatter_type preserves theme background for terminal" do
      assert {:ok, {:terminal, opts}} = Lumis.formatter_type({:terminal, [background: :theme]})

      assert Keyword.get(opts, :background) == :theme
    end

    test "formatter_type preserves width for terminal" do
      assert {:ok, {:terminal, opts}} =
               Lumis.formatter_type({:terminal, [width: 120]})

      assert Keyword.get(opts, :width) == 120
    end

    test "formatter_type preserves language for bbcode_scoped" do
      assert {:ok, {:bbcode_scoped, opts}} =
               Lumis.formatter_type({:bbcode_scoped, [language: "rust"]})

      assert Keyword.get(opts, :language) == "rust"
    end
  end

  test "accept empty theme" do
    assert {:ok, result} =
             Lumis.highlight("#!/usr/bin/env bash\necho 'test'",
               formatter: {:html_inline, theme: "noop"}
             )

    assert result =~ ~s|class="language-bash"|
  end

  test "detects language from shebang" do
    assert {:ok, result} = Lumis.highlight("#!/usr/bin/env bash\necho 'test'")
    assert result =~ ~s|class="language-bash"|
  end

  test "handles code with unicode characters" do
    assert {:ok, result} =
             Lumis.highlight("def π() do\n  3.14\nend",
               formatter: {:html_inline, language: "elixir"}
             )

    assert result =~ "π"
  end

  test "raises on invalid formatter options" do
    assert_raise NimbleOptions.ValidationError, fn ->
      Lumis.highlight!("test", formatter: :invalid)
    end

    assert_raise NimbleOptions.ValidationError, fn ->
      Lumis.highlight!("test", formatter: {:html_inline, :invalid})
    end
  end

  describe "formatter: inline" do
    test "with default opts" do
      assert_output(
        "defmodule Test do\n  @lang :elixir\nend",
        ~s"""
        <pre class="lumis" style="color: #abb2bf; background-color: #282c34;"><code class="language-elixir" translate="no" tabindex="0"><div class="l-line" data-line="1"><span style="color: #c678dd;">defmodule</span> <span style="color: #e5c07b;">Test</span> <span style="color: #c678dd;">do</span>
        </div><div class="l-line" data-line="2">  <span style="color: #56b6c2;"><span style="color: #d19a66;">@<span style="color: #61afef;"><span style="color: #d19a66;">lang <span style="color: #e06c75;">:elixir</span></span></span></span></span>
        </div><div class="l-line" data-line="3"><span style="color: #c678dd;">end</span>
        </div></code></pre>
        """,
        formatter: {:html_inline, language: "elixir", theme: "onedark"}
      )
    end

    test "with named theme" do
      assert_output(
        "defmodule Test do\n  @lang :elixir\nend",
        ~s"""
        <pre class="lumis" style="color: #f8f8f2; background-color: #282a36;"><code class="language-elixir" translate="no" tabindex="0"><div class="l-line" data-line="1"><span style="color: #8be9fd;">defmodule</span> <span style="color: #ffb86c;">Test</span> <span style="color: #ff79c6;">do</span>
        </div><div class="l-line" data-line="2">  <span style="color: #ff79c6;"><span style="color: #bd93f9;">@<span style="color: #50fa7b;"><span style="color: #bd93f9;">lang <span style="color: #bd93f9;">:elixir</span></span></span></span></span>
        </div><div class="l-line" data-line="3"><span style="color: #ff79c6;">end</span>
        </div></code></pre>
        """,
        formatter: {:html_inline, language: "elixir", theme: "dracula"}
      )
    end

    test "with struct theme" do
      assert_output(
        "defmodule Test do\n  @lang :elixir\nend",
        ~s"""
        <pre class="lumis" style="color: #f8f8f2; background-color: #282a36;"><code class="language-elixir" translate="no" tabindex="0"><div class="l-line" data-line="1"><span style="color: #8be9fd;">defmodule</span> <span style="color: #ffb86c;">Test</span> <span style="color: #ff79c6;">do</span>
        </div><div class="l-line" data-line="2">  <span style="color: #ff79c6;"><span style="color: #bd93f9;">@<span style="color: #50fa7b;"><span style="color: #bd93f9;">lang <span style="color: #bd93f9;">:elixir</span></span></span></span></span>
        </div><div class="l-line" data-line="3"><span style="color: #ff79c6;">end</span>
        </div></code></pre>
        """,
        formatter: {:html_inline, language: "elixir", theme: Lumis.Theme.get("dracula")}
      )
    end

    test "with pre_class" do
      assert_contains(
        "defmodule Test do\n  @lang :elixir\nend",
        ~s|<pre class="lumis test-pre-class"|,
        formatter: {:html_inline, language: "elixir", pre_class: "test-pre-class"}
      )
    end

    test "with include_highlights" do
      assert_output(
        "defmodule Test do\n  @lang :elixir\nend",
        ~s"""
        <pre class="lumis" style="color: #abb2bf; background-color: #282c34;"><code class="language-elixir" translate="no" tabindex="0"><div class="l-line" data-line="1"><span data-highlight="keyword.function" style="color: #c678dd;">defmodule</span> <span data-highlight="module" style="color: #e5c07b;">Test</span> <span data-highlight="keyword" style="color: #c678dd;">do</span>
        </div><div class="l-line" data-line="2">  <span data-highlight="operator" style="color: #56b6c2;"><span data-highlight="constant" style="color: #d19a66;">@<span data-highlight="function.call" style="color: #61afef;"><span data-highlight="constant" style="color: #d19a66;">lang <span data-highlight="string.special.symbol" style="color: #e06c75;">:elixir</span></span></span></span></span>
        </div><div class="l-line" data-line="3"><span data-highlight="keyword" style="color: #c678dd;">end</span>
        </div></code></pre>
        """,
        formatter: {:html_inline, language: "elixir", theme: "onedark", include_highlights: true}
      )
    end
  end

  describe "formatter: linked" do
    test "with default opts" do
      assert_output(
        "defmodule Test do\n  @lang :elixir\nend",
        ~s"""
        <pre class="lumis"><code class="language-elixir" translate="no" tabindex="0"><div class="l-line" data-line="1"><span class="l-keyword-function">defmodule</span> <span class="l-module">Test</span> <span class="l-keyword">do</span>
        </div><div class="l-line" data-line="2">  <span class="l-operator"><span class="l-constant">@<span class="l-function-call"><span class="l-constant">lang <span class="l-string-special-symbol">:elixir</span></span></span></span></span>
        </div><div class="l-line" data-line="3"><span class="l-keyword">end</span>
        </div></code></pre>
        """,
        formatter: {:html_linked, language: "elixir"}
      )
    end

    test "with pre_class option" do
      assert {:ok, result} =
               Lumis.highlight("defmodule Test do\nend",
                 formatter: {:html_linked, language: "elixir", pre_class: "custom-class"}
               )

      assert result =~ ~s|<pre class="lumis custom-class"|
    end
  end

  describe "formatter: terminal" do
    test "with default opts" do
      assert_output(
        "defmodule Test do\n  @lang :elixir\nend",
        "\e[0m\e[38;2;198;120;221mdefmodule\e[0m \e[0m\e[38;2;229;192;123mTest\e[0m \e[0m\e[38;2;198;120;221mdo\e[0m\n  \e[0m\e[38;2;209;154;102m@\e[0m\e[0m\e[38;2;209;154;102mlang \e[0m\e[0m\e[38;2;224;108;117m:elixir\e[0m\n\e[0m\e[38;2;198;120;221mend\e[0m",
        formatter: {:terminal, language: "elixir", theme: "onedark"}
      )
    end

    test "with theme" do
      assert_output(
        "defmodule Test do\n  @lang :elixir\nend",
        "\e[0m\e[38;2;207;34;46mdefmodule\e[0m \e[0m\e[38;2;149;56;0mTest\e[0m \e[0m\e[38;2;207;34;46mdo\e[0m\n  \e[0m\e[38;2;5;80;174m@\e[0m\e[0m\e[38;2;5;80;174mlang \e[0m\e[0m\e[38;2;5;80;174m:elixir\e[0m\n\e[0m\e[38;2;207;34;46mend\e[0m",
        formatter: {:terminal, language: "elixir", theme: "github_light"}
      )
    end
  end

  describe "formatter: bbcode_scoped" do
    test "with default opts" do
      assert_output(
        "defmodule Test do\n  value = \"[url=x]\"\nend",
        ~s"""
        [keyword-function-elixir]defmodule[/keyword-function-elixir] [module-elixir]Test[/module-elixir] [keyword-elixir]do[/keyword-elixir]
          [variable-elixir]value[/variable-elixir] [operator-elixir]=[/operator-elixir] [string-elixir]\"&#91;url=x&#93;\"[/string-elixir]
        [keyword-elixir]end[/keyword-elixir]
        """,
        formatter: {:bbcode_scoped, language: "elixir"}
      )
    end
  end

  describe "formatter: html_multi_themes" do
    test "with basic dual theme support" do
      assert_contains(
        "defmodule Test do\nend",
        ~s|--lumis-light-bg:#ffffff;|,
        formatter:
          {:html_multi_themes,
           language: "elixir", themes: [light: "github_light", dark: "github_dark"]}
      )
    end

    test "with single theme" do
      assert_output(
        "test code",
        ~s"""
        <pre class="lumis lumis-themes main" style="--lumis-main:#abb2bf; --lumis-main-bg:#282c34;"><code class="language-elixir" translate="no" tabindex="0"><div class="l-line" data-line="1"><span style="--lumis-main:#61afef; --lumis-main-font-style:normal; --lumis-main-font-weight:normal; --lumis-main-text-decoration:none;">test</span> <span style="--lumis-main:#e06c75; --lumis-main-font-style:normal; --lumis-main-font-weight:normal; --lumis-main-text-decoration:none;">code</span>
        </div></code></pre>
        """,
        formatter: {:html_multi_themes, language: "elixir", themes: [main: "onedark"]}
      )
    end

    test "with light-dark() function" do
      assert_contains(
        "test",
        ~s|style="color: light-dark(#1f2328, #e6edf3); background-color: light-dark(#ffffff, #0d1117)|,
        formatter:
          {:html_multi_themes,
           language: "elixir",
           themes: [light: "github_light", dark: "github_dark"],
           default_theme: "light-dark()"}
      )
    end

    test "with custom css_variable_prefix" do
      assert_contains(
        "test",
        ~s|style="--custom-light:#1f2328; --custom-light-bg:#ffffff;|,
        formatter:
          {:html_multi_themes,
           language: "elixir", themes: [light: "github_light"], css_variable_prefix: "--custom"}
      )
    end

    test "with pre_class option" do
      assert_contains(
        "test",
        ~s|class="lumis lumis-themes custom-class main"|,
        formatter:
          {:html_multi_themes,
           language: "elixir", themes: [main: "onedark"], pre_class: "custom-class"}
      )
    end

    test "with highlight_lines option" do
      highlight_lines = %{
        lines: [1],
        style: "background-color: yellow;",
        class: nil
      }

      assert_contains(
        "line1\nline2",
        ~s|style="background-color: yellow;"|,
        formatter:
          {:html_multi_themes,
           language: "elixir", themes: [main: "onedark"], highlight_lines: highlight_lines}
      )
    end

    test "with header option" do
      header = %{
        open_tag: ~s|<div class="code-wrapper">|,
        close_tag: "</div>"
      }

      assert_contains(
        "test",
        ~s|<div class="code-wrapper">|,
        formatter:
          {:html_multi_themes, language: "elixir", themes: [main: "onedark"], header: header}
      )
    end

    test "with Theme struct" do
      theme = Lumis.Theme.get("onedark")

      assert_output(
        "test code",
        ~s"""
        <pre class="lumis lumis-themes main" style="--lumis-main:#abb2bf; --lumis-main-bg:#282c34;"><code class="language-elixir" translate="no" tabindex="0"><div class="l-line" data-line="1"><span style="--lumis-main:#61afef; --lumis-main-font-style:normal; --lumis-main-font-weight:normal; --lumis-main-text-decoration:none;">test</span> <span style="--lumis-main:#e06c75; --lumis-main-font-style:normal; --lumis-main-font-weight:normal; --lumis-main-text-decoration:none;">code</span>
        </div></code></pre>
        """,
        formatter: {:html_multi_themes, language: "elixir", themes: [main: theme]}
      )
    end

    test "with mixed string and Theme struct" do
      theme = Lumis.Theme.get("onedark")

      assert_contains(
        "test",
        ~s|--lumis-light:#1f2328; --lumis-light-bg:#ffffff;|,
        formatter:
          {:html_multi_themes, language: "elixir", themes: [light: "github_light", dark: theme]}
      )
    end

    test "raises when themes option is missing" do
      assert_raise NimbleOptions.ValidationError, ~r/required :themes option not found/, fn ->
        Lumis.highlight("test",
          formatter: {:html_multi_themes, language: "elixir"}
        )
      end
    end

    test "raises when themes list is empty" do
      assert_raise NimbleOptions.ValidationError, ~r/empty/, fn ->
        Lumis.highlight("test",
          formatter: {:html_multi_themes, language: "elixir", themes: []}
        )
      end
    end

    test "raises when theme not found" do
      assert_raise NimbleOptions.ValidationError, ~r/not found/, fn ->
        Lumis.highlight("test",
          formatter: {:html_multi_themes, language: "elixir", themes: [main: "nonexistent"]}
        )
      end
    end
  end

  describe "formatter: highlight_lines" do
    test "html_inline with single line highlighting" do
      highlight_lines = %{
        lines: [2],
        style: "background-color: yellow;"
      }

      result =
        Lumis.highlight!(
          "def hello\n  puts 'world'\nend",
          formatter: {:html_inline, language: "ruby", highlight_lines: highlight_lines}
        )

      assert String.contains?(
               result,
               ~s|<div class="l-line" style="background-color: yellow;" data-line="2">|
             )
    end

    test "html_inline with multiple line ranges" do
      highlight_lines = %{
        lines: [1, 3],
        style: "background-color: yellow;"
      }

      result =
        Lumis.highlight!(
          "line 1\nline 2\nline 3",
          formatter: {:html_inline, language: "text", highlight_lines: highlight_lines}
        )

      assert String.contains?(
               result,
               ~s|<div class="l-line" style="background-color: yellow;" data-line="1">|
             )

      refute String.contains?(
               result,
               ~s|<div class="l-line" style="background-color: yellow;" data-line="2">|
             )

      assert String.contains?(
               result,
               ~s|<div class="l-line" style="background-color: yellow;" data-line="3">|
             )
    end

    test "html_inline with mixed integers and ranges" do
      highlight_lines = %{
        lines: [1, 3..5, 7],
        style: "background-color: yellow;"
      }

      result =
        Lumis.highlight!(
          "line 1\nline 2\nline 3\nline 4\nline 5\nline 6\nline 7\nline 8\nline 9",
          formatter: {:html_inline, language: "text", highlight_lines: highlight_lines}
        )

      assert String.contains?(
               result,
               ~s|<div class="l-line" style="background-color: yellow;" data-line="1">|
             )

      refute String.contains?(
               result,
               ~s|<div class="l-line" style="background-color: yellow;" data-line="2">|
             )

      assert String.contains?(
               result,
               ~s|<div class="l-line" style="background-color: yellow;" data-line="3">|
             )

      assert String.contains?(
               result,
               ~s|<div class="l-line" style="background-color: yellow;" data-line="4">|
             )

      assert String.contains?(
               result,
               ~s|<div class="l-line" style="background-color: yellow;" data-line="5">|
             )

      refute String.contains?(
               result,
               ~s|<div class="l-line" style="background-color: yellow;" data-line="6">|
             )

      assert String.contains?(
               result,
               ~s|<div class="l-line" style="background-color: yellow;" data-line="7">|
             )
    end

    test "html_inline with theme style" do
      highlight_lines = %{
        lines: [1],
        style: :theme
      }

      result =
        Lumis.highlight!(
          "def test\nend",
          formatter:
            {:html_inline, language: "ruby", theme: "dracula", highlight_lines: highlight_lines}
        )

      assert String.contains?(
               result,
               ~s|<div class="l-line" style="background-color: #44475a;" data-line="1">|
             )
    end

    test "html_inline with default theme style (style field omitted)" do
      highlight_lines = %{
        lines: [1]
        # style field is omitted, should default to :theme
      }

      result =
        Lumis.highlight!(
          "def test\nend",
          formatter:
            {:html_inline, language: "ruby", theme: "dracula", highlight_lines: highlight_lines}
        )

      assert String.contains?(
               result,
               ~s|<div class="l-line" style="background-color: #44475a;" data-line="1">|
             )
    end

    test "html_inline with class option" do
      highlight_lines = %{
        lines: [1, 2],
        class: "highlight-custom"
      }

      result =
        Lumis.highlight!(
          "def test\n  puts 'hello'\nend",
          formatter:
            {:html_inline, language: "ruby", theme: "onedark", highlight_lines: highlight_lines}
        )

      assert String.contains?(
               result,
               ~s|<div class="l-line highlight-custom" style="background-color: #282c34;" data-line="1">|
             )

      assert String.contains?(
               result,
               ~s|<div class="l-line highlight-custom" style="background-color: #282c34;" data-line="2">|
             )
    end

    test "html_inline with both style and class" do
      highlight_lines = %{
        lines: [2],
        style: "background-color: #ffcccc;",
        class: "error-line"
      }

      result =
        Lumis.highlight!(
          "def test\n  raise 'error'\nend",
          formatter: {:html_inline, language: "ruby", highlight_lines: highlight_lines}
        )

      assert String.contains?(
               result,
               ~s|<div class="l-line error-line" style="background-color: #ffcccc;" data-line="2">|
             )
    end

    test "html_inline with nil style and class" do
      highlight_lines = %{
        lines: [1],
        style: nil,
        class: "custom-highlight"
      }

      result =
        Lumis.highlight!(
          "def test\nend",
          formatter: {:html_inline, language: "ruby", highlight_lines: highlight_lines}
        )

      assert String.contains?(result, ~s|<div class="l-line custom-highlight" data-line="1">|)
    end

    test "html_linked with single line and default theme" do
      highlight_lines = %{
        lines: [1]
      }

      result =
        Lumis.highlight!(
          "line 1\nline 2\nline 3\nline 4\nline 5\nline 6\nline 7\nline 8\nline 9",
          formatter: {:html_linked, language: "text", highlight_lines: highlight_lines}
        )

      assert String.contains?(result, ~s|<div class="l-line l-highlighted" data-line="1">|)
    end

    test "html_linked with multiple lines and default theme " do
      highlight_lines = %{
        lines: [1, 2]
      }

      result =
        Lumis.highlight!(
          "line 1\nline 2\nline 3\nline 4\nline 5\nline 6\nline 7\nline 8\nline 9",
          formatter: {:html_linked, language: "text", highlight_lines: highlight_lines}
        )

      assert String.contains?(result, ~s|<div class="l-line l-highlighted" data-line="1">|)
      assert String.contains?(result, ~s|<div class="l-line l-highlighted" data-line="2">|)
      refute String.contains?(result, ~s|<div class="l-line l-highlighted" data-line="3">|)
    end

    test "html_linked with mixes lines and ranges and default theme " do
      highlight_lines = %{
        lines: [1, 2, 2..4]
      }

      result =
        Lumis.highlight!(
          "line 1\nline 2\nline 3\nline 4\nline 5\nline 6\nline 7\nline 8\nline 9",
          formatter: {:html_linked, language: "text", highlight_lines: highlight_lines}
        )

      assert String.contains?(result, ~s|<div class="l-line l-highlighted" data-line="1">|)
      assert String.contains?(result, ~s|<div class="l-line l-highlighted" data-line="2">|)
      assert String.contains?(result, ~s|<div class="l-line l-highlighted" data-line="3">|)
      refute String.contains?(result, ~s|<div class="l-line l-highlighted" data-line="5">|)
    end

    test "html_linked with CSS class" do
      highlight_lines = %{
        lines: [1],
        class: "hl-test"
      }

      result =
        Lumis.highlight!(
          "def broken\n  raise 'error'\nend",
          formatter: {:html_linked, language: "ruby", highlight_lines: highlight_lines}
        )

      assert String.contains?(result, ~s|<div class="l-line hl-test" data-line="1">|)
    end

    test "invalid highlight_lines format raises error" do
      assert_raise NimbleOptions.ValidationError, fn ->
        Lumis.highlight!(
          "test",
          formatter: {:html_inline, language: "text", highlight_lines: "invalid"}
        )
      end
    end

    test "invalid lines format raises error" do
      highlight_lines = %{
        lines: ["invalid"],
        style: :theme
      }

      assert_raise NimbleOptions.ValidationError, ~r/invalid value for :highlight_lines/, fn ->
        Lumis.highlight!(
          "test",
          formatter: {:html_inline, language: "text", highlight_lines: highlight_lines}
        )
      end
    end

    test "invalid style format raises error" do
      highlight_lines = %{
        lines: [1],
        style: 123
      }

      assert_raise NimbleOptions.ValidationError, ~r/invalid value for :style option/, fn ->
        Lumis.highlight!(
          "test",
          formatter: {:html_inline, language: "text", highlight_lines: highlight_lines}
        )
      end
    end
  end

  describe "formatter: header" do
    test "html_inline with custom wrapper" do
      header = %{
        open_tag: ~s|<div class="wrapper">|,
        close_tag: "</div>"
      }

      result =
        Lumis.highlight!(
          "puts 'hello'",
          formatter: {:html_inline, language: "ruby", header: header}
        )

      assert String.starts_with?(result, ~s|<div class="wrapper"><pre class="lumis"|)
      assert String.ends_with?(result, "</div>")
    end

    test "invalid header format raises error" do
      assert_raise NimbleOptions.ValidationError, fn ->
        Lumis.highlight!(
          "test",
          formatter: {:html_inline, language: "text", header: "invalid"}
        )
      end
    end

    test "invalid header keys raise error" do
      header = %{
        open_tag: "<div>"
        # missing close_tag
      }

      assert_raise NimbleOptions.ValidationError,
                   ~r/invalid value for :header option/,
                   fn ->
                     Lumis.highlight!(
                       "test",
                       formatter: {:html_inline, language: "text", header: header}
                     )
                   end
    end

    test "non-string header values raise error" do
      header = %{
        open_tag: 123,
        close_tag: "</div>"
      }

      assert_raise NimbleOptions.ValidationError,
                   ~r/invalid value for :open_tag option/,
                   fn ->
                     Lumis.highlight!(
                       "test",
                       formatter: {:html_inline, language: "text", header: header}
                     )
                   end
    end
  end

  describe "formatter: combined features" do
    test "highlight_lines and header together" do
      highlight_lines = %{
        lines: [1],
        style: "background-color: #f8d7da;"
      }

      header = %{
        open_tag: "<div class='code-block' data-highlighted='true'>",
        close_tag: "</div>"
      }

      result =
        Lumis.highlight!(
          "error_line\nnormal_line",
          formatter:
            {:html_inline, language: "text", highlight_lines: highlight_lines, header: header}
        )

      assert String.starts_with?(result, "<div class='code-block' data-highlighted='true'>")
      assert String.ends_with?(result, "</div>")
      assert String.contains?(result, "background-color: #f8d7da;")
      assert String.contains?(result, "data-line=\"1\"")
    end

    test "all options together" do
      highlight_lines = %{
        lines: [2],
        style: :theme
      }

      header = %{
        open_tag: "<section class='example'>",
        close_tag: "</section>"
      }

      result =
        Lumis.highlight!(
          "def example\n  # this line is highlighted\n  puts 'done'\nend",
          formatter: {
            :html_inline,
            language: "ruby",
            theme: "dracula",
            pre_class: "custom-code",
            italic: true,
            include_highlights: true,
            highlight_lines: highlight_lines,
            header: header
          }
        )

      assert String.starts_with?(result, "<section class='example'>")
      assert String.ends_with?(result, "</section>")
      assert String.contains?(result, "class=\"lumis custom-code\"")
      assert String.contains?(result, "data-highlight=")
      assert String.contains?(result, "data-line=\"2\"")
      # Dracula theme colors
      assert String.contains?(result, "background-color: #282a36")
    end
  end

  describe "validate_options!/1" do
    test "validates valid options" do
      assert [formatter: {:html_inline, formatter_opts}] =
               Lumis.validate_options!(formatter: {:html_inline, language: "elixir"})

      assert Keyword.equal?(
               [
                 header: nil,
                 highlight_lines: nil,
                 include_highlights: false,
                 rainbow_brackets: false,
                 italic: false,
                 pre_class: nil,
                 theme: nil,
                 language: "elixir"
               ],
               formatter_opts
             )
    end

    test "validates options with default values" do
      assert [formatter: {:html_inline, formatter_opts}] =
               Lumis.validate_options!([])

      assert Keyword.equal?(
               [
                 header: nil,
                 highlight_lines: nil,
                 include_highlights: false,
                 rainbow_brackets: false,
                 italic: false,
                 pre_class: nil,
                 theme: nil,
                 language: nil
               ],
               formatter_opts
             )
    end

    test "validates formatter options" do
      assert [formatter: {:html_inline, formatter_opts}] =
               Lumis.validate_options!(formatter: {:html_inline, theme: "dracula", italic: true})

      assert Keyword.equal?(
               [
                 language: nil,
                 header: nil,
                 highlight_lines: nil,
                 include_highlights: false,
                 rainbow_brackets: false,
                 pre_class: nil,
                 theme: "dracula",
                 italic: true
               ],
               formatter_opts
             )
    end

    test "validates deprecated options" do
      capture_io(:stderr, fn ->
        assert [
                 formatter:
                   {:html_inline,
                    [
                      language: nil,
                      rainbow_brackets: false,
                      header: nil,
                      highlight_lines: nil,
                      include_highlights: false,
                      italic: false,
                      pre_class: nil,
                      theme: nil
                    ]},
                 theme: "dracula",
                 inline_style: true,
                 pre_class: "custom"
               ] =
                 Lumis.validate_options!(
                   theme: "dracula",
                   inline_style: true,
                   pre_class: "custom"
                 )
      end)
    end

    test "copies deprecated language into formatter language" do
      assert [formatter: {:html_inline, formatter_opts}, language: "rust"] =
               Lumis.validate_options!(language: "rust")

      assert Keyword.equal?(
               [
                 language: "rust",
                 header: nil,
                 highlight_lines: nil,
                 include_highlights: false,
                 rainbow_brackets: false,
                 italic: false,
                 pre_class: nil,
                 theme: nil
               ],
               formatter_opts
             )
    end

    test "formatter language takes precedence over deprecated language" do
      capture_io(:stderr, fn ->
        assert [formatter: {:html_inline, formatter_opts}, language: "elixir"] =
                 Lumis.validate_options!(
                   language: "elixir",
                   formatter: {:html_inline, language: "rust"}
                 )

        assert Keyword.equal?(
                 [
                   language: "rust",
                   header: nil,
                   highlight_lines: nil,
                   include_highlights: false,
                   rainbow_brackets: false,
                   italic: false,
                   pre_class: nil,
                   theme: nil
                 ],
                 formatter_opts
               )
      end)
    end

    test "raises on invalid language type" do
      assert_raise NimbleOptions.ValidationError, fn ->
        Lumis.validate_options!(formatter: {:html_inline, language: 123})
      end
    end

    test "raises on invalid formatter" do
      assert_raise NimbleOptions.ValidationError, fn ->
        Lumis.validate_options!(formatter: :invalid_formatter)
      end
    end

    test "raises on invalid formatter options" do
      assert_raise NimbleOptions.ValidationError, fn ->
        Lumis.validate_options!(formatter: {:html_inline, invalid_option: true})
      end
    end
  end

  describe "rust_options!/1" do
    test "converts basic options to rust format" do
      options =
        Lumis.validate_options!(formatter: {:html_inline, language: "elixir", theme: "onedark"})

      assert %{
               language: "elixir",
               formatter:
                 {:html_inline,
                  %{
                    header: nil,
                    highlight_lines: nil,
                    include_highlights: false,
                    italic: false,
                    pre_class: nil,
                    theme: {:string, "onedark"}
                  }}
             } = Lumis.rust_options!(options)
    end

    test "uses formatter language when deprecated language is present" do
      options =
        Lumis.validate_options!(
          language: "elixir",
          formatter: {:html_inline, language: "rust"}
        )

      assert %{language: "rust", formatter: {:html_inline, %{theme: nil}}} =
               Lumis.rust_options!(options)
    end

    test "handles deprecated theme option" do
      capture_io(:stderr, fn ->
        options =
          Lumis.validate_options!(
            formatter: {:html_inline, theme: "dracula"},
            theme: "github_light"
          )

        assert %{
                 formatter:
                   {:html_inline,
                    %{
                      header: nil,
                      highlight_lines: nil,
                      include_highlights: false,
                      italic: false,
                      pre_class: nil,
                      theme: {:string, "github_light"}
                    }}
               } = Lumis.rust_options!(options)
      end)
    end

    test "handles deprecated pre_class option" do
      capture_io(:stderr, fn ->
        options =
          Lumis.validate_options!(
            formatter: {:html_inline, pre_class: "formatter-class"},
            pre_class: "deprecated-class"
          )

        assert %{
                 formatter:
                   {:html_inline,
                    %{
                      header: nil,
                      highlight_lines: nil,
                      include_highlights: false,
                      italic: false,
                      pre_class: "deprecated-class",
                      theme: nil
                    }}
               } = Lumis.rust_options!(options)
      end)
    end

    test "handles deprecated inline_style option converting to html_inline" do
      capture_io(:stderr, fn ->
        options = Lumis.validate_options!(formatter: :html_linked, inline_style: true)

        assert %{
                 formatter:
                   {:html_inline,
                    %{
                      header: nil,
                      highlight_lines: nil,
                      include_highlights: false,
                      italic: false,
                      pre_class: nil,
                      theme: nil
                    }},
                 language: nil
               } = Lumis.rust_options!(options)
      end)
    end

    test "handles deprecated inline_style option converting to html_linked" do
      capture_io(:stderr, fn ->
        options = Lumis.validate_options!(formatter: :html_inline, inline_style: false)

        assert %{
                 formatter:
                   {:html_linked,
                    %{
                      header: nil,
                      highlight_lines: nil,
                      pre_class: nil
                    }},
                 language: nil
               } = Lumis.rust_options!(options)
      end)
    end

    test "converts theme struct to rust format" do
      theme = Lumis.Theme.get("dracula")
      options = Lumis.validate_options!(formatter: {:html_inline, theme: theme})

      assert %{
               formatter:
                 {:html_inline,
                  %{
                    header: nil,
                    highlight_lines: nil,
                    include_highlights: false,
                    italic: false,
                    pre_class: nil,
                    theme: {:theme, ^theme}
                  }}
             } = Lumis.rust_options!(options)
    end

    test "converts theme string to rust format" do
      options = Lumis.validate_options!(formatter: {:html_inline, theme: "dracula"})

      assert %{
               formatter:
                 {:html_inline,
                  %{
                    header: nil,
                    highlight_lines: nil,
                    include_highlights: false,
                    italic: false,
                    pre_class: nil,
                    theme: {:string, "dracula"}
                  }}
             } = Lumis.rust_options!(options)
    end

    test "handles nil theme" do
      options = Lumis.validate_options!(formatter: {:html_inline, theme: nil})

      assert %{
               formatter:
                 {:html_inline,
                  %{
                    header: nil,
                    highlight_lines: nil,
                    include_highlights: false,
                    italic: false,
                    pre_class: nil,
                    theme: nil
                  }}
             } = Lumis.rust_options!(options)
    end

    test "converts html_linked formatter" do
      options = Lumis.validate_options!(formatter: {:html_linked, pre_class: "test"})

      assert %{
               formatter:
                 {:html_linked,
                  %{
                    header: nil,
                    highlight_lines: nil,
                    pre_class: "test"
                  }}
             } = Lumis.rust_options!(options)
    end

    test "converts terminal formatter" do
      options = Lumis.validate_options!(formatter: {:terminal, theme: "github_light"})

      assert %{formatter: {:terminal, %{theme: {:string, "github_light"}}}} =
               Lumis.rust_options!(options)
    end

    test "converts terminal formatter custom background" do
      options =
        Lumis.validate_options!(
          formatter: {:terminal, theme: "github_light", background: "#ffffff"}
        )

      assert %{
               formatter:
                 {:terminal,
                  %{theme: {:string, "github_light"}, background: {:string, "#ffffff"}}}
             } = Lumis.rust_options!(options)
    end

    test "converts terminal formatter width" do
      options =
        Lumis.validate_options!(
          formatter: {:terminal, theme: "github_light", background: "#ffffff", width: 120}
        )

      assert %{
               formatter:
                 {:terminal,
                  %{
                    theme: {:string, "github_light"},
                    background: {:string, "#ffffff"},
                    width: 120
                  }}
             } = Lumis.rust_options!(options)
    end

    test "converts terminal formatter theme background" do
      options =
        Lumis.validate_options!(formatter: {:terminal, theme: "github_light", background: :theme})

      assert %{
               formatter: {:terminal, %{theme: {:string, "github_light"}, background: :theme}}
             } = Lumis.rust_options!(options)
    end

    test "converts bbcode_scoped formatter" do
      options = Lumis.validate_options!(formatter: :bbcode_scoped)

      assert %{formatter: {:bbcode_scoped, %{}}} = Lumis.rust_options!(options)
    end

    test "handles highlight_lines option" do
      highlight_lines = %{lines: [1, 2..4], style: :theme, class: nil}

      options =
        Lumis.validate_options!(formatter: {:html_inline, highlight_lines: highlight_lines})

      assert %{
               formatter:
                 {:html_inline,
                  %{
                    header: nil,
                    highlight_lines: %Lumis.HtmlInlineHighlightLines{},
                    include_highlights: false,
                    italic: false,
                    pre_class: nil,
                    theme: nil
                  }}
             } = Lumis.rust_options!(options)
    end

    test "handles header option" do
      header = %{open_tag: "<div>", close_tag: "</div>"}
      options = Lumis.validate_options!(formatter: {:html_inline, header: header})

      assert %{
               formatter:
                 {:html_inline,
                  %{
                    header: %Lumis.HtmlElement{open_tag: "<div>", close_tag: "</div>"},
                    highlight_lines: nil,
                    include_highlights: false,
                    italic: false,
                    pre_class: nil,
                    theme: nil
                  }}
             } = Lumis.rust_options!(options)
    end

    test "returns map instead of keyword list" do
      options = Lumis.validate_options!(formatter: {:html_inline, language: "elixir"})

      assert %{} = Lumis.rust_options!(options)
    end
  end

  describe "error contract" do
    @bad_default {:html_multi_themes,
                  language: "elixir", themes: [light: "github_light"], default_theme: "nope"}

    test "highlight/2 returns {:error, _} rather than raising" do
      assert {:error, message} = Lumis.highlight("x = 1", formatter: @bad_default)
      assert message =~ "Default theme"
    end

    test "highlight!/2 raises where highlight/2 returns an error" do
      assert_raise Lumis.HighlightError, ~r/Default theme/, fn ->
        Lumis.highlight!("x = 1", formatter: @bad_default)
      end
    end

    test "an unknown language falls back to plaintext rather than erroring" do
      assert {:ok, output} =
               Lumis.highlight("x = 1", formatter: {:html_inline, language: "not_a_language"})

      assert output =~ "language-plaintext"
    end

    test "invalid options still raise from highlight/2, being a caller mistake" do
      assert_raise NimbleOptions.ValidationError, fn ->
        Lumis.highlight("x = 1", formatter: {:html_inline, nonsense: true})
      end
    end
  end

  describe "data_dir/0" do
    test "reports the directory Lumis.Application configured the store with" do
      expected =
        cond do
          path = Application.get_env(:lumis, :data_dir) -> Path.expand(path)
          path = System.get_env("LUMIS_DATA_DIR") -> Path.expand(path)
          true -> Path.join(List.to_string(:code.priv_dir(:lumis)), "lumis")
        end

      assert Lumis.data_dir() == expected
    end

    test "names the directory the store actually writes parsers into" do
      assert :ok = Lumis.Languages.load("json")
      assert File.dir?(Path.join(Lumis.data_dir(), "parsers"))
    end
  end
end

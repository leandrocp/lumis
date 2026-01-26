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
        assert {:ok, hl} = Lumis.highlight("elixir", ":test")

        assert hl =~
                 ~s|<pre class="athl" style="color: #abb2bf; background-color: #282c34;"><code class="language-elixir"|

        assert {:ok, hl} = Lumis.highlight("elixir", ":test", theme: "dracula")

        assert hl =~
                 ~s|<pre class="athl" style="color: #f8f8f2; background-color: #282a36;"><code class="language-elixir"|
      end)
    end

    test "highlight!" do
      capture_io(:stderr, fn ->
        assert Lumis.highlight!("elixir", ":test") =~
                 ~s|<pre class="athl" style="color: #abb2bf; background-color: #282c34;"><code class="language-elixir"|

        assert Lumis.highlight!("elixir", ":test", theme: "dracula") =~
                 ~s|<pre class="athl" style="color: #f8f8f2; background-color: #282a36;"><code class="language-elixir"|
      end)
    end

    test "theme option" do
      capture_io(:stderr, fn ->
        assert {:ok, highlighted} =
                 Lumis.highlight(":test", language: "elixir", theme: "github_light")

        assert highlighted =~
                 ~s|<pre class="athl" style="color: #1f2328; background-color: #ffffff;">|
      end)
    end

    test "inline_style option" do
      capture_io(:stderr, fn ->
        assert {:ok,
                "<pre class=\"athl\" style=\"color: #abb2bf; background-color: #282c34;\"><code class=\"language-elixir\" translate=\"no\" tabindex=\"0\"><div class=\"line\" data-line=\"1\"><span style=\"color: #e06c75;\">:test</span>\n</div></code></pre>"} =
                 Lumis.highlight(":test", language: "elixir", inline_style: true)
      end)
    end

    test "pre_class option" do
      capture_io(:stderr, fn ->
        assert {:ok, highlighted} =
                 Lumis.highlight(":test", language: "elixir", pre_class: "deprecated")

        assert highlighted =~ ~s|<pre class="athl deprecated"|
      end)
    end
  end

  test "available_languages" do
    available_languages = Lumis.available_languages()
    assert available_languages |> Map.keys() |> length() == 74
    assert available_languages["elixir"] == {"Elixir", ["*.ex", "*.exs"]}
  end

  test "available_themes" do
    assert Lumis.available_themes() |> length() == 117
  end

  test "default_options/0" do
    assert [formatter: {:html_inline, formatter_opts}, language: nil] =
             Lumis.default_options()

    assert Keyword.equal?(
             [
               header: nil,
               italic: false,
               theme: "onedark",
               pre_class: nil,
               include_highlights: false,
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
                 italic: false,
                 theme: "onedark",
                 pre_class: nil,
                 include_highlights: false,
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
                 pre_class: nil,
                 highlight_lines: nil,
                 header: nil
               ],
               formatter_opts
             )
    end
  end

  describe "formatter_type: :terminal" do
    test "default opts" do
      assert Lumis.formatter_type(:terminal) ==
               {:ok, {:terminal, theme: "onedark"}}
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
    assert {:ok, result} = Lumis.highlight("def π() do\n  3.14\nend", language: "elixir")
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
        <pre class="athl" style="color: #abb2bf; background-color: #282c34;"><code class="language-elixir" translate="no" tabindex="0"><div class="line" data-line="1"><span style="color: #c678dd;">defmodule</span> <span style="color: #e5c07b;">Test</span> <span style="color: #c678dd;">do</span>
        </div><div class="line" data-line="2">  <span style="color: #56b6c2;"><span style="color: #d19a66;">@<span style="color: #61afef;"><span style="color: #d19a66;">lang <span style="color: #e06c75;">:elixir</span></span></span></span></span>
        </div><div class="line" data-line="3"><span style="color: #c678dd;">end</span>
        </div></code></pre>
        """,
        language: "elixir"
      )
    end

    test "with named theme" do
      assert_output(
        "defmodule Test do\n  @lang :elixir\nend",
        ~s"""
        <pre class="athl" style="color: #f8f8f2; background-color: #282a36;"><code class="language-elixir" translate="no" tabindex="0"><div class="line" data-line="1"><span style="color: #ff79c6;">defmodule</span> <span style="color: #ffb86c;">Test</span> <span style="color: #ff79c6;">do</span>
        </div><div class="line" data-line="2">  <span style="color: #ff79c6;"><span style="color: #bd93f9;">@<span style="color: #50fa7b;"><span style="color: #bd93f9;">lang <span style="color: #bd93f9;">:elixir</span></span></span></span></span>
        </div><div class="line" data-line="3"><span style="color: #ff79c6;">end</span>
        </div></code></pre>
        """,
        language: "elixir",
        formatter: {:html_inline, theme: "dracula"}
      )
    end

    test "with struct theme" do
      assert_output(
        "defmodule Test do\n  @lang :elixir\nend",
        ~s"""
        <pre class="athl" style="color: #f8f8f2; background-color: #282a36;"><code class="language-elixir" translate="no" tabindex="0"><div class="line" data-line="1"><span style="color: #ff79c6;">defmodule</span> <span style="color: #ffb86c;">Test</span> <span style="color: #ff79c6;">do</span>
        </div><div class="line" data-line="2">  <span style="color: #ff79c6;"><span style="color: #bd93f9;">@<span style="color: #50fa7b;"><span style="color: #bd93f9;">lang <span style="color: #bd93f9;">:elixir</span></span></span></span></span>
        </div><div class="line" data-line="3"><span style="color: #ff79c6;">end</span>
        </div></code></pre>
        """,
        language: "elixir",
        formatter: {:html_inline, theme: Lumis.Theme.get("dracula")}
      )
    end

    test "with pre_class" do
      assert_contains(
        "defmodule Test do\n  @lang :elixir\nend",
        ~s|<pre class="athl test-pre-class"|,
        language: "elixir",
        formatter: {:html_inline, pre_class: "test-pre-class"}
      )
    end

    test "with include_highlights" do
      assert_output(
        "defmodule Test do\n  @lang :elixir\nend",
        ~s"""
        <pre class="athl" style="color: #abb2bf; background-color: #282c34;"><code class="language-elixir" translate="no" tabindex="0"><div class="line" data-line="1"><span data-highlight="keyword" style="color: #c678dd;">defmodule</span> <span data-highlight="module" style="color: #e5c07b;">Test</span> <span data-highlight="keyword" style="color: #c678dd;">do</span>
        </div><div class="line" data-line="2">  <span data-highlight="operator" style="color: #56b6c2;"><span data-highlight="constant" style="color: #d19a66;">@<span data-highlight="function.call" style="color: #61afef;"><span data-highlight="constant" style="color: #d19a66;">lang <span data-highlight="string.special.symbol" style="color: #e06c75;">:elixir</span></span></span></span></span>
        </div><div class="line" data-line="3"><span data-highlight="keyword" style="color: #c678dd;">end</span>
        </div></code></pre>
        """,
        language: "elixir",
        formatter: {:html_inline, include_highlights: true}
      )
    end
  end

  describe "formatter: linked" do
    test "with default opts" do
      assert_output(
        "defmodule Test do\n  @lang :elixir\nend",
        ~s"""
        <pre class="athl"><code class="language-elixir" translate="no" tabindex="0"><div class="line" data-line="1"><span class="keyword">defmodule</span> <span class="module">Test</span> <span class="keyword">do</span>
        </div><div class="line" data-line="2">  <span class="operator"><span class="constant">@<span class="function-call"><span class="constant">lang <span class="string-special-symbol">:elixir</span></span></span></span></span>
        </div><div class="line" data-line="3"><span class="keyword">end</span>
        </div></code></pre>
        """,
        language: "elixir",
        formatter: :html_linked
      )
    end

    test "with pre_class option" do
      assert {:ok, result} =
               Lumis.highlight("defmodule Test do\nend",
                 language: "elixir",
                 formatter: {:html_linked, pre_class: "custom-class"}
               )

      assert result =~ ~s|<pre class="athl custom-class"|
    end
  end

  describe "formatter: terminal" do
    test "with default opts" do
      assert_output(
        "defmodule Test do\n  @lang :elixir\nend",
        "\e[0m\e[38;2;198;120;221mdefmodule\e[0m \e[0m\e[38;2;229;192;123mTest\e[0m \e[0m\e[38;2;198;120;221mdo\e[0m\n  \e[0m\e[38;2;209;154;102m@\e[0m\e[0m\e[38;2;209;154;102mlang \e[0m\e[0m\e[38;2;224;108;117m:elixir\e[0m\n\e[0m\e[38;2;198;120;221mend\e[0m",
        language: "elixir",
        formatter: :terminal
      )
    end

    test "with theme" do
      assert_output(
        "defmodule Test do\n  @lang :elixir\nend",
        "\e[0m\e[38;2;207;34;46mdefmodule\e[0m \e[0m\e[38;2;149;56;0mTest\e[0m \e[0m\e[38;2;207;34;46mdo\e[0m\n  \e[0m\e[38;2;5;80;174m@\e[0m\e[0m\e[38;2;5;80;174mlang \e[0m\e[0m\e[38;2;5;80;174m:elixir\e[0m\n\e[0m\e[38;2;207;34;46mend\e[0m",
        language: "elixir",
        formatter: {:terminal, theme: "github_light"}
      )
    end
  end

  describe "formatter: html_multi_themes" do
    test "with basic dual theme support" do
      assert_contains(
        "defmodule Test do\nend",
        ~s|--athl-light-bg: #ffffff;|,
        language: "elixir",
        formatter: {:html_multi_themes, themes: [light: "github_light", dark: "github_dark"]}
      )
    end

    test "with single theme" do
      assert_output(
        "test code",
        ~s"""
        <pre class="athl athl-themes main" style="--athl-main: #abb2bf; --athl-main-bg: #282c34;"><code class="language-elixir" translate="no" tabindex="0"><div class="line" data-line="1"><span style="--athl-main: #61afef; --athl-main-font-style: normal; --athl-main-font-weight: normal; --athl-main-text-decoration: none;">test</span> <span style="--athl-main: #e06c75; --athl-main-font-style: normal; --athl-main-font-weight: normal; --athl-main-text-decoration: none;">code</span>
        </div></code></pre>
        """,
        language: "elixir",
        formatter: {:html_multi_themes, themes: [main: "onedark"]}
      )
    end

    test "with light-dark() function" do
      assert_contains(
        "test",
        ~s|style="color: light-dark(#1f2328, #e6edf3); background-color: light-dark(#ffffff, #0d1117)|,
        language: "elixir",
        formatter:
          {:html_multi_themes,
           themes: [light: "github_light", dark: "github_dark"], default_theme: "light-dark()"}
      )
    end

    test "with custom css_variable_prefix" do
      assert_contains(
        "test",
        ~s|style="--custom-light: #1f2328; --custom-light-bg: #ffffff;|,
        language: "elixir",
        formatter:
          {:html_multi_themes, themes: [light: "github_light"], css_variable_prefix: "--custom"}
      )
    end

    test "with pre_class option" do
      assert_contains(
        "test",
        ~s|class="athl athl-themes custom-class main"|,
        language: "elixir",
        formatter: {:html_multi_themes, themes: [main: "onedark"], pre_class: "custom-class"}
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
        language: "elixir",
        formatter:
          {:html_multi_themes, themes: [main: "onedark"], highlight_lines: highlight_lines}
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
        language: "elixir",
        formatter: {:html_multi_themes, themes: [main: "onedark"], header: header}
      )
    end

    test "with Theme struct" do
      theme = Lumis.Theme.get("onedark")

      assert_output(
        "test code",
        ~s"""
        <pre class="athl athl-themes main" style="--athl-main: #abb2bf; --athl-main-bg: #282c34;"><code class="language-elixir" translate="no" tabindex="0"><div class="line" data-line="1"><span style="--athl-main: #61afef; --athl-main-font-style: normal; --athl-main-font-weight: normal; --athl-main-text-decoration: none;">test</span> <span style="--athl-main: #e06c75; --athl-main-font-style: normal; --athl-main-font-weight: normal; --athl-main-text-decoration: none;">code</span>
        </div></code></pre>
        """,
        language: "elixir",
        formatter: {:html_multi_themes, themes: [main: theme]}
      )
    end

    test "with mixed string and Theme struct" do
      theme = Lumis.Theme.get("onedark")

      assert_contains(
        "test",
        ~s|--athl-light: #1f2328; --athl-light-bg: #ffffff;|,
        language: "elixir",
        formatter: {:html_multi_themes, themes: [light: "github_light", dark: theme]}
      )
    end

    test "raises when themes option is missing" do
      assert_raise NimbleOptions.ValidationError, ~r/required :themes option not found/, fn ->
        Lumis.highlight("test", language: "elixir", formatter: :html_multi_themes)
      end
    end

    test "raises when themes list is empty" do
      assert_raise NimbleOptions.ValidationError, ~r/empty/, fn ->
        Lumis.highlight("test", language: "elixir", formatter: {:html_multi_themes, themes: []})
      end
    end

    test "raises when theme not found" do
      assert_raise NimbleOptions.ValidationError, ~r/not found/, fn ->
        Lumis.highlight("test",
          language: "elixir",
          formatter: {:html_multi_themes, themes: [main: "nonexistent"]}
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
          language: "ruby",
          formatter: {:html_inline, highlight_lines: highlight_lines}
        )

      assert String.contains?(
               result,
               ~s|<div class="line" style="background-color: yellow;" data-line="2">|
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
          language: "text",
          formatter: {:html_inline, highlight_lines: highlight_lines}
        )

      assert String.contains?(
               result,
               ~s|<div class="line" style="background-color: yellow;" data-line="1">|
             )

      refute String.contains?(
               result,
               ~s|<div class="line" style="background-color: yellow;" data-line="2">|
             )

      assert String.contains?(
               result,
               ~s|<div class="line" style="background-color: yellow;" data-line="3">|
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
          language: "text",
          formatter: {:html_inline, highlight_lines: highlight_lines}
        )

      assert String.contains?(
               result,
               ~s|<div class="line" style="background-color: yellow;" data-line="1">|
             )

      refute String.contains?(
               result,
               ~s|<div class="line" style="background-color: yellow;" data-line="2">|
             )

      assert String.contains?(
               result,
               ~s|<div class="line" style="background-color: yellow;" data-line="3">|
             )

      assert String.contains?(
               result,
               ~s|<div class="line" style="background-color: yellow;" data-line="4">|
             )

      assert String.contains?(
               result,
               ~s|<div class="line" style="background-color: yellow;" data-line="5">|
             )

      refute String.contains?(
               result,
               ~s|<div class="line" style="background-color: yellow;" data-line="6">|
             )

      assert String.contains?(
               result,
               ~s|<div class="line" style="background-color: yellow;" data-line="7">|
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
          language: "ruby",
          formatter: {:html_inline, theme: "dracula", highlight_lines: highlight_lines}
        )

      assert String.contains?(
               result,
               ~s|<div class="line" style="background-color: #44475a;" data-line="1">|
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
          language: "ruby",
          formatter: {:html_inline, theme: "dracula", highlight_lines: highlight_lines}
        )

      assert String.contains?(
               result,
               ~s|<div class="line" style="background-color: #44475a;" data-line="1">|
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
          language: "ruby",
          formatter: {:html_inline, highlight_lines: highlight_lines}
        )

      assert String.contains?(
               result,
               ~s|<div class="line highlight-custom" style="background-color: #282c34;" data-line="1">|
             )

      assert String.contains?(
               result,
               ~s|<div class="line highlight-custom" style="background-color: #282c34;" data-line="2">|
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
          language: "ruby",
          formatter: {:html_inline, highlight_lines: highlight_lines}
        )

      assert String.contains?(
               result,
               ~s|<div class="line error-line" style="background-color: #ffcccc;" data-line="2">|
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
          language: "ruby",
          formatter: {:html_inline, highlight_lines: highlight_lines}
        )

      assert String.contains?(result, ~s|<div class="line custom-highlight" data-line="1">|)
    end

    test "html_linked with single line and default theme" do
      highlight_lines = %{
        lines: [1]
      }

      result =
        Lumis.highlight!(
          "line 1\nline 2\nline 3\nline 4\nline 5\nline 6\nline 7\nline 8\nline 9",
          language: "text",
          formatter: {:html_linked, highlight_lines: highlight_lines}
        )

      assert String.contains?(result, ~s|<div class="line highlighted" data-line="1">|)
    end

    test "html_linked with multiple lines and default theme " do
      highlight_lines = %{
        lines: [1, 2]
      }

      result =
        Lumis.highlight!(
          "line 1\nline 2\nline 3\nline 4\nline 5\nline 6\nline 7\nline 8\nline 9",
          language: "text",
          formatter: {:html_linked, highlight_lines: highlight_lines}
        )

      assert String.contains?(result, ~s|<div class="line highlighted" data-line="1">|)
      assert String.contains?(result, ~s|<div class="line highlighted" data-line="2">|)
      refute String.contains?(result, ~s|<div class="line highlighted" data-line="3">|)
    end

    test "html_linked with mixes lines and ranges and default theme " do
      highlight_lines = %{
        lines: [1, 2, 2..4]
      }

      result =
        Lumis.highlight!(
          "line 1\nline 2\nline 3\nline 4\nline 5\nline 6\nline 7\nline 8\nline 9",
          language: "text",
          formatter: {:html_linked, highlight_lines: highlight_lines}
        )

      assert String.contains?(result, ~s|<div class="line highlighted" data-line="1">|)
      assert String.contains?(result, ~s|<div class="line highlighted" data-line="2">|)
      assert String.contains?(result, ~s|<div class="line highlighted" data-line="3">|)
      refute String.contains?(result, ~s|<div class="line highlighted" data-line="5">|)
    end

    test "html_linked with CSS class" do
      highlight_lines = %{
        lines: [1],
        class: "hl-test"
      }

      result =
        Lumis.highlight!(
          "def broken\n  raise 'error'\nend",
          language: "ruby",
          formatter: {:html_linked, highlight_lines: highlight_lines}
        )

      assert String.contains?(result, ~s|<div class="line hl-test" data-line="1">|)
    end

    test "invalid highlight_lines format raises error" do
      assert_raise NimbleOptions.ValidationError, fn ->
        Lumis.highlight!(
          "test",
          language: "text",
          formatter: {:html_inline, highlight_lines: "invalid"}
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
          language: "text",
          formatter: {:html_inline, highlight_lines: highlight_lines}
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
          language: "text",
          formatter: {:html_inline, highlight_lines: highlight_lines}
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
          language: "ruby",
          formatter: {:html_inline, header: header}
        )

      assert String.starts_with?(result, ~s|<div class="wrapper"><pre class="athl"|)
      assert String.ends_with?(result, "</div>")
    end

    test "invalid header format raises error" do
      assert_raise NimbleOptions.ValidationError, fn ->
        Lumis.highlight!(
          "test",
          language: "text",
          formatter: {:html_inline, header: "invalid"}
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
                       language: "text",
                       formatter: {:html_inline, header: header}
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
                       language: "text",
                       formatter: {:html_inline, header: header}
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
          language: "text",
          formatter: {:html_inline, highlight_lines: highlight_lines, header: header}
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
          language: "ruby",
          formatter: {
            :html_inline,
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
      assert String.contains?(result, "class=\"athl custom-code\"")
      assert String.contains?(result, "data-highlight=")
      assert String.contains?(result, "data-line=\"2\"")
      # Dracula theme colors
      assert String.contains?(result, "background-color: #282a36")
    end
  end

  describe "validate_options!/1" do
    test "validates valid options" do
      assert [
               language: "elixir",
               formatter:
                 {:html_inline,
                  [
                    header: nil,
                    highlight_lines: nil,
                    include_highlights: false,
                    italic: false,
                    pre_class: nil,
                    theme: "onedark"
                  ]}
             ] = Lumis.validate_options!(language: "elixir", formatter: :html_inline)
    end

    test "validates options with default values" do
      assert [
               formatter:
                 {:html_inline,
                  [
                    header: nil,
                    highlight_lines: nil,
                    include_highlights: false,
                    italic: false,
                    pre_class: nil,
                    theme: "onedark"
                  ]},
               language: nil
             ] = Lumis.validate_options!([])
    end

    test "validates formatter options" do
      assert [
               language: nil,
               formatter:
                 {:html_inline,
                  [
                    header: nil,
                    highlight_lines: nil,
                    include_highlights: false,
                    pre_class: nil,
                    theme: "dracula",
                    italic: true
                  ]}
             ] =
               Lumis.validate_options!(formatter: {:html_inline, theme: "dracula", italic: true})
    end

    test "validates deprecated options" do
      capture_io(:stderr, fn ->
        assert [
                 formatter:
                   {:html_inline,
                    [
                      header: nil,
                      highlight_lines: nil,
                      include_highlights: false,
                      italic: false,
                      pre_class: nil,
                      theme: "onedark"
                    ]},
                 language: nil,
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

    test "raises on invalid language type" do
      assert_raise NimbleOptions.ValidationError, fn ->
        Lumis.validate_options!(language: 123)
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
        Lumis.validate_options!(language: "elixir", formatter: {:html_inline, theme: "onedark"})

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
                      theme: {:string, "onedark"}
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
                    theme: {:string, "onedark"}
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
                    theme: {:string, "onedark"}
                  }}
             } = Lumis.rust_options!(options)
    end

    test "returns map instead of keyword list" do
      options = Lumis.validate_options!(language: "elixir", formatter: :html_inline)

      assert %{} = Lumis.rust_options!(options)
    end
  end
end

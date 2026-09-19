defmodule Lumis.FormatterHelpersTest do
  @moduledoc """
  Elixir's half of the cross-runtime formatter helper check.

  `fixtures/formatter-helpers.json` lists the helper capabilities every runtime
  must offer a custom formatter. A module reports its own functions, so this
  reflects rather than naming them again:

  - "exports every helper in the manifest" fails on a capability Elixir lacks.
  - "exports nothing the manifest does not account for" fails on a public
    function in neither the manifest's helper set nor `runtime_only`. This is the
    one that catches drift: the three helper modules reached 46 names between
    them with 11 in all three before anything checked (#1381).
  - "matches the shared output contract" feeds every helper the manifest's
    inputs and compares its string result with Rust's.

  Arity is not checked. Elixir keeps its own argument shapes — a table where Rust
  takes a theme, a keyword list where JavaScript takes an object — and only the
  capability has to exist everywhere.
  """
  use ExUnit.Case, async: true

  alias Lumis.Formatter.ANSI
  alias Lumis.Formatter.HTML
  alias Lumis.Theme
  alias Lumis.Theme.Style
  alias Lumis.Theme.TextDecoration

  @manifest_path Path.expand("../../../../fixtures/formatter-helpers.json", __DIR__)
  @external_resource @manifest_path

  @modules %{"html" => Lumis.Formatter.HTML, "ansi" => Lumis.Formatter.ANSI}

  # Read at runtime rather than into a module attribute: inlining the decoded
  # JSON gives the compiler a literal type precise enough to warn on `Map.keys/1`.
  defp manifest, do: @manifest_path |> File.read!() |> Jason.decode!()

  defp exported(module) do
    module.__info__(:functions)
    |> Enum.map(fn {name, _arity} -> Atom.to_string(name) end)
    |> Enum.uniq()
    |> Enum.sort()
  end

  defp manifest_helpers(module) do
    manifest()
    |> Map.fetch!("modules")
    |> Map.fetch!(module)
    |> Map.fetch!("helpers")
    |> Enum.map(&(get_in(&1, ["spelling", "elixir"]) || &1["name"]))
    |> Enum.sort()
  end

  defp runtime_only(module) do
    manifest()
    |> Map.fetch!("runtime_only")
    |> Map.get("elixir", %{})
    |> Map.get(module, %{})
    |> Map.keys()
    |> Enum.reject(&String.starts_with?(&1, "$"))
  end

  defp fixture_theme(themes, name) do
    themes
    |> Map.fetch!(name)
    |> Jason.encode!()
    |> Theme.from_json()
    |> then(fn {:ok, theme} -> theme end)
  end

  defp style(input) do
    %Style{
      fg: input["fg"],
      bg: input["bg"],
      bold: input["bold"],
      italic: input["italic"],
      text_decoration: %TextDecoration{
        underline: underline(input["underline"]),
        strikethrough: input["strikethrough"]
      }
    }
  end

  defp underline("none"), do: :none
  defp underline("solid"), do: :solid
  defp underline("wavy"), do: :wavy
  defp underline("double"), do: :double
  defp underline("dotted"), do: :dotted
  defp underline("dashed"), do: :dashed

  defp line_spec(number) when is_integer(number), do: number
  defp line_spec([first, last]), do: first..last

  defp event(%{"type" => "start", "scope" => scope, "language" => language}) do
    {:start, %{scope: scope, language: language}}
  end

  defp event(%{"type" => "source", "start" => start, "end" => stop}) do
    {:source, %{start: start, end: stop}}
  end

  defp event(%{"type" => "end"}), do: :end

  defp rgb_string(nil), do: ""
  defp rgb_string({red, green, blue}), do: Enum.join([red, green, blue], ",")

  defp opening_tag(name, attrs), do: HTML.open_tag(name, attrs)

  test "preserves the shared line-ending contract" do
    for %{"source" => source, "expected" => expected} <-
          manifest()["contract"]["html"]["lineEndingCases"] do
      events = [{:source, %{start: 0, end: byte_size(source)}}]

      assert HTML.render_lines_from_events(source, events, %{}) == expected,
             "source #{inspect(source)}"
    end
  end

  defp contract_outputs(contract) do
    html = contract["html"]
    ansi = contract["ansi"]
    fixture_themes = contract["themes"]
    theme = fixture_theme(fixture_themes, html["theme"])

    themes =
      Map.new(html["themes"], fn {name, theme_name} ->
        {name, fixture_theme(fixture_themes, theme_name)}
      end)

    style = style(contract["style"])
    inline_attrs = HTML.span_attrs(theme: theme, language: html["language"])

    multi_theme_attrs =
      HTML.span_multi_themes_attrs(themes: themes, language: html["language"])

    lines = Enum.map(html["lines"], &line_spec/1)
    events = Enum.map(html["events"], &event/1)
    line = html["line"]
    [red, green, blue] = ansi["rgb"]

    %{
      "html" => %{
        "escape" => HTML.escape(html["escapeText"]),
        "escape_attr" => HTML.escape_attr(html["escapeText"]),
        "escape_braces" => HTML.escape_braces(html["bracedText"]),
        "scope_to_class" => HTML.scope_to_class(html["linkedScope"]),
        "style_to_css" => HTML.style_to_css(style, italic: true),
        "text_decoration" => HTML.text_decoration(style.text_decoration),
        "sanitize_theme_name" => HTML.sanitize_theme_name(html["themeName"]),
        "open_span" =>
          HTML.open_span(%{html["scope"] => HTML.span_linked_attrs(html["scope"])}, html["scope"]),
        "span_inline_attrs" => HTML.open_span(inline_attrs, html["scope"]),
        "span_inline" => HTML.span_inline(html["text"], inline_attrs, html["scope"]),
        "span_linked_attrs" => HTML.span_linked_attrs(html["linkedScope"]),
        "span_linked" => HTML.span_linked(html["text"], html["linkedScope"]),
        "span_multi_themes_attrs" => HTML.open_span(multi_theme_attrs, html["scope"]),
        "span_multi_themes" =>
          HTML.span_multi_themes(html["text"], multi_theme_attrs, html["scope"]),
        "open_tag" => HTML.open_tag("pre", class: "lumis #{html["preClass"]}", hidden: true),
        "valid_attr_name" => to_string(HTML.valid_attr_name?("x onclick=alert(1)")),
        "pre_attrs" => opening_tag("pre", HTML.pre_attrs(class: html["preClass"], theme: theme)),
        "multi_themes_pre_attrs" =>
          opening_tag(
            "pre",
            HTML.multi_themes_pre_attrs(class: html["preClass"], themes: themes)
          ),
        "code_attrs" => opening_tag("code", HTML.code_attrs(html["language"])),
        "open_pre_tag" => HTML.open_pre_tag(class: html["preClass"], theme: theme),
        "open_multi_themes_pre_tag" =>
          HTML.open_multi_themes_pre_tag(class: html["preClass"], themes: themes),
        "open_code_tag" => HTML.open_code_tag(html["language"]),
        "close_pre_tag" => HTML.close_pre_tag(),
        "close_code_tag" => HTML.close_code_tag(),
        "closing_tags" => HTML.closing_tags(),
        "wrap_line" =>
          HTML.wrap_line(line["number"], line["content"],
            class_suffix: " #{line["class"]}",
            style: line["style"]
          ),
        "line_is_highlighted" => to_string(HTML.line_is_highlighted(lines, html["selectedLine"])),
        "highlight_line_class" =>
          HTML.highlight_line_class(lines, html["selectedLine"],
            class: html["highlightClass"],
            default_class: html["defaultHighlightClass"]
          ) || "",
        "render_lines_from_events" =>
          Jason.encode!(
            HTML.render_lines_from_events(html["source"], events, %{
              html["scope"] => HTML.span_linked_attrs(html["scope"])
            })
          )
      },
      "ansi" => %{
        "hex_to_rgb" => ansi["hex"] |> ANSI.hex_to_rgb() |> rgb_string(),
        "rgb_to_ansi" => ANSI.rgb_to_ansi(red, green, blue, ansi["background"]),
        "style_to_ansi" => ANSI.style_to_ansi(style),
        "paint" => ANSI.paint(ansi["text"], style),
        "reset" => ANSI.reset()
      }
    }
  end

  test "covers the modules Elixir publishes" do
    assert manifest() |> Map.fetch!("modules") |> Map.keys() |> Enum.sort() ==
             @modules |> Map.keys() |> Enum.sort()
  end

  test "matches the shared output contract" do
    manifest = manifest()
    outputs = contract_outputs(manifest["contract"])

    for {module, entry} <- manifest["modules"] do
      expected = Map.new(entry["helpers"], &{&1["name"], &1["expected"]})
      actual_names = outputs[module] |> Map.keys() |> Enum.sort()
      expected_names = expected |> Map.keys() |> Enum.sort()

      assert actual_names == expected_names,
             "#{module}: contract adapters drifted"

      for {name, value} <- expected do
        assert outputs[module][name] == value, "#{module}.#{name}"
      end
    end
  end

  for {module, alias} <- @modules do
    test "#{module} exports every helper in the manifest" do
      module = unquote(module)
      exported = exported(unquote(alias))
      missing = Enum.reject(manifest_helpers(module), &(&1 in exported))

      assert missing == [],
             "#{unquote(alias)} is missing #{inspect(missing)}"
    end

    test "#{module} exports nothing the manifest does not account for" do
      module = unquote(module)
      accounted = manifest_helpers(module) ++ runtime_only(module)
      unaccounted = Enum.reject(exported(unquote(alias)), &(&1 in accounted))

      assert unaccounted == [],
             "#{unquote(alias)} exports #{inspect(unaccounted)}, which is in neither the " <>
               "manifest's helper set nor runtime_only. Add them to " <>
               "fixtures/formatter-helpers.json and to the other runtimes, or classify them."
    end

    test "#{module} still exports every runtime_only helper" do
      module = unquote(module)
      exported = exported(unquote(alias))
      gone = Enum.reject(runtime_only(module), &(&1 in exported))

      assert gone == [], "runtime_only lists #{inspect(gone)}, which #{unquote(alias)} no longer
             exports; drop the entry"
    end
  end

  test "no waiver outlives its reason" do
    waivers =
      manifest()
      |> Map.fetch!("waived")
      |> Map.keys()
      |> Enum.reject(&String.starts_with?(&1, "$"))

    assert waivers == [],
           "every runtime offers every capability; drop these waivers: #{inspect(waivers)}"
  end
end

defmodule Lumis.FormatterTest do
  use ExUnit.Case, async: true

  defmodule LanguageFormatter do
    @moduledoc false
    @behaviour Lumis.Formatter

    @impl true
    def render(_source, _events, options) do
      Keyword.fetch!(options, :language)
    end
  end

  @bash "#!/usr/bin/env bash\nID=1\n"

  setup_all do
    :ok = Lumis.Languages.load(~w(elixir bash json))
    :ok
  end

  defp language_of(source, options \\ []) do
    Lumis.highlight!(source, formatter: {LanguageFormatter, options})
  end

  describe "the :language a formatter receives" do
    test "is the language highlighting detected, not the nothing the caller named" do
      # Rust's `Formatter::language()` and JavaScript's `this.language` both
      # carry the resolved language. Elixir used to hand over the caller's
      # `nil`, so a formatter could not label its own output without running
      # detection a second time.
      assert language_of(@bash) == "bash"
    end

    test "agrees with what the built-in formatters label the same source" do
      built_in = Lumis.highlight!(@bash, formatter: :html_linked)

      assert String.contains?(built_in, "class=\"language-#{language_of(@bash)}\"")
    end

    test "agrees with Lumis.Languages.guess/2" do
      for source <- [@bash, "defmodule App do\nend\n", "nothing identifies this\n"] do
        assert language_of(source) == Lumis.Languages.guess(nil, source)
      end
    end

    test "is the language the caller named when they named one" do
      assert language_of(@bash, language: "elixir") == "elixir"
    end

    test "is plaintext when nothing matches" do
      assert language_of("nothing identifies this\n") == "plaintext"
    end
  end

  defmodule BudgetFormatter do
    @moduledoc false
    @behaviour Lumis.Formatter

    @impl true
    def render(_source, _events, options) do
      send(self(), {:budget_exhausted, Keyword.fetch!(options, :budget_exhausted)})
      []
    end
  end

  describe "the :budget_exhausted a formatter receives" do
    @ordinary "defmodule A do\n  def b, do: (1 + (2 * 3))\nend\n"

    defp budget_exhausted(source, language, budget) do
      Lumis.highlight!(source, formatter: {BudgetFormatter, language: language}, budget: budget)
      assert_received {:budget_exhausted, exhausted}
      exhausted
    end

    test "is nil when no limit bound the render" do
      assert budget_exhausted(@ordinary, "elixir", time_limit: 0) == nil
    end

    test "is :matches when the match limit bound the query" do
      assert budget_exhausted(@ordinary, "elixir", match_limit: 1) == :matches
    end

    test "is :time when the time limit ran out" do
      # 200k unclosed brackets cannot highlight in 50 ms on any machine.
      pathological = String.duplicate("[", 200_000)

      assert budget_exhausted(pathological, "json", time_limit: 50) == :time
    end
  end

  defmodule RaisingFormatter do
    @moduledoc false
    @behaviour Lumis.Formatter

    @impl true
    def render(_source, _events, _options), do: raise(ArgumentError, "cannot render")
  end

  defmodule NotIodataFormatter do
    @moduledoc false
    @behaviour Lumis.Formatter

    @impl true
    def render(_source, _events, _options), do: :not_iodata
  end

  describe "a formatter that fails" do
    test "makes highlight/2 return a :custom_formatter error naming it" do
      assert {:error, %Lumis.RenderError{reason: :custom_formatter, detail: detail}} =
               Lumis.highlight("x = 1", formatter: {RaisingFormatter, language: "elixir"})

      assert detail =~ "Lumis.FormatterTest.RaisingFormatter.render/3"
      assert detail =~ "(ArgumentError) cannot render"
    end

    test "makes highlight!/2 raise" do
      error =
        assert_raise Lumis.HighlightError, fn ->
          Lumis.highlight!("x = 1", formatter: {RaisingFormatter, language: "elixir"})
        end

      assert %Lumis.RenderError{reason: :custom_formatter} = error.error

      assert Exception.message(error.error) ==
               "custom formatter failed in Lumis.FormatterTest.RaisingFormatter.render/3: " <>
                 "** (ArgumentError) cannot render"
    end

    test "includes returning something other than iodata" do
      assert {:error, %Lumis.RenderError{reason: :custom_formatter, detail: detail}} =
               Lumis.highlight("x = 1", formatter: {NotIodataFormatter, language: "elixir"})

      assert detail =~ "NotIodataFormatter.render/3"
    end
  end
end

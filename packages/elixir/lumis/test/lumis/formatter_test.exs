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
    :ok = Lumis.Languages.load(~w(elixir bash))
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
end

defmodule Lumis.FormatterOptionsTest do
  @moduledoc """
  Elixir's half of the cross-runtime formatter option check.

  `fixtures/formatter-options.json` lists the options every covered runtime must accept.
  `Lumis.formatter_type/1` returns the validated defaults for a formatter, so
  unlike the other runtimes this can read the option names straight off the
  schema rather than naming them again.
  """
  use ExUnit.Case, async: false

  @manifest_path Path.expand("../../../../fixtures/formatter-options.json", __DIR__)
  @external_resource @manifest_path

  # Read at runtime rather than into a module attribute: inlining the decoded
  # JSON gives the compiler a literal type precise enough to warn on `Map.keys/1`.
  defp manifest, do: @manifest_path |> File.read!() |> Jason.decode!()

  defp accepted_options(formatter) do
    {:ok, {^formatter, opts}} = Lumis.formatter_type(formatter)

    opts
    |> Keyword.keys()
    |> Enum.map(&Atom.to_string/1)
    |> Enum.sort()
  end

  defp manifest_options(formatter) do
    manifest()
    |> Map.fetch!("formatters")
    |> Map.fetch!(formatter)
    |> Map.fetch!("options")
    |> Enum.map(& &1["name"])
    |> Enum.sort()
  end

  test "covers all five formatters" do
    assert manifest() |> Map.fetch!("formatters") |> Map.keys() |> Enum.sort() ==
             ~w(bbcode_scoped html_inline html_linked html_multi_themes terminal)
  end

  # `html_multi_themes` requires `:themes`, so its defaults cannot be built from
  # an empty list the way the others can.
  for formatter <- [:html_inline, :html_linked, :terminal, :bbcode_scoped] do
    test "#{formatter} accepts every option in the manifest" do
      formatter = unquote(formatter)

      assert accepted_options(formatter) == manifest_options(Atom.to_string(formatter))
    end
  end

  test "html_multi_themes accepts every option in the manifest" do
    {:ok, {:html_multi_themes, opts}} =
      Lumis.formatter_type({:html_multi_themes, themes: [light: "github_light"]})

    accepted = opts |> Keyword.keys() |> Enum.map(&Atom.to_string/1) |> Enum.sort()

    assert accepted == manifest_options("html_multi_themes")
  end

  test "italic warns only when supplied and preserves defaults and explicit values" do
    for {formatter, required} <- [
          {:html_inline, []},
          {:html_multi_themes, [themes: [light: "github_light"]]}
        ] do
      assert ExUnit.CaptureIO.capture_io(:stderr, fn ->
               assert {:ok, {^formatter, options}} = Lumis.formatter_type({formatter, required})
               assert options[:italic] == false
             end) == ""

      for italic <- [false, true] do
        warning =
          ExUnit.CaptureIO.capture_io(:stderr, fn ->
            assert {:ok, {^formatter, options}} =
                     Lumis.formatter_type({formatter, Keyword.put(required, :italic, italic)})

            assert options[:italic] == italic
          end)

        assert warning =~ "deprecated"
        assert warning =~ "Lumis.Theme.map_styles/2"
      end
    end
  end

  test "no waiver outlives its reason" do
    waivers =
      manifest()
      |> Map.fetch!("waived")
      |> Map.keys()
      |> Enum.reject(&String.starts_with?(&1, "$"))

    assert waivers == [],
           "every covered runtime offers every option; drop these waivers: #{inspect(waivers)}"
  end
end

defmodule Lumis.ApiParityTest do
  @moduledoc """
  Elixir's half of the cross-runtime top-level API check.

  `fixtures/api.json` lists the entry points every runtime must offer a caller. A
  module reports its own functions, so this reflects rather than naming them
  again:

  - "exports every capability in the manifest" fails on a capability Elixir
    lacks. That is the direction that stayed silent for `highlight_events/3`
    until #1543: Rust and JavaScript had it, Elixir handed the event stream only
    to a custom formatter's `render/3`, and nothing failed.
  - "exports nothing the manifest does not account for" fails on a public
    function in neither the manifest's capability set nor `runtime_only`. This is
    the one that catches drift: an entry point added here and nowhere else has to
    be classified before it can ship.
  - "still exports every runtime_only entry point" fails on a `runtime_only` name
    that is gone, so the list cannot outlive its reasons.
  - "no waiver outlives its reason" fails on a waived capability Elixir has since
    grown, so `waived` can only shrink.

  `@doc false` is not public API, so `Code.fetch_docs/1` filters it: that is what
  keeps the option-schema and NIF plumbing out of the manifest. Arity is not
  checked either. Elixir keeps its own argument shapes -- a keyword list where
  Rust takes a builder -- and only the capability has to exist everywhere.
  """
  use ExUnit.Case, async: true

  @manifest_path Path.expand("../../../../fixtures/api.json", __DIR__)
  @external_resource @manifest_path

  @runtime "elixir"

  # The module a caller reaches first. A capability living anywhere else carries a
  # `spelling` naming its home, and is checked by being called below.
  @surface Lumis

  # Read at runtime rather than into a module attribute: inlining the decoded
  # JSON gives the compiler a literal type precise enough to warn on `Map.keys/1`.
  defp manifest, do: @manifest_path |> File.read!() |> Jason.decode!()

  defp elixir_names(capability) do
    case get_in(capability, ["spelling", @runtime]) do
      nil -> [capability["name"]]
      names when is_list(names) -> names
      name -> [name]
    end
  end

  # Every name Elixir is required to offer: the canonical set minus what `waived`
  # exempts it from.
  defp required_names do
    waived = section_names("waived")

    manifest()
    |> Map.fetch!("capabilities")
    |> Enum.reject(&(&1["name"] in waived))
    |> Enum.flat_map(&elixir_names/1)
    |> Enum.sort()
  end

  # Names the reflection can see. A spelling naming a home outside `@surface`
  # contains a `.`, and the call below is what proves it.
  defp required_in_scope do
    Enum.reject(required_names(), &String.contains?(&1, "."))
  end

  defp section_names(section) do
    manifest()
    |> Map.fetch!(section)
    |> Map.get(@runtime, %{})
    |> Map.keys()
    |> Enum.reject(&String.starts_with?(&1, "$"))
    |> Enum.sort()
  end

  # Public functions, minus `@doc false`. `Code.fetch_docs/1` reports those as
  # `:hidden`, which is the only way to tell them from an undocumented export.
  defp exported(module) do
    {:docs_v1, _, :elixir, _, _, _, docs} = Code.fetch_docs(module)

    hidden =
      for {{:function, name, _arity}, _anno, _sig, :hidden, _meta} <- docs,
          do: Atom.to_string(name)

    module.__info__(:functions)
    |> Enum.map(fn {name, _arity} -> Atom.to_string(name) end)
    |> Enum.uniq()
    |> Enum.reject(&(&1 in hidden))
    |> Enum.sort()
  end

  test "exports every capability in the manifest" do
    exported = exported(@surface)
    missing = Enum.reject(required_in_scope(), &(&1 in exported))

    assert missing == [],
           "#{inspect(@surface)} is missing #{inspect(missing)}. Add them, or waive them in " <>
             "fixtures/api.json with a reason."
  end

  test "exports nothing the manifest does not account for" do
    # A waived name is accounted for, so a runtime that grows one fails
    # "no waiver outlives its reason" alone rather than here as well.
    accounted = required_in_scope() ++ section_names("runtime_only") ++ section_names("waived")
    unaccounted = Enum.reject(exported(@surface), &(&1 in accounted))

    assert unaccounted == [],
           "#{inspect(@surface)} exports #{inspect(unaccounted)}, which is in neither the " <>
             "manifest's capability set nor runtime_only. Add them to fixtures/api.json and " <>
             "to the other runtimes, or classify them."
  end

  test "still exports every runtime_only entry point" do
    exported = exported(@surface)
    gone = Enum.reject(section_names("runtime_only"), &(&1 in exported))

    assert gone == [],
           "runtime_only lists #{inspect(gone)}, which #{inspect(@surface)} no longer exports; " <>
             "drop the entry"
  end

  test "no waiver outlives its reason" do
    exported = exported(@surface)
    kept = Enum.filter(section_names("waived"), &(&1 in exported))

    assert kept == [], "Elixir offers #{inspect(kept)}; drop the waiver"
  end

  test "every waiver names a capability" do
    names = manifest() |> Map.fetch!("capabilities") |> Enum.map(& &1["name"])

    for {runtime, waived} <- Map.fetch!(manifest(), "waived"),
        not String.starts_with?(runtime, "$"),
        name <- Map.keys(waived),
        not String.starts_with?(name, "$") do
      assert name in names,
             "waived.#{runtime} lists #{name}, which is not a capability in this manifest; " <>
               "a waiver only exempts a runtime from the canonical set"
    end
  end

  describe "every capability the manifest claims for Elixir is callable" do
    @source "defmodule A do\n  @x 1\nend\n"

    test "highlight" do
      assert {:ok, html} = Lumis.highlight(@source, formatter: {:html_inline, language: "elixir"})
      assert html =~ "defmodule"
      assert Lumis.highlight!(@source, formatter: {:html_inline, language: "elixir"}) == html
    end

    test "highlight_events" do
      assert {:ok, events} = Lumis.highlight_events(@source, "elixir")
      assert events != []
      assert Lumis.highlight_events!(@source, "elixir") == events
    end

    test "guess" do
      assert Lumis.Languages.guess("ex", "") == "elixir"
      assert Lumis.Languages.guess("lib/a.ex", "") == "elixir"
    end

    test "available_languages" do
      assert Lumis.available_languages() != []
    end

    test "available_themes" do
      assert Lumis.available_themes() != []
    end

    test "language_info" do
      assert Lumis.Languages.get("ex").id == "elixir"
    end

    test "the manifest claims exactly these" do
      claimed =
        manifest()
        |> Map.fetch!("capabilities")
        |> Enum.map(& &1["name"])
        |> Enum.reject(&(&1 in section_names("waived")))
        |> Enum.sort()

      assert claimed == [
               "available_languages",
               "available_themes",
               "guess",
               "highlight",
               "highlight_events",
               "language_info"
             ],
             "add a test above for the capabilities this list gained"
    end
  end
end

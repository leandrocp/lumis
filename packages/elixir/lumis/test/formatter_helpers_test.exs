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

  Arity is not checked. Elixir keeps its own argument shapes — a table where Rust
  takes a theme, a keyword list where JavaScript takes an object — and only the
  capability has to exist everywhere.
  """
  use ExUnit.Case, async: true

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

  test "covers the modules Elixir publishes" do
    assert manifest() |> Map.fetch!("modules") |> Map.keys() |> Enum.sort() ==
             @modules |> Map.keys() |> Enum.sort()
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

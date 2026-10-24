defmodule Lumis.ThemesTest do
  use ExUnit.Case, async: true

  test "discovery ships independently of either highlighting NIF" do
    assert :code.which(Lumis.Native) == :non_existing
    assert :code.which(MDEx.Native) == :non_existing
    assert [file] = Lumis.Themes.installed_files()
    assert Path.basename(file) == "lumis-themes.json"
    assert File.regular?(file)
    assert Mix.Project.config()[:deps] == []
  end
end

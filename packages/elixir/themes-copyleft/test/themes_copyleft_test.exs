defmodule LumisThemesCopyleftTest do
  use ExUnit.Case, async: true

  test "the optional package contains only assets and has no runtime dependencies" do
    assert Mix.Project.config()[:deps] == []
    assert Application.spec(:lumis_themes_copyleft, :modules) == []

    file = Path.join(to_string(:code.priv_dir(:lumis_themes_copyleft)), "lumis-themes.json")
    assert File.regular?(file)
  end
end

defmodule Lumis.Native.ArtifactURLTest do
  use ExUnit.Case, async: true

  alias Lumis.Native.ArtifactURL

  @version Mix.Project.config()[:version]
  @file_name "liblumis_nif-v#{@version}-nif-2.16-x86_64-unknown-linux-gnu.so.tar.gz"

  test "defaults to GitHub releases" do
    assert ArtifactURL.url(@file_name) ==
             "https://github.com/leandrocp/lumis/releases/download/hex-lumis/v#{@version}/#{@file_name}"
  end

  test "serves the same file name from the Cloudflare mirror" do
    assert ArtifactURL.url(@file_name, :cloudflare) ==
             "https://artifacts.lumis.sh/releases/download/hex-lumis/v#{@version}/#{@file_name}"
  end

  test "both sources agree on everything after the base URL" do
    "https://github.com/leandrocp/lumis/releases/download/" <> github =
      ArtifactURL.url(@file_name, :github)

    "https://artifacts.lumis.sh/releases/download/" <> cloudflare =
      ArtifactURL.url(@file_name, :cloudflare)

    assert github == cloudflare
  end

  test "rejects a source that is not one of the two" do
    # Built at runtime so the type checker cannot rule it out the way it does a
    # literal, which is also how a source reaches us from config or the environment.
    source = String.to_atom("s3")

    assert_raise FunctionClauseError, fn -> ArtifactURL.url(@file_name, source) end
  end
end

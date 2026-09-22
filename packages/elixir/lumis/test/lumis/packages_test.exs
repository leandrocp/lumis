defmodule Lumis.PackagesTest do
  use ExUnit.Case, async: false

  alias Lumis.Packages

  setup do
    previous = Application.get_env(:lumis, :parser_dirs)
    on_exit(fn -> Application.put_env(:lumis, :parser_dirs, previous) end)
    :ok
  end

  describe "finding parsers" do
    test "the configured directories are declared" do
      dir = Path.join(System.tmp_dir!(), "lumis-packages-#{System.unique_integer([:positive])}")
      File.mkdir_p!(dir)
      on_exit(fn -> File.rm_rf!(dir) end)

      Application.put_env(:lumis, :parser_dirs, [dir])

      assert Path.expand(dir) in Packages.installed_dirs()
    end

    # Relative paths are expanded so the answer does not depend on where the VM
    # was started, which for a release is wherever the init system put it.
    test "a relative directory is expanded" do
      Application.put_env(:lumis, :parser_dirs, ["parsers"])

      assert Packages.installed_dirs() == [Path.expand("parsers")]
    end

    test "declaring nothing is an empty list, not an error" do
      Application.put_env(:lumis, :parser_dirs, [])

      assert Packages.installed_dirs() == []
    end

    test "the same directory named twice is listed once" do
      Application.put_env(:lumis, :parser_dirs, ["parsers", "parsers"])

      assert Packages.installed_dirs() == [Path.expand("parsers")]
    end

    # Only applications whose name marks them as parsers. Scanning every
    # directory on the code path would make any dependency that happened to ship
    # a `parsers` directory into a declaration.
    test "only lumis_wasm_* applications count" do
      refute "lumis" in Packages.applications()

      for app <- Packages.applications() do
        assert String.starts_with?(app, Packages.prefix())
      end
    end

    # A release names the directory with its version and never loads a parser
    # application, since nothing depends on it at the OTP level. Reading the
    # code path covers both that and the undecorated Mix layout.
    test "a versioned release directory is still recognised" do
      dir = Path.join(System.tmp_dir!(), "lumis-rel-#{System.unique_integer([:positive])}")
      root = Path.join(dir, "lumis_wasm_fake-0.26.3")
      File.mkdir_p!(Path.join(root, "ebin"))
      File.mkdir_p!(Path.join([root, "priv", "parsers"]))
      on_exit(fn -> File.rm_rf!(dir) end)

      Code.append_path(Path.join(root, "ebin"))
      on_exit(fn -> Code.delete_path(Path.join(root, "ebin")) end)

      assert "lumis_wasm_fake-0.26.3" in Packages.applications()
      assert Path.join([root, "priv", "parsers"]) in Packages.installed_dirs()
    end
  end

  describe "what the suite itself declares" do
    # The suite points `:parser_dirs` at the staged fixtures rather than taking
    # an escape hatch, so these tests exercise the path a real project takes.
    test "the staged fixtures are what makes highlighting work here" do
      assert [_ | _] = dirs = Packages.installed_dirs()
      assert Enum.any?(dirs, &(Path.wildcard(Path.join(&1, "*.lumis.json")) != []))
    end

    test "a language outside the staged set is refused rather than downloaded" do
      assert {:error, :not_installed} = Lumis.Languages.load("erlang")
    end

    test "a language inside it loads" do
      assert :ok = Lumis.Languages.load("ruby")
    end
  end
end

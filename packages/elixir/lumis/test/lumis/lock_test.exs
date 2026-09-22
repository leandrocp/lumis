defmodule Lumis.LockTest do
  use ExUnit.Case, async: false

  alias Lumis.Lock

  # These drive the lock through explicit paths rather than the configured store,
  # because the store is built once at boot: a lock placed in the test data
  # directory would govern every other test in the suite.

  setup do
    dir = Path.join(System.tmp_dir!(), "lumis-lock-test-#{System.unique_integer([:positive])}")
    File.mkdir_p!(dir)
    on_exit(fn -> File.rm_rf!(dir) end)
    {:ok, dir: dir, lock: Path.join(dir, Lock.file_name())}
  end

  describe "managing a lock" do
    @tag :tmp_dir
    test "add records a package and its language", %{lock: lock} do
      assert {:ok, [change]} = Lumis.Native.lock_add(lock, ["json"])
      assert change.package == "@lumis-sh/wasm-json"
      assert change.previous_version == nil
      refute change.moved

      contents = File.read!(lock)
      assert contents =~ ~s(name = "@lumis-sh/wasm-json")
      assert contents =~ ~s(languages = ["json"])
      assert contents =~ "manifest_sha256"
    end

    test "two languages from one package share a row", %{lock: lock} do
      assert {:ok, _} = Lumis.Native.lock_add(lock, ["ejs"])
      assert {:ok, _} = Lumis.Native.lock_add(lock, ["erb"])

      contents = File.read!(lock)
      # A multi-element array is written one language per line, so a later `add`
      # shows up as a single added line rather than a rewritten one.
      assert contents =~ ~s("ejs")
      assert contents =~ ~s("erb")

      assert length(String.split(contents, "[[package]]")) == 2,
             "ejs and erb are both @lumis-sh/wasm-embedded-template, so one row:\n#{contents}"
    end

    test "removing one language keeps a package the other still needs", %{lock: lock} do
      assert {:ok, _} = Lumis.Native.lock_add(lock, ["ejs", "erb"])
      assert {:ok, {["erb"], []}} = Lumis.Native.lock_remove(lock, ["erb"])

      assert {:ok, ["ejs"]} = Lumis.Native.lock_languages(lock)
    end

    test "removing the last language drops the package", %{lock: lock} do
      assert {:ok, _} = Lumis.Native.lock_add(lock, ["json"])
      assert {:ok, {["json"], []}} = Lumis.Native.lock_remove(lock, ["json"])
      assert {:ok, []} = Lumis.Native.lock_languages(lock)
    end

    test "removing something that was never pinned reports nothing removed", %{lock: lock} do
      assert {:ok, _} = Lumis.Native.lock_add(lock, ["json"])
      assert {:ok, {[], ["haskell"]}} = Lumis.Native.lock_remove(lock, ["haskell"])
      assert {:ok, ["json"]} = Lumis.Native.lock_languages(lock)
    end

    test "a bundle expands and its name is not recorded", %{lock: lock} do
      assert {:ok, changes} = Lumis.Native.lock_add(lock, ["bundle-web"])
      assert length(changes) > 1

      contents = File.read!(lock)
      refute contents =~ "bundle-web", "the members are recorded, the bundle name is not"
      assert contents =~ ~s(languages = ["css"])
    end

    test "update refuses a language that is not locked", %{lock: lock} do
      assert {:ok, _} = Lumis.Native.lock_add(lock, ["json"])

      assert {:error, message} = Lumis.Native.lock_update(lock, ["haskell"], false)
      assert message =~ "haskell is not in #{Lock.file_name()}"
    end

    test "update --all leaves an already-current lock unmoved", %{lock: lock} do
      assert {:ok, _} = Lumis.Native.lock_add(lock, ["json"])
      assert {:ok, changes} = Lumis.Native.lock_update(lock, [], true)
      assert Enum.all?(changes, &(not &1.moved))
    end

    # Removing a bundle removes its members; the bundle name is never recorded,
    # so reporting against the raw argument would call a success a miss.
    test "removing a bundle reports its members, not the bundle name", %{lock: lock} do
      assert {:ok, _} = Lumis.Native.lock_add(lock, ["bundle-web"])
      assert {:ok, {removed, missing}} = Lumis.Native.lock_remove(lock, ["bundle-web"])

      assert "css" in removed
      assert missing == [], "every member was pinned, so nothing is missing"
      refute "bundle-web" in missing
    end

    test "remove requires an existing lock", %{lock: lock} do
      assert {:error, message} = Lumis.Native.lock_remove(lock, ["json"])
      assert message =~ Lock.file_name()
    end

    test "a lock from another tree-sitter series names the fix", %{lock: lock} do
      File.write!(lock, ~s(version = 1\nrange = "0.1"\n))

      assert {:error, message} = Lumis.Native.lock_languages(lock)
      assert message =~ "0.1"
      assert message =~ "update --all"
    end

    test "reading where there is no lock is not an error", %{lock: lock} do
      assert {:ok, nil} = Lumis.Native.lock_languages(lock)
    end
  end

  describe "finding the lock" do
    test "an explicit config path wins", %{dir: dir, lock: lock} do
      File.write!(lock, ~s(version = 1\nrange = "0.26"\n))
      Application.put_env(:lumis, :lock, lock)
      on_exit(fn -> Application.delete_env(:lumis, :lock) end)

      assert Lock.path() == Path.expand(lock)
      assert dir != nil
    end

    test "project_path walks up to an existing lock", %{dir: dir, lock: lock} do
      File.write!(lock, ~s(version = 1\nrange = "0.26"\n))
      nested = Path.join([dir, "apps", "web"])
      File.mkdir_p!(nested)

      in_dir(nested, fn ->
        assert {:ok, found} = Lock.project_path(false)
        assert Path.expand(found) == Path.expand(lock)
      end)
    end

    test "project_path refuses to invent one unless asked", %{dir: dir} do
      in_dir(dir, fn ->
        assert {:error, message} = Lock.project_path(false)
        assert message =~ Lock.file_name()
      end)
    end

    # Deliberately the repository root rather than the working directory, so a
    # task run inside an umbrella child still writes the repository's one lock.
    test "project_path creates at the repository root", %{dir: dir} do
      File.mkdir_p!(Path.join(dir, ".git"))
      nested = Path.join([dir, "apps", "web"])
      File.mkdir_p!(nested)

      in_dir(nested, fn ->
        assert {:ok, path} = Lock.project_path(true)
        assert Path.expand(path) == Path.expand(Path.join(dir, Lock.file_name()))
      end)
    end

    # In a worktree `.git` is a file holding a gitlink, not a directory. Checking
    # for a directory walks past the worktree into whatever repository encloses
    # it, which is how an early version wrote the lock outside the checkout.
    test "project_path stops at a git worktree", %{dir: dir} do
      File.write!(Path.join(dir, ".git"), "gitdir: /somewhere/.git/worktrees/x\n")
      nested = Path.join([dir, "apps", "web"])
      File.mkdir_p!(nested)

      in_dir(nested, fn ->
        assert {:ok, path} = Lock.project_path(true)
        assert Path.expand(path) == Path.expand(Path.join(dir, Lock.file_name()))
      end)
    end
  end

  describe "syncing into the data directory" do
    test "copies the lock so a release reads what was pinned", %{lock: lock} do
      File.write!(lock, ~s(version = 1\nrange = "0.26"\n))

      assert :ok = Lock.sync(lock)

      copy = Path.join(Lumis.Application.resolved_data_dir(), Lock.file_name())
      on_exit(fn -> File.rm_rf!(copy) end)
      assert File.read!(copy) == File.read!(lock)
    end

    test "syncing a lock that is already the copy is a no-op", %{dir: dir} do
      copy = Path.join(Lumis.Application.resolved_data_dir(), Lock.file_name())
      File.mkdir_p!(Path.dirname(copy))
      File.write!(copy, ~s(version = 1\nrange = "0.26"\n))
      on_exit(fn -> File.rm_rf!(copy) end)

      assert :ok = Lock.sync(copy)
      assert File.read!(copy) =~ "0.26"
      assert dir != nil
    end
  end

  defp in_dir(dir, fun) do
    previous = File.cwd!()
    File.cd!(dir)

    try do
      fun.()
    after
      File.cd!(previous)
    end
  end
end

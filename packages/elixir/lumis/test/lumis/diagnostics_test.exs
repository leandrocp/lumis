defmodule Lumis.DiagnosticsTest do
  use ExUnit.Case, async: false

  import ExUnit.CaptureLog

  alias Lumis.Diagnostics

  setup do
    Diagnostics.reset()
    previous = Application.get_env(:lumis, :report_unresolved)

    on_exit(fn ->
      Diagnostics.reset()

      case previous do
        nil -> Application.delete_env(:lumis, :report_unresolved)
        value -> Application.put_env(:lumis, :report_unresolved, value)
      end
    end)

    :ok
  end

  test "names the language and what to do about it" do
    log = capture_log(fn -> Diagnostics.report(["haskell"]) end)

    assert log =~ "haskell"
    assert log =~ "mix lumis.add haskell"
    assert log =~ "report_unresolved: false"
  end

  # A README with twenty fenced blocks in one uninstalled language is one thing
  # to fix. Twenty identical lines would bury the next one that matters.
  test "reports a language once, not once per document" do
    assert capture_log(fn -> Diagnostics.report(["haskell"]) end) =~ "haskell"
    assert capture_log(fn -> Diagnostics.report(["haskell"]) end) == ""
  end

  test "a second language is still worth saying" do
    assert capture_log(fn -> Diagnostics.report(["haskell"]) end) =~ "haskell"

    log = capture_log(fn -> Diagnostics.report(["erlang"]) end)
    assert log =~ "erlang"
  end

  test "the setting silences it" do
    Application.put_env(:lumis, :report_unresolved, false)

    refute Diagnostics.report_unresolved?()
  end

  test "reporting is on unless something turns it off" do
    Application.delete_env(:lumis, :report_unresolved)

    assert Diagnostics.report_unresolved?()
  end

  test "nothing skipped says nothing" do
    assert capture_log(fn -> Diagnostics.report([]) end) == ""
  end
end

import { afterEach, describe, expect, it, vi } from "vitest";
import {
  resetUnresolvedReports,
  setReportUnresolved,
  warnUnresolvedInjection,
} from "../src/events.js";

afterEach(() => {
  resetUnresolvedReports();
  vi.restoreAllMocks();
});

describe("reporting a language highlighting skipped", () => {
  it("names the language, once", () => {
    const warn = vi.spyOn(console, "warn").mockImplementation(() => {});

    warnUnresolvedInjection("haskell");
    warnUnresolvedInjection("haskell");

    expect(warn).toHaveBeenCalledOnce();
    expect(warn.mock.calls[0]?.[0]).toContain("haskell");
  });

  it("stays quiet once the switch is off", () => {
    const warn = vi.spyOn(console, "warn").mockImplementation(() => {});

    setReportUnresolved(false);
    warnUnresolvedInjection("haskell");

    expect(warn).not.toHaveBeenCalled();
  });

  // Turning reporting back on must actually report, or the switch would be a
  // one-way door and a test suite that flipped it would silence every later one.
  it("reports again once the switch is back on", () => {
    const warn = vi.spyOn(console, "warn").mockImplementation(() => {});

    setReportUnresolved(false);
    warnUnresolvedInjection("haskell");
    setReportUnresolved(true);
    warnUnresolvedInjection("haskell");

    expect(warn).toHaveBeenCalledOnce();
  });

  // html captures the raw `<script type=...>` value, so "module" reaches this
  // function just before a later pattern injects javascript into the same
  // block. That block highlights, so saying anything would be a false alarm.
  it("ignores a name that is not a language", () => {
    const warn = vi.spyOn(console, "warn").mockImplementation(() => {});

    warnUnresolvedInjection("module");
    warnUnresolvedInjection("importmap");

    expect(warn).not.toHaveBeenCalled();
  });
});

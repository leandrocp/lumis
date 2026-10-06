/**
 * `platformDataDir` is a port of `etcetera::choose_base_strategy`, the resolution
 * the addon, the CLI and the Elixir NIF all run in Rust. Node normally asks the
 * addon and never uses the port, but the port answers wherever no addon is
 * built, so it has to agree with Rust or the two runtimes would keep separate
 * stores. That divergence already shipped once: the port picked
 * `~/Library/Application Support` on macOS while every Rust caller used XDG.
 */
import { afterEach, describe, expect, it } from "vitest";
import { loadAddon, nativeTarget } from "../src/native-binding.js";
import { platformDataDir } from "../src/runtime/node-cache.js";

// Asserted directly, so a run that selected the Wasm runtime still covers this.
const binding = loadAddon();
const hasPrebuiltAddon = nativeTarget() !== undefined;

const XDG = "XDG_DATA_HOME";
const original = process.env[XDG];

function setXdg(value: string | undefined): void {
  if (value === undefined) {
    delete process.env[XDG];
  } else {
    process.env[XDG] = value;
  }
}

afterEach(() => {
  setXdg(original);
});

describe("default data directory", () => {
  /**
   * The parity cases below need an addon to compare against, so a prebuilt
   * target that has not built one fails here rather than skipping them. Only a
   * target with no addon at all leaves parity unverifiable, and there the Wasm
   * runtime is the sole reader of the directory, so nothing can disagree.
   */
  it("has an addon to pin the port against wherever one is built", () => {
    if (hasPrebuiltAddon && !binding) {
      throw new Error("run `pnpm build:native` (or `mise run test-javascript`) first");
    }
    expect(binding !== undefined).toBe(hasPrebuiltAddon);
  });

  // `process.env` writes reach `std::env::var_os`, so the addon re-reads each of
  // these the same way a fresh process would.
  const cases: { name: string; xdg: string | undefined }[] = [
    { name: "unset", xdg: undefined },
    {
      name: "an absolute path",
      xdg: process.platform === "win32" ? "C:\\lumis-xdg" : "/tmp/lumis-xdg",
    },
    // The XDG spec says a relative value is ignored; `etcetera` implements that.
    { name: "a relative path", xdg: "relative/lumis-xdg" },
    { name: "empty", xdg: "" },
  ];

  for (const { name, xdg } of cases) {
    it.runIf(binding)(`matches the addon with XDG_DATA_HOME ${name}`, async () => {
      setXdg(xdg);
      expect(await platformDataDir()).toBe(binding!.defaultDataDir());
    });
  }

  it("ends at a lumis directory", async () => {
    setXdg(undefined);
    expect(await platformDataDir()).toMatch(/[/\\]lumis$/u);
  });
});

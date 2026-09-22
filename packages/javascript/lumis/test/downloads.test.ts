/**
 * `package.json` pins what a project installed, but an injection can name a
 * language it never installed and that one still reaches a CDN — so the set of
 * languages a JavaScript project can use is open even when it looks closed.
 * Rust has no such gap: a language that was not compiled in does not exist.
 *
 * `configureDownloads(false)` closes it. These run on both runtimes, because
 * Node resolves through the addon by default and the browser through
 * web-tree-sitter: a switch only one of them honoured would make the answer
 * depend on whether an addon happened to be built for the platform.
 */
import { mkdtempSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";

// Its own store, and deliberately an empty one: what is under test is what
// happens when a language is *not* already there.
process.env.LUMIS_DATA_DIR = mkdtempSync(join(tmpdir(), "lumis-downloads-"));

const { configureDownloads, loadLanguages, loadedLanguages } = await import("../src/index.js");
const { configureLocalWasmResolver } = await import("./wasm.js");

// `loadLanguages` rejects with a wrapper whose message names no reason, and
// buries the real one in an `AggregateError`'s `errors` and then a `cause`
// chain. Asserting on the wrapper alone would pass for any load failure at all.
function reasons(error: unknown, found: string[] = []): string[] {
  if (!(error instanceof Error)) return found;
  found.push(error.message);
  for (const nested of (error as AggregateError).errors ?? []) reasons(nested, found);
  reasons((error as { cause?: unknown }).cause, found);
  return found;
}

async function expectRefused(promise: Promise<unknown>): Promise<void> {
  const error = await promise.then(
    () => undefined,
    (reason: unknown) => reason,
  );
  expect(error, "expected the load to be refused").toBeDefined();
  // The wasm path says "downloads are disabled"; the addon's store says
  // "network access is disabled". Either proves the switch refused, rather than
  // the download merely failing for some other reason.
  expect(reasons(error).join(" | ")).toMatch(/downloads are disabled|network access is disabled/);
}

afterEach(() => {
  vi.restoreAllMocks();
  // Restore the default, or a later test inherits whichever one ran last.
  configureDownloads(true);
});

describe("configureDownloads", () => {
  it("resolves an uninstalled language by default", async () => {
    configureLocalWasmResolver(["json"]);

    await expect(loadLanguages(["json"])).resolves.toBeTruthy();
    expect(loadedLanguages()).toContain("json");
  });

  it("refuses to resolve one once downloads are off", async () => {
    configureDownloads(false);

    await expectRefused(loadLanguages(["elixir"]));
  });

  it("makes no request at all rather than failing after one", async () => {
    const fetchSpy = vi.spyOn(globalThis, "fetch");
    configureDownloads(false);

    await expectRefused(loadLanguages(["ruby"]));
    expect(fetchSpy).not.toHaveBeenCalled();
  });

  it("still serves what the project already has", async () => {
    configureLocalWasmResolver(["css"]);
    await loadLanguages(["css"]);

    // Warm, so it comes from the store rather than being resolved again.
    configureDownloads(false);
    await expect(loadLanguages(["css"])).resolves.toBeTruthy();
    expect(loadedLanguages()).toContain("css");
  });

  it("can be turned back on", async () => {
    configureDownloads(false);
    await expectRefused(loadLanguages(["javascript"]));

    configureLocalWasmResolver(["javascript"]);
    configureDownloads(true);
    await expect(loadLanguages(["javascript"])).resolves.toBeTruthy();
    expect(loadedLanguages()).toContain("javascript");
  });
});

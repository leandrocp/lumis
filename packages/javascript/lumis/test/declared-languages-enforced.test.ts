/**
 * The declaration enforced end to end, in its own file because it changes the
 * working directory before importing Lumis: `declaresLanguages` is read once per
 * process, so a suite that imports first would memoize the wrong answer.
 *
 * Both runtimes have to agree. Node resolves through the addon, which fetches in
 * Rust without coming back to JavaScript, so a check only the TypeScript side
 * made would refuse in the browser and download on the server.
 */
import { mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";

// A project that declares one parser it does not have on disk, so resolving it
// is refused rather than fetched.
const root = mkdtempSync(join(tmpdir(), "lumis-declared-project-"));
writeFileSync(
  join(root, "package.json"),
  JSON.stringify({ name: "declared", dependencies: { "@lumis-sh/wasm-json": "^0.26" } }),
);
process.chdir(root);
process.env.LUMIS_DATA_DIR = mkdtempSync(join(tmpdir(), "lumis-declared-store-"));

const { loadLanguages } = await import("../src/index.js");

function reasons(error: unknown, found: string[] = []): string[] {
  if (!(error instanceof Error)) return found;
  found.push(error.message);
  for (const nested of (error as AggregateError).errors ?? []) reasons(nested, found);
  reasons((error as { cause?: unknown }).cause, found);
  return found;
}

describe("a project that declares its languages", () => {
  // The complement, and the reason this is safe to turn on by default: a
  // project that declared nothing still resolves whatever a document names.
  it("is the declaration that closes the set, not the feature existing", async () => {
    const { declaresLanguages } = await import("../src/runtime/node-cache.js");
    await expect(declaresLanguages()).resolves.toBe(true);
  });

  it("refuses one it did not declare", async () => {
    const error = await loadLanguages(["elixir"]).then(
      () => undefined,
      (reason: unknown) => reason,
    );

    expect(error, "expected the load to be refused").toBeDefined();
    // The TypeScript path names the package; the addon's store says the network
    // is disabled. Either proves the declaration was enforced rather than the
    // download merely failing.
    expect(reasons(error).join(" | ")).toMatch(
      /this project depends on|network access is disabled/,
    );
  });
});

/**
 * The declaration enforced end to end, in its own file because it changes the
 * working directory before importing Lumis.
 *
 * Both runtimes have to agree. Node resolves through the addon, which fetches in
 * Rust without coming back to JavaScript, so a check only the TypeScript side
 * made would refuse in the browser and download on the server.
 */
import { mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";

// A project with no parser installed. Nothing is declared, and under the rule
// every runtime now shares, nothing is what it gets.
const root = mkdtempSync(join(tmpdir(), "lumis-declared-project-"));
writeFileSync(join(root, "package.json"), JSON.stringify({ name: "declared", dependencies: {} }));
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

describe("a project that installed no parser", () => {
  // The rule this file exists for: installing no parser is not "declared
  // nothing, so allow everything". It is "declared nothing, so load nothing" —
  // the same answer Rust gives for a language you did not compile in and Elixir
  // gives for one you did not depend on.
  it("refuses a language when the project installed none", async () => {
    const error = await loadLanguages(["elixir"]).then(
      () => undefined,
      (reason: unknown) => reason,
    );

    expect(error, "expected the load to be refused").toBeDefined();
    // The TypeScript path names the package; the addon says it is not one the
    // project declared. Either proves the declaration was enforced, rather than
    // the download merely failing for some other reason.
    expect(reasons(error).join(" | ")).toMatch(/this project (depends on|declares)/);
  });
});

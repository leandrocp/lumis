/**
 * The switch on the paths a project actually takes, with no resolver configured.
 *
 * Its own file on purpose. `configureLocalWasmResolver` is global and sticky, so
 * a suite that calls it makes every later load caller-resolved, which routes
 * through the TypeScript pipeline. Sharing a file with those tests is how this
 * one silently stops testing anything.
 *
 * Two different paths are covered here, and they are enforced in two places:
 *
 *   - Naming a language resolves its package in TypeScript first, so the guard
 *     in `core/languages.ts` is what refuses.
 *   - A language an *injection* names during a native walk is resolved by the
 *     addon itself, in Rust, without coming back to TypeScript. Only the
 *     addon's own `NoNetwork` store refuses that one.
 *
 * The second is the case this whole change exists for, and it is the one a
 * TypeScript-only guard would miss.
 */
import { mkdtempSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { beforeEach, describe, expect, it } from "vitest";

// Empty, and only ever populated with markdown below. Ruby must stay absent.
process.env.LUMIS_DATA_DIR = mkdtempSync(join(tmpdir(), "lumis-downloads-default-"));

const { configureDownloads, highlight, loadLanguages } = await import("../src/index.js");
const { htmlInline } = await import("../src/formatters.js");

// The addon keeps one store for the process, so the switch is global and
// sticky. Set it per test rather than restoring afterwards, or the order tests
// happen to run in decides what they mean.
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

beforeEach(() => {
  configureDownloads(true);
});

describe("configureDownloads on the default resolution path", () => {
  it("refuses a named language the project does not have", async () => {
    configureDownloads(false);

    await expectRefused(loadLanguages(["elixir"]));
  });

  it("costs one block, not the document, when an injection cannot be resolved", async () => {
    // Markdown is the root and has to work, so fetch it before switching off.
    // Ruby is never fetched, so the fenced block below cannot be resolved.
    await loadLanguages(["markdown", "markdown_inline"]);
    configureDownloads(false);

    const document = ["# Title", "", "```ruby", 'puts "hi"', "```", ""].join("\n");
    const html = await highlight(document, htmlInline({ language: "markdown" }));

    // A refusal reaches the addon's injection callback during a native walk,
    // and has to behave there like any other language it cannot load: the block
    // stays plain and the document renders. Propagating it would fail the page.
    expect(html).toContain("Title");
    expect(html).toContain("puts");
  });
});

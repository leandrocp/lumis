/**
 * Hold `unverified-parsers.json` to what npm publishes.
 *
 * The waiver lists the languages whose installed `@lumis-sh/wasm-*` package
 * cannot verify their queries, because it is missing or was built from a
 * revision other than the one `languages.toml` pins. `query-compile.test.ts`
 * skips those languages when it judges only published parsers, so this is what
 * keeps that skip declared.
 *
 * It describes the whole corpus, so it runs once, apart from the query check.
 * That check runs in batches of four languages in CI and against a single
 * parser in a release. When it carried this test, one lagging package failed
 * every batch of every shard before any query was compiled, and blocked the
 * release of every other parser.
 *
 * Not part of `pnpm test`: it describes npm rather than this package. No
 * grammar is loaded, so the whole corpus fits in one process.
 */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { parsers, publishedParser } from "./published-parsers.js";

const unverified = JSON.parse(
  readFileSync(new URL("./unverified-parsers.json", import.meta.url), "utf8"),
) as {
  reason: string;
  languages: string[];
};
const waived = new Set(unverified.languages);

const published = new Map<string, { path: string } | { unavailable: string }>(
  parsers.map(([id, entry]) => [id, publishedParser(id, entry)]),
);

describe("unverified parser waiver", () => {
  const unpublished = parsers.filter(([id]) => "unavailable" in published.get(id)!);

  it("has a parser catalog to check", () => {
    expect(parsers.length).toBeGreaterThan(100);
  });

  it("lists every language whose published package cannot verify it", () => {
    // A new gap must be declared. Otherwise a parser bump silently drops a
    // language out of coverage, which is how the §1 defects escaped review.
    const undeclared = unpublished
      .filter(([id]) => !waived.has(id))
      .map(([id]) => `${id}: ${(published.get(id) as { unavailable: string }).unavailable}`);

    expect(
      undeclared,
      "add these to test/unverified-parsers.json, or publish the parser packages",
    ).toEqual([]);
  });

  it("has no stale entries", () => {
    // The list can only shrink. Once a package publishes at the pinned revision,
    // its waiver must go.
    const stale = parsers
      .filter(([id]) => waived.has(id) && "path" in published.get(id)!)
      .map(([id]) => id);

    expect(stale, "these packages are published at the pinned rev, remove them").toEqual([]);
  });

  it("names only real languages", () => {
    const known = new Set(parsers.map(([id]) => id));
    expect(unverified.languages.filter((id) => !known.has(id))).toEqual([]);
  });
});

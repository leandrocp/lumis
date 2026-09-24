/**
 * The JavaScript half of the shared language-name conformance suite.
 *
 * `fixtures/conformance-languages/cases.json` states what loading and the CLI's
 * download must agree on in every runtime. This asserts the JavaScript
 * implementation against it;
 * `crates/lumis-wasm-runtime/tests/languages_conformance.rs` and
 * `packages/elixir/lumis/test/languages_conformance_test.exs` assert theirs
 * against the same file.
 */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { expandBundles } from "../src/core/language-names.js";

interface Cases {
  spellings: { groups: string[][] };
  passthrough: { names: string[] };
  deduplication: { bundle: string; alsoNamed: string };
  ordering: { input: string[]; firstIs: string; lastIs: string };
  unknownBundles: { names: string[] };
}

const cases = JSON.parse(
  readFileSync(
    new URL("../../../../fixtures/conformance-languages/cases.json", import.meta.url),
    "utf8",
  ),
) as Cases;

describe("language name conformance", () => {
  it("expands every spelling of a bundle the same way", () => {
    expect(cases.spellings.groups.length).toBeGreaterThanOrEqual(5);

    for (const spellings of cases.spellings.groups) {
      const expected = expandBundles([spellings[0]]);
      expect(expected.length, `${spellings[0]} expanded to nothing`).toBeGreaterThan(0);

      for (const spelling of spellings.slice(1)) {
        expect(expandBundles([spelling]), `${spelling} disagrees with ${spellings[0]}`).toEqual(
          expected,
        );
      }
    }
  });

  it("lets a name that is not a bundle survive expansion", () => {
    for (const name of cases.passthrough.names) {
      expect(expandBundles([name])).toEqual([name]);
    }
  });

  it("keeps a bundle member named twice only once", () => {
    const { bundle, alsoNamed } = cases.deduplication;
    const alone = expandBundles([bundle]);
    expect(alone).toContain(alsoNamed);

    const withRepeat = expandBundles([bundle, alsoNamed, alsoNamed]);

    expect(withRepeat.filter((name) => name === alsoNamed)).toHaveLength(1);
    expect(withRepeat).toHaveLength(alone.length);
  });

  it("keeps the order it was given", () => {
    const { input, firstIs, lastIs } = cases.ordering;

    const expanded = expandBundles(input);

    expect(expanded[0]).toBe(firstIs);
    expect(expanded.at(-1)).toBe(lastIs);
  });

  it("rejects an unknown bundle rather than treating it as a language", () => {
    for (const name of cases.unknownBundles.names) {
      expect(() => expandBundles([name]), `${name} should be rejected`).toThrow(
        new RegExp(`Unknown bundle "${name}"`),
      );
    }
  });
});

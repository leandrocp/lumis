/**
 * The TypeScript half of the ANSI helper parity check.
 *
 * `fixtures/ansi-parity.json` holds one expected value per case.
 * `crates/lumis-core/tests/ansi_parity.rs` asserts Rust produces it; this
 * asserts the port does too. Rust is the reference, so a difference here is a
 * bug in this port.
 *
 * The port exists because the browser cannot call into Rust. Nothing pinned it
 * until now, and it had drifted: `hexToRgb` read `'ff79cz'` as `[255, 121, 12]`
 * where Rust read no color at all (#1389).
 */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

import { hexToRgb, paint, styleToAnsi } from "../src/formatter/ansi-core.js";
import type { HighlightStyle } from "../src/types.js";

type Helper = "hexToRgb" | "styleToAnsi" | "paint";

interface Case {
  name: string;
  helper: Helper;
  /** An RGB triple or `null` for `hexToRgb`, a string for the other two. */
  expected: [number, number, number] | null | string;
  hex?: string;
  text?: string;
  /** The JSON shape a theme already carries, so neither side reshapes it. */
  style?: HighlightStyle;
}

const manifest: { cases: Case[] } = JSON.parse(
  readFileSync(new URL("../../../../fixtures/ansi-parity.json", import.meta.url), "utf8"),
);

function expectedRgb(testCase: Case): [number, number, number] | undefined {
  if (typeof testCase.expected === "string") {
    throw new TypeError(`${testCase.name}: expected an RGB triple, not a string`);
  }
  return testCase.expected ?? undefined;
}

function expectedText(testCase: Case): string {
  if (typeof testCase.expected !== "string") {
    throw new TypeError(`${testCase.name}: expected a string, not an RGB triple`);
  }
  return testCase.expected;
}

describe("ansi helper parity", () => {
  it("covers the input a valid prefix used to slip through", () => {
    const names = manifest.cases.map((testCase) => testCase.name);

    for (const required of [
      "hex/trailing-non-hex-digit",
      "hex/last-component-half-valid",
      "hex/trailing-space",
      "hex/leading-space",
      "hex/signed-component",
      "hex/non-ascii-in-the-last-component",
      "hex/repeated-hash-is-trimmed",
      "paint/an-empty-background-still-takes-the-per-line-branch",
    ]) {
      expect(names, `the corpus lost its \`${required}\` case`).toContain(required);
    }
  });

  for (const testCase of manifest.cases) {
    it(`answers what Rust answers for ${testCase.name}`, () => {
      switch (testCase.helper) {
        case "hexToRgb": {
          expect(hexToRgb(testCase.hex ?? ""), testCase.name).toEqual(expectedRgb(testCase));
          break;
        }
        case "styleToAnsi": {
          expect(styleToAnsi(testCase.style), testCase.name).toBe(expectedText(testCase));
          break;
        }
        case "paint": {
          expect(paint(testCase.text ?? "", testCase.style), testCase.name).toBe(
            expectedText(testCase),
          );
          break;
        }
        // `helper` comes from JSON, so a corpus typo would otherwise run
        // nothing and report the same green as a case that checked something.
        default: {
          throw new TypeError(`${testCase.name}: \`${String(testCase.helper)}\` is not a helper`);
        }
      }
    });
  }
});

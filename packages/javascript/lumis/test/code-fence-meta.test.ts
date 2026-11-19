/**
 * The TypeScript half of the code fence meta parity check.
 *
 * `fixtures/code-fence-meta.json` holds one expected result per case.
 * `crates/lumis-core/src/code_fence.rs` asserts Rust produces it; this asserts
 * the port does too, from both published roots. Rust is the reference, so a
 * difference here is a bug in this port.
 */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

import * as browserEntry from "../src/index.browser.js";
import * as nodeEntry from "../src/index.js";
import type { CodeFenceMeta } from "../src/index.js";

interface Case {
  name: string;
  meta: string;
  expected: Partial<CodeFenceMeta>;
}

const manifest: { cases: Case[] } = JSON.parse(
  readFileSync(new URL("../../../../fixtures/code-fence-meta.json", import.meta.url), "utf8"),
);

describe("parseCodeFenceMeta", () => {
  it("loads the shared corpus", () => {
    expect(manifest.cases.length).toBeGreaterThan(40);
  });

  for (const [entry, { parseCodeFenceMeta }] of [
    ["node", nodeEntry],
    ["browser", browserEntry],
  ] as const) {
    it.each(manifest.cases)(`${entry}: $name`, ({ meta, expected }) => {
      expect(parseCodeFenceMeta(meta)).toEqual({ unrecognized: [], ...expected });
    });
  }
});

/**
 * The TypeScript half of the annotation composition parity check.
 *
 * `fixtures/annotation-composition.json` holds one expected event stream per
 * case. `crates/lumis-core/tests/annotation_composition.rs` asserts Rust
 * produces it; this asserts the port does too. Rust is the reference, so a
 * difference here is a bug in this port.
 */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

import { composeAnnotations } from "../src/annotations.js";
import { buildSourceIndex } from "../src/events.js";
import type { Annotation, SyntaxHighlightEvent } from "../src/types.js";

interface Case {
  name: string;
  source: string;
  events: SyntaxHighlightEvent[];
  annotations: Annotation<string>[];
  expected: string;
}

const manifest: { cases: Case[] } = JSON.parse(
  readFileSync(
    new URL("../../../../fixtures/annotation-composition.json", import.meta.url),
    "utf8",
  ),
);

function render(testCase: Case): string {
  try {
    return composeAnnotations(
      testCase.events,
      testCase.annotations,
      buildSourceIndex(testCase.source),
    )
      .map((event) => {
        switch (event.type) {
          case "start":
            return `S:${event.scope}`;
          case "source":
            return `T:${event.startByte}-${event.endByte}`;
          case "end":
            return "E";
          case "annotationEnd":
            return "A-";
          default:
            return `A+${event.annotation.data}@${event.annotation.range.start}-${event.annotation.range.end}`;
        }
      })
      .join(" ");
  } catch (error) {
    return `ERROR: ${(error as Error).message}`;
  }
}

describe("annotation composition parity", () => {
  it("covers the shapes composition has to get right", () => {
    // A discovery bug that found nothing would otherwise pass silently.
    expect(manifest.cases.length).toBeGreaterThanOrEqual(24);

    const names = manifest.cases.map((c) => c.name);
    for (const required of [
      "scope/annotation-inside-scope",
      "multi/overlapping",
      "multi/nested",
      "position/across-lines",
      "utf8/mid-character-offset",
    ]) {
      expect(names, `the corpus lost its \`${required}\` case`).toContain(required);
    }
  });

  it("produces the same stream as Rust", () => {
    for (const testCase of manifest.cases) {
      expect(render(testCase), `${testCase.name}: diverged from Rust`).toBe(testCase.expected);
    }
  });
});

/**
 * The TypeScript half of the line-decoration composition parity check.
 *
 * `fixtures/decoration-composition.json` holds one expected event stream per
 * case. `crates/lumis-core/tests/decoration_composition.rs` asserts Rust
 * produces it; this asserts the port does too. Rust is the reference, so a
 * difference here is a bug in this port.
 *
 * Each case runs the pipeline a formatter sees: caller annotations composed
 * first, then the line decorations over the top.
 */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

import { composeAnnotations } from "../src/annotations.js";
import { LineSelection, composeLineDecorations } from "../src/decorations.js";
import { buildSourceIndex } from "../src/events.js";
import type { Annotation, HighlightEvent, LineSpec, SyntaxHighlightEvent } from "../src/types.js";

interface Case {
  name: string;
  source: string;
  events: SyntaxHighlightEvent[];
  annotations?: Array<{ start: number; end: number; data: string }>;
  highlightLines: LineSpec[];
  expected: string;
}

const manifest: { cases: Case[] } = JSON.parse(
  readFileSync(
    new URL("../../../../fixtures/decoration-composition.json", import.meta.url),
    "utf8",
  ),
);

function compose(testCase: Case): HighlightEvent<string>[] {
  const annotations: Annotation<string>[] = (testCase.annotations ?? []).map((annotation) => ({
    range: { type: "offset", start: annotation.start, end: annotation.end },
    data: annotation.data,
  }));
  const composed = composeAnnotations(
    testCase.events,
    annotations,
    buildSourceIndex(testCase.source),
  );

  return composeLineDecorations(
    new TextEncoder().encode(testCase.source),
    composed,
    new LineSelection(testCase.highlightLines),
  );
}

function notation(event: HighlightEvent<string>): string {
  switch (event.type) {
    case "start":
      return `S:${event.scope}`;
    case "source":
      return `T:${event.start}-${event.end}`;
    case "end":
      return "E";
    case "annotationStart":
      return `A+${event.annotation.data}@${event.annotation.range.start}-${event.annotation.range.end}`;
    case "annotationEnd":
      return "A-";
    case "decorationEnd":
      return "L-";
    default:
      return `L+${event.decoration.number}${event.decoration.highlighted ? "*" : ""}`;
  }
}

describe("line decoration composition parity", () => {
  it("covers the shapes composition has to get right", () => {
    // A discovery bug that found nothing would otherwise pass silently.
    expect(manifest.cases.length).toBeGreaterThanOrEqual(20);

    const names = manifest.cases.map((testCase) => testCase.name);
    for (const required of [
      "empty/no-events",
      "lines/trailing-newline-opens-one-more",
      "lines/blank-line-in-the-middle",
      "scope/closed-and-reopened-across-a-newline",
      "scope/unbalanced-start-closes-before-the-last-line-ends",
      "annotation/closed-and-reopened-across-a-newline",
      "utf8/multibyte-lines",
      "highlight/overlapping-ranges-merge",
      "highlight/range-beyond-the-document",
      "highlight/blank-line",
    ]) {
      expect(names, `the corpus lost its \`${required}\` case`).toContain(required);
    }
  });

  it("produces the same stream as Rust", () => {
    for (const testCase of manifest.cases) {
      expect(
        compose(testCase)
          .map((event) => notation(event))
          .join(" "),
        `${testCase.name}: diverged from Rust`,
      ).toBe(testCase.expected);
    }
  });

  it("preserves the source", () => {
    const decoder = new TextDecoder();

    for (const testCase of manifest.cases) {
      const bytes = new TextEncoder().encode(testCase.source);
      const covered = testCase.events
        .filter((event) => event.type === "source")
        .map((event) => decoder.decode(bytes.subarray(event.start, event.end)))
        .join("");
      const rendered = compose(testCase)
        .filter((event) => event.type === "source")
        .map((event) => decoder.decode(bytes.subarray(event.start, event.end)))
        .join("");

      expect(rendered, `${testCase.name}: source changed`).toBe(covered);
    }
  });
});

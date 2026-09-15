import { beforeAll, describe, expect, it } from "vitest";

import javascript from "../langs/javascript.ts";
import { createHighlighter, type Annotation } from "../src/index.js";
import type { Formatter } from "../src/formatters.js";
import { renderExample } from "../examples/annotations.ts";
import { configureLocalWasmResolver } from "./wasm.js";

interface Change {
  id: number;
}

describe("annotations", () => {
  beforeAll(() => {
    configureLocalWasmResolver(["javascript"]);
  }, 120_000);

  it("flows typed annotations through the public highlight options", async () => {
    const source = "const total = (price + tax)";
    const changedText = "price + tax";
    const start = new TextEncoder().encode(source.slice(0, source.indexOf(changedText))).length;
    const annotations: Annotation<Change>[] = [
      {
        range: { type: "offset", start, end: start + new TextEncoder().encode(changedText).length },
        data: { id: 7 },
      },
    ];
    const formatter: Formatter<Change> = {
      language: javascript,
      render(rendered, events) {
        const bytes = new TextEncoder().encode(rendered);
        const decoder = new TextDecoder();
        const output: string[] = [];

        for (const event of events) {
          if (event.type === "start") output.push(`<syntax:${event.scope}>`);
          if (event.type === "end") output.push("</syntax>");
          if (event.type === "annotationStart") {
            output.push(`<annotation:${event.annotation.data.id}>`);
          }
          if (event.type === "annotationEnd") output.push("</annotation>");
          if (event.type === "source") {
            output.push(decoder.decode(bytes.subarray(event.start, event.end)));
          }
        }

        return output.join("");
      },
    };
    const highlighter = await createHighlighter({ languages: [javascript] });

    const output = highlighter.highlight(source, formatter, {
      annotations,
      rainbowBrackets: true,
    });

    expect(output).toContain("<syntax:");
    expect(output).toContain("<syntax:punctuation.bracket.rainbow.");
    expect(output).toContain("<annotation:7>");
    expect(output).toContain("</annotation>");
    expect(output.replaceAll(/<[^>]+>/g, "")).toBe(source);
  }, 30_000);

  it("rejects annotation offsets that are not UTF-8 boundaries", async () => {
    const source = "const café = 1";
    const annotations: Annotation[] = [
      {
        range: { type: "offset", start: 10, end: 11 },
        data: {},
      },
    ];
    const formatter: Formatter = {
      language: javascript,
      render(_source, events) {
        return JSON.stringify(events);
      },
    };
    const highlighter = await createHighlighter({ languages: [javascript] });

    expect(() => highlighter.highlight(source, formatter, { annotations })).toThrow(
      "is not a UTF-8 character boundary",
    );
  }, 30_000);

  it("resolves position ranges to UTF-8 byte ranges before formatting", async () => {
    const source = "const π = 3\nconst café = 4";
    let resolvedRange: { start: number; end: number } | undefined;
    const annotations: Annotation<Change>[] = [
      {
        range: {
          type: "position",
          start: { line: 1, column: 6 },
          end: { line: 1, column: 11 },
        },
        data: { id: 8 },
      },
    ];
    const formatter: Formatter<Change> = {
      language: javascript,
      render(rendered, events) {
        const sourceBytes = new TextEncoder().encode(rendered);
        const decoder = new TextDecoder();

        return events
          .map((event) => {
            if (event.type === "annotationStart") {
              resolvedRange = event.annotation.range;
            }
            if (event.type === "source") {
              return decoder.decode(sourceBytes.subarray(event.start, event.end));
            }
            return "";
          })
          .join("");
      },
    };
    const highlighter = await createHighlighter({ languages: [javascript] });

    const output = highlighter.highlight(source, formatter, { annotations });
    const start = new TextEncoder().encode(source.slice(0, source.indexOf("café"))).length;

    expect(output).toBe(source);
    expect(resolvedRange).toEqual({ start, end: start + new TextEncoder().encode("café").length });
  }, 30_000);

  it("rejects position columns that are not UTF-8 boundaries", async () => {
    const source = "π\ncafé";
    const annotations: Annotation[] = [
      {
        range: {
          type: "position",
          start: { line: 1, column: 0 },
          end: { line: 1, column: 4 },
        },
        data: {},
      },
    ];
    const formatter: Formatter = {
      language: javascript,
      render(_source, events) {
        return JSON.stringify(events);
      },
    };
    const highlighter = await createHighlighter({ languages: [javascript] });

    expect(() => highlighter.highlight(source, formatter, { annotations })).toThrow(
      "is not a UTF-8 character boundary",
    );
  }, 30_000);

  it("marks a blank line with an empty annotation", async () => {
    // The line-based review case: a blank line has nothing to cover, but it is
    // still somewhere a comment can land.
    const source = "const a = 1;\n\nconst b = 2;";
    const blankLineOffset = source.indexOf("\n") + 1;
    const annotations: Annotation<Change>[] = [
      {
        range: { type: "offset", start: blankLineOffset, end: blankLineOffset },
        data: { id: 1 },
      },
    ];
    const formatter: Formatter<Change> = {
      language: javascript,
      render(rendered, events) {
        const bytes = new TextEncoder().encode(rendered);
        const decoder = new TextDecoder();
        const parts: string[] = [];
        for (const event of events) {
          if (event.type === "annotationStart") parts.push(`<mark:${event.annotation.data.id}>`);
          else if (event.type === "annotationEnd") parts.push("</mark>");
          else if (event.type === "source") {
            parts.push(decoder.decode(bytes.subarray(event.start, event.end)));
          }
        }
        return parts.join("");
      },
    };
    const highlighter = await createHighlighter({ languages: [javascript] });

    expect(highlighter.highlight(source, formatter, { annotations })).toBe(
      "const a = 1;\n<mark:1></mark>\nconst b = 2;",
    );
  }, 30_000);

  it("does not let a point annotation leak into the rest of the document", async () => {
    const source = "const a = 1;";
    const annotations: Annotation<Change>[] = [
      { range: { type: "offset", start: 5, end: 5 }, data: { id: 2 } },
    ];
    const formatter: Formatter<Change> = {
      language: javascript,
      render(_rendered, events) {
        const starts = events.filter((event) => event.type === "annotationStart").length;
        const ends = events.filter((event) => event.type === "annotationEnd").length;
        return `${starts}:${ends}`;
      },
    };
    const highlighter = await createHighlighter({ languages: [javascript] });

    expect(highlighter.highlight(source, formatter, { annotations })).toBe("1:1");
  }, 30_000);

  it("rejects a range that runs backwards", async () => {
    const source = "const a = 1;";
    const annotations: Annotation<Change>[] = [
      { range: { type: "offset", start: 8, end: 5 }, data: { id: 3 } },
    ];
    const formatter: Formatter<Change> = {
      language: javascript,
      render: (_rendered, events) => String(events.length),
    };
    const highlighter = await createHighlighter({ languages: [javascript] });

    expect(() => highlighter.highlight(source, formatter, { annotations })).toThrow(
      "must not be after its end",
    );
  }, 30_000);

  it("renders every annotation shape in the example", async () => {
    const output = await renderExample();

    // A position range crossing a line boundary.
    expect(output).toContain('<span class="line-changed">');
    // An offset range starting mid-token splits the `variable` scope.
    expect(output).toContain(
      '<span class="l-variable">p</span><mark class="edit"><span class="l-variable">rice</span>',
    );
    // Byte offsets over a multibyte literal.
    expect(output).toContain("&quot;☕ café&quot;");
    // A point renders as an empty element.
    expect(output).toContain('<i data-note="why the gap?"></i>');
    // The overlapping annotation is closed and reopened, so it starts twice.
    expect(output.match(/<mark class="right">/g)).toHaveLength(2);
  }, 30_000);
});

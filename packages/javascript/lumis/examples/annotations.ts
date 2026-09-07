/**
 * Every shape an annotation can take, over one small source.
 *
 * Lumis does not compute the ranges. A diff library, a search index or a
 * compiler supplies them; Lumis places them correctly relative to the syntax
 * and hands them to this formatter.
 */

import { pathToFileURL } from "node:url";

import javascript from "../langs/javascript.ts";
import {
  createHighlighter,
  type Annotation,
  type AnnotationRange,
  type HighlightEvent,
} from "../src/index.ts";
import type { Formatter } from "../src/formatters.ts";
import { escape, scopeToClass } from "../src/formatter/html.ts";

const SOURCE = 'let total = price + tax;\nlet label = "☕ café";\n\nlet net = total - fee;';

const encoder = new TextEncoder();
const decoder = new TextDecoder();

/** What each annotation means. Lumis never inspects this. */
type Mark =
  | { type: "line"; kind: string }
  | { type: "span"; name: string }
  | { type: "note"; label: string };

const formatter: Formatter<Mark> = {
  language: javascript,
  render(source: string, events: readonly HighlightEvent<Mark>[]): string {
    const bytes = encoder.encode(source);
    // `annotationEnd` carries no payload, so keep a stack of what was opened.
    const open: string[] = [];
    const out: string[] = [];

    for (const event of events) {
      if (event.type === "start") {
        out.push(`<span class="${scopeToClass(event.scope)}">`);
      } else if (event.type === "end") {
        out.push("</span>");
      } else if (event.type === "annotationStart") {
        const mark = event.annotation.data;
        if (mark.type === "line") {
          out.push(`<span class="line-${mark.kind}">`);
          open.push("span");
        } else if (mark.type === "span") {
          out.push(`<mark class="${mark.name}">`);
          open.push("mark");
        } else {
          // A point opens and closes with nothing between it.
          out.push(`<i data-note="${escape(mark.label)}">`);
          open.push("i");
        }
      } else if (event.type === "annotationEnd") {
        out.push(`</${open.pop()}>`);
      } else {
        out.push(escape(decoder.decode(bytes.subarray(event.startByte, event.endByte))));
      }
    }

    return out.join("");
  },
};

/** Byte range of the first occurrence of `text`. A diff or search library would
 *  report these; nothing here is hand-counted. */
function find(text: string): AnnotationRange {
  const start = encoder.encode(SOURCE.slice(0, SOURCE.indexOf(text))).length;
  return { type: "offset", start, end: start + encoder.encode(text).length };
}

/** Lines `first` through `last`, as zero-based lines and UTF-8 byte columns. */
function lines(first: number, last: number): AnnotationRange {
  const width = encoder.encode(SOURCE.split("\n")[last] ?? "").length;
  return {
    type: "position",
    start: { line: first, column: 0 },
    end: { line: last, column: width },
  };
}

const annotations: Annotation<Mark>[] = [
  // A position range, and one that crosses a line boundary. Zero-based line,
  // UTF-8 byte column, which is what a diff reports.
  { range: lines(0, 1), data: { type: "line", kind: "changed" } },
  // An offset range starting mid-token. Lumis closes and reopens the
  // `variable` scope around it, so `price` renders as `p` + `rice`.
  { range: find("rice"), data: { type: "span", name: "edit" } },
  // Offsets are UTF-8 bytes, not characters. `☕` is 3 and `é` is 2, and `find`
  // counts them, so the whole literal is covered.
  { range: find('"☕ café"'), data: { type: "span", name: "text" } },
  // An empty range is a point. Line 2 is blank, so there is nothing to cover,
  // and a review comment still has somewhere to land.
  {
    range: { type: "position", start: { line: 2, column: 0 }, end: { line: 2, column: 0 } },
    data: { type: "note", label: "why the gap?" },
  },
  { range: lines(3, 3), data: { type: "line", kind: "added" } },
  // Two that overlap without either containing the other. Lumis closes `left`
  // and reopens `right` after it, so `right` opens twice.
  { range: find("total - "), data: { type: "span", name: "left" } },
  { range: find("- fee"), data: { type: "span", name: "right" } },
];

export async function renderExample(): Promise<string> {
  const highlighter = await createHighlighter({ languages: [javascript] });
  return highlighter.highlight(SOURCE, formatter, { annotations });
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  console.log(await renderExample());
}

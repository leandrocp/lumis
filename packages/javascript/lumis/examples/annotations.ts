/**
 * Every shape an annotation can take, over one small source.
 *
 * Lumis does not compute the ranges. A diff library, a search index or a
 * compiler supplies them; Lumis places them correctly relative to the syntax
 * and hands them to this formatter.
 */

import { pathToFileURL } from "node:url";

import javascript from "../langs/javascript.ts";
import { createHighlighter, type Annotation, type HighlightEvent } from "../src/index.ts";
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

const annotations: Annotation<Mark>[] = [
  // Zero-based line and UTF-8 byte column. This one crosses a line.
  {
    range: { type: "position", start: { line: 0, column: 0 }, end: { line: 1, column: 24 } },
    data: { type: "line", kind: "changed" },
  },
  // `rice`, inside `price`. Starting mid-token makes Lumis close and reopen
  // the `variable` scope, so it renders as `p` + `rice`.
  { range: { type: "offset", start: 13, end: 17 }, data: { type: "span", name: "edit" } },
  // `"☕ café"`. Offsets are UTF-8 bytes, so `☕` costs 3 and `é` costs 2.
  { range: { type: "offset", start: 37, end: 48 }, data: { type: "span", name: "text" } },
  // An empty range is a point. Line 2 is blank, so there is nothing to cover,
  // and a review comment still has somewhere to land.
  {
    range: { type: "position", start: { line: 2, column: 0 }, end: { line: 2, column: 0 } },
    data: { type: "note", label: "why the gap?" },
  },
  {
    range: { type: "position", start: { line: 3, column: 0 }, end: { line: 3, column: 22 } },
    data: { type: "line", kind: "added" },
  },
  // `total - ` and `- fee` overlap without either containing the other, so
  // Lumis closes `left` and reopens `right` after it.
  { range: { type: "offset", start: 61, end: 69 }, data: { type: "span", name: "left" } },
  { range: { type: "offset", start: 67, end: 72 }, data: { type: "span", name: "right" } },
];

export async function renderExample(): Promise<string> {
  const highlighter = await createHighlighter({ languages: [javascript] });
  return highlighter.highlight(SOURCE, formatter, { annotations });
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  console.log(await renderExample());
}

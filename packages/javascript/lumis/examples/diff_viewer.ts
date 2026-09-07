/**
 * A side-by-side diff viewer built with Lumis annotations.
 *
 * The example intentionally does not compute a diff. An upstream diff library
 * would supply the line states, changed spans, and annotation ranges below.
 */

import { pathToFileURL } from "node:url";

import javascript from "../langs/javascript.ts";
import {
  createHighlighter,
  type OffsetAnnotationRange,
  type Annotation,
  type HighlightEvent,
  type LanguageRef,
} from "../src/index.ts";
import type { Formatter } from "../src/formatters.ts";
import {
  closingTags,
  decodeSourceSlice,
  encodeSource,
  escape,
  openCodeTag,
  openPreTag,
  openSpanTag,
  scopeToClass,
} from "../src/formatter/html.ts";

type LineKind = "context" | "added" | "removed" | "changed";
type SpanKind = "added" | "removed";

type DiffAnnotation =
  | { type: "line"; number: number; kind: LineKind }
  | { type: "span"; kind: SpanKind }
  | { type: "annotation"; label: string };

const lineMarkers: Record<LineKind, string> = {
  context: " ",
  added: "+",
  removed: "-",
  changed: "~",
};

class DiffHtmlFormatter implements Formatter<DiffAnnotation> {
  constructor(readonly language: LanguageRef) {}

  render(source: string, events: readonly HighlightEvent<DiffAnnotation>[]): string {
    const sourceBytes = encodeSource(source);
    const output = [openPreTag({ preClass: "diff-code" }), openCodeTag(this.language)];
    const annotationClosings: string[] = [];

    for (const event of events) {
      this.renderEvent(event, sourceBytes, output, annotationClosings);
    }

    if (annotationClosings.length > 0) {
      throw new Error("annotation event stream contains an unclosed annotation");
    }

    output.push(closingTags());
    return output.join("");
  }

  private renderEvent(
    event: HighlightEvent<DiffAnnotation>,
    sourceBytes: Uint8Array,
    output: string[],
    annotationClosings: string[],
  ): void {
    if (event.type === "start") {
      output.push(openSpanTag({ class: scopeToClass(event.scope) }));
      return;
    }
    if (event.type === "end") {
      output.push("</span>");
      return;
    }
    if (event.type === "source") {
      output.push(escape(decodeSourceSlice(sourceBytes, event.startByte, event.endByte)));
      return;
    }
    if (event.type === "annotationEnd") {
      const closing = annotationClosings.pop();
      if (!closing) {
        throw new Error("annotation event stream contains an unmatched end event");
      }
      output.push(closing);
      return;
    }

    const { data } = event.annotation;
    if (data.type === "line") {
      output.push(
        openSpanTag({
          class: `diff-line diff-line-${data.kind}`,
          "data-line": data.number,
          "data-marker": lineMarkers[data.kind],
        }),
      );
      annotationClosings.push("</span>");
      return;
    }
    if (data.type === "span") {
      output.push(`<mark class="diff-span diff-span-${data.kind}">`);
      annotationClosings.push("</mark>");
      return;
    }

    output.push(
      openSpanTag({
        class: "diff-annotation",
        "data-label": data.label,
      }),
    );
    annotationClosings.push("</span>");
  }
}

function rangeOf(source: string, text: string): OffsetAnnotationRange {
  const index = source.indexOf(text);
  if (index === -1) throw new Error(`expected ${JSON.stringify(text)} in the example source`);
  return offsetRange(source, text, index);
}

function lastRangeOf(source: string, text: string): OffsetAnnotationRange {
  const index = source.lastIndexOf(text);
  if (index === -1) throw new Error(`expected ${JSON.stringify(text)} in the example source`);
  return offsetRange(source, text, index);
}

function offsetRange(source: string, text: string, index: number): OffsetAnnotationRange {
  const encoder = new TextEncoder();
  const start = encoder.encode(source.slice(0, index)).length;
  return { type: "offset", start, end: start + encoder.encode(text).length };
}

export async function renderExample(): Promise<string> {
  const highlighter = await createHighlighter({ languages: [javascript] });
  const formatter = new DiffHtmlFormatter(javascript);

  const oldSource = `function calculate(price, tax) {
  return price + tax
}
`;
  const newSource = `function calculate(price, tax, fee) {
  const subtotal = price + tax
  return subtotal + fee
}
`;

  // A diff library would determine these ranges. Lumis only consumes them.
  const oldAnnotations = [
    {
      range: rangeOf(oldSource, "function calculate(price, tax) {"),
      data: { type: "line", number: 1, kind: "changed" },
    },
    {
      range: rangeOf(oldSource, "  return price + tax"),
      data: { type: "line", number: 2, kind: "removed" },
    },
    {
      range: lastRangeOf(oldSource, "}"),
      data: { type: "line", number: 3, kind: "context" },
    },
    {
      range: rangeOf(oldSource, "price, tax"),
      data: { type: "span", kind: "removed" },
    },
    {
      range: rangeOf(oldSource, "price + tax"),
      data: { type: "span", kind: "removed" },
    },
  ] satisfies Annotation<DiffAnnotation>[];

  const newAnnotations = [
    {
      range: rangeOf(newSource, "function calculate(price, tax, fee) {"),
      data: { type: "line", number: 1, kind: "changed" },
    },
    {
      range: rangeOf(newSource, "  const subtotal = price + tax"),
      data: { type: "line", number: 2, kind: "added" },
    },
    {
      range: rangeOf(newSource, "  return subtotal + fee"),
      data: { type: "line", number: 3, kind: "added" },
    },
    {
      range: lastRangeOf(newSource, "}"),
      data: { type: "line", number: 4, kind: "context" },
    },
    {
      range: rangeOf(newSource, "price, tax, fee"),
      data: { type: "span", kind: "added" },
    },
    {
      range: rangeOf(newSource, "  const subtotal = price + tax"),
      data: { type: "span", kind: "added" },
    },
    {
      range: rangeOf(newSource, "  return subtotal + fee"),
      data: { type: "span", kind: "added" },
    },
    {
      range: lastRangeOf(newSource, "fee"),
      data: { type: "annotation", label: "New service fee" },
    },
  ] satisfies Annotation<DiffAnnotation>[];

  const oldHtml = highlighter.highlight(oldSource, formatter, {
    annotations: oldAnnotations,
  });
  const newHtml = highlighter.highlight(newSource, formatter, {
    annotations: newAnnotations,
  });

  return `${pageStart}${oldHtml}${pageMiddle}${newHtml}${pageEnd}`;
}

const pageStart = `<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Lumis JavaScript annotation diff viewer</title>
<style>
:root { color-scheme: dark; font-family: ui-sans-serif, system-ui, sans-serif; }
body { margin: 0; background: #0d1117; color: #e6edf3; }
main { padding: 2rem; }
h1 { margin: 0 0 .35rem; font-size: 1.35rem; }
.subtitle { margin: 0 0 1.25rem; color: #8b949e; }
.diff-viewer { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); border: 1px solid #30363d; border-radius: 10px; overflow: hidden; }
.diff-pane + .diff-pane { border-left: 1px solid #30363d; }
.diff-pane h2 { margin: 0; padding: .7rem 1rem; background: #161b22; border-bottom: 1px solid #30363d; font-size: .85rem; font-weight: 600; }
.diff-code { margin: 0; padding: 0; overflow-x: auto; background: #0d1117; }
.diff-code code { display: block; min-width: max-content; padding: .5rem 0; }
.diff-line { display: inline-block; min-width: 100%; min-height: 1.45em; }
.diff-line::before { display: inline-block; width: 4.5rem; margin-right: .75rem; padding-right: .7rem; color: #6e7681; text-align: right; content: attr(data-line) " " attr(data-marker); user-select: none; }
.diff-line-added { background: rgba(46, 160, 67, .18); }
.diff-line-removed { background: rgba(248, 81, 73, .18); }
.diff-line-changed { background: rgba(187, 128, 9, .18); }
.diff-line-added::before { color: #56d364; background: rgba(46, 160, 67, .16); }
.diff-line-removed::before { color: #ff7b72; background: rgba(248, 81, 73, .16); }
.diff-line-changed::before { color: #e3b341; background: rgba(187, 128, 9, .16); }
.diff-span { border-radius: 3px; color: inherit; }
.diff-span-added { background: rgba(46, 160, 67, .42); }
.diff-span-removed { background: rgba(248, 81, 73, .42); }
.diff-annotation { position: relative; border-bottom: 2px dotted #d2a8ff; }
.diff-annotation::after { position: absolute; z-index: 1; left: 0; bottom: 1.5rem; width: max-content; max-width: 12rem; padding: .3rem .45rem; border: 1px solid #8957e5; border-radius: 5px; background: #2d1b4e; color: #d2a8ff; font: 11px/1.2 ui-sans-serif, system-ui, sans-serif; content: "● " attr(data-label); }
.l-keyword, .l-keyword-function { color: #ff7b72; }
.l-function { color: #d2a8ff; }
.l-variable, .l-variable-parameter { color: #ffa657; }
.l-type, .l-type-builtin { color: #79c0ff; }
.l-operator, .l-punctuation-bracket, .l-punctuation-delimiter { color: #8b949e; }
@media (max-width: 850px) {
  main { padding: 1rem; }
  .diff-viewer { grid-template-columns: 1fr; }
  .diff-pane + .diff-pane { border-left: 0; border-top: 1px solid #30363d; }
}
</style>
</head>
<body>
<main>
<h1>Annotation API: JavaScript diff viewer</h1>
<p class="subtitle">Caller-supplied ranges composed with Lumis syntax events</p>
<div class="diff-viewer">
<section class="diff-pane">
<h2>calculator.js · before</h2>
`;

const pageMiddle = `
</section>
<section class="diff-pane">
<h2>calculator.js · after</h2>
`;

const pageEnd = `
</section>
</div>
</main>
</body>
</html>
`;

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  process.stdout.write(await renderExample());
}

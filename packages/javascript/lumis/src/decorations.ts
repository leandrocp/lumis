import type { Decoration, HighlightEvent, LineSpec } from "./types.js";

/**
 * The lines a formatter was asked to highlight, resolved once.
 *
 * Ranges are clamped, sorted and merged, so testing lines in ascending order
 * walks them with a cursor rather than rescanning. Nothing here depends on how
 * many lines the document has, so a selection covering a billion lines costs
 * what one covering ten costs.
 *
 * The port of `LineSelection` in `crates/lumis-core/src/decorations.rs`.
 */
export class LineSelection {
  /** Merged, disjoint, ascending, 1-based and inclusive at both ends. */
  private readonly spans: Array<[number, number]>;
  private index = 0;

  constructor(lines: readonly LineSpec[] | undefined) {
    const intervals = (lines ?? [])
      .map((line) => lineInterval(line))
      .filter((interval): interval is [number, number] => interval !== undefined)
      .sort((left, right) => left[0] - right[0] || left[1] - right[1]);

    this.spans = [];
    for (const [start, end] of intervals) {
      const last = this.spans.at(-1);
      if (last && start <= last[1] + 1) {
        last[1] = Math.max(last[1], end);
      } else {
        this.spans.push([start, end]);
      }
    }
  }

  get isEmpty(): boolean {
    return this.spans.length === 0;
  }

  /**
   * Whether `line` is selected.
   *
   * `line` must not go backwards between calls, which is how the composer
   * visits lines.
   */
  contains(line: number): boolean {
    while (this.index < this.spans.length && this.spans[this.index]![1] < line) {
      this.index += 1;
    }

    const span = this.spans[this.index];
    return span !== undefined && span[0] <= line && line <= span[1];
  }
}

function lineInterval(line: LineSpec): [number, number] | undefined {
  if (typeof line === "number") {
    return Number.isInteger(line) && line >= 1 ? [line, line] : undefined;
  }

  // Line 0 does not exist, and a reversed range covers nothing.
  const start = Math.max(1, Math.ceil(line[0]));
  const end = Math.floor(line[1]);
  return start <= end ? [start, end] : undefined;
}

/** One layer held open across a line boundary. */
type OpenLayer =
  | { type: "syntax"; event: HighlightEvent }
  | { type: "annotation"; event: HighlightEvent };

/**
 * Compose one line decoration per rendered line into `events`.
 *
 * Lines are the outermost layer, so every syntax scope and caller annotation
 * still open at a newline is closed before the line ends and reopened on the
 * next one. That is why a formatter can write the stream straight out instead of
 * assembling lines and wrapping them afterwards.
 *
 * The returned stream always holds at least one line: an empty document is one
 * empty line, the same line a caller sees numbered `1`.
 *
 * The port of `compose_line_decorations` in
 * `crates/lumis-core/src/decorations.rs`; `test/decoration-composition.test.ts`
 * holds the two to the same answers.
 */
export function composeLineDecorations<T>(
  sourceBytes: Uint8Array,
  events: readonly HighlightEvent<T>[],
  selection: LineSelection,
): HighlightEvent<T>[] {
  const output: HighlightEvent<T>[] = [];
  const layers: OpenLayer[] = [];
  let line = 1;

  output.push(lineStart(line, selection));

  for (const event of events) {
    line = applyEvent(output, sourceBytes, event, layers, line, selection);
  }

  // An unbalanced input stream would otherwise leave a scope open past the last
  // line, which no formatter can close.
  closeLayers(output, layers);
  output.push({ type: "decorationEnd" });

  return output;
}

/** Copy one event through, and return the line number it ends on. */
function applyEvent<T>(
  output: HighlightEvent<T>[],
  sourceBytes: Uint8Array,
  event: HighlightEvent<T>,
  layers: OpenLayer[],
  line: number,
  selection: LineSelection,
): number {
  switch (event.type) {
    case "start":
      output.push(event);
      layers.push({ type: "syntax", event });
      return line;
    case "end":
      return closeLayer(output, layers, "syntax", event, line);
    case "annotationStart":
      output.push(event);
      layers.push({ type: "annotation", event });
      return line;
    case "annotationEnd":
      return closeLayer(output, layers, "annotation", event, line);
    case "source":
      return splitSource(output, sourceBytes, event, layers, line, selection);
    // A stream that already carries lines is re-composed, not nested.
    default:
      return line;
  }
}

/** Close the innermost layer, when it is the kind `event` closes. */
function closeLayer<T>(
  output: HighlightEvent<T>[],
  layers: OpenLayer[],
  kind: OpenLayer["type"],
  event: HighlightEvent<T>,
  line: number,
): number {
  if (layers.at(-1)?.type === kind) {
    layers.pop();
    output.push(event);
  }

  return line;
}

function lineStart<T>(line: number, selection: LineSelection): HighlightEvent<T> {
  const decoration: Decoration = {
    type: "line",
    number: line,
    highlighted: selection.contains(line),
  };
  return { type: "decorationStart", decoration };
}

function closeLayers<T>(output: HighlightEvent<T>[], layers: readonly OpenLayer[]): void {
  for (let index = layers.length - 1; index >= 0; index -= 1) {
    output.push(layers[index]!.type === "syntax" ? { type: "end" } : { type: "annotationEnd" });
  }
}

function reopenLayers<T>(output: HighlightEvent<T>[], layers: readonly OpenLayer[]): void {
  for (const layer of layers) {
    output.push(layer.event as HighlightEvent<T>);
  }
}

const NEWLINE = 0x0a;

/**
 * Emit a source event, ending a line at every newline it contains.
 *
 * The newline stays inside the line it ends, so concatenating the source events
 * of the composed stream still reproduces the source exactly.
 */
function splitSource<T>(
  output: HighlightEvent<T>[],
  sourceBytes: Uint8Array,
  event: { start: number; end: number },
  layers: readonly OpenLayer[],
  line: number,
  selection: LineSelection,
): number {
  let cursor = Math.min(event.start, sourceBytes.length);
  const end = Math.max(Math.min(event.end, sourceBytes.length), cursor);
  let current = line;

  while (cursor < end) {
    const newline = sourceBytes.indexOf(NEWLINE, cursor);
    if (newline === -1 || newline >= end) break;

    output.push({ type: "source", start: cursor, end: newline + 1 });
    closeLayers(output, layers);
    output.push({ type: "decorationEnd" });

    current += 1;
    output.push(lineStart(current, selection));
    reopenLayers(output, layers);

    cursor = newline + 1;
  }

  if (cursor < end) {
    output.push({ type: "source", start: cursor, end });
  }

  return current;
}

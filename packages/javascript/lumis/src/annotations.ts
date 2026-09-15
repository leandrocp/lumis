import type { SourceIndex } from "./events.js";
import type {
  Annotation,
  AnnotationRange,
  HighlightEvent,
  HighlightRange,
  Position,
  ResolvedAnnotation,
  SyntaxHighlightEvent,
} from "./types.js";

interface Boundary {
  starts: number[];
  ends: number[];
  /** Empty annotations sitting exactly here. They never join the layer stack —
   *  an interval that opens and closes at one offset would be entered and never
   *  left — so they are emitted as a start/end pair. */
  points: number[];
}

interface SyntaxLayer {
  type: "syntax";
  id: number;
  scope: string;
  language: string;
}

interface AnnotationLayer<T> {
  type: "annotation";
  index: number;
  annotation: ResolvedAnnotation<T>;
}

type ActiveLayer<T> = SyntaxLayer | AnnotationLayer<T>;

function isUtf8Boundary(bytes: Uint8Array, offset: number): boolean {
  return (
    offset === 0 ||
    offset === bytes.length ||
    (offset < bytes.length && (bytes[offset]! & 0xc0) !== 0x80)
  );
}

function validateByteOffset(sourceIndex: SourceIndex, index: number, offset: number): void {
  if (!Number.isSafeInteger(offset) || offset < 0) {
    throw new RangeError(`annotation ${index} byte offset must be a non-negative integer`);
  }
  if (offset > sourceIndex.sourceBytes.length) {
    throw new RangeError(
      `annotation ${index} ends at byte ${offset}, beyond source length ${sourceIndex.sourceBytes.length}`,
    );
  }
  if (!isUtf8Boundary(sourceIndex.sourceBytes, offset)) {
    throw new RangeError(`annotation ${index} offset ${offset} is not a UTF-8 character boundary`);
  }
}

function validatePosition(index: number, position: Position): void {
  if (
    !Number.isSafeInteger(position.line) ||
    position.line < 0 ||
    !Number.isSafeInteger(position.column) ||
    position.column < 0
  ) {
    throw new RangeError(
      `annotation ${index} position must contain non-negative integer line and column values`,
    );
  }
}

function lineByteRange(
  sourceIndex: SourceIndex,
  index: number,
  line: number,
): { start: number; end: number } {
  const lineStartIndex = sourceIndex.lineStarts[line];
  if (lineStartIndex === undefined) {
    throw new RangeError(
      `annotation ${index} line ${line} is outside the source's ${sourceIndex.lineStarts.length} lines`,
    );
  }
  const nextLineStartIndex = sourceIndex.lineStarts[line + 1];
  const nextLineStart =
    nextLineStartIndex === undefined ? undefined : sourceIndex.utf8Offsets[nextLineStartIndex];

  return {
    start: sourceIndex.utf8Offsets[lineStartIndex] ?? 0,
    // The trailing newline is not part of the line.
    end: nextLineStart === undefined ? sourceIndex.sourceBytes.length : nextLineStart - 1,
  };
}

function resolvePosition(sourceIndex: SourceIndex, index: number, position: Position): number {
  validatePosition(index, position);

  const line = lineByteRange(sourceIndex, index, position.line);
  const lineStart = line.start;
  const lineLength = line.end - lineStart;

  if (position.column > lineLength) {
    throw new RangeError(
      `annotation ${index} column ${position.column} is beyond line ${position.line} byte length ${lineLength}`,
    );
  }

  const offset = lineStart + position.column;
  validateByteOffset(sourceIndex, index, offset);
  return offset;
}

function resolveRange(
  sourceIndex: SourceIndex,
  index: number,
  range: AnnotationRange,
): HighlightRange {
  let start: number;
  let end: number;

  if (range.type === "offset") {
    start = range.start;
    end = range.end;
    validateByteOffset(sourceIndex, index, start);
    validateByteOffset(sourceIndex, index, end);
  } else if (range.type === "position") {
    start = resolvePosition(sourceIndex, index, range.start);
    end = resolvePosition(sourceIndex, index, range.end);
  } else {
    throw new RangeError(`annotation ${index} has an unknown range type`);
  }

  if (start > end) {
    throw new RangeError(
      `annotation ${index} range start must not be after its end: ${start}..${end}`,
    );
  }

  return { start, end };
}

function resolveAnnotations<T>(
  sourceIndex: SourceIndex,
  annotations: readonly Annotation<T>[],
): ResolvedAnnotation<T>[] {
  return annotations.map((annotation, index) => ({
    range: resolveRange(sourceIndex, index, annotation.range),
    data: annotation.data,
  }));
}

function annotationBoundaries<T>(
  annotations: readonly ResolvedAnnotation<T>[],
): Map<number, Boundary> {
  const boundaries = new Map<number, Boundary>();

  const at = (offset: number): Boundary => {
    const boundary = boundaries.get(offset) ?? { starts: [], ends: [], points: [] };
    boundaries.set(offset, boundary);
    return boundary;
  };

  annotations.forEach((annotation, index) => {
    if (annotation.range.start === annotation.range.end) {
      at(annotation.range.start).points.push(index);
      return;
    }
    at(annotation.range.start).starts.push(index);
    at(annotation.range.end).ends.push(index);
  });

  return boundaries;
}

function applyBoundary(boundary: Boundary, active: Set<number>): void {
  for (const index of boundary.ends) active.delete(index);
  for (const index of boundary.starts) active.add(index);
}

/** Every annotation boundary in the source, in ascending byte order. */
interface BoundaryTable {
  byOffset: Map<number, Boundary>;
  offsets: number[];
}

/** Applies every boundary at or before `limit`, and reports where that left off. */
function advanceBoundaries(
  boundaries: BoundaryTable,
  active: Set<number>,
  from: number,
  limit: number,
): number {
  let index = from;
  while (index < boundaries.offsets.length && boundaries.offsets[index]! <= limit) {
    applyBoundary(boundaries.byOffset.get(boundaries.offsets[index]!)!, active);
    index += 1;
  }
  return index;
}

function sameLayer<T>(left: ActiveLayer<T>, right: ActiveLayer<T>): boolean {
  if (left.type !== right.type) return false;

  if (left.type === "syntax" && right.type === "syntax") {
    return left.id === right.id && left.scope === right.scope && left.language === right.language;
  }

  return left.type === "annotation" && right.type === "annotation" && left.index === right.index;
}

function desiredLayers<T>(
  activeAnnotations: Set<number>,
  annotations: readonly ResolvedAnnotation<T>[],
  syntaxLayers: readonly SyntaxLayer[],
): ActiveLayer<T>[] {
  const layers: ActiveLayer<T>[] = [...activeAnnotations]
    .sort((left, right) => left - right)
    .map((index) => ({
      type: "annotation",
      index,
      annotation: annotations[index]!,
    }));

  layers.push(...syntaxLayers);
  return layers;
}

function transitionLayers<T>(
  output: HighlightEvent<T>[],
  current: ActiveLayer<T>[],
  desired: ActiveLayer<T>[],
): ActiveLayer<T>[] {
  let common = 0;
  while (
    common < current.length &&
    common < desired.length &&
    sameLayer(current[common]!, desired[common]!)
  ) {
    common += 1;
  }

  for (let index = current.length - 1; index >= common; index -= 1) {
    output.push({
      type: current[index]!.type === "syntax" ? "end" : "annotationEnd",
    });
  }

  for (const layer of desired.slice(common)) {
    if (layer.type === "syntax") {
      output.push({
        type: "start",
        scope: layer.scope,
        language: layer.language,
      });
    } else {
      output.push({
        type: "annotationStart",
        annotation: layer.annotation,
      });
    }
  }

  return desired;
}

/** What the walk over the syntax events carries from one source event to the next. */
interface ComposeState<T> {
  boundaryIndex: number;
  activeLayers: ActiveLayer<T>[];
  pendingPoints: Set<number>;
}

/**
 * Emits the empty annotations sitting at `offset`, if any, as start/end pairs.
 *
 * Called with the layer stack already transitioned for `offset`, so a point at
 * the start of a span lands inside it and a point where a span ends lands
 * outside it, matching the half-open ranges everywhere else.
 */
function emitPoints<T>(
  output: HighlightEvent<T>[],
  boundaries: BoundaryTable,
  annotations: readonly ResolvedAnnotation<T>[],
  pending: Set<number>,
  offset: number,
): void {
  if (!pending.delete(offset)) return;
  for (const index of boundaries.byOffset.get(offset)!.points) {
    output.push({ type: "annotationStart", annotation: annotations[index]! });
    output.push({ type: "annotationEnd" });
  }
}

/**
 * Splits one source event at every annotation boundary inside it, reopening the
 * syntax layers around each piece so the emitted stream stays nested.
 */
function composeSourceEvent<T>(
  event: { start: number; end: number },
  state: ComposeState<T>,
  boundaries: BoundaryTable,
  activeAnnotations: Set<number>,
  annotations: readonly ResolvedAnnotation<T>[],
  syntaxLayers: readonly SyntaxLayer[],
  output: HighlightEvent<T>[],
): void {
  state.boundaryIndex = advanceBoundaries(
    boundaries,
    activeAnnotations,
    state.boundaryIndex,
    event.start,
  );

  let cursor = event.start;
  while (cursor < event.end) {
    const boundary = boundaries.offsets[state.boundaryIndex];
    const next = boundary !== undefined && boundary < event.end ? boundary : event.end;

    state.activeLayers = transitionLayers(
      output,
      state.activeLayers,
      desiredLayers(activeAnnotations, annotations, syntaxLayers),
    );
    emitPoints(output, boundaries, annotations, state.pendingPoints, cursor);

    if (cursor < next) {
      output.push({ type: "source", start: cursor, end: next });
    }
    cursor = next;

    state.boundaryIndex = advanceBoundaries(
      boundaries,
      activeAnnotations,
      state.boundaryIndex,
      cursor,
    );
  }
}

/** @internal */
export function composeAnnotations<T>(
  syntaxEvents: readonly SyntaxHighlightEvent[],
  annotations: readonly Annotation<T>[],
  sourceIndex: SourceIndex,
): HighlightEvent<T>[] {
  if (annotations.length === 0) return [...syntaxEvents];

  const resolvedAnnotations = resolveAnnotations(sourceIndex, annotations);

  const byOffset = annotationBoundaries(resolvedAnnotations);
  const boundaries: BoundaryTable = {
    byOffset,
    offsets: [...byOffset.keys()].sort((left, right) => left - right),
  };
  const activeAnnotations = new Set<number>();
  const syntaxLayers: SyntaxLayer[] = [];
  const output: HighlightEvent<T>[] = [];
  const state: ComposeState<T> = {
    boundaryIndex: 0,
    activeLayers: [],
    pendingPoints: new Set(
      [...byOffset].filter(([, boundary]) => boundary.points.length > 0).map(([offset]) => offset),
    ),
  };
  let nextSyntaxId = 0;

  for (const event of syntaxEvents) {
    if (event.type === "start") {
      syntaxLayers.push({
        type: "syntax",
        id: nextSyntaxId,
        scope: event.scope,
        language: event.language,
      });
      nextSyntaxId += 1;
    } else if (event.type === "end") {
      syntaxLayers.pop();
    } else {
      composeSourceEvent(
        event,
        state,
        boundaries,
        activeAnnotations,
        resolvedAnnotations,
        syntaxLayers,
        output,
      );
    }
  }

  transitionLayers(output, state.activeLayers, []);

  // A point at the end of the source has no following text to split, and one in
  // a source with no events at all is never reached by the walk. Both land here,
  // outside every scope, which is where the end of the document is.
  for (const offset of [...state.pendingPoints].sort((left, right) => left - right)) {
    emitPoints(output, boundaries, resolvedAnnotations, state.pendingPoints, offset);
  }

  return output;
}

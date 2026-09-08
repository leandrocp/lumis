import type { Node, Point, QueryCapture, QueryMatch, Range } from "web-tree-sitter";
import { LANGUAGES } from "./generated/languages-meta.js";
import { languageIdForFilename } from "./guess-language.js";
import type { LoadedLanguage, QueryCaptureOffset } from "./types.js";

interface RuntimeLookup {
  getLoadedLanguage(nameOrAlias: string): LoadedLanguage | undefined;
}

interface HighlightCapture {
  startByte: number;
  endByte: number;
  scope: string;
  language: string;
}

interface HighlightLayer {
  depth: number;
  language: LoadedLanguage;
  captures: LayerQueryCapture[];
  localDefinitionValueEnds: Uint32Array;
}

interface LayerQueryCapture {
  matchIndex: number;
  patternIndex: number;
  name: string;
  nodeId: number;
  startByte: number;
  endByte: number;
  isMissing: boolean;
  setProperties?: Record<string, string | null>;
}

interface CaptureSnapshot {
  captures: LayerQueryCapture[];
  matchCount: number;
}

/** @internal */
export interface SourceIndex {
  sourceBytes: Uint8Array;
  utf8Offsets: number[];
  lineStarts: number[];
}

export interface SourceMaps extends SourceIndex {
  utf16Indices: Array<number | undefined>;
  byteLineStarts: number[];
  sourceLength: number;
  sourceUtf8ByteLength: number;
}

interface LocalDef {
  name: string;
  valueEndByte: number;
  highlight?: string;
}

interface LocalScope {
  inherits: boolean;
  endByte: number;
  localDefs: LocalDef[];
}

export interface HighlightStartEvent {
  type: "start";
  scope: string;
  language: string;
}

export interface HighlightSourceEvent {
  type: "source";
  startByte: number;
  endByte: number;
}

export interface HighlightEndEvent {
  type: "end";
}

export type HighlightEvent = HighlightStartEvent | HighlightSourceEvent | HighlightEndEvent;

const encoder = new TextEncoder();
const decoder = new TextDecoder();

function utf8ByteLength(codePoint: number): number {
  if (codePoint <= 0x7f) return 1;
  if (codePoint <= 0x7ff) return 2;
  if (codePoint <= 0xffff) return 3;
  return 4;
}

function buildUtf8OffsetMap(source: string): number[] {
  const offsets = Array.from<number>({ length: source.length + 1 });
  offsets[0] = 0;
  let codeUnitOffset = 0;
  let byteOffset = 0;

  for (const char of source) {
    offsets[codeUnitOffset] = byteOffset;

    const codePoint = char.codePointAt(0)!;
    const charCodeUnits = char.length;
    const charBytes = utf8ByteLength(codePoint);

    for (let i = 1; i < charCodeUnits; i += 1) {
      offsets[codeUnitOffset + i] = byteOffset;
    }

    codeUnitOffset += charCodeUnits;
    byteOffset += charBytes;
    offsets[codeUnitOffset] = byteOffset;
  }

  return offsets;
}

/** One warning per language per process, not one per document. */
const warnedUnresolved = new Set<string>();

/**
 * Mirrors `catalog::find` in `lumis-wasm-runtime`, which gates the same warning
 * on the Rust side.
 */
const catalogNames = new Set(
  LANGUAGES.flatMap(({ id, aliases }) => aliases.concat(id)).map((name) => name.toLowerCase()),
);

/**
 * Say so when a document named a language that is not loaded.
 *
 * `web-tree-sitter` cannot fetch a parser inside a synchronous walk, so the
 * block stays plain. The native addon reports the same thing from Rust, so both
 * Node runtimes behave and sound identical.
 *
 * An injection query can name something that is not a language at all: html
 * captures the raw `<script type=...>` value, so `type="module"` asks for
 * "module" and `type="importmap"` for "importmap", each just before a more
 * specific pattern injects javascript or json into the same block. Those blocks
 * do highlight, so naming the discarded value would warn about correct output.
 */
export function warnUnresolvedInjection(id: string): void {
  if (!catalogNames.has(id.toLowerCase())) return;
  if (warnedUnresolved.has(id)) return;
  warnedUnresolved.add(id);
  console.warn(
    `Lumis could not load "${id}", injected inside the document being highlighted. ` +
      "Load it up front, prefetch it with cacheLanguages(), or use a bundle. " +
      "See https://lumis.sh/docs/advanced/wasm-and-cdn#highlighting-loads-what-a-document-needs",
  );
}

export function buildLineStartMap(source: string): number[] {
  const starts = [0];

  for (let i = 0; i < source.length; i += 1) {
    if (source[i] === "\n") {
      starts.push(i + 1);
    }
  }

  return starts;
}

export function buildSourceMaps(source: string): SourceMaps {
  const utf8Offsets = buildUtf8OffsetMap(source);
  const lineStarts = buildLineStartMap(source);
  const sourceUtf8ByteLength = utf8Offsets[source.length] ?? 0;
  const utf16Indices = Array.from<number | undefined>({ length: sourceUtf8ByteLength + 1 });
  for (const [index, byte] of utf8Offsets.entries()) {
    utf16Indices[byte] ??= index;
  }
  return {
    utf8Offsets,
    utf16Indices,
    lineStarts,
    byteLineStarts: lineStarts.map((index) => utf8Offsets[index] ?? 0),
    sourceBytes: encoder.encode(source),
    sourceLength: source.length,
    sourceUtf8ByteLength,
  };
}

/** @internal */
export function buildSourceIndex(source: string): SourceIndex {
  return buildSourceMaps(source);
}

function nodeStartByte(node: Node, maps: SourceMaps): number {
  return maps.utf8Offsets[node.startIndex] ?? 0;
}

function nodeEndByte(node: Node, maps: SourceMaps): number {
  return maps.utf8Offsets[node.endIndex] ?? 0;
}

interface MatchQueue {
  indexes: number[];
  cursor: number;
}

type CaptureMatchQueues = Map<number, Map<number, Map<string, MatchQueue>>>;

// Query.captures() preserves Tree-sitter's stream order but omits match identity.
// Query.matches() preserves match identity but not that order. Join both views so
// whole matches can be discarded exactly when the native highlighter discards them.
function snapshotCapturesWithMatches(
  captures: QueryCapture[],
  matches: QueryMatch[],
  maps: SourceMaps,
  firstHighlightPattern: number,
  captureOffsets: Array<Record<string, QueryCaptureOffset> | undefined>,
): CaptureSnapshot {
  const queues = buildCaptureMatchQueues(matches, firstHighlightPattern);

  const result: LayerQueryCapture[] = [];
  let nextMatchIndex = matches.length;

  for (const capture of captures) {
    if (capture.patternIndex < firstHighlightPattern) continue;

    const queue = queues.get(capture.patternIndex)?.get(capture.node.id)?.get(capture.name);
    let matchIndex = queue?.indexes[queue.cursor];
    if (queue && matchIndex != null) {
      queue.cursor += 1;
    } else {
      // web-tree-sitter can omit valid captures from matches(). Give each
      // unmatched capture its own identity while preserving captures() order.
      matchIndex = nextMatchIndex;
      nextMatchIndex += 1;
    }

    result.push(snapshotCapture(capture, matchIndex, maps, captureOffsets));
  }

  return { captures: result, matchCount: nextMatchIndex };
}

// The match indexes each (pattern, node, capture name) was seen at, in order,
// so a capture from `captures()` can be paired with its match from `matches()`.
function buildCaptureMatchQueues(
  matches: QueryMatch[],
  firstHighlightPattern: number,
): CaptureMatchQueues {
  const queues: CaptureMatchQueues = new Map();

  for (const [matchIndex, match] of matches.entries()) {
    if (match.patternIndex < firstHighlightPattern) continue;

    for (const capture of match.captures) {
      let nodes = queues.get(capture.patternIndex);
      if (!nodes) {
        nodes = new Map();
        queues.set(capture.patternIndex, nodes);
      }

      let names = nodes.get(capture.node.id);
      if (!names) {
        names = new Map();
        nodes.set(capture.node.id, names);
      }

      const queue = names.get(capture.name);
      if (queue) {
        queue.indexes.push(matchIndex);
      } else {
        names.set(capture.name, { indexes: [matchIndex], cursor: 0 });
      }
    }
  }

  return queues;
}

function snapshotCapture(
  capture: QueryCapture,
  matchIndex: number,
  maps: SourceMaps,
  captureOffsets: Array<Record<string, QueryCaptureOffset> | undefined>,
): LayerQueryCapture {
  // Neovim resolves a highlight capture's range through `get_range`, so `#offset!`
  // narrows the highlighted span as well as injection ranges.
  const offset = captureOffsets[capture.patternIndex]?.[capture.name];
  const adjusted = offset ? applyCaptureOffset(nodeToRange(capture.node), offset, maps) : undefined;

  return {
    matchIndex,
    patternIndex: capture.patternIndex,
    name: capture.name,
    nodeId: capture.node.id,
    startByte: adjusted
      ? (maps.utf8Offsets[adjusted.startIndex] ?? nodeStartByte(capture.node, maps))
      : nodeStartByte(capture.node, maps),
    endByte: adjusted
      ? (maps.utf8Offsets[adjusted.endIndex] ?? nodeEndByte(capture.node, maps))
      : nodeEndByte(capture.node, maps),
    isMissing: capture.node.isMissing,
    setProperties: capture.setProperties,
  };
}

function resolveInjection(
  source: string,
  match: QueryMatch,
  language: LoadedLanguage,
  maps: SourceMaps,
  parentLanguageName?: string,
): { languageName?: string; ranges: Range[]; combined: boolean } {
  const offsets = language.config.captureOffsets[match.patternIndex];
  const captured = readInjectionCaptures(source, match, language, maps, offsets);
  const setProperties = match.setProperties ?? {};

  const includeChildren = "injection.include-children" in setProperties;
  const ranges = captured.contentCaptures.flatMap((capture) =>
    getCaptureRanges(capture, maps, includeChildren, offsets?.[capture.name]),
  );

  return {
    languageName: injectionLanguageName(captured, setProperties, language, parentLanguageName),
    ranges,
    combined: "injection.combined" in setProperties,
  };
}

function readInjectionCaptures(
  source: string,
  match: QueryMatch,
  language: LoadedLanguage,
  maps: SourceMaps,
  offsets: Record<string, QueryCaptureOffset> | undefined,
): { languageName?: string; filenameLanguage?: string; contentCaptures: QueryCapture[] } {
  let languageName: string | undefined;
  let filenameLanguage: string | undefined;
  const contentCaptures: QueryCapture[] = [];

  for (const capture of match.captures) {
    const metadata = language.config.captureMetadata[capture.name];
    if (!metadata) continue;

    if (metadata.isInjectionContent) {
      contentCaptures.push(capture);
    } else if (metadata.isInjectionLanguage) {
      languageName ??= capture.node.text;
    } else if (metadata.isInjectionFilename) {
      filenameLanguage ??= filenameCaptureLanguage(source, capture, maps, offsets);
    }
  }

  return { languageName, filenameLanguage, contentCaptures };
}

// Neovim resolves this capture through `vim.filetype.match`, so the text is a
// path rather than a language name. A diff names `b/lib/varsel.ex`. The whole
// capture is one path, so masking named children out of it, as injection content
// wants, would resolve a fragment of the filename. `injection_for_match` reads
// the same undivided range.
function filenameCaptureLanguage(
  source: string,
  capture: QueryCapture,
  maps: SourceMaps,
  offsets: Record<string, QueryCaptureOffset> | undefined,
): string | undefined {
  const range = applyCaptureOffset(nodeToRange(capture.node), offsets?.[capture.name], maps);
  return languageIdForFilename(source.slice(range.startIndex, range.endIndex));
}

// In precedence order: an explicit `@injection.language` capture, a filename the
// capture resolved to, `injection.language`, `injection.self`, then
// `injection.parent`.
function injectionLanguageName(
  captured: { languageName?: string; filenameLanguage?: string },
  setProperties: NonNullable<QueryMatch["setProperties"]>,
  language: LoadedLanguage,
  parentLanguageName?: string,
): string | undefined {
  let languageName = captured.languageName ?? captured.filenameLanguage;
  if (!languageName) {
    languageName = setProperties["injection.language"] ?? undefined;
  }
  if (!languageName && "injection.self" in setProperties) {
    languageName = language.definition.id;
  }
  if (!languageName && "injection.parent" in setProperties) {
    languageName = parentLanguageName;
  }

  return languageName;
}

function collectHighlightLayers(
  source: string,
  maps: SourceMaps,
  runtime: RuntimeLookup,
  language: LoadedLanguage,
  depth: number,
  includedRanges?: Range[],
  parentLanguageName?: string,
): HighlightLayer[] {
  const tree = language.parser.parse(source, null, includedRanges ? { includedRanges } : undefined);
  if (!tree) return [];

  try {
    const rootNode = tree.rootNode;
    const queryMatches = language.config.query.matches(rootNode);
    const queryCaptures = language.config.query.captures(rootNode);
    const snapshot = snapshotCapturesWithMatches(
      queryCaptures,
      queryMatches,
      maps,
      language.config.injectionPatternEnd,
      language.config.captureOffsets,
    );
    const localDefinitionValueEnds = collectLocalDefinitionValueEnds(
      queryMatches,
      language,
      maps,
      snapshot.matchCount,
    );

    const layers: HighlightLayer[] = [
      {
        depth,
        language,
        captures: snapshot.captures,
        localDefinitionValueEnds,
      },
    ];

    if (language.config.injectionPatternEnd === 0) {
      return layers;
    }

    layers.push(
      ...collectInjectedLayers(
        queryMatches,
        source,
        maps,
        runtime,
        language,
        depth,
        parentLanguageName,
      ),
    );

    return layers;
  } finally {
    tree.delete();
  }
}

// Every layer the injection patterns of this language contribute.
//
// `injection.combined` means every match of the pattern feeds one layer, not one
// layer each. Markdown injects each html tag separately, so without this
// `</h1>` is parsed on its own and never becomes a closing tag.
function collectInjectedLayers(
  queryMatches: QueryMatch[],
  source: string,
  maps: SourceMaps,
  runtime: RuntimeLookup,
  language: LoadedLanguage,
  depth: number,
  parentLanguageName?: string,
): HighlightLayer[] {
  const layers: HighlightLayer[] = [];
  const combined = new Map<number, { languageName: string; ranges: Range[] }>();

  const inject = (languageName: string, ranges: Range[]) => {
    layers.push(...injectedLayers(languageName, ranges, source, maps, runtime, language, depth));
  };

  for (const match of queryMatches) {
    if (match.patternIndex >= language.config.injectionPatternEnd) continue;

    const resolved = resolveInjection(source, match, language, maps, parentLanguageName);
    if (!resolved.languageName || resolved.ranges.length === 0) continue;

    if (resolved.combined) {
      addCombinedInjection(combined, match.patternIndex, resolved.languageName, resolved.ranges);
    } else {
      inject(resolved.languageName, resolved.ranges);
    }
  }

  for (const { languageName, ranges } of combined.values()) {
    // Tree-sitter requires included ranges in ascending order.
    ranges.sort((a, b) => a.startIndex - b.startIndex);
    inject(languageName, ranges);
  }

  return layers;
}

// The layers an injected language contributes, or none when the runtime has not
// loaded it.
function injectedLayers(
  languageName: string,
  ranges: Range[],
  source: string,
  maps: SourceMaps,
  runtime: RuntimeLookup,
  language: LoadedLanguage,
  depth: number,
): HighlightLayer[] {
  const injectedLanguage = runtime.getLoadedLanguage(languageName);
  if (!injectedLanguage) {
    warnUnresolvedInjection(languageName);
    return [];
  }

  return collectHighlightLayers(
    source,
    maps,
    runtime,
    injectedLanguage,
    depth + 1,
    ranges,
    language.definition.id,
  );
}

// The end byte of each match's `@local.definition.value` capture, indexed by
// match, or 0 where the match has none.
function collectLocalDefinitionValueEnds(
  queryMatches: QueryMatch[],
  language: LoadedLanguage,
  maps: SourceMaps,
  matchCount: number,
): Uint32Array {
  const ends = new Uint32Array(matchCount);

  for (const [matchIndex, match] of queryMatches.entries()) {
    const value = match.captures.find(
      (capture) => language.config.captureMetadata[capture.name]?.isLocalDefinitionValue,
    );
    if (value) {
      ends[matchIndex] = nodeEndByte(value.node, maps);
    }
  }

  return ends;
}

function addCombinedInjection(
  combined: Map<number, { languageName: string; ranges: Range[] }>,
  patternIndex: number,
  languageName: string,
  ranges: Range[],
): void {
  const group = combined.get(patternIndex);
  if (group) {
    group.ranges.push(...ranges);
  } else {
    combined.set(patternIndex, { languageName, ranges: [...ranges] });
  }
}

function getCaptureRanges(
  capture: QueryCapture,
  maps: SourceMaps,
  includeChildren: boolean,
  offset?: QueryCaptureOffset,
): Range[] {
  // The outer bounds come from the `#offset!`-adjusted range; children are still masked
  // out from the node itself, matching Neovim's `get_node_ranges`.
  const range = applyCaptureOffset(nodeToRange(capture.node), offset, maps);

  if (includeChildren || capture.node.childCount === 0) {
    return [range];
  }

  return maskNamedChildren(capture, range, maps);
}

// Masking has to happen inside the adjusted bounds, not inside the node's own.
// `(#offset! @c 0 1 0 1)` on a diff hunk line pushes the end past the node onto
// the newline, and intersecting with the unadjusted node would drop it again,
// joining every hunk line into one. Mirrors `intersect_ranges` in
// lumis-wasm-runtime, which builds the same spans from `content.range`.
function maskNamedChildren(capture: QueryCapture, range: Range, maps: SourceMaps): Range[] {
  const ranges: Range[] = [];
  let startIndex = range.startIndex;

  for (const child of capture.node.children) {
    if (!child?.isNamed) continue;

    const maskStart = Math.max(child.startIndex, range.startIndex);
    const maskEnd = Math.min(child.endIndex, range.endIndex);
    // A child wholly outside the adjusted range clips to nothing. Masking it
    // anyway would push a range out to its unclipped start, past the end the
    // offset asked for.
    if (maskStart >= maskEnd || maskEnd <= startIndex) continue;

    if (maskStart > startIndex) {
      ranges.push(makeRangeAt(startIndex, maskStart, maps));
    }
    startIndex = maskEnd;
  }

  if (startIndex < range.endIndex) {
    ranges.push(makeRangeAt(startIndex, range.endIndex, maps));
  }

  return ranges;
}

/**
 * Applies `#offset!` to a range.
 *
 * Neovim applies the deltas to UTF-8 byte columns. Lumis also requires both
 * adjusted endpoints to remain valid UTF-8 boundaries and inside the document;
 * otherwise it keeps the capture's original range.
 */
export function applyCaptureOffset(
  range: Range,
  offset: QueryCaptureOffset | undefined,
  maps: SourceMaps,
): Range {
  if (!offset) return range;

  const start = shiftEndpoint(
    range.startIndex,
    range.startPosition.row,
    offset.startRow,
    offset.startColumn,
    maps,
  );
  const end = shiftEndpoint(
    range.endIndex,
    range.endPosition.row,
    offset.endRow,
    offset.endColumn,
    maps,
  );
  if (!start || !end || start.byte > end.byte) {
    return range;
  }

  return makeRange(start.index, end.index, start.position, end.position);
}

function shiftEndpoint(
  index: number,
  row: number,
  rowDelta: number,
  columnDelta: number,
  maps: SourceMaps,
): { byte: number; index: number; position: Point } | undefined {
  const byte = maps.utf8Offsets[index];
  const originalLineStart = maps.byteLineStarts[row];
  const targetRow = row + rowDelta;
  const targetLineStart = maps.byteLineStarts[targetRow];
  if (byte == null || originalLineStart == null || targetLineStart == null) return undefined;

  const byteColumn = byte - originalLineStart + columnDelta;
  if (!isSafeColumn(targetRow, byteColumn)) return undefined;

  const targetByte = targetLineStart + byteColumn;
  if (!Number.isSafeInteger(targetByte) || targetByte > endpointLimit(targetRow, rowDelta, maps)) {
    return undefined;
  }

  const targetIndex = maps.utf16Indices[targetByte];
  const targetUtf16LineStart = maps.lineStarts[targetRow];
  if (targetIndex == null || targetUtf16LineStart == null) return undefined;

  return {
    byte: targetByte,
    index: targetIndex,
    position: { row: targetRow, column: targetIndex - targetUtf16LineStart },
  };
}

function isSafeColumn(targetRow: number, byteColumn: number): boolean {
  return Number.isSafeInteger(targetRow) && Number.isSafeInteger(byteColumn) && byteColumn >= 0;
}

// The furthest byte a shifted endpoint may land on. A same-row column one past
// the end addresses that line's newline, which the diff injection queries rely
// on to keep joined hunk lines apart. Stopping there rather than at the document
// end keeps the returned byte and point describing one place; Neovim clamps
// neither, and returns a point whose row no longer holds its byte.
function endpointLimit(targetRow: number, rowDelta: number, maps: SourceMaps): number {
  const nextLineStart = maps.byteLineStarts[targetRow + 1];

  if (rowDelta === 0) {
    return nextLineStart ?? maps.sourceUtf8ByteLength;
  }

  return nextLineStart == null ? maps.sourceUtf8ByteLength : nextLineStart - 1;
}

function indexToPoint(index: number, lineStarts: number[]): Point {
  let low = 0;
  let high = lineStarts.length - 1;

  while (low <= high) {
    const mid = Math.floor((low + high) / 2);
    const lineStart = lineStarts[mid] ?? 0;
    const nextLineStart = lineStarts[mid + 1] ?? Number.POSITIVE_INFINITY;

    if (index < lineStart) {
      high = mid - 1;
    } else if (index >= nextLineStart) {
      low = mid + 1;
    } else {
      return { row: mid, column: index - lineStart };
    }
  }

  throw new Error(`Invalid source index ${index}`);
}

export function getInjectionRanges(node: Node, includeChildren: boolean): Range[] {
  if (includeChildren || node.childCount === 0) {
    return [nodeToRange(node)];
  }

  const ranges: Range[] = [];
  let startIndex = node.startIndex;
  let startPosition = node.startPosition;

  for (const child of node.children) {
    if (!child) continue;

    if (!child.isNamed) {
      continue;
    }

    if (startIndex < child.startIndex) {
      ranges.push(makeRange(startIndex, child.startIndex, startPosition, child.startPosition));
    }

    startIndex = child.endIndex;
    startPosition = child.endPosition;
  }

  if (startIndex < node.endIndex) {
    ranges.push(makeRange(startIndex, node.endIndex, startPosition, node.endPosition));
  }

  return ranges;
}

function makeRangeAt(startIndex: number, endIndex: number, maps: SourceMaps): Range {
  return makeRange(
    startIndex,
    endIndex,
    indexToPoint(startIndex, maps.lineStarts),
    indexToPoint(endIndex, maps.lineStarts),
  );
}

function nodeToRange(node: Node): Range {
  return makeRange(node.startIndex, node.endIndex, node.startPosition, node.endPosition);
}

function makeRange(
  startIndex: number,
  endIndex: number,
  startPosition: Point,
  endPosition: Point,
): Range {
  return { startIndex, endIndex, startPosition, endPosition };
}

/** @internal */
export function buildHighlightEventsWithSourceIndex(
  source: string,
  language: LoadedLanguage,
  runtime: RuntimeLookup,
  options: { rainbowBrackets?: boolean } = {},
): { events: HighlightEvent[]; sourceIndex: SourceIndex } {
  const maps = buildSourceMaps(source);
  const layers = collectHighlightLayers(source, maps, runtime, language, 0);
  const events = buildNestedEvents(layers, maps);
  return {
    events: options.rainbowBrackets ? applyRainbowBrackets(source, events, language, maps) : events,
    sourceIndex: maps,
  };
}

export function buildHighlightEvents(
  source: string,
  language: LoadedLanguage,
  runtime: RuntimeLookup,
  options: { rainbowBrackets?: boolean } = {},
): HighlightEvent[] {
  return buildHighlightEventsWithSourceIndex(source, language, runtime, options).events;
}

const RAINBOW_BRACKET_SCOPES = [
  "punctuation.bracket.rainbow.1",
  "punctuation.bracket.rainbow.2",
  "punctuation.bracket.rainbow.3",
  "punctuation.bracket.rainbow.4",
  "punctuation.bracket.rainbow.5",
  "punctuation.bracket.rainbow.6",
];

interface BracketPair {
  open: { startByte: number; endByte: number };
  close: { startByte: number; endByte: number };
}

function queryRainbowBracketRanges(
  source: string,
  language: LoadedLanguage,
  maps: SourceMaps,
): Array<{ startByte: number; endByte: number; scope: string }> {
  if (!language.brackets) return [];

  const tree = language.parser.parse(source);
  if (!tree) return [];

  try {
    const pairs: BracketPair[] = [];
    for (const match of language.brackets.query.matches(tree.rootNode)) {
      if (language.brackets.rainbowExcludePatterns[match.patternIndex]) continue;
      pairs.push(...matchBracketPairs(match, language.brackets.captureMetadata, maps));
    }

    return colorizeBracketPairs(pairs);
  } finally {
    tree.delete();
  }
}

// The open/close captures of one match, paired positionally. A pair counts only
// when the open precedes the close and one side is a single byte, which is what
// keeps a multi-byte token from pairing with another multi-byte one.
function matchBracketPairs(
  match: QueryMatch,
  captureMetadata: Record<string, { isOpen?: boolean; isClose?: boolean } | undefined>,
  maps: SourceMaps,
): BracketPair[] {
  const opens: Array<{ startByte: number; endByte: number }> = [];
  const closes: Array<{ startByte: number; endByte: number }> = [];

  for (const capture of match.captures) {
    const metadata = captureMetadata[capture.name];
    const range = {
      startByte: nodeStartByte(capture.node, maps),
      endByte: nodeEndByte(capture.node, maps),
    };
    if (metadata?.isOpen) {
      opens.push(range);
    } else if (metadata?.isClose) {
      closes.push(range);
    }
  }

  const pairs: BracketPair[] = [];
  for (let index = 0; index < Math.min(opens.length, closes.length); index += 1) {
    const open = opens[index]!;
    const close = closes[index]!;
    if (isBracketPair(open, close)) {
      pairs.push({ open, close });
    }
  }

  return pairs;
}

function isBracketPair(
  open: { startByte: number; endByte: number },
  close: { startByte: number; endByte: number },
): boolean {
  return (
    open.startByte < close.endByte &&
    (open.endByte - open.startByte === 1 || close.endByte - close.startByte === 1)
  );
}

function colorizeBracketPairs(
  pairs: BracketPair[],
): Array<{ startByte: number; endByte: number; scope: string }> {
  const opens = pairs
    .map((pair) => pair.open)
    .sort((a, b) => a.startByte - b.startByte || a.endByte - b.endByte)
    .filter((range, index, all) => {
      const previous = all[index - 1];
      return (
        !previous || previous.startByte !== range.startByte || previous.endByte !== range.endByte
      );
    });

  const colorPairs = pairs.slice().sort((a, b) => a.close.endByte - b.close.endByte);
  const openStack: Array<{ startByte: number; endByte: number }> = [];
  const ranges: Array<{ startByte: number; endByte: number; scope: string }> = [];
  let openIndex = 0;

  for (const pair of colorPairs) {
    while (openIndex < opens.length && opens[openIndex]!.startByte < pair.close.startByte) {
      openStack.push(opens[openIndex]!);
      openIndex += 1;
    }

    const lastOpen = openStack.at(-1);
    if (
      lastOpen &&
      lastOpen.startByte === pair.open.startByte &&
      lastOpen.endByte === pair.open.endByte
    ) {
      const scope = RAINBOW_BRACKET_SCOPES[(openStack.length - 1) % RAINBOW_BRACKET_SCOPES.length]!;
      ranges.push({ startByte: pair.open.startByte, endByte: pair.open.endByte, scope });
      ranges.push({ startByte: pair.close.startByte, endByte: pair.close.endByte, scope });
      openStack.pop();
    }
  }

  return ranges.sort((a, b) => a.startByte - b.startByte || a.endByte - b.endByte);
}

function applyRainbowBrackets(
  source: string,
  events: HighlightEvent[],
  language: LoadedLanguage,
  maps: SourceMaps,
): HighlightEvent[] {
  const ranges = queryRainbowBracketRanges(source, language, maps);
  if (ranges.length === 0) return events;

  const output: HighlightEvent[] = [];
  let rangeIndex = 0;

  for (const event of events) {
    if (event.type !== "source") {
      output.push(event);
      continue;
    }

    while (rangeIndex < ranges.length && ranges[rangeIndex]!.endByte <= event.startByte) {
      rangeIndex += 1;
    }

    output.push(...splitSourceEvent(event, ranges, rangeIndex, language.definition.id));
  }

  return output;
}

// One source event, with a start/source/end triple spliced in for every bracket
// range it wholly contains. A range that straddles the event is left to the
// event that does contain it.
function splitSourceEvent(
  event: { type: "source"; startByte: number; endByte: number },
  ranges: Array<{ startByte: number; endByte: number; scope: string }>,
  rangeIndex: number,
  languageId: string,
): HighlightEvent[] {
  const output: HighlightEvent[] = [];
  let cursor = event.startByte;

  for (let index = rangeIndex; index < ranges.length; index += 1) {
    const range = ranges[index]!;
    if (range.startByte >= event.endByte) break;
    if (range.startByte < event.startByte || range.endByte > event.endByte) continue;

    if (cursor < range.startByte) {
      output.push({ type: "source", startByte: cursor, endByte: range.startByte });
    }
    output.push({ type: "start", scope: range.scope, language: languageId });
    output.push({ type: "source", startByte: range.startByte, endByte: range.endByte });
    output.push({ type: "end" });
    cursor = range.endByte;
  }

  if (cursor < event.endByte) {
    output.push({ type: "source", startByte: cursor, endByte: event.endByte });
  }

  return output;
}

interface LayerState extends HighlightLayer {
  captureIndex: number;
  removedMatches: Uint8Array;
  scopeStack: LocalScope[];
  highlightEndStack: number[];
}

interface Boundary {
  offset: number;
  isStart: boolean;
}

function takeCapture(layer: LayerState): LayerQueryCapture | undefined {
  const capture = peekCapture(layer);
  if (capture) layer.captureIndex += 1;
  return capture;
}

// Walks the locals captures on one node, recording scopes and definitions, and
// returns the capture the highlight patterns should see. Returns undefined when
// the node's captures run out before leaving the locals patterns.
function consumeLocalCaptures(
  layer: LayerState,
  initialCapture: LayerQueryCapture,
  maps: SourceMaps,
):
  | { capture: LayerQueryCapture; definitionTarget?: LocalDef; referenceHighlight?: string }
  | undefined {
  let capture = initialCapture;
  let definitionTarget: LocalDef | undefined;
  let referenceHighlight: string | undefined;

  while (capture.patternIndex < layer.language.config.localsPatternEnd) {
    const applied = applyLocalCapture(layer, capture, maps, definitionTarget != null);
    definitionTarget = applied.definitionTarget ?? definitionTarget;
    referenceHighlight = applied.referenceHighlight ?? referenceHighlight;

    const next = peekCapture(layer);
    if (!next || next.nodeId !== capture.nodeId) return undefined;
    capture = takeCapture(layer)!;
  }

  return { capture, definitionTarget, referenceHighlight };
}

// A scope opens, a definition is recorded, or a reference resolves against the
// definitions already in scope. A reference is only resolved before this node
// has produced a definition of its own.
function applyLocalCapture(
  layer: LayerState,
  capture: LayerQueryCapture,
  maps: SourceMaps,
  hasDefinition: boolean,
): { definitionTarget?: LocalDef; referenceHighlight?: string } {
  const metadata = layer.language.config.captureMetadata[capture.name];

  if (metadata?.isLocalScope) {
    pushLocalScope(layer, capture);
  } else if (metadata?.isLocalDefinition) {
    return { definitionTarget: pushLocalDefinition(layer, capture, maps) };
  } else if (metadata?.isLocalReference && !hasDefinition) {
    return { referenceHighlight: findReferenceHighlight(layer, capture, maps) };
  }

  return {};
}

// The last capture on the node that resolves to a highlight wins it.
function chooseHighlightCapture(
  layer: LayerState,
  initialCapture: LayerQueryCapture,
  isLocal: boolean,
): LayerQueryCapture {
  let capture = initialCapture;

  while (true) {
    const next = peekCapture(layer);
    if (!next || next.nodeId !== capture.nodeId) break;

    const following = takeCapture(layer)!;
    if (isLocal && layer.language.config.nonLocalVariablePatterns[following.patternIndex]) {
      continue;
    }

    // A capture with no recognized highlight does not win the node.
    // nvim-treesitter marks helper captures `@_name`, and Neovim skips them
    // because they resolve to no highlight group. Letting one win here would
    // blank the node and discard its match's other captures with it.
    if (!layer.language.config.captureMetadata[following.name]?.highlightScope) continue;

    layer.removedMatches[capture.matchIndex] = 1;
    capture = following;
  }

  return capture;
}

function consumeNextHighlight(
  layer: LayerState,
  lastRange: { startByte: number; endByte: number; depth: number } | undefined,
  maps: SourceMaps,
): HighlightCapture | undefined {
  let capture = takeCapture(layer);
  if (!capture) return undefined;

  while (layer.scopeStack.length > 1 && capture.startByte > layer.scopeStack.at(-1)!.endByte) {
    layer.scopeStack.pop();
  }

  const localCaptures = consumeLocalCaptures(layer, capture, maps);
  if (!localCaptures) return undefined;
  ({ capture } = localCaptures);
  const { definitionTarget, referenceHighlight } = localCaptures;

  if (shadowedByShallowerLayer(capture, layer, lastRange)) return undefined;

  capture = chooseHighlightCapture(layer, capture, Boolean(definitionTarget ?? referenceHighlight));

  // A MISSING node is synthesised by error recovery, spans no bytes, and so can
  // only ever produce an empty span. Skip it: this runtime and the Rust one do
  // not recover identically — given `write!(x, "y")`, the injected macro body
  // parses to a `MISSING ";"` here and to an `ERROR` there — and highlighting
  // the artefact would leak that disagreement into output the two are required
  // to render identically.
  if (capture.isMissing) return undefined;

  return highlightCapture(layer, capture, definitionTarget, referenceHighlight);
}

// A reference takes the highlight of the definition it resolved to; anything
// else takes its capture's own. A definition also records its highlight, so the
// references that follow it can find one.
function highlightCapture(
  layer: LayerState,
  capture: LayerQueryCapture,
  definitionTarget: LocalDef | undefined,
  referenceHighlight: string | undefined,
): HighlightCapture | undefined {
  const scope = layer.language.config.captureMetadata[capture.name]?.highlightScope;
  if (definitionTarget) {
    definitionTarget.highlight = scope;
  }

  const effectiveScope = referenceHighlight ?? scope;
  if (!effectiveScope) return undefined;

  return {
    startByte: capture.startByte,
    endByte: capture.endByte,
    scope: effectiveScope,
    language: layer.language.definition.id,
  };
}

// The same span already highlighted by a layer closer to the surface.
function shadowedByShallowerLayer(
  capture: LayerQueryCapture,
  layer: LayerState,
  lastRange: { startByte: number; endByte: number; depth: number } | undefined,
): boolean {
  return (
    lastRange != null &&
    capture.startByte === lastRange.startByte &&
    capture.endByte === lastRange.endByte &&
    layer.depth < lastRange.depth
  );
}

// `local.scope-inherits` defaults to true when the property is absent.
function pushLocalScope(layer: LayerState, capture: LayerQueryCapture): void {
  const inheritsValue = capture.setProperties?.["local.scope-inherits"];
  layer.scopeStack.push({
    inherits: inheritsValue == null || inheritsValue === "true",
    endByte: capture.endByte,
    localDefs: [],
  });
}

function pushLocalDefinition(
  layer: LayerState,
  capture: LayerQueryCapture,
  maps: SourceMaps,
): LocalDef {
  const definition: LocalDef = {
    name: decoder.decode(maps.sourceBytes.subarray(capture.startByte, capture.endByte)),
    valueEndByte: layer.localDefinitionValueEnds[capture.matchIndex]!,
  };
  layer.scopeStack.at(-1)!.localDefs.push(definition);
  return definition;
}

// Whichever comes first: the next capture that opens, or the innermost
// highlight that closes.
function nextBoundary(layer: LayerState): Boundary | undefined {
  const nextStart = peekCapture(layer)?.startByte;
  const nextEnd = layer.highlightEndStack.at(-1);

  if (nextStart != null && nextEnd != null) {
    return nextStart < nextEnd
      ? { offset: nextStart, isStart: true }
      : { offset: nextEnd, isStart: false };
  }
  if (nextStart != null) return { offset: nextStart, isStart: true };
  if (nextEnd != null) return { offset: nextEnd, isStart: false };
  return undefined;
}

// The highlight of the nearest enclosing definition of the same name, searching
// outward until a scope that does not inherit.
function findReferenceHighlight(
  layer: LayerState,
  capture: LayerQueryCapture,
  maps: SourceMaps,
): string | undefined {
  const name = decoder.decode(maps.sourceBytes.subarray(capture.startByte, capture.endByte));

  for (let scopeIndex = layer.scopeStack.length - 1; scopeIndex >= 0; scopeIndex -= 1) {
    const scope = layer.scopeStack[scopeIndex]!;
    for (let defIndex = scope.localDefs.length - 1; defIndex >= 0; defIndex -= 1) {
      const definition = scope.localDefs[defIndex]!;
      if (definition.name === name && capture.startByte >= definition.valueEndByte) {
        return definition.highlight;
      }
    }
    if (!scope.inherits) break;
  }
  return undefined;
}

function peekCapture(layer: LayerState): LayerQueryCapture | undefined {
  while (layer.captureIndex < layer.captures.length) {
    const capture = layer.captures[layer.captureIndex]!;
    if (layer.removedMatches[capture.matchIndex] !== 0) {
      layer.captureIndex += 1;
      continue;
    }
    return capture;
  }
  return undefined;
}

function precedes(
  candidate: Boundary,
  candidateDepth: number,
  current: Boundary,
  currentDepth: number,
): boolean {
  if (candidate.offset !== current.offset) {
    return candidate.offset < current.offset;
  }
  if (candidate.isStart !== current.isStart) {
    return !candidate.isStart;
  }
  return candidateDepth > currentDepth;
}

/** Merge parser layers using tree-sitter-highlight's boundary ordering. */
// The layer whose next boundary comes first, deepest layer winning a tie.
function nextLayerBoundary(
  layers: LayerState[],
): { layer: LayerState; boundary: Boundary } | undefined {
  let layer: LayerState | undefined;
  let boundary: Boundary | undefined;

  for (const candidate of layers) {
    const candidateBoundary = nextBoundary(candidate);
    if (
      candidateBoundary &&
      (!boundary || !layer || precedes(candidateBoundary, candidate.depth, boundary, layer.depth))
    ) {
      layer = candidate;
      boundary = candidateBoundary;
    }
  }

  return layer && boundary ? { layer, boundary } : undefined;
}

function buildNestedEvents(inputLayers: HighlightLayer[], maps: SourceMaps): HighlightEvent[] {
  const events: HighlightEvent[] = [];

  const layers: LayerState[] = inputLayers.map((layer) => ({
    ...layer,
    captureIndex: 0,
    removedMatches: new Uint8Array(layer.localDefinitionValueEnds.length),
    scopeStack: [
      {
        inherits: false,
        endByte: Number.POSITIVE_INFINITY,
        localDefs: [],
      },
    ],
    highlightEndStack: [],
  }));
  let cursor = 0;
  let lastHighlightRange: { startByte: number; endByte: number; depth: number } | undefined;

  function emitSource(endByte: number): void {
    if (endByte > cursor) {
      events.push({ type: "source", startByte: cursor, endByte });
      cursor = endByte;
    }
  }

  while (true) {
    const next = nextLayerBoundary(layers);
    if (!next) break;
    const { layer, boundary } = next;

    if (!boundary.isStart) {
      layer.highlightEndStack.pop();
      emitSource(boundary.offset);
      events.push({ type: "end" });
      continue;
    }

    const capture = consumeNextHighlight(layer, lastHighlightRange, maps);
    if (!capture) continue;

    emitSource(capture.startByte);
    events.push({
      type: "start",
      scope: capture.scope,
      language: capture.language,
    });
    layer.highlightEndStack.push(capture.endByte);
    lastHighlightRange = {
      startByte: capture.startByte,
      endByte: capture.endByte,
      depth: layer.depth,
    };
  }

  emitSource(maps.sourceUtf8ByteLength);

  return events;
}

/**
 * The lines of a highlight.
 *
 * Every line-based formatter walks the same line-decorated stream: text split
 * at each newline, terminators left out, and every open scope reopened on the
 * next line. {@link linesFromEvents} returns those lines as data; the HTML
 * helpers render them.
 */
import type {
  Decoration,
  HighlightEvent,
  HighlightRange,
  HighlightSpan,
  HighlightStyle,
} from "../types.js";
import { LineSelection, composeLineDecorations, rainbowBracketScope } from "../decorations.js";
import { HIGHLIGHT_NAMES } from "../highlights.js";

const _encoder = new TextEncoder();
const _decoder = new TextDecoder();

// The largest range fully inside `startByte..endByte` that splits no character,
// the same clamping as Rust's `source_range`.
export function sourceRange(
  sourceBytes: Uint8Array,
  startByte: number,
  endByte: number,
): HighlightRange {
  let start = Math.min(Math.max(startByte, 0), sourceBytes.length);
  while (start < sourceBytes.length && isUtf8Continuation(sourceBytes[start])) start += 1;

  let end = Math.min(Math.max(endByte, 0), sourceBytes.length);
  while (end > 0 && isUtf8Continuation(sourceBytes[end])) end -= 1;

  return { start, end: Math.max(start, end) };
}

function isUtf8Continuation(byte: number | undefined): boolean {
  return byte !== undefined && (byte & 0xc0) === 0x80;
}

function emptySpan(scope: string, language: string): HighlightSpan {
  return {
    startByte: 0,
    endByte: 0,
    scope,
    language,
  };
}

/** What a formatter does with each step of a line-decorated stream. */
export interface LineRenderOptions<T = unknown> {
  formatText?: (text: string) => string;
  openSpan: (span: HighlightSpan, style: HighlightStyle | undefined) => string;
  closeSpan?: (span: HighlightSpan, style: HighlightStyle | undefined) => string;
  /**
   * Replace a `plaintext` starting language with the language of the first scope
   * that holds text. Rust's line walks keep the starting language for rainbow
   * brackets, and so does every walk here that mirrors one; only
   * `formatHighlightIterLines`, which reports the language it resolved, infers.
   */
  inferLanguage?: boolean;
  /** The style a span opens with, handed to `openSpan` and `closeSpan`. */
  spanStyle?: (scope: string, language: string) => HighlightStyle | undefined;
  /** Sees each composed event before the walk renders it. */
  onEvent?: (event: HighlightEvent<T>) => void;
  /** Receives each run of line content with its range and the innermost open span. */
  onText?: LineRenderContext["onText"];
}

interface LineRenderState {
  /** The current line's rendered content, without its terminator. */
  line: string;
  /** The exact source terminator held until every syntax span has closed. */
  ending: string;
  decoration: Extract<Decoration, { type: "line" }>;
  /** Lumis-owned layers, used to distinguish line ends from rainbow ends. */
  decorations: Decoration[];
  /** The document's language: the innermost scope's, once one has been open. */
  language: string;
  /** The close tag of each open scope, empty when the formatter omitted it. */
  openScopes: Array<{ close: string; scope: string; language: string }>;
  /**
   * A line boundary reopens every span still open, so resolving a scope's tags
   * once is the difference between paying per scope and paying per scope per
   * line.
   */
  tags: Map<string, { open: string; close: string }>;
}

interface LineRenderContext {
  sourceBytes: Uint8Array;
  spanStyle: (scope: string, language: string) => HighlightStyle | undefined;
  formatText: (text: string) => string;
  openSpan: (span: HighlightSpan, style: HighlightStyle | undefined) => string;
  closeSpan: (span: HighlightSpan, style: HighlightStyle | undefined) => string;
  inferLanguage: boolean;
  onLine: (
    content: string,
    decoration: Extract<Decoration, { type: "line" }>,
    ending: string,
  ) => void;
  onText?: (
    content: string,
    range: HighlightRange,
    span: { scope: string; language: string } | undefined,
  ) => void;
}

function openSpanEvent(
  state: LineRenderState,
  context: LineRenderContext,
  event: { scope: string; language: string },
): void {
  const key = `${event.language} ${event.scope}`;
  let tags = state.tags.get(key);

  if (tags === undefined) {
    const span = emptySpan(event.scope, event.language);
    const style = context.spanStyle(event.scope, event.language);
    const open = context.openSpan(span, style);
    // An `openSpan` that returns nothing means the formatter is deliberately
    // omitting the span, so nothing closes it either.
    tags = { open, close: open.length > 0 ? context.closeSpan(span, style) : "" };
    state.tags.set(key, tags);
  }

  state.line += tags.open;
  state.openScopes.push({ close: tags.close, scope: event.scope, language: event.language });
}

function sourceLineEnding(text: string, followedByLf: boolean): string {
  if (text.endsWith("\n")) return text.endsWith("\r\n") ? "\r\n" : "\n";
  return followedByLf && text.endsWith("\r") ? "\r" : "";
}

function sourceEvent(
  state: LineRenderState,
  context: LineRenderContext,
  event: { start: number; end: number },
): void {
  if (context.inferLanguage && (!state.language || state.language === "plaintext")) {
    state.language = state.openScopes.at(-1)?.language ?? state.language;
  }

  const range = sourceRange(context.sourceBytes, event.start, event.end);
  const text = _decoder.decode(context.sourceBytes.subarray(range.start, range.end));
  const ending = sourceLineEnding(text, context.sourceBytes[event.end] === 10);
  const content = ending ? text.slice(0, -ending.length) : text;
  state.ending += ending;
  state.line += context.formatText(content);
  context.onText?.(
    content,
    { start: range.start, end: range.end - ending.length },
    state.openScopes.at(-1),
  );
}

function startLineDecoration(
  state: LineRenderState,
  context: LineRenderContext,
  decoration: Decoration,
): void {
  state.decorations.push(decoration);
  if (decoration.type === "line") {
    state.decoration = decoration;
    state.line = "";
    state.ending = "";
    return;
  }
  openSpanEvent(state, context, {
    scope: rainbowBracketScope(decoration.depth),
    language: state.language,
  });
}

function endLineDecoration(state: LineRenderState, context: LineRenderContext): void {
  const decoration = state.decorations.pop();
  if (decoration?.type === "line") {
    context.onLine(state.line, state.decoration, state.ending);
  } else if (decoration?.type === "rainbowBracket") {
    state.line += state.openScopes.pop()?.close ?? "";
  }
}

function applyLineEvent(
  state: LineRenderState,
  context: LineRenderContext,
  event: HighlightEvent,
): void {
  switch (event.type) {
    case "decorationStart":
      startLineDecoration(state, context, event.decoration);
      break;
    case "decorationEnd":
      endLineDecoration(state, context);
      break;
    case "start":
      openSpanEvent(state, context, event);
      break;
    case "end":
      state.line += state.openScopes.pop()?.close ?? "";
      break;
    case "source":
      sourceEvent(state, context, event);
      break;
    // Caller annotations carry data a built-in formatter has never seen.
    default:
      break;
  }
}

/**
 * Walk a line-decorated stream, handing each line's rendered content to `onLine`.
 *
 * Every span still open at a line boundary was closed and reopened by
 * {@link composeLineDecorations}, so this only has to render what it is given.
 *
 * Returns the document's language.
 */
export function renderDecoratedLines<T>(
  sourceBytes: Uint8Array,
  composed: readonly HighlightEvent<T>[],
  language: string,
  options: LineRenderOptions<T>,
  onLine: LineRenderContext["onLine"],
): string {
  const context: LineRenderContext = {
    sourceBytes,
    spanStyle: options.spanStyle ?? (() => undefined),
    formatText: options.formatText ?? ((text) => text),
    openSpan: options.openSpan,
    closeSpan: options.closeSpan ?? (() => "</span>"),
    inferLanguage: options.inferLanguage ?? false,
    onLine,
    onText: options.onText,
  };
  const state: LineRenderState = {
    line: "",
    ending: "",
    decoration: { type: "line", number: 1, highlighted: false },
    decorations: [],
    language,
    openScopes: [],
    tags: new Map(),
  };

  for (const event of composed) {
    options.onEvent?.(event);
    applyLineEvent(state, context, event);
  }

  return state.language;
}

// The language of a stream's first scope, which is also what Rust's
// `render_lines_from_events` hands a rainbow bracket that opens before any text.
export function streamLanguage(events: readonly HighlightEvent[]): string {
  return events.find((event) => event.type === "start")?.language ?? "plaintext";
}

/** One line of {@link linesFromEvents}. */
export interface Line<T = unknown> {
  /** The 1-based line number. */
  number: number;
  /** The line's text in source order, without its terminator. A blank line has none. */
  tokens: Token[];
  /** The data of every annotation covering any part of the line, once each, in the order they open. */
  annotations: T[];
}

/** A run of one line's text under one scope. */
export interface Token {
  /** The text, which never holds a line terminator. */
  text: string;
  /** Where `text` sits in the source, in UTF-8 bytes. */
  range: HighlightRange;
  /** The innermost open scope, or `""` outside every scope and for a scope Lumis does not name. */
  scope: string;
  /**
   * The innermost open scope's language. A rainbow bracket, and text outside
   * every scope, report the language of the stream's first scope, or
   * `plaintext` when it has none.
   */
  language: string;
}

const HIGHLIGHT_NAME_SET: ReadonlySet<string> = new Set(HIGHLIGHT_NAMES);

/**
 * Split highlight events into lines of tokens, with the annotations each line touches.
 *
 * These are the lines {@link renderLinesFromEvents} renders, as data, for
 * output that is not an HTML string: React elements, SVG, canvas. Content only,
 * a final newline adds no line, and an empty source is one empty line. A scope
 * that crosses a newline gives one token on each line, and a token's `range`
 * leaves the terminator out.
 *
 * Nothing here resolves a style. Look one up with
 * `getScopedThemeStyle(theme, token.scope, token.language)`, which falls back
 * to the parent scopes the built-in formatters fall back to;
 * `theme.highlights[token.scope]` misses those.
 *
 * A line lists the data of every annotation covering any part of it, once
 * each, in the order they open. A point annotation lands on the line holding
 * it, including a blank one, and a point at the very end of a source that ends
 * in a newline lands on the last line.
 *
 * ```ts
 * const formatter: Formatter = {
 *   language: rust,
 *   render(source, events) {
 *     return linesFromEvents(source, events)
 *       .map((line) => `${line.number}: ${line.tokens.map((token) => token.text).join("")}`)
 *       .join("\n")
 *   },
 * }
 * ```
 */
export function linesFromEvents<T = unknown>(
  source: string,
  events: readonly HighlightEvent<T>[],
): Line<T>[] {
  const sourceBytes = _encoder.encode(source);
  const language = streamLanguage(events);
  const lastLine = source.split("\n").length - Number(source.endsWith("\n"));
  const lines: Line<T>[] = [];
  const annotations = new LineAnnotations();

  renderDecoratedLines(
    sourceBytes,
    composeLineDecorations(sourceBytes, events, new LineSelection(undefined)),
    language,
    {
      openSpan: () => "",
      formatText: () => "",
      onEvent: (event) => {
        if (event.type === "decorationStart" && event.decoration.type === "line") {
          // The empty line after a final newline is not returned, so whatever
          // lands on it belongs to the last line.
          if (event.decoration.number > lastLine) return;
          annotations.newLine();
          lines.push({ number: event.decoration.number, tokens: [], annotations: [] });
        } else if (event.type === "annotationStart") {
          if (annotations.opens(event.range)) lines.at(-1)?.annotations.push(event.data);
        } else if (event.type === "annotationEnd") {
          annotations.close();
        }
      },
      onText: (text, range, span) => {
        if (text.length === 0) return;
        lines.at(-1)?.tokens.push({
          text,
          range,
          scope: span && HIGHLIGHT_NAME_SET.has(span.scope) ? span.scope : "",
          language: span?.language ?? language,
        });
      },
    },
    () => {},
  );

  return lines;
}

/**
 * Which annotations one line lists already.
 *
 * Composition closes and reopens an annotation at every line boundary and
 * around every annotation that opens inside it, but never opens one twice at
 * once. So two annotations sharing a range are always open together, and the
 * n-th open one with a range is the n-th distinct one. A point never reopens,
 * so each one is new. That keys on the range, the same in every runtime,
 * without asking whether two annotations' data are equal.
 */
class LineAnnotations {
  #open: HighlightRange[] = [];
  #listed = new Map<string, number>();

  newLine(): void {
    this.#listed.clear();
  }

  /** Records an annotation opening, and reports whether the line should list it. */
  opens(range: HighlightRange): boolean {
    this.#open.push(range);
    const key = `${range.start}:${range.end}`;
    const listed = this.#listed.get(key) ?? 0;
    const open = this.#open.filter(
      (candidate) => candidate.start === range.start && candidate.end === range.end,
    ).length;
    if (range.start !== range.end && open <= listed) return false;
    this.#listed.set(key, listed + 1);
    return true;
  }

  close(): void {
    this.#open.pop();
  }
}

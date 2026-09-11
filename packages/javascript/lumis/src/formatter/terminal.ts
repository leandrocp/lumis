import type { HighlightEvent, HighlightStyle, TerminalFormatter, Theme } from "../types.js";
import { LineSelection, composeLineDecorations } from "../decorations.js";
import { encodeSource, decodeSourceSlice, getScopedThemeStyle, getThemeStyle } from "./html.js";
import { paint } from "./ansi-core.js";

function fallbackBackground(formatter: TerminalFormatter): string | undefined {
  const background = formatter.background;
  if (background === undefined) return undefined;
  if (background === "theme") return getThemeStyle(formatter.theme, "normal")?.bg;
  return background;
}

/**
 * The background a highlighted line is painted with, if any.
 *
 * Only the background is taken from the theme. A foreground would overwrite the
 * colour every token on the line was already given.
 */
function highlightBackground(formatter: TerminalFormatter): string | undefined {
  const highlight = formatter.highlightLines;
  if (!highlight) return undefined;
  return highlight.background ?? getThemeStyle(formatter.theme, "highlighted")?.bg;
}

function paintWithBackground(
  text: string,
  style: HighlightStyle | undefined,
  fallbackBg: string | undefined,
): string {
  if (style && fallbackBg && style.bg === undefined) {
    return paint(text, { ...style, bg: fallbackBg });
  }
  if (style) return paint(text, style);
  if (fallbackBg) return paint(text, { bg: fallbackBg });
  return text;
}

function displayWidth(text: string): number {
  let width = 0;
  for (const char of text) {
    width += char === "\t" ? 4 : 1;
  }
  return width;
}

function linePadding(
  fallbackBg: string | undefined,
  width: number | undefined,
  lineWidth: number,
): string {
  if (fallbackBg === undefined || width === undefined || lineWidth >= width) {
    return "";
  }

  return paintWithBackground(" ".repeat(width - lineWidth), undefined, fallbackBg);
}

/** Split keeping the newline on the segment it ended, like Rust's `split_inclusive`. */
function splitInclusive(text: string): string[] {
  const segments = text.split("\n");
  const last = segments.pop() ?? "";
  const result = segments.map((segment) => `${segment}\n`);
  if (last !== "") result.push(last);
  return result;
}

function activeStyle(
  scopeStack: Array<{ scope: string; language: string }>,
  theme: Theme | undefined,
): HighlightStyle | undefined {
  const active = scopeStack.at(-1);
  if (!active || active.scope.length === 0) return undefined;
  return getScopedThemeStyle(theme, active.scope, active.language);
}

// One segment of a source event, which is either a run of text or that run and
// the newline that ends it. A newline pads the line out to the formatter's width
// so the background reaches the edge, and resets the width count.
function paintSegment(
  segment: string,
  style: HighlightStyle | undefined,
  fallbackBg: string,
  width: number | undefined,
  lineWidth: number,
): { output: string; lineWidth: number } {
  const hasNewline = segment.endsWith("\n");
  const content = hasNewline ? segment.slice(0, -1) : segment;

  let output = "";
  let nextLineWidth = lineWidth;

  if (content !== "") {
    output += paintWithBackground(content, style, fallbackBg);
    nextLineWidth += displayWidth(content);
  }

  if (hasNewline) {
    output += linePadding(fallbackBg, width, nextLineWidth);
    output += "\n";
    nextLineWidth = 0;
  }

  return { output, lineWidth: nextLineWidth };
}

/**
 * Paint one source event, padding each line it ends out to the width.
 *
 * Without a background there is nothing to pad out to, so the text goes out in
 * one piece and the width is never needed.
 */
function paintSource(
  text: string,
  style: HighlightStyle | undefined,
  lineBg: string | undefined,
  width: number | undefined,
  lineWidth: number,
): { output: string; lineWidth: number } {
  if (lineBg === undefined) {
    return { output: paintWithBackground(text, style, undefined), lineWidth };
  }

  let output = "";
  let nextLineWidth = lineWidth;

  for (const segment of splitInclusive(text)) {
    const painted = paintSegment(segment, style, lineBg, width, nextLineWidth);
    output += painted.output;
    nextLineWidth = painted.lineWidth;
  }

  return { output, lineWidth: nextLineWidth };
}

/** What the walk carries from one event to the next. */
interface TerminalState {
  output: string;
  scopeStack: Array<{ scope: string; language: string }>;
  /** The background the current line is painted with. */
  lineBg: string | undefined;
  lineWidth: number;
}

function applyTerminalEvent(
  state: TerminalState,
  formatter: TerminalFormatter,
  sourceBytes: Uint8Array,
  backgrounds: { fallback: string | undefined; highlight: string | undefined },
  event: HighlightEvent,
): void {
  switch (event.type) {
    case "start":
      state.scopeStack.push({ scope: event.scope, language: event.language });
      break;
    case "end":
      state.scopeStack.pop();
      break;
    case "decorationStart":
      state.lineBg = event.decoration.highlighted
        ? (backgrounds.highlight ?? backgrounds.fallback)
        : backgrounds.fallback;
      state.lineWidth = 0;
      break;
    case "source": {
      const painted = paintSource(
        decodeSourceSlice(sourceBytes, event.start, event.end),
        activeStyle(state.scopeStack, formatter.theme),
        state.lineBg,
        formatter.width,
        state.lineWidth,
      );
      state.output += painted.output;
      state.lineWidth = painted.lineWidth;
      break;
    }
    // Caller annotations carry data this formatter has never seen.
    default:
      break;
  }
}

export function formatTerminal(
  source: string,
  events: readonly HighlightEvent[],
  formatter: TerminalFormatter,
): string {
  const sourceBytes = encodeSource(source);
  const backgrounds = {
    fallback: fallbackBackground(formatter),
    highlight: highlightBackground(formatter),
  };

  // A terminal writes one line after another whether or not it is told which
  // lines to mark, so the line decorations are only worth composing when there
  // is something to mark. Without them the stream, and the output, are exactly
  // what they were.
  const selection = new LineSelection(formatter.highlightLines?.lines);
  const decorated = selection.isEmpty
    ? events
    : composeLineDecorations(sourceBytes, events, selection);

  const state: TerminalState = {
    output: "",
    scopeStack: [],
    lineBg: backgrounds.fallback,
    lineWidth: 0,
  };

  for (const event of decorated) {
    applyTerminalEvent(state, formatter, sourceBytes, backgrounds, event);
  }

  if (!source.endsWith("\n")) {
    state.output += linePadding(state.lineBg, formatter.width, state.lineWidth);
  }

  return state.output;
}

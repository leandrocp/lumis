import type { HighlightEvent, HighlightStyle, TerminalFormatter, Theme } from "../types.js";
import {
  LineSelection,
  composeLineDecorations,
  gutterWidth,
  lastLineNumber,
} from "../decorations.js";
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

/** The Neovim number-column style for this line. */
function gutterStyle(theme: Theme | undefined, highlighted: boolean): HighlightStyle | undefined {
  return getThemeStyle(theme, highlighted ? "line_number.highlighted" : "line_number");
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
  /**
   * The current line's number, until its first text is written.
   *
   * A terminal writes nothing at all for a line with no text. The last line of a
   * source ending in a newline is one, and numbering it would leave a bare
   * number after the output.
   */
  pendingNumber: { number: number; highlighted: boolean } | undefined;
}

/** How a line's number is written, once the widest one is known. */
interface Gutter {
  width: number;
  style: HighlightStyle | undefined;
  highlightedStyle: HighlightStyle | undefined;
}

/** Write one source event, opening the line with its number if it has not been. */
function writeSource(
  state: TerminalState,
  formatter: TerminalFormatter,
  gutter: Gutter | undefined,
  fallbackBg: string | undefined,
  text: string,
): void {
  if (gutter && state.pendingNumber !== undefined && text !== "") {
    const pending = state.pendingNumber;
    const written = `${String(pending.number).padStart(gutter.width, " ")} `;
    // Neovim draws the number column with `CursorLineNr` alone, so `CursorLine`
    // does not reach it: a highlighted line's background starts at its text.
    const style = pending.highlighted ? gutter.highlightedStyle : gutter.style;
    state.output += paintWithBackground(written, style, fallbackBg);
    state.lineWidth = written.length;
    state.pendingNumber = undefined;
  }

  const painted = paintSource(
    text,
    activeStyle(state.scopeStack, formatter.theme),
    state.lineBg,
    formatter.width,
    state.lineWidth,
  );
  state.output += painted.output;
  state.lineWidth = painted.lineWidth;
}

function applyTerminalEvent(
  state: TerminalState,
  formatter: TerminalFormatter,
  sourceBytes: Uint8Array,
  backgrounds: { fallback: string | undefined; highlight: string | undefined },
  gutter: Gutter | undefined,
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
      state.pendingNumber = gutter
        ? { number: event.decoration.number, highlighted: event.decoration.highlighted }
        : undefined;
      break;
    case "source":
      writeSource(
        state,
        formatter,
        gutter,
        backgrounds.fallback,
        decodeSourceSlice(sourceBytes, event.start, event.end),
      );
      break;
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
  // lines to mark or number, so the line decorations are only worth composing
  // when one of the two was asked for. Without them the stream, and the output,
  // are exactly what they were.
  const selection = new LineSelection(formatter.highlightLines?.lines);
  const numbered = formatter.lineNumbers === true;
  const decorated =
    selection.isEmpty && !numbered
      ? events
      : composeLineDecorations(sourceBytes, events, selection);

  // The gutter is padded to the widest number it will show, which is only known
  // once the lines are.
  const gutter: Gutter | undefined = numbered
    ? {
        width: gutterWidth(lastLineNumber(decorated)),
        style: gutterStyle(formatter.theme, false),
        highlightedStyle: gutterStyle(formatter.theme, true),
      }
    : undefined;

  const state: TerminalState = {
    output: "",
    scopeStack: [],
    lineBg: backgrounds.fallback,
    lineWidth: 0,
    pendingNumber: undefined,
  };

  for (const event of decorated) {
    applyTerminalEvent(state, formatter, sourceBytes, backgrounds, gutter, event);
  }

  if (!source.endsWith("\n")) {
    state.output += linePadding(state.lineBg, formatter.width, state.lineWidth);
  }

  return state.output;
}

import type { HighlightEvent, HighlightStyle, TerminalFormatter, Theme } from "../types.js";
import { encodeSource, decodeSourceSlice, getScopedThemeStyle, getThemeStyle } from "./html.js";
import { paint } from "./ansi-core.js";

function fallbackBackground(formatter: TerminalFormatter): string | undefined {
  const background = formatter.background;
  if (background === undefined) return undefined;
  if (background === "theme") return getThemeStyle(formatter.theme, "normal")?.bg;
  return background;
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

export function formatTerminal(
  source: string,
  events: readonly HighlightEvent[],
  formatter: TerminalFormatter,
): string {
  let output = "";
  const sourceBytes = encodeSource(source);
  const scopeStack: Array<{ scope: string; language: string }> = [];
  const fallbackBg = fallbackBackground(formatter);
  let lineWidth = 0;

  for (const event of events) {
    if (event.type === "start") {
      scopeStack.push({ scope: event.scope, language: event.language });
      continue;
    }

    if (event.type === "end") {
      scopeStack.pop();
      continue;
    }

    if (event.type === "annotationStart" || event.type === "annotationEnd") {
      continue;
    }

    const text = decodeSourceSlice(sourceBytes, event.startByte, event.endByte);
    const style = activeStyle(scopeStack, formatter.theme);

    if (fallbackBg === undefined) {
      output += paintWithBackground(text, style, undefined);
      continue;
    }

    for (const segment of splitInclusive(text)) {
      const painted = paintSegment(segment, style, fallbackBg, formatter.width, lineWidth);
      output += painted.output;
      lineWidth = painted.lineWidth;
    }
  }

  if (!source.endsWith("\n")) {
    output += linePadding(fallbackBg, formatter.width, lineWidth);
  }

  return output;
}

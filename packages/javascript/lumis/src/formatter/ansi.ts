import type { HighlightRange, LanguageRef, Theme } from "../types.js";
import { paint } from "./ansi-core.js";

export { ANSI_RESET, hexToRgb, paint, rgbToAnsi, styleToAnsi, wrapWithAnsi } from "./ansi-core.js";

/** A rendered ANSI segment paired with its byte range. */
export type AnsiSegment = [string, HighlightRange];

/**
 * Highlight source and collect ANSI-wrapped segments.
 *
 * ```ts
 * const segments = []
 * highlightIter(source, language, theme, (text, _language, range, _scope, style) => {
 *   segments.push([paint(text, style), range])
 * })
 * // [["\x1b[0m...const\x1b[0m", { start: 0, end: 5 }], ...]
 * ```
 *
 * @deprecated Use `highlightIter()` with `paint()` instead.
 */
export async function highlightIterWithAnsi(
  source: string,
  language: LanguageRef | undefined,
  theme: Theme | undefined,
): Promise<AnsiSegment[]> {
  const { highlight, highlightIter } = await import("../index.js");
  const segments: AnsiSegment[] = [];

  await highlight(source, {
    language,
    render(src) {
      highlightIter(src, this.language, theme, (text, _language, range, _scope, style) => {
        segments.push([paint(text, style), range]);
      });
      return "";
    },
  });

  return segments;
}

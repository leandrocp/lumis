import type {
  HighlightEvent,
  HighlightSpan,
  HighlightStyle,
  HtmlStructure,
  LanguageRef,
  Theme,
} from "../types.js";
import { formatHtmlLines } from "./html.js";

/**
 * Whether a formatter's `structure` asks for the token spans alone.
 *
 * Any other value is refused rather than read as a block, so the JavaScript
 * formatters and the native addon, which cannot parse one, fail alike.
 */
export function isInlineStructure(structure: HtmlStructure | undefined): boolean {
  switch (structure) {
    case undefined:
    case "block":
      return false;
    case "inline":
      return true;
    default:
      throw new TypeError(
        `structure must be "block" or "inline", got ${JSON.stringify(structure)}`,
      );
  }
}

/**
 * What a formatter writes for the `"inline"` structure: the lines of a block
 * without the elements around them.
 */
export function formatHtmlSpans(
  source: string,
  events: readonly HighlightEvent[],
  formatter: {
    language: LanguageRef | undefined;
    theme: Theme | undefined;
    openSpan: (span: HighlightSpan, style: HighlightStyle | undefined) => string;
  },
): string {
  return formatHtmlLines(source, events, {
    ...formatter,
    structure: "inline",
    lines: undefined,
    lineNumbers: false,
    lineNumberAttrs: { regular: {}, highlighted: {} },
    highlightedAttrs: {},
  });
}

import type { HighlightEvent, HtmlLinkedFormatter } from "../types.js";
import {
  closingTags,
  formatHighlightIterLines,
  getHighlightLineClass,
  openCodeTag,
  openPreTag,
  openSpanTag,
  scopeToClass,
  wrapLine,
  wrapWithHeader,
} from "./html.js";

function highlightLineClass(
  formatter: HtmlLinkedFormatter,
  lineNumber: number,
): string | undefined {
  return getHighlightLineClass(
    formatter.highlightLines?.lines,
    lineNumber,
    formatter.highlightLines?.class,
    "l-highlighted",
  );
}

function getLineAttrs(formatter: HtmlLinkedFormatter, lineNumber: number): { className?: string } {
  return {
    className: highlightLineClass(formatter, lineNumber),
  };
}

export function formatHtmlLinked(
  source: string,
  events: readonly HighlightEvent[],
  formatter: HtmlLinkedFormatter,
): string {
  const { lines } = formatHighlightIterLines(source, events, formatter.language, undefined, {
    openSpan: (span) => openSpanTag({ class: scopeToClass(span.scope) }),
  });

  const pre = openPreTag({ preClass: formatter.preClass });
  const code = openCodeTag(formatter.language);
  const body = lines
    .map((line, idx) => wrapLine(idx + 1, line, getLineAttrs(formatter, idx + 1)))
    .join("");

  return wrapWithHeader(`${pre}${code}${body}${closingTags()}`, formatter.header);
}

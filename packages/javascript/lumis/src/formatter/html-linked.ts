import type { HighlightEvent, HtmlLinkedFormatter } from "../types.js";
import {
  closingTags,
  formatHighlightIterLines,
  openCodeTag,
  openPreTag,
  openSpanTag,
  scopeToClass,
  wrapLine,
  wrapWithHeader,
} from "./html.js";
import { selectedLineFlags } from "./line-highlights.js";

function highlightLineClass(
  formatter: HtmlLinkedFormatter,
  isHighlighted: boolean,
): string | undefined {
  return isHighlighted ? (formatter.highlightLines?.class ?? "l-highlighted") : undefined;
}

function getLineAttrs(
  formatter: HtmlLinkedFormatter,
  isHighlighted: boolean,
): { className?: string } {
  return {
    className: highlightLineClass(formatter, isHighlighted),
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
  const highlighted = selectedLineFlags(formatter.highlightLines?.lines, lines.length);
  const body = lines
    .map((line, idx) =>
      wrapLine(idx + 1, line, getLineAttrs(formatter, Boolean(highlighted?.[idx]))),
    )
    .join("");

  return wrapWithHeader(`${pre}${code}${body}${closingTags()}`, formatter.header);
}

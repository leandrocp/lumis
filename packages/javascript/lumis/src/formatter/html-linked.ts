import type { HighlightEvent, HtmlLinkedFormatter } from "../types.js";
import {
  closingTags,
  formatHtmlLines,
  openCodeTag,
  openPreTag,
  openSpanTag,
  scopeToClass,
  wrapWithHeader,
} from "./html.js";

export function formatHtmlLinked(
  source: string,
  events: readonly HighlightEvent[],
  formatter: HtmlLinkedFormatter,
): string {
  const body = formatHtmlLines(source, events, {
    language: formatter.language,
    theme: undefined,
    lines: formatter.highlightLines?.lines,
    lineNumbers: formatter.lineNumbers,
    lineNumberAttrs: { regular: {}, highlighted: {} },
    highlightedAttrs: { className: formatter.highlightLines?.class ?? "l-highlighted" },
    openSpan: (span) => openSpanTag({ class: scopeToClass(span.scope) }),
  });

  const pre = openPreTag({ preClass: formatter.preClass, attrs: formatter.preAttrs });
  const code = openCodeTag(formatter.language, formatter.codeAttrs);

  return wrapWithHeader(`${pre}${code}${body}${closingTags()}`, formatter.header);
}

import type {
  BudgetExhausted,
  HighlightEvent,
  HighlightSpan,
  HtmlLinkedFormatter,
} from "../types.js";
import {
  budgetAttrs,
  closingTags,
  formatHtmlLines,
  openCodeTag,
  openPreTag,
  openSpanTag,
  scopeToClass,
  wrapWithHeader,
} from "./html.js";
import { formatHtmlSpans, isInlineStructure } from "./html-structure.js";

export function formatHtmlLinked(
  source: string,
  events: readonly HighlightEvent[],
  formatter: HtmlLinkedFormatter,
  budget?: BudgetExhausted,
): string {
  const openSpan = (span: HighlightSpan): string =>
    openSpanTag({ class: scopeToClass(span.scope) });
  if (isInlineStructure(formatter.structure)) {
    return formatHtmlSpans(source, events, {
      language: formatter.language,
      theme: undefined,
      openSpan,
    });
  }

  const body = formatHtmlLines(source, events, {
    language: formatter.language,
    theme: undefined,
    lines: formatter.highlightLines?.lines,
    lineNumbers: formatter.lineNumbers,
    lineNumberAttrs: { regular: {}, highlighted: {} },
    highlightedAttrs: { className: formatter.highlightLines?.class ?? "l-highlighted" },
    openSpan,
  });

  const pre = openPreTag({
    preClass: formatter.preClass,
    attrs: budgetAttrs(formatter.preAttrs, budget),
  });
  const code = openCodeTag(formatter.language, formatter.codeAttrs);

  return wrapWithHeader(`${pre}${code}${body}${closingTags()}`, formatter.header);
}

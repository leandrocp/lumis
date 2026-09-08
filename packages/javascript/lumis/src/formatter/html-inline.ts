import type { HighlightEvent, HighlightSpan, HtmlInlineFormatter } from "../types.js";
import {
  closingTags,
  formatHighlightIterLines,
  getHighlightLineClass,
  getScopedThemeStyle,
  getThemeStyle,
  lineIsHighlighted,
  openCodeTag,
  openPreTag,
  openSpanTag,
  styleToCss,
  wrapLine,
  wrapWithHeader,
} from "./html.js";

function spanAttrs(
  span: HighlightSpan,
  formatter: HtmlInlineFormatter,
): Record<string, string | undefined> {
  const attrs: Record<string, string | undefined> = {};

  if (formatter.includeHighlights) {
    attrs["data-highlight"] = span.scope;
  }

  const css = styleToCss(getScopedThemeStyle(formatter.theme, span.scope, span.language), {
    italic: formatter.italic,
  });
  if (css) {
    attrs.style = css;
  }

  return attrs;
}

function highlightLineStyle(
  formatter: HtmlInlineFormatter,
  lineNumber: number,
): string | undefined {
  const highlightLines = formatter.highlightLines;
  if (!lineIsHighlighted(highlightLines?.lines, lineNumber)) {
    return undefined;
  }

  // Explicit `null` opts out of the inline style entirely, leaving the class to
  // do the highlighting. Absent still means the theme's `highlighted` style.
  if (highlightLines?.style === null) {
    return undefined;
  }

  if (highlightLines?.style && highlightLines.style !== "theme") {
    return highlightLines.style;
  }

  const style = getThemeStyle(formatter.theme, "highlighted");
  return styleToCss(style, { italic: formatter.italic }) || undefined;
}

function highlightLineClass(
  formatter: HtmlInlineFormatter,
  lineNumber: number,
): string | undefined {
  return getHighlightLineClass(
    formatter.highlightLines?.lines,
    lineNumber,
    formatter.highlightLines?.class,
  );
}

function getLineAttrs(
  formatter: HtmlInlineFormatter,
  lineNumber: number,
): { className?: string; style?: string } {
  return {
    className: highlightLineClass(formatter, lineNumber),
    style: highlightLineStyle(formatter, lineNumber),
  };
}

export function formatHtmlInline(
  source: string,
  events: readonly HighlightEvent[],
  formatter: HtmlInlineFormatter,
): string {
  const { lines } = formatHighlightIterLines(source, events, formatter.language, formatter.theme, {
    openSpan: (span) => openSpanTag(spanAttrs(span, formatter)),
  });

  const pre = openPreTag({ preClass: formatter.preClass, theme: formatter.theme });
  const code = openCodeTag(formatter.language);
  const body = lines
    .map((line, idx) => wrapLine(idx + 1, line, getLineAttrs(formatter, idx + 1)))
    .join("");

  return wrapWithHeader(`${pre}${code}${body}${closingTags()}`, formatter.header);
}

import type { HighlightEvent, HighlightSpan, HtmlInlineFormatter } from "../types.js";
import {
  closingTags,
  formatHtmlLines,
  getScopedThemeStyle,
  getThemeStyle,
  openCodeTag,
  openPreTag,
  openSpanTag,
  styleToCss,
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

function highlightLineStyle(formatter: HtmlInlineFormatter): string | undefined {
  const highlightLines = formatter.highlightLines;

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

function lineNumberAttrs(
  formatter: HtmlInlineFormatter,
  highlighted: boolean,
): Record<string, string> {
  const scope = highlighted ? "line_number.highlighted" : "line_number";
  const style = styleToCss(getThemeStyle(formatter.theme, scope), { italic: formatter.italic });
  return style === "" ? {} : { style };
}

export function formatHtmlInline(
  source: string,
  events: readonly HighlightEvent[],
  formatter: HtmlInlineFormatter,
): string {
  const body = formatHtmlLines(source, events, {
    language: formatter.language,
    theme: formatter.theme,
    lines: formatter.highlightLines?.lines,
    lineNumbers: formatter.lineNumbers,
    lineNumberAttrs: {
      regular: lineNumberAttrs(formatter, false),
      highlighted: lineNumberAttrs(formatter, true),
    },
    highlightedAttrs: {
      className: formatter.highlightLines?.class,
      style: highlightLineStyle(formatter),
    },
    openSpan: (span) => openSpanTag(spanAttrs(span, formatter)),
  });

  const pre = openPreTag({
    preClass: formatter.preClass,
    theme: formatter.theme,
    attrs: formatter.preAttrs,
  });
  const code = openCodeTag(formatter.language, formatter.codeAttrs);

  return wrapWithHeader(`${pre}${code}${body}${closingTags()}`, formatter.header);
}

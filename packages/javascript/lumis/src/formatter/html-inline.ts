import type { HighlightEvent, HighlightSpan, HtmlInlineFormatter } from "../types.js";
import {
  closingTags,
  formatHighlightIterLines,
  getScopedThemeStyle,
  getThemeStyle,
  openCodeTag,
  openPreTag,
  openSpanTag,
  styleToCss,
  wrapLine,
  wrapWithHeader,
} from "./html.js";
import { selectedLineFlags } from "./line-highlights.js";

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
  isHighlighted: boolean,
): string | undefined {
  const highlightLines = formatter.highlightLines;
  if (!isHighlighted) {
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
  isHighlighted: boolean,
): string | undefined {
  return isHighlighted ? formatter.highlightLines?.class : undefined;
}

function getLineAttrs(
  formatter: HtmlInlineFormatter,
  isHighlighted: boolean,
): { className?: string; style?: string } {
  return {
    className: highlightLineClass(formatter, isHighlighted),
    style: highlightLineStyle(formatter, isHighlighted),
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
  const highlighted = selectedLineFlags(formatter.highlightLines?.lines, lines.length);
  const body = lines
    .map((line, idx) =>
      wrapLine(idx + 1, line, getLineAttrs(formatter, Boolean(highlighted?.[idx]))),
    )
    .join("");

  return wrapWithHeader(`${pre}${code}${body}${closingTags()}`, formatter.header);
}

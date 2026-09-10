import type { HighlightEvent, HighlightSpan, HtmlMultiThemesFormatter } from "../types.js";
import {
  type HtmlAttrs,
  closingTags,
  formatHighlightIterLines,
  getHighlightLineClass,
  getThemeStyle,
  lineIsHighlighted,
  openCodeTag,
  openMultiThemesPreTag,
  openSpanTag,
  spanMultiThemesAttrs,
  styleToCss,
  wrapLine,
  wrapWithHeader,
} from "./html.js";

function spanAttrs(span: HighlightSpan, formatter: HtmlMultiThemesFormatter): HtmlAttrs {
  return spanMultiThemesAttrs({
    language: span.language,
    scope: span.scope,
    themes: formatter.themes,
    defaultTheme: formatter.defaultTheme,
    cssVariablePrefix: formatter.cssVariablePrefix,
    italic: formatter.italic,
    includeHighlights: formatter.includeHighlights,
  });
}

function highlightLineStyle(
  formatter: HtmlMultiThemesFormatter,
  lineNumber: number,
): string | undefined {
  const highlightLines = formatter.highlightLines;
  if (!lineIsHighlighted(highlightLines?.lines, lineNumber)) return undefined;

  // Explicit `null` opts out of the inline style entirely, leaving the class to
  // do the highlighting. Absent still means the theme's `highlighted` style.
  if (highlightLines?.style === null) return undefined;
  if (highlightLines?.style && highlightLines.style !== "theme") return highlightLines.style;

  return themeHighlightStyle(formatter);
}

function themeHighlightStyle(formatter: HtmlMultiThemesFormatter): string | undefined {
  if (!formatter.defaultTheme) return undefined;
  if (formatter.defaultTheme === "light-dark()") return lightDarkHighlightStyle(formatter);

  const style = getThemeStyle(formatter.themes[formatter.defaultTheme], "highlighted");
  return styleToCss(style, { italic: formatter.italic }) || undefined;
}

function lightDarkHighlightStyle(formatter: HtmlMultiThemesFormatter): string | undefined {
  const light = getThemeStyle(formatter.themes.light, "highlighted");
  const dark = getThemeStyle(formatter.themes.dark, "highlighted");
  if (!light?.bg || !dark?.bg) {
    return undefined;
  }

  return `background-color: light-dark(${light.bg}, ${dark.bg});`;
}

function highlightLineClass(
  formatter: HtmlMultiThemesFormatter,
  lineNumber: number,
): string | undefined {
  return getHighlightLineClass(
    formatter.highlightLines?.lines,
    lineNumber,
    formatter.highlightLines?.class,
  );
}

function getLineAttrs(
  formatter: HtmlMultiThemesFormatter,
  lineNumber: number,
): { className?: string; style?: string } {
  return {
    className: highlightLineClass(formatter, lineNumber),
    style: highlightLineStyle(formatter, lineNumber),
  };
}

export function formatHtmlMultiThemes(
  source: string,
  events: readonly HighlightEvent[],
  formatter: HtmlMultiThemesFormatter,
): string {
  const theme = formatter.defaultTheme ? formatter.themes[formatter.defaultTheme] : undefined;
  const { lines } = formatHighlightIterLines(source, events, formatter.language, theme, {
    openSpan: (span, _style) => openSpanTag(spanAttrs(span, formatter)),
  });

  const pre = openMultiThemesPreTag({
    preClass: formatter.preClass,
    themes: formatter.themes,
    defaultTheme: formatter.defaultTheme,
    cssVariablePrefix: formatter.cssVariablePrefix,
  });
  const code = openCodeTag(formatter.language);
  const body = lines
    .map((line, idx) => wrapLine(idx + 1, line, getLineAttrs(formatter, idx + 1)))
    .join("");

  return wrapWithHeader(`${pre}${code}${body}${closingTags()}`, formatter.header);
}

import type { HighlightEvent, HighlightSpan, HtmlMultiThemesFormatter } from "../types.js";
import {
  type HtmlAttrs,
  closingTags,
  formatHighlightIterLines,
  getThemeStyle,
  openCodeTag,
  openMultiThemesPreTag,
  openSpanTag,
  spanMultiThemesAttrs,
  styleToCss,
  wrapLine,
  wrapWithHeader,
} from "./html.js";
import { selectedLineFlags } from "./line-highlights.js";

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
  isHighlighted: boolean,
): string | undefined {
  const highlightLines = formatter.highlightLines;
  if (!isHighlighted) return undefined;

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
  isHighlighted: boolean,
): string | undefined {
  return isHighlighted ? formatter.highlightLines?.class : undefined;
}

function getLineAttrs(
  formatter: HtmlMultiThemesFormatter,
  isHighlighted: boolean,
): { className?: string; style?: string } {
  return {
    className: highlightLineClass(formatter, isHighlighted),
    style: highlightLineStyle(formatter, isHighlighted),
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
  const highlighted = selectedLineFlags(formatter.highlightLines?.lines, lines.length);
  const body = lines
    .map((line, idx) =>
      wrapLine(idx + 1, line, getLineAttrs(formatter, Boolean(highlighted?.[idx]))),
    )
    .join("");

  return wrapWithHeader(`${pre}${code}${body}${closingTags()}`, formatter.header);
}

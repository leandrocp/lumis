import type { HighlightEvent, HighlightSpan, HtmlMultiThemesFormatter, Theme } from "../types.js";
import {
  type HtmlAttrs,
  buildNormalThemeVars,
  closingTags,
  formatHighlightIterLines,
  getHighlightLineClass,
  getThemeStyle,
  joinClasses,
  lineIsHighlighted,
  openCodeTag,
  openSpanTag,
  openTag,
  sortedThemeNames,
  spanMultiThemesAttrs,
  styleToCss,
  wrapLine,
  wrapWithHeader,
} from "./html.js";

// The `<pre>` colours for `light-dark()`, falling back to black on white and
// white on black where a theme leaves them unset.
function lightDarkPreStyles(themes: Record<string, Theme>): string[] {
  const lightNormal = getThemeStyle(themes.light, "normal");
  const darkNormal = getThemeStyle(themes.dark, "normal");
  const lightFg = lightNormal?.fg ?? "#000000";
  const lightBg = lightNormal?.bg ?? "#ffffff";
  const darkFg = darkNormal?.fg ?? "#ffffff";
  const darkBg = darkNormal?.bg ?? "#000000";

  return [
    `color: light-dark(${lightFg}, ${darkFg});`,
    `background-color: light-dark(${lightBg}, ${darkBg});`,
  ];
}

function buildPreThemeStyle(options: {
  themes: Record<string, Theme>;
  defaultTheme?: string;
  cssVariablePrefix?: string;
}): string | undefined {
  const prefix = options.cssVariablePrefix ?? "--lumis";
  const styles: string[] = [];

  if (options.defaultTheme === "light-dark()") {
    styles.push(...lightDarkPreStyles(options.themes));
  } else if (options.defaultTheme) {
    const defaultStyle = getThemeStyle(options.themes[options.defaultTheme], "normal");
    if (defaultStyle?.fg) styles.push(`color:${defaultStyle.fg};`);
    if (defaultStyle?.bg) styles.push(`background-color:${defaultStyle.bg};`);
    buildNormalThemeVars(styles, prefix, options.themes, options.defaultTheme);
  } else {
    buildNormalThemeVars(styles, prefix, options.themes);
  }

  return styles.length > 0 ? styles.join(" ") : undefined;
}

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

function generatePreClasses(formatter: HtmlMultiThemesFormatter): string {
  return (
    joinClasses(
      "lumis",
      "lumis-themes",
      formatter.preClass,
      ...sortedThemeNames(formatter.themes),
    ) ?? "lumis lumis-themes"
  );
}

function generatePreStyle(formatter: HtmlMultiThemesFormatter): string | undefined {
  return buildPreThemeStyle({
    themes: formatter.themes,
    defaultTheme: formatter.defaultTheme,
    cssVariablePrefix: formatter.cssVariablePrefix,
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

  const pre = openTag("pre", {
    class: generatePreClasses(formatter),
    style: generatePreStyle(formatter),
  });
  const code = openCodeTag(formatter.language);
  const body = lines
    .map((line, idx) => wrapLine(idx + 1, line, getLineAttrs(formatter, idx + 1)))
    .join("");

  return wrapWithHeader(`${pre}${code}${body}${closingTags()}`, formatter.header);
}

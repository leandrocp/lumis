import type { StyleEntry, ThemeData } from "./types.js";

/** Options for {@link buildCss}. */
export interface BuildCssOptions {
  /** Whether italic theme styles should be emitted. Defaults to `true`. */
  enableItalic?: boolean;
  /** Parent selector prepended to every generated selector. Defaults to `""`. */
  scope?: string;
  /** Selector used for the container code block rule. Defaults to `".lumis"`. */
  containerSelector?: string;
  /**
   * Extra `[property, value]` declarations for the container code block rule. A property that matches
   * one the theme already sets (`color`, `background-color`) replaces that value instead of duplicating it.
   */
  containerStyle?: [string, string][];
}

/**
 * Build a CSS stylesheet for a theme.
 *
 * Most applications can use the bundled CSS files from `@lumis-sh/themes/css/*`,
 * but this is useful when CSS needs to be embedded, scoped, or customized, for
 * example to scope a dark theme under a `data-theme` selector.
 *
 * ```ts
 * import { buildCss } from '@lumis-sh/themes'
 * import githubDark from '@lumis-sh/themes/github_dark'
 *
 * const css = buildCss(githubDark, {
 *   scope: 'html[data-theme="dark"]',
 *   containerSelector: '.lumis',
 *   containerStyle: [
 *     ['background-color', 'var(--code-background)'],
 *     ['border-radius', '0.375rem'],
 *     ['padding', '1rem'],
 *   ],
 * })
 * ```
 */
export function buildCss(theme: ThemeData, options: BuildCssOptions = {}): string {
  const enableItalic = options.enableItalic ?? true;
  const scope = options.scope ?? "";
  const containerSelector = options.containerSelector ?? ".lumis";
  const containerStyle = options.containerStyle ?? [];

  const rules: string[] = [
    `/* ${theme.name}\n * revision: ${theme.revision ?? ""}\n */\n${scopedSelector(scope, containerSelector)}`,
  ];

  const normal = theme.highlights["normal"];
  const scopeStyle = renderContainerStyle(normal, containerStyle, "\n  ");

  if (scopeStyle === "") {
    rules.push(" {}\n");
  } else {
    rules.push(` {\n  ${scopeStyle}\n}\n`);
  }

  rules.push(...scopeRules(theme, scope, enableItalic));

  return rules.join("");
}

// One rule per scope that renders to anything. `normal` defines the code block's
// base colors, already emitted above and inherited by all text; it is never
// applied as a token class, so a `.normal` rule would be dead CSS.
function scopeRules(theme: ThemeData, scope: string, enableItalic: boolean): string[] {
  const entries = Object.entries(theme.highlights).sort(([a], [b]) => (a < b ? -1 : a > b ? 1 : 0));
  const rules: string[] = [];

  for (const [scopeName, style] of entries) {
    if (scopeName === "normal") continue;

    const styleCss = renderStyle(style, enableItalic, "\n  ");
    if (styleCss !== "") {
      const className = `l-${scopeName.replaceAll(/[._]/g, "-")}`;
      const selector =
        scopeName === "line_number" && theme.highlights["line_number.highlighted"] !== undefined
          ? `.${className}:not(.l-line-number-highlighted)`
          : `.${className}`;
      rules.push(`${scopePrefix(scope)}${selector} {\n  ${styleCss}\n}\n`);
    }
  }

  return rules;
}

function scopePrefix(scope: string): string {
  return scope === "" ? "" : `${scope} `;
}

function scopedSelector(scope: string, selector: string): string {
  return scope === "" ? selector : `${scope} ${selector}`;
}

function renderContainerStyle(
  normal: StyleEntry | undefined,
  containerStyle: [string, string][],
  separator: string,
): string {
  // Start from the theme's container declarations, then merge `containerStyle` over them: a rule whose
  // property already exists replaces it in place, otherwise it is appended. This keeps a single
  // declaration per property (overriding `background-color` does not duplicate the theme's).
  const decls: [string, string][] = [];

  if (normal?.fg) {
    decls.push(["color", normal.fg]);
  }
  if (normal?.bg) {
    decls.push(["background-color", normal.bg]);
  }

  for (const [property, value] of containerStyle) {
    const existing = decls.find(([p]) => p === property);
    if (existing) {
      existing[1] = value;
    } else {
      decls.push([property, value]);
    }
  }

  return decls.map(([property, value]) => `${property}: ${value};`).join(separator);
}

function renderStyle(style: StyleEntry, enableItalic: boolean, separator: string): string {
  const rules: string[] = [];

  if (style.fg) {
    rules.push(`color: ${style.fg};`);
  }

  if (style.bg) {
    rules.push(`background-color: ${style.bg};`);
  }

  if (style.bold) {
    rules.push("font-weight: bold;");
  }

  if (enableItalic && style.italic) {
    rules.push("font-style: italic;");
  }

  const decoration = textDecorationRule(style);
  if (decoration) rules.push(decoration);

  return rules.join(separator);
}

function textDecorationRule(style: StyleEntry): string | undefined {
  const parts = [
    underlineDecoration(style.underline),
    style.strikethrough && "line-through",
  ].filter(Boolean);

  return parts.length > 0 ? `text-decoration: ${parts.join(" ")};` : undefined;
}

function underlineDecoration(underline: StyleEntry["underline"]): string | undefined {
  switch (underline) {
    case "solid":
      return "underline";
    case "wavy":
    case "undercurl":
      return "underline wavy";
    case "double":
      return "underline double";
    case "dotted":
      return "underline dotted";
    case "dashed":
      return "underline dashed";
    default:
      return undefined;
  }
}

/**
 * Discover and transform themes for highlighting.
 *
 * To generate and load a theme outside the bundled collection, see
 * {@link https://docs.lumis.sh/recipes/generate-theme-from-neovim-colorscheme | Generate a theme from a Neovim colorscheme},
 * including source-license and redistribution considerations.
 *
 * @module
 */
import { THEME_DATA } from "../generated/theme-data.js";
import type { HighlightStyle, Theme } from "../types.js";

export { availableThemes as available } from "../themes.js";
export { closest } from "./closest.js";
export type { TerminalColors } from "./closest.js";
export type { Theme, ThemeInfo, HighlightStyle } from "../types.js";

/**
 * Return a new theme with every existing scope's style transformed once.
 *
 * The callback receives a scope name and a copy of its complete style, in an
 * unspecified order. Its return value replaces that style, rather than merging
 * with it. Scope names and metadata stay unchanged; fallback scopes are not added.
 *
 * ```ts
 * import githubLight from '@lumis-sh/themes/github_light'
 * import { mapStyles } from '@lumis-sh/lumis/themes'
 *
 * const theme = mapStyles(githubLight, (_scope, style) => ({ ...style, italic: false }))
 * ```
 */
export function mapStyles(
  theme: Readonly<Theme>,
  transform: (scope: string, style: Readonly<HighlightStyle>) => HighlightStyle,
): Theme {
  return {
    ...theme,
    highlights: Object.fromEntries(
      Object.entries(theme.highlights).map(([scope, style]) => [
        scope,
        { ...transform(scope, { ...style }) },
      ]),
    ),
  };
}

/** Look up a built-in theme by exact name, or return undefined. */
export function get(name: string): Theme | undefined {
  const theme = THEME_DATA.find((candidate) => candidate.name === name);
  return theme && structuredClone(theme);
}

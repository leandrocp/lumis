import { THEMES } from "./generated/themes-meta.js";
import { cloneThemeInfo } from "./catalog-metadata.js";
import type { ThemeInfo } from "./types.js";

/**
 * List all built-in themes with their name and appearance.
 *
 * ```ts
 * import { availableThemes } from '@lumis-sh/lumis'
 * const themes = availableThemes()
 * // [{ name: 'dracula', appearance: 'dark' }, ...]
 * ```
 */
export function availableThemes(): ThemeInfo[] {
  return THEMES.map((theme) => cloneThemeInfo(theme));
}

/**
 * Replace non-alphanumeric characters in a theme name with hyphens.
 *
 * ```ts
 * sanitizeThemeName('github light')  // "github-light"
 * ```
 */
export function sanitizeThemeName(name: string): string {
  return name.replaceAll(/[^\p{Alphabetic}\p{Number}_-]/gu, "-");
}

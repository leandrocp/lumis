import { THEME_DATA } from "../generated/theme-data.js";
import { closestFrom } from "./closest.js";
import { optionalThemes } from "./installed.js";
import type { TerminalColors } from "./closest.js";
import type { Theme } from "../types.js";

export { available } from "./installed.js";
export type { TerminalColors } from "./closest.js";
export type { Theme, ThemeInfo, HighlightStyle } from "../types.js";

/** Look up an installed theme by exact name, or return undefined. */
export function get(name: string): Theme | undefined {
  const theme = [...THEME_DATA, ...optionalThemes()].find((candidate) => candidate.name === name);
  return theme && structuredClone(theme);
}

/** Find a readable installed theme matching the supplied terminal colors. */
export function closest(colors: TerminalColors): Theme | undefined {
  return closestFrom(colors, [...THEME_DATA, ...optionalThemes()]);
}

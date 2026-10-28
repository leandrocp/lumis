/**
 * Build CSS from bundled or custom theme objects.
 *
 * To generate and load a theme outside the bundled collection, see
 * {@link https://docs.lumis.sh/recipes/generate-theme-from-neovim-colorscheme | Generate a theme from a Neovim colorscheme},
 * including source-license and redistribution considerations.
 *
 * @module
 */
export { buildCss } from "./css.js";
export type { BuildCssOptions } from "./css.js";
export type { ThemeData, StyleEntry } from "./types.js";

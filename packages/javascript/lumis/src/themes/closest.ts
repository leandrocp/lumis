import { hexToRgb } from "../formatter/ansi-core.js";
import { THEME_DATA } from "../generated/theme-data.js";
import type { Theme } from "../types.js";

type Rgb = readonly [number, number, number];

/** Surrounding colors as six-digit RGB hex strings, with an optional `#`. */
export interface TerminalColors {
  readonly background?: string | null;
  readonly foreground?: string | null;
  /** Entries 0–15, with null for unknown colors. Accents 1–6 affect matching. */
  readonly ansi?: readonly (string | null)[] | null;
}

const CORE_SCOPES = [
  "keyword",
  "string",
  "comment",
  "function",
  "type",
  "constant",
  "number",
  "variable",
  "operator",
  "punctuation",
];
const MAX_BACKGROUND_DISTANCE = 6_400;
const BACKGROUND_TOLERANCE = 400;

interface Candidate {
  readonly theme: Theme;
  readonly foreground: Rgb;
  readonly backgroundDistance: number;
}

function color(hex: string | null | undefined): Rgb | undefined {
  if (hex == null) return undefined;
  const rgb = hexToRgb(hex);
  if (!rgb) throw new TypeError(`Invalid RGB color: ${hex}`);
  return rgb;
}

function linear(channel: number): number {
  const value = channel / 255;
  return value <= 0.04045 ? value / 12.92 : ((value + 0.055) / 1.055) ** 2.4;
}

function luminance([r, g, b]: Rgb): number {
  return 0.2126 * linear(r) + 0.7152 * linear(g) + 0.0722 * linear(b);
}

function contrast(left: Rgb, right: Rgb): number {
  const a = luminance(left);
  const b = luminance(right);
  return (Math.max(a, b) + 0.05) / (Math.min(a, b) + 0.05);
}

function distance(left: Rgb, right: Rgb): number {
  const redMean = Math.floor((left[0] + right[0]) / 2);
  return (
    Math.floor(((512 + redMean) * (left[0] - right[0]) ** 2) / 256) +
    4 * (left[1] - right[1]) ** 2 +
    Math.floor(((767 - redMean) * (left[2] - right[2]) ** 2) / 256)
  );
}

function appearance(background: Rgb, foreground: Rgb | undefined): Theme["appearance"] {
  const bg = luminance(background);
  const fg = foreground && luminance(foreground);
  const dark =
    fg !== undefined && Math.abs(fg - bg) > Number.EPSILON
      ? fg > bg
      : contrast([255, 255, 255], background) >= contrast([0, 0, 0], background);
  return dark ? "dark" : "light";
}

export function coreColors(theme: Theme): Set<string> {
  const colors = CORE_SCOPES.map((scope) => {
    const hex = theme.highlights[scope]?.fg ?? theme.highlights.normal?.fg;
    return hex && hexToRgb(hex)?.join(",");
  });
  return new Set(colors.filter((value): value is string => value !== undefined));
}

function candidate(
  theme: Theme,
  background: Rgb,
  mode: Theme["appearance"],
): Candidate | undefined {
  const foreground = color(theme.highlights.normal?.fg);
  const bg = color(theme.highlights.normal?.bg);
  if (!foreground || !bg || theme.appearance !== mode) return undefined;
  const backgroundDistance = distance(background, bg);
  if (
    backgroundDistance > MAX_BACKGROUND_DISTANCE ||
    coreColors(theme).size <= 3 ||
    contrast(foreground, background) < 3
  )
    return undefined;
  return { theme, foreground, backgroundDistance };
}

function paletteDistance(accents: readonly Rgb[], theme: Theme): number {
  const syntax = Object.entries(theme.highlights)
    .filter(([scope]) => scope !== "normal")
    .flatMap(([, style]) => {
      const rgb = color(style.fg);
      return rgb ? [rgb] : [];
    });
  return accents.reduce(
    (sum, accent) => sum + Math.min(...syntax.map((rgb) => distance(accent, rgb))),
    0,
  );
}

/**
 * Find a readable built-in theme matching the supplied colors.
 *
 * Returns undefined without a background or a sufficiently close theme with
 * at least 3:1 text contrast. Monochrome themes are excluded. Foreground and
 * ANSI accents break ties between similar backgrounds. The returned theme is
 * an independent copy, ready to pass to a formatter.
 *
 * @throws {TypeError} For invalid RGB hex strings or an ANSI palette other than 16 entries.
 */
export function closest(colors: TerminalColors): Theme | undefined {
  return closestFrom(colors, THEME_DATA);
}

export function closestFrom(colors: TerminalColors, themes: readonly Theme[]): Theme | undefined {
  const background = color(colors.background);
  const foreground = color(colors.foreground);
  if (colors.ansi != null && colors.ansi.length !== 16) {
    throw new TypeError("ANSI palette must contain 16 entries");
  }
  const ansi = colors.ansi?.map(color) ?? [];
  if (!background) return undefined;
  const accents = ansi.slice(1, 7).filter((rgb): rgb is Rgb => rgb !== undefined);
  const mode = appearance(background, foreground);
  const candidates = themes.flatMap((theme) => {
    const value = candidate(theme, background, mode);
    return value ? [value] : [];
  });
  const nearest = Math.min(...candidates.map((value) => value.backgroundDistance));
  const tolerance = foreground || accents.length > 0 ? BACKGROUND_TOLERANCE : 0;
  const ranked = candidates
    .filter((value) => value.backgroundDistance <= nearest + tolerance)
    .map((value) => ({
      theme: value.theme,
      backgroundDistance: value.backgroundDistance,
      foregroundDistance: foreground ? distance(foreground, value.foreground) : 0,
      paletteDistance: paletteDistance(accents, value.theme),
    }));
  ranked.sort(
    (a, b) =>
      a.foregroundDistance - b.foregroundDistance ||
      a.paletteDistance - b.paletteDistance ||
      a.backgroundDistance - b.backgroundDistance ||
      (a.theme.name < b.theme.name ? -1 : Number(a.theme.name > b.theme.name)),
  );
  const best = ranked[0];
  return best && structuredClone(best.theme);
}

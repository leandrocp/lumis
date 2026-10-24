import type { StyleEntry, ThemeData } from "./types.js";

export type ThemePack = "default" | "copyleft";

function object(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function style(value: unknown): StyleEntry {
  if (!object(value)) throw new TypeError("Invalid theme highlight");
  const result: StyleEntry = {};
  for (const key of ["fg", "bg"] as const) {
    const entry = value[key];
    if (entry === undefined) continue;
    if (typeof entry !== "string") throw new TypeError(`Invalid theme ${key}`);
    result[key] = entry;
  }
  for (const key of ["bold", "italic", "strikethrough"] as const) {
    const entry = value[key];
    if (entry === undefined) continue;
    if (typeof entry !== "boolean") throw new TypeError(`Invalid theme ${key}`);
    result[key] = entry;
  }
  result.underline = underline(value.underline, value.undercurl);
  return result;
}

function underline(value: unknown, undercurl: unknown): StyleEntry["underline"] {
  if (undercurl !== undefined && typeof undercurl !== "boolean") {
    throw new TypeError("Invalid theme undercurl");
  }
  if (undercurl === true) return "undercurl";
  if (value === undefined || value === false) return undefined;
  if (value === true) return "solid";
  const styles = ["solid", "wavy", "double", "dotted", "dashed", "undercurl"] as const;
  const match = styles.find((entry) => entry === value);
  if (match === undefined) throw new TypeError("Invalid theme underline");
  return match;
}

function themeName(value: unknown): string {
  if (typeof value !== "string" || !/^[a-z0-9_]+$/u.test(value)) {
    throw new TypeError("Invalid theme name");
  }
  return value;
}

function theme(value: unknown): ThemeData {
  if (
    !object(value) ||
    (value.appearance !== "light" && value.appearance !== "dark") ||
    !object(value.highlights)
  ) {
    throw new TypeError("Invalid theme data");
  }
  const highlights = Object.fromEntries(
    Object.entries(value.highlights).map(([scope, entry]) => [scope, style(entry)]),
  );
  if (value.revision !== undefined && typeof value.revision !== "string") {
    throw new TypeError("Invalid theme revision");
  }
  return {
    name: themeName(value.name),
    appearance: value.appearance,
    revision: value.revision,
    highlights,
  };
}

/** Validate and normalize a version-one theme pack manifest. */
export function parseThemePack(value: unknown, pack: ThemePack): ThemeData[] {
  if (
    !object(value) ||
    value.schema_version !== 1 ||
    value.pack !== pack ||
    !Array.isArray(value.themes)
  ) {
    throw new TypeError(`Invalid ${pack} theme pack`);
  }
  const entries: unknown[] = value.themes;
  const themes = entries.map((entry) => theme(entry));
  if (new Set(themes.map((entry) => entry.name)).size !== themes.length) {
    throw new TypeError("Duplicate theme name");
  }
  return themes;
}

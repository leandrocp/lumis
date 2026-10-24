import type { Theme } from "@lumis-sh/lumis";
import { parseThemePack } from "@lumis-sh/themes/manifest";
import catalog from "../../../themes/catalog.json";

function labelizeTheme(id: string): string {
  return id
    .split("_")
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(" ");
}

if (catalog.schema_version !== 1) throw new Error("Unsupported theme catalog");

export const THEMES = catalog.themes
  .map((theme) => ({
    name: theme.name,
    appearance: theme.appearance,
    pack: theme.pack,
    license: theme.license,
    url: theme.url,
    revision: theme.revision,
    id: theme.name,
    label: labelizeTheme(theme.name),
    available: theme.pack === "default" || theme.pack === "copyleft",
  }))
  .sort((a, b) => a.label.localeCompare(b.label));

export type ThemeOption = (typeof THEMES)[number];

export const THEMES_BY_ID = new Map(THEMES.map((theme) => [theme.id, theme]));
export const AVAILABLE_THEMES = THEMES.filter((theme) => theme.available);

const packCache = new Map<string, Promise<Theme[]>>();

export async function loadTheme(id: string): Promise<Theme> {
  const entry = THEMES_BY_ID.get(id);
  if (!entry) {
    throw new Error(`Unknown theme: ${id}`);
  }
  if (!entry.available) {
    throw new Error(`Theme unavailable for redistribution: ${id}`);
  }

  let pack = packCache.get(entry.pack);
  if (!pack) {
    pack =
      entry.pack === "copyleft"
        ? import("@lumis-sh/themes-copyleft/lumis-themes.json").then((data) =>
            parseThemePack(data.default, "copyleft"),
          )
        : import("@lumis-sh/themes/lumis-themes.json").then((data) =>
            parseThemePack(data.default, "default"),
          );
    packCache.set(entry.pack, pack);
  }
  const theme = (await pack).find((candidate) => candidate.name === id);
  if (!theme) throw new Error(`Theme missing from ${entry.pack} pack: ${id}`);
  return theme;
}

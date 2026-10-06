import type { Theme } from "@lumis-sh/lumis";

const themeModules = import.meta.glob<Theme>("../../../themes/*.json", {
  import: "default",
});

function labelizeTheme(id: string): string {
  return id
    .split("_")
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(" ");
}

function stripPrefix(path: string): string {
  const filename = path.split("/").pop();
  return filename?.replace(/\.json$/u, "") ?? path;
}

export const THEMES = Object.keys(themeModules)
  .map((path) => {
    const id = stripPrefix(path);
    if (!id) {
      throw new Error(`Could not derive a theme id from ${path}`);
    }

    return {
      id,
      label: labelizeTheme(id),
      path,
    };
  })
  .sort((a, b) => a.label.localeCompare(b.label));

export type ThemeOption = (typeof THEMES)[number];

export const THEMES_BY_ID = new Map(THEMES.map((theme) => [theme.id, theme]));

const themeCache = new Map<string, Promise<Theme>>();

export function loadTheme(id: string): Promise<Theme> {
  const existing = themeCache.get(id);
  if (existing) return existing;

  const entry = THEMES_BY_ID.get(id);
  if (!entry) {
    throw new Error(`Unknown theme: ${id}`);
  }

  const loader = themeModules[entry.path];
  if (!loader) {
    throw new Error(`No loader for theme: ${id}`);
  }

  const promise = loader();
  themeCache.set(id, promise);
  return promise;
}

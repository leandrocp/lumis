import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { resolve } from "node:path";
import { parseThemePack } from "@lumis-sh/themes/manifest";
import { packageRoots } from "../runtime/package-roots.js";
import { THEMES } from "../generated/themes-meta.js";
import type { Theme, ThemeInfo } from "../types.js";

const ownRequire = createRequire(typeof __filename === "string" ? __filename : import.meta.url);
const manifests = new Map<string, Theme[]>();

function optionalManifest(): string | undefined {
  const sources = [
    ...packageRoots,
    resolve(process.cwd(), "package.json"),
    ...(process.argv[1] ? [resolve(process.argv[1])] : []),
  ];
  for (const source of sources) {
    try {
      return createRequire(source).resolve("@lumis-sh/themes-copyleft/lumis-themes.json");
    } catch (error) {
      if (!missingModule(error)) throw error;
    }
  }
  try {
    return ownRequire.resolve("@lumis-sh/themes-copyleft/lumis-themes.json");
  } catch (error) {
    if (!missingModule(error)) throw error;
    return undefined;
  }
}

function missingModule(error: unknown): boolean {
  return error instanceof Error && "code" in error && error.code === "MODULE_NOT_FOUND";
}

export function optionalThemes(): readonly Theme[] {
  const manifest = optionalManifest();
  if (!manifest) return [];
  const cached = manifests.get(manifest);
  if (cached) return cached;
  const data: unknown = JSON.parse(readFileSync(manifest, "utf8"));
  const themes = parseThemePack(data, "copyleft");
  const names = new Set(THEMES.map((theme) => theme.name));
  if (themes.some((theme) => names.has(theme.name))) {
    throw new TypeError("Duplicate installed theme name");
  }
  manifests.set(manifest, themes);
  return themes;
}

export function available(): ThemeInfo[] {
  return [...THEMES, ...optionalThemes()]
    .map(({ name, appearance }) => ({ name, appearance }))
    .sort((a, b) => a.name.localeCompare(b.name));
}

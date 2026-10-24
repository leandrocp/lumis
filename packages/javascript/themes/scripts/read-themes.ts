import fs from "node:fs";
import path from "node:path";
import { parseThemePack } from "../src/manifest.js";
import type { ThemePack } from "../src/manifest.js";

const THEMES_SRC = path.resolve(import.meta.dirname, "../../../../themes/packs");

export function readThemes(pack: ThemePack = "default") {
  const data: unknown = JSON.parse(fs.readFileSync(path.join(THEMES_SRC, `${pack}.json`), "utf8"));
  return parseThemePack(data, pack).map((theme) => ({
    name: theme.name,
    file: `${theme.name}.json`,
    data: theme,
  }));
}

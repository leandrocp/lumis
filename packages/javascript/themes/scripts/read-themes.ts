import fs from "node:fs";
import path from "node:path";
import type { ThemeData } from "../src/types.js";

const THEMES_SRC = path.resolve(import.meta.dirname, "../../../../themes");

function normalize(data: ThemeData) {
  for (const entry of Object.values(data.highlights)) {
    if (entry.underline === true) {
      entry.underline = "solid";
    }
    if (entry.undercurl) {
      entry.underline = "undercurl";
      delete entry.undercurl;
    }
  }
}

export function readThemes() {
  const files = fs
    .readdirSync(THEMES_SRC)
    .filter((f) => f.endsWith(".json"))
    .sort();
  const themes: { name: string; file: string; data: ThemeData }[] = [];

  for (const file of files) {
    const name = path.basename(file, ".json");
    const json = fs.readFileSync(path.join(THEMES_SRC, file), "utf-8");
    const data: ThemeData = JSON.parse(json);

    if (!data.name || !data.highlights) {
      console.warn(`  skipping ${file}: missing name or highlights`);
      continue;
    }

    normalize(data);
    themes.push({ name, file, data });
  }

  return themes;
}

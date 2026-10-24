import { defineConfig } from "tsup";
import fs from "node:fs";
import path from "node:path";

const themesDir = path.resolve(import.meta.dirname, "themes");
const themeEntries: Record<string, string> = {};
if (fs.existsSync(themesDir)) {
  for (const file of fs.readdirSync(themesDir)) {
    if (file.endsWith(".ts")) {
      const name = path.basename(file, ".ts");
      themeEntries[`themes/${name}`] = `themes/${file}`;
    }
  }
}

export default defineConfig({
  entry: {
    index: "src/index.ts",
    manifest: "src/manifest.ts",
    ...themeEntries,
  },
  format: ["esm", "cjs"],
  experimentalDts: true,
  splitting: true,
  clean: true,
  treeshake: true,
});

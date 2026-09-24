import { defineConfig } from "tsup";
import fs from "node:fs";
import path from "node:path";

// Dynamically discover generated language modules
const langsDir = path.resolve(import.meta.dirname, "langs");
const langEntries: Record<string, string> = {};
if (fs.existsSync(langsDir)) {
  for (const file of fs.readdirSync(langsDir)) {
    if (file.endsWith(".ts")) {
      const name = path.basename(file, ".ts");
      langEntries[`langs/${name}`] = `langs/${file}`;
    }
  }
}

// Dynamically discover generated bundle modules
const bundlesDir = path.resolve(import.meta.dirname, "bundles");
const bundleEntries: Record<string, string> = {};
if (fs.existsSync(bundlesDir)) {
  for (const file of fs.readdirSync(bundlesDir)) {
    if (file.endsWith(".ts")) {
      const name = path.basename(file, ".ts");
      bundleEntries[`bundles/${name}`] = `bundles/${file}`;
    }
  }
}

export default defineConfig({
  entry: {
    index: "src/index.ts",
    "index.browser": "src/index.browser.ts",
    formatters: "src/formatters.ts",
    "formatters/html": "src/formatter/html.ts",
    "formatters/ansi": "src/formatter/ansi.ts",
    ...langEntries,
    ...bundleEntries,
  },
  format: ["esm", "cjs"],
  experimentalDts: true,
  splitting: true,
  clean: true,
  treeshake: true,
  noExternal: ["web-tree-sitter"],
});

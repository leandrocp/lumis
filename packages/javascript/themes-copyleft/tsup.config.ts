import { defineConfig } from "tsup";
import fs from "node:fs";

const entries: Record<string, string> = { index: "src/index.ts" };
for (const file of fs.readdirSync(new URL("./themes", import.meta.url))) {
  if (file.endsWith(".ts")) entries[`themes/${file.slice(0, -3)}`] = `themes/${file}`;
}

export default defineConfig({
  entry: entries,
  format: ["esm", "cjs"],
  experimentalDts: true,
  splitting: true,
  clean: true,
  treeshake: true,
});

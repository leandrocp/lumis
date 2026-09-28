import { readFileSync } from "node:fs";
import { defineConfig } from "vite";

// Pre-bundling moves a package's code away from its parser file, which the
// package finds relative to itself, so leave the parser packages out of it:
// the bundle and every language package it imports.
const bundle = JSON.parse(
  readFileSync(new URL("node_modules/@lumis-sh/wasm-bundle-web/package.json", import.meta.url)),
);

export default defineConfig({
  optimizeDeps: {
    exclude: ["@lumis-sh/wasm-bundle-web", ...Object.keys(bundle.dependencies)],
  },
});

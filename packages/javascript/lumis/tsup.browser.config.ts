import { copyFile } from "node:fs/promises";
import { defineConfig } from "tsup";

export default defineConfig({
  entry: {
    "index.browser": "src/index.browser.ts",
    "index.workerd": "src/index.workerd.ts",
    "index.edge": "src/index.edge.ts",
    "languages.browser": "src/languages.browser.ts",
    "themes.browser": "src/themes/index.ts",
  },
  format: ["esm"],
  platform: "browser",
  target: "es2022",
  experimentalDts: true,
  tsconfig: "tsconfig.build.json",
  splitting: true,
  clean: false,
  treeshake: true,
  noExternal: ["web-tree-sitter"],
  // Rollup removes this dead branch after esbuild resolves its imports.
  external: ["module", /\.wasm(?:\?module)?$/u],
  // CDN process polyfills can report a Node version. Resolve both of
  // Tree-sitter's Node checks here, without changing the Node Wasm fallback.
  // The Workers and Edge entries import Tree-sitter's runtime as a module, which the
  // host compiles at deploy time; it cannot compile bytes at run time.
  onSuccess: () =>
    copyFile("node_modules/web-tree-sitter/web-tree-sitter.wasm", "dist/web-tree-sitter.wasm"),
  define: {
    process: "undefined",
    "globalThis.process": "undefined",
  },
});

import { defineConfig } from "tsup";

export default defineConfig({
  entry: { "index.browser": "src/index.browser.ts" },
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
  external: ["module"],
  // CDN process polyfills can report a Node version. Resolve both of
  // Tree-sitter's Node checks here, without changing the Node Wasm fallback.
  define: {
    process: "undefined",
    "globalThis.process": "undefined",
  },
});

import { defineConfig } from "vite";

// Pre-bundling moves a package's code away from its parser file, which the
// package finds relative to itself, so leave the parser packages out of it.
export default defineConfig({
  optimizeDeps: {
    exclude: ["@lumis-sh/wasm-diff", "@lumis-sh/wasm-elixir"],
  },
});

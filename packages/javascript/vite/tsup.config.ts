import { defineConfig } from "tsup";

export default defineConfig({
  entry: {
    index: "src/index.ts",
  },
  format: ["esm", "cjs"],
  experimentalDts: true,
  splitting: false,
  clean: true,
  treeshake: true,
});

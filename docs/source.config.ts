import { defineConfig } from "fumadocs-mdx/config";
import remarkLumis from "./plugins/remark-lumis.mjs";

export default defineConfig({
  mdxOptions: {
    remarkPlugins: [remarkLumis],
  },
});

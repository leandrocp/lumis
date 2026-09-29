import { fileURLToPath } from "node:url";
import { defineConfig, type Plugin } from "vite";
import tailwindcss from "@tailwindcss/vite";

const siteUrl = new URL("https://lumis.sh/");
const pages = {
  main: "index.html",
  comparison: "comparison/index.html",
  showcase: "showcase/index.html",
};

function escapeXml(value: string): string {
  return value
    .replaceAll("&", "&amp;")
    .replaceAll("'", "&apos;")
    .replaceAll('"', "&quot;")
    .replaceAll(">", "&gt;")
    .replaceAll("<", "&lt;");
}

function sitemap(): Plugin {
  const entries = Object.values(pages)
    .map((page) => new URL(page.replace(/index\.html$/, ""), siteUrl))
    .map((url) => `  <url>\n    <loc>${escapeXml(url.href)}</loc>\n  </url>`)
    .join("\n");

  return {
    name: "lumis-sitemap",
    apply: "build",
    generateBundle() {
      this.emitFile({
        type: "asset",
        fileName: "sitemap.xml",
        source: `<?xml version="1.0" encoding="UTF-8"?>\n<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n${entries}\n</urlset>\n`,
      });
    },
  };
}

export default defineConfig({
  plugins: [tailwindcss(), sitemap()],
  build: {
    rollupOptions: {
      input: Object.fromEntries(
        Object.entries(pages).map(([name, page]) => [
          name,
          fileURLToPath(new URL(page, import.meta.url)),
        ]),
      ),
    },
  },
  worker: {
    format: "es",
  },
  server: {
    host: "0.0.0.0",
    port: 4321,
  },
});

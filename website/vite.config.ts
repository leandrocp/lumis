import { readdirSync, readFileSync } from "node:fs";
import { join, relative } from "node:path";
import { fileURLToPath } from "node:url";
import { createServer, defineConfig, normalizePath, type Plugin, type ResolvedConfig } from "vite";
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
    .map((page) => new URL(page.replace(/index\.html$/u, ""), siteUrl))
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

// The demos ship both packs' theme data in their chunks, so each pack's notice
// and upstream license texts must ship beside them.
function themeLicenses(): Plugin {
  const packs = { default: "@lumis-sh/themes", copyleft: "@lumis-sh/themes-copyleft" };
  return {
    name: "lumis-theme-licenses",
    apply: "build",
    generateBundle() {
      for (const [pack, name] of Object.entries(packs)) {
        const root = fileURLToPath(new URL(`./node_modules/${name}/dist`, import.meta.url));
        const files = [
          join(root, "NOTICE.md"),
          ...readdirSync(join(root, "licenses"), { recursive: true, withFileTypes: true })
            .filter((entry) => entry.isFile())
            .map((entry) => join(entry.parentPath, entry.name)),
        ];
        for (const file of files) {
          this.emitFile({
            type: "asset",
            fileName: `theme-licenses/${pack}/${normalizePath(relative(root, file))}`,
            source: readFileSync(file),
          });
        }
      }
    },
  };
}

function prerender(): Plugin {
  let config: ResolvedConfig;
  const rendered = new Map<string, string>();
  return {
    name: "lumis-prerender",
    apply: "build",
    configResolved(resolvedConfig) {
      config = resolvedConfig;
    },
    async buildStart() {
      const server = await createServer({
        configFile: config.configFile ?? false,
        root: config.root,
        mode: config.mode,
        server: { middlewareMode: true, watch: null, hmr: false, ws: false },
        appType: "custom",
        // Disables the client dependency optimizer, which SSR never uses and
        // which would otherwise overwrite the dev server's cache.
        optimizeDeps: { noDiscovery: true, include: [] },
      });
      try {
        const entry = await server.ssrLoadModule("/src/entry-server.ts");
        const renderedPages: unknown = entry.renderPages();
        if (!(renderedPages instanceof Map)) throw new TypeError("Prerender did not return pages");
        for (const [path, html] of renderedPages) {
          if (typeof path !== "string" || typeof html !== "string") {
            throw new TypeError("Prerender returned an invalid page");
          }
          rendered.set(path, html);
        }
      } finally {
        await server.close();
      }
    },
    transformIndexHtml: {
      order: "pre",
      handler(html, context) {
        const content = rendered.get(context.path);
        if (!content) throw new Error(`No prerendered content for ${context.path}`);
        const outlet = '<div id="app"></div>';
        if (!html.includes(outlet)) throw new Error(`No app outlet in ${context.path}`);
        return html.replace(outlet, () => `<div id="app">${content}</div>`);
      },
    },
  };
}

export default defineConfig({
  plugins: [tailwindcss(), sitemap(), themeLicenses(), prerender()],
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

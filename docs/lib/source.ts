import { llms, loader } from "fumadocs-core/source";
import { lucideIconsPlugin } from "fumadocs-core/source/lucide-icons";
import { docsRoute } from "./shared";
import { defineDocs } from "fumadocs-mdx/macro";
import { metaSchema, pageSchema } from "fumadocs-core/source/schema";
import { z } from "zod";

const lumisPageSchema = pageSchema.extend({
  keywords: z.array(z.string()).optional(),
});

function agentMarkdown(raw: string): string {
  const body = raw.replace(/^---\n[\s\S]*?\n---\n/, "");
  let fence: string | undefined;
  return body
    .split("\n")
    .flatMap((line) => {
      const delimiter = line.match(/^\s*(`{3,}|~{3,})/);
      if (delimiter) {
        if (!fence) fence = delimiter[1][0];
        else if (fence === delimiter[1][0]) fence = undefined;
        return [line];
      }
      if (fence) return [line];
      if (/^import \{ Tabs, Tab \} from 'fumadocs-ui\/components\/tabs';$/.test(line)) return [];
      if (/^\s*<Tabs\b.*>\s*$/.test(line) || /^\s*<\/(?:Tab|Tabs)>\s*$/.test(line)) return [];
      const tab = line.match(/^\s*<Tab value="([^"]+)">\s*$/);
      if (tab) return [`### ${tab[1]}`];
      return [line];
    })
    .join("\n")
    .trim();
}

const docs = defineDocs({
  dir: "content",
  docs: {
    schema: lumisPageSchema,
    postprocess: {
      includeProcessedMarkdown: true,
    },
  },
  meta: {
    schema: metaSchema,
  },
});

// See https://fumadocs.dev/docs/headless/source-api for more info
export const source = loader({
  baseUrl: docsRoute,
  source: docs.toFumadocsSource(),
  plugins: [lucideIconsPlugin()],
});

export const docsLlms = llms(source, {
  renderPage: async (page) => {
    const { readFile } = await import("node:fs/promises");
    const { join } = await import("node:path");
    const raw = await readFile(join(process.cwd(), "content", page.path), "utf8");
    const markdown = agentMarkdown(raw);
    return `# ${page.data.title} (${page.url})\n\n${markdown}`;
  },
});

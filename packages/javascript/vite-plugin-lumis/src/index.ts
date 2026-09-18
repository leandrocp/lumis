import type { Plugin } from "vite";
import type { RehypeLumisOptions } from "@lumis-sh/rehype-lumis";
import rehypeLumis from "@lumis-sh/rehype-lumis";
import { fromHtml } from "hast-util-from-html";
import { toHtml } from "hast-util-to-html";
import { unified } from "unified";
import { visit } from "unist-util-visit";

export type VitePluginLumisOptions = RehypeLumisOptions;

type Tree = ReturnType<typeof fromHtml>;
type Pre = Extract<Tree["children"][number], { tagName: string }>;

interface Block {
  start: number;
  end: number;
  pre: Pre;
}

function createProcessor(options: VitePluginLumisOptions) {
  return unified().use(rehypeLumis, options).freeze();
}

// Vite splices its own HTML edits into the source string by offset rather than
// reserializing the document, and this has to do the same. Serializing the whole
// tree rewrites everything a plugin never touched: `&amp;` comes back as
// `&#x26;`, bare entry HTML gains `<html><head><body>`, and `<%= title %>` in a
// text node turns into `&#x3C;%= title %>`, which the EJS-style plugins that run
// after this one no longer recognize.
function findBlocks(tree: Tree): Block[] {
  const blocks: Block[] = [];

  visit(tree, "element", (node) => {
    if (node.tagName !== "pre") {
      return;
    }

    const start = node.position?.start.offset;
    const end = node.position?.end.offset;
    if (start === undefined || end === undefined) {
      return;
    }

    blocks.push({ start, end, pre: node });
  });

  return blocks;
}

/** Highlight `pre > code` blocks in Vite HTML entry points. */
export default function lumis(options: VitePluginLumisOptions): Plugin {
  let processor: ReturnType<typeof createProcessor> | undefined;
  let ready: Promise<void> | undefined;

  function getProcessor() {
    processor ??= createProcessor(options);
    return processor;
  }

  function warmHighlighter(): Promise<void> {
    ready ??= getProcessor()
      .run({ type: "root", children: [] })
      .then(() => undefined);
    return ready;
  }

  return {
    name: "@lumis-sh/vite-plugin-lumis",
    buildStart: warmHighlighter,
    configureServer: warmHighlighter,
    async transformIndexHtml(html) {
      const blocks = findBlocks(fromHtml(html));
      if (blocks.length === 0) {
        return;
      }

      await warmHighlighter();
      let out = "";
      let cursor = 0;

      for (const block of blocks) {
        // `pre` nests, and `rehype-lumis` descends into one it declined to reach
        // the blocks inside it. Blocks arrive outermost first, so anything behind
        // the cursor is already part of a replacement.
        if (block.start < cursor) {
          continue;
        }

        const result = await getProcessor().run({ type: "root", children: [block.pre] });
        // `rehype-lumis` replaces the node it accepts, so a `pre` still standing
        // is one it declined: no `code` first child, or a block that failed to
        // highlight. Leaving the source slice alone is what declining means here.
        if (result.children[0] === block.pre) {
          continue;
        }

        out += html.slice(cursor, block.start) + toHtml(result as Parameters<typeof toHtml>[0]);
        cursor = block.end;
      }

      return cursor === 0 ? undefined : out + html.slice(cursor);
    },
  };
}

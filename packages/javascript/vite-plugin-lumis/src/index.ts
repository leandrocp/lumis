import type { Plugin } from "vite";
import type { RehypeLumisOptions } from "@lumis-sh/rehype-lumis";
import rehypeLumis from "@lumis-sh/rehype-lumis";
import { fromHtml } from "hast-util-from-html";
import { toHtml } from "hast-util-to-html";
import { unified } from "unified";

export type VitePluginLumisOptions = RehypeLumisOptions;

// Parsing and reserializing rewrites a document even when nothing was
// highlighted: bare entry HTML gains `<html><head><body>`, `&amp;` comes back as
// `&#x26;`, and unquoted attributes gain quotes. `rehype-lumis` only ever
// replaces `pre` elements, so a document without one is returned untouched.
const PRE_TAG = /<pre[\s/>]/i;

function createProcessor(options: VitePluginLumisOptions) {
  return unified().use(rehypeLumis, options).freeze();
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
      if (!PRE_TAG.test(html)) {
        return;
      }

      await warmHighlighter();
      const tree = fromHtml(html);
      const transformed = await getProcessor().run(tree);
      // The parser and the serializer resolve separate patches of `@types/hast`,
      // whose nodes are structurally identical but nominally distinct. Deriving
      // the parameter keeps this correct whichever patch either one lands on.
      return toHtml(transformed as Parameters<typeof toHtml>[0]);
    },
  };
}

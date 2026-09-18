import type { Plugin } from "vite";
import type { RehypeLumisOptions } from "@lumis-sh/rehype-lumis";
import rehypeLumis from "@lumis-sh/rehype-lumis";
import { fromHtml } from "hast-util-from-html";
import { toHtml } from "hast-util-to-html";
import { unified } from "unified";

export type VitePluginLumisOptions = RehypeLumisOptions;

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
      await warmHighlighter();
      const tree = fromHtml(html);
      const transformed = await getProcessor().run(tree);
      // @ts-expect-error -- the serializer resolves a newer patch of the same HAST types.
      return toHtml(transformed);
    },
  };
}

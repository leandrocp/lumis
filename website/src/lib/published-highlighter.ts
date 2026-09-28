import { createHighlighter } from "@lumis-sh/lumis";
import type { Highlighter, Language } from "@lumis-sh/lumis";

// The homepage points the resolver at the parsers in `node_modules`, because it
// bundles them. These demos show a page that bundles none: every parser comes
// from the CDN, at the version the release pins. A browser loads only packages
// it imported unless resolvers say otherwise, so these say so.
const CDN = "https://cdn.jsdelivr.net/npm";
let shared: Promise<Highlighter> | undefined;
const loaded = new Map<string, Promise<void>>();

/**
 * Loads languages the way a browser has to: ahead of the document that names
 * them. A parser cannot be fetched inside the synchronous walk, so an injected
 * language is loaded here rather than discovered mid-highlight.
 */
export async function highlighterFor(...languages: Language[]): Promise<Highlighter> {
  shared ??= createHighlighter({
    languagePackageResolver: (packageName, versionRange) =>
      `${CDN}/${packageName}@${versionRange}/lumis.json`,
    wasmResolver: (_language, wasm) =>
      `${CDN}/${wasm.packageName}@${wasm.version}/${wasm.name}.wasm`,
  });
  const hl = await shared;

  await Promise.all(
    languages.map((language) => {
      let loading = loaded.get(language.id);
      if (!loading) {
        loading = hl.loadLanguage(language);
        loaded.set(language.id, loading);
      }
      return loading;
    }),
  );

  return hl;
}

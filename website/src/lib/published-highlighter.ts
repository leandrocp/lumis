import { createHighlighter } from "@lumis-sh/lumis";
import type { Highlighter, Language } from "@lumis-sh/lumis";

// The homepage points the resolver at the parsers in `node_modules`, because it
// bundles them. Nothing here does that: these demos resolve every parser the way
// an install does, from the CDN, at the version the release pins. That is the
// browser path documented in /usage/javascript, and it is the one worth showing.
let shared: Promise<Highlighter> | undefined;
const loaded = new Map<string, Promise<void>>();

/**
 * Loads languages the way a browser has to: ahead of the document that names
 * them. A parser cannot be fetched inside the synchronous walk, so an injected
 * language is loaded here rather than discovered mid-highlight.
 */
export async function highlighterFor(...languages: Language[]): Promise<Highlighter> {
  shared ??= createHighlighter();
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

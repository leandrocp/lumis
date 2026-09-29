import { createHighlighter } from "@lumis-sh/lumis";
import type { Highlighter, Language } from "@lumis-sh/lumis";

// These demos import each language from its parser package, the same way the
// code beside them does, so the page ships the parsers it imports and fetches
// nothing else.
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

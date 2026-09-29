import { availableLanguages } from "@lumis-sh/lumis";
import type { LazyLanguage } from "@lumis-sh/lumis";
import full from "@lumis-sh/wasm-bundle-full";
import { getSample } from "./samples";

export interface LanguageOption {
  id: string;
  label: string;
  language: LazyLanguage;
}

// Display names come from the catalog, which is the same list the CLI, Elixir
// and Rust print, so a language added upstream arrives here already named. The
// table this replaced had drifted: it rendered `toml` as "Toml" and
// `jinja_inline` as "Jinja_inline".
const CATALOG_NAMES = new Map(availableLanguages().map(({ id, name }) => [id, name]));

export const LANGUAGES: LanguageOption[] = Object.entries(full)
  .map(([id, language]) => ({
    id,
    label: CATALOG_NAMES.get(id) || id,
    language,
  }))
  .sort((a, b) => a.label.localeCompare(b.label));

export const LANGUAGES_BY_ID = new Map(LANGUAGES.map((language) => [language.id, language]));

/**
 * What the Playground shows before the visitor picks anything.
 *
 * Fixed rather than random so the first thing a visitor sees is the same every
 * time, and so the parser behind it can be warmed while the page is still
 * rendering. Rust is the reference runtime, leads the install tabs, and its
 * sample carries doc comments with a fenced code block in them, so injections
 * show up without anyone having to go looking for them.
 */
export const PLAYGROUND_DEFAULT_LANGUAGE = "rust";

/**
 * The languages this page highlights before any interaction: the Playground
 * default, every Quickstart tab, and the injections demo.
 *
 * Warming these is not what makes them work — every section still loads what it
 * needs on demand — it only starts the fetches together and early instead of
 * one at a time as each section reaches them. An entry that drifts out of date
 * costs that head start and nothing else.
 *
 * The streaming demo's `markdown` is deliberately absent: that section waits for
 * the visitor to scroll to it, so warming it would download a parser for a
 * section most visits never reach.
 */
export const FIRST_PAINT_LANGUAGES = [
  PLAYGROUND_DEFAULT_LANGUAGE,
  "bash",
  "javascript",
  "elixir",
  "java",
  "html",
  "css",
];

export { getSample };

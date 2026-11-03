import { createHighlighter, type Highlighter } from "@lumis-sh/lumis/client";
import type { ReactNode } from "react";
import {
  getLanguageId,
  highlightHtml,
  toReactNode,
  type InlineCodeOptions,
  type LumisBaseOptions,
  type LumisRenderOptions,
} from "./common.js";

export { getLanguageId, highlightHtml, toReactNode } from "./common.js";

export interface LumisOptions extends LumisBaseOptions {
  highlighter?: HighlighterInput;
}

export interface CodeBlockProps extends LumisOptions {}

export interface InlineCodeProps extends LumisOptions, InlineCodeOptions {}

export type HighlighterInput = Highlighter | Promise<Highlighter>;

const sharedHighlighter = createHighlighter();

export function getHighlighter(highlighter?: HighlighterInput): HighlighterInput {
  return highlighter ?? sharedHighlighter;
}

export function isHighlighterPromise(
  highlighter: HighlighterInput,
): highlighter is Promise<Highlighter> {
  return highlighter instanceof Promise;
}

export function isUnknownLanguage(
  highlighter: Highlighter,
  language: LumisBaseOptions["formatter"]["language"],
): boolean {
  const languageId = getLanguageId(language);
  return languageId != null && !highlighter.registeredLanguages.includes(languageId);
}

export function isLoadedLanguage(
  highlighter: Highlighter,
  language: LumisBaseOptions["formatter"]["language"],
): boolean {
  const languageId = getLanguageId(language);
  return languageId != null && highlighter.languages.includes(languageId);
}

export async function highlightToReactNode(
  highlighterInput: HighlighterInput | undefined,
  options: LumisRenderOptions,
): Promise<ReactNode> {
  const highlighter = await getHighlighter(highlighterInput);
  if (options.formatter.language != null) {
    await highlighter.loadLanguage(options.formatter.language);
  }

  const html = highlightHtml(highlighter, options);
  return toReactNode(html);
}

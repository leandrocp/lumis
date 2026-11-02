import { createHighlighter, type Highlighter } from "@lumis-sh/lumis";
import type { ReactNode } from "react";
import {
  highlightHtml,
  toReactNode,
  type InlineCodeOptions,
  type LumisBaseOptions,
  type LumisRenderOptions,
} from "./common.js";

export interface RenderCodeBlockOptions extends LumisBaseOptions {
  highlighter?: HighlighterInput;
}

export interface RenderInlineCodeOptions extends RenderCodeBlockOptions, InlineCodeOptions {}

export type HighlighterInput = Highlighter | Promise<Highlighter>;

const sharedHighlighter = createHighlighter();

export function getHighlighter(highlighter?: HighlighterInput): HighlighterInput {
  return highlighter ?? sharedHighlighter;
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

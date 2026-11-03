import type { ReactNode } from "react";
import type { RenderInlineCodeOptions } from "./serverShared.js";
import { highlightToReactNode } from "./serverShared.js";

export async function renderInlineCode(options: RenderInlineCodeOptions): Promise<ReactNode> {
  return highlightToReactNode(options.highlighter, { ...options, inline: true });
}

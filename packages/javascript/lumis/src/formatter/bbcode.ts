import type { BBCodeScopedFormatter, HighlightEvent } from "../types.js";
import { LineSelection, composeLineDecorations } from "../decorations.js";
import { decodeSourceSlice, encodeSource } from "./html.js";

/**
 * The tag a highlighted line is wrapped in.
 *
 * Every other tag this formatter emits is a highlight scope with its dots turned
 * into hyphens, and `highlighted` is the scope a theme styles a highlighted line
 * with, so this one is derived the same way rather than configured.
 */
const HIGHLIGHTED_TAG = "highlighted";

function escapeBbcodeText(text: string): string {
  return text.replaceAll("[", "&#91;").replaceAll("]", "&#93;");
}

function scopeToTagName(scope: string, language: string): string {
  return `${scope}.${language}`.replaceAll(".", "-");
}

export function formatBBCode(
  source: string,
  events: readonly HighlightEvent[],
  formatter?: BBCodeScopedFormatter,
): string {
  const sourceBytes = encodeSource(source);
  const parts: string[] = [];
  const scopeStack: string[] = [];

  // BBCode carries no line structure of its own, so the line decorations are
  // only worth composing when there is a line to mark. Without them the stream,
  // and the output, are exactly what they were.
  const selection = new LineSelection(formatter?.highlightLines?.lines);
  const decorated = selection.isEmpty
    ? events
    : composeLineDecorations(sourceBytes, events, selection);
  const state = { lineHighlighted: false };

  for (const event of decorated) {
    appendEvent(parts, sourceBytes, scopeStack, state, event);
  }

  return parts.join("");
}

function appendEvent(
  parts: string[],
  sourceBytes: Uint8Array,
  scopeStack: string[],
  state: { lineHighlighted: boolean },
  event: HighlightEvent,
): void {
  switch (event.type) {
    case "start": {
      const tagName = scopeToTagName(event.scope, event.language);
      parts.push(`[${tagName}]`);
      scopeStack.push(tagName);
      break;
    }
    case "end": {
      const tagName = scopeStack.pop();
      if (tagName) parts.push(`[/${tagName}]`);
      break;
    }
    case "decorationStart":
      state.lineHighlighted = event.decoration.highlighted;
      if (state.lineHighlighted) parts.push(`[${HIGHLIGHTED_TAG}]`);
      break;
    case "decorationEnd":
      if (state.lineHighlighted) parts.push(`[/${HIGHLIGHTED_TAG}]`);
      state.lineHighlighted = false;
      break;
    case "source":
      parts.push(escapeBbcodeText(decodeSourceSlice(sourceBytes, event.start, event.end)));
      break;
    // Caller annotations carry data this formatter has never seen.
    default:
      break;
  }
}

import type { BBCodeScopedFormatter, Decoration, HighlightEvent } from "../types.js";
import { LineSelection, composeLineDecorations, rainbowBracketScope } from "../decorations.js";
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

function formatterLanguage(formatter: BBCodeScopedFormatter | undefined): string {
  if (typeof formatter?.language === "string") return formatter.language;
  return formatter?.language?.id ?? "plaintext";
}

function formatterEvents(
  sourceBytes: Uint8Array,
  events: readonly HighlightEvent[],
  formatter: BBCodeScopedFormatter | undefined,
): readonly HighlightEvent[] {
  const selection = new LineSelection(formatter?.highlightLines?.lines);
  if (selection.isEmpty) return events;
  return composeLineDecorations(sourceBytes, events, selection);
}

export function formatBBCode(
  source: string,
  events: readonly HighlightEvent[],
  formatter?: BBCodeScopedFormatter,
): string {
  const sourceBytes = encodeSource(source);
  const parts: string[] = [];
  const scopeStack: string[] = [];
  const decorations: Decoration[] = [];
  const documentLanguage = formatterLanguage(formatter);

  // BBCode carries no line structure of its own, so the line decorations are
  // only worth composing when there is a line to mark. Without them the stream,
  // and the output, are exactly what they were.
  const decorated = formatterEvents(sourceBytes, events, formatter);
  for (const event of decorated) {
    appendEvent(parts, sourceBytes, scopeStack, decorations, documentLanguage, event);
  }

  return parts.join("");
}

function appendDecorationStart(
  parts: string[],
  decorations: Decoration[],
  documentLanguage: string,
  decoration: Decoration,
): void {
  decorations.push(decoration);
  if (decoration.type === "line") {
    if (decoration.highlighted) parts.push(`[${HIGHLIGHTED_TAG}]`);
    return;
  }
  const tagName = scopeToTagName(rainbowBracketScope(decoration.depth), documentLanguage);
  parts.push(`[${tagName}]`);
}

function appendDecorationEnd(
  parts: string[],
  decorations: Decoration[],
  documentLanguage: string,
): void {
  const decoration = decorations.pop();
  if (decoration?.type === "line") {
    if (decoration.highlighted) parts.push(`[/${HIGHLIGHTED_TAG}]`);
    return;
  }
  if (decoration?.type === "rainbowBracket") {
    const tagName = scopeToTagName(rainbowBracketScope(decoration.depth), documentLanguage);
    parts.push(`[/${tagName}]`);
  }
}

function appendEvent(
  parts: string[],
  sourceBytes: Uint8Array,
  scopeStack: string[],
  decorations: Decoration[],
  documentLanguage: string,
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
    case "decorationStart": {
      appendDecorationStart(parts, decorations, documentLanguage, event.decoration);
      break;
    }
    case "decorationEnd": {
      appendDecorationEnd(parts, decorations, documentLanguage);
      break;
    }
    case "source":
      parts.push(escapeBbcodeText(decodeSourceSlice(sourceBytes, event.start, event.end)));
      break;
    // Caller annotations carry data this formatter has never seen.
    default:
      break;
  }
}

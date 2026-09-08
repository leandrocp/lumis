import type { HighlightEvent } from "../types.js";
import { decodeSourceSlice, encodeSource } from "./html.js";

function escapeBbcodeText(text: string): string {
  return text.replaceAll("[", "&#91;").replaceAll("]", "&#93;");
}

function scopeToTagName(scope: string, language: string): string {
  return `${scope}.${language}`.replaceAll(".", "-");
}

export function formatBBCode(source: string, events: readonly HighlightEvent[]): string {
  const sourceBytes = encodeSource(source);
  const parts: string[] = [];
  const scopeStack: string[] = [];

  for (const event of events) {
    if (event.type === "start") {
      const tagName = scopeToTagName(event.scope, event.language);
      parts.push(`[${tagName}]`);
      scopeStack.push(tagName);
      continue;
    }

    if (event.type === "end") {
      const tagName = scopeStack.pop();
      if (tagName) {
        parts.push(`[/${tagName}]`);
      }
      continue;
    }

    if (event.type === "annotationStart" || event.type === "annotationEnd") {
      continue;
    }

    const text = decodeSourceSlice(sourceBytes, event.startByte, event.endByte);
    parts.push(escapeBbcodeText(text));
  }

  return parts.join("");
}

import { HIGHLIGHT_NAMES } from "../highlights.js";
import type { SyntaxHighlightEvent } from "../types.js";

const SOURCE_EVENT = 0;
const START_EVENT = 1;
const END_EVENT = 2;
const SOURCE_EVENT_BYTES = 8;
const START_EVENT_HEADER_BYTES = 4;
const decoder = new TextDecoder("utf-8", { fatal: true });

function requireBytes(offset: number, count: number, length: number): void {
  if (offset + count > length) {
    throw new Error("Invalid native Lumis event buffer");
  }
}

/**
 * Decode the compact event stream returned by the native addon.
 *
 * Every event starts with a one-byte tag:
 * - source (0): start byte as u32 LE, end byte as u32 LE
 * - start  (1): scope index as u16 LE, language byte length as u16 LE, UTF-8 language
 * - end    (2): no payload
 */
export function decodeNativeEvents(data: Uint8Array): SyntaxHighlightEvent[] {
  const view = new DataView(data.buffer, data.byteOffset, data.byteLength);
  const events: SyntaxHighlightEvent[] = [];
  let offset = 0;

  while (offset < data.byteLength) {
    const tag = view.getUint8(offset++);

    switch (tag) {
      case SOURCE_EVENT: {
        requireBytes(offset, SOURCE_EVENT_BYTES, data.byteLength);
        const startByte = view.getUint32(offset, true);
        const endByte = view.getUint32(offset + 4, true);
        offset += SOURCE_EVENT_BYTES;
        events.push({ type: "source", startByte, endByte });
        break;
      }
      case START_EVENT: {
        requireBytes(offset, START_EVENT_HEADER_BYTES, data.byteLength);
        const scopeIndex = view.getUint16(offset, true);
        const languageLength = view.getUint16(offset + 2, true);
        offset += START_EVENT_HEADER_BYTES;
        requireBytes(offset, languageLength, data.byteLength);

        const scope = HIGHLIGHT_NAMES[scopeIndex];
        if (scope === undefined) {
          throw new Error(`Unknown native Lumis highlight index ${scopeIndex}`);
        }

        const language = decoder.decode(data.subarray(offset, offset + languageLength));
        offset += languageLength;
        events.push({ type: "start", scope, language });
        break;
      }
      case END_EVENT:
        events.push({ type: "end" });
        break;
      default:
        throw new Error(`Unknown native Lumis event tag ${tag}`);
    }
  }

  return events;
}

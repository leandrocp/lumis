import { describe, expect, it } from "vitest";

import { decodeNativeEvents } from "../src/core/native-event-codec.js";
import { HIGHLIGHT_NAMES } from "../src/highlights.js";

describe("native event codec", () => {
  it("decodes start, source, and end events", () => {
    const encoded = new Uint8Array([
      1,
      0,
      0,
      2,
      0,
      106,
      115, // start: scope 0, language "js"
      0,
      1,
      0,
      0,
      0,
      5,
      0,
      0,
      0, // source: bytes 1..5
      2, // end
    ]);

    expect(decodeNativeEvents(encoded)).toEqual([
      { type: "start", scope: HIGHLIGHT_NAMES[0], language: "js" },
      { type: "source", start: 1, end: 5 },
      { type: "end" },
    ]);
  });

  it("honors Uint8Array offsets", () => {
    const wrapped = new Uint8Array([255, 0, 1, 0, 0, 0, 5, 0, 0, 0, 255]);

    expect(decodeNativeEvents(wrapped.subarray(1, -1))).toEqual([
      { type: "source", start: 1, end: 5 },
    ]);
  });

  it.each([new Uint8Array([0]), new Uint8Array([1]), new Uint8Array([1, 0, 0, 2, 0, 106])])(
    "rejects truncated buffers",
    (encoded) => {
      expect(() => decodeNativeEvents(encoded)).toThrow("Invalid native Lumis event buffer");
    },
  );

  it("rejects unknown event tags", () => {
    expect(() => decodeNativeEvents(new Uint8Array([3]))).toThrow(
      "Unknown native Lumis event tag 3",
    );
  });

  it("rejects unknown highlight indices", () => {
    expect(() => decodeNativeEvents(new Uint8Array([1, 255, 255, 0, 0]))).toThrow(
      "Unknown native Lumis highlight index 65535",
    );
  });
});

import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { beforeAll, describe, expect, it } from "vitest";

import { applyCaptureOffset, buildSourceMaps } from "../src/events.js";
import { compileHighlightConfig } from "../src/core/languages.js";
import { ensureLocalWasm } from "./wasm.js";

interface Endpoint {
  startRow: number;
  startColumn: number;
  startByte: number;
  endRow: number;
  endColumn: number;
  endByte: number;
}

interface Case {
  name: string;
  source: string;
  query: string;
  offset: string[];
  original: Endpoint;
  expected: Endpoint;
}

interface OperandCoercion {
  name: string;
  operand: string;
  expected: number | null;
}

const fixturePath = fileURLToPath(
  new URL("../../../../fixtures/offset-directive.json", import.meta.url),
);
const { cases, operandCoercions } = JSON.parse(readFileSync(fixturePath, "utf8")) as {
  cases: Case[];
  operandCoercions: OperandCoercion[];
};

let Query: typeof import("web-tree-sitter").Query;
let language: import("web-tree-sitter").Language;

beforeAll(async () => {
  const treeSitter = await import("web-tree-sitter");
  await treeSitter.Parser.init();
  Query = treeSitter.Query;
  language = await treeSitter.Language.load(
    new Uint8Array(readFileSync(fileURLToPath(ensureLocalWasm("json")))),
  );
}, 60_000);

/**
 * Representable ranges in the fixture came out of Neovim itself. Cases named as
 * invalid ranges pin Lumis's documented safety fallback. The Rust runtime reads
 * the same file, so either implementation drifting fails here.
 *
 * The offsets are read back through `compileHighlightConfig`, the path the
 * runtime actually uses, so breaking predicate extraction fails here too.
 */
describe("#offset! matches Neovim", () => {
  it("has a corpus", () => {
    expect(cases.length).toBeGreaterThanOrEqual(15);
    expect(operandCoercions.length).toBeGreaterThanOrEqual(24);
  });

  it.each(cases.map((entry) => [entry.name, entry] as const))("%s", (_name, entry) => {
    const config = compileHighlightConfig(language, Query, entry.query, "", "");
    const offset = Object.values(config.captureOffsets[0] ?? {})[0];
    expect(offset, "the directive must survive compilation").toBeDefined();

    const maps = buildSourceMaps(entry.source);
    const startIndex = maps.utf16Indices[entry.original.startByte];
    const endIndex = maps.utf16Indices[entry.original.endByte];
    expect(startIndex, "the original start must be a UTF-8 boundary").toBeDefined();
    expect(endIndex, "the original end must be a UTF-8 boundary").toBeDefined();
    const actual = applyCaptureOffset(
      {
        startIndex: startIndex!,
        endIndex: endIndex!,
        startPosition: {
          row: entry.original.startRow,
          column: startIndex! - maps.lineStarts[entry.original.startRow],
        },
        endPosition: {
          row: entry.original.endRow,
          column: endIndex! - maps.lineStarts[entry.original.endRow],
        },
      },
      offset,
      maps,
    );

    const actualStartByte = maps.utf8Offsets[actual.startIndex];
    const actualEndByte = maps.utf8Offsets[actual.endIndex];

    expect({
      startRow: actual.startPosition.row,
      startColumn: actualStartByte - maps.byteLineStarts[actual.startPosition.row],
      startByte: actualStartByte,
      endRow: actual.endPosition.row,
      endColumn: actualEndByte - maps.byteLineStarts[actual.endPosition.row],
      endByte: actualEndByte,
    }).toEqual(entry.expected);
  });

  // Neovim reads pred[3] through pred[6] and stops, so a fifth operand of any
  // kind is untouched. Mirrors `a_fifth_operand_never_voids_the_directive`.
  it.each([
    "((string) @cap (#offset! @cap 0 1 0 -1))",
    "((string) @cap (#offset! @cap 0 1 0 -1 99))",
    "((string) @cap (#offset! @cap 0 1 0 -1 nope))",
    "((string) @cap (#offset! @cap 0 1 0 -1 @cap))",
  ])("a fifth operand never voids %s", (query) => {
    const config = compileHighlightConfig(language, Query, query, "", "");
    expect(Object.values(config.captureOffsets[0] ?? {})[0]).toEqual({
      startRow: 0,
      startColumn: 1,
      endRow: 0,
      endColumn: -1,
    });
  });

  it.each(operandCoercions.map((entry) => [entry.name, entry] as const))(
    "coerces %s like Neovim where Tree-sitter points can represent it",
    (_name, entry) => {
      const operand = JSON.stringify(entry.operand);
      const config = compileHighlightConfig(
        language,
        Query,
        `((string) @cap (#offset! @cap 0 ${operand} 0 -1))`,
        "",
        "",
      );
      const offset = Object.values(config.captureOffsets[0] ?? {})[0];

      if (entry.expected == null) {
        // oxlint-disable-next-line vitest/no-conditional-expect -- each corpus row asserts either an absent directive or its expected offset.
        expect(offset).toBeUndefined();
      } else {
        // oxlint-disable-next-line vitest/no-conditional-expect -- each corpus row asserts either an absent directive or its expected offset.
        expect(offset?.startColumn).toBe(entry.expected);
      }
    },
  );

  it.each([
    ["start", '0 "99999999999" 0 -1'],
    ["end", '0 1 0 "99999999999"'],
  ])(
    "discards a wide adjusted %s point instead of constructing an invalid range",
    (_name, operands) => {
      const config = compileHighlightConfig(
        language,
        Query,
        `((string) @cap (#offset! @cap ${operands}))`,
        "",
        "",
      );
      const offset = Object.values(config.captureOffsets[0] ?? {})[0];
      const source = '"abc"';
      const original = {
        startIndex: 0,
        endIndex: source.length,
        startPosition: { row: 0, column: 0 },
        endPosition: { row: 0, column: source.length },
      };

      expect(applyCaptureOffset(original, offset, buildSourceMaps(source))).toEqual(original);
    },
  );

  it("reaches its own newline on a same-row offset", () => {
    const source = "a\nb";
    const original = {
      startIndex: 0,
      endIndex: 1,
      startPosition: { row: 0, column: 0 },
      endPosition: { row: 0, column: 1 },
    };

    expect(
      applyCaptureOffset(
        original,
        { startRow: 0, startColumn: 0, endRow: 0, endColumn: 1 },
        buildSourceMaps(source),
      ),
    ).toEqual({
      startIndex: 0,
      endIndex: 2,
      startPosition: { row: 0, column: 0 },
      endPosition: { row: 0, column: 2 },
    });
  });

  // Neovim raises a Lua error rather than defining a behaviour for a literal
  // that cannot coerce to a number. Capture operands are numeric IDs and are
  // covered by the shared range fixture above.
  it("discards a literal whose slot is not a number", () => {
    const query = "((string) @cap (#offset! @cap 0 nope 0 -1))";
    const config = compileHighlightConfig(language, Query, query, "", "");
    expect(config.captureOffsets[0]).toBeUndefined();
  });
});

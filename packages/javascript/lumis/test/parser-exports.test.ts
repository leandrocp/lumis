/**
 * The grammar check reads a parser's exports without compiling it. This pins
 * that reader to what the platform reports for every committed parser.
 */
import { readdirSync, readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { exportedFunctionNames } from "../src/core/languages.js";

const parsers = new URL("../../../../fixtures/test-parsers/", import.meta.url);
const files = readdirSync(parsers)
  .filter((name) => name.endsWith(".wasm"))
  .sort();

function parser(file: string): Uint8Array {
  return new Uint8Array(readFileSync(new URL(file, parsers)));
}

describe("exportedFunctionNames", () => {
  it("finds the committed parsers", () => {
    expect(files.length).toBeGreaterThanOrEqual(17);
  });

  it.each(files)("reads the functions WebAssembly.Module.exports reports for %s", (file) => {
    const bytes = parser(file);
    const compiled = WebAssembly.Module.exports(new WebAssembly.Module(bytes))
      .filter(({ kind }) => kind === "function")
      .map(({ name }) => name);

    expect(exportedFunctionNames(bytes)).toEqual(compiled);
  });

  it("throws on bytes that are not a module, or that end inside a section", () => {
    expect(() => exportedFunctionNames(new TextEncoder().encode("not wasm"))).toThrow(
      "not a WebAssembly module",
    );
    expect(() => exportedFunctionNames(parser("tree-sitter-json.wasm").subarray(0, 12))).toThrow(
      "truncated",
    );
  });
});

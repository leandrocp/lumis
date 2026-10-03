import { padded } from "../padded-parser.js";
/**
 * A parser over 8 MB, loaded on the main thread. Chrome refuses a synchronous
 * compile that large there, so Lumis has to read the parser's grammar without
 * one, as fsharp, nim and systemverilog need.
 */
import { lowestCompatibleLanguagePackageVersion } from "../../src/core/languages.ts";
import { htmlLinked } from "../../src/formatters.ts";
import { createHighlighter } from "../../src/index.browser.ts";

export interface LargeParserOutcome {
  bytes?: number;
  html?: string;
  error?: string;
}

declare global {
  interface Window {
    __lumisLargeParser?: LargeParserOutcome;
  }
}

const [parserUrl] = Object.values(
  import.meta.glob<string>("../../../../../fixtures/test-parsers/tree-sitter-json.wasm", {
    eager: true,
    query: "?url",
    import: "default",
  }),
);
const [highlights] = Object.values(
  import.meta.glob<string>("../../../../../queries/processed/json/highlights.scm", {
    eager: true,
    query: "?raw",
    import: "default",
  }),
);

try {
  const parser = new Uint8Array(await (await fetch(parserUrl!)).arrayBuffer());
  const wasm = padded(parser, 9 * 1024 * 1024);
  const highlighter = await createHighlighter({
    languages: [
      {
        id: "json",
        aliases: [],
        packageName: "@lumis-sh/wasm-json",
        wasm,
        languagePackage: {
          packageName: "@lumis-sh/wasm-json",
          version: lowestCompatibleLanguagePackageVersion(),
          parser: { name: "tree-sitter-json", grammarName: "json", sha256: "0".repeat(64) },
          languages: { json: { aliases: [], highlights } },
        },
      },
    ],
  });
  window.__lumisLargeParser = {
    bytes: wasm.byteLength,
    html: highlighter.highlight('{"answer": 42}', htmlLinked({ language: "json" })),
  };
} catch (error) {
  window.__lumisLargeParser = { error: String(error) };
}

import { highlight } from "../../dist/index.browser.js";
import { htmlInline } from "../../dist/formatters.js";
import json from "../../dist/langs/json.js";
import dracula from "../../../../../themes/dracula.json";

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

const language = {
  ...json,
  wasm: new Uint8Array(await (await fetch(parserUrl!)).arrayBuffer()),
  languagePackage: {
    packageName: "@lumis-sh/wasm-json",
    version: "0.26.0",
    parser: { name: "tree-sitter-json", grammarName: "json", sha256: "0".repeat(64) },
    languages: { json: { aliases: [], highlights } },
  },
};

document.querySelector("#output")!.innerHTML = await highlight(
  '{"answer": 42}',
  htmlInline({ language, theme: dracula }),
);

import assert from "node:assert/strict";
import { mkdir, writeFile } from "node:fs/promises";
import { createHighlighter, runtimeKind } from "../../packages/javascript/lumis/src/index.ts";
import rust from "../../packages/javascript/lumis/langs/rust.ts";
import dracula from "../../packages/javascript/themes/themes/dracula.ts";
import {
  configureLocalWasmResolver,
  localLanguagePackageMetadata,
} from "../../packages/javascript/lumis/test/wasm.ts";
import { renderCases } from "./render.mjs";

configureLocalWasmResolver(["rust"]);
const highlighter = await createHighlighter({ languages: [rust] });
assert.equal(runtimeKind(), process.env.LUMIS_TEST_RUNTIME);
const output = new URL("../../target/html-line-qa/", import.meta.url);
await mkdir(output, { recursive: true });
await writeFile(
  new URL(`node-${runtimeKind()}.json`, output),
  JSON.stringify(renderCases(highlighter, rust, dracula)),
);
await writeFile(
  new URL("language.json", output),
  JSON.stringify(localLanguagePackageMetadata("@lumis-sh/wasm-rust")),
);
console.log(`Rendered Node ${runtimeKind()} QA cases`);

/**
 * Cloudflare Workers compile no WebAssembly at run time: a `.wasm` import is
 * already a `WebAssembly.Module`, and compiling bytes throws. These run what
 * the `workerd` entry does with every byte-compiling API refusing, as workerd's
 * do, so a code path that compiles bytes fails here rather than in a deploy.
 */
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { localPackageLanguage } from "./wasm.js";

const require = createRequire(import.meta.url);
const runtimeModule = new WebAssembly.Module(
  readFileSync(require.resolve("web-tree-sitter/web-tree-sitter.wasm")),
);

function precompiled(language: ReturnType<typeof localPackageLanguage>) {
  return {
    ...language,
    wasm: new WebAssembly.Module(language.wasm),
    requires: language.requires?.map(precompiled),
  };
}

const markdown = precompiled(
  localPackageLanguage("markdown", [localPackageLanguage("markdown_inline")]),
);
const json = precompiled(localPackageLanguage("json"));

const { Module, instantiate } = WebAssembly;
const refused = () => {
  throw new WebAssembly.CompileError("Wasm code generation disallowed by embedder");
};

beforeEach(() => {
  vi.stubGlobal(
    "WebAssembly",
    Object.create(WebAssembly, {
      Module: { value: new Proxy(Module, { construct: refused }) },
      compile: { value: refused },
      compileStreaming: { value: refused },
      instantiateStreaming: { value: refused },
      instantiate: {
        value: (source: unknown, imports?: WebAssembly.Imports) =>
          source instanceof Module ? instantiate(source, imports) : refused(),
      },
    }),
  );
});

afterEach(() => {
  vi.unstubAllGlobals();
});

it("highlights from precompiled modules alone, injections included", async () => {
  const { setRuntimeModule } = await import("../src/runtime/browser.js");
  const { createHighlighter } = await import("../src/index.browser.js");
  const { htmlLinked } = await import("../src/formatters.js");

  setRuntimeModule(runtimeModule);
  const hl = await createHighlighter({ languages: [markdown, json] });
  const html = hl.highlight(
    '# Title\n\n```json\n{"a": 1}\n```\n',
    htmlLinked({ language: "markdown" }),
  );

  expect(html).toContain('class="l-markup-heading');
  expect(html).toContain('class="l-number"');
});

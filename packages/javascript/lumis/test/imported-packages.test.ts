/**
 * A browser has no project to read, so it loads only the parser packages
 * imported into it. These run the browser entry with `fetch` replaced, so a
 * request to a CDN is something the test sees rather than something that
 * quietly succeeds.
 */
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { createHighlighter, withWasm } from "../src/index.browser.js";
import { htmlLinked } from "../src/formatters.js";
import jsonHandle from "../langs/json.ts";
import { localPackageLanguage } from "./wasm.js";

// What `import json from "@lumis-sh/wasm-json"` gives a bundler.
const json = localPackageLanguage("json");
let requests: string[];

beforeEach(() => {
  requests = [];
  vi.stubGlobal("fetch", async (input: RequestInfo | URL) => {
    const url = String(input instanceof Request ? input.url : input);
    requests.push(url);
    if (url.endsWith("/lumis.json")) return Response.json(json.manifest);
    if (url.endsWith(".wasm")) return new Response(json.wasm);
    return new Response(null, { status: 404 });
  });
});

afterEach(() => {
  vi.unstubAllGlobals();
});

describe("a browser", () => {
  it("loads a language a package exports and fetches nothing", async () => {
    const hl = await createHighlighter({ languages: [json] });

    expect(hl.highlight('{"a": "b"}', htmlLinked({ language: json }))).toContain(
      'class="l-string"',
    );
    expect(requests).toEqual([]);
  });

  // `markdown` names `markdown_inline` in `requires`, as `@lumis-sh/wasm-markdown`
  // does, so importing one package is enough.
  it("loads the grammars a language requires along with it", async () => {
    const markdown = localPackageLanguage("markdown", [localPackageLanguage("markdown_inline")]);
    const hl = await createHighlighter({ languages: [markdown] });

    expect(hl.languages).toEqual(expect.arrayContaining(["markdown", "markdown_inline"]));
    expect(requests).toEqual([]);
  });

  // What `import web from "@lumis-sh/wasm-bundle-web"` gives a bundler: each
  // entry imports its own package the first time it loads.
  it("loads a bundle entry by name", async () => {
    const bundle = { json: Object.assign(async () => json, { id: "json", aliases: [] }) };
    const hl = await createHighlighter({ languages: [bundle] });

    await hl.loadLanguage("json");

    expect(hl.languages).toContain("json");
    expect(requests).toEqual([]);
  });

  it("refuses a language whose package was not imported", async () => {
    const hl = await createHighlighter({ languages: [] });

    await expect(hl.loadLanguage(jsonHandle)).rejects.toThrow(
      "@lumis-sh/wasm-json was not imported",
    );
    expect(requests).toEqual([]);
  });

  // Parser bytes alone carry no queries, and fetching them from a CDN is what
  // this rule rules out.
  it("asks for the package when given only its parser", async () => {
    const hl = await createHighlighter({ languages: [] });

    await expect(hl.loadLanguage(withWasm(jsonHandle, json.wasm))).rejects.toThrow(
      "pass the whole package to withWasm()",
    );
    expect(requests).toEqual([]);
  });

  // Code written as `withWasm(json, jsonWasm)` against the old default export,
  // or with the package imported as a namespace, keeps working.
  it("still takes a package through withWasm()", async () => {
    const fromLanguage = await createHighlighter({ languages: [withWasm(jsonHandle, json)] });
    const fromNamespace = await createHighlighter({
      languages: [withWasm(jsonHandle, { default: json, manifest: json.manifest })],
    });

    expect(fromLanguage.languages).toContain("json");
    expect(fromNamespace.languages).toContain("json");
    expect(requests).toEqual([]);
  });

  it("fetches through a resolver the caller configured", async () => {
    const hl = await createHighlighter({
      languages: [jsonHandle],
      languagePackageResolver: (packageName) => `https://cdn.test/${packageName}/lumis.json`,
      wasmResolver: (_language, wasm) => `https://cdn.test/${wasm.packageName}/${wasm.name}.wasm`,
    });

    expect(hl.highlight('{"a": "b"}', htmlLinked({ language: "json" }))).toContain(
      'class="l-string"',
    );
    expect(requests).toEqual([
      "https://cdn.test/@lumis-sh/wasm-json/lumis.json",
      "https://cdn.test/@lumis-sh/wasm-json/tree-sitter-json.wasm",
    ]);
  });
});

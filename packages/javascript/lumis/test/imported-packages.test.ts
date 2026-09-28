/**
 * A browser has no project to read, so it loads only the parser packages
 * imported into it. These run the browser entry with `fetch` replaced, so a
 * request to a CDN is something the test sees rather than something that
 * quietly succeeds.
 */
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { createHighlighter, withWasm } from "../src/index.browser.js";
import { htmlLinked } from "../src/formatters.js";
import json from "../langs/json.ts";
import { localLanguagePackageExports } from "./wasm.js";

const jsonPackage = localLanguagePackageExports("json");
let requests: string[];

beforeEach(() => {
  requests = [];
  vi.stubGlobal("fetch", async (input: RequestInfo | URL) => {
    const url = String(input instanceof Request ? input.url : input);
    requests.push(url);
    if (url.endsWith("/lumis.json")) return Response.json(jsonPackage.manifest);
    if (url.endsWith(".wasm")) return new Response(jsonPackage.default);
    return new Response(null, { status: 404 });
  });
});

afterEach(() => {
  vi.unstubAllGlobals();
});

describe("a browser", () => {
  it("loads a language from its imported package and fetches nothing", async () => {
    const hl = await createHighlighter({ languages: [withWasm(json, jsonPackage)] });

    expect(hl.highlight('{"a": "b"}', htmlLinked({ language: "json" }))).toContain(
      'class="l-string"',
    );
    expect(requests).toEqual([]);
  });

  it("refuses a language whose package was not imported", async () => {
    const hl = await createHighlighter({ languages: [] });

    await expect(hl.loadLanguage(json)).rejects.toThrow("@lumis-sh/wasm-json was not imported");
    expect(requests).toEqual([]);
  });

  // Parser bytes alone carry no queries, and fetching them from a CDN is what
  // this rule rules out.
  it("asks for the whole package when given only its parser", async () => {
    const hl = await createHighlighter({ languages: [] });

    await expect(hl.loadLanguage(withWasm(json, jsonPackage.default))).rejects.toThrow(
      "pass the whole package to withWasm()",
    );
    expect(requests).toEqual([]);
  });

  it("fetches through a resolver the caller configured", async () => {
    const hl = await createHighlighter({
      languages: [json],
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

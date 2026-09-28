import { mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import type { RuntimeWasmInput } from "../src/types.js";
import {
  ensureLocalParserWasm,
  ensureLocalWasm,
  localLanguagePackageMetadata,
  localLanguagePackageResolver,
} from "./wasm.js";

const CACHE_DIR = ".tmp/wasm-resolver-cache";
process.env.LUMIS_DATA_DIR = CACHE_DIR;

beforeEach(async () => {
  // Clear FS cache so the resolver is always called
  try {
    rmSync(CACHE_DIR, { recursive: true });
  } catch {}
  const { configureLanguagePackageResolver } = await import("../src/index.js");
  configureLanguagePackageResolver(localLanguagePackageResolver);
});

afterEach(() => {
  vi.restoreAllMocks();
  vi.resetModules();
});

describe("Wasm resolver", () => {
  it("rejects incomplete language package metadata", async () => {
    const { parseLanguagePackage } = await import("../src/core/languages.js");
    const packageName = "@lumis-sh/wasm-invalid";
    const bytes = new TextEncoder().encode(
      JSON.stringify({
        packageName,
        version: "0.26.0",
        definitionHash: "definition",
        parser: {
          name: "tree-sitter-invalid",
          grammarName: "invalid",
          sha256: "digest",
          size: 1,
        },
        languages: {
          invalid: { aliases: [], highlights: 42 },
        },
      }),
    );

    expect(() => parseLanguagePackage(bytes, packageName)).toThrow(
      "Invalid Lumis language package",
    );
  });

  it("rejects language package metadata that escapes cache paths", async () => {
    const { parseLanguagePackage } = await import("../src/core/languages.js");
    const packageName = "@lumis-sh/wasm-diff";
    const metadata = structuredClone(localLanguagePackageMetadata(packageName));
    metadata.parser.name = "../tree-sitter-diff";

    expect(() =>
      parseLanguagePackage(new TextEncoder().encode(JSON.stringify(metadata)), packageName),
    ).toThrow("Invalid Lumis language package");
  });

  it("keeps exact parser metadata in the language package", async () => {
    const { default: diff } = await import("../langs/diff.ts");
    const packageMetadata = localLanguagePackageMetadata(diff.packageName);

    expect(diff).toEqual({
      id: "diff",
      aliases: [],
      packageName: "@lumis-sh/wasm-diff",
    });
    expect(packageMetadata.parser).toEqual({
      name: "tree-sitter-diff",
      grammarName: "diff",
      sha256: expect.stringMatching(/^[0-9a-f]{64}$/),
      size: expect.any(Number),
    });
  });

  it("resolves the shared compatible package range", async () => {
    const { DEFAULT_LANGUAGE_PACKAGE_RESOLVER } = await import("../src/core/languages.js");
    const { LANGUAGE_PACKAGE_VERSION_RANGE } =
      await import("../src/generated/package-version-range.js");

    expect(
      DEFAULT_LANGUAGE_PACKAGE_RESOLVER("@lumis-sh/wasm-diff", LANGUAGE_PACKAGE_VERSION_RANGE),
    ).toBe(
      `https://cdn.jsdelivr.net/npm/@lumis-sh/wasm-diff@${LANGUAGE_PACKAGE_VERSION_RANGE}/lumis.json`,
    );
  });

  it("rejects an incompatible package returned by a custom resolver", async () => {
    const { createHighlighter } = await import("../src/index.js");
    const { default: diff } = await import("../langs/diff.ts");
    const packageMetadata = structuredClone(localLanguagePackageMetadata(diff.packageName));
    packageMetadata.version = "0.27.0";
    const dataUrl = `data:application/json;base64,${Buffer.from(
      JSON.stringify(packageMetadata),
    ).toString("base64")}`;

    await expect(
      createHighlighter({
        languages: [diff],
        languagePackageResolver: () => dataUrl,
        wasmResolver: (language, wasm) => ensureLocalParserWasm(language, wasm.name),
      }),
    ).rejects.toThrow("does not satisfy the supported range");
  });

  it("loads WasmRef bundles from file:// URLs in Node without fetch", async () => {
    const { createHighlighter } = await import("../src/index.js");
    const { htmlLinked } = await import("../src/formatters.js");
    const { default: diff } = await import("../langs/diff.ts");

    const hl = await createHighlighter({
      languages: [{ ...diff, wasm: ensureLocalWasm("diff") }],
      wasmResolver: (language, wasm) => ensureLocalParserWasm(language, wasm.name),
    });

    const html = hl.highlight("- old\n+ new", htmlLinked({ language: diff }));
    expect(html).toContain('class="language-diff"');
  }, 30_000);

  it("supports withWasm for explicit runtime wasm inputs", async () => {
    const { createHighlighter, withWasm } = await import("../src/index.js");
    const { htmlLinked } = await import("../src/formatters.js");
    const { default: diff } = await import("../langs/diff.ts");

    const language = withWasm(diff, ensureLocalWasm("diff"));
    const hl = await createHighlighter({
      languages: [language],
      wasmResolver: (requested, wasm) => ensureLocalParserWasm(requested, wasm.name),
    });

    const html = hl.highlight("- old\n+ new", htmlLinked({ language }));
    expect(html).toContain('class="language-diff"');
  }, 30_000);

  it("verifies withWasm bytes against the language package paired with them", async () => {
    const { createHighlighter, withWasm } = await import("../src/index.js");
    const { htmlLinked } = await import("../src/formatters.js");
    const { default: diff } = await import("../langs/diff.ts");

    // The range resolves to a newer patch whose parser and queries both differ
    // from the bytes the app bundled, as on the CDN after a parser release.
    const languagePackage = localLanguagePackageMetadata(diff.packageName);
    const newer = structuredClone(languagePackage);
    newer.version = "0.26.1";
    newer.parser.sha256 = "0".repeat(64);
    newer.parser.size += 33;
    newer.languages.diff.highlights = "(";
    const newerUrl = `data:application/json;base64,${Buffer.from(JSON.stringify(newer)).toString("base64")}`;
    const wasm = new Uint8Array(readFileSync(fileURLToPath(ensureLocalWasm("diff"))));

    await expect(
      createHighlighter({
        languages: [withWasm(diff, wasm)],
        languagePackageResolver: () => newerUrl,
      }),
    ).rejects.toThrow("Invalid WASM size");
    await expect(
      createHighlighter({
        languages: [withWasm(diff, { wasm, languagePackage: newer })],
        languagePackageResolver: () => newerUrl,
      }),
    ).rejects.toThrow("Invalid WASM size");

    const language = withWasm(diff, { wasm, languagePackage });
    const hl = await createHighlighter({
      languages: [language],
      languagePackageResolver: () => newerUrl,
    });

    expect(hl.highlight("- old\n+ new", htmlLinked({ language }))).toContain(
      'class="language-diff"',
    );
  }, 30_000);

  it("accepts wasmResolver in createHighlighter options", async () => {
    const { createHighlighter } = await import("../src/index.js");
    const { htmlLinked } = await import("../src/formatters.js");
    const { default: diff } = await import("../langs/diff.ts");

    const resolver = (_language: string, wasm: { name: string }) =>
      ensureLocalParserWasm(_language, wasm.name);

    const hl = await createHighlighter({
      languages: [diff],
      wasmResolver: resolver,
    });

    const html = hl.highlight("- old\n+ new", htmlLinked({ language: diff }));
    expect(html).toContain('class="language-diff"');
  }, 30_000);

  it("shares verified parser bytes across highlighter instances", async () => {
    const { createHighlighter } = await import("../src/index.js");
    const { default: diff } = await import("../langs/diff.ts");
    const resolver = vi.fn<(language: string, wasm: { name: string }) => URL>((language, wasm) =>
      ensureLocalParserWasm(language, wasm.name),
    );

    await createHighlighter({ languages: [diff], wasmResolver: resolver });
    await createHighlighter({ languages: [diff], wasmResolver: resolver });

    expect(resolver).toHaveBeenCalledTimes(1);
  }, 30_000);

  it("replaces a corrupted persistent cache entry", async () => {
    const { createHighlighter } = await import("../src/index.js");
    const { htmlLinked } = await import("../src/formatters.js");
    const { default: diff } = await import("../langs/diff.ts");
    const packageMetadata = localLanguagePackageMetadata(diff.packageName);
    const key = `${packageMetadata.parser.name}-${packageMetadata.version}-${packageMetadata.parser.sha256}`;
    const parsers = join(CACHE_DIR, "parsers");
    const cacheFile = join(parsers, `${encodeURIComponent(key)}.wasm`);
    mkdirSync(parsers, { recursive: true });
    writeFileSync(cacheFile, new Uint8Array([0, 1, 2, 3]));

    const hl = await createHighlighter({
      languages: [diff],
      wasmResolver: (language, wasm) => ensureLocalParserWasm(language, wasm.name),
    });

    expect(hl.highlight("- old\n+ new", htmlLinked({ language: diff }))).toContain(
      'class="language-diff"',
    );
    expect(readFileSync(cacheFile).byteLength).toBe(packageMetadata.parser.size);
  }, 30_000);

  it("rejects parser bytes that do not match the exact package entry", async () => {
    const { createHighlighter } = await import("../src/index.js");
    const { default: diff } = await import("../langs/diff.ts");

    await expect(
      createHighlighter({
        languages: [diff],
        wasmResolver: (language) => ensureLocalParserWasm(language, "tree-sitter-html"),
      }),
    ).rejects.toThrow(/Invalid WASM (size|integrity)/);
  }, 30_000);

  it.each(["Uint8Array", "ArrayBuffer", "string", "URL", "Response"] as const)(
    "verifies an explicit %s parser against the package entry",
    async (kind) => {
      const { createHighlighter } = await import("../src/index.js");
      const { default: json } = await import("../langs/json.ts");
      const url = ensureLocalWasm("markdown");
      const bytes = new Uint8Array(readFileSync(url));
      const inputs: Record<typeof kind, RuntimeWasmInput> = {
        Uint8Array: bytes,
        ArrayBuffer: bytes.buffer,
        string: fileURLToPath(url),
        URL: url,
        Response: new Response(bytes),
      };

      await expect(
        createHighlighter({ languages: [{ ...json, wasm: inputs[kind] }] }),
      ).rejects.toThrow(/Invalid WASM (size|integrity)/);
    },
    30_000,
  );

  it("per-instance resolver is isolated from global resolver", async () => {
    const { createHighlighter, configureWasmResolver } = await import("../src/index.js");
    const { htmlLinked } = await import("../src/formatters.js");
    const { default: diff } = await import("../langs/diff.ts");

    const globalCalls: string[] = [];
    const instanceCalls: string[] = [];

    configureWasmResolver((language, wasm) => {
      globalCalls.push(language);
      return ensureLocalParserWasm(language, wasm.name);
    });

    const hl = await createHighlighter({
      languages: [diff],
      wasmResolver: (language, wasm) => {
        instanceCalls.push(language);
        return ensureLocalParserWasm(language, wasm.name);
      },
    });

    const html = hl.highlight("- old\n+ new", htmlLinked({ language: diff }));
    expect(html).toContain('class="language-diff"');
    expect(instanceCalls.length).toBeGreaterThan(0);
    expect(globalCalls).not.toContain("diff");
  }, 30_000);

  it("global configureWasmResolver applies to createHighlighter without explicit resolver", async () => {
    const { createHighlighter, configureWasmResolver } = await import("../src/index.js");
    const { htmlLinked } = await import("../src/formatters.js");
    const { default: diff } = await import("../langs/diff.ts");

    const calls: string[] = [];
    configureWasmResolver((language, wasm) => {
      calls.push(language);
      return ensureLocalParserWasm(language, wasm.name);
    });

    const hl = await createHighlighter({ languages: [diff] });
    const html = hl.highlight("- old\n+ new", htmlLinked({ language: diff }));

    expect(html).toContain('class="language-diff"');
    expect(calls).toContain("diff");
  }, 30_000);

  it("configureWasmResolver called after createHighlighter affects highlighters without explicit resolver", async () => {
    const { createHighlighter, configureWasmResolver } = await import("../src/index.js");
    const { default: html } = await import("../langs/html.ts");

    configureWasmResolver((language, wasm) => ensureLocalParserWasm(language, wasm.name));
    const hl = await createHighlighter();

    const calls: string[] = [];
    configureWasmResolver((language, wasm) => {
      calls.push(language);
      return ensureLocalParserWasm(language, wasm.name);
    });

    await hl.loadLanguage(html);

    expect(calls).toContain("html");
  }, 30_000);
});

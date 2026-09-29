import { createHash } from "node:crypto";
import { once } from "node:events";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { createServer } from "node:http";
import type { AddressInfo } from "node:net";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
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
        parser: {
          name: "tree-sitter-invalid",
          grammarName: "invalid",
          sha256: "digest",
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
    expect(createHash("sha256").update(readFileSync(cacheFile)).digest("hex")).toBe(
      packageMetadata.parser.sha256,
    );
  }, 30_000);

  it("rejects parser bytes that do not match the exact package entry", async () => {
    const { createHighlighter } = await import("../src/index.js");
    const { default: diff } = await import("../langs/diff.ts");

    await expect(
      createHighlighter({
        languages: [diff],
        wasmResolver: (language) => ensureLocalParserWasm(language, "tree-sitter-html"),
      }),
    ).rejects.toThrow(/Invalid WASM integrity/);
  }, 30_000);

  // Bytes handed over directly are the caller's to vouch for, so another build
  // of the same grammar loads with the package's queries.
  it.each(["Uint8Array", "ArrayBuffer", "string", "URL", "Response"] as const)(
    "uses an explicit %s parser as it is handed over",
    async (kind) => {
      const { createHighlighter } = await import("../src/index.js");
      const { htmlLinked } = await import("../src/formatters.js");
      const { default: json } = await import("../langs/json.ts");
      const bytes = anotherBuild(new Uint8Array(readFileSync(ensureLocalWasm("json"))));
      const file = join(mkdtempSync(join(tmpdir(), "lumis-explicit-")), "tree-sitter-json.wasm");
      writeFileSync(file, bytes);
      const inputs: Record<typeof kind, RuntimeWasmInput> = {
        Uint8Array: bytes,
        ArrayBuffer: bytes.buffer,
        string: file,
        URL: pathToFileURL(file),
        Response: new Response(bytes),
      };

      const hl = await createHighlighter({ languages: [{ ...json, wasm: inputs[kind] }] });

      expect(hl.highlight('{"answer": 42}', htmlLinked({ language: json }))).toContain(
        'class="l-number"',
      );
    },
    30_000,
  );

  it("checks an explicit parser it downloads against the package entry", async () => {
    const { createHighlighter } = await import("../src/index.js");
    const { default: json } = await import("../langs/json.ts");
    const bytes = anotherBuild(new Uint8Array(readFileSync(ensureLocalWasm("json"))));
    const server = createServer((_request, response) => response.end(bytes));
    server.listen(0, "127.0.0.1");
    await once(server, "listening");

    try {
      const { port } = server.address() as AddressInfo;
      await expect(
        createHighlighter({
          languages: [{ ...json, wasm: `http://127.0.0.1:${port}/tree-sitter-json.wasm` }],
        }),
      ).rejects.toThrow(/Invalid WASM integrity/);
    } finally {
      server.close();
    }
  }, 30_000);

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

/** The same parser with an empty custom section appended: another build of one grammar. */
function anotherBuild(parser: Uint8Array): Uint8Array {
  // Section id 0, 5 bytes long, holding only the name "lumi".
  const section = [0x00, 0x05, 0x04, 0x6c, 0x75, 0x6d, 0x69];
  const bytes = new Uint8Array(parser.byteLength + section.length);
  bytes.set(parser);
  bytes.set(section, parser.byteLength);
  return bytes;
}

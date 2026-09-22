import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, dirname } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
import { downloadLanguages, expandBundles } from "../src/cache.js";
import { cacheKey } from "../src/core/languages.js";
import { wasmCachePath } from "../src/runtime/node-cache.js";
import {
  ensureLocalParserWasm,
  localLanguagePackageMetadata,
  localLanguagePackageResolver,
} from "./wasm.js";

const temporaryDirectories: string[] = [];

afterEach(async () => {
  const { rm } = await import("node:fs/promises");
  await Promise.all(
    temporaryDirectories.splice(0).map((directory) => rm(directory, { recursive: true })),
  );
  vi.restoreAllMocks();
});

async function temporaryDirectory(): Promise<string> {
  const { mkdtemp } = await import("node:fs/promises");
  const { tmpdir } = await import("node:os");
  const directory = await mkdtemp(join(tmpdir(), "lumis-js-cache-"));
  temporaryDirectories.push(directory);
  return directory;
}

describe("downloadLanguages", () => {
  it("persists verified parsers and reuses them without resolving again", async () => {
    const directory = await temporaryDirectory();
    const resolver = (_language: string, wasm: { name: string }) =>
      ensureLocalParserWasm(_language, wasm.name);

    const first = await downloadLanguages(["diff", "plaintext"], {
      directory,
      resolver,
      languagePackageResolver: localLanguagePackageResolver,
    });
    expect(first).toHaveLength(1);
    expect(first[0]).toMatchObject({ language: "diff", downloaded: true });

    const unavailableWasm = vi.fn<() => never>(() => {
      throw new Error("WASM resolver must not run");
    });
    const unavailablePackage = vi.fn<() => never>(() => {
      throw new Error("package resolver must not run");
    });
    const second = await downloadLanguages(["diff"], {
      directory,
      resolver: unavailableWasm,
      languagePackageResolver: unavailablePackage,
    });

    expect(second[0]).toMatchObject({ language: "diff", downloaded: false });
    expect(unavailableWasm).not.toHaveBeenCalled();
    expect(unavailablePackage).not.toHaveBeenCalled();
    expect(readFileSync(second[0].path).byteLength).toBeGreaterThan(0);
  });

  it("resolves package metadata again when forced", async () => {
    const directory = await temporaryDirectory();
    const resolver = (_language: string, wasm: { name: string }) =>
      ensureLocalParserWasm(_language, wasm.name);
    await downloadLanguages(["diff"], {
      directory,
      resolver,
      languagePackageResolver: localLanguagePackageResolver,
    });
    const packageResolver = vi
      .fn<typeof localLanguagePackageResolver>()
      .mockImplementation(localLanguagePackageResolver);

    await downloadLanguages(["diff"], {
      directory,
      force: true,
      resolver,
      languagePackageResolver: packageResolver,
    });

    expect(packageResolver).toHaveBeenCalledOnce();
  });

  // Mirrors `a_failed_forced_refresh_preserves_the_previous_offline_cache` in
  // crates/lumis-wasm-runtime. `parsers/<suffix>.lumis.json` is the file the Rust
  // store reads, so replacing it before its parser is cached breaks the CLI and
  // Elixir too, not only this runtime.
  it("preserves the previous offline cache when a forced refresh fails", async () => {
    const directory = await temporaryDirectory();
    const resolver = (language: string, wasm: { name: string }) =>
      ensureLocalParserWasm(language, wasm.name);
    const first = await downloadLanguages(["diff"], {
      directory,
      resolver,
      languagePackageResolver: localLanguagePackageResolver,
    });
    const cachedVersion = first[0].wasm.version;
    const shared = join(directory, "parsers", "diff.lumis.json");
    expect(JSON.parse(readFileSync(shared, "utf8")).version).toBe(cachedVersion);

    const newer = structuredClone(localLanguagePackageMetadata("@lumis-sh/wasm-diff"));
    newer.version = "0.26.9999";
    const dataUrl = `data:application/json;base64,${Buffer.from(JSON.stringify(newer)).toString(
      "base64",
    )}`;

    await expect(
      downloadLanguages(["diff"], {
        directory,
        force: true,
        resolver: () => {
          throw new Error("parser unavailable");
        },
        languagePackageResolver: () => dataUrl,
      }),
    ).rejects.toThrow("parser unavailable");

    expect(JSON.parse(readFileSync(shared, "utf8")).version).toBe(cachedVersion);

    const offline = await downloadLanguages(["diff"], {
      directory,
      resolver: () => {
        throw new Error("the store must still be complete offline");
      },
      languagePackageResolver: () => {
        throw new Error("the store must still be complete offline");
      },
    });
    expect(offline[0]).toMatchObject({ language: "diff", downloaded: false });
  });

  it("rejects an incompatible package returned by a custom resolver", async () => {
    const directory = await temporaryDirectory();
    const packageMetadata = structuredClone(localLanguagePackageMetadata("@lumis-sh/wasm-diff"));
    packageMetadata.version = "0.27.0";
    const dataUrl = `data:application/json;base64,${Buffer.from(
      JSON.stringify(packageMetadata),
    ).toString("base64")}`;

    await expect(
      downloadLanguages(["diff"], {
        directory,
        resolver: (_language, wasm) => ensureLocalParserWasm(_language, wasm.name),
        languagePackageResolver: () => dataUrl,
      }),
    ).rejects.toThrow("does not satisfy the supported range");
  });

  it("replaces corrupt persistent bytes", async () => {
    const directory = await temporaryDirectory();
    const packageMetadata = localLanguagePackageMetadata("@lumis-sh/wasm-diff");
    const ref = {
      packageName: packageMetadata.packageName,
      name: packageMetadata.parser.name,
      version: packageMetadata.version,
      sha256: packageMetadata.parser.sha256,
      size: packageMetadata.parser.size,
    };
    const key = cacheKey(ref);
    const cacheFile = await wasmCachePath(key, directory);
    mkdirSync(dirname(cacheFile), { recursive: true });
    writeFileSync(cacheFile, "corrupt");

    const result = await downloadLanguages(["diff"], {
      directory,
      resolver: (_language, wasm) => ensureLocalParserWasm(_language, wasm.name),
      languagePackageResolver: localLanguagePackageResolver,
    });

    expect(result[0]).toMatchObject({ downloaded: true });
    expect(readFileSync(cacheFile).byteLength).toBe(ref.size);
  });

  // A real child process, not `vi.resetModules()`. The native addon builds its
  // store once per process, so re-importing the JavaScript is not a restart for
  // it, and the assertion would pass for the wrong reason.
  it("loads a cached parser after a runtime restart without the network", async () => {
    const directory = await temporaryDirectory();
    await downloadLanguages(["diff"], {
      directory,
      resolver: (_language, wasm) => ensureLocalParserWasm(_language, wasm.name),
      languagePackageResolver: localLanguagePackageResolver,
    });

    const script = `
      globalThis.fetch = () => { throw new Error("network disabled"); };
      const { createHighlighter } = await import("./dist/index.js");
      const { default: diff } = await import("./dist/langs/diff.js");
      const highlighter = await createHighlighter({ languages: [diff] });
      if (!highlighter.languages.includes("diff")) throw new Error("diff was not loaded");
      console.log("ok");
    `;

    const { execFileSync } = await import("node:child_process");
    const output = execFileSync(process.execPath, ["--input-type=module", "-e", script], {
      cwd: new URL("..", import.meta.url).pathname,
      env: {
        ...process.env,
        LUMIS_DATA_DIR: directory,
      },
      encoding: "utf8",
    });

    expect(output).toContain("ok");
  }, 60_000);
});

describe("expandBundles", () => {
  it("expands a bundle into its members", () => {
    expect(expandBundles(["bundle-web"])).toEqual([
      "css",
      "html",
      "javascript",
      "json",
      "tsx",
      "typescript",
    ]);
  });

  it("accepts underscores, so Elixir's :bundle_web_extra reaches the same entry", () => {
    expect(expandBundles(["bundle_web_extra"])).toEqual(expandBundles(["bundle-web-extra"]));
  });

  it("leaves plain language names alone", () => {
    expect(expandBundles(["rust", "elixir"])).toEqual(["rust", "elixir"]);
  });

  it("deduplicates across a bundle and an explicit name", () => {
    const expanded = expandBundles(["bundle-web", "css"]);

    expect(expanded.filter((id) => id === "css")).toHaveLength(1);
  });

  it("rejects a name that looks like a bundle but is not one", () => {
    expect(() => expandBundles(["bundle-nope"])).toThrow(/Unknown bundle "bundle-nope"/);
  });
});

describe("the former name", () => {
  // Renaming a published export without keeping the old one turns an upgrade
  // into a build error for every caller.
  it("still points at downloadLanguages", async () => {
    const cache = await import("../src/cache.js");

    // eslint-disable-next-line @typescript-eslint/no-deprecated -- being deprecated is the point
    expect(cache.cacheLanguages).toBe(downloadLanguages);
  });
});

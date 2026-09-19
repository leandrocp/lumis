/**
 * Node's default runtime is the native addon, which is the same Wasmtime
 * highlighting the CLI and the Elixir bindings run. These pin what makes it
 * worth having: it resolves parsers itself, and it loads a language injected
 * inside a document during the walk that finds it.
 */
import { existsSync, mkdtempSync, readFileSync, readdirSync, rmSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { describe, expect, it, vi } from "vitest";
import { cacheLanguages } from "../src/cache.js";
import { createNativeLanguagesModule } from "../src/core/native-languages.js";
import { LANGUAGE_PACKAGE_VERSION_RANGE } from "../src/generated/package-version-range.js";
import type { LanguagesModule, RuntimeLike } from "../src/core/languages.js";
import type { NativeBinding, NativeRuntimeInstance } from "../src/native-binding.js";
import type { LoadedLanguage } from "../src/types.js";
import {
  ensureLocalParserWasm,
  localLanguagePackageMetadata,
  localLanguagePackageResolver,
} from "./wasm.js";

// Read when the addon builds its store, which is the first call into it below.
// Without `mise run stage-test-parsers` there is nothing staged, so fall back to
// an empty directory rather than pointing the store at a path that is not there.
const stagedParsers = resolve(import.meta.dirname, "../../../../target/test-parsers");
process.env.LUMIS_DATA_DIR ??= existsSync(stagedParsers)
  ? stagedParsers
  : mkdtempSync(join(tmpdir(), "lumis-native-test-"));

const { loadAddon, nativeTarget } = await import("../src/native-binding.js");

// Asserted directly, so a run that selected the Wasm runtime still covers this.
const binding = loadAddon();
const hasPrebuiltAddon = nativeTarget() !== undefined;

/**
 * `it`, for a test the addon is the subject of.
 *
 * Skipping is keyed on whether Lumis publishes an addon for this platform, never
 * on whether one loaded. Those differ exactly when the addon is missing or
 * broken on a platform that should have it, which is the defect these tests
 * exist to catch — `it.runIf(binding)` reported that as green.
 *
 * The remaining skip is a platform Lumis publishes no addon for, where there is
 * nothing to run rather than something unverified. `native-targets.test.ts` pins
 * that list, so the waiver can only shrink as targets are added.
 */
const itWithAddon = hasPrebuiltAddon ? it : it.skip;

function newRuntime(): NativeRuntimeInstance {
  if (!binding) {
    throw new Error(
      `no addon loaded for ${nativeTarget()}; run \`pnpm build:native\` (or \`mise run test-javascript\`) first`,
    );
  }

  return new binding.NativeRuntime();
}

function filesUnder(directory: string): string[] {
  if (!existsSync(directory)) return [];

  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const path = join(directory, entry.name);
    return entry.isDirectory() ? filesUnder(path) : [path];
  });
}

describe("native runtime", () => {
  it("is present wherever an addon is built", () => {
    if (!hasPrebuiltAddon) {
      expect(binding).toBeUndefined();
      return;
    }
    if (!existsSync(new URL("../native", import.meta.url))) {
      throw new Error("run `pnpm build:native` (or `mise run test-javascript`) first");
    }
    expect(binding?.runtimeKind()).toBe("native");
  });

  itWithAddon("resolves a parser without any JavaScript resolver", () => {
    const runtime = newRuntime();
    runtime.loadLanguage("json");
    expect(runtime.hasLanguage("json")).toBe(true);
  });

  // Compiling is the addon's, so this asserts one thing on a native run and its
  // negation on a Wasm one. Only the native half needs a binding: a host with no
  // published target runs the portable fallback, which is exactly where "caches
  // without compiling" is the behaviour in use, so gating that half on an addon
  // would drop the assertion on the platform it describes.
  const compilesWhenCaching = process.env.LUMIS_TEST_RUNTIME !== "wasm";
  const itForCacheCompilation = compilesWhenCaching ? itWithAddon : it;

  itForCacheCompilation("persists compiled modules only where the addon does it", async () => {
    // Fixes the process-global engine's cache directory first. Preparing another
    // one afterwards is the case this pins: it needs an engine of its own.
    if (compilesWhenCaching) newRuntime().loadLanguage("json");
    const directory = mkdtempSync(join(tmpdir(), "lumis-native-cache-"));
    try {
      await cacheLanguages(["diff"], {
        directory,
        resolver: (language, wasm) => ensureLocalParserWasm(language, wasm.name),
        languagePackageResolver: localLanguagePackageResolver,
      });

      const compiled = filesUnder(join(directory, "compiled", "modules"));
      if (compilesWhenCaching) {
        expect(compiled).not.toEqual([]);
      } else {
        expect(compiled).toEqual([]);
      }

      // Either way the parser itself is cached, which is the half that does not
      // depend on which runtime the process selected.
      expect(filesUnder(join(directory, "parsers"))).not.toEqual([]);
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });

  itWithAddon("shares catalog languages until caller behavior requires isolation", () => {
    const first = newRuntime();
    const second = newRuntime();
    first.loadLanguage("markdown");
    expect(second.hasLanguage("markdown")).toBe(true);

    second.loadLanguage("markdown");
    const wasm = readFileSync(ensureLocalParserWasm("json", "tree-sitter-json"));
    const privateId = second.loadLanguageDefinition(
      { id: "private-json", aliases: [], highlights: "(string) @string" },
      wasm,
    );

    expect(first.hasLanguage(privateId)).toBe(false);
    expect(second.hasLanguage(privateId)).toBe(true);
    expect(second.hasLanguage("markdown")).toBe(true);
    expect(
      second.highlightEvents("plain markdown", "markdown", false).events.length,
    ).toBeGreaterThan(0);
  });

  itWithAddon("loads an injected language during the walk that finds it", () => {
    const runtime = newRuntime();
    runtime.loadLanguage("html");
    expect(runtime.hasLanguage("javascript")).toBe(false);

    runtime.highlightEvents("<script>const answer = 42</script>", "html", false);

    expect(runtime.hasLanguage("javascript")).toBe(true);
  });

  itWithAddon("rejects installed metadata for a different package", () => {
    const runtime = newRuntime();
    const metadata = localLanguagePackageMetadata("@lumis-sh/wasm-json");
    const wasm = readFileSync(ensureLocalParserWasm("json", metadata.parser.name));

    expect(() =>
      runtime.loadLanguagePackage("json", "@test/not-json", JSON.stringify(metadata), wasm),
    ).toThrow(/resolver returned @lumis-sh\/wasm-json for @test\/not-json/);
  });

  itWithAddon("replays a trusted installed definition when resolver use isolates it", () => {
    const runtime = newRuntime();
    const metadata = localLanguagePackageMetadata("@lumis-sh/wasm-json");
    const wasm = readFileSync(ensureLocalParserWasm("json", metadata.parser.name));
    const internalId = runtime.loadInstalledLanguagePackage(
      "json",
      "@lumis-sh/wasm-json",
      JSON.stringify(metadata),
      wasm,
    );

    const highlighted = runtime.highlightEvents(
      '{"answer": 42}',
      internalId,
      false,
      undefined,
      () => {},
      () => {},
    );

    expect(highlighted.events.length).toBeGreaterThan(0);
    expect(runtime.hasLanguage(internalId)).toBe(true);
  });

  itWithAddon("rejects direct addon reentry from a resolver callback", () => {
    const source = '```json\n{"answer": 42}\n```\n';
    let runtime: NativeRuntimeInstance;
    let reentrantError: unknown;
    runtime = newRuntime();
    const wasmResolver = (language: string, wasmJson: string) => {
      if (language === "json" && reentrantError === undefined) {
        try {
          runtime.highlightEvents(source, "markdown", false);
        } catch (error) {
          reentrantError = error;
        }
      }
      const wasm = JSON.parse(wasmJson) as { name: string };
      return ensureLocalParserWasm(language, wasm.name).href;
    };
    runtime.loadLanguage("markdown");

    const highlighted = runtime.highlightEvents(
      source,
      "markdown",
      false,
      undefined,
      localLanguagePackageResolver,
      wasmResolver,
    );

    expect(String(reentrantError)).toContain(
      "native highlighting cannot be called from a language resolver callback",
    );
    expect(highlighted.unresolved).toEqual([]);
  });

  itWithAddon("uses stable content ids without sharing caller definitions", () => {
    const parserDirectory = resolve(process.env.LUMIS_DATA_DIR!, "parsers");
    const parser = readdirSync(parserDirectory).find(
      (name) => name.startsWith("tree-sitter-json-") && name.endsWith(".wasm"),
    );
    expect(parser).toBeDefined();
    const wasm = readFileSync(resolve(parserDirectory, parser!));
    const spec = {
      id: "definition-identity",
      aliases: [],
      highlights: "(string) @string",
    };

    const firstRuntime = newRuntime();
    const identicalRuntime = newRuntime();
    const differentRuntime = newRuntime();
    const observer = newRuntime();
    const first = firstRuntime.loadLanguageDefinition(spec, wasm);
    const identical = identicalRuntime.loadLanguageDefinition(spec, wasm);
    const different = differentRuntime.loadLanguageDefinition(
      { ...spec, highlights: "(number) @number" },
      wasm,
    );

    expect(identical).toBe(first);
    expect(different).not.toBe(first);
    expect(firstRuntime.hasLanguage(first)).toBe(true);
    expect(identicalRuntime.hasLanguage(first)).toBe(true);
    expect(observer.hasLanguage(first)).toBe(false);
  });

  itWithAddon("lets async work outlive an isolated runtime wrapper", () => {
    const nativeDirectory = resolve(import.meta.dirname, "../native");
    const addon = readdirSync(nativeDirectory).find(
      (name) => name.startsWith("lumis-native.") && name.endsWith(".node"),
    );
    expect(addon).toBeDefined();
    const parserDirectory = resolve(process.env.LUMIS_DATA_DIR!, "parsers");
    const parser = readdirSync(parserDirectory).find(
      (name) => name.startsWith("tree-sitter-json-") && name.endsWith(".wasm"),
    );
    expect(parser).toBeDefined();

    const result = spawnSync(
      process.execPath,
      [
        "--expose-gc",
        "--input-type=commonjs",
        "--eval",
        `
          const { readFileSync } = require("node:fs");
          const binding = require(process.argv[1]);
          const wasm = readFileSync(process.argv[2]);
          const spec = {
            id: "json",
            aliases: [],
            highlights: "(string) @string",
          };

          async function collect(reference) {
            for (let attempt = 0; attempt < 40; attempt += 1) {
              new ArrayBuffer(1024 * 1024);
              global.gc();
              await new Promise(setImmediate);
              if (reference.deref() === undefined) return true;
              await new Promise(setImmediate);
            }
            return false;
          }

          (async () => {
            let runtime = new binding.NativeRuntime();
            const id = runtime.loadLanguageDefinition(spec, wasm);
            const observer = new binding.NativeRuntime();
            if (observer.hasLanguage(id)) throw new Error("private definition leaked globally");
            const pending = runtime.formatAsync('"kept alive"', id, {
              kind: "html-linked",
              options: {},
            });
            const reference = new WeakRef(runtime);
            runtime = undefined;
            if (!(await collect(reference))) throw new Error("runtime wrapper was retained");
            const formatted = await pending;
            if (!formatted.output.includes('class="l-string"')) {
              throw new Error("async work lost its private runtime");
            }
          })().catch((error) => {
            console.error(error);
            process.exitCode = 1;
          });
        `,
        resolve(nativeDirectory, addon!),
        resolve(parserDirectory, parser!),
      ],
      {
        encoding: "utf8",
        env: process.env,
        timeout: 30_000,
      },
    );

    expect(result.error).toBeUndefined();
    expect(result.stderr).toBe("");
    expect(result.status).toBe(0);
  });

  itWithAddon("uses caller-owned injected definitions in async formatting", async () => {
    const runtime = newRuntime();
    const wasm = readFileSync(ensureLocalParserWasm("json", "tree-sitter-json"));
    runtime.loadLanguageDefinition(
      {
        id: "json",
        aliases: [],
        highlights: "(string) @string",
      },
      wasm,
    );
    runtime.loadLanguage("markdown");
    const source = '```json\n{"answer": 42}\n```\n';
    const formatter = { kind: "html-linked", options: {} };

    const sync = runtime.format(source, "markdown", formatter).output;
    const async = (await runtime.formatAsync(source, "markdown", formatter)).output;

    expect(sync).toContain('class="l-string"');
    expect(sync).not.toContain('class="l-number"');
    expect(async).toBe(sync);
  });

  itWithAddon("leaves a document alone when an injected language is unavailable", () => {
    const nativeDirectory = resolve(import.meta.dirname, "../native");
    const addon = readdirSync(nativeDirectory).find(
      (name) => name.startsWith("lumis-native.") && name.endsWith(".node"),
    );
    expect(addon).toBeDefined();

    // `comment` is staged and `regex` is not, but both are published, so a
    // store that can reach the CDN resolves `regex` and reports nothing. Only
    // an unreachable one makes "not staged" mean "unavailable"; the child
    // process is what keeps the poisoned proxy out of every other test.
    const result = spawnSync(
      process.execPath,
      [
        "--input-type=commonjs",
        "--eval",
        `
          const binding = require(process.argv[1]);
          const runtime = new binding.NativeRuntime();
          const highlighted = runtime.highlightEvents(process.argv[2], "markdown", false);
          process.stdout.write(
            JSON.stringify({
              events: highlighted.events.length,
              unresolved: highlighted.unresolved,
            }),
          );
        `,
        resolve(nativeDirectory, addon!),
        "```regex\n[a-z]+\n```\n",
      ],
      {
        encoding: "utf8",
        env: { ...process.env, ALL_PROXY: "http://127.0.0.1:1", NO_PROXY: "", no_proxy: "" },
        timeout: 30_000,
      },
    );

    expect(result.error).toBeUndefined();
    expect(result.status, result.stderr).toBe(0);

    const highlighted = JSON.parse(result.stdout) as { events: number; unresolved: string[] };
    expect(highlighted.events).toBeGreaterThan(0);
    // Reported rather than swallowed, so a caller that resolves parsers itself
    // can tell the difference between "no injection" and "could not load it".
    expect(highlighted.unresolved).toContain("regex");
  });
});

describe("native adapter routing", () => {
  it("passes addon resolver callbacks only when an override is active", () => {
    const highlightEvents = vi.fn<NativeRuntimeInstance["highlightEvents"]>(() => ({
      events: new Uint8Array(),
      unresolved: [],
    }));
    const nativeRuntime = { highlightEvents } as unknown as NativeRuntimeInstance;
    const addon = {
      NativeRuntime: function NativeRuntime() {
        return nativeRuntime;
      },
      configureStore: () => true,
      runtimeKind: () => "native",
    } as unknown as NativeBinding;
    const resolvers = {
      createRuntime: () => ({}),
    } as unknown as LanguagesModule;
    const languages = createNativeLanguagesModule(addon, resolvers);
    const loaded = { definition: { id: "json", aliases: [] } } as LoadedLanguage;

    languages.createRuntime().highlightEvents("{}", loaded);
    languages
      .createRuntime({ wasmResolver: () => new URL("file:///unused") })
      .highlightEvents("{}", loaded);

    expect(highlightEvents.mock.calls[0].slice(3)).toEqual([undefined, undefined, undefined]);
    expect(highlightEvents.mock.calls[1][4]).toBeTypeOf("function");
    expect(highlightEvents.mock.calls[1][5]).toBeTypeOf("function");
    const packageResolver = highlightEvents.mock.calls[1][4] as (packageName: string) => string;
    expect(packageResolver("@lumis-sh/wasm-json")).toContain(
      `@lumis-sh/wasm-json@${LANGUAGE_PACKAGE_VERSION_RANGE}/lumis.json`,
    );
  });

  it("coalesces concurrent loads of the same definition", async () => {
    const loadLanguageDefinition = vi.fn<NativeRuntimeInstance["loadLanguageDefinition"]>(
      () => "concurrent\u0001definition",
    );
    const nativeRuntime = { loadLanguageDefinition } as unknown as NativeRuntimeInstance;
    const addon = {
      NativeRuntime: function NativeRuntime() {
        return nativeRuntime;
      },
      configureStore: () => true,
      runtimeKind: () => "native",
    } as unknown as NativeBinding;
    const parserBytes = new Uint8Array([0]);
    const resolvers = {
      createRuntime: () => ({
        resolveLanguagePackage: () =>
          Promise.resolve({
            definition: { id: "concurrent", aliases: [] },
            wasm: {
              packageName: "@test/concurrent",
              name: "tree-sitter-concurrent",
              version: "0.0.0",
              sha256: createHash("sha256").update(parserBytes).digest("hex"),
              size: parserBytes.byteLength,
            },
            grammarName: "concurrent",
            highlights: "(string) @string",
          }),
      }),
    } as unknown as LanguagesModule;
    const runtime = createNativeLanguagesModule(addon, resolvers).createRuntime();
    const language = {
      definition: { id: "concurrent", aliases: [] },
      packageName: "@test/concurrent",
      wasm: parserBytes,
    };

    const [first, second] = await Promise.all([
      runtime.loadLanguage(language),
      runtime.loadLanguage(language),
    ]);

    expect(first).toBe(second);
    expect(loadLanguageDefinition).toHaveBeenCalledOnce();
  });

  it("does not replace a caller-selected package with the catalog package", async () => {
    const metadata = localLanguagePackageMetadata("@lumis-sh/wasm-json");
    const wasm = new Uint8Array(readFileSync(ensureLocalParserWasm("json", metadata.parser.name)));
    const loadLanguage = vi.fn<NativeRuntimeInstance["loadLanguage"]>();
    const loadLanguageDefinition = vi.fn<NativeRuntimeInstance["loadLanguageDefinition"]>(
      () => "json\u0001caller-package",
    );
    const nativeRuntime = {
      loadLanguage,
      loadLanguageDefinition,
    } as unknown as NativeRuntimeInstance;
    const addon = {
      NativeRuntime: function NativeRuntime() {
        return nativeRuntime;
      },
      configureStore: () => true,
      runtimeKind: () => "native",
    } as unknown as NativeBinding;
    const resolveLanguagePackage = vi.fn<RuntimeLike["resolveLanguagePackage"]>(async () => ({
      definition: { id: "json", aliases: [] },
      wasm: {
        packageName: "@test/custom-json",
        name: metadata.parser.name,
        version: metadata.version,
        sha256: metadata.parser.sha256,
        size: metadata.parser.size,
      },
      highlights: "(string) @string",
    }));
    const resolverRuntime = {
      resolveLanguagePackage,
      resolveParserWasm: async () => wasm,
    } as unknown as RuntimeLike;
    const resolvers = {
      createRuntime: () => resolverRuntime,
    } as unknown as LanguagesModule;
    const runtime = createNativeLanguagesModule(addon, resolvers).createRuntime();

    await runtime.loadLanguage({
      definition: { id: "json", aliases: [] },
      packageName: "@test/custom-json",
    });

    expect(resolveLanguagePackage).toHaveBeenCalledWith(
      { id: "json", aliases: [] },
      "@test/custom-json",
    );
    expect(loadLanguageDefinition).toHaveBeenCalledOnce();
    expect(loadLanguage).not.toHaveBeenCalled();
  });
});

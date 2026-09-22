import type { RuntimeEnvironment } from "./runtime.js";
import { createLanguagesModule } from "../core/languages.js";
import type { LanguagePackageResolver, LanguagesModule, WasmResolver } from "../core/languages.js";
import { createNativeLanguagesModule } from "../core/native-languages.js";
import { loadNativeBinding } from "../native-binding.js";
import treeSitterWasmBinary from "../tree-sitter-wasm.js";
import {
  isUrlString,
  readCachedWasm,
  wasmCacheFilename,
  withWasmCacheLock,
  writeCachedWasm,
} from "./node-cache.js";

// oxlint-disable-next-line no-useless-concat -- keeps a bundler from resolving the specifier statically.
const nodeFsPromises = "node:fs" + "/promises";
const nodePath = "node:path";
const nodeUrl = "node:url";

export const nodeRuntime: RuntimeEnvironment = {
  async resolveWasm(wasm) {
    if (wasm instanceof URL) {
      if (wasm.protocol === "file:") {
        const { fileURLToPath } = await import(nodeUrl);
        return fileURLToPath(wasm);
      }
      return wasm.href;
    }

    if (wasm instanceof Response) {
      return new Uint8Array(await wasm.arrayBuffer());
    }

    if (wasm instanceof ArrayBuffer) {
      return new Uint8Array(wasm);
    }

    return wasm;
  },

  async readFsCache(key) {
    return readCachedWasm(key);
  },

  async writeFsCache(key, data) {
    try {
      await writeCachedWasm(key, data);
    } catch {
      // cache write failures are non-fatal
    }
  },

  async withFsCacheLock(key, operation) {
    return withWasmCacheLock(key, operation);
  },

  async readStagedAsset(filename) {
    const root = process.env.LUMIS_DATA_DIR;
    if (!root) return;
    const { join } = await import(nodePath);
    const { readFile } = await import(nodeFsPromises);
    try {
      return new Uint8Array(await readFile(join(root, "parsers", filename)));
    } catch {
      return;
    }
  },

  async readResolvedWasmFromDisk(source) {
    const { isAbsolute } = await import(nodePath);

    if (source instanceof URL) {
      if (source.protocol !== "file:") {
        return;
      }

      const { fileURLToPath } = await import(nodeUrl);
      return new Uint8Array(await (await import(nodeFsPromises)).readFile(fileURLToPath(source)));
    }

    if (source.startsWith("file://")) {
      const { fileURLToPath } = await import(nodeUrl);
      return new Uint8Array(
        await (await import(nodeFsPromises)).readFile(fileURLToPath(new URL(source))),
      );
    }

    if (isUrlString(source)) {
      return;
    }

    const { readFile } = await import(nodeFsPromises);
    try {
      return new Uint8Array(await readFile(source));
    } catch {
      if (!isAbsolute(source)) return;
      throw new Error(`Failed to read parser WASM from ${source}`);
    }
  },

  async parserInitOptions() {
    return {
      wasmBinary: treeSitterWasmBinary,
    };
  },

  declaresLanguages: true,

  async resolveInstalledManifest(packageName: string): Promise<URL | undefined> {
    // From the application's directory, not Lumis's own: resolving relative to
    // this package would find whatever parser version Lumis itself happens to
    // carry, which is not what the project declared.
    const { createRequire } = await import("node:module");
    const { pathToFileURL } = await import("node:url");
    const { join } = await import("node:path");
    const resolveFromProject = createRequire(pathToFileURL(join(process.cwd(), "noop.js")));

    try {
      return pathToFileURL(resolveFromProject.resolve(`${packageName}/lumis.json`));
    } catch {
      // Not installed, or published before the manifest was part of the
      // package. Either way there is nothing here to read.
      return;
    }
  },
};

export { wasmCacheFilename };

export type {
  HighlighterRuntimeOptions,
  LoadLanguageOptions,
  SharedRuntimeCache,
  RuntimeLike,
  LanguagePackageResolver,
  WasmResolver,
} from "../core/languages.js";

const binding = loadNativeBinding();

/**
 * Node highlights through the native addon, the same Wasmtime runtime the CLI
 * and the Elixir bindings use, so all three produce identical output from
 * identical input and load parsers the same way.
 *
 * Platforms with no prebuilt addon fall back to `web-tree-sitter`, which the
 * browser uses too. It cannot load a language during the walk that discovers
 * it, so an injected language has to be loaded before the document mentioning
 * it is highlighted.
 */
const wasmRuntime = createLanguagesModule(nodeRuntime);

const runtime: LanguagesModule = binding
  ? createNativeLanguagesModule(binding, wasmRuntime, true)
  : wasmRuntime;

/**
 * Which runtime is highlighting: the native addon, or `web-tree-sitter`.
 *
 * Node prefers the addon and falls back silently, so anything that needs to
 * know which one it got — a benchmark reporting a number, a bug report — has to
 * be able to ask.
 *
 * ```ts
 * import { runtimeKind } from '@lumis-sh/lumis'
 * runtimeKind() // 'native' | 'wasm'
 * ```
 */
export function runtimeKind(): "native" | "wasm" {
  return binding ? "native" : "wasm";
}

export function createRuntime(...args: Parameters<LanguagesModule["createRuntime"]>) {
  return runtime.createRuntime(...args);
}
/**
 * Set a custom WASM resolver for parser binaries. Applies globally.
 *
 * ```ts
 * import { configureWasmResolver } from '@lumis-sh/lumis'
 *
 * configureWasmResolver((_language, wasm) =>
 *   `https://unpkg.com/${wasm.packageName}@${wasm.version}/${wasm.name}.wasm`
 * )
 * ```
 */
export function configureWasmResolver(fn: WasmResolver) {
  runtime.configureWasmResolver(fn);
}
export function configureLanguagePackageResolver(fn: LanguagePackageResolver) {
  runtime.configureLanguagePackageResolver(fn);
}
export function initParser(...args: Parameters<LanguagesModule["initParser"]>) {
  return runtime.initParser(...args);
}
export function registerLanguage(...args: Parameters<LanguagesModule["registerLanguage"]>) {
  runtime.registerLanguage(...args);
}
export function resolveLanguageId(...args: Parameters<LanguagesModule["resolveLanguageId"]>) {
  return runtime.resolveLanguageId(...args);
}
export function loadLanguage(...args: Parameters<LanguagesModule["loadLanguage"]>) {
  return runtime.loadLanguage(...args);
}
export function loadPlaintext(...args: Parameters<LanguagesModule["loadPlaintext"]>) {
  return runtime.loadPlaintext(...args);
}
export function getLoadedLanguage(...args: Parameters<LanguagesModule["getLoadedLanguage"]>) {
  return runtime.getLoadedLanguage(...args);
}
export function getLoadedLanguageIds(...args: Parameters<LanguagesModule["getLoadedLanguageIds"]>) {
  return runtime.getLoadedLanguageIds(...args);
}
/**
 * Ids of the languages loaded into this process, ready to highlight without a download.
 *
 * The complement of {@link availableLanguages}. Elixir spells it `Lumis.loaded_languages/0`.
 *
 * ```ts
 * import { loadedLanguages } from '@lumis-sh/lumis'
 * loadedLanguages()  // ['json', 'rust']
 * ```
 */
export function loadedLanguages(): string[] {
  return runtime.getLoadedLanguageIds();
}
/**
 * List all supported languages with their ID, name, aliases, and file extensions.
 *
 * ```ts
 * import { availableLanguages } from '@lumis-sh/lumis'
 * const languages = availableLanguages()
 * // [{ id: 'javascript', name: 'JavaScript', aliases: ['js', 'jsx'], extensions: ['*.js', ...] }, ...]
 * ```
 */
export function availableLanguages(...args: Parameters<LanguagesModule["availableLanguages"]>) {
  return runtime.availableLanguages(...args);
}
export function getDefaultRuntime(...args: Parameters<LanguagesModule["getDefaultRuntime"]>) {
  return runtime.getDefaultRuntime(...args);
}

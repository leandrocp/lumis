import type { RuntimeEnvironment } from "./runtime.js";
import { createLanguagesModule } from "../core/languages.js";
import type { LanguagePackageResolver, LanguagesModule, WasmResolver } from "../core/languages.js";
import { createNativeLanguagesModule } from "../core/native-languages.js";
import { BUNDLES } from "../generated/bundles-meta.js";
import { LANGUAGE_PACKAGE_NAMES } from "../generated/language-packages.js";
import { loadNativeBinding } from "../native-binding.js";
import treeSitterWasmBinary from "../tree-sitter-wasm.js";
import type { LanguageInfo } from "../types.js";
import { importNodeBuiltin } from "./node-builtins.js";
import {
  isUrlString,
  readCachedWasm,
  wasmCacheFilename,
  withWasmCacheLock,
  writeCachedWasm,
} from "./node-cache.js";

const BUNDLE_PACKAGE_NAMES = Object.keys(BUNDLES).map((name) => `@lumis-sh/wasm-bundle-${name}`);

interface InstalledPackage {
  root: string;
  /** Absent for a package published before the manifest was part of it. */
  manifest?: string;
}

function findPackage(
  resolve: NodeJS.RequireResolve,
  dirname: (path: string) => string,
  name: string,
): InstalledPackage | undefined {
  try {
    const manifest = resolve(`${name}/lumis.json`);
    return { root: dirname(manifest), manifest };
  } catch {
    // A bundle has no manifest, and neither has an old parser package.
  }
  try {
    return { root: dirname(resolve(name)) };
  } catch {
    return undefined;
  }
}

async function wasmDependencies(packageJson: string): Promise<string[]> {
  const { readFile } = await importNodeBuiltin("node:fs/promises");
  try {
    const manifest: unknown = JSON.parse(await readFile(packageJson, "utf8"));
    return wasmDependencyNames(manifest);
  } catch {
    return [];
  }
}

function wasmDependencyNames(manifest: unknown): string[] {
  if (typeof manifest !== "object" || manifest === null || !("dependencies" in manifest)) {
    return [];
  }

  const dependencies = manifest.dependencies;
  if (typeof dependencies !== "object" || dependencies === null || Array.isArray(dependencies)) {
    return [];
  }

  const entries: [string, unknown][] = Object.entries(dependencies);
  return entries.flatMap(([name, version]) =>
    name.startsWith("@lumis-sh/wasm-") && typeof version === "string" ? [name] : [],
  );
}

/**
 * Where each of `packageNames` this project installed keeps its `lumis.json`.
 *
 * From the application's directory, not Lumis's own: resolving relative to
 * this package would find whatever parser version Lumis itself happens to
 * carry, which is not what the project declared. pnpm links only the direct
 * dependencies there, and keeps the packages a bundle, or markdown, depends on
 * next to that package instead, so those are looked up from the package that
 * declared them.
 */
async function resolveInstalledManifests(
  packageNames: readonly string[],
): Promise<Map<string, URL>> {
  const { createRequire } = await import("node:module");
  const { pathToFileURL } = await importNodeBuiltin("node:url");
  const { dirname, join } = await importNodeBuiltin("node:path");
  const wanted = new Set(packageNames);
  const manifests = new Map<string, URL>();
  // By directory rather than name: an old copy without a manifest, found
  // first, must not hide the one a bundle brought.
  const explored = new Set<string>();
  const searches = [
    { from: join(process.cwd(), "noop.js"), names: [...packageNames, ...BUNDLE_PACKAGE_NAMES] },
  ];

  for (const { from, names } of searches) {
    const { resolve } = createRequire(pathToFileURL(from));
    for (const name of names) {
      const found = manifests.has(name) ? undefined : findPackage(resolve, dirname, name);
      if (!found || explored.has(found.root)) continue;
      explored.add(found.root);
      if (found.manifest && wanted.has(name)) manifests.set(name, pathToFileURL(found.manifest));
      const packageJson = join(found.root, "package.json");
      const dependencies = await wasmDependencies(packageJson);
      if (dependencies.length > 0) searches.push({ from: packageJson, names: dependencies });
    }
  }
  return manifests;
}

async function resolveInstalledManifest(packageName: string): Promise<URL | undefined> {
  // A package linked where the project can see it, or one a bundle brought.
  // Failing that, one that a package the project installed brought.
  return (
    (await resolveInstalledManifests([packageName])).get(packageName) ??
    (await resolveInstalledManifests(LANGUAGE_PACKAGE_NAMES)).get(packageName)
  );
}

export const nodeRuntime: RuntimeEnvironment = {
  async resolveWasm(wasm) {
    if (wasm instanceof URL) {
      if (wasm.protocol === "file:") {
        const { fileURLToPath } = await importNodeBuiltin("node:url");
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

  async readResolvedWasmFromDisk(source) {
    const { isAbsolute } = await importNodeBuiltin("node:path");
    let data: Uint8Array | undefined;

    if (source instanceof URL) {
      if (source.protocol === "file:") {
        const { fileURLToPath } = await importNodeBuiltin("node:url");
        const { readFile } = await importNodeBuiltin("node:fs/promises");
        data = new Uint8Array(await readFile(fileURLToPath(source)));
      }
    } else if (source.startsWith("file://")) {
      const { fileURLToPath } = await importNodeBuiltin("node:url");
      const { readFile } = await importNodeBuiltin("node:fs/promises");
      data = new Uint8Array(await readFile(fileURLToPath(new URL(source))));
    } else if (!isUrlString(source)) {
      const { readFile } = await importNodeBuiltin("node:fs/promises");
      try {
        data = new Uint8Array(await readFile(source));
      } catch {
        if (isAbsolute(source)) {
          throw new Error(`Failed to read parser WASM from ${source}`);
        }
      }
    }
    return data;
  },

  async parserInitOptions() {
    return {
      wasmBinary: treeSitterWasmBinary,
    };
  },

  resolveInstalledManifest,
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
  ? createNativeLanguagesModule(binding, wasmRuntime, resolveInstalledManifests)
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
export function availableLanguages(): LanguageInfo[] {
  return runtime.availableLanguages();
}
export function getDefaultRuntime(...args: Parameters<LanguagesModule["getDefaultRuntime"]>) {
  return runtime.getDefaultRuntime(...args);
}

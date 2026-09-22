import {
  cacheKey,
  DEFAULT_LANGUAGE_PACKAGE_RESOLVER,
  DEFAULT_RESOLVER,
  isCompatibleLanguagePackageVersion,
  languagePackageCacheKey,
  parseLanguagePackage,
  serializeLanguagePackageCache,
  verifyWasm,
  type LanguagePackage,
  type LanguagePackageResolver,
  type WasmResolver,
} from "./core/languages.js";
import { expandBundles, resolveLanguage } from "./core/language-names.js";
import { LANGUAGE_PACKAGE_VERSION_RANGE } from "./generated/package-version-range.js";
import {
  readCachedWasm,
  isUrlString,
  wasmCacheDir,
  wasmCachePath,
  withWasmCacheLock,
  writeCachedWasm,
} from "./runtime/node-cache.js";
import { loadNativeBinding } from "./native-binding.js";
import type { Language, WasmRef } from "./types.js";

export { expandBundles } from "./core/language-names.js";

export interface DownloadLanguagesOptions {
  /** Destination for verified parsers and native compiled modules. */
  directory?: string;
  /** Resolve the compatible package range again and replace verified parser files. */
  force?: boolean;
  /** Override the exact-version jsDelivr resolver. */
  resolver?: WasmResolver;
  /** Override the compatible-range language-package metadata resolver. */
  languagePackageResolver?: LanguagePackageResolver;
}

export interface DownloadedLanguage {
  language: string;
  path: string;
  downloaded: boolean;
  wasm: WasmRef;
}

function languagePackageName(language: Language): string {
  if (!language.packageName) {
    throw new Error(`Language "${language.id}" does not have a language package`);
  }
  return language.packageName;
}

async function fetchWasm(language: Language, ref: WasmRef, resolver: WasmResolver) {
  const source = resolver(language.id, ref);
  if (source instanceof URL ? source.protocol === "file:" : source.startsWith("file://")) {
    const { readFile } = await import("node:fs/promises");
    const { fileURLToPath } = await import("node:url");
    const url = source instanceof URL ? source : new URL(source);
    return verifyWasm(ref, new Uint8Array(await readFile(fileURLToPath(url))));
  }
  if (typeof source === "string" && !isUrlString(source)) {
    const { readFile } = await import("node:fs/promises");
    return verifyWasm(ref, new Uint8Array(await readFile(source)));
  }
  const response = await fetch(typeof source === "string" ? source : source.href);
  if (!response.ok) {
    throw new Error(
      `could not download parser WASM ${ref.name}@${ref.version}: HTTP ${response.status} ${response.statusText}`,
    );
  }
  return verifyWasm(ref, new Uint8Array(await response.arrayBuffer()));
}

async function fetchLanguagePackage(
  packageName: string,
  resolver: LanguagePackageResolver,
): Promise<LanguagePackage> {
  const source = resolver(packageName, LANGUAGE_PACKAGE_VERSION_RANGE);
  let packageMetadata: LanguagePackage;
  if (source instanceof URL ? source.protocol === "file:" : source.startsWith("file://")) {
    const { readFile } = await import("node:fs/promises");
    const { fileURLToPath } = await import("node:url");
    const url = source instanceof URL ? source : new URL(source);
    packageMetadata = parseLanguagePackage(
      new Uint8Array(await readFile(fileURLToPath(url))),
      packageName,
    );
  } else if (typeof source === "string" && !isUrlString(source)) {
    const { readFile } = await import("node:fs/promises");
    packageMetadata = parseLanguagePackage(new Uint8Array(await readFile(source)), packageName);
  } else {
    const response = await fetch(typeof source === "string" ? source : source.href);
    if (!response.ok) {
      throw new Error(
        `could not download language package ${packageName}: HTTP ${response.status} ${response.statusText}`,
      );
    }
    packageMetadata = parseLanguagePackage(
      new Uint8Array(await response.arrayBuffer()),
      packageName,
    );
  }

  if (!isCompatibleLanguagePackageVersion(packageMetadata.version)) {
    throw new Error(
      `Language package ${packageMetadata.packageName}@${packageMetadata.version} does not satisfy the supported range ${LANGUAGE_PACKAGE_VERSION_RANGE}`,
    );
  }
  return packageMetadata;
}

/**
 * Also write the package where the Rust store reads it, as `<suffix>.lumis.json`.
 *
 * Node highlights through the Wasmtime addon by default, and that store names
 * this file differently from the JavaScript cache. Prefetching would otherwise
 * fill a cache the runtime doing the highlighting never looks in, and the first
 * request would download it all again.
 */
async function writeSharedLanguagePackage(
  packageName: string,
  packageMetadata: LanguagePackage,
  directory?: string,
): Promise<void> {
  const { join } = await import("node:path");
  const { mkdir, writeFile } = await import("node:fs/promises");

  const suffix = packageName.replace(/^@lumis-sh\/wasm-/, "");
  if (suffix.includes("/")) return;

  const parsers = await wasmCacheDir(directory);
  await mkdir(parsers, { recursive: true });
  await writeFile(join(parsers, `${suffix}.lumis.json`), JSON.stringify(packageMetadata), "utf8");
}

/**
 * Resolve compatible packages and cache exact, integrity-pinned parser WASMs.
 * When the native Node addon is available, also compile and validate every
 * selected language and persist its Wasmtime module in the same directory.
 *
 * Accepts language names and `bundle-<name>` tokens, the same set
 * `Lumis.Languages.download/2` and `lumis languages download` accept.
 *
 * Point `LUMIS_DATA_DIR` at the same directory in the deployed process.
 */
export async function downloadLanguages(
  names: Iterable<string>,
  options: DownloadLanguagesOptions = {},
): Promise<DownloadedLanguage[]> {
  const directory = options.directory;
  const resolver = options.resolver ?? DEFAULT_RESOLVER;
  const packageResolver = options.languagePackageResolver ?? DEFAULT_LANGUAGE_PACKAGE_RESOLVER;
  const languages = await Promise.all(expandBundles(names).map((name) => resolveLanguage(name)));
  const seen = new Set<string>();
  const packages = new Map<string, LanguagePackage>();
  const persisted = new Set<string>();
  const cached: DownloadedLanguage[] = [];

  for (const language of languages) {
    if (language.id === "plaintext") continue;

    const packageName = languagePackageName(language);
    const packageMetadata = await languagePackageFor(
      packageName,
      packages,
      options,
      packageResolver,
    );
    requirePackagedLanguage(packageMetadata, language.id);

    const ref: WasmRef = {
      packageName,
      name: packageMetadata.parser.name,
      version: packageMetadata.version,
      sha256: packageMetadata.parser.sha256,
      size: packageMetadata.parser.size,
    };
    const key = cacheKey(ref);
    if (seen.has(key)) continue;
    seen.add(key);

    const result = await withWasmCacheLock(
      key,
      async () => {
        const existing = options.force ? undefined : await readVerifiedWasm(key, ref, directory);
        if (existing) return existing;

        const bytes = await fetchWasm(language, ref, resolver);
        return { path: await writeCachedWasm(key, bytes, directory), downloaded: true };
      },
      directory,
    );

    // A manifest names a parser, so it is only writable once that parser is in
    // the store. Replacing it first leaves a forced refresh that failed halfway
    // pointing every runtime sharing this directory at bytes nothing holds.
    if (!persisted.has(packageName)) {
      persisted.add(packageName);
      await persistLanguagePackage(packageName, packageMetadata, directory);
    }

    cached.push({ language: language.id, wasm: ref, ...result });
  }

  await precompileNatively(languages, directory);

  return cached;
}

// The manifest for a package, read from this run's map, then the cache, then
// the network.
async function languagePackageFor(
  packageName: string,
  packages: Map<string, LanguagePackage>,
  options: DownloadLanguagesOptions,
  packageResolver: LanguagePackageResolver,
): Promise<LanguagePackage> {
  const known = packages.get(packageName);
  if (known) return known;

  const metadata =
    (options.force ? undefined : await readCachedLanguagePackage(packageName, options.directory)) ??
    (await fetchLanguagePackage(packageName, packageResolver));
  packages.set(packageName, metadata);
  return metadata;
}

function requirePackagedLanguage(packageMetadata: LanguagePackage, languageId: string): void {
  if (!packageMetadata.languages[languageId]) {
    throw new Error(
      `Language "${languageId}" is not provided by ${packageMetadata.packageName}@${packageMetadata.version}`,
    );
  }
}

// A cached manifest is only usable when it parses and its version is one this
// build still understands; anything else is refetched.
async function readCachedLanguagePackage(
  packageName: string,
  directory: string | undefined,
): Promise<LanguagePackage | undefined> {
  const bytes = await readCachedWasm(languagePackageCacheKey(packageName), directory);
  if (!bytes) return undefined;

  try {
    const candidate = parseLanguagePackage(bytes, packageName);
    return isCompatibleLanguagePackageVersion(candidate.version) ? candidate : undefined;
  } catch {
    return undefined;
  }
}

// Cached parser bytes, if they are there and still match the digest the package
// names. A corrupt entry reads as absent and is replaced.
async function readVerifiedWasm(
  key: string,
  ref: WasmRef,
  directory: string | undefined,
): Promise<{ path: string; downloaded: boolean } | undefined> {
  const existing = await readCachedWasm(key, directory);
  if (!existing) return undefined;

  try {
    await verifyWasm(ref, existing);
    return { path: await wasmCachePath(key, directory), downloaded: false };
  } catch {
    return undefined;
  }
}

// A manifest names a parser, so it is only writable once that parser is in the
// store. Replacing it first leaves a forced refresh that failed halfway pointing
// every runtime sharing this directory at bytes nothing holds.
async function persistLanguagePackage(
  packageName: string,
  packageMetadata: LanguagePackage,
  directory: string | undefined,
): Promise<void> {
  await writeCachedWasm(
    languagePackageCacheKey(packageName),
    serializeLanguagePackageCache(packageMetadata),
    directory,
  );
  await writeSharedLanguagePackage(packageName, packageMetadata, directory);
}

// Every id, not one per parser: languages sharing a grammar have their own
// queries, and compiling validates those too.
async function precompileNatively(
  languages: Array<{ id: string }>,
  directory: string | undefined,
): Promise<void> {
  const binding = loadNativeBinding();
  if (!binding) return;

  const compilable = [...new Set(languages.map((language) => language.id))].filter(
    (id) => id !== "plaintext",
  );
  if (compilable.length > 0) await binding.precompileLanguages(compilable, directory);
}

/**
 * Former name of {@link downloadLanguages}. Still works.
 *
 * @deprecated Renamed to `downloadLanguages`. The store it fills is still a
 * cache; what a caller asks for is a download, which is the half they choose.
 */
export const cacheLanguages = downloadLanguages;

/** @deprecated Renamed to `DownloadLanguagesOptions`. */
export type CacheLanguagesOptions = DownloadLanguagesOptions;

/** @deprecated Renamed to `DownloadedLanguage`. */
export type CachedLanguage = DownloadedLanguage;

import type {
  Language,
  LanguageBundle,
  LanguagePackageExports,
  LazyLanguage,
  RuntimeLanguagePackageBundle,
  RuntimeWasmBundle,
  RuntimeWasmInput,
} from "./types.js";

export function lazy(
  id: string,
  aliases: string[],
  load: () => Promise<{ default: Language }>,
): LazyLanguage {
  return Object.assign(() => load().then((m) => m.default), { id, aliases });
}

export function mapBundle(
  bundle: LanguageBundle,
  map: (language: Language) => Language,
): LanguageBundle {
  return Object.fromEntries(
    Object.entries(bundle).map(([key, handle]) => [
      key,
      Object.assign(() => handle().then(map), { id: handle.id, aliases: handle.aliases }),
    ]),
  );
}

function isPackageExports(
  source: LanguagePackageExports | RuntimeWasmInput,
): source is LanguagePackageExports {
  return (
    typeof source === "object" &&
    !(source instanceof Uint8Array) &&
    !(source instanceof ArrayBuffer) &&
    !(source instanceof URL) &&
    !(source instanceof Response) &&
    "default" in source
  );
}

/** Shared by both entry points' `withWasm()`. */
export function languageWithPackage<T extends Language>(
  language: T,
  source: LanguagePackageExports | RuntimeWasmInput,
): Omit<T, "wasm" | "manifest"> & { wasm: RuntimeWasmInput; manifest?: object } {
  const { manifest: _manifest, wasm: _wasm, ...rest } = language;
  if (!isPackageExports(source)) return { ...rest, wasm: source };
  return source.manifest
    ? { ...rest, wasm: source.default, manifest: source.manifest }
    : { ...rest, wasm: source.default };
}

/** Shared by both entry points' `withWasmBundle()`. */
export function bundleWithPackages(
  bundle: LanguageBundle,
  sources: RuntimeLanguagePackageBundle | RuntimeWasmBundle,
): LanguageBundle {
  return mapBundle(bundle, (language) => {
    const source = sources[language.id];
    return source ? languageWithPackage(language, source) : language;
  });
}

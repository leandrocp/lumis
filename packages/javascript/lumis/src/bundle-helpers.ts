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
  return {
    ...bundle,
    ...Object.fromEntries(
      Object.entries(bundle).map(([key, handle]) => [
        key,
        Object.assign(() => handle().then(map), { id: handle.id, aliases: handle.aliases }),
      ]),
    ),
  };
}

function isObjectSource(
  source: Language | LanguagePackageExports | RuntimeWasmInput,
): source is Language | LanguagePackageExports {
  return (
    typeof source === "object" &&
    !(source instanceof Uint8Array) &&
    !(source instanceof ArrayBuffer) &&
    !(source instanceof URL) &&
    !(source instanceof Response)
  );
}

/** What a package or a language it exports carries: the parser, and its language package and companions when it has them. */
function packageParts(source: Language | LanguagePackageExports | RuntimeWasmInput): {
  wasm: RuntimeWasmInput;
  languagePackage?: object;
  requires?: Language[];
} {
  if (!isObjectSource(source)) return { wasm: source };
  if ("default" in source) {
    const parts = packageParts(source.default);
    return parts.languagePackage || !source.languagePackage
      ? parts
      : { ...parts, languagePackage: source.languagePackage };
  }
  const { wasm, languagePackage, requires } = source;
  if (wasm === undefined || (typeof wasm === "object" && "sha256" in wasm)) {
    throw new Error(`Language "${source.id}" carries no parser to attach`);
  }
  return { wasm, languagePackage, requires };
}

/** Shared by both entry points' `withWasm()`. */
export function languageWithPackage<T extends Language>(
  language: T,
  source: Language | LanguagePackageExports | RuntimeWasmInput,
): Omit<T, "wasm" | "languagePackage"> & { wasm: RuntimeWasmInput; languagePackage?: object } {
  const { languagePackage: _languagePackage, wasm: _wasm, ...rest } = language;
  const parts = packageParts(source);
  return {
    ...rest,
    wasm: parts.wasm,
    ...(parts.languagePackage ? { languagePackage: parts.languagePackage } : {}),
    ...(parts.requires ? { requires: parts.requires } : {}),
  };
}

/** Shared by both entry points' `withWasmBundle()`. */
export function bundleWithPackages(
  bundle: LanguageBundle,
  sources: RuntimeLanguagePackageBundle | RuntimeWasmBundle | Partial<Record<string, Language>>,
): LanguageBundle {
  return mapBundle(bundle, (language) => {
    const source = sources[language.id];
    return source ? languageWithPackage(language, source) : language;
  });
}

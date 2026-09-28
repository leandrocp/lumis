import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve, sep } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { parse as parseToml } from "smol-toml";

import {
  configureLanguagePackageResolver as configureDefaultLanguagePackageResolver,
  configureWasmResolver as configureDefaultWasmResolver,
} from "../src/index.js";
import { lowestCompatibleLanguagePackageVersion } from "../src/core/languages.js";
import type {
  LanguagePackage,
  LanguagePackageResolver,
  WasmResolver,
} from "../src/core/languages.js";

const repositoryRoot = resolve(process.cwd(), "../../..");
const fixturesRoot = pathToFileURL(join(repositoryRoot, "fixtures/test-parsers") + sep);
const packageDataUrls = new Map<string, string>();
const packageMetadataCache = new Map<string, LanguagePackage>();
const parserDefinitions = (
  parseToml(readFileSync(join(repositoryRoot, "languages.toml"), "utf8")) as {
    parsers: Record<
      string,
      {
        aliases?: string[];
        query_name?: string;
        wasm_name?: string;
      }
    >;
  }
).parsers;

export function ensureLocalWasm(language: string): URL {
  return ensureLocalParserWasm(language, `tree-sitter-${language}`);
}

export function ensureLocalParserWasm(language: string, parser: string): URL {
  const wasmUrl = new URL(`${parser}.wasm`, fixturesRoot);

  if (!existsSync(fileURLToPath(wasmUrl))) {
    throw new Error(`Missing committed test WASM for ${language} at ${fileURLToPath(wasmUrl)}`);
  }

  return wasmUrl;
}

interface ResolverConfiguration {
  configureLanguagePackageResolver(resolver: LanguagePackageResolver): void;
  configureWasmResolver(resolver: WasmResolver): void;
}

const defaultResolverConfiguration: ResolverConfiguration = {
  configureLanguagePackageResolver: configureDefaultLanguagePackageResolver,
  configureWasmResolver: configureDefaultWasmResolver,
};

export function configureLocalWasmResolver(
  languages: string[],
  configuration: ResolverConfiguration = defaultResolverConfiguration,
  wasmResolver: WasmResolver = (language, wasm) => ensureLocalParserWasm(language, wasm.name),
): void {
  for (const language of languages) {
    ensureLocalWasm(language);
  }

  configuration.configureLanguagePackageResolver(localLanguagePackageResolver);
  configuration.configureWasmResolver(wasmResolver);
}

export function localLanguagePackageResolver(packageName: string): string {
  const cached = packageDataUrls.get(packageName);
  if (cached) return cached;

  const packageMetadata = localLanguagePackageMetadata(packageName);
  const dataUrl = `data:application/json;base64,${Buffer.from(
    JSON.stringify(packageMetadata),
  ).toString("base64")}`;
  packageDataUrls.set(packageName, dataUrl);
  return dataUrl;
}

export function localLanguagePackageMetadata(packageName: string): LanguagePackage {
  const cached = packageMetadataCache.get(packageName);
  if (cached) return cached;

  const packageLanguages = Object.entries(parserDefinitions).filter(([language, definition]) => {
    const wasmName = definition.wasm_name ?? `tree-sitter-${language}`;
    const suffix = wasmName.replace(/^tree-sitter-/, "");
    return `@lumis-sh/wasm-${suffix}` === packageName;
  });
  if (packageLanguages.length === 0) {
    throw new Error(`Unknown local language package: ${packageName}`);
  }
  const [parserId, parserDefinition] = packageLanguages[0];
  const wasmName = parserDefinition.wasm_name ?? `tree-sitter-${parserId}`;
  const wasm = readFileSync(fileURLToPath(ensureLocalParserWasm(parserId, wasmName)));
  const query = (language: string, name: string): string => {
    const queryName = parserDefinitions[language].query_name ?? language;
    const path = join(repositoryRoot, "queries/processed", queryName, `${name}.scm`);
    return existsSync(path) ? readFileSync(path, "utf8") : "";
  };
  const defaultBrackets = readFileSync(
    join(repositoryRoot, "queries/processed/default/brackets.scm"),
    "utf8",
  );
  const grammarNames = WebAssembly.Module.exports(new WebAssembly.Module(wasm))
    .filter(({ kind, name }) => kind === "function" && name.startsWith("tree_sitter_"))
    .map(({ name }) => name.slice("tree_sitter_".length))
    .filter((name) => !name.startsWith("external_scanner_"));
  if (grammarNames.length !== 1) {
    throw new Error(`Expected one grammar export for ${language}: ${grammarNames.join(", ")}`);
  }

  const packageMetadata: LanguagePackage = {
    packageName,
    version: lowestCompatibleLanguagePackageVersion(),
    definitionHash: createHash("sha256").update(wasm).digest("hex"),
    parser: {
      name: wasmName,
      grammarName: grammarNames[0],
      sha256: createHash("sha256").update(wasm).digest("hex"),
      size: wasm.byteLength,
    },
    languages: Object.fromEntries(
      packageLanguages.map(([language, definition]) => [
        language,
        {
          aliases: definition.aliases ?? [],
          highlights: query(language, "highlights"),
          injections: query(language, "injections"),
          locals: query(language, "locals"),
          brackets: query(language, "brackets") || defaultBrackets,
        },
      ]),
    ),
  };
  packageMetadataCache.set(packageName, packageMetadata);
  return packageMetadata;
}

export function ensureLocalParserWasmDataUrl(language: string, parser: string): string {
  const wasmUrl = ensureLocalParserWasm(language, parser);
  const bytes = readFileSync(fileURLToPath(wasmUrl));
  return `data:application/wasm;base64,${bytes.toString("base64")}`;
}

/**
 * Install `languages` under `root/node_modules`, shaped the way a published
 * package is: the manifest under `./lumis.json` in the export map and as the
 * entry's `manifest` export, and the parser beside it named after `parser.name`.
 *
 * Returns each package's `lumis.json` by package name.
 */
export function installLocalPackages(root: string, languages: string[]): Record<string, string> {
  const manifests: Record<string, string> = {};
  for (const language of languages) {
    const metadata = localLanguagePackageMetadata(`@lumis-sh/wasm-${language}`);
    const directory = join(root, "node_modules", metadata.packageName);
    const parser = `${metadata.parser.name}.wasm`;
    mkdirSync(directory, { recursive: true });
    writeFileSync(join(directory, "lumis.json"), JSON.stringify(metadata));
    writeFileSync(
      join(directory, parser),
      readFileSync(ensureLocalParserWasm(language, metadata.parser.name)),
    );
    writeFileSync(
      join(directory, "index.js"),
      `const wasm = new URL("./${parser}", import.meta.url);
export const manifest = ${JSON.stringify(metadata)};
export const ${language} = { id: ${JSON.stringify(language)}, aliases: ${JSON.stringify(metadata.languages[language]?.aliases ?? [])}, packageName: ${JSON.stringify(metadata.packageName)}, wasm, manifest };
export default ${language};
`,
    );
    writeFileSync(
      join(directory, "package.json"),
      JSON.stringify({
        name: metadata.packageName,
        version: metadata.version,
        type: "module",
        exports: { ".": "./index.js", "./lumis.json": "./lumis.json" },
      }),
    );
    manifests[metadata.packageName] = join(directory, "lumis.json");
  }
  return manifests;
}

/**
 * What `import language from "@lumis-sh/wasm-<language>"` gives a bundler: the
 * language, its parser and manifest included.
 */
export function localPackageLanguage(
  language: string,
  requires?: ReturnType<typeof localPackageLanguage>[],
): {
  id: string;
  aliases: string[];
  packageName: string;
  wasm: Uint8Array;
  manifest: LanguagePackage;
  requires?: ReturnType<typeof localPackageLanguage>[];
} {
  const manifest = localLanguagePackageMetadata(`@lumis-sh/wasm-${language}`);
  return {
    id: language,
    aliases: manifest.languages[language]?.aliases ?? [],
    packageName: manifest.packageName,
    wasm: new Uint8Array(readFileSync(ensureLocalParserWasm(language, manifest.parser.name))),
    manifest,
    ...(requires ? { requires } : {}),
  };
}

import {
  configureLanguagePackageResolver,
  configureWasmResolver,
  highlight,
  loadedLanguages,
  loadLanguages,
} from "@lumis-sh/lumis";
import { bundledLanguages } from "@lumis-sh/lumis/bundles/full";
import { htmlInline, htmlMultiThemes } from "@lumis-sh/lumis/formatters";
import type { Language, Theme } from "@lumis-sh/lumis";

const DEFAULT_PRE_CLASS =
  "m-0 overflow-x-auto p-5 font-mono text-[13px] leading-relaxed sm:p-6 sm:text-sm";

const wasmModules = import.meta.glob<string>(
  [
    "../../node_modules/@lumis-sh/wasm-*/*.wasm",
    "../../node_modules/.pnpm/@lumis-sh+wasm-*/node_modules/@lumis-sh/wasm-*/*.wasm",
    "!../../node_modules/@lumis-sh/wasm-bundle-*/*.wasm",
    "!../../node_modules/.pnpm/@lumis-sh+wasm-bundle-*/node_modules/@lumis-sh/wasm-bundle-*/*.wasm",
  ],
  { eager: true, import: "default", query: "?url" },
);

const wasmVersions = import.meta.glob<string>(
  [
    "../../node_modules/@lumis-sh/wasm-*/package.json",
    "../../node_modules/.pnpm/@lumis-sh+wasm-*/node_modules/@lumis-sh/wasm-*/package.json",
    "!../../node_modules/@lumis-sh/wasm-bundle-*/package.json",
    "!../../node_modules/.pnpm/@lumis-sh+wasm-bundle-*/node_modules/@lumis-sh/wasm-bundle-*/package.json",
  ],
  { eager: true, import: "version" },
);

const languagePackages = import.meta.glob<string>(
  [
    "../../node_modules/@lumis-sh/wasm-*/lumis.json",
    "../../node_modules/.pnpm/@lumis-sh+wasm-*/node_modules/@lumis-sh/wasm-*/lumis.json",
    "!../../node_modules/@lumis-sh/wasm-bundle-*/lumis.json",
    "!../../node_modules/.pnpm/@lumis-sh+wasm-bundle-*/node_modules/@lumis-sh/wasm-bundle-*/lumis.json",
  ],
  { eager: true, import: "default", query: "?url" },
);

const versionByDirectory = new Map(
  Object.entries(wasmVersions).map(([path, version]) => [directoryOf(path), version]),
);

const wasmUrls = new Map(
  Object.entries(wasmModules).map(([path, url]) => {
    const match = path.match(/(@lumis-sh\/wasm-[^/]+)\/([^/]+\.wasm)$/);
    if (!match) {
      throw new Error(`Could not derive wasm package name and file from ${path}`);
    }

    const version = versionByDirectory.get(directoryOf(path));
    if (!version) {
      throw new Error(`Could not find the installed version for ${path}`);
    }

    return [`${match[1]}@${version}/${match[2]}`, url];
  }),
);

const packageUrls = new Map(
  Object.entries(languagePackages).map(([path, url]) => {
    const match = path.match(/(@lumis-sh\/wasm-[^/]+)\/lumis\.json$/);
    if (!match) {
      throw new Error(`Could not derive the language package name from ${path}`);
    }

    return [match[1], url];
  }),
);

function directoryOf(path: string) {
  return path.slice(0, path.lastIndexOf("/"));
}

// The catalog resolves each parser package against the registry, so it can ask
// for a version newer than the one installed here, and `pnpm` leaves the
// superseded copy on disk, so a name alone can match either. The bytes are
// checked against the digest the catalog carries, and a near miss
// fails every language rather than falling back, so only the exact version is
// served locally.
configureWasmResolver((_language, wasm) => {
  const file = `${wasm.packageName}@${wasm.version}/${wasm.name}.wasm`;
  return wasmUrls.get(file) ?? `https://cdn.jsdelivr.net/npm/${file}`;
});

// Serving the metadata locally is what keeps the parser local too. The manifest
// names the exact version and digest the parser is then fetched by, so reading
// it from the CDN can name a release newer than the one `pnpm` installed, and
// the resolver above would miss and pull megabytes over the network for a file
// already in the build. Both come from the same directory, or neither does.
configureLanguagePackageResolver((packageName, versionRange) => {
  return (
    packageUrls.get(packageName) ??
    `https://cdn.jsdelivr.net/npm/${packageName}@${versionRange}/lumis.json`
  );
});

async function getLanguage(languageId: string): Promise<Language> {
  const handle = bundledLanguages[languageId];
  if (!handle) {
    throw new Error(`Unknown language: ${languageId}`);
  }
  return handle();
}

export type WorkerRequest =
  | {
      id: number;
      type: "highlight";
      languageId: string;
      theme: Theme;
      source: string;
      preClass?: string;
    }
  | {
      id: number;
      type: "highlightMultiTheme";
      languageId: string;
      lightTheme: Theme;
      darkTheme: Theme;
      source: string;
      preClass?: string;
    }
  | { id: number; type: "loadLanguages"; languageIds: string[] };

export type WorkerResponse =
  | { id: number; type: "result"; html: string }
  | { id: number; type: "done"; loaded: string[] }
  | { id: number; type: "error"; message: string };

async function handleMessage(req: WorkerRequest): Promise<WorkerResponse> {
  switch (req.type) {
    case "highlight": {
      const language = await getLanguage(req.languageId);
      const html = await highlight(
        req.source,
        htmlInline({
          language,
          theme: req.theme,
          preClass: req.preClass ?? DEFAULT_PRE_CLASS,
          includeHighlights: true,
          italic: false,
        }),
      );
      return { id: req.id, type: "result", html };
    }
    case "highlightMultiTheme": {
      const language = await getLanguage(req.languageId);
      const html = await highlight(
        req.source,
        htmlMultiThemes({
          language,
          themes: { light: req.lightTheme, dark: req.darkTheme },
          defaultTheme: "light-dark()",
          preClass: req.preClass ?? DEFAULT_PRE_CLASS,
          italic: false,
        }),
      );
      return { id: req.id, type: "result", html };
    }
    case "loadLanguages": {
      // Warm-up is an optimization, so one unavailable parser reports itself and
      // leaves the rest loaded rather than failing the batch. `loadLanguages`
      // attempts every name concurrently and rejects with an AggregateError.
      await loadLanguages(req.languageIds).catch((error: unknown) => {
        console.warn("Lumis warm-up did not finish; languages load on demand", error);
      });
      return { id: req.id, type: "done", loaded: loadedLanguages() };
    }
  }
}

self.onmessage = async (e: MessageEvent<WorkerRequest>) => {
  try {
    const response = await handleMessage(e.data);
    self.postMessage(response);
  } catch (err) {
    self.postMessage({
      id: e.data.id,
      type: "error",
      message: String(err),
    } satisfies WorkerResponse);
  }
};

import {
  createHighlighter,
  runtimeKind,
  withWasm,
  type Highlighter,
} from "../../src/index.browser.ts";
import { htmlLinked } from "../../src/formatters.ts";
import type { LanguageDefinition } from "../../src/types.ts";

const languageModules = import.meta.glob<{ default: LanguageDefinition }>("../../langs/*.ts", {
  eager: true,
});
const languageById = new Map(
  Object.entries(languageModules).map(([path, module]) => [
    path.split("/").at(-1)!.slice(0, -3),
    module.default,
  ]),
);

interface InitInput {
  packages: Record<string, string>;
  wasms: Record<string, string>;
}

interface RenderInput {
  iterations: number;
  language: string;
  source: string;
}

interface BrowserMemory {
  usedJsHeapBytes?: number;
}

interface BrowserIteration {
  iteration: number;
  status: "ok";
  wallMs: number;
  outputBytes: number;
  outputSha256: string;
  memory: { before: BrowserMemory; after: BrowserMemory };
}

interface BrowserCaseResult {
  sourceBytes: number;
  sourceSha256: string;
  iterations: BrowserIteration[];
}

interface StressApi {
  init(input: InitInput): Promise<{ preloadWallMs: number; runtimeKind: string }>;
  render(input: RenderInput): Promise<BrowserCaseResult>;
}

function decodeBase64(encoded: string): Uint8Array {
  const binary = atob(encoded);
  return Uint8Array.from(binary, (byte) => byte.charCodeAt(0));
}

function memorySnapshot(): BrowserMemory {
  const memory = (performance as Performance & { memory?: { usedJSHeapSize: number } }).memory;
  return { usedJsHeapBytes: memory?.usedJSHeapSize };
}

async function digest(bytes: Uint8Array): Promise<string> {
  const hash = await crypto.subtle.digest("SHA-256", bytes);
  return [...new Uint8Array(hash)].map((byte) => byte.toString(16).padStart(2, "0")).join("");
}

let highlighter: Highlighter | undefined;
const loadedLanguages = new Map<string, LanguageDefinition>();

const api: StressApi = {
  async init({ packages, wasms }) {
    const packageUrls = new Map(
      Object.entries(packages).map(([packageName, source]) => [
        packageName,
        URL.createObjectURL(new Blob([source], { type: "application/json" })),
      ]),
    );
    const languages = Object.entries(wasms).map(([id, encoded]) => {
      const language = languageById.get(id);
      if (!language) throw new Error(`unknown browser language: ${id}`);
      const loaded = withWasm(language, decodeBase64(encoded));
      loadedLanguages.set(id, loaded);
      return loaded;
    });
    const started = performance.now();
    highlighter = await createHighlighter({
      languages,
      languagePackageResolver(packageName) {
        const url = packageUrls.get(packageName);
        if (!url) throw new Error(`missing stress language package: ${packageName}`);
        return url;
      },
    });
    return { preloadWallMs: performance.now() - started, runtimeKind: runtimeKind() };
  },

  async render({ iterations, language, source }) {
    if (!highlighter) throw new Error("stress highlighter was not initialized");
    const definition = loadedLanguages.get(language);
    if (!definition) throw new Error(`stress language was not loaded: ${language}`);
    const encoder = new TextEncoder();
    const sourceBytes = encoder.encode(source);
    const measurements: BrowserIteration[] = [];

    for (let iteration = 1; iteration <= iterations; iteration += 1) {
      const before = memorySnapshot();
      const started = performance.now();
      const output = highlighter.highlight(source, htmlLinked({ language: definition }));
      const wallMs = performance.now() - started;
      const outputBytes = encoder.encode(output);
      measurements.push({
        iteration,
        status: "ok",
        wallMs,
        outputBytes: outputBytes.length,
        outputSha256: await digest(outputBytes),
        memory: { before, after: memorySnapshot() },
      });
    }

    return {
      sourceBytes: sourceBytes.length,
      sourceSha256: await digest(sourceBytes),
      iterations: measurements,
    };
  },
};

Object.assign(window, { __lumisStressApi: api, __lumisStressReady: true });

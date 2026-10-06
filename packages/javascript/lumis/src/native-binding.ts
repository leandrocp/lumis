import { createRequire } from "node:module";
import type {
  BBCodeScopedOptions,
  HtmlInlineOptions,
  HtmlLinkedOptions,
  TerminalOptions,
} from "./types.js";

/** A string value, or `true`/`false` for the boolean form and for removal. */
export type NativeHtmlAttrs = Array<[string, string | boolean]>;

type NativeHtmlInlineOptions = Omit<
  Pick<
    HtmlInlineOptions,
    | "structure"
    | "theme"
    | "preClass"
    | "preAttrs"
    | "codeAttrs"
    | "italic"
    | "includeHighlights"
    | "highlightLines"
    | "lineNumbers"
    | "header"
  >,
  "preAttrs" | "codeAttrs"
> & {
  preAttrs: NativeHtmlAttrs;
  codeAttrs: NativeHtmlAttrs;
};
type NativeHtmlLinkedOptions = Omit<
  Pick<
    HtmlLinkedOptions,
    | "structure"
    | "preClass"
    | "preAttrs"
    | "codeAttrs"
    | "highlightLines"
    | "lineNumbers"
    | "header"
  >,
  "preAttrs" | "codeAttrs"
> & {
  preAttrs: NativeHtmlAttrs;
  codeAttrs: NativeHtmlAttrs;
};
type NativeTerminalOptions = Pick<
  TerminalOptions,
  "theme" | "background" | "width" | "highlightLines" | "lineNumbers"
>;
type NativeBBCodeScopedOptions = Pick<BBCodeScopedOptions, "highlightLines">;

interface NativeBudget {
  matchLimit?: number;
  timeLimit?: number;
}

interface NativeFormatterBase {
  rainbowBrackets?: boolean;
  budget?: NativeBudget;
}

export type NativeFormatter = NativeFormatterBase &
  (
    | {
        kind: "html-inline";
        options: NativeHtmlInlineOptions;
      }
    | {
        kind: "html-linked";
        options: NativeHtmlLinkedOptions;
      }
    | { kind: "bbcode-scoped"; options: NativeBBCodeScopedOptions }
    | { kind: "terminal"; options: NativeTerminalOptions }
  );

export interface NativeLanguageSpec {
  id: string;
  aliases: string[];
  /** Read from the parser's exports when omitted. */
  grammarName?: string;
  highlights: string;
  injections?: string;
  locals?: string;
  brackets?: string;
}

interface NativeRuntimeInstance {
  /** Load a catalog language from the installed package that ships it. */
  loadLanguage(id: string): void;
  loadLanguagePackage(
    id: string,
    expectedPackageName: string,
    packageJson: string,
    wasm: Uint8Array,
  ): string;
  loadLanguageDefinition(spec: NativeLanguageSpec, wasm: Uint8Array): string;
  hasLanguage(id: string): boolean;
  highlightEvents(
    source: string,
    language: string,
    rainbowBrackets?: boolean,
    budget?: NativeBudget,
    packageResolver?: (packageName: string) => string | undefined,
    wasmResolver?: (language: string, wasmJson: string) => string | undefined,
  ): { events: Uint8Array; unresolved: string[]; budget?: "time" | "matches" };
  format(
    source: string,
    language: string,
    formatter: NativeFormatter,
    packageResolver?: (packageName: string) => string | undefined,
    wasmResolver?: (language: string, wasmJson: string) => string | undefined,
  ): { output: string; unresolved: string[] };
  formatAsync(
    source: string,
    language: string,
    formatter: NativeFormatter,
  ): Promise<{ output: string; unresolved: string[] }>;
}

export interface NativeBinding {
  NativeRuntime: new () => NativeRuntimeInstance;
  runtimeKind(): string;
  /**
   * Where compiled parser modules are kept. `false` once the first runtime has
   * read it.
   */
  configureStore(dataDir?: string): boolean;
  /**
   * The `lumis.json` of every `@lumis-sh/wasm-*` package this project
   * installed, by package name.
   *
   * These are the whole set the addon loads from. JavaScript resolves them,
   * because the addon cannot follow Node's resolution, and the addon reads them,
   * because it loads an injected language during a native walk without
   * returning to JavaScript.
   */
  setInstalledPackages(manifests: Record<string, string>): void;
  /** The data directory when `LUMIS_DATA_DIR` names nothing. */
  defaultDataDir(): string;
}

let cachedBinding: NativeBinding | null | undefined;

function linuxLibc(): "gnu" | "musl" {
  const report: unknown = process.report?.getReport?.();
  if (!isObject(report) || !("header" in report)) return "musl";

  const header: unknown = report.header;
  if (!isObject(header) || !("glibcVersionRuntime" in header)) return "musl";
  return typeof header.glibcVersionRuntime === "string" ? "gnu" : "musl";
}

function isObject(value: unknown): value is object {
  return typeof value === "object" && value !== null;
}

function hasFunctions(value: object, names: readonly string[]): boolean {
  return names.every((name) => typeof Reflect.get(value, name) === "function");
}

function isNativeBinding(value: unknown): value is NativeBinding {
  if (!isObject(value)) return false;

  const nativeRuntime: unknown = Reflect.get(value, "NativeRuntime");
  if (typeof nativeRuntime !== "function") return false;

  const prototype: unknown = Reflect.get(nativeRuntime, "prototype");
  return (
    isObject(prototype) &&
    hasFunctions(value, [
      "runtimeKind",
      "configureStore",
      "setInstalledPackages",
      "defaultDataDir",
    ]) &&
    hasFunctions(prototype, [
      "loadLanguage",
      "loadLanguagePackage",
      "loadLanguageDefinition",
      "hasLanguage",
      "highlightEvents",
      "format",
      "formatAsync",
    ])
  );
}

/**
 * The platform package name for a host, or `undefined` where none is published.
 *
 * `native/npm/meta/index.js` repeats this for installs that resolve the addon
 * through `@lumis-sh/lumis-native`, because that package ships alone and cannot
 * import from here. `test/native-targets.test.ts` pins the two together and to
 * the published set, so a new target has to be added in both places.
 */
export function nativeTargetFor(
  platform: string,
  arch: string,
  libc: "gnu" | "musl",
): string | undefined {
  if (platform === "darwin" && ["arm64", "x64"].includes(arch)) {
    return `darwin-${arch}`;
  }
  if (platform === "linux" && ["arm64", "x64"].includes(arch)) {
    return `linux-${arch}-${libc}`;
  }
  if (platform === "win32" && ["arm64", "x64"].includes(arch)) {
    return `win32-${arch}-msvc`;
  }
  return undefined;
}

export function nativeTarget(): string | undefined {
  return nativeTargetFor(
    process.platform,
    process.arch,
    process.platform === "linux" ? linuxLibc() : "gnu",
  );
}

/**
 * The platform addon, ignoring whether this process wants to use it.
 *
 * Separate from {@link loadNativeBinding} so a test can assert the addon works
 * even in a run that has asked for the Wasm runtime.
 */
export function loadAddon(): NativeBinding | undefined {
  const target = nativeTarget();
  if (!target) return undefined;

  const require = createRequire(import.meta.url);
  const candidates = [
    `../native/lumis-native.${target}.node`,
    "@lumis-sh/lumis-native",
    `@lumis-sh/lumis-native-${target}`,
  ];
  for (const candidate of candidates) {
    try {
      const binding: unknown = require(candidate);
      if (!isNativeBinding(binding)) continue;
      if (binding.runtimeKind?.() === "native") return binding;
    } catch {
      // Missing or unloadable platform packages transparently use the Wasm runtime.
    }
  }
  return undefined;
}

/** Load the platform addon without making native support a public API choice. */
export function loadNativeBinding(): NativeBinding | undefined {
  if (cachedBinding !== undefined) {
    return cachedBinding ?? undefined;
  }
  if (process.env.LUMIS_TEST_RUNTIME === "wasm") {
    cachedBinding = null;
    return undefined;
  }

  const binding = loadAddon();
  cachedBinding = binding ?? null;
  if (!binding && process.env.LUMIS_TEST_RUNTIME === "native") {
    throw new Error(
      `Lumis native runtime is required but unavailable for ${process.platform}-${process.arch}`,
    );
  }
  return binding;
}

export type { NativeRuntimeInstance };

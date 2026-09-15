import { createRequire } from "node:module";
import type {
  BBCodeScopedOptions,
  HtmlInlineOptions,
  HtmlLinkedOptions,
  TerminalOptions,
} from "./types.js";

type NativeHtmlInlineOptions = Pick<
  HtmlInlineOptions,
  "theme" | "preClass" | "italic" | "includeHighlights" | "highlightLines" | "header"
>;
type NativeHtmlLinkedOptions = Pick<HtmlLinkedOptions, "preClass" | "highlightLines" | "header">;
type NativeTerminalOptions = Pick<
  TerminalOptions,
  "theme" | "background" | "width" | "highlightLines"
>;
type NativeBBCodeScopedOptions = Pick<BBCodeScopedOptions, "highlightLines">;

interface NativeFormatterBase {
  rainbowBrackets?: boolean;
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
  loadLanguage(id: string): void;
  loadLanguagePackage(
    id: string,
    expectedPackageName: string,
    packageJson: string,
    wasm: Uint8Array,
  ): string;
  loadInstalledLanguagePackage(
    id: string,
    expectedPackageName: string,
    packageJson: string,
    wasm: Uint8Array,
  ): string;
  loadLanguageDefinition(spec: NativeLanguageSpec, wasm: Uint8Array): string;
  hasLanguage(id: string): boolean;
  cacheLanguage(id: string, directory?: string, force?: boolean): string;
  highlightEvents(
    source: string,
    language: string,
    rainbowBrackets?: boolean,
    packageResolver?: (packageName: string) => string | undefined,
    wasmResolver?: (language: string, wasmJson: string) => string | undefined,
  ): { events: Uint8Array; unresolved: string[] };
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
  precompileLanguages(names: string[], directory?: string): Promise<boolean>;
  /** `false` once the runtime has read them, which it does on first use. */
  configureStore(dataDir?: string): boolean;
  /** Where the store lives when `LUMIS_DATA_DIR` names nothing. */
  defaultDataDir(): string;
}

let cachedBinding: NativeBinding | null | undefined;

function linuxLibc(): "gnu" | "musl" {
  const report = process.report?.getReport?.() as
    | { header?: { glibcVersionRuntime?: string } }
    | undefined;
  return report?.header?.glibcVersionRuntime ? "gnu" : "musl";
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
      const binding = require(candidate) as NativeBinding;
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

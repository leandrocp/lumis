import type { Language, PredicateStep, Query as TreeSitterQuery } from "web-tree-sitter";
import satisfies from "semver/functions/satisfies.js";
import minVersion from "semver/ranges/min-version.js";
import { buildHighlightEvents } from "../events.js";
import { LANGUAGES } from "../generated/languages-meta.js";
import { cloneLanguageInfo, normalizeLanguageName } from "../catalog-metadata.js";
import { LANGUAGE_PACKAGE_VERSION_RANGE } from "../generated/package-version-range.js";
import { sha256 } from "@noble/hashes/sha2.js";
import { HIGHLIGHT_NAMES } from "../highlights.js";
import type { RuntimeEnvironment } from "../runtime/runtime.js";
import type {
  CaptureMetadata,
  CompiledBracketConfig,
  CompiledHighlightConfig,
  Formatter,
  HighlightOptions,
  LanguageDefinition,
  LoadedLanguage,
  QueryCaptureOffset,
  SyntaxHighlightEvent,
  WasmRef,
} from "../types.js";
import { PLAINTEXT_LANG_ID, type LanguageInfo } from "../types.js";

export type WasmResolver = (language: string, wasm: WasmRef) => string | URL;
export type LanguagePackageResolver = (
  packageName: string,
  /** npm range compatible with this runtime's Tree-sitter ABI. */
  versionRange: string,
) => string | URL;

export interface PackagedLanguage {
  aliases: string[];
  highlights: string;
  injections?: string;
  locals?: string;
  brackets?: string;
}

/**
 * A language package as published inside `@lumis-sh/wasm-*`. Mirrors
 * `LanguagePackage` in `crates/lumis-wasm-runtime/src/package.rs`.
 *
 * There is deliberately no `formatVersion` gate: runtimes resolve this document
 * from a compatible range, so the format is additive-only by contract and
 * compatibility is decided by shape.
 */
export interface LanguagePackage {
  packageName: string;
  version: string;
  definitionHash: string;
  parser: {
    name: string;
    grammarName: string;
    upstreamVersion?: string;
    revision?: string;
    sha256: string;
    size: number;
  };
  languages: Record<string, PackagedLanguage>;
}

export interface ResolvedLanguagePackage {
  definition: LanguageDefinition;
  wasm: WasmRef;
  grammarName: string;
  highlights: string;
  injections?: string;
  locals?: string;
  brackets?: string;
}

export interface SharedRuntimeCache {
  parserInit?: Promise<void>;
  wasmBytes: Map<string, Uint8Array>;
  wasmLoads: Map<string, Promise<Uint8Array>>;
  packages: Map<string, LanguagePackage>;
  packageLoads: Map<string, Promise<LanguagePackage>>;
}

function compileBracketConfig(
  language: Language,
  Query: typeof TreeSitterQuery,
  bracketsQuery?: string,
): CompiledBracketConfig | undefined {
  if (!bracketsQuery) return undefined;

  // A bracket query can name tokens a grammar lacks, such as "(" in HTML, so a
  // compile failure means "no rainbow brackets here" rather than a load failure.
  let query: InstanceType<typeof Query>;
  try {
    query = new Query(language, bracketsQuery);
  } catch {
    return undefined;
  }

  const captureMetadata: CompiledBracketConfig["captureMetadata"] = {};

  for (const captureName of query.captureNames) {
    captureMetadata[captureName] = {
      isOpen: matchesSpecialCapture(captureName, "open"),
      isClose: matchesSpecialCapture(captureName, "close"),
    };
  }

  // `(#set! rainbow.exclude)` has no value, so the property is stored as
  // `{ "rainbow.exclude": null }`. Detect it by key presence, not by value.
  const rainbowExcludePatterns = Array.from({ length: query.patternCount() }, (_, patternIndex) => {
    const properties = query.setProperties[patternIndex];
    return properties != null && "rainbow.exclude" in properties;
  });

  return { query, captureMetadata, rainbowExcludePatterns };
}

export interface LoadLanguageOptions {
  definition: LanguageDefinition;
  packageName?: string;
  /** Where the parser bytes come from. Queries always come from the package. */
  wasm?: WasmRef | Uint8Array | ArrayBuffer | string | URL | Response;
}

export interface HighlighterRuntimeOptions {
  wasmResolver?: WasmResolver;
  languagePackageResolver?: LanguagePackageResolver;
  sharedCache?: SharedRuntimeCache;
}

export interface RuntimeLike {
  configureWasmResolver(fn: WasmResolver): void;
  configureLanguagePackageResolver(fn: LanguagePackageResolver): void;
  initParser(): Promise<void>;
  registerLanguage(def: LanguageDefinition): void;
  resolveLanguageId(nameOrAlias: string): string;
  getLoadedLanguage(nameOrAlias: string): LoadedLanguage | undefined;
  getLoadedLanguageIds(): string[];
  loadLanguage(opts: LoadLanguageOptions): Promise<LoadedLanguage>;
  resolveLanguagePackage(
    language: LanguageDefinition,
    packageName: string,
  ): Promise<ResolvedLanguagePackage>;
  /** Parser bytes for `wasm`, resolved through this runtime's resolver, verified and cached. */
  resolveParserWasm(language: string, wasm: WasmRef): Promise<Uint8Array>;
  loadPlaintext(): Promise<LoadedLanguage>;
  highlightEvents(
    source: string,
    language: LoadedLanguage,
    options?: { rainbowBrackets?: boolean },
  ): SyntaxHighlightEvent[];
  format?(
    source: string,
    language: LoadedLanguage,
    formatter: Formatter,
    options?: HighlightOptions,
  ): string | undefined;
  formatAsync?(
    source: string,
    language: LoadedLanguage,
    formatter: Formatter,
    options?: HighlightOptions,
  ): Promise<string | undefined>;
}

export interface LanguagesModule {
  createRuntime(options?: HighlighterRuntimeOptions): RuntimeLike;
  configureWasmResolver(fn: WasmResolver): void;
  configureLanguagePackageResolver(fn: LanguagePackageResolver): void;
  initParser(): Promise<void>;
  registerLanguage(def: LanguageDefinition): void;
  resolveLanguageId(nameOrAlias: string): string;
  loadLanguage(opts: LoadLanguageOptions): Promise<LoadedLanguage>;
  resolveLanguagePackage(
    language: LanguageDefinition,
    packageName: string,
  ): Promise<ResolvedLanguagePackage>;
  resolveParserWasm(language: string, wasm: WasmRef): Promise<Uint8Array>;
  loadPlaintext(): Promise<LoadedLanguage>;
  getLoadedLanguage(nameOrAlias: string): LoadedLanguage | undefined;
  getLoadedLanguageIds(): string[];
  availableLanguages(): LanguageInfo[];
  getDefaultRuntime(): RuntimeLike;
}

/** Tried in order; both serve the same `<package>@<version>/<file>` layout. */
export const CDNS = ["https://cdn.jsdelivr.net/npm", "https://unpkg.com"] as const;

/** @internal */
export const DEFAULT_RESOLVER: WasmResolver = (_language, wasm) =>
  `${CDNS[0]}/${wasm.packageName}@${wasm.version}/${wasm.name}.wasm`;
export const DEFAULT_LANGUAGE_PACKAGE_RESOLVER: LanguagePackageResolver = (
  packageName,
  versionRange,
) => `${CDNS[0]}/${packageName}@${versionRange}/lumis.json`;

/** Only used with the default resolver; a custom resolver names one location. */
async function fetchFromCdns(primary: string, isDefault: boolean): Promise<Response> {
  const urls = isDefault ? CDNS.map((base) => primary.replace(CDNS[0], base)) : [primary];
  const failures: string[] = [];
  for (const url of urls) {
    try {
      const response = await fetch(url);
      if (response.ok) return response;
      failures.push(`HTTP ${response.status} ${response.statusText}`.trim());
    } catch (error) {
      failures.push(error instanceof Error ? error.message : String(error));
    }
  }
  throw new Error([...new Set(failures)].join("; "));
}

const HIGHLIGHT_NAMES_SET = new Set(HIGHLIGHT_NAMES);
const PLAINTEXT_ALIASES = LANGUAGES.find(({ id }) => id === PLAINTEXT_LANG_ID)?.aliases ?? [];
const MAX_JSON_CONTAINER_DEPTH = 127;
const JSON_NUMBER = /-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?/y;

const encoder = new TextEncoder();
const decoder = new TextDecoder("utf-8", { fatal: true, ignoreBOM: true });

function createSharedRuntimeCache(): SharedRuntimeCache {
  return {
    wasmBytes: new Map<string, Uint8Array>(),
    wasmLoads: new Map<string, Promise<Uint8Array>>(),
    packages: new Map<string, LanguagePackage>(),
    packageLoads: new Map<string, Promise<LanguagePackage>>(),
  };
}

/** @internal */
export function cacheKey(ref: WasmRef): string {
  return `${ref.name}-${ref.version}-${ref.sha256}`;
}

function isWasmRef(wasm: object): wasm is WasmRef {
  return (
    "packageName" in wasm &&
    "name" in wasm &&
    "version" in wasm &&
    "sha256" in wasm &&
    "size" in wasm
  );
}

function isRuntimeWasmInput(
  wasm: NonNullable<LoadLanguageOptions["wasm"]>,
): wasm is Uint8Array | ArrayBuffer | string | URL | Response {
  return !(typeof wasm === "object" && wasm !== null && isWasmRef(wasm));
}

export function languagePackageCacheKey(packageName: string): string {
  return `language-package-${packageName}`;
}

export function parseLanguagePackage(
  data: Uint8Array,
  expectedPackageName: string,
): LanguagePackage {
  const json = decoder.decode(data);
  if (!hasValidRawJsonProfile(json)) {
    throw new Error(`Invalid Lumis language package: ${expectedPackageName}`);
  }
  const parsed: unknown = JSON.parse(json);
  if (!hasOnlyUnicodeScalarStrings(parsed)) {
    throw new Error(`Invalid Lumis language package: ${expectedPackageName}`);
  }
  const value = parseLanguagePackageValue(parsed, expectedPackageName);
  const owners = new Map<string, string>();
  for (const [id, language] of Object.entries(value.languages)) {
    const owner = normalizeLanguageName(id);
    if (owners.has(owner)) {
      throw new Error(`Invalid Lumis language package: ${expectedPackageName}`);
    }
    owners.set(owner, owner);
    for (const alias of language.aliases) {
      const existing = owners.get(normalizeLanguageName(alias));
      if (existing !== undefined && existing !== owner) {
        throw new Error(`Invalid Lumis language package: ${expectedPackageName}`);
      }
      owners.set(normalizeLanguageName(alias), owner);
    }
  }
  return value;
}

function invalidLanguagePackage(expectedPackageName: string): never {
  throw new Error(`Invalid Lumis language package: ${expectedPackageName}`);
}

function requireObject(value: unknown, expectedPackageName: string): object {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    return invalidLanguagePackage(expectedPackageName);
  }
  return value;
}

function property(value: object, key: string): unknown {
  const result: unknown = Reflect.get(value, key);
  return result;
}

function requireString(value: unknown, expectedPackageName: string): string {
  return typeof value === "string" ? value : invalidLanguagePackage(expectedPackageName);
}

function optionalString(value: unknown, expectedPackageName: string): string | undefined {
  if (value === undefined) return undefined;
  return requireString(value, expectedPackageName);
}

function optionalNullableString(value: unknown, expectedPackageName: string): string | undefined {
  if (value === undefined || value === null) return undefined;
  return requireString(value, expectedPackageName);
}

function requireStringArray(value: unknown, expectedPackageName: string): string[] {
  if (!Array.isArray(value)) return invalidLanguagePackage(expectedPackageName);
  return value.map((entry) => requireString(entry, expectedPackageName));
}

function parsePackagedLanguage(value: unknown, expectedPackageName: string): PackagedLanguage {
  const language = requireObject(value, expectedPackageName);
  return {
    aliases: requireStringArray(property(language, "aliases"), expectedPackageName),
    highlights: requireString(property(language, "highlights"), expectedPackageName),
    injections: optionalString(property(language, "injections"), expectedPackageName),
    locals: optionalString(property(language, "locals"), expectedPackageName),
    brackets: optionalString(property(language, "brackets"), expectedPackageName),
  };
}

function parseLanguagePackageValue(value: unknown, expectedPackageName: string): LanguagePackage {
  const packageValue = requireObject(value, expectedPackageName);
  const packageName = requireString(property(packageValue, "packageName"), expectedPackageName);
  const version = requireString(property(packageValue, "version"), expectedPackageName);
  const definitionHash = requireString(
    property(packageValue, "definitionHash"),
    expectedPackageName,
  );
  const parserValue = requireObject(property(packageValue, "parser"), expectedPackageName);
  const parserName = requireString(property(parserValue, "name"), expectedPackageName);
  const grammarName = requireString(property(parserValue, "grammarName"), expectedPackageName);
  const parserSha256 = requireString(property(parserValue, "sha256"), expectedPackageName);
  const size = property(parserValue, "size");
  const languagesValue = requireObject(property(packageValue, "languages"), expectedPackageName);

  if (
    !isValidPackageManifest({
      packageName,
      expectedPackageName,
      version,
      definitionHash,
      parserName,
      grammarName,
      parserSha256,
      size,
    }) ||
    !isPositiveSafeInteger(size)
  ) {
    return invalidLanguagePackage(expectedPackageName);
  }

  const languages: Record<string, PackagedLanguage> = Object.create(null);
  for (const id of Object.keys(languagesValue)) {
    languages[id] = parsePackagedLanguage(property(languagesValue, id), expectedPackageName);
  }
  if (Object.keys(languages).length === 0) return invalidLanguagePackage(expectedPackageName);

  return {
    packageName,
    version,
    definitionHash,
    parser: {
      name: parserName,
      grammarName,
      upstreamVersion: optionalNullableString(
        property(parserValue, "upstreamVersion"),
        expectedPackageName,
      ),
      revision: optionalNullableString(property(parserValue, "revision"), expectedPackageName),
      sha256: parserSha256,
      size,
    },
    languages,
  };
}

// Every field a package manifest has to satisfy before its languages are read.
function isValidPackageManifest(fields: {
  packageName: string;
  expectedPackageName: string;
  version: string;
  definitionHash: string;
  parserName: string;
  grammarName: string;
  parserSha256: string;
  size: unknown;
}): boolean {
  return (
    fields.packageName === fields.expectedPackageName &&
    isValidPackageName(fields.expectedPackageName) &&
    isSafePackagePathSegment(fields.version) &&
    fields.definitionHash.length > 0 &&
    isSafePackagePathSegment(fields.parserName) &&
    fields.grammarName.length > 0 &&
    /^[0-9a-f]{64}$/.test(fields.parserSha256) &&
    isPositiveSafeInteger(fields.size)
  );
}

function isPositiveSafeInteger(value: unknown): value is number {
  return typeof value === "number" && Number.isSafeInteger(value) && value > 0;
}

// Every surrogate has to be half of a pair; a lone one is not a scalar value.
function isUnicodeScalarString(value: string): boolean {
  for (let index = 0; index < value.length; index += 1) {
    const unit = value.charCodeAt(index);
    if (unit >= 0xd800 && unit <= 0xdbff) {
      const next = value.charCodeAt(index + 1);
      if (!(next >= 0xdc00 && next <= 0xdfff)) return false;
      index += 1;
    } else if (unit >= 0xdc00 && unit <= 0xdfff) {
      return false;
    }
  }
  return true;
}

// JSON.parse discards overwritten members and turns 1e400 into Infinity, so
// validate raw tokens before it collapses the document.
function hasValidRawJsonProfile(json: string): boolean {
  let depth = 0;
  let index = 0;
  while (index < json.length) {
    const character = json[index]!;

    if (character === '"') {
      const end = scanJsonString(json, index);
      if (end < 0) return false;
      index = end;
      continue;
    }

    if (isJsonNumberStart(character)) {
      const end = scanJsonNumber(json, index);
      if (end < 0) return false;
      if (end > index) {
        index = end;
        continue;
      }
    } else {
      depth += containerDepthDelta(character);
      if (depth > MAX_JSON_CONTAINER_DEPTH) return false;
    }

    index += 1;
  }
  return true;
}

function isJsonNumberStart(character: string): boolean {
  return character === "-" || (character >= "0" && character <= "9");
}

function containerDepthDelta(character: string): number {
  if (character === "{" || character === "[") return 1;
  if (character === "}" || character === "]") return -1;
  return 0;
}

// The index just past the string literal at `start`, or -1 when it is
// unterminated or carries a lone surrogate.
function scanJsonString(json: string, start: number): number {
  let end = start + 1;
  while (end < json.length && json[end] !== '"') {
    end += json[end] === "\\" ? 2 : 1;
  }
  if (end >= json.length) return -1;

  let decoded: unknown;
  try {
    decoded = JSON.parse(json.slice(start, end + 1));
  } catch {
    return -1;
  }

  return hasOnlyUnicodeScalarStrings(decoded) ? end + 1 : -1;
}

// The index just past the number at `start`, `start` itself when no number
// begins there, or -1 when the number does not survive as a finite value.
function scanJsonNumber(json: string, start: number): number {
  JSON_NUMBER.lastIndex = start;
  const number = JSON_NUMBER.exec(json);
  if (number === null) return start;

  return Number.isFinite(Number(number[0])) ? JSON_NUMBER.lastIndex : -1;
}

function hasOnlyUnicodeScalarStrings(value: unknown): boolean {
  if (typeof value === "string") return isUnicodeScalarString(value);
  if (Array.isArray(value)) return value.every((entry) => hasOnlyUnicodeScalarStrings(entry));
  if (typeof value === "object" && value !== null) {
    return Object.entries(value).every(
      ([key, child]) => hasOnlyUnicodeScalarStrings(key) && hasOnlyUnicodeScalarStrings(child),
    );
  }
  return true;
}

function isValidPackageName(value: string): boolean {
  if (value.length === 0 || encoder.encode(value).byteLength > 214) return false;

  let segments: string[];
  if (value.startsWith("@")) {
    const scoped = value.slice(1);
    const slash = scoped.indexOf("/");
    if (slash < 0 || scoped.includes("/", slash + 1)) return false;
    segments = [scoped.slice(0, slash), scoped.slice(slash + 1)];
  } else {
    if (value.includes("/")) return false;
    segments = [value];
  }
  return segments.every((segment) => /^[a-z0-9][a-z0-9._-]*$/.test(segment));
}

export { normalizeLanguageName };

function isSafePackagePathSegment(value: string): boolean {
  const stem = value.split(".", 1)[0]!.replace(/[ .]+$/, "");
  let hasForbiddenCharacter = false;
  for (let index = 0; index < value.length; index += 1) {
    if (value.charCodeAt(index) <= 0x1f || '<>:"/\\|?*'.includes(value[index]!)) {
      hasForbiddenCharacter = true;
      break;
    }
  }
  return (
    value !== "" &&
    value !== "." &&
    value !== ".." &&
    !/[ .]$/.test(value) &&
    !hasForbiddenCharacter &&
    !/^(?:con|prn|aux|nul|clock\$|conin\$|conout\$|com[1-9¹²³]|lpt[1-9¹²³])$/i.test(stem)
  );
}

export function serializeLanguagePackageCache(packageMetadata: LanguagePackage): Uint8Array {
  return encoder.encode(JSON.stringify(packageMetadata));
}

/** @internal */
export function isCompatibleLanguagePackageVersion(version: string): boolean {
  return satisfies(version, LANGUAGE_PACKAGE_VERSION_RANGE);
}

/**
 * The lowest version {@link LANGUAGE_PACKAGE_VERSION_RANGE} accepts.
 *
 * Fixtures and staged packages need one concrete version the runtimes will
 * serve. Deriving it from the range keeps them correct if the range ever stops
 * being a bare `MAJOR.MINOR` series, which appending `.0` to it would not.
 *
 * @internal
 */
export function lowestCompatibleLanguagePackageVersion(): string {
  const lowest = minVersion(LANGUAGE_PACKAGE_VERSION_RANGE);
  if (!lowest) {
    throw new Error(`no version satisfies the supported range ${LANGUAGE_PACKAGE_VERSION_RANGE}`);
  }
  return lowest.version;
}

function incompatiblePackageVersion(packageMetadata: LanguagePackage): Error {
  return new Error(
    `Language package ${packageMetadata.packageName}@${packageMetadata.version} does not satisfy the supported range ${LANGUAGE_PACKAGE_VERSION_RANGE}`,
  );
}

function packagedLanguage(
  language: LanguageDefinition,
  packageMetadata: LanguagePackage,
): [string, PackagedLanguage] {
  const requested = normalizeLanguageName(language.id);
  for (const [id, definition] of Object.entries(packageMetadata.languages)) {
    if (
      normalizeLanguageName(id) === requested ||
      definition.aliases.some((alias) => normalizeLanguageName(alias) === requested)
    ) {
      return [id, definition];
    }
  }
  throw new Error(
    `Language "${language.id}" is not provided by ${packageMetadata.packageName}@${packageMetadata.version}`,
  );
}

let treeSitterPromise: Promise<typeof import("web-tree-sitter")> | undefined;

async function loadTreeSitter() {
  treeSitterPromise ??= import("web-tree-sitter");
  return treeSitterPromise;
}

async function trackLoad<T>(
  loads: Map<string, Promise<T>>,
  key: string,
  promise: Promise<T>,
): Promise<T> {
  loads.set(key, promise);

  try {
    return await promise;
  } finally {
    loads.delete(key);
  }
}

function hex(bytes: Uint8Array): string {
  return Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

async function sha256Hex(data: Uint8Array): Promise<string> {
  // Browsers withhold crypto.subtle from non-secure origins, so plain HTTP
  // reaches here with it undefined. Keep the native path: on a 1.6 MiB parser
  // it is 1.3 ms against 35.9 ms, and this runs on every load, cache hits too.
  if (!globalThis.crypto?.subtle) return hex(sha256(data));

  const bytes =
    data.buffer instanceof ArrayBuffer &&
    data.byteOffset === 0 &&
    data.byteLength === data.buffer.byteLength
      ? data.buffer
      : data.slice().buffer;
  return hex(new Uint8Array(await globalThis.crypto.subtle.digest("SHA-256", bytes)));
}

type ParserGrammar = { ok: true; name: string } | { ok: false; detail: string };

interface CachedParserModule {
  grammar?: ParserGrammar;
  language?: Promise<Language>;
}

function inspectParserGrammar(data: Uint8Array): ParserGrammar {
  let module: WebAssembly.Module;
  try {
    const bytes =
      data.buffer instanceof ArrayBuffer &&
      data.byteOffset === 0 &&
      data.byteLength === data.buffer.byteLength
        ? data.buffer
        : data.slice().buffer;
    module = new WebAssembly.Module(bytes);
  } catch (error) {
    return { ok: false, detail: error instanceof Error ? error.message : String(error) };
  }

  const names = WebAssembly.Module.exports(module)
    .filter(({ kind }) => kind === "function")
    .map(({ name }) => name)
    .filter((name) => name.startsWith("tree_sitter_"))
    .map((name) => name.slice("tree_sitter_".length))
    .filter((name) => !name.startsWith("external_scanner_"));
  return names.length === 1
    ? { ok: true, name: names[0]! }
    : { ok: false, detail: `expected one grammar export, got ${names.length}` };
}

async function downloadParserWasm(source: string, languageId: string): Promise<Uint8Array> {
  const response = await fetch(source);
  if (!response.ok) {
    throw new Error(
      `could not download parser WASM for ${languageId}: HTTP ${response.status} ${response.statusText}`,
    );
  }

  return new Uint8Array(await response.arrayBuffer());
}

function requireParserGrammar(
  cached: CachedParserModule,
  data: Uint8Array,
  parser: WasmRef,
  expected: string,
): void {
  cached.grammar ??= inspectParserGrammar(data);
  if (!cached.grammar.ok) {
    throw new Error(`invalid parser grammar export for '${parser.name}': ${cached.grammar.detail}`);
  }
  if (cached.grammar.name !== expected) {
    throw new Error(
      `invalid parser grammar for '${parser.name}': expected '${expected}', got '${cached.grammar.name}'`,
    );
  }
}

/** @internal */
export async function verifyWasm(ref: WasmRef, data: Uint8Array): Promise<Uint8Array> {
  if (data.byteLength !== ref.size) {
    throw new Error(
      `Invalid WASM size for ${ref.name}@${ref.version}: expected ${ref.size}, got ${data.byteLength}`,
    );
  }
  const actual = await sha256Hex(data);
  if (actual !== ref.sha256) {
    throw new Error(
      `Invalid WASM integrity for ${ref.name}@${ref.version}: expected sha256-${ref.sha256}, got sha256-${actual}`,
    );
  }
  return data;
}

function matchesSpecialCapture(name: string, base: string): boolean {
  return name === base;
}

const NON_HIGHLIGHT_CAPTURES = [
  "injection.content",
  "injection.language",
  "local.scope",
  "local.definition",
  "local.definition-value",
  "local.reference",
];

// A helper capture, `@_name` in nvim-treesitter, or one of the captures the
// injection and locals passes own rather than the highlight pass.
function isNonHighlightCapture(name: string): boolean {
  return (
    name.length === 0 ||
    name.startsWith("_") ||
    NON_HIGHLIGHT_CAPTURES.some((special) => matchesSpecialCapture(name, special))
  );
}

function resolveHighlightName(captureName: string): string | undefined {
  const name = captureName.startsWith("@") ? captureName.slice(1) : captureName;

  if (isNonHighlightCapture(name)) return undefined;

  if (HIGHLIGHT_NAMES_SET.has(name)) {
    return name;
  }

  const captureParts = new Set(name.split("."));
  let best: string | undefined;
  let bestLen = 0;

  for (const recognized of HIGHLIGHT_NAMES) {
    const recognizedParts = recognized.split(".");
    if (
      recognizedParts.length > bestLen &&
      recognizedParts.every((part) => captureParts.has(part))
    ) {
      best = recognized;
      bestLen = recognizedParts.length;
    }
  }

  return best;
}

const DECIMAL_OFFSET = /^[+-]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE]([+-]?\d+))?$/;
const BINARY_OFFSET = /^([+-]?)0[bB]([01]+)$/;
const HEX_OFFSET = /^([+-]?)0[xX]([\da-fA-F]+(?:\.[\da-fA-F]*)?|\.[\da-fA-F]+)(?:[pP]([+-]?\d+))?$/;
const MAX_LUA_EXPONENT = "1048575";

function isLuaExponent(value: string | undefined): boolean {
  if (value === undefined) return true;
  const magnitude = value.replace(/^[+-]/, "").replace(/^0+/, "");
  return (
    magnitude.length < MAX_LUA_EXPONENT.length ||
    (magnitude.length === MAX_LUA_EXPONENT.length && magnitude <= MAX_LUA_EXPONENT)
  );
}

function parseBinaryOffset(match: RegExpExecArray): number {
  let value = 0;
  for (const digit of match[2]!) {
    value = value * 2 + Number(digit);
  }
  return match[1] === "-" ? -value : value;
}

const SIGNIFICAND_BITS = 52;
const MINIMUM_SUBNORMAL_EXPONENT = -1074;
const INFINITY_BITS = 0x7ffn << 52n;

function parseHexOffset(match: RegExpExecArray): number {
  const exponent = { value: Number(match[3] ?? 0) };
  const mantissa = parseHexMantissa(match[2]!, exponent);
  const negative = match[1] === "-";

  if (mantissa.significand === 0n) return negative ? -0 : 0;

  const significand = mantissa.inexact ? mantissa.significand | 1n : mantissa.significand;
  const rounded = roundHexSignificand(significand, exponent);
  return hexBitsToFloat(rounded, exponent, negative);
}

// Accumulates the mantissa's hex digits, adjusting `exponent` for the radix
// point and for any digit past the 128 bits the accumulator holds.
function parseHexMantissa(
  mantissa: string,
  exponent: { value: number },
): { significand: bigint; inexact: boolean } {
  let significand = 0n;
  let seenPoint = false;
  let inexact = false;

  for (const character of mantissa) {
    if (character === ".") {
      seenPoint = true;
      continue;
    }

    const digit = BigInt(Number.parseInt(character, 16));
    if (significand >> 124n === 0n) {
      significand = (significand << 4n) | digit;
    } else {
      exponent.value += 4;
      inexact ||= digit !== 0n;
    }
    if (seenPoint) exponent.value -= 4;
  }

  return { significand, inexact };
}

// Rounds the significand to the 53 bits an f64 holds, to nearest and ties to
// even, adjusting `exponent` by the same amount.
function roundHexSignificand(significand: bigint, exponent: { value: number }): bigint {
  let roundBits = significand.toString(2).length - 1 - SIGNIFICAND_BITS;
  if (exponent.value < MINIMUM_SUBNORMAL_EXPONENT - roundBits) {
    roundBits = MINIMUM_SUBNORMAL_EXPONENT - exponent.value;
  }
  exponent.value += roundBits;

  if (roundBits < 0) return significand << BigInt(-roundBits);
  if (roundBits === 0) return significand;

  const shifted = shiftForRounding(significand, roundBits);
  const trailing = Number(shifted & 0b111n);
  return (shifted >> 2n) + BigInt((0b11001000 >> trailing) & 1);
}

// Leaves the three bits rounding needs: the last kept bit, the round bit, and a
// sticky bit standing for everything discarded below them.
function shiftForRounding(significand: bigint, roundBits: number): bigint {
  if (roundBits === 1) return significand << 1n;
  if (roundBits <= 2) return significand;

  const shift = roundBits - 2;
  if (shift >= 128) return 1n;

  const discarded = significand & ((1n << BigInt(shift)) - 1n);
  return (significand >> BigInt(shift)) | BigInt(discarded !== 0n);
}

function hexBitsToFloat(
  significand: bigint,
  exponent: { value: number },
  negative = false,
): number {
  const encodedExponent = BigInt(exponent.value - MINIMUM_SUBNORMAL_EXPONENT) << 52n;
  let bits = significand + encodedExponent;
  if (bits >= INFINITY_BITS) bits = INFINITY_BITS;
  if (negative) bits |= 1n << 63n;

  const buffer = new ArrayBuffer(8);
  const view = new DataView(buffer);
  view.setBigUint64(0, bits);
  return view.getFloat64(0);
}

function parseOffsetDelta(input: string): number | undefined {
  const value = input.replace(/^[\t\n\v\f\r ]+/, "").replace(/[\t\n\v\f\r ]+$/, "");
  if (value.length === 0) return undefined;

  const binary = BINARY_OFFSET.exec(value);
  const hexadecimal = HEX_OFFSET.exec(value);
  const decimal = DECIMAL_OFFSET.exec(value);
  let parsed = NaN;
  if (binary) {
    parsed = parseBinaryOffset(binary);
  } else if (hexadecimal && isLuaExponent(hexadecimal[3])) {
    parsed = parseHexOffset(hexadecimal);
  } else if (decimal && isLuaExponent(decimal[1])) {
    parsed = Number(value);
  }

  // LuaJIT accepts fractional and non-finite numeric strings, but Neovim's
  // highlighter cannot turn their coordinates into integral extmarks.
  return Number.isSafeInteger(parsed) ? (parsed === 0 ? 0 : parsed) : undefined;
}

export function compileHighlightConfig(
  language: Language,
  Query: typeof TreeSitterQuery,
  highlightsQuery: string,
  injectionsQuery = "",
  localsQuery = "",
): CompiledHighlightConfig {
  const querySource = `${injectionsQuery}${localsQuery}${highlightsQuery}`;
  const localsQueryOffset = injectionsQuery.length;
  const highlightsQueryOffset = injectionsQuery.length + localsQuery.length;
  const query = new Query(language, querySource);

  let injectionPatternEnd = 0;
  let localsPatternEnd = 0;
  for (let i = 0; i < query.patternCount(); i += 1) {
    const patternOffset = query.startIndexForPattern(i);
    if (patternOffset < highlightsQueryOffset) {
      localsPatternEnd += 1;
    }
    if (patternOffset < localsQueryOffset) {
      injectionPatternEnd += 1;
    }
  }

  const captureMetadata: Record<string, CaptureMetadata> = {};

  for (const captureName of query.captureNames) {
    captureMetadata[captureName] = {
      highlightScope: resolveHighlightName(captureName),
      isInjectionContent: matchesSpecialCapture(captureName, "injection.content"),
      isInjectionLanguage: matchesSpecialCapture(captureName, "injection.language"),
      isInjectionFilename: matchesSpecialCapture(captureName, "injection.filename"),
      isLocalScope: matchesSpecialCapture(captureName, "local.scope"),
      isLocalDefinition: matchesSpecialCapture(captureName, "local.definition"),
      isLocalDefinitionValue: matchesSpecialCapture(captureName, "local.definition-value"),
      isLocalReference: matchesSpecialCapture(captureName, "local.reference"),
    };
  }

  const nonLocalVariablePatterns = Array.from({ length: query.patternCount() }, (_, index) => {
    const refuted = query.refutedProperties[index] ?? {};
    return Object.prototype.hasOwnProperty.call(refuted, "local");
  });

  /**
   * Neovim reads exactly `pred[3]` through `pred[6]` and never looks further,
   * so an omitted operand is zero and anything after the fourth is untouched —
   * including a non-numeric one. The operands use LuaJIT's numeric-string
   * coercion, narrowed only where an integral Tree-sitter point cannot hold the
   * result.
   */
  function parseOffsetDeltas(deltas: PredicateStep[]): QueryCaptureOffset | undefined {
    const values = [0, 0, 0, 0];

    for (const [index, delta] of deltas.slice(0, values.length).entries()) {
      let value: number | undefined;
      if (delta.type === "capture") {
        const captureIndex = query.captureIndexForName(delta.name);
        value = captureIndex >= 0 ? captureIndex + 1 : undefined;
      } else {
        value = parseOffsetDelta(delta.value);
      }
      if (value === undefined) return undefined;
      values[index] = value;
    }

    return {
      startRow: values[0] as number,
      startColumn: values[1] as number,
      endRow: values[2] as number,
      endColumn: values[3] as number,
    };
  }

  // Neovim applies `#offset!` to injection ranges *and* highlight ranges, so this
  // collects offsets for every pattern, not just the injection ones. It has to
  // agree with `parse_offset_operands` in `crates/lumis-wasm-runtime`.
  const captureOffsets = Array.from(
    { length: query.patternCount() },
    (_, patternIndex): Record<string, QueryCaptureOffset> | undefined => {
      let offsets: Record<string, QueryCaptureOffset> | undefined;

      for (const predicate of query.predicatesForPattern(patternIndex) ?? []) {
        if (predicate.operator !== "offset!") continue;

        const [captureStep, ...deltas] = predicate.operands;
        if (captureStep?.type !== "capture") continue;

        const parsed = parseOffsetDeltas(deltas);
        if (!parsed) continue;

        offsets ??= {};
        offsets[captureStep.name] = parsed;
      }

      return offsets;
    },
  );

  return {
    query,
    injectionPatternEnd,
    localsPatternEnd,
    captureMetadata,
    nonLocalVariablePatterns,
    captureOffsets,
  };
}

export function createLanguagesModule(runtime: RuntimeEnvironment): LanguagesModule {
  let configuredDefaultResolver: WasmResolver = DEFAULT_RESOLVER;
  let configuredLanguagePackageResolver: LanguagePackageResolver =
    DEFAULT_LANGUAGE_PACKAGE_RESOLVER;
  const moduleCache = createSharedRuntimeCache();
  const parserModules = new Map<string, CachedParserModule>();

  // One tree-sitter Language per parser digest, shared across every language the
  // parser serves. web-tree-sitter cannot reclaim a failed dynamic-linker load,
  // so retaining the rejection keeps identical bad bytes from growing its global
  // store.
  async function loadParserLanguage(
    Language: Awaited<ReturnType<typeof loadTreeSitter>>["Language"],
    wasmInput: Uint8Array,
    packaged: { wasm: WasmRef; grammarName: string },
  ) {
    const parserKey = packaged.wasm.sha256;
    let parserModule = parserModules.get(parserKey);
    if (!parserModule) {
      parserModule = {};
      parserModules.set(parserKey, parserModule);
    }

    requireParserGrammar(parserModule, wasmInput, packaged.wasm, packaged.grammarName);
    parserModule.language ??= Language.load(wasmInput);
    return parserModule.language;
  }

  class HighlighterRuntime implements RuntimeLike {
    private explicitResolver: WasmResolver | undefined;
    private explicitLanguagePackageResolver: LanguagePackageResolver | undefined;
    private readonly sharedCache: SharedRuntimeCache;
    private readonly loadedLanguages = new Map<string, LoadedLanguage>();
    private readonly aliasMap = new Map<string, string>();
    private readonly languageLoads = new Map<string, Promise<LoadedLanguage>>();
    private readonly bracketCompilers = new WeakMap<
      LoadedLanguage,
      () => CompiledBracketConfig | undefined
    >();

    constructor(options: HighlighterRuntimeOptions = {}) {
      this.explicitResolver = options.wasmResolver;
      this.explicitLanguagePackageResolver = options.languagePackageResolver;
      this.sharedCache = options.sharedCache ?? moduleCache;

      for (const alias of PLAINTEXT_ALIASES) {
        this.aliasMap.set(normalizeLanguageName(alias), PLAINTEXT_LANG_ID);
      }
    }

    private get resolver(): WasmResolver {
      return this.explicitResolver ?? configuredDefaultResolver;
    }

    private get languagePackageResolver(): LanguagePackageResolver {
      return this.explicitLanguagePackageResolver ?? configuredLanguagePackageResolver;
    }

    private acceptsPackage(packageMetadata: LanguagePackage): boolean {
      return isCompatibleLanguagePackageVersion(packageMetadata.version);
    }

    private async readCachedLanguagePackage(
      packageName: string,
    ): Promise<LanguagePackage | undefined> {
      const bytes = await runtime.readFsCache(languagePackageCacheKey(packageName));
      if (!bytes) return undefined;
      try {
        return parseLanguagePackage(bytes, packageName);
      } catch {
        return undefined;
      }
    }

    private async loadInstalledLanguagePackage(
      packageName: string,
    ): Promise<LanguagePackage | undefined> {
      try {
        const mod = await import(
          /* webpackIgnore: true */
          /* turbopackIgnore: true */
          /* @vite-ignore */
          packageName
        );
        const base: unknown = mod.default;
        if (!(base instanceof URL) && typeof base !== "string") return undefined;
        const source = new URL("./lumis.json", base instanceof URL ? base : new URL(base));
        const disk = await runtime.readResolvedWasmFromDisk(source);
        if (disk) return parseLanguagePackage(disk, packageName);
        const response = await fetch(source);
        if (!response.ok) return undefined;
        return parseLanguagePackage(new Uint8Array(await response.arrayBuffer()), packageName);
      } catch {
        return undefined;
      }
    }

    private async fetchLanguagePackage(packageName: string): Promise<LanguagePackage> {
      const resolver = this.languagePackageResolver;
      const source = resolver(packageName, LANGUAGE_PACKAGE_VERSION_RANGE);
      const disk = await runtime.readResolvedWasmFromDisk(source);
      if (disk) {
        const packageMetadata = parseLanguagePackage(disk, packageName);
        if (this.acceptsPackage(packageMetadata)) return packageMetadata;
        throw incompatiblePackageVersion(packageMetadata);
      }
      const href = typeof source === "string" ? source : source.href;
      const response = await fetchFromCdns(
        href,
        resolver === DEFAULT_LANGUAGE_PACKAGE_RESOLVER,
      ).catch((error: Error) => {
        throw new Error(`could not download language package ${packageName}: ${error.message}`);
      });
      const packageMetadata = parseLanguagePackage(
        new Uint8Array(await response.arrayBuffer()),
        packageName,
      );
      if (!this.acceptsPackage(packageMetadata)) throw incompatiblePackageVersion(packageMetadata);
      return packageMetadata;
    }

    private async resolvePackage(packageName: string): Promise<LanguagePackage> {
      // Every branch below either returns a package this runtime accepts or
      // throws, so what the shared cache holds is already compatible. Rechecking
      // it here would start a second load for the same name and leave two
      // callers of one in-flight request with different results.
      const memory = this.sharedCache.packages.get(packageName);
      if (memory) return memory;
      const inFlight = this.sharedCache.packageLoads.get(packageName);
      if (inFlight) return inFlight;

      const load = (async () => {
        const staged = await runtime.readStagedAsset?.(
          `${packageName.replace(/^@lumis-sh\/wasm-/, "")}.lumis.json`,
        );
        if (staged) {
          const packageMetadata = parseLanguagePackage(staged, packageName);
          if (this.acceptsPackage(packageMetadata)) return packageMetadata;
        }

        const installed = await this.loadInstalledLanguagePackage(packageName);
        if (installed && this.acceptsPackage(installed)) return installed;

        const cached = await this.readCachedLanguagePackage(packageName);
        if (cached && this.acceptsPackage(cached)) return cached;

        const packageMetadata = await this.fetchLanguagePackage(packageName);
        await runtime.writeFsCache(
          languagePackageCacheKey(packageName),
          serializeLanguagePackageCache(packageMetadata),
        );
        return packageMetadata;
      })().then((value) => {
        this.sharedCache.packages.set(packageName, value);
        return value;
      });

      return trackLoad(this.sharedCache.packageLoads, packageName, load);
    }

    private async readVerifiedCache(ref: WasmRef, key: string): Promise<Uint8Array | undefined> {
      const fsCached = await runtime.readFsCache(key);
      if (fsCached) {
        try {
          return await verifyWasm(ref, fsCached);
        } catch {
          // Corrupt or stale entries are replaced atomically by the locked writer.
        }
      }
      return undefined;
    }

    private async loadInstalledPackage(ref: WasmRef): Promise<Uint8Array | undefined> {
      try {
        const mod = await import(
          /* webpackIgnore: true */
          /* turbopackIgnore: true */
          /* @vite-ignore */
          ref.packageName
        );
        const input: unknown = mod.default;
        if (
          input instanceof Uint8Array ||
          input instanceof ArrayBuffer ||
          input instanceof URL ||
          typeof input === "string"
        ) {
          const resolved = await runtime.resolveWasm(input);
          if (resolved instanceof Uint8Array) return resolved;
          const disk = await runtime.readResolvedWasmFromDisk(resolved);
          if (disk) return disk;
          const response = await fetch(resolved);
          if (!response.ok) return undefined;
          return new Uint8Array(await response.arrayBuffer());
        }
      } catch {}
      return undefined;
    }

    private async fetchResolvedWasm(language: string, ref: WasmRef): Promise<Uint8Array> {
      // Verified here rather than by the caller: the store directory holds both
      // staged and downloaded parsers, so a corrupt file found this way has to
      // fall through to a refetch instead of failing the load. `cached_parser`
      // in the Rust store discards such a file for the same reason.
      const staged = await runtime.readStagedAsset?.(
        `${ref.name}-${ref.version}-${ref.sha256}.wasm`,
      );
      if (staged) {
        try {
          return await verifyWasm(ref, staged);
        } catch {}
      }

      const installed = await this.loadInstalledPackage(ref);
      if (installed) return installed;

      const url = this.resolver(language, ref);
      const diskData = await runtime.readResolvedWasmFromDisk(url);
      if (diskData) return diskData;
      const href = typeof url === "string" ? url : url.href;
      const response = await fetchFromCdns(href, this.resolver === DEFAULT_RESOLVER).catch(
        (error: Error) => {
          throw new Error(
            `could not download parser WASM ${ref.name}@${ref.version}: ${error.message}`,
          );
        },
      );
      return new Uint8Array(await response.arrayBuffer());
    }

    private async loadWasmBytes(language: string, ref: WasmRef, key: string): Promise<Uint8Array> {
      const fsCached = await this.readVerifiedCache(ref, key);
      if (fsCached) return fsCached;

      return runtime.withFsCacheLock(key, async () => {
        const lockedCache = await this.readVerifiedCache(ref, key);
        if (lockedCache) return lockedCache;

        const data = await verifyWasm(ref, await this.fetchResolvedWasm(language, ref));
        await runtime.writeFsCache(key, data);
        return data;
      });
    }

    // A package reference resolves through the store; anything else is a source
    // the runtime knows how to reach.
    private async readParserWasm(
      resolved: { wasm: NonNullable<LoadLanguageOptions["wasm"]>; definition: { id: string } },
      requestedId: string,
    ): Promise<Uint8Array> {
      const { wasm } = resolved;
      if (typeof wasm === "object" && isWasmRef(wasm)) {
        return this.resolveParserWasm(resolved.definition.id, wasm);
      }
      if (!isRuntimeWasmInput(wasm)) {
        throw new Error(`Unsupported WASM input for language "${requestedId}"`);
      }

      const source = await runtime.resolveWasm(wasm);
      if (source instanceof Uint8Array) return source;

      const disk = await runtime.readResolvedWasmFromDisk(source);
      if (disk) return disk;

      return downloadParserWasm(source, resolved.definition.id);
    }

    private async createLoadedLanguage(opts: LoadLanguageOptions): Promise<LoadedLanguage> {
      await this.initParser();

      // Queries always come from the package, so a parser can only ever run
      // against the queries it was released and tested with.
      if (!opts.packageName) {
        throw new Error(`Language "${opts.definition.id}" has no packageName`);
      }

      const packaged = await this.resolveLanguagePackage(opts.definition, opts.packageName);
      const resolved = { ...packaged, ...(opts.wasm === undefined ? {} : { wasm: opts.wasm }) };
      if (!resolved.wasm || resolved.highlights === undefined) {
        throw new Error(`Language package "${opts.packageName}" has no parser or highlights query`);
      }

      const wasmInput = await this.readParserWasm(resolved, opts.definition.id);
      // Always, including when the caller chose where the bytes come from:
      // `withWasm()` selects a source, it does not waive the package's digest.
      await verifyWasm(packaged.wasm, wasmInput);

      const { Language, Parser, Query } = await loadTreeSitter();
      const language = await loadParserLanguage(Language, wasmInput, packaged);
      const config = compileHighlightConfig(
        language,
        Query,
        resolved.highlights,
        resolved.injections,
        resolved.locals,
      );
      let parser: InstanceType<typeof Parser> | undefined;
      try {
        parser = new Parser();
        parser.setLanguage(language);
      } catch (error) {
        config.query.delete();
        parser?.delete();
        throw error;
      }

      const loaded: LoadedLanguage = {
        definition: resolved.definition,
        parser,
        language,
        config,
      };
      if (resolved.brackets) {
        this.bracketCompilers.set(loaded, () =>
          compileBracketConfig(language, Query, resolved.brackets),
        );
      }

      this.loadedLanguages.set(resolved.definition.id, loaded);
      this.registerLanguage(resolved.definition);
      return loaded;
    }

    configureWasmResolver(fn: WasmResolver): void {
      this.explicitResolver = fn;
    }

    configureLanguagePackageResolver(fn: LanguagePackageResolver): void {
      this.explicitLanguagePackageResolver = fn;
    }

    async resolveLanguagePackage(
      language: LanguageDefinition,
      packageName: string,
    ): Promise<ResolvedLanguagePackage> {
      const packageMetadata = await this.resolvePackage(packageName);
      const [id, packaged] = packagedLanguage(language, packageMetadata);
      return {
        definition: { id, aliases: packaged.aliases },
        wasm: {
          packageName: packageMetadata.packageName,
          name: packageMetadata.parser.name,
          version: packageMetadata.version,
          sha256: packageMetadata.parser.sha256,
          size: packageMetadata.parser.size,
        },
        grammarName: packageMetadata.parser.grammarName,
        highlights: packaged.highlights,
        injections: packaged.injections,
        locals: packaged.locals,
        brackets: packaged.brackets,
      };
    }

    async initParser(): Promise<void> {
      this.sharedCache.parserInit ??= Promise.all([
        loadTreeSitter(),
        runtime.parserInitOptions?.() ?? Promise.resolve(),
      ]).then(([{ Parser }, initOptions]) => Parser.init(initOptions));
      await this.sharedCache.parserInit;
    }

    registerLanguage(def: LanguageDefinition): void {
      this.aliasMap.set(normalizeLanguageName(def.id), def.id);
      for (const alias of def.aliases) {
        this.aliasMap.set(normalizeLanguageName(alias), def.id);
      }
    }

    resolveLanguageId(nameOrAlias: string): string {
      return this.aliasMap.get(normalizeLanguageName(nameOrAlias)) ?? nameOrAlias;
    }

    getLoadedLanguage(nameOrAlias: string): LoadedLanguage | undefined {
      const id = this.resolveLanguageId(nameOrAlias);
      return this.loadedLanguages.get(id);
    }

    getLoadedLanguageIds(): string[] {
      return [...this.loadedLanguages.keys()];
    }

    async resolveParserWasm(language: string, ref: WasmRef): Promise<Uint8Array> {
      const key = cacheKey(ref);
      const cached = this.sharedCache.wasmBytes.get(key);
      if (cached) return cached;

      const existingLoad = this.sharedCache.wasmLoads.get(key);
      if (existingLoad) {
        return existingLoad;
      }

      const load = this.loadWasmBytes(language, ref, key).then((data) => {
        this.sharedCache.wasmBytes.set(key, data);
        return data;
      });
      return trackLoad(this.sharedCache.wasmLoads, key, load);
    }

    async loadLanguage(opts: LoadLanguageOptions): Promise<LoadedLanguage> {
      if (opts.definition.id === PLAINTEXT_LANG_ID) {
        return this.createPlaintext(opts.definition);
      }
      const existing = this.getLoadedLanguage(opts.definition.id);
      if (existing) return existing;

      const loadKey = normalizeLanguageName(opts.definition.id);
      const inFlight = this.languageLoads.get(loadKey);
      if (inFlight) return inFlight;

      return trackLoad(this.languageLoads, loadKey, this.createLoadedLanguage(opts));
    }

    async loadPlaintext(): Promise<LoadedLanguage> {
      return this.createPlaintext({ id: PLAINTEXT_LANG_ID, aliases: PLAINTEXT_ALIASES });
    }

    private createPlaintext(definition: LanguageDefinition): LoadedLanguage {
      const existing = this.loadedLanguages.get(PLAINTEXT_LANG_ID);
      if (existing) return existing;

      const loaded = { definition } as LoadedLanguage;
      this.loadedLanguages.set(PLAINTEXT_LANG_ID, loaded);
      this.registerLanguage(definition);
      return loaded;
    }

    highlightEvents(
      source: string,
      language: LoadedLanguage,
      options: { rainbowBrackets?: boolean } = {},
    ): SyntaxHighlightEvent[] {
      if (language.definition.id === PLAINTEXT_LANG_ID) {
        return [{ type: "source", startByte: 0, endByte: encoder.encode(source).byteLength }];
      }
      if (options.rainbowBrackets && !language.brackets) {
        const compile = this.bracketCompilers.get(language);
        if (compile) {
          this.bracketCompilers.delete(language);
          language.brackets = compile();
        }
      }
      return buildHighlightEvents(source, language, this, options) as SyntaxHighlightEvent[];
    }
  }

  const defaultRuntime = new HighlighterRuntime();

  return {
    createRuntime(options = {}) {
      return new HighlighterRuntime(options);
    },
    configureWasmResolver(fn) {
      configuredDefaultResolver = fn;
      defaultRuntime.configureWasmResolver(fn);
    },
    configureLanguagePackageResolver(fn) {
      configuredLanguagePackageResolver = fn;
      defaultRuntime.configureLanguagePackageResolver(fn);
    },
    initParser() {
      return defaultRuntime.initParser();
    },
    registerLanguage(def) {
      defaultRuntime.registerLanguage(def);
    },
    resolveLanguageId(nameOrAlias) {
      return defaultRuntime.resolveLanguageId(nameOrAlias);
    },
    loadLanguage(opts) {
      return defaultRuntime.loadLanguage(opts);
    },
    resolveLanguagePackage(language, packageName) {
      return defaultRuntime.resolveLanguagePackage(language, packageName);
    },
    resolveParserWasm(language, wasm) {
      return defaultRuntime.resolveParserWasm(language, wasm);
    },
    loadPlaintext() {
      return defaultRuntime.loadPlaintext();
    },
    getLoadedLanguage(nameOrAlias) {
      return defaultRuntime.getLoadedLanguage(nameOrAlias);
    },
    getLoadedLanguageIds() {
      return defaultRuntime.getLoadedLanguageIds();
    },
    availableLanguages() {
      return LANGUAGES.map((language) => cloneLanguageInfo(language));
    },
    getDefaultRuntime() {
      return defaultRuntime;
    },
  };
}

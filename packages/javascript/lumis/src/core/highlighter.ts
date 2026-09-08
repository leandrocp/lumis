import type {
  HighlightCallback,
  HighlightEvent,
  HighlightOptions,
  Language,
  LanguageBundle,
  LanguageDefinition,
  LoadableLanguage,
  LanguageInput,
  LanguageRef,
  LazyLanguage,
  Formatter,
  SyntaxHighlightEvent,
  Theme,
} from "../types.js";
import { PLAINTEXT_LANG_ID } from "../types.js";
import type { LanguagePackageResolver, RuntimeLike, WasmResolver } from "./languages.js";
import { normalizeLanguageName } from "./languages.js";
import { composeAnnotations } from "../annotations.js";
import { buildSourceIndex } from "../events.js";
import { getScopedThemeStyle } from "../formatter/html.js";
import { LANGUAGE_LOADERS } from "../generated/language-loaders.js";
import { guessLanguage } from "../guess-language.js";
import { builtinFormatterKind } from "./builtin-formatter.js";

const decoder = new TextDecoder();
// Removed from the public API. Named here so an object still carrying one is
// rejected at the boundary rather than loaded with the field quietly dropped.
const LANGUAGE_QUERY_FIELDS = ["highlights", "injections", "locals", "brackets"] as const;
const LANGUAGE_LOAD_FIELDS = ["packageName", ...LANGUAGE_QUERY_FIELDS, "wasm"] as const;
const LANGUAGE_SHAPE_FIELDS = ["id", "aliases", ...LANGUAGE_LOAD_FIELDS] as const;

function decodeSlice(sourceBytes: Uint8Array, startByte: number, endByte: number): string {
  return decoder.decode(sourceBytes.subarray(startByte, endByte));
}

/** A reusable highlighter with loaded or lazily registered languages. */
export interface Highlighter {
  /** Highlight source code synchronously. The language must already be loaded. */
  highlight<T>(source: string, formatter: Formatter<T>, options?: HighlightOptions<T>): string;
  /** Low-level token iterator. Calls `onToken` for each highlighted span. Languages must already be loaded. */
  highlightIter(
    source: string,
    language: LanguageRef | undefined,
    theme: Theme | undefined,
    onToken: HighlightCallback,
    options?: HighlightOptions,
  ): void;
  /** Load a language by object, lazy handle, or string ID from a registered bundle. No-op if already loaded. */
  loadLanguage(language: Language | LazyLanguage | string): Promise<void>;
  /** IDs of languages that have been loaded and are ready to highlight. */
  readonly languages: string[];
  /** IDs of all languages, including those registered lazily from bundles. */
  readonly registeredLanguages: string[];
}

export interface HighlighterModuleFactory {
  createRuntime(options?: {
    wasmResolver?: WasmResolver;
    languagePackageResolver?: LanguagePackageResolver;
  }): RuntimeLike;
  getDefaultRuntime(): RuntimeLike;
}

/** Options for {@link createHighlighter}. */
export interface CreateHighlighterOptions {
  /** Languages to load during setup or register lazily. */
  languages?: LanguageInput[];
  /** Optional resolver for external WASM assets. */
  wasmResolver?: WasmResolver;
  /** Optional resolver for self-contained language package metadata. */
  languagePackageResolver?: LanguagePackageResolver;
}

/**
 * Narrowed to the one method it calls so `loadLanguages()` can share it, rather
 * than growing a second place that decides what a `Language` becomes on the way
 * into a runtime.
 */
export async function loadLanguageDefinition(
  runtime: Pick<RuntimeLike, "loadLanguage">,
  language: Language,
): Promise<void> {
  await runtime.loadLanguage({
    definition: { id: language.id, aliases: language.aliases },
    packageName: language.packageName,
    wasm: language.wasm,
  });
}

/** Resolve a LanguageRef to a language ID string. */
function resolveRefId(ref: LanguageRef | undefined): string {
  if (!ref) return PLAINTEXT_LANG_ID;
  if (typeof ref === "string") return ref;
  return ref.id;
}

function isPlaintextRef(ref?: LanguageRef): boolean {
  return normalizeLanguageName(resolveRefId(ref)) === PLAINTEXT_LANG_ID;
}

function detectLanguageRef(source: string, ref?: LanguageRef): LanguageRef | string {
  if (ref && typeof ref !== "string") {
    validateLanguageBoundary(ref);
    return ref;
  }

  return guessLanguage(ref, source);
}

async function loadBuiltinLanguageById(id: string): Promise<Language | undefined> {
  const loader = LANGUAGE_LOADERS[normalizeLanguageName(id)];
  if (!loader) {
    return undefined;
  }

  const mod = await loader();
  return requireLoadableLanguage(mod.default, `Built-in language "${id}"`);
}

async function ensureLanguageLoaded(
  runtime: RuntimeLike,
  ref: LanguageRef | string,
  lazyRegistry?: Map<string, LazyLanguage>,
): Promise<void> {
  validateLanguageBoundary(ref);

  if (isPlaintextRef(ref)) {
    await runtime.loadPlaintext();
    return;
  }

  if (typeof ref !== "string") {
    if (!runtime.getLoadedLanguage(ref.id)) {
      await loadLanguageRef(runtime, ref, lazyRegistry);
    }
    return;
  }

  const languageId = ref;
  if (runtime.getLoadedLanguage(languageId)) {
    return;
  }

  const lazy = lazyRegistry?.get(normalizeLanguageName(languageId));
  if (lazy) {
    const language = requireLoadableLanguage(await lazy(), `Lazy language "${lazy.id}"`);
    await loadLanguageDefinition(runtime, language);
    return;
  }

  const builtin = await loadBuiltinLanguageById(languageId);
  if (builtin) {
    await loadLanguageDefinition(runtime, builtin);
  }
}

// A `Language` loads directly, a `LazyLanguage` after resolving, and anything
// else by its id.
async function loadLanguageRef(
  runtime: RuntimeLike,
  ref: Exclude<LanguageRef, string>,
  lazyRegistry?: Map<string, LazyLanguage>,
): Promise<void> {
  if (isLanguage(ref)) {
    await loadLanguageDefinition(runtime, ref);
  } else if (isLazyLanguage(ref)) {
    await loadLanguageDefinition(
      runtime,
      requireLoadableLanguage(await ref(), `Lazy language "${ref.id}"`),
    );
  } else {
    await ensureLanguageLoaded(runtime, ref.id, lazyRegistry);
  }
}

function resolveLoadedLanguage(runtime: RuntimeLike, ref?: LanguageRef) {
  const languageId = resolveRefId(ref);
  const loaded = runtime.getLoadedLanguage(languageId);
  if (!loaded) {
    if (languageId === PLAINTEXT_LANG_ID) {
      throw new Error(
        `Language "${languageId}" is not loaded. ` +
          `Load the plaintext bundle before using omitted/plaintext sync highlighting.`,
      );
    }
    throw new Error(
      `Language "${languageId}" is not loaded. ` +
        `Pass it to createHighlighter({ languages: [...] }) or call hl.loadLanguage(bundle).`,
    );
  }

  return loaded;
}

function runHighlightIter(
  runtime: RuntimeLike,
  source: string,
  language: LanguageRef | undefined,
  theme: Theme | undefined,
  onToken: HighlightCallback,
  options: HighlightOptions = {},
): void {
  const loaded = resolveLoadedLanguage(runtime, language);
  const events = runtime.highlightEvents(source, loaded, {
    rainbowBrackets: options.rainbowBrackets,
  });
  const bytes = buildSourceIndex(source).sourceBytes;
  const scopeStack: Array<{ scope: string; language: string }> = [];

  for (const event of events) {
    if (event.type === "start") {
      scopeStack.push({ scope: event.scope, language: event.language });
      continue;
    }
    if (event.type === "end") {
      scopeStack.pop();
      continue;
    }

    emitToken(event, scopeStack, bytes, theme, loaded.definition.id, onToken);
  }
}

// The innermost open span decides a token's scope and language; a token outside
// any span carries the document's language and no scope.
function emitToken(
  event: { startByte: number; endByte: number },
  scopeStack: Array<{ scope: string; language: string }>,
  bytes: Uint8Array,
  theme: Theme | undefined,
  documentLanguage: string,
  onToken: HighlightCallback,
): void {
  const active = scopeStack.at(-1);
  const scope = active?.scope ?? "";
  const tokenLanguage = active?.language ?? documentLanguage;

  onToken(
    decodeSlice(bytes, event.startByte, event.endByte),
    tokenLanguage,
    { start: event.startByte, end: event.endByte },
    scope,
    scope.length > 0 ? getScopedThemeStyle(theme, scope, tokenLanguage) : undefined,
  );
}

function runHighlightEvents<T>(
  runtime: RuntimeLike,
  source: string,
  language: LanguageRef | undefined,
  options: HighlightOptions<T> = {},
): HighlightEvent<T>[] {
  const loaded = resolveLoadedLanguage(runtime, language);
  const events = runtime.highlightEvents(source, loaded, {
    rainbowBrackets: options.rainbowBrackets,
  });
  const annotations = options.annotations ?? [];
  if (annotations.length === 0) return events;

  return composeAnnotations(events, annotations, buildSourceIndex(source));
}

// Ambient runtime for sync free functions called inside `Formatter.render()`.
// JS is single-threaded, so swapping a module-level reference around the call
// gives the same guarantee Rust gets from `thread_local!` in `highlight.rs`.
let currentRuntime: RuntimeLike | undefined;

function requireCurrentRuntime(fnName: string): RuntimeLike {
  if (!currentRuntime) {
    throw new Error(
      `${fnName}() must be called inside Formatter.render(). ` +
        `For top-level token iteration, create a highlighter with ` +
        `createHighlighter({ languages: [...] }) and call hl.highlightIter().`,
    );
  }
  return currentRuntime;
}

/**
 * Iterate over highlighted tokens for `source`, calling `onToken` for each flat span.
 *
 * Sync free function usable inside {@link Formatter.render}. For top-level
 * (non-formatter) iteration, use `hl.highlightIter` on a {@link Highlighter}
 * instance instead.
 *
 * `options.rainbowBrackets` reports `punctuation.bracket.rainbow.N` scopes,
 * the same ones the built-in formatters render.
 */
export function highlightIter(
  source: string,
  language: LanguageRef | undefined,
  theme: Theme | undefined,
  onToken: HighlightCallback,
  options: HighlightOptions = {},
): void {
  const runtime = requireCurrentRuntime("highlightIter");
  runHighlightIter(runtime, source, detectLanguageRef(source, language), theme, onToken, options);
}

/**
 * Return the nested highlight event stream for `source`.
 *
 * Sync free function usable inside {@link Formatter.render}. Use this when your
 * formatter needs paired open/close markers around nested scopes (e.g. BBCode
 * tags) that the flat {@link highlightIter} callback API would lose.
 */
export function highlightEvents(
  source: string,
  language: LanguageRef | undefined,
  options?: { rainbowBrackets?: boolean },
): SyntaxHighlightEvent[];
export function highlightEvents<T>(
  source: string,
  language: LanguageRef | undefined,
  options: HighlightOptions<T>,
): HighlightEvent<T>[];
export function highlightEvents<T>(
  source: string,
  language: LanguageRef | undefined,
  options: HighlightOptions<T> = {},
): HighlightEvent<T>[] {
  const runtime = requireCurrentRuntime("highlightEvents");
  return runHighlightEvents(runtime, source, detectLanguageRef(source, language), options);
}

function runFormatter<T>(
  runtime: RuntimeLike,
  source: string,
  fmt: Formatter<T>,
  detectedRef: LanguageRef | string,
  options: HighlightOptions<T>,
): string {
  const loaded = resolveLoadedLanguage(runtime, detectedRef);
  if (builtinFormatterKind(fmt)) {
    const nativeOutput = runtime.format?.(source, loaded, fmt, options);
    if (nativeOutput !== undefined) return nativeOutput;
  }

  const events = runHighlightEvents(runtime, source, detectedRef, options);
  const prevRuntime = currentRuntime;
  const prevLanguage = fmt.language;
  currentRuntime = runtime;
  fmt.language = detectedRef;
  try {
    return fmt.render(source, events);
  } finally {
    fmt.language = prevLanguage;
    currentRuntime = prevRuntime;
  }
}

async function runFormatterAsync<T>(
  runtime: RuntimeLike,
  source: string,
  fmt: Formatter<T>,
  detectedRef: LanguageRef | string,
  options: HighlightOptions<T>,
): Promise<string> {
  const loaded = resolveLoadedLanguage(runtime, detectedRef);
  if (builtinFormatterKind(fmt)) {
    const nativeOutput = await runtime.formatAsync?.(source, loaded, fmt, options);
    if (nativeOutput !== undefined) return nativeOutput;
  }
  return runFormatter(runtime, source, fmt, detectedRef, options);
}

function isObjectLike(value: unknown): value is object {
  return (typeof value === "object" && value !== null) || typeof value === "function";
}

function isLanguageLike(value: unknown): boolean {
  return isObjectLike(value) && LANGUAGE_SHAPE_FIELDS.some((field) => field in value);
}

function isLanguageDefinition(value: unknown): value is LanguageDefinition {
  if (!isObjectLike(value) || !("id" in value) || !("aliases" in value)) return false;
  return (
    typeof value.id === "string" &&
    value.id.length > 0 &&
    Array.isArray(value.aliases) &&
    value.aliases.every((alias) => typeof alias === "string")
  );
}

function hasLanguageLoadFields(value: object): boolean {
  return LANGUAGE_LOAD_FIELDS.some((field) => field in value);
}

function hasLoadableLanguageShape(candidate: Language): candidate is LoadableLanguage {
  if (candidate.id === PLAINTEXT_LANG_ID) return !hasLanguageLoadFields(candidate);
  return (
    typeof candidate.packageName === "string" &&
    candidate.packageName.length > 0 &&
    !LANGUAGE_QUERY_FIELDS.some((field) => field in candidate)
  );
}

function malformedLanguageDefinition(): Error {
  return new Error('Language definition has an invalid or missing "id" or "aliases".');
}

function incompleteLanguageDefinition(id: string): Error {
  return new Error(`Language "${id}" has an incomplete or conflicting load definition.`);
}

function validateLanguageBoundary(value: unknown): void {
  if (!isLanguageLike(value)) return;
  if (!isLanguageDefinition(value)) throw malformedLanguageDefinition();

  for (const field of LANGUAGE_QUERY_FIELDS) {
    const query: unknown = Reflect.get(value, field);
    if (field in value && typeof query !== "string") {
      throw incompleteLanguageDefinition(value.id);
    }
  }

  if (hasLanguageLoadFields(value) && !hasLoadableLanguageShape(value as Language)) {
    throw incompleteLanguageDefinition(value.id);
  }
}

/** Check if a value is a package handle or plaintext. */
function isLanguage(value: unknown): value is LoadableLanguage {
  validateLanguageBoundary(value);
  return (
    typeof value === "object" &&
    value !== null &&
    isLanguageDefinition(value) &&
    hasLoadableLanguageShape(value as Language)
  );
}

function requireLoadableLanguage(value: unknown, source: string): Language {
  validateLanguageBoundary(value);
  if (isLanguage(value)) return value;
  if (isLanguageDefinition(value)) throw incompleteLanguageDefinition(value.id);
  throw new Error(`${source} did not return a complete language definition.`);
}

function isLazyLanguage(value: unknown): value is LazyLanguage {
  if (typeof value !== "function") return false;
  validateLanguageBoundary(value);
  return isLanguageDefinition(value);
}

/** Check if a value is a LanguageBundle (Record<string, LazyLanguage>). */
function isLanguageBundle(value: unknown): value is LanguageBundle {
  if (typeof value !== "object" || value === null || Array.isArray(value)) return false;
  if (LANGUAGE_SHAPE_FIELDS.some((field) => field in value) || "default" in value) {
    return false;
  }
  const entries = Object.entries(value);
  return entries.length > 0 && entries.every(([id, lazy]) => id.length > 0 && isLazyLanguage(lazy));
}

/** Resolve a single LanguageInput into Language(s), registering lazy ones. */
async function resolveInitialLanguage(
  input: LanguageInput,
  lazyRegistry: Map<string, LazyLanguage>,
): Promise<Language | undefined> {
  validateLanguageBoundary(input);

  // Language object — load eagerly
  if (isLanguage(input)) {
    return input;
  }

  if (isLazyLanguage(input)) {
    return requireLoadableLanguage(await input(), `Lazy language "${input.id}"`);
  }

  if (isLanguageDefinition(input)) {
    return resolveDefinition(input, lazyRegistry);
  }

  // LanguageBundle (Record<string, LazyLanguage>) — register all lazily
  if (isLanguageBundle(input)) {
    registerBundleLazily(input, lazyRegistry);
    return undefined;
  }

  // () => Promise<{ default: Language }> — lazy function
  if (typeof input === "function") {
    const mod = await input();
    return requireLoadableLanguage(mod.default, "Lazy language import");
  }

  // Promise<{ default: Language }> — eager dynamic import
  const mod = await input;
  return requireLoadableLanguage(mod.default, "Language import");
}

// A definition names a language the caller expects some bundle to provide.
async function resolveDefinition(
  input: { id: string },
  lazyRegistry: Map<string, LazyLanguage>,
): Promise<Language> {
  const lazy = lazyRegistry.get(normalizeLanguageName(input.id));
  if (lazy) {
    return requireLoadableLanguage(await lazy(), `Lazy language "${lazy.id}"`);
  }

  const builtin = await loadBuiltinLanguageById(input.id);
  if (builtin) return builtin;

  throw new Error(`Language "${input.id}" is not registered in any bundle.`);
}

// The first bundle to claim an id or alias keeps it.
//
// `registerLazyBundle` is this plus a `validateLanguageBoundary` on every entry.
// The caller here has already validated the bundle it was handed, and validating
// each entry again would reject bundles this path accepts today.
function registerBundleLazily(
  input: LanguageBundle,
  lazyRegistry: Map<string, LazyLanguage>,
): void {
  for (const [id, lazy] of Object.entries(input)) {
    const key = normalizeLanguageName(id);
    if (lazyRegistry.has(key)) continue;

    lazyRegistry.set(key, lazy);
    for (const alias of lazy.aliases) {
      lazyRegistry.set(normalizeLanguageName(alias), lazy);
    }
  }
}

function registerLazyBundle(bundle: LanguageBundle, lazyRegistry: Map<string, LazyLanguage>): void {
  for (const [id, lazy] of Object.entries(bundle)) {
    validateLanguageBoundary(lazy);
    const key = normalizeLanguageName(id);
    if (lazyRegistry.has(key)) {
      continue;
    }

    lazyRegistry.set(key, lazy);
    for (const alias of lazy.aliases) {
      lazyRegistry.set(normalizeLanguageName(alias), lazy);
    }
  }
}

async function loadHighlighterLanguage(
  input: Language | LazyLanguage | string,
  runtime: RuntimeLike,
  lazyRegistry: Map<string, LazyLanguage>,
): Promise<void> {
  validateLanguageBoundary(input);

  const id = typeof input === "string" ? input : input.id;
  if (runtime.getLoadedLanguage(id)) {
    return;
  }

  if (isLanguage(input)) {
    await loadLanguageDefinition(runtime, input);
    return;
  }

  if (typeof input === "string") {
    const lazy = lazyRegistry.get(normalizeLanguageName(input));
    if (lazy) {
      const language = requireLoadableLanguage(await lazy(), `Lazy language "${lazy.id}"`);
      await loadLanguageDefinition(runtime, language);
      return;
    }

    const builtin = await loadBuiltinLanguageById(input);
    if (!builtin) {
      throw new Error(`Language "${input}" is not registered in any bundle.`);
    }

    await loadLanguageDefinition(runtime, builtin);
    return;
  }

  if (isLazyLanguage(input)) {
    const language = requireLoadableLanguage(await input(), `Lazy language "${input.id}"`);
    await loadLanguageDefinition(runtime, language);
    return;
  }

  await loadHighlighterLanguage(input.id, runtime, lazyRegistry);
}

function getRegisteredLanguageIds(
  runtime: RuntimeLike,
  lazyRegistry: Map<string, LazyLanguage>,
): string[] {
  return [...new Set([...runtime.getLoadedLanguageIds(), ...lazyRegistry.keys()])];
}

async function loadInitialLanguages(
  inputs: LanguageInput[],
  runtime: RuntimeLike,
  lazyRegistry: Map<string, LazyLanguage>,
): Promise<void> {
  const eagerDefinitions: Array<Promise<Language | undefined>> = [];

  for (const input of inputs) {
    if (isLanguageBundle(input)) {
      registerLazyBundle(input, lazyRegistry);
      continue;
    }

    eagerDefinitions.push(resolveInitialLanguage(input, lazyRegistry));
  }

  const languages = (await Promise.all(eagerDefinitions)).filter(
    (language): language is Language => language !== undefined,
  );
  for (const language of languages) {
    runtime.registerLanguage({ id: language.id, aliases: language.aliases });
  }

  await Promise.all([
    runtime.loadPlaintext(),
    ...languages.map((language) => loadLanguageDefinition(runtime, language)),
  ]);
}

async function prepareRuntimeHighlight(
  runtime: RuntimeLike,
  source: string,
  language: LanguageRef | undefined,
): Promise<LanguageRef | string> {
  await runtime.initParser();

  const detectedRef = detectLanguageRef(source, language);
  await ensureLanguageLoaded(runtime, detectedRef);
  return detectedRef;
}

export function createHighlighterModule(factory: HighlighterModuleFactory) {
  return {
    async createHighlighter(init: CreateHighlighterOptions = {}): Promise<Highlighter> {
      const runtime = factory.createRuntime({
        wasmResolver: init.wasmResolver,
        languagePackageResolver: init.languagePackageResolver,
      });
      await runtime.initParser();

      const lazyRegistry = new Map<string, LazyLanguage>();
      await loadInitialLanguages(init.languages ?? [], runtime, lazyRegistry);

      return {
        highlight: (source, fmt, options = {}) => {
          const detectedRef = detectLanguageRef(source, fmt.language);
          return runFormatter(runtime, source, fmt, detectedRef, options);
        },
        highlightIter: (source, language, theme, onToken, options) => {
          runHighlightIter(
            runtime,
            source,
            detectLanguageRef(source, language),
            theme,
            onToken,
            options,
          );
        },
        async loadLanguage(input) {
          await loadHighlighterLanguage(input, runtime, lazyRegistry);
        },
        get languages() {
          return runtime.getLoadedLanguageIds();
        },
        get registeredLanguages() {
          return getRegisteredLanguageIds(runtime, lazyRegistry);
        },
      };
    },

    async highlight<T>(
      source: string,
      fmt: Formatter<T>,
      options: HighlightOptions<T> = {},
    ): Promise<string> {
      const runtime = factory.getDefaultRuntime();
      const detectedRef = await prepareRuntimeHighlight(runtime, source, fmt.language);

      return runFormatterAsync(runtime, source, fmt, detectedRef, options);
    },
  };
}

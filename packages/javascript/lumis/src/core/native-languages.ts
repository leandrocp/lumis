import { LANGUAGES } from "../generated/languages-meta.js";
import { cloneLanguageInfo } from "../catalog-metadata.js";
import { LANGUAGE_LOADERS } from "../generated/language-loaders.js";
import { EXACT_LANGUAGE_MAP } from "../generated/language-detection.js";
import { LANGUAGE_PACKAGES, LANGUAGE_PACKAGE_NAMES } from "../generated/language-packages.js";
import { LANGUAGE_PACKAGE_VERSION_RANGE } from "../generated/package-version-range.js";
import type {
  NativeBinding,
  NativeFormatter,
  NativeHtmlAttrs,
  NativeRuntimeInstance,
} from "../native-binding.js";
import type {
  Budget,
  BudgetExhausted,
  Formatter,
  HighlightOptions,
  HtmlAttrs,
  HtmlInlineFormatter,
  HtmlLinkedFormatter,
  HtmlStructure,
  LanguageDefinition,
  LanguageInfo,
  LoadedLanguage,
  NativeLoadedLanguage,
  PlaintextLoadedLanguage,
  LumisHighlightEvent,
  WasmRef,
} from "../types.js";
import { getBuiltinFormatter } from "./builtin-formatter.js";
import { isInlineStructure } from "../formatter/html-structure.js";
import { assertBudget, warnUnresolvedInjection } from "../events.js";
import { decodeNativeEvents } from "./native-event-codec.js";
import { PLAINTEXT_LANG_ID } from "../types.js";
import { DEFAULT_RESOLVER, normalizeLanguageName, notDeclared, verifyWasm } from "./languages.js";
import type {
  HighlighterRuntimeOptions,
  LanguagePackageResolver,
  LanguagesModule,
  LoadLanguageOptions,
  ResolvedLanguagePackage,
  RuntimeLike,
  WasmResolver,
} from "./languages.js";

const PLAINTEXT_ALIASES = LANGUAGES.find(({ id }) => id === PLAINTEXT_LANG_ID)?.aliases ?? [];
const CATALOG_LANGUAGE_IDS = new Set(LANGUAGES.map(({ id }) => normalizeLanguageName(id)));
const encoder = new TextEncoder();

/**
 * Rust takes the same three states JavaScript writes: a string, `true` for the
 * bare boolean form, and `false` to drop an attribute Lumis generated. `null`
 * and `undefined` mean the caller left the field off, which is `false`.
 */
function nativeHtmlAttrs(attrs: HtmlAttrs | undefined): NativeHtmlAttrs {
  if (!attrs) return [];

  return Object.entries(attrs).map(([name, value]) =>
    typeof value === "boolean" || value == null ? [name, value === true] : [name, String(value)],
  );
}

function nativeHtmlStructure(structure: HtmlStructure | undefined): HtmlStructure {
  return isInlineStructure(structure) ? "inline" : "block";
}

function nativeHtmlInlineFormatter(
  formatter: HtmlInlineFormatter,
  rainbowBrackets: HighlightOptions["rainbowBrackets"],
): NativeFormatter {
  return {
    rainbowBrackets,
    kind: "html-inline",
    options: {
      structure: nativeHtmlStructure(formatter.structure),
      theme: formatter.theme,
      preClass: formatter.preClass,
      preAttrs: nativeHtmlAttrs(formatter.preAttrs),
      codeAttrs: nativeHtmlAttrs(formatter.codeAttrs),
      italic: formatter.italic,
      includeHighlights: formatter.includeHighlights,
      highlightLines: formatter.highlightLines,
      lineNumbers: formatter.lineNumbers,
      header: formatter.header,
    },
  };
}

function nativeHtmlLinkedFormatter(
  formatter: HtmlLinkedFormatter,
  rainbowBrackets: HighlightOptions["rainbowBrackets"],
): NativeFormatter {
  return {
    rainbowBrackets,
    kind: "html-linked",
    options: {
      structure: nativeHtmlStructure(formatter.structure),
      preClass: formatter.preClass,
      preAttrs: nativeHtmlAttrs(formatter.preAttrs),
      codeAttrs: nativeHtmlAttrs(formatter.codeAttrs),
      highlightLines: formatter.highlightLines,
      lineNumbers: formatter.lineNumbers,
      header: formatter.header,
    },
  };
}

function hasStringProperties(value: object, names: readonly string[]): boolean {
  return names.every((name) => typeof Reflect.get(value, name) === "string");
}

function isWasmRef(wasm: unknown): wasm is WasmRef {
  if (typeof wasm !== "object" || wasm === null) return false;
  if (!hasStringProperties(wasm, ["packageName", "name", "version", "sha256"])) return false;

  const sha256: unknown = Reflect.get(wasm, "sha256");
  return typeof sha256 === "string" && /^[0-9a-f]{64}$/u.test(sha256);
}

function parseWasmRef(json: string): WasmRef {
  const value: unknown = JSON.parse(json);
  if (!isWasmRef(value)) throw new Error("native runtime returned invalid parser metadata");
  return value;
}

type RuntimeWasmInput = Exclude<NonNullable<LoadLanguageOptions["wasm"]>, WasmRef>;

interface NativeResolverState {
  wasmResolver?: WasmResolver;
  languagePackageResolver?: LanguagePackageResolver;
}

interface AddonLanguage {
  addonId: string;
  definition: LanguageDefinition;
}

/**
 * Parser bytes from whatever the caller handed over: buffer, path, URL or
 * Response. Only what this downloads is checked against `packaged`.
 */
async function readWasmInput(wasm: RuntimeWasmInput, packaged: WasmRef): Promise<Uint8Array> {
  if (wasm instanceof Uint8Array) return wasm;
  if (wasm instanceof ArrayBuffer) return new Uint8Array(wasm);
  if (wasm instanceof Response) return new Uint8Array(await wasm.arrayBuffer());

  const { readFile } = await import("node:fs/promises");
  const { fileURLToPath } = await import("node:url");
  const download = async (url: string | URL) =>
    verifyWasm(packaged, new Uint8Array(await (await fetch(url)).arrayBuffer()));

  if (wasm instanceof URL) {
    if (wasm.protocol !== "file:") return download(wasm);
    return new Uint8Array(await readFile(fileURLToPath(wasm)));
  }

  if (wasm.startsWith("file://")) {
    return new Uint8Array(await readFile(fileURLToPath(new URL(wasm))));
  }
  if (/^https?:\/\//u.test(wasm)) return download(wasm);
  return new Uint8Array(await readFile(wasm));
}

/**
 * Node highlighting over the Wasmtime addon.
 *
 * Rust loads parsers from the packages this project installed, which is what
 * lets an injected language load during the walk that finds it. A caller can
 * still take that over — `configureWasmResolver`, `configureLanguagePackageResolver`,
 * an explicit `wasm`, or a complete custom `Language` — and those all mean the
 * same thing here as under `web-tree-sitter`. When one is in play the JavaScript
 * pipeline in `createLanguagesModule` resolves roots before handing the
 * bytes to the addon. During a native walk, the addon reads and verifies the
 * source locations returned by the same callbacks synchronously.
 *
 * `resolvers` is that pipeline: the same module the `web-tree-sitter` runtime
 * uses, so there is one implementation of resolve, verify and cache rather than
 * a native copy that can drift.
 */
const resolverSource = (source: string | URL): string =>
  source instanceof URL ? source.href : source;

/**
 * Whether a language came from a package this project also installed. It is
 * that package, so it stays on the installed path, where the shared runtime
 * keeps it and a native walk loads what it injects.
 */
function isImportedInstalled(
  opts: LoadLanguageOptions,
  manifests: Record<string, string> | undefined,
): boolean {
  return (
    opts.languagePackage !== undefined &&
    opts.packageName !== undefined &&
    manifests?.[opts.packageName] !== undefined
  );
}

export function createNativeLanguagesModule(
  binding: NativeBinding,
  resolvers: LanguagesModule,
  resolveInstalledManifests?: (packageNames: readonly string[]) => Promise<Map<string, URL>>,
): LanguagesModule {
  let globalWasmResolver: WasmResolver | undefined;
  let globalLanguagePackageResolver: LanguagePackageResolver | undefined;
  let resolverCallbackDepth = 0;

  /**
   * Hand the addon the data directory the environment names before it builds
   * its first runtime, which fixes where compiled parsers are kept.
   * `LUMIS_DATA_DIR` can be set at any point in a Node process, and a caller
   * that sets it late would otherwise silently keep writing to the platform
   * data directory.
   */
  function newNativeRuntime(): NativeRuntimeInstance {
    binding.configureStore(process.env.LUMIS_DATA_DIR);
    return new binding.NativeRuntime();
  }

  let installed: Promise<Record<string, string> | undefined> | undefined;

  /**
   * Hand the addon the `lumis.json` of every package this project installed,
   * once, and keep them to check a load against.
   *
   * Those packages are the whole set Node loads from. The addon reads them
   * itself, because it loads a language injected inside a document during a
   * native walk, without coming back to JavaScript.
   */
  function tellAddon(): Promise<Record<string, string> | undefined> {
    if (installed) return installed;
    if (resolveInstalledManifests) {
      installed = (async () => {
        const { fileURLToPath } = await import("node:url");
        const manifests: Record<string, string> = {};
        for (const [packageName, manifest] of await resolveInstalledManifests(
          LANGUAGE_PACKAGE_NAMES,
        )) {
          manifests[packageName] = fileURLToPath(manifest);
        }
        binding.setInstalledPackages(manifests);
        return manifests;
      })();
    } else {
      installed = Promise.resolve(undefined);
    }
    return installed;
  }

  function hasResolverOverride(state: NativeResolverState): boolean {
    return Boolean(
      state.wasmResolver ??
      state.languagePackageResolver ??
      globalWasmResolver ??
      globalLanguagePackageResolver,
    );
  }

  function resolvedPackageSource(
    state: NativeResolverState,
    packageName: string,
  ): string | undefined {
    // Without a package resolver of its own, the caller has not said where
    // packages come from, so the addon loads the installed one.
    const resolver = state.languagePackageResolver ?? globalLanguagePackageResolver;
    if (!resolver) return undefined;
    resolverCallbackDepth += 1;
    try {
      return resolverSource(resolver(packageName, LANGUAGE_PACKAGE_VERSION_RANGE));
    } finally {
      resolverCallbackDepth -= 1;
    }
  }

  function resolvedWasmSource(
    state: NativeResolverState,
    language: string,
    wasmJson: string,
  ): string | undefined {
    if (!hasResolverOverride(state)) return undefined;
    const resolver = state.wasmResolver ?? globalWasmResolver ?? DEFAULT_RESOLVER;
    resolverCallbackDepth += 1;
    try {
      return resolverSource(resolver(language, parseWasmRef(wasmJson)));
    } finally {
      resolverCallbackDepth -= 1;
    }
  }

  function rejectReentrantHighlight(): void {
    if (resolverCallbackDepth > 0) {
      throw new Error("native highlighting cannot be called from a language resolver callback");
    }
  }

  class NativeHighlighterRuntime implements RuntimeLike {
    private readonly native: NativeRuntimeInstance;
    private readonly loadedLanguages = new Map<string, LoadedLanguage>();
    private readonly languageLoads = new Map<string, Promise<LoadedLanguage>>();
    private readonly aliasMap = new Map<string, string>();
    private readonly ownDefinitions = new Map<string, string>();
    private readonly resolver: RuntimeLike;
    private readonly resolverState: NativeResolverState;
    private readonly packageResolverCallback: (packageName: string) => string | undefined;
    private readonly wasmResolverCallback: (
      language: string,
      wasmJson: string,
    ) => string | undefined;

    constructor(options: HighlighterRuntimeOptions = {}) {
      for (const alias of PLAINTEXT_ALIASES) {
        this.aliasMap.set(normalizeLanguageName(alias), PLAINTEXT_LANG_ID);
      }
      this.resolver = resolvers.createRuntime(options);
      this.resolverState = {
        wasmResolver: options.wasmResolver,
        languagePackageResolver: options.languagePackageResolver,
      };
      const state = this.resolverState;
      this.packageResolverCallback = (packageName) => resolvedPackageSource(state, packageName);
      this.wasmResolverCallback = (language, wasmJson) =>
        resolvedWasmSource(state, language, wasmJson);
      this.native = newNativeRuntime();
    }

    private reportUnresolved(unresolved: string[]): void {
      for (const id of unresolved) {
        const languageId = EXACT_LANGUAGE_MAP[normalizeLanguageName(id)];
        const packageName = languageId && LANGUAGE_PACKAGES[languageId];
        if (packageName) {
          warnUnresolvedInjection(id, `${notDeclared(packageName, packageName).message}.`);
        }
      }
    }

    configureWasmResolver(fn: WasmResolver): void {
      this.resolverState.wasmResolver = fn;
      this.resolver.configureWasmResolver(fn);
    }

    configureLanguagePackageResolver(fn: LanguagePackageResolver): void {
      this.resolverState.languagePackageResolver = fn;
      this.resolver.configureLanguagePackageResolver(fn);
    }

    resolveLanguagePackage(
      language: LanguageDefinition,
      packageName: string,
      languagePackage?: object,
    ): Promise<ResolvedLanguagePackage> {
      return this.resolver.resolveLanguagePackage(language, packageName, languagePackage);
    }

    resolveParserWasm(language: string, wasm: WasmRef): Promise<Uint8Array> {
      return this.resolver.resolveParserWasm(language, wasm);
    }

    async initParser(): Promise<void> {
      // The addon resolves and compiles parsers on demand; nothing to set up.
    }

    registerLanguage(definition: LanguageDefinition): void {
      for (const alias of definition.aliases) {
        this.aliasMap.set(normalizeLanguageName(alias), normalizeLanguageName(definition.id));
      }
    }

    resolveLanguageId(nameOrAlias: string): string {
      const normalized = normalizeLanguageName(nameOrAlias);
      return this.aliasMap.get(normalized) ?? normalized;
    }

    getLoadedLanguage(nameOrAlias: string): LoadedLanguage | undefined {
      return this.loadedLanguages.get(this.resolveLanguageId(nameOrAlias));
    }

    getLoadedLanguageIds(): string[] {
      return [...this.loadedLanguages.values()].map(({ definition }) => definition.id);
    }

    private async createLoadedLanguage(opts: LoadLanguageOptions): Promise<LoadedLanguage> {
      const { addonId, definition } = await this.loadThroughAddon(opts);
      if (addonId !== definition.id) {
        this.ownDefinitions.set(normalizeLanguageName(opts.definition.id), addonId);
        this.ownDefinitions.set(normalizeLanguageName(definition.id), addonId);
      }
      if (normalizeLanguageName(opts.definition.id) !== normalizeLanguageName(definition.id)) {
        this.aliasMap.set(
          normalizeLanguageName(opts.definition.id),
          normalizeLanguageName(definition.id),
        );
      }
      const loaded: NativeLoadedLanguage = { kind: "native", definition };
      this.loadedLanguages.set(normalizeLanguageName(definition.id), loaded);
      this.registerLanguage(definition);
      return loaded;
    }

    /**
     * Whether the caller has taken over resolution for this load.
     *
     * Rust cannot see a JavaScript resolver or a `Uint8Array` of parser bytes,
     * so anything carrying one has to be resolved here instead. Everything else
     * stays on the Rust path, where a language injected inside the document
     * still loads mid-walk.
     */
    private async isCallerResolved(
      opts: LoadLanguageOptions,
      manifests: Record<string, string> | undefined,
    ): Promise<boolean> {
      const importedInstalled = isImportedInstalled(opts, manifests);
      if (
        hasResolverOverride(this.resolverState) ||
        (opts.wasm !== undefined && !importedInstalled) ||
        !CATALOG_LANGUAGE_IDS.has(normalizeLanguageName(opts.definition.id))
      ) {
        return true;
      }

      const catalog = await LANGUAGE_LOADERS[normalizeLanguageName(opts.definition.id)]?.();
      return opts.packageName !== catalog?.default.packageName;
    }

    private async loadThroughAddon(opts: LoadLanguageOptions): Promise<AddonLanguage> {
      // Before anything else: the walk that follows loads injected languages
      // from these in Rust, whichever way the root was loaded.
      const manifests = await tellAddon();

      if (!(await this.isCallerResolved(opts, manifests))) {
        const packageName = opts.packageName!;
        if (manifests && !manifests[packageName]) throw notDeclared(packageName, packageName);
        this.native.loadLanguage(opts.definition.id);
        return { addonId: opts.definition.id, definition: opts.definition };
      }

      // Queries always come from the package, so a parser can only ever run
      // against the queries it was released and tested with.
      if (!opts.packageName) {
        throw new Error(`Language "${opts.definition.id}" has no packageName`);
      }

      const packaged = await this.resolveLanguagePackage(
        opts.definition,
        opts.packageName,
        opts.languagePackage,
      );
      const resolved = { ...packaged, ...(opts.wasm === undefined ? {} : { wasm: opts.wasm }) };
      if (!resolved.wasm || resolved.highlights === undefined) {
        throw new Error(`Language package "${opts.packageName}" has no parser or highlights query`);
      }

      const wasm = isWasmRef(resolved.wasm)
        ? await this.resolveParserWasm(resolved.definition.id, resolved.wasm)
        : await readWasmInput(resolved.wasm, packaged.wasm);

      return {
        addonId: this.native.loadLanguageDefinition(
          {
            id: resolved.definition.id,
            aliases: resolved.definition.aliases,
            grammarName: packaged.grammarName,
            highlights: resolved.highlights,
            injections: resolved.injections,
            locals: resolved.locals,
            brackets: resolved.brackets,
          },
          wasm,
        ),
        definition: resolved.definition,
      };
    }

    async loadLanguage(opts: LoadLanguageOptions): Promise<LoadedLanguage> {
      if (opts.definition.id === PLAINTEXT_LANG_ID) {
        return this.loadPlaintext();
      }
      const existing = this.getLoadedLanguage(opts.definition.id);
      if (existing) return existing;
      const loadKey = normalizeLanguageName(opts.definition.id);
      const inFlight = this.languageLoads.get(loadKey);
      if (inFlight) return inFlight;

      const load = this.createLoadedLanguage(opts);
      this.languageLoads.set(loadKey, load);
      try {
        return await load;
      } finally {
        if (this.languageLoads.get(loadKey) === load) {
          this.languageLoads.delete(loadKey);
        }
      }
    }

    async loadPlaintext(): Promise<LoadedLanguage> {
      return this.createPlaintext({ id: PLAINTEXT_LANG_ID, aliases: PLAINTEXT_ALIASES });
    }

    private createPlaintext(definition: LanguageDefinition): LoadedLanguage {
      const existing = this.getLoadedLanguage(PLAINTEXT_LANG_ID);
      if (existing) return existing;

      const loaded: PlaintextLoadedLanguage = { kind: "plaintext", definition };
      this.loadedLanguages.set(PLAINTEXT_LANG_ID, loaded);
      this.registerLanguage(definition);
      return loaded;
    }

    /**
     * One pass over the document. Languages injected inside it are downloaded
     * and loaded during the walk that finds them, so nothing has to be declared
     * up front.
     *
     * The addon calls a configured JavaScript resolver for each injected
     * language, then performs the synchronous fetch and verification in Rust
     * before the walk descends into the newly loaded parser.
     */
    highlightEvents(
      source: string,
      language: LoadedLanguage,
      options: { rainbowBrackets?: boolean; budget?: Budget } = {},
      report?: { budget?: BudgetExhausted },
    ): LumisHighlightEvent[] {
      rejectReentrantHighlight();
      assertBudget(options.budget);
      if (language.definition.id === PLAINTEXT_LANG_ID) {
        return [{ type: "source", start: 0, end: encoder.encode(source).byteLength }];
      }
      const hasResolvers = hasResolverOverride(this.resolverState);
      const highlighted = this.native.highlightEvents(
        source,
        this.addonIdFor(language),
        options.rainbowBrackets ?? false,
        options.budget,
        hasResolvers ? this.packageResolverCallback : undefined,
        hasResolvers ? this.wasmResolverCallback : undefined,
      );
      this.reportUnresolved(highlighted.unresolved);
      if (report) report.budget = highlighted.budget;
      return decodeNativeEvents(highlighted.events);
    }

    private addonIdFor(language: LoadedLanguage): string {
      return (
        this.ownDefinitions.get(normalizeLanguageName(language.definition.id)) ??
        language.definition.id
      );
    }

    format(
      source: string,
      language: LoadedLanguage,
      formatter: Formatter,
      options: HighlightOptions = {},
    ): string | undefined {
      rejectReentrantHighlight();
      const nativeFormatter = this.nativeFormatter(language, formatter, options, true);
      if (!nativeFormatter) return undefined;

      const hasResolvers = hasResolverOverride(this.resolverState);
      const formatted = this.native.format(
        source,
        this.addonIdFor(language),
        nativeFormatter,
        hasResolvers ? this.packageResolverCallback : undefined,
        hasResolvers ? this.wasmResolverCallback : undefined,
      );
      this.reportUnresolved(formatted.unresolved);
      return formatted.output;
    }

    async formatAsync(
      source: string,
      language: LoadedLanguage,
      formatter: Formatter,
      options: HighlightOptions = {},
    ): Promise<string | undefined> {
      rejectReentrantHighlight();
      const nativeFormatter = this.nativeFormatter(language, formatter, options, false);
      if (!nativeFormatter) return undefined;

      const formatted = await this.native.formatAsync(
        source,
        this.addonIdFor(language),
        nativeFormatter,
      );
      this.reportUnresolved(formatted.unresolved);
      return formatted.output;
    }

    private nativeFormatter(
      language: LoadedLanguage,
      formatter: Formatter,
      highlightOptions: HighlightOptions,
      canCallResolver: boolean,
    ): NativeFormatter | undefined {
      const builtin = getBuiltinFormatter(formatter);
      // Rust names the `language-*` class from `lumis_core::Language`, which has
      // no variant for a language the caller defined. Highlighting still runs
      // natively; only the string assembly falls back to JavaScript, exactly as
      // it already does for `html-multi-themes`. A Node worker cannot call the
      // JavaScript resolver needed by a language first discovered mid-walk, so
      // the async path also stays on the main thread when one is configured.
      if (!builtin || builtin.kind === "html-multi-themes") return undefined;
      if (!this.canFormatNatively(language, canCallResolver)) return undefined;

      const rainbowBrackets = highlightOptions.rainbowBrackets;
      const budget = highlightOptions.budget;
      assertBudget(budget);

      switch (builtin.kind) {
        case "html-inline":
          return { ...nativeHtmlInlineFormatter(builtin.formatter, rainbowBrackets), budget };
        case "html-linked":
          return { ...nativeHtmlLinkedFormatter(builtin.formatter, rainbowBrackets), budget };
        case "bbcode-scoped":
          return {
            rainbowBrackets,
            budget,
            kind: builtin.kind,
            options: { highlightLines: builtin.formatter.highlightLines },
          };
        case "terminal":
          return {
            rainbowBrackets,
            budget,
            kind: builtin.kind,
            options: {
              theme: builtin.formatter.theme,
              background: builtin.formatter.background,
              width: builtin.formatter.width,
              highlightLines: builtin.formatter.highlightLines,
              lineNumbers: builtin.formatter.lineNumbers,
            },
          };
      }
      return undefined;
    }

    private canFormatNatively(language: LoadedLanguage, canCallResolver: boolean): boolean {
      return (
        (canCallResolver || !hasResolverOverride(this.resolverState)) &&
        language.definition.id !== PLAINTEXT_LANG_ID &&
        CATALOG_LANGUAGE_IDS.has(normalizeLanguageName(language.definition.id))
      );
    }
  }

  const defaultRuntime = new NativeHighlighterRuntime();

  return {
    createRuntime(options) {
      return new NativeHighlighterRuntime(options);
    },
    // Applies to the default runtime and to every runtime created afterwards,
    // matching the web-tree-sitter module this delegates to.
    configureWasmResolver(fn) {
      globalWasmResolver = fn;
      resolvers.configureWasmResolver(fn);
      defaultRuntime.configureWasmResolver(fn);
    },
    configureLanguagePackageResolver(fn) {
      globalLanguagePackageResolver = fn;
      resolvers.configureLanguagePackageResolver(fn);
      defaultRuntime.configureLanguagePackageResolver(fn);
    },
    resolveLanguagePackage(language, packageName, manifest) {
      return defaultRuntime.resolveLanguagePackage(language, packageName, manifest);
    },
    resolveParserWasm(language, wasm) {
      return defaultRuntime.resolveParserWasm(language, wasm);
    },
    initParser() {
      return defaultRuntime.initParser();
    },
    registerLanguage(definition) {
      defaultRuntime.registerLanguage(definition);
    },
    resolveLanguageId(nameOrAlias) {
      return defaultRuntime.resolveLanguageId(nameOrAlias);
    },
    loadLanguage(options) {
      return defaultRuntime.loadLanguage(options);
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
    availableLanguages(): LanguageInfo[] {
      return LANGUAGES.map((language) => cloneLanguageInfo(language));
    },
    getDefaultRuntime() {
      return defaultRuntime;
    },
  };
}

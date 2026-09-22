import type { WasmRef } from "../types.js";
import type { Parser } from "web-tree-sitter";

type ParserInitOptions = Parameters<typeof Parser.init>[0];

export interface RuntimeEnvironment {
  resolveWasm(
    wasm: Uint8Array | ArrayBuffer | string | URL | Response,
  ): Promise<Uint8Array | string>;
  readFsCache(key: string): Promise<Uint8Array | undefined>;
  writeFsCache(key: string, data: Uint8Array): Promise<void>;
  withFsCacheLock<T>(key: string, operation: () => Promise<T>): Promise<T>;
  /** Read a file already under `$LUMIS_DATA_DIR/parsers`, where the runtime has one. */
  readStagedAsset?(filename: string): Promise<Uint8Array | undefined>;
  readResolvedWasmFromDisk(source: string | URL): Promise<Uint8Array | undefined>;
  /**
   * Whether this runtime's parsers come from packages installed in a project.
   *
   * True on Node: the `@lumis-sh/wasm-*` packages a project depends on are the
   * whole set, the same way `Cargo.toml` features are in Rust and `mix.exs`
   * dependencies are in Elixir. A language outside it is not fetched.
   *
   * Absent where there is no project to read, such as the browser: a bundle
   * declares by what it imported, and there is no manifest to consult.
   */
  declaresLanguages?: boolean;
  /**
   * Where an installed `@lumis-sh/wasm-*` package keeps its `lumis.json`.
   *
   * Node resolves it through the package's export map. Absent in the browser,
   * which has no module resolution to ask.
   */
  resolveInstalledManifest?(packageName: string): Promise<URL | undefined>;
  /**
   * Which of `candidates` this project installed.
   *
   * Installing a package is the declaration, whether or not it ships a
   * manifest of its own — one published before manifests were part of a parser
   * package is still declared, and still has to be reachable.
   */
  installedPackages?(candidates: string[]): Promise<string[]>;
  parserInitOptions?(): Promise<ParserInitOptions>;
}

export interface RuntimeEnvironmentResolver {
  language: string;
  ref: WasmRef;
}

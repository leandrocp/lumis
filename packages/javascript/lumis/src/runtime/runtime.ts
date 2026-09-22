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
   * Whether this project declares the languages it uses, by depending on
   * `@lumis-sh/wasm-*` packages.
   *
   * When it does, that declaration is the whole set — the same way
   * `Cargo.toml` features are in Rust and `lumis-lock.toml` is in Elixir — and
   * a language outside it is not fetched. Absent, nothing is declared and
   * everything resolves on demand, which is what makes the zero-configuration
   * case work.
   *
   * Unimplemented where there is no project to read, such as the browser: a
   * bundle declares by what it imported, and there is no manifest to consult.
   */
  declaresLanguages?(): Promise<boolean>;
  parserInitOptions?(): Promise<ParserInitOptions>;
}

export interface RuntimeEnvironmentResolver {
  language: string;
  ref: WasmRef;
}

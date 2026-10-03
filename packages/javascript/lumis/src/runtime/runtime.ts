import type { WasmRef } from "../types.js";

export interface RuntimeEnvironment {
  resolveWasm(
    wasm: Uint8Array | ArrayBuffer | string | URL | Response,
  ): Promise<Uint8Array | string>;
  readFsCache(key: string): Promise<Uint8Array | undefined>;
  writeFsCache(key: string, data: Uint8Array): Promise<void>;
  withFsCacheLock<T>(key: string, operation: () => Promise<T>): Promise<T>;
  readResolvedWasmFromDisk(source: string | URL): Promise<Uint8Array | undefined>;
  /**
   * Where an installed `@lumis-sh/wasm-*` package keeps its `lumis.json`.
   *
   * Node resolves it through the package's export map, and loads only those
   * packages unless a resolver is configured. Absent in the browser, which has
   * no project to read and loads only the packages passed to `withWasm()`.
   */
  resolveInstalledManifest?(packageName: string): Promise<URL | undefined>;
}

export interface RuntimeEnvironmentResolver {
  language: string;
  ref: WasmRef;
}

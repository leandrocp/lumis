import { LOCK_RETRY_MS, LOCK_STALE_AFTER_MS, LOCK_TIMEOUT_MS } from "../cache-timing.js";

// oxlint-disable-next-line no-useless-concat -- keeps a bundler from resolving the specifier statically.
const nodeFsPromises = "node:fs" + "/promises";
const nodePath = "node:path";
const nodeOs = "node:os";

/** @internal */
export function isUrlString(source: string): boolean {
  if (/^[a-zA-Z]:/.test(source)) return false;
  try {
    // oxlint-disable-next-line no-new -- constructing it is the validity test.
    new URL(source);
    return true;
  } catch {
    return false;
  }
}

/** @internal */
export function wasmCacheFilename(key: string): string {
  const extension = key.startsWith("language-package-") ? "json" : "wasm";
  return `${encodeURIComponent(key)}.${extension}`;
}

/** @internal */
/**
 * Everything Lumis persists lives under one directory, `LUMIS_DATA_DIR`.
 *
 * The addon resolves the default through `etcetera`, so asking it rather than
 * deciding here keeps the Wasm runtime on the store the native runtime, the CLI
 * and the Elixir NIF already share. {@link platformDataDir} answers only where
 * no addon is built, and a test pins it against the Rust result.
 */
export async function dataDir(): Promise<string> {
  if (process.env.LUMIS_DATA_DIR) return process.env.LUMIS_DATA_DIR;
  const { loadAddon } = await import("../native-binding.js");
  return loadAddon()?.defaultDataDir() ?? (await platformDataDir());
}

/** @internal */
/**
 * `etcetera::choose_base_strategy`, ported: XDG everywhere except Windows, where
 * it is `%APPDATA%`. A relative `XDG_DATA_HOME` is ignored, as the XDG spec
 * requires and as `etcetera` implements.
 *
 * Exported so `test/data-dir-parity.test.ts` can pin it against the addon.
 */
export async function platformDataDir(): Promise<string> {
  const { isAbsolute, join } = await import(nodePath);
  const { homedir } = await import(nodeOs);

  if (process.platform === "win32") {
    const appData = process.env.APPDATA ?? join(homedir(), "AppData", "Roaming");
    return join(appData, "lumis");
  }

  const xdgDataHome = process.env.XDG_DATA_HOME;
  return xdgDataHome && isAbsolute(xdgDataHome)
    ? join(xdgDataHome, "lumis")
    : join(homedir(), ".local", "share", "lumis");
}

/**
 * Language packages and parser WASM, the same subdirectory every runtime uses.
 *
 * A `directory` argument anywhere in this module means a data directory, the
 * thing `LUMIS_DATA_DIR` names, so writers and readers cannot disagree.
 */
export async function wasmCacheDir(directory?: string): Promise<string> {
  const { join } = await import(nodePath);
  return join(directory ?? (await dataDir()), "parsers");
}

/** @internal */
export async function wasmCachePath(key: string, directory?: string): Promise<string> {
  const { join } = await import(nodePath);
  return join(await wasmCacheDir(directory), wasmCacheFilename(key));
}

/** @internal */
export async function readCachedWasm(
  key: string,
  directory?: string,
): Promise<Uint8Array | undefined> {
  try {
    const { readFile } = await import(nodeFsPromises);
    return new Uint8Array(await readFile(await wasmCachePath(key, directory)));
  } catch {
    return undefined;
  }
}

/** @internal */
export async function writeCachedWasm(
  key: string,
  data: Uint8Array,
  directory?: string,
): Promise<string> {
  const { join } = await import(nodePath);
  const resolvedDirectory = await wasmCacheDir(directory);
  const filePath = join(resolvedDirectory, wasmCacheFilename(key));
  const temporary = `${filePath}.${process.pid}.${Date.now()}.tmp`;
  try {
    const { writeFile, mkdir, rename, rm } = await import(nodeFsPromises);
    await mkdir(resolvedDirectory, { recursive: true });
    await writeFile(temporary, data, { flag: "wx" });
    try {
      await rename(temporary, filePath);
    } catch (error) {
      if (!isNodeError(error, "EEXIST") && !isNodeError(error, "EPERM")) throw error;
      await rm(filePath, { force: true });
      await rename(temporary, filePath);
    }
    return filePath;
  } finally {
    try {
      const { rm } = await import(nodeFsPromises);
      await rm(temporary, { force: true });
    } catch {
      // best-effort temporary cleanup
    }
  }
}

function isNodeError(error: unknown, code: string): boolean {
  return error instanceof Error && "code" in error && error.code === code;
}

interface LockOwner {
  host: string;
  pid: number;
}

/** @internal */
export function lockOwnerIsGone(owner: LockOwner | undefined, host: string): boolean {
  // A pid is only meaningful on the machine that wrote it. Anywhere else, and
  // for an unreadable lock, fall back to the staleness timer.
  if (!owner || owner.host !== host) return false;
  try {
    process.kill(owner.pid, 0);
    return false;
  } catch (error) {
    // EPERM means the process exists but belongs to someone else.
    return !isNodeError(error, "EPERM");
  }
}

async function readLockOwner(lockPath: string): Promise<LockOwner | undefined> {
  const { readFile } = await import(nodeFsPromises);
  try {
    const owner: unknown = JSON.parse(await readFile(lockPath, "utf8"));
    return isLockOwner(owner) ? { host: owner.host, pid: owner.pid } : undefined;
  } catch {
    return undefined;
  }
}

function isLockOwner(owner: unknown): owner is LockOwner {
  if (typeof owner !== "object" || owner === null) return false;

  const record = owner as Record<string, unknown>;
  return (
    typeof record.host === "string" &&
    typeof record.pid === "number" &&
    Number.isInteger(record.pid) &&
    record.pid > 0
  );
}

/** @internal */
export async function withWasmCacheLock<T>(
  key: string,
  operation: () => Promise<T>,
  directory?: string,
): Promise<T> {
  const resolvedDirectory = await wasmCacheDir(directory);
  const { mkdir, open, rm, stat } = await import(nodeFsPromises);
  const { join } = await import(nodePath);
  const { hostname } = await import(nodeOs);
  await mkdir(resolvedDirectory, { recursive: true });
  const lockPath = join(resolvedDirectory, `${wasmCacheFilename(key)}.lock`);
  const host = hostname();
  const deadline = Date.now() + LOCK_TIMEOUT_MS;
  let lock: { close(): Promise<void>; writeFile(data: string): Promise<void> } | undefined;

  while (!lock) {
    try {
      const opened = await open(lockPath, "wx");
      await opened.writeFile(JSON.stringify({ host, pid: process.pid } satisfies LockOwner));
      lock = opened;
    } catch (error) {
      if (!isNodeError(error, "EEXIST")) throw error;

      if (lockOwnerIsGone(await readLockOwner(lockPath), host)) {
        await rm(lockPath, { force: true });
        continue;
      }

      const age = await stat(lockPath)
        .then((info: { mtimeMs: number }) => Date.now() - info.mtimeMs)
        .catch(() => 0);
      if (age > LOCK_STALE_AFTER_MS) {
        await rm(lockPath, { force: true });
        continue;
      }

      if (Date.now() >= deadline) {
        throw new Error(`Timed out waiting for Lumis WASM cache lock: ${lockPath}`, {
          cause: error,
        });
      }
      await new Promise((resolve) => {
        setTimeout(resolve, LOCK_RETRY_MS);
      });
    }
  }

  try {
    return await operation();
  } finally {
    try {
      await lock.close();
    } finally {
      await rm(lockPath, { force: true });
    }
  }
}

/**
 * Whether the nearest `package.json` depends on any `@lumis-sh/wasm-*` package.
 *
 * That dependency list is a JavaScript project's declaration of the languages it
 * uses, so its presence closes the set the way a `lumis-lock.toml` does for
 * Elixir. Searched upward from the working directory, as a lock is, so a command
 * run inside a workspace package still finds the manifest that installed them.
 *
 * Read once. A dependency list does not change while a process runs, and asking
 * the filesystem on every resolution would put a stat in the highlight path.
 */
let declaredLanguages: Promise<boolean> | undefined;

export function declaresLanguages(): Promise<boolean> {
  declaredLanguages ??= detectDeclaredLanguages();
  return declaredLanguages;
}

/** Drop the memoized answer. For tests, which move between fixture projects. */
export function __resetDeclaredLanguages(): void {
  declaredLanguages = undefined;
}

async function detectDeclaredLanguages(): Promise<boolean> {
  const { readFile } = await import("node:fs/promises");
  const { dirname, join } = await import("node:path");

  let directory = process.cwd();
  for (;;) {
    try {
      const manifest: unknown = JSON.parse(await readFile(join(directory, "package.json"), "utf8"));
      if (dependsOnParser(manifest)) return true;
    } catch {
      // No manifest here, or one that is not readable JSON. Keep walking: a
      // missing file is the common case at every level but one.
    }

    const parent = dirname(directory);
    if (parent === directory) return false;
    directory = parent;
  }
}

/**
 * `dependencies` only.
 *
 * `devDependencies` are not shipped, so they say nothing about what an
 * application may load — they are what its own tests and build need. Counting
 * them would close the set for any library that tests against a parser,
 * including this package, whose `devDependencies` list two.
 *
 * `optionalDependencies` are excluded for a different reason: they fail to
 * install silently, by design. A parser listed there that did not install would
 * still close the set and then be refused itself, which is a worse outcome than
 * downloading it.
 */
function dependsOnParser(manifest: unknown): boolean {
  if (typeof manifest !== "object" || manifest === null) return false;
  const dependencies = (manifest as Record<string, unknown>).dependencies;
  if (typeof dependencies !== "object" || dependencies === null) return false;
  return Object.keys(dependencies).some((name) => name.startsWith("@lumis-sh/wasm-"));
}

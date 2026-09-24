import type { TomlTableWithoutBigInt, TomlValueWithoutBigInt } from "smol-toml";

export interface ParserEntry {
  aliases?: string[];
  emacs?: string[];
  shebang?: string[];
  wasm_name?: string;
  display_name?: string;
  variant?: string;
  globs?: string[];
}

export interface BundleEntry {
  parsers: string[] | "all";
  /** Parsers to leave out of a `parsers: "all"` bundle. */
  exclude?: string[];
}

export interface LanguagesToml {
  parsers: Record<string, ParserEntry>;
  bundles?: Record<string, BundleEntry>;
}

function invalid(path: string): never {
  throw new Error(`Invalid languages.toml: ${path}`);
}

function requireSafeName(value: string, path: string): void {
  if (value.length === 0 || value === "." || value === ".." || /[/\\]/.test(value)) {
    invalid(path);
  }
}

function requireTable(
  value: TomlValueWithoutBigInt | undefined,
  path: string,
): TomlTableWithoutBigInt {
  if (
    typeof value !== "object" ||
    value === null ||
    Array.isArray(value) ||
    value instanceof Date
  ) {
    return invalid(path);
  }
  return value;
}

function optionalString(
  table: TomlTableWithoutBigInt,
  key: string,
  path: string,
): string | undefined {
  const value = table[key];
  if (value === undefined || typeof value === "string") return value;
  return invalid(`${path}.${key}`);
}

function optionalStringArray(
  table: TomlTableWithoutBigInt,
  key: string,
  path: string,
): string[] | undefined {
  const value = table[key];
  if (value === undefined) return undefined;
  if (Array.isArray(value) && value.every((entry) => typeof entry === "string")) return value;
  return invalid(`${path}.${key}`);
}

function parseParser(value: TomlValueWithoutBigInt, id: string): ParserEntry {
  const path = `parsers.${id}`;
  const table = requireTable(value, path);
  return {
    aliases: optionalStringArray(table, "aliases", path),
    emacs: optionalStringArray(table, "emacs", path),
    shebang: optionalStringArray(table, "shebang", path),
    wasm_name: optionalString(table, "wasm_name", path),
    display_name: optionalString(table, "display_name", path),
    variant: optionalString(table, "variant", path),
    globs: optionalStringArray(table, "globs", path),
  };
}

function parseBundle(value: TomlValueWithoutBigInt, name: string): BundleEntry {
  const path = `bundles.${name}`;
  const table = requireTable(value, path);
  const parsers = table.parsers;
  const rawExclude = table.exclude;
  let exclude: string[] | undefined;
  if (rawExclude !== undefined) {
    if (!Array.isArray(rawExclude) || !rawExclude.every((entry) => typeof entry === "string")) {
      return invalid(`${path}.exclude`);
    }
    exclude = rawExclude;
  }
  if (parsers === "all") return { parsers, exclude };
  if (Array.isArray(parsers) && parsers.every((entry) => typeof entry === "string")) {
    return { parsers, exclude };
  }
  return invalid(`${path}.parsers`);
}

export function parseLanguagesToml(document: TomlTableWithoutBigInt): LanguagesToml {
  const parserTable = requireTable(document.parsers, "parsers");
  const parsers = Object.fromEntries(
    Object.entries(parserTable).map(([id, value]) => {
      requireSafeName(id, `parsers.${id}`);
      return [id, parseParser(value, id)];
    }),
  );

  if (document.bundles === undefined) return { parsers };
  const bundleTable = requireTable(document.bundles, "bundles");
  const bundles = Object.fromEntries(
    Object.entries(bundleTable).map(([name, value]) => {
      const path = `bundles.${name}`;
      requireSafeName(name, path);
      const bundle = parseBundle(value, name);
      if (
        bundle.parsers !== "all" &&
        bundle.parsers.some((id) => id !== "plaintext" && !Object.hasOwn(parsers, id))
      ) {
        return invalid(`${path}.parsers`);
      }
      return [name, bundle];
    }),
  );
  return { parsers, bundles };
}

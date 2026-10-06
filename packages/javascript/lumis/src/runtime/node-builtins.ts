type NodeBuiltinName = "node:fs/promises" | "node:path" | "node:os" | "node:url";
type NodeBuiltin =
  | typeof import("node:fs/promises")
  | typeof import("node:path")
  | typeof import("node:os")
  | typeof import("node:url");

export function importNodeBuiltin(
  name: "node:fs/promises",
): Promise<typeof import("node:fs/promises")>;
export function importNodeBuiltin(name: "node:path"): Promise<typeof import("node:path")>;
export function importNodeBuiltin(name: "node:os"): Promise<typeof import("node:os")>;
export function importNodeBuiltin(name: "node:url"): Promise<typeof import("node:url")>;
export async function importNodeBuiltin(name: NodeBuiltinName): Promise<NodeBuiltin> {
  // Keep the runtime specifier opaque to browser bundlers; TypeScript types this
  // dynamic built-in import as any even though callers use this closed union.
  // The shared lint config permits this assignment only in this boundary module.
  const module: NodeBuiltin = await import(name);
  return module;
}

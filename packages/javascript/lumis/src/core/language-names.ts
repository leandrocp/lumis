/**
 * Turning the names a caller writes into the language definitions the runtime
 * takes. Free of Node imports, so the browser entry can use it too.
 */
import { BUNDLES } from "../generated/bundles-meta.js";
import { EXACT_LANGUAGE_MAP } from "../generated/language-detection.js";
import { LANGUAGE_LOADERS } from "../generated/language-loaders.js";
import type { Language } from "../types.js";

/**
 * Members of `bundle-<name>`, accepting `-` or `_` between words so Elixir's
 * `:bundle_web_extra` and the CLI's `bundle-web-extra` reach the same entry.
 */
function bundleMembers(name: string): string[] | undefined {
  const lower = name.toLowerCase();
  if (!lower.startsWith("bundle-") && !lower.startsWith("bundle_")) return undefined;

  const suffix = lower.slice("bundle-".length).replaceAll("_", "-");
  return BUNDLES[suffix];
}

/**
 * Expand every `bundle-<name>` token into its members, leaving other names
 * alone. Mirrors `catalog::expand_bundles` in `lumis-wasm-runtime`.
 */
export function expandBundles(names: Iterable<string>): string[] {
  const expanded: string[] = [];

  for (const name of names) {
    const members = bundleMembers(name);
    if (members) {
      expanded.push(...members);
      continue;
    }

    const lower = name.toLowerCase();
    if (lower.startsWith("bundle-") || lower.startsWith("bundle_")) {
      throw new Error(`Unknown bundle "${name}"`);
    }

    expanded.push(name);
  }

  return [...new Set(expanded)];
}

/** Resolve one catalog name or alias to its language definition. */
export async function resolveLanguage(name: string): Promise<Language> {
  const normalized = name.toLowerCase();
  const id = EXACT_LANGUAGE_MAP[normalized] ?? normalized;
  const loader = LANGUAGE_LOADERS[id];
  if (!loader) throw new Error(`Unknown language "${name}"`);
  return (await loader()).default;
}

import fs from "node:fs";
import path from "node:path";
import { parse as parseToml } from "smol-toml";
import {
  parseLanguagesToml,
  type BundleEntry,
  type LanguagesToml,
  type ParserEntry,
} from "../lumis/scripts/languages-toml.js";

const WORKSPACE_ROOT = path.resolve(import.meta.dirname, "../../..");
const LANGUAGES_TOML = path.join(WORKSPACE_ROOT, "languages.toml");
const LUMIS_PACKAGE_JSON = path.resolve(import.meta.dirname, "../lumis/package.json");
/// Bundles are generated, never committed. They carry no code of their own —
/// a manifest, an import list and a README, all derivable from languages.toml —
/// so a copy in the tree is a second source of truth that goes stale the moment
/// membership changes without someone bumping a version by hand.
///
/// `--out` is where they land and `--version` is what they claim. The release
/// planner owns both: it decides the version from what is published, the same
/// way it does for parsers.
function argValue(flag: string): string | undefined {
  const index = process.argv.indexOf(flag);
  return index === -1 ? undefined : process.argv[index + 1];
}

const OUT_DIR = path.resolve(WORKSPACE_ROOT, argValue("--out") ?? path.join("tmp", "wasm", "npm"));
const VERSION = argValue("--version") ?? "0.0.0";

/**
 * Carried as `lumis.bundleFormat`, so a change to what a bundle exports publishes
 * a new version even when its members stay the same. 3: the bundle carries its
 * module URL so Node can find its dependencies from any working directory. Must match
 * `BUNDLE_FORMAT_VERSION` in `crates/dev`.
 */
const BUNDLE_FORMAT_VERSION = 3;

function readLanguagesToml(): LanguagesToml {
  const text = fs.readFileSync(LANGUAGES_TOML, "utf-8");
  return parseLanguagesToml(parseToml(text));
}

function treeSitterCompatRange(): string {
  const packageJson: { devDependencies?: Record<string, string>; version?: string } = JSON.parse(
    fs.readFileSync(LUMIS_PACKAGE_JSON, "utf-8"),
  );
  // A dev dependency: tsup inlines it into `dist`.
  const spec = packageJson.devDependencies?.["web-tree-sitter"];
  const match = spec?.match(/(\d+\.\d+)/u);

  if (!match) {
    throw new Error("Could not determine web-tree-sitter compatibility from package.json");
  }

  return `^${match[1]}.0`;
}

/** The first Lumis that reads a bundle's default export. */
function lumisVersionRange(): string {
  return ">=0.9.0";
}

function wasmNameForLanguage(id: string, entry: ParserEntry | undefined): string {
  return entry?.wasm_name || `tree-sitter-${id}`;
}

function wasmPackageName(wasmName: string): string {
  return `@lumis-sh/wasm-${wasmName.startsWith("tree-sitter-") ? wasmName.slice("tree-sitter-".length) : wasmName}`;
}

function packageDir(bundleName: string): string {
  return path.join(OUT_DIR, `wasm-bundle-${bundleName}`);
}

function unique<T>(values: T[]): T[] {
  return [...new Set(values)];
}

function bundleLanguageIds(bundle: BundleEntry, allParserIds: string[]): string[] {
  // `exclude` only applies to `"all"`: an explicit list already says what it
  // wants. Without this the npm manifest keeps the excluded parser, and
  // `stage_hex_bundle` copies those dependencies straight into the Hex bundle.
  return bundle.parsers === "all"
    ? allParserIds.filter((id) => !bundle.exclude?.includes(id))
    : bundle.parsers;
}

function writeBundlePackage(
  bundleName: string,
  languageIds: string[],
  parsers: Record<string, ParserEntry>,
) {
  const dir = packageDir(bundleName);
  fs.rmSync(dir, { recursive: true, force: true });
  fs.mkdirSync(dir, { recursive: true });

  const wasmPackagesByLanguage = Object.fromEntries(
    languageIds.map((id) => {
      const wasmName = wasmNameForLanguage(id, parsers[id]);
      return [id, wasmPackageName(wasmName)];
    }),
  );

  const dependencyPackages = unique(Object.values(wasmPackagesByLanguage)).sort();
  const wasmVersionRange = treeSitterCompatRange();
  const lumisVersion = lumisVersionRange();
  const publishedPackages = new Set(dependencyPackages);

  const lazyEntries = languageIds
    .filter((id) => id !== "plaintext")
    .map((id) => {
      const pkg = JSON.stringify(wasmPackagesByLanguage[id]!);
      const aliases = JSON.stringify(parsers[id]?.aliases ?? []);
      return `  ${JSON.stringify(id)}: lazy(${JSON.stringify(id)}, ${aliases}, () => import(${pkg}).then((m) => m[${JSON.stringify(id)}])),`;
    })
    .join("\n");

  // Dynamic imports, so a bundler splits each language into its own chunk and a
  // page downloads only the languages it highlights.
  const indexJs = `const lazy = (id, aliases, load) => Object.assign(load, { id, aliases })

/**
 * Every language in this bundle, each loaded from its package the first time
 * it is used. Pass it to \`createHighlighter({ languages })\`.
 */
const bundle = {
  [Symbol.for('@lumis-sh/package-url')]: import.meta.url,
${lazyEntries}
}

export default bundle
`;

  const indexDts = `import type { LanguageBundle } from '@lumis-sh/lumis'

declare const bundle: LanguageBundle
export default bundle
`;

  const dependencies = Object.fromEntries(
    [...publishedPackages].sort().map((pkg) => [pkg, wasmVersionRange]),
  );
  const packageJson = {
    name: `@lumis-sh/wasm-bundle-${bundleName}`,
    version: VERSION,
    description: `Lumis WASM ${bundleName} language bundle`,
    author: "Leandro Pereira",
    license: "MIT",
    repository: {
      type: "git",
      url: "git+https://github.com/leandrocp/lumis.git",
      directory: `packages/javascript/wasm-bundle-${bundleName}`,
    },
    bugs: "https://github.com/leandrocp/lumis/issues",
    lumis: { bundleFormat: BUNDLE_FORMAT_VERSION },
    homepage: "https://lumis.sh",
    keywords: ["lumis-sh", "tree-sitter", "wasm", "bundle"],
    sideEffects: false,
    type: "module",
    packageManager: "pnpm@11.15.1",
    exports: {
      ".": {
        types: "./index.d.ts",
        import: "./index.js",
        default: "./index.js",
      },
    },
    files: ["index.js", "index.d.ts", "README.md"],
    publishConfig: {
      access: "public",
    },
    peerDependencies: {
      "@lumis-sh/lumis": lumisVersion,
    },
    devDependencies: {
      "@lumis-sh/lumis": lumisVersion,
    },
    dependencies,
  };

  const readme = `# @lumis-sh/wasm-bundle-${bundleName}

Lumis WASM ${bundleName} language bundle.

## Install

\`\`\`sh
npm install @lumis-sh/lumis @lumis-sh/wasm-bundle-${bundleName}
\`\`\`

## Usage

The package exports the bundle, in Node and in a browser. Each language loads from its package the first time it is used:

\`\`\`ts
import { createHighlighter } from '@lumis-sh/lumis'
import ${bundleName.replaceAll(/-(.)/gu, (_m, c: string) => c.toUpperCase())} from '@lumis-sh/wasm-bundle-${bundleName}'

const highlighter = await createHighlighter({ languages: [${bundleName.replaceAll(/-(.)/gu, (_m, c: string) => c.toUpperCase())}] })
\`\`\`
`;

  fs.writeFileSync(path.join(dir, "index.js"), indexJs);
  fs.writeFileSync(path.join(dir, "index.d.ts"), indexDts);
  fs.writeFileSync(path.join(dir, "package.json"), JSON.stringify(packageJson, null, 2) + "\n");
  fs.writeFileSync(path.join(dir, "README.md"), readme);

  console.log(`  wasm bundle ${bundleName}@${VERSION}: ${path.relative(WORKSPACE_ROOT, dir)}`);
}

function main() {
  const config = readLanguagesToml();
  const bundles = config.bundles ?? {};
  const allParserIds = Object.keys(config.parsers);
  const only = argValue("--bundle");

  for (const [bundleName, bundle] of Object.entries(bundles)) {
    if (only !== undefined && only !== bundleName) continue;
    writeBundlePackage(bundleName, bundleLanguageIds(bundle, allParserIds), config.parsers);
  }
}

main();

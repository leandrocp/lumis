import { execFileSync } from "node:child_process";
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
const PACKAGES_DIR = path.join(WORKSPACE_ROOT, "packages", "javascript");

interface BundlePackageJson {
  version?: string;
}

function readLanguagesToml(): LanguagesToml {
  const text = fs.readFileSync(LANGUAGES_TOML, "utf-8");
  return parseLanguagesToml(parseToml(text));
}

function treeSitterCompatRange(): string {
  const packageJson: { dependencies?: Record<string, string>; version?: string } = JSON.parse(
    fs.readFileSync(LUMIS_PACKAGE_JSON, "utf-8"),
  );
  const spec = packageJson.dependencies?.["web-tree-sitter"];
  const match = spec?.match(/(\d+\.\d+)/);

  if (!match) {
    throw new Error("Could not determine web-tree-sitter compatibility from package.json");
  }

  return `^${match[1]}.0`;
}

function lumisVersionRange(): string {
  return ">=0.0.1";
}

function wasmNameForLanguage(id: string, entry: ParserEntry | undefined): string {
  if (id === "plaintext") return "tree-sitter-diff";
  return entry?.wasm_name || `tree-sitter-${id}`;
}

function wasmPackageName(wasmName: string): string {
  return `@lumis-sh/wasm-${wasmName.startsWith("tree-sitter-") ? wasmName.slice("tree-sitter-".length) : wasmName}`;
}

function packageDir(bundleName: string): string {
  return path.join(PACKAGES_DIR, `wasm-bundle-${bundleName}`);
}

function importName(packageName: string): string {
  const cleaned = packageName
    .replace("@lumis-sh/", "")
    .replaceAll(/[^a-zA-Z0-9]+(.)/g, (_match, next: string) => next.toUpperCase())
    .replaceAll(/[^a-zA-Z0-9]/g, "");
  return cleaned.replace(/^[A-Z]/, (char) => char.toLowerCase());
}

function unique<T>(values: T[]): T[] {
  return [...new Set(values)];
}

function readBundlePackageJson(dir: string): BundlePackageJson | null {
  const file = path.join(dir, "package.json");
  if (!fs.existsSync(file)) return null;
  return JSON.parse(fs.readFileSync(file, "utf-8")) as BundlePackageJson;
}

function readBundleChangelog(dir: string): string | null {
  const file = path.join(dir, "CHANGELOG.md");
  if (!fs.existsSync(file)) return null;
  return fs.readFileSync(file, "utf-8");
}

function bundleLanguageIds(bundle: BundleEntry, allParserIds: string[]): string[] {
  // `exclude` only applies to `"all"`: an explicit list already says what it
  // wants. Without this the npm manifest keeps the excluded parser, and
  // `stage_hex_bundle` copies those dependencies straight into the Hex bundle.
  const parserIds =
    bundle.parsers === "all"
      ? allParserIds.filter((id) => !bundle.exclude?.includes(id))
      : bundle.parsers;
  return parserIds.includes("plaintext") ? parserIds : [...parserIds, "plaintext"];
}

function writeBundlePackage(
  bundleName: string,
  languageIds: string[],
  parsers: Record<string, ParserEntry>,
) {
  const dir = packageDir(bundleName);
  fs.mkdirSync(dir, { recursive: true });

  const existingPackageJson = readBundlePackageJson(dir);
  const existingChangelog = readBundleChangelog(dir);

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

  const importLines = [...publishedPackages]
    .sort()
    .map((pkg) => `import ${importName(pkg)} from ${JSON.stringify(pkg)}`)
    .join("\n");

  const entries = languageIds
    .map((id) => `  ${JSON.stringify(id)}: ${importName(wasmPackagesByLanguage[id]!)},`)
    .join("\n");

  const indexJs = `${importLines}

export const bundledWasms = {
${entries}
}

export default bundledWasms
`;

  const indexDts = `import type { RuntimeWasmBundle } from '@lumis-sh/lumis'

export declare const bundledWasms: RuntimeWasmBundle
export default bundledWasms
`;

  const dependencies = Object.fromEntries(
    [...publishedPackages].sort().map((pkg) => [pkg, wasmVersionRange]),
  );
  const packageJson = {
    name: `@lumis-sh/wasm-bundle-${bundleName}`,
    version: existingPackageJson?.version ?? "0.0.1",
    description: `Lumis WASM ${bundleName} language bundle`,
    author: "Leandro Pereira",
    license: "MIT",
    repository: {
      type: "git",
      url: "git+https://github.com/leandrocp/lumis.git",
      directory: `packages/javascript/wasm-bundle-${bundleName}`,
    },
    bugs: "https://github.com/leandrocp/lumis/issues",
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
    files: ["index.js", "index.d.ts", "README.md", "CHANGELOG.md"],
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

## Node.js

Install this package alongside \`@lumis-sh/lumis/bundles/${bundleName}\` and Lumis will resolve the local parser packages automatically.

## Browser bundlers

\`\`\`ts
import { createHighlighter, withWasmBundle } from '@lumis-sh/lumis'
import { bundledLanguages } from '@lumis-sh/lumis/bundles/${bundleName}'
import { bundledWasms } from '@lumis-sh/wasm-bundle-${bundleName}'

const languages = withWasmBundle(bundledLanguages, bundledWasms)
const highlighter = await createHighlighter({ languages: [languages] })
\`\`\`
`;

  const changelog = "# Changelog\n\n";

  fs.writeFileSync(path.join(dir, "index.js"), indexJs);
  fs.writeFileSync(path.join(dir, "index.d.ts"), indexDts);
  fs.writeFileSync(path.join(dir, "package.json"), JSON.stringify(packageJson, null, 2) + "\n");
  fs.writeFileSync(path.join(dir, "README.md"), readme);
  fs.writeFileSync(path.join(dir, "CHANGELOG.md"), existingChangelog ?? changelog);

  execFileSync("oxfmt", [path.join(dir, "index.js"), path.join(dir, "index.d.ts")], {
    stdio: "inherit",
  });
  console.log(`  wasm bundle ${bundleName}: packages/javascript/wasm-bundle-${bundleName}`);
}

function main() {
  const config = readLanguagesToml();
  const bundles = config.bundles ?? {};
  const allParserIds = Object.keys(config.parsers);

  for (const entry of fs.readdirSync(PACKAGES_DIR, { withFileTypes: true })) {
    if (!entry.isDirectory() || !entry.name.startsWith("wasm-bundle-")) continue;
    const bundleName = entry.name.slice("wasm-bundle-".length);
    if (bundleName in bundles) continue;
    fs.rmSync(path.join(PACKAGES_DIR, entry.name), { recursive: true, force: true });
    console.log(`  removed stale wasm bundle ${bundleName}: packages/javascript/${entry.name}`);
  }

  for (const [bundleName, bundle] of Object.entries(bundles)) {
    writeBundlePackage(bundleName, bundleLanguageIds(bundle, allParserIds), config.parsers);
  }
}

main();

import { existsSync, readdirSync, statSync } from "node:fs";
import { join, parse } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

import { describe, expect, it } from "vitest";

import { availableLanguages } from "../src/index.js";

type LanguageModule = {
  id: string;
  aliases?: string[];
  packageName?: string;
};

const repoRoot = fileURLToPath(new URL("../../../..", import.meta.url));
const langsRoot = fileURLToPath(new URL("../langs/", import.meta.url));
const samplesDir = join(repoRoot, "samples");

const sampleBaseNameOverrides = new Map<string, string>([["ocaml_interface", "ocamlinterface"]]);

const sampleFiles = new Map(
  readdirRecursive(samplesDir).map(
    (filePath) => [parse(filePath).name.toLowerCase(), filePath] as const,
  ),
);

function readdirRecursive(dir: string): string[] {
  return readdirSync(dir).flatMap((entry) => {
    const fullPath = join(dir, entry);
    return statSync(fullPath).isDirectory() ? readdirRecursive(fullPath) : [fullPath];
  });
}

function listSourceLanguageIds(): string[] {
  return readdirSync(langsRoot)
    .filter((entry) => entry.endsWith(".ts"))
    .map((entry) => parse(entry).name)
    .sort((a, b) => a.localeCompare(b));
}

async function loadLanguageModule(id: string): Promise<LanguageModule> {
  const moduleUrl = pathToFileURL(join(langsRoot, `${id}.ts`)).href;
  const mod = await import(moduleUrl);
  return mod.default as LanguageModule;
}

function resolveSamplePath(language: LanguageModule): string | undefined {
  const candidates = [
    language.id,
    sampleBaseNameOverrides.get(language.id),
    ...(language.aliases ?? []),
  ]
    .filter((value): value is string => value != null)
    .map((value) => value.toLowerCase());

  for (const candidate of candidates) {
    const samplePath = sampleFiles.get(candidate);
    if (samplePath) return samplePath;
  }

  return undefined;
}

const sourceLanguageIds = listSourceLanguageIds();
const availableLanguageIds = availableLanguages()
  .map((language) => language.id)
  .sort((a, b) => a.localeCompare(b));

const languageFixtures = await Promise.all(
  sourceLanguageIds.map(async (id) => {
    const language = await loadLanguageModule(id);
    const samplePath = resolveSamplePath(language);

    if (!samplePath) {
      throw new Error(`No sample file found for language "${id}"`);
    }

    return { id, language, samplePath };
  }),
);

describe.skipIf(languageFixtures.length === 0)("all languages", () => {
  it("matches source modules to available languages", () => {
    expect(sourceLanguageIds).toEqual(availableLanguageIds);
  });

  it.each(languageFixtures)(
    "loads $id and has a sample fixture",
    async ({ id, language, samplePath }) => {
      expect(language.id).toBe(id);
      if (id === "plaintext") {
        // oxlint-disable-next-line vitest/no-conditional-expect -- plaintext has no parser package; both fixture branches assert the package contract.
        expect(language.packageName).toBeUndefined();
      } else {
        // oxlint-disable-next-line vitest/no-conditional-expect -- plaintext has no parser package; both fixture branches assert the package contract.
        expect(typeof language.packageName).toBe("string");
      }
      expect(existsSync(samplePath)).toBe(true);
    },
  );
});

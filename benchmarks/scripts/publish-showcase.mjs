#!/usr/bin/env node

import { cp, mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { lumisReference } from "./implementations.mjs";

const benchmarksDir = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const generatedDir = resolve(benchmarksDir, "showcase/generated");
const target = resolve(benchmarksDir, "../website/public/comparison-data");

const manifest = JSON.parse(await readFile(resolve(generatedDir, "manifest.json"), "utf8"));
const lumis = manifest.implementations.filter((entry) => entry.id.startsWith("lumis-"));
const referenceRuntime = lumis.find((entry) => entry.id === lumisReference);
const others = manifest.implementations.filter((entry) => !entry.id.startsWith("lumis-"));

// Every Lumis runtime renders a document identically, so the site ships one copy
// and says so, rather than five tabs of the same bytes and a third of the weight
// again. Publishing asserts that rather than trusting it: if they ever diverge,
// the page must not quietly present one of them as the answer.
for (const document of manifest.documents) {
  for (const theme of manifest.themes) {
    const fragments = await Promise.all(
      lumis.map(async (entry) => ({
        label: entry.label,
        bytes: await readFile(
          resolve(generatedDir, "fragments", document.id, theme.id, `${entry.id}.html`),
        ),
      })),
    );
    const [reference, ...rest] = fragments;
    for (const other of rest) {
      if (!other.bytes.equals(reference.bytes)) {
        throw new Error(
          `${other.label} and ${reference.label} disagree on ${document.id} in ${theme.name}; ` +
            "the comparison cannot be published as a single Lumis output",
        );
      }
    }
  }
}

const implementations = [
  { id: "lumis", label: "Lumis", version: referenceRuntime.version, theme: referenceRuntime.theme },
  ...others,
];

const published = {
  schemaVersion: 4,
  themes: manifest.themes,
  lumisRuntimes: lumis.map((entry) => entry.label),
  implementations,
  documents: manifest.documents.map((document) => ({
    id: document.id,
    label: document.label,
    language: document.language,
    languageLabel: document.languageLabel,
    source: document.source,
    lines: document.lines,
    bytes: document.bytes,
    injections: document.injections,
    // The implementations with no output for this document, so the page can say
    // why rather than load a file that was never written.
    unsupported: document.unsupported,
    ...publishedOutputs(document),
  })),
};

// The website is deployed by a workflow that builds only `website/` and `docs/`,
// so it can never run the showcase itself: that needs Rust, Elixir, bat and every
// parser. These bytes are therefore committed, and this is what refreshes them.
await rm(target, { recursive: true, force: true });
await mkdir(target, { recursive: true });
await writeFile(resolve(target, "manifest.json"), `${JSON.stringify(published, null, 2)}\n`);

for (const document of manifest.documents) {
  for (const theme of manifest.themes) {
    await mkdir(resolve(target, document.id, theme.id), { recursive: true });
    await cp(
      resolve(generatedDir, document.id, theme.id, `${referenceRuntime.id}.html`),
      resolve(target, document.id, theme.id, "lumis.html"),
    );
    for (const entry of others) {
      if (document.unsupported.includes(entry.id)) continue;
      await cp(
        resolve(generatedDir, document.id, theme.id, `${entry.id}.html`),
        resolve(target, document.id, theme.id, `${entry.id}.html`),
      );
    }
  }
}

console.log(
  `Published ${published.documents.length} documents × ${published.implementations.length} ` +
    `implementations × ${published.themes.length} flavours ` +
    `(${lumis.length} Lumis runtimes verified identical) to website/public/comparison-data.`,
);

// The Lumis runtimes were just proven byte-identical, so one of their outputs
// stands for all of them, its token count and hashes included. The hashes name
// each output exactly, so a timing can say which output it measured and the page
// can drop one that describes an output it no longer shows.
function publishedOutputs(document) {
  const tokens = {};
  const outputSha256 = {};
  for (const { id, label } of implementations) {
    const output = document.outputs.find(
      (entry) => entry.id === (id === "lumis" ? referenceRuntime.id : id),
    );
    if (!output) continue;
    if (!Number.isSafeInteger(output.tokens) || output.tokens <= 0) {
      throw new Error(`${label} has no token count for ${document.id}; regenerate the showcase`);
    }
    tokens[id] = output.tokens;
    outputSha256[id] = output.sha256;
  }
  return { tokens, outputSha256 };
}

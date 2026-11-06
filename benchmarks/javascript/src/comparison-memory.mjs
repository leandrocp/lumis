import { readFile, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { loadLibraries } from "./libraries.mjs";

// One fresh process per measurement: `benchmarks/scripts/comparison-memory.mjs`
// starts this once per run, so a peak never includes another library or an
// earlier run. With `baseline` it loads nothing, which is what Node alone costs.
const [implementation, documentId, fragmentPath] = process.argv.slice(2);
if (!implementation || !documentId) {
  throw new Error("usage: comparison-memory.mjs <library|baseline> <document> [fragment]");
}

const assetsDir = resolve(import.meta.dirname, "../../showcase/generated/assets");
// The gallery documents, then the ones only the timing passes highlight.
const documents = [
  ...JSON.parse(await readFile(resolve(assetsDir, "documents.json"), "utf8")),
  ...JSON.parse(await readFile(resolve(assetsDir, "timed-documents.json"), "utf8")),
];
const themes = JSON.parse(await readFile(resolve(assetsDir, "themes.json"), "utf8"));
const document = documents.find((entry) => entry.id === documentId);
if (!document) throw new Error(`no showcase document is named ${documentId}`);
// The flavour the timing pass renders, so the output is the one it timed.
const theme = themes[0];
const source = await readFile(resolve(assetsDir, document.file), "utf8");

// Setup is building the highlighter and its first highlight, so it covers
// whatever a library defers to that first call.
const started = performance.now();
let fragment;
if (implementation !== "baseline") {
  const { libraries } = await loadLibraries({
    assetsDir,
    documents: [document],
    themes: [theme],
    ids: [implementation],
  });
  const [library] = libraries;
  const output = await library.highlight(source, document, theme);
  if (output === undefined) throw new Error(`${implementation} cannot highlight ${documentId}`);
  fragment = library.fragment(output, document, theme);
}

// Read before the fragment is written, so the peak is building the highlighter
// and the one highlight, not this harness's file I/O. Node reports kibibytes.
const setupNs = Math.round((performance.now() - started) * 1e6);
const peakRssBytes = process.resourceUsage().maxRSS * 1024;
if (fragment !== undefined) {
  if (!fragmentPath) throw new Error("a fragment path is required to check the output");
  await writeFile(fragmentPath, fragment);
}
process.stdout.write(`${JSON.stringify({ peakRssBytes, setupNs })}\n`);

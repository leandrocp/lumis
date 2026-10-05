import { mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { implementationById } from "../../scripts/implementations.mjs";
import { loadLibraries } from "./libraries.mjs";
import { measureCall } from "./measure.mjs";

const benchmarksDir = resolve(import.meta.dirname, "../..");
const assetsDir = resolve(benchmarksDir, "showcase/generated/assets");
const outputPath = process.env.BENCH_OUTPUT;
const fragmentsDir = process.env.BENCH_FRAGMENTS_DIR;
if (!outputPath) throw new Error("BENCH_OUTPUT is required");
if (!fragmentsDir) throw new Error("BENCH_FRAGMENTS_DIR is required");

const documents = JSON.parse(await readFile(resolve(assetsDir, "documents.json"), "utf8"));
const themes = JSON.parse(await readFile(resolve(assetsDir, "themes.json"), "utf8"));
// A flavour changes the colours a library writes, not how much work it does to
// find them, so one flavour is timed: the first, as in the Rust bench.
const theme = themes[0];
const { libraries, dispose } = await loadLibraries({ assetsDir, documents, themes });

const results = [];
for (const document of documents) {
  const source = await readFile(resolve(assetsDir, document.file), "utf8");
  for (const library of libraries) {
    const output = await library.highlight(source, document, theme);
    if (output === undefined) continue;

    // What was timed is kept, so the timings can be checked against the published
    // pages and tied to the exact output they describe.
    const fragment = resolve(fragmentsDir, document.id, `${library.id}.html`);
    await mkdir(dirname(fragment), { recursive: true });
    await writeFile(fragment, library.fragment(output, document, theme));

    const stats = await measureCall(() => library.highlight(source, document, theme));
    results.push({ document: document.id, implementation: library.id, totalNs: stats.p50 });
    console.log(
      `${document.id} · ${implementationById(library.id).label}: ` +
        `${(stats.p50 / 1e6).toFixed(2)} ms`,
    );
  }
}
dispose();

await mkdir(dirname(outputPath), { recursive: true });
await writeFile(outputPath, `${JSON.stringify({ results }, null, 2)}\n`);
console.log(outputPath);

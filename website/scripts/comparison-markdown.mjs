// Renders public/comparison/index.md from the published comparison and its
// timing report, so the
// Markdown page states what the data says rather than what someone last wrote
// about it. Without `--write` it fails when the committed
// page is not what the data renders.
import { readFile, writeFile } from "node:fs/promises";

const markdownUrl = new URL("../public/comparison/index.md", import.meta.url);
const vercelUrl = new URL("../vercel.json", import.meta.url);
const publishedManifest = await readJson(
  new URL("../public/comparison-data/manifest.json", import.meta.url),
);
const timings = await readJson(
  new URL("../public/benchmark-data/comparison.json", import.meta.url),
  true,
);

const markdown = render(publishedManifest);
const tokens = Math.ceil(Buffer.byteLength(markdown, "utf8") / 4);

if (process.argv.includes("--write")) {
  await writeFile(markdownUrl, markdown);
  const vercel = await readFile(vercelUrl, "utf8");
  await writeFile(
    vercelUrl,
    vercel.replace(
      /("src": "\^\/comparison\/\?\$"[\s\S]*?"x-markdown-tokens": ")\d+"/u,
      (_match, prefix) => `${prefix}${tokens}"`,
    ),
  );
  console.log(`Wrote public/comparison/index.md (${tokens} tokens).`);
} else if ((await readFile(markdownUrl, "utf8")) === markdown) {
  console.log("public/comparison/index.md matches the published comparison.");
} else {
  throw new Error(
    "public/comparison/index.md does not match the published comparison; " +
      "run `node website/scripts/comparison-markdown.mjs --write`",
  );
}

function render(manifest) {
  const libraries = listOf(manifest.implementations.map((entry) => entry.label));
  return [
    "---",
    "title: Lumis - Visual output comparison",
    `description: The same source files highlighted by ${libraries}.`,
    "---",
    "",
    "# Visual output comparison",
    "",
    `The [interactive comparison](https://lumis.sh/comparison/) renders the same source files with ${libraries}, ` +
      "each through its own public API, in Catppuccin " +
      `${listOf(manifest.themes.map((theme) => theme.name.replace(/^Catppuccin /u, "")))}. ` +
      "Every Lumis runtime renders each file byte for byte identically, so one Lumis output is shown.",
    "",
    "## Documents",
    "",
    ...manifest.documents.map(documentLine),
    "",
    ...(timings ? timingSection(timings) : []),
    "The source files, rendering pipeline, themes, and generated outputs are available in the " +
      "[Lumis repository](https://github.com/leandrocp/lumis).",
    "",
    "[Lumis home](https://lumis.sh/) · [Documentation](https://docs.lumis.sh) · [Showcase](https://lumis.sh/showcase/)",
    "",
  ].join("\n");
}

function documentLine(document) {
  const injections =
    document.injections.length > 0 ? ` with ${listOf(document.injections)} injections` : "";
  const unsupported =
    document.unsupported.length > 0
      ? `; not supported by ${listOf(document.unsupported.map((id) => labelOf(id)))}`
      : "";
  return (
    `- [${document.label}](${document.source}): ${document.languageLabel}${injections}, ` +
    `${document.lines.toLocaleString("en")} lines${unsupported}`
  );
}

function labelOf(id) {
  return publishedManifest.implementations.find((entry) => entry.id === id)?.label ?? id;
}

function listOf(items) {
  if (items.length < 2) return items.join("");
  return `${items.slice(0, -1).join(", ")}${items.length > 2 ? "," : ""} and ${items.at(-1)}`;
}

// Rows are libraries and columns are the report's small and large files, as on
// the page.
function timingSection(report) {
  const { documents } = report;
  const table = (title, note, format, value) => [
    `### ${title}`,
    "",
    note,
    "",
    `| Library | ${documents.map((document) => `${document.size}: ${document.label} (${formatBytes(document.bytes)})`).join(" | ")} |`,
    `| --- | ${documents.map(() => "---:").join(" | ")} |`,
    ...report.implementations.map(
      ({ id, label }) =>
        `| ${label} | ${documents
          .map((document) => {
            const result = document.results.find((entry) => entry.id === id);
            return result ? format(value(result)) : "—";
          })
          .join(" | ")} |`,
    ),
    "",
  ];
  return [
    "## Speed and output",
    "",
    `Measured ${report.measuredAt.slice(0, 10)} on ${report.system.cpu} ` +
      `(${report.system.cores} cores, ${report.system.os}), Node.js ${report.runtimes.node}, ` +
      `${report.runtimes.rust}. Each figure is a median. ` +
      documents
        .map(
          (document) =>
            `The ${document.size.toLowerCase()} file is [${document.label}](${document.source}), ` +
            `${document.languageLabel}, ${document.lines.toLocaleString("en")} lines.`,
        )
        .join(" "),
    "",
    ...table(
      "Highlight",
      `Time: ${report.timingBoundary}.`,
      formatDuration,
      (result) => result.totalNs,
    ),
    ...table(
      "Setup + first highlight",
      `Time: ${report.setupBoundary}.`,
      formatDuration,
      (result) => result.setupNs,
    ),
    ...table(
      "Peak RSS",
      "The most memory that fresh process held, its runtime included.",
      formatBytes,
      (result) => result.peakRssBytes,
    ),
    ...table(
      "Tokens",
      "Spans the highlighter gave a colour to: how finely the file was resolved, not a score.",
      (value) => value.toLocaleString("en"),
      (result) => result.tokens,
    ),
  ];
}

function formatDuration(ns) {
  if (ns >= 1e9) return `${(ns / 1e9).toFixed(2)} s`;
  if (ns >= 1e6) return `${(ns / 1e6).toFixed(ns >= 1e8 ? 0 : 1)} ms`;
  return `${(ns / 1e3).toFixed(0)} µs`;
}

function formatBytes(bytes) {
  if (bytes >= 2 ** 20) return `${(bytes / 2 ** 20).toFixed(1)} MiB`;
  return `${(bytes / 2 ** 10).toFixed(0)} KiB`;
}

async function readJson(url, optional = false) {
  try {
    return JSON.parse(await readFile(url, "utf8"));
  } catch (error) {
    if (optional && error.code === "ENOENT") return;
    throw error;
  }
}

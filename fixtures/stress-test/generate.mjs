import { createHash } from "node:crypto";
import { mkdir, readFile, rename, writeFile } from "node:fs/promises";
import { relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const repoDir = fileURLToPath(new URL("../../", import.meta.url));
const corpusPath = fileURLToPath(new URL("corpus.json", import.meta.url));

function parseArguments(argv) {
  const options = {
    cases: [],
    output: resolve(repoDir, "target/stress-test/corpus"),
    profile: "all",
    scale: 1,
  };

  for (let index = 0; index < argv.length; index += 1) {
    const argument = argv[index];
    if (argument === "--case") options.cases.push(argv[++index]);
    else if (argument === "--output") options.output = resolve(argv[++index]);
    else if (argument === "--profile") options.profile = argv[++index];
    else if (argument === "--scale") options.scale = Number(argv[++index]);
    else throw new Error(`unknown argument: ${argument}`);
  }

  if (!(options.scale > 0 && options.scale <= 1)) {
    throw new Error("--scale must be greater than zero and at most one");
  }
  return options;
}

function pattern(language) {
  const patterns = {
    cpp: "template<class T> auto value(T x){return x+x;} ",
    csv: "1,synthetic,stress,value,fixture",
    css: ".value{color:#123456;margin:0;padding:0}",
    elixir: "fn x -> %{id: x, values: [x, x], ok: true} end; ",
    erlang: "fun(X) -> {value, X, [X, X], true} end, ",
    gleam: "fn(x) { #(x, [x, x], True) } ",
    javascript: "(()=>({items:[1,2,3],ok:true}))();",
    json: "0,",
  };
  return patterns[language];
}

function delimiters(language) {
  return language === "css" || language === "cpp" ? ["{", "}"] : ["[", "]"];
}

function fill(value, bytes) {
  if (bytes === 0) return "";
  const copies = Math.floor(bytes / value.length);
  return value.repeat(copies) + value.slice(0, bytes % value.length);
}

function line(language, bytes, depth) {
  const [open, close] = delimiters(language);
  const nesting = Math.min(depth, Math.floor(bytes / 2));
  const prefix = open.repeat(nesting);
  const suffix = close.repeat(nesting);
  return prefix + fill(pattern(language), bytes - prefix.length - suffix.length) + suffix;
}

function shapedSource(testCase, scale) {
  const { target } = testCase;
  const lines = Math.max(1, Math.round(target.lines * scale));
  const bytes = Math.max(lines * 2 - 1, Math.round(target.bytes * scale));
  const contentBytes = bytes - lines + 1;
  const average = Math.ceil(contentBytes / lines);
  const maxLineBytes = Math.min(
    Math.max(Math.round(target.maxLineBytes * scale), average),
    contentBytes - lines + 1,
  );
  const first = line(testCase.language, maxLineBytes, testCase.generator.depth);
  if (lines === 1) return first;

  const remainingLines = lines - 1;
  const remainingBytes = contentBytes - maxLineBytes;
  const shortBytes = Math.floor(remainingBytes / remainingLines);
  const longLines = remainingBytes % remainingLines;
  const shortLines = remainingLines - longLines;
  return (
    first +
    `\n${line(testCase.language, shortBytes + 1, 0)}`.repeat(longLines) +
    `\n${line(testCase.language, shortBytes, 0)}`.repeat(shortLines)
  );
}

function generateSource(testCase, scale) {
  if (testCase.generator.kind === "deep") {
    return testCase.generator.byte.repeat(Math.max(1, Math.round(testCase.target.bytes * scale)));
  }
  if (testCase.generator.kind === "shaped") return shapedSource(testCase, scale);
  throw new Error(`unknown generator for ${testCase.id}: ${testCase.generator.kind}`);
}

function metrics(source) {
  let lines = 1;
  let lineBytes = 0;
  let maxLineBytes = 0;
  for (let index = 0; index < source.length; index += 1) {
    if (source.charCodeAt(index) === 10) {
      lines += 1;
      maxLineBytes = Math.max(maxLineBytes, lineBytes);
      lineBytes = 0;
    } else {
      lineBytes += 1;
    }
  }
  return {
    bytes: Buffer.byteLength(source),
    lines,
    maxLineBytes: Math.max(maxLineBytes, lineBytes),
  };
}

function selectCases(corpus, options) {
  const known = new Set(corpus.cases.map(({ id }) => id));
  const unknown = options.cases.filter((id) => !known.has(id));
  if (unknown.length > 0) throw new Error(`unknown corpus cases: ${unknown.join(", ")}`);

  const selected = corpus.cases.filter(
    (testCase) =>
      (options.cases.length === 0 || options.cases.includes(testCase.id)) &&
      (options.profile === "all" || testCase.profile === options.profile),
  );
  if (selected.length === 0) {
    throw new Error(`no corpus cases matched profile ${JSON.stringify(options.profile)}`);
  }
  return selected;
}

function validateCorpus(corpus) {
  if (corpus.schemaVersion !== 1 || !Array.isArray(corpus.cases) || corpus.cases.length === 0) {
    throw new Error("corpus.json must use schema version 1 and contain cases");
  }

  const caseIds = new Set();
  const origins = new Set();
  for (const testCase of corpus.cases) {
    if (caseIds.has(testCase.id)) throw new Error(`duplicate corpus case: ${testCase.id}`);
    caseIds.add(testCase.id);
    if (!["deep", "shaped"].includes(testCase.generator?.kind)) {
      throw new Error(`invalid generator for ${testCase.id}`);
    }
    for (const field of ["bytes", "lines", "maxLineBytes"]) {
      if (!(testCase.target?.[field] > 0)) {
        throw new Error(`invalid ${field} target for ${testCase.id}`);
      }
    }
    if (!(testCase.target?.structuralDepth >= 0)) {
      throw new Error(`invalid structuralDepth target for ${testCase.id}`);
    }
    if (!Array.isArray(testCase.origins) || testCase.origins.length === 0) {
      throw new Error(`missing origins for ${testCase.id}`);
    }
    for (const origin of testCase.origins) {
      const key = `${origin.source}\0${origin.package}\0${origin.version}\0${origin.path}`;
      if (origins.has(key)) throw new Error(`duplicate corpus origin: ${origin.path}`);
      origins.add(key);
      if (
        origin.source !== "hex_package" ||
        !/^[0-9a-f]{64}$/.test(origin.sha256) ||
        !(origin.metrics?.bytes > 0) ||
        !(origin.metrics?.lines > 0) ||
        !(origin.metrics?.maxLineBytes > 0) ||
        !(origin.metrics?.structuralDepth >= 0)
      ) {
        throw new Error(`invalid provenance for ${testCase.id}: ${origin.path}`);
      }
    }
  }
}

async function writeJson(path, value) {
  const temporary = `${path}.tmp`;
  await writeFile(temporary, `${JSON.stringify(value, null, 2)}\n`);
  await rename(temporary, path);
}

const options = parseArguments(process.argv.slice(2));
const corpus = JSON.parse(await readFile(corpusPath, "utf8"));
validateCorpus(corpus);
const selected = selectCases(corpus, options);
await mkdir(options.output, { recursive: true });

const cases = [];
for (const testCase of selected) {
  const source = generateSource(testCase, options.scale);
  const generated = metrics(source);
  const sourcePath = resolve(options.output, `${testCase.id}.${testCase.extension}`);
  await writeFile(sourcePath, source);
  cases.push({
    ...testCase,
    generated,
    generatedPath: relative(repoDir, sourcePath),
    scale: options.scale,
    sourceSha256: createHash("sha256").update(source).digest("hex"),
  });
  process.stdout.write(`${testCase.id}: ${generated.bytes} bytes\n`);
}

const manifestPath = resolve(options.output, "manifest.json");
await writeJson(manifestPath, {
  schemaVersion: corpus.schemaVersion,
  generatedAt: new Date().toISOString(),
  scale: options.scale,
  discovery: corpus.discovery,
  cases,
});
process.stdout.write(`${relative(repoDir, manifestPath)}\n`);

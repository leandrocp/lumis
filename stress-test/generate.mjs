import { createHash } from "node:crypto";
import { mkdir, readFile, rename, writeFile } from "node:fs/promises";
import { relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { numberArgument, pathArgument, stringArgument } from "./report.mjs";

const repoDir = fileURLToPath(new URL("../", import.meta.url));
const corpusPath = fileURLToPath(new URL("corpus.json", import.meta.url));

const VALUES = new Map([
  ["--case", (options, raw) => options.cases.push(stringArgument("--case", raw))],
  ["--output", (options, raw) => (options.output = pathArgument("--output", raw))],
  ["--profile", (options, raw) => (options.profile = stringArgument("--profile", raw))],
  ["--scale", (options, raw) => (options.scale = numberArgument("--scale", raw, Number.MIN_VALUE))],
]);

function parseArguments(argv) {
  const options = {
    cases: [],
    output: resolve(repoDir, "target/stress-test/corpus"),
    profile: "all",
    scale: 1,
  };

  for (let index = 0; index < argv.length; index += 1) {
    const argument = argv[index];
    const withValue = VALUES.get(argument);
    if (!withValue) throw new Error(`unknown argument: ${argument}`);
    withValue(options, argv[++index]);
  }

  if (options.scale > 1) throw new Error("--scale must be greater than zero and at most one");
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

function metrics(source, language) {
  const [open, close] = delimiters(language);
  const openCode = open.charCodeAt(0);
  const closeCode = close.charCodeAt(0);
  let lines = 1;
  let lineBytes = 0;
  let maxLineBytes = 0;
  let depth = 0;
  let structuralDepth = 0;

  for (let index = 0; index < source.length; index += 1) {
    const code = source.charCodeAt(index);
    if (code === 10) {
      lines += 1;
      maxLineBytes = Math.max(maxLineBytes, lineBytes);
      lineBytes = 0;
      continue;
    }
    lineBytes += 1;
    if (code === openCode) {
      depth += 1;
      structuralDepth = Math.max(structuralDepth, depth);
    } else if (code === closeCode) {
      depth = Math.max(0, depth - 1);
    }
  }

  return {
    bytes: Buffer.byteLength(source),
    lines,
    maxLineBytes: Math.max(maxLineBytes, lineBytes),
    structuralDepth,
  };
}

/**
 * A case whose generated shape drifts from its target still hashes consistently,
 * so every runtime would verify it and report green on an input that no longer
 * exercises the regression. Only a full-size corpus can be checked against the
 * recorded targets; `--scale` deliberately produces a smaller shape.
 */
function verifyGenerated(testCase, generated, scale) {
  if (scale !== 1) return;

  for (const field of ["bytes", "lines", "maxLineBytes"]) {
    if (generated[field] !== testCase.target[field]) {
      throw new Error(
        `${testCase.id}: generated ${field} ${generated[field]} does not match ` +
          `target ${testCase.target[field]}`,
      );
    }
  }
  if (generated.structuralDepth < testCase.target.structuralDepth) {
    throw new Error(
      `${testCase.id}: generated structural depth ${generated.structuralDepth} is below ` +
        `target ${testCase.target.structuralDepth}`,
    );
  }
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

function validateLanguage(testCase) {
  if (!pattern(testCase.language)) {
    throw new Error(`unknown corpus language for ${testCase.id}: ${testCase.language}`);
  }
  if (typeof testCase.extension !== "string" || testCase.extension === "") {
    throw new Error(`missing extension for ${testCase.id}`);
  }
}

function validateTarget(testCase) {
  for (const field of ["bytes", "lines", "maxLineBytes"]) {
    if (!(testCase.target?.[field] > 0)) {
      throw new Error(`invalid ${field} target for ${testCase.id}`);
    }
  }
  if (!(testCase.target?.structuralDepth >= 0)) {
    throw new Error(`invalid structuralDepth target for ${testCase.id}`);
  }
}

function validateDeepGenerator(generator, id) {
  if (typeof generator.byte !== "string" || generator.byte.length === 0) {
    throw new Error(`invalid generator byte for ${id}`);
  }
}

function validateGenerator(testCase) {
  const { generator, id } = testCase;
  const kind = generator?.kind;
  if (kind === "deep") {
    validateDeepGenerator(generator, id);
    return;
  }
  if (kind !== "shaped") throw new Error(`invalid generator for ${id}`);
  if (!Number.isInteger(generator.depth) || generator.depth < 0) {
    throw new Error(`invalid generator depth for ${id}`);
  }
  if (generator.depth !== testCase.target.structuralDepth) {
    throw new Error(
      `${id}: generator depth ${generator.depth} disagrees with structuralDepth ` +
        `target ${testCase.target.structuralDepth}`,
    );
  }
}

function validOriginMetrics(measured) {
  if (!measured) return false;
  return (
    measured.bytes > 0 &&
    measured.lines > 0 &&
    measured.maxLineBytes > 0 &&
    measured.structuralDepth >= 0
  );
}

function validProvenance(origin) {
  return (
    origin.source === "hex_package" &&
    /^[0-9a-f]{64}$/u.test(origin.sha256) &&
    validOriginMetrics(origin.metrics)
  );
}

function validateOrigins(testCase, seen) {
  if (!Array.isArray(testCase.origins) || testCase.origins.length === 0) {
    throw new Error(`missing origins for ${testCase.id}`);
  }
  for (const origin of testCase.origins) {
    const key = `${origin.source}\0${origin.package}\0${origin.version}\0${origin.path}`;
    if (seen.has(key)) throw new Error(`duplicate corpus origin: ${origin.path}`);
    seen.add(key);
    if (!validProvenance(origin)) {
      throw new Error(`invalid provenance for ${testCase.id}: ${origin.path}`);
    }
  }
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
    validateLanguage(testCase);
    validateTarget(testCase);
    validateGenerator(testCase);
    validateOrigins(testCase, origins);
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
  const generated = metrics(source, testCase.language);
  verifyGenerated(testCase, generated, options.scale);
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

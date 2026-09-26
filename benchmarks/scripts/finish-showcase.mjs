#!/usr/bin/env node

import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { basename, dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { showcaseImplementations } from "./implementations.mjs";

const benchmarksDir = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const repoDir = resolve(benchmarksDir, "..");
const generatedDir = resolve(benchmarksDir, "showcase/generated");
const assetsDir = resolve(generatedDir, "assets");
const documents = JSON.parse(await readFile(resolve(assetsDir, "documents.json"), "utf8"));
const themes = JSON.parse(await readFile(resolve(assetsDir, "themes.json"), "utf8"));

// Catppuccin gives every flavour the same palette names, so an expectation below
// names a colour rather than repeating a hex per flavour, and the two flavours
// are held to the same choice for a scope.
const palettes = {
  latte: {
    flamingo: "#dd7878",
    pink: "#ea76cb",
    mauve: "#8839ef",
    maroon: "#e64553",
    peach: "#fe640b",
    yellow: "#df8e1d",
    green: "#40a02b",
    teal: "#179299",
    blue: "#1e66f5",
    lavender: "#7287fd",
    text: "#4c4f69",
    overlay2: "#7c7f93",
  },
  frappe: {
    flamingo: "#eebebe",
    pink: "#f4b8e4",
    mauve: "#ca9ee6",
    maroon: "#ea999c",
    peach: "#ef9f76",
    yellow: "#e5c890",
    green: "#a6d189",
    teal: "#81c8be",
    blue: "#8caaee",
    lavender: "#babbf1",
    text: "#c6d0f5",
    overlay2: "#949cbb",
  },
};

// A theme is only worth comparing if it actually reached the scopes a language
// turns on, so every document names text that must have been coloured, and the
// colour it must have been given. A document with no entry here fails rather
// than being silently exempt from the check.
//
// The text has to be a whole span, and same-scope spans are coalesced, so it is
// every adjacent token the scope covers rather than the one that names it: an
// Elixir sigil is `~w(` and not the `~` the query captures.
const scopeExpectations = {
  webgpu: [
    ["HTML tag delimiter", "&lt;", "teal"],
    ["HTML tag", "title", "blue"],
    ["HTML attribute", "lang", "yellow"],
    ["HTML title text", "three.js webgpu - compute reduction", "text"],
    ["injected CSS property", "background-color", "blue"],
    ["injected JSON key", "&quot;imports&quot;", "lavender"],
    ["injected JavaScript keyword", "import", "mauve"],
  ],
  ripgrep: [
    ["Rust keyword", "pub", "mauve"],
    ["Rust type", "SearcherBuilder", "yellow"],
    ["Rust builtin type", "usize", "mauve"],
    ["Rust function", "new", "blue"],
    ["Rust field", "config", "lavender"],
    ["Rust attribute", "derive", "pink"],
    ["Rust comment", "// always required.", "overlay2"],
  ],
  livebook: [
    ["Elixir keyword", "end", "mauve"],
    ["Elixir module", "Phoenix.Component", "yellow"],
    ["Elixir atom", ":string", "flamingo"],
    ["Elixir sigil", "~w(", "pink"],
    ["injected HEEx tag", "div", "blue"],
    ["injected HEEx attribute", "class", "yellow"],
  ],
  go: [
    ["Go keyword", "return", "mauve"],
    ["Go builtin type", "string", "mauve"],
    ["Go type", "encodeState", "yellow"],
    ["Go function", "WriteString", "blue"],
    ["Go parameter", "opts", "maroon"],
    ["Go comment", "// an error.", "overlay2"],
  ],
  readme: [
    ["Markdown heading", "## Features", "peach"],
    ["injected HTML tag", "img", "blue"],
    ["injected Rust type", "HtmlInlineBuilder", "yellow"],
    ["injected Elixir atom", ":html_inline", "flamingo"],
    ["injected JavaScript keyword", "await", "mauve"],
  ],
  shadcn: [
    ["TSX keyword", "const", "mauve"],
    ["TSX type", "ComponentProps", "yellow"],
    ["TSX builtin type", "boolean", "mauve"],
    ["JSX attribute", "className", "yellow"],
    ["TSX constant", "SIDEBAR_WIDTH", "peach"],
    ["TSX string", "&quot;button&quot;", "green"],
  ],
};

const commandEnv = {
  ...process.env,
  XDG_CACHE_HOME: resolve(repoDir, "target/benchmarks/cli/xdg-cache"),
  LUMIS_DATA_DIR: resolve(repoDir, "target/benchmarks/cli/data"),
  LUMIS_CONFIG: resolve(repoDir, "target/benchmarks/cli/missing-config.toml"),
};

const lumisBinary = resolve(
  repoDir,
  "target/benchmarks/rust-target/release",
  process.platform === "win32" ? "lumis.exe" : "lumis",
);

// Each implementation renders with its own Catppuccin port, because each
// consumes a different theme format. Some colour differences are therefore about
// the theme rather than the parse, and the page has to say so rather than let a
// reader assume otherwise.
// The version that actually rendered, not the range that selected it. The
// comparison libraries float, so a specifier here would publish "shiki latest"
// and leave a reader unable to tell what was measured.
const installedVersion = async (name) => {
  const manifest = createRequire(import.meta.url).resolve(`${name}/package.json`, {
    paths: [resolve(benchmarksDir, "javascript")],
  });
  return JSON.parse(await readFile(manifest, "utf8")).version;
};
const benchmarkCargo = await readFile(resolve(benchmarksDir, "rust/Cargo.toml"), "utf8");
const crateVersion = (name) =>
  benchmarkCargo.match(new RegExp(`^${name} = "([^"]+)"`, "m"))?.[1] ?? "unknown";
const lumisVersion = JSON.parse(
  await readFile(resolve(repoDir, "packages/javascript/lumis/package.json"), "utf8"),
).version;

const provenance = {
  syntect: {
    version: `syntect ${crateVersion("syntect")} with two-face ${crateVersion("two-face")}`,
    theme: "catppuccin/bat tmTheme files, pinned by SHA-256",
  },
  shiki: {
    version: `shiki ${await installedVersion("shiki")}`,
    theme: "Shiki's bundled catppuccin themes",
  },
  "highlight-js": {
    version: `highlight.js ${await installedVersion("highlight.js")}`,
    theme: `@catppuccin/highlightjs ${await installedVersion("@catppuccin/highlightjs")} stylesheets`,
  },
};

const manifest = {
  schemaVersion: 6,
  themes: themes.map(({ id, name, appearance, source }) => ({ id, name, appearance, source })),
  implementations: showcaseImplementations.map(({ id, label }) => ({
    id,
    label,
    version: provenance[id]?.version ?? `Lumis ${lumisVersion}`,
    theme: provenance[id]?.theme ?? "Neovim Catppuccin colorschemes, extracted by this repository",
  })),
  documents: [],
};

for (const document of documents) {
  const expectations = scopeExpectations[document.id];
  if (!expectations) {
    throw new Error(`showcase document ${document.id} has no scope expectations`);
  }

  const sourcePath = resolve(assetsDir, document.file);
  const source = await readFile(sourcePath, "utf8");
  const sourceBytes = Buffer.byteLength(source);
  const sourceLines = source.split("\n").length;
  const unsupported = new Set(document.unsupported);
  const outputs = new Map();

  for (const theme of themes) {
    const fragmentsDir = resolve(generatedDir, "fragments", document.id, theme.id);
    await mkdir(fragmentsDir, { recursive: true });
    await mkdir(resolve(generatedDir, document.id, theme.id), { recursive: true });

    const lumisCli = run(lumisBinary, [
      "--data-dir",
      commandEnv.LUMIS_DATA_DIR,
      "highlight",
      "--language",
      document.language,
      "--formatter",
      "html-inline",
      "--theme",
      theme.lumis,
      sourcePath,
    ]);
    validateHtml(lumisCli, sourceBytes, "Lumis CLI");
    await writeFile(resolve(fragmentsDir, "lumis-cli.html"), lumisCli);

    const lumisFragments = [];

    for (const { id, label } of showcaseImplementations) {
      // An implementation that renders nothing has to have said so: silence is only
      // acceptable where it was declared, and a declaration that stops being true
      // fails too, so the list can only shrink.
      const fragment = await readFile(resolve(fragmentsDir, `${id}.html`), "utf8").catch(
        () => undefined,
      );
      if (fragment === undefined) {
        if (!unsupported.has(id)) {
          throw new Error(`${label} produced no ${theme.name} output for ${document.id}`);
        }
        continue;
      }
      if (unsupported.has(id)) {
        throw new Error(
          `${label} is declared unsupported for ${document.id} but produced output; remove it from the list`,
        );
      }

      validateHtml(fragment, sourceBytes, label);
      if (id.startsWith("lumis-")) {
        validateLumisScopes(fragment, label, document, expectations, theme);
        lumisFragments.push({ label, fragment });
      }
      await writeFile(
        resolve(generatedDir, document.id, theme.id, `${id}.html`),
        pageHtml({ fragment, label, theme }),
      );

      // A flavour changes the colours a document is given, never how finely it was
      // resolved, so one token count covers both and a disagreement is a mix-up in
      // the pipeline rather than a second number to publish.
      const tokens = countTokens(fragment, label);
      const recorded = outputs.get(id);
      if (recorded === undefined) {
        outputs.set(id, { id, tokens, outputBytes: { [theme.id]: Buffer.byteLength(fragment) } });
      } else if (recorded.tokens === tokens) {
        recorded.outputBytes[theme.id] = Buffer.byteLength(fragment);
      } else {
        throw new Error(
          `${label} found ${tokens} tokens in ${document.id} with ${theme.name} but ` +
            `${recorded.tokens} with ${themes[0].name}`,
        );
      }
    }

    validateLumisAgreement(lumisFragments, document, theme);
  }

  manifest.documents.push({
    id: document.id,
    label: document.label,
    language: document.language,
    languageLabel: document.languageLabel,
    source: document.source,
    sha256: createHash("sha256").update(source).digest("hex"),
    bytes: sourceBytes,
    lines: sourceLines,
    injections: document.injections,
    unsupported: document.unsupported,
    outputs: [...outputs.values()],
  });
}

const serializedManifest = JSON.stringify(manifest, null, 2);
await Promise.all([
  writeFile(resolve(generatedDir, "manifest.json"), `${serializedManifest}\n`),
  writeFile(
    resolve(generatedDir, "manifest.js"),
    `globalThis.LUMIS_BENCHMARK_SHOWCASE = ${serializedManifest};\n`,
  ),
]);
console.log(
  `Generated ${showcaseImplementations.length} visual comparisons of ${documents.length} documents ` +
    `in ${themes.map((theme) => theme.name).join(" and ")} ` +
    `(${manifest.documents.map((d) => `${d.lines.toLocaleString()} lines`).join(", ")}).`,
);

function run(command, args) {
  const result = spawnSync(command, args, {
    cwd: repoDir,
    env: commandEnv,
    encoding: "utf8",
    maxBuffer: 128 * 1024 * 1024,
  });
  if (result.error) throw result.error;
  if (result.status !== 0) {
    throw new Error(`${basename(command)} failed:\n${result.stderr}`);
  }
  return result.stdout;
}

// A token is a span the highlighter gave a colour to, whether that colour arrives
// inline or through one of highlight.js's `hljs-` classes. Spans that carry
// neither are structure rather than a token: Shiki wraps every line in
// `<span class="line">`, so counting all spans would credit it one per line.
function countTokens(fragment, implementation) {
  let tokens = 0;
  for (const tag of fragment.match(/<span\b[^>]*>/gi) ?? []) {
    if (
      /style=(?:"[^"]*|'[^']*)\bcolor\s*:/i.test(tag) ||
      /class=(?:"[^"]*|'[^']*)\bhljs-/i.test(tag)
    ) {
      tokens += 1;
    }
  }
  if (tokens === 0) throw new Error(`${implementation} produced no coloured tokens`);
  return tokens;
}

function validateHtml(output, sourceBytes, implementation) {
  if (
    Buffer.byteLength(output) <= sourceBytes ||
    !output.includes("<pre") ||
    !output.includes("<span")
  ) {
    throw new Error(`${implementation} did not produce highlighted HTML`);
  }
}

// Bold and italic are left out on purpose: the question is whether the theme
// reached this scope at all, and Catppuccin styles several of these scopes
// differently per flavour while agreeing on the colour.
function validateLumisScopes(output, implementation, document, expectations, theme) {
  const palette = palettes[theme.id];
  if (!palette) throw new Error(`no palette is recorded for ${theme.name}`);

  for (const [scope, text, colour] of expectations) {
    const expected = palette[colour];
    if (!expected) throw new Error(`${theme.name} has no ${colour} in its palette`);

    const found = spanColours(output, text);
    if (found.has(expected)) continue;

    throw new Error(
      found.size === 0
        ? `${implementation} coloured no ${scope} in ${document.id}: "${text}" is not a span`
        : `${implementation} coloured ${scope} in ${document.id} ${[...found].join(", ")} ` +
            `rather than ${theme.name} ${colour} (${expected})`,
    );
  }
}

// Every colour the output gives to spans whose text is exactly `text`.
function spanColours(output, text) {
  const found = new Set();

  for (const [, style, span] of output.matchAll(/<span style="([^"]*)">([^<]*)<\/span>/g)) {
    if (span === text) found.add(style.match(/color:\s*(#[0-9a-f]{6})/i)?.[1]?.toLowerCase());
  }

  return found;
}

// Every Lumis runtime has to render this document identically. The comparison is
// the point of the showcase, so a divergence names the line rather than only
// reporting that two hashes differ.
function validateLumisAgreement(fragments, document, theme) {
  const [reference, ...rest] = fragments;
  if (!reference) {
    throw new Error(`the showcase produced no Lumis output for ${document.id} with ${theme.name}`);
  }
  const referenceLines = reference.fragment.split("\n");
  for (const { label, fragment } of rest) {
    if (fragment === reference.fragment) continue;
    const lines = fragment.split("\n");
    const differing = lines.findIndex((line, index) => line !== referenceLines[index]);
    const at = differing === -1 ? Math.min(lines.length, referenceLines.length) : differing;
    throw new Error(
      `${label} does not match ${reference.label} on line ${at + 1} of ${document.id} ` +
        `with ${theme.name}\n` +
        `  ${label}: ${lines[at] ?? "<no line>"}\n` +
        `  ${reference.label}: ${referenceLines[at] ?? "<no line>"}`,
    );
  }
}

function pageHtml({ fragment, label, theme }) {
  return `<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>${escapeHtml(label)} · ${escapeHtml(theme.name)} · Lumis visual comparison</title>
  <meta name="color-scheme" content="${theme.appearance}">
  <style>
    * { box-sizing: border-box; }
    html { background: ${theme.chrome.background}; color: ${theme.chrome.foreground}; font: 14px/1.5 Inter, ui-sans-serif, system-ui, sans-serif; }
    body { margin: 0; }
    main { padding: 16px; min-width: max-content; }
    pre { margin: 0 !important; padding: 20px !important; border: 1px solid ${theme.chrome.border}; border-radius: 10px; overflow: visible !important; font: 13px/1.55 "SFMono-Regular", Consolas, "Liberation Mono", monospace !important; tab-size: 4; }
    /* highlight.js themes paint the panel on the code element and pad it, and
       @catppuccin/highlightjs paints it without making that element a block, so
       one panel would be ragged where the other three are solid. */
    pre code.hljs { display: block; padding: 0 !important; }
  </style>
</head>
<body>
  <main>${fragment}</main>
  <script>
    (function () {
      var at = location.hash.match(/^#at=(\\d+),(\\d+)$/);
      if (at) window.scrollTo(Number(at[1]), Number(at[2]));
      addEventListener("scroll", function () {
        parent.postMessage({ lumisShowcaseScroll: { x: scrollX, y: scrollY } }, "*");
      }, { passive: true });
    })();
  </script>
</body>
</html>
`;
}

function escapeHtml(value) {
  return value
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");
}

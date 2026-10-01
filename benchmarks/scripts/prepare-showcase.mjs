#!/usr/bin/env node

import { createHash } from "node:crypto";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const benchmarksDir = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const generatedDir = resolve(benchmarksDir, "showcase/generated");
const assetsDir = resolve(generatedDir, "assets");

// One list, read by the Rust, JavaScript, Elixir and CLI producers alike, so the
// set of documents cannot drift across four languages. `load` is what a runtime
// that cannot fetch inside a synchronous walk has to hold before it starts.
const documents = [
  {
    id: "webgpu",
    label: "three.js WebGPU compute reduce",
    language: "html",
    languageLabel: "HTML",
    file: "webgpu_compute_reduce.html",
    sha256: "e1b31d91c25e9103931d7e830b9dfb9e075d97c175623e3e44fb3dc3685067af",
    source:
      "https://github.com/mrdoob/three.js/blob/6365c1a0af6a32ed45f99712197555fee2f4b24a/examples/webgpu_compute_reduce.html",
    injections: ["CSS", "JSON", "JavaScript"],
    load: ["html", "comment", "css", "json", "javascript"],
    unsupported: [],
  },
  {
    id: "ripgrep",
    label: "ripgrep searcher",
    language: "rust",
    languageLabel: "Rust",
    file: "ripgrep_searcher.rs",
    sha256: "b1ba96daf77e0298c70bc2b381d2b50b358b632c6dfa954e5a0925f0ace32e27",
    source:
      "https://github.com/BurntSushi/ripgrep/blob/4649aa9700619f94cf9c66876e9549d83420e16c/crates/searcher/src/searcher/mod.rs",
    injections: ["Comment"],
    load: ["rust", "comment"],
    unsupported: ["tanstack-highlight"],
  },
  {
    id: "livebook",
    label: "Livebook core components",
    language: "elixir",
    languageLabel: "Elixir",
    file: "livebook_core_components.ex",
    sha256: "d5cd474afb5f8f87ed135a2d704f772cfd08bb829a708ba7b5f322b5ea77a69a",
    source:
      "https://github.com/livebook-dev/livebook/blob/5133601d9678fb8ef0c87484e5d52eff193813e5/lib/livebook_web/components/core_components.ex",
    injections: ["HEEx", "Markdown", "Comment"],
    load: ["elixir", "heex", "comment", "markdown", "markdown_inline"],
    unsupported: ["tanstack-highlight"],
  },
  {
    id: "go",
    label: "Go encoding/json",
    language: "go",
    languageLabel: "Go",
    file: "go_json_encode.go",
    sha256: "c711a284c0fb68148f1e72137b034630a83f03d2f73bc4e0f5d885f786ef0a1a",
    source:
      "https://github.com/golang/go/blob/3901409b5d0fb7c85a3e6730a59943cc93b2835c/src/encoding/json/encode.go",
    injections: ["Comment"],
    load: ["go", "comment"],
    unsupported: [],
  },
  {
    id: "readme",
    label: "Lumis README",
    language: "markdown",
    languageLabel: "Markdown",
    file: "lumis_readme.md",
    sha256: "24808cd3b6812e243a5a37319f357baacf7d06c54f389b6aeea1f4406904f340",
    source: "https://github.com/leandrocp/lumis/blob/main/README.md",
    injections: ["Bash", "Elixir", "Java", "JavaScript", "Rust"],
    // Every fence names its own language, and a browser cannot fetch a parser
    // inside a synchronous walk, so each one has to be held before the walk.
    load: [
      "markdown",
      "markdown_inline",
      "comment",
      "bash",
      "elixir",
      "java",
      "javascript",
      "rust",
    ],
    unsupported: [],
  },
  {
    id: "shadcn",
    label: "shadcn/ui sidebar",
    language: "tsx",
    languageLabel: "TSX",
    file: "shadcn_sidebar.tsx",
    sha256: "78dd9a5c50628d7bc8d5c637fdbfe613e49f4174fd3d525c9cc0be2152e67994",
    source:
      "https://github.com/shadcn-ui/ui/blob/607e8a9717fe6ff0d374ba74c651012f9c052534/apps/v4/registry/new-york-v4/ui/sidebar.tsx",
    injections: ["Comment"],
    load: ["tsx", "comment"],
    unsupported: [],
  },
];

// The website renders Catppuccin Latte in light and Frappé in dark, and the
// comparison is a page of pre-rendered files that cannot follow the reader the
// way a live highlighter does. Rendering both is what lets it try: the page
// picks a flavour from `prefers-color-scheme` and loads that copy.
const catppuccinBat = "6810349b28055dce54076712fc05fc68da4b8ec0";
const themes = [
  {
    id: "latte",
    name: "Catppuccin Latte",
    appearance: "light",
    lumis: "catppuccin_latte",
    shiki: "catppuccin-latte",
    highlightJs: "@catppuccin/highlightjs/css/catppuccin-latte.css",
    tmTheme: "catppuccin-latte.tmTheme",
    tmThemeUrl: `https://raw.githubusercontent.com/catppuccin/bat/${catppuccinBat}/themes/Catppuccin%20Latte.tmTheme`,
    tmThemeSha256: "a2ddb65bfcf7328802ee4770d1e34ef4093a20fd5c300be3138c99f8a45ed5cb",
    source: `https://github.com/catppuccin/bat/blob/${catppuccinBat}/themes/Catppuccin%20Latte.tmTheme`,
    // Every flavour names its surfaces the same way, so the page around the
    // output is `mantle`, `text` and `surface1` in both.
    chrome: { background: "#e6e9ef", foreground: "#4c4f69", border: "#bcc0cc" },
  },
  {
    id: "frappe",
    name: "Catppuccin Frappé",
    appearance: "dark",
    lumis: "catppuccin_frappe",
    shiki: "catppuccin-frappe",
    highlightJs: "@catppuccin/highlightjs/css/catppuccin-frappe.css",
    tmTheme: "catppuccin-frappe.tmTheme",
    tmThemeUrl: `https://raw.githubusercontent.com/catppuccin/bat/${catppuccinBat}/themes/Catppuccin%20Frappe.tmTheme`,
    tmThemeSha256: "3446d8a3cfb9eb559bc65a3894e6ae8f3579030fac6130c4f96ff524f3e2784f",
    source: `https://github.com/catppuccin/bat/blob/${catppuccinBat}/themes/Catppuccin%20Frappe.tmTheme`,
    chrome: { background: "#292c3c", foreground: "#c6d0f5", border: "#51576d" },
  },
];

const assets = [
  ...documents.map((document) => ({
    name: document.file,
    source: resolve(benchmarksDir, document.file),
    sha256: document.sha256,
  })),
  ...themes.map((theme) => ({
    name: theme.tmTheme,
    url: theme.tmThemeUrl,
    sha256: theme.tmThemeSha256,
  })),
];

await mkdir(assetsDir, { recursive: true });

for (const asset of assets) {
  const path = resolve(assetsDir, asset.name);
  let bytes = asset.source ? await readFile(asset.source) : undefined;

  if (!bytes) {
    try {
      const cached = await readFile(path);
      if (sha256(cached) === asset.sha256) bytes = cached;
    } catch {
      // Download below.
    }
    if (!bytes) {
      const response = await fetch(asset.url);
      if (!response.ok) throw new Error(`failed to download ${asset.url}: HTTP ${response.status}`);
      bytes = Buffer.from(await response.arrayBuffer());
    }
  }

  const actual = sha256(bytes);
  if (actual !== asset.sha256) {
    throw new Error(`${asset.name} SHA-256 mismatch: expected ${asset.sha256}, got ${actual}`);
  }
  await writeFile(path, bytes);
}

await writeFile(resolve(assetsDir, "documents.json"), `${JSON.stringify(documents, null, 2)}\n`);
await writeFile(resolve(assetsDir, "themes.json"), `${JSON.stringify(themes, null, 2)}\n`);

console.log(generatedDir);

function sha256(bytes) {
  return createHash("sha256").update(bytes).digest("hex");
}

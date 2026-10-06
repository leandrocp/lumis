/**
 * remark-lumis — Remark plugin that replaces fenced code blocks with
 * Lumis-highlighted HTML at build time.
 *
 * Uses the multi-themes formatter so a single HTML block supports both
 * light and dark mode via CSS custom properties.
 *
 * Works in Fumadocs MDX pipelines where code blocks inside JSX
 * components (e.g. <Tabs>) must be processed at the remark (MDAST) level
 * rather than rehype (HAST).
 *
 * @see https://lumis.sh
 * @see https://github.com/remarkjs/remark/blob/main/doc/plugins.md
 */

"use strict";

import { visit } from "unist-util-visit";
import { createRequire } from "node:module";
const require = createRequire(import.meta.url);

// Native ESM dynamic import — avoids jiti interception of import()
// which would break resolution of ESM-only packages like @lumis-sh/lumis.
// oxlint-disable-next-line no-new-func -- Function keeps jiti from rewriting the native dynamic import.
const dynamicImport = new Function("specifier", "return import(specifier)");

/** Resolve the root directory of an installed npm package. */
function packageRoot(name) {
  const entry = require.resolve(name);
  return entry.replace(/[/\\]dist[/\\].*$/u, "");
}

/** Aliases for common markdown language identifiers. */
const LANGUAGE_ALIASES = {
  sh: "bash",
  shell: "bash",
  zsh: "bash",
  ts: "typescript",
  tsx: "tsx",
  js: "javascript",
  jsx: "javascript",
  py: "python",
  rb: "ruby",
  yml: "yaml",
  heex: "heex",
  eex: "eex",
  ex: "elixir",
  exs: "elixir",
  rs: "rust",
  md: "markdown",
};

/** Languages that should not be highlighted (pass through to Fumadocs). */
const SKIP_LANGUAGES = new Set(["plaintext", "text", "txt", "xml"]);

// ── Singleton loader ────────────────────────────────────────────────

let highlighterPromise = null;
let formatterPromise = null;

function loadHighlighter() {
  if (!highlighterPromise) {
    const root = packageRoot("@lumis-sh/lumis");
    highlighterPromise = Promise.all([
      dynamicImport(root + "/dist/index.js"),
      dynamicImport(require.resolve("@lumis-sh/wasm-bundle-full")),
    ]).then(([{ createHighlighter }, { default: full }]) =>
      createHighlighter({ languages: [full] }),
    );
  }
  return highlighterPromise;
}

function loadFormatter() {
  if (!formatterPromise) {
    const lumisRoot = packageRoot("@lumis-sh/lumis");
    const themesRoot = packageRoot("@lumis-sh/themes");
    formatterPromise = Promise.all([
      dynamicImport(lumisRoot + "/dist/formatters.js"),
      dynamicImport(themesRoot + "/dist/themes/catppuccin_latte.js"),
      dynamicImport(themesRoot + "/dist/themes/catppuccin_frappe.js"),
    ]).then(([fmt, light, dark]) => ({
      htmlMultiThemes: fmt.htmlMultiThemes,
      light: light.default,
      dark: dark.default,
    }));
  }
  return formatterPromise;
}

// ── Plugin ──────────────────────────────────────────────────────────

function remarkLumis() {
  return async function transformer(tree) {
    const [hl, { htmlMultiThemes, light, dark }] = await Promise.all([
      loadHighlighter(),
      loadFormatter(),
    ]);

    // Collect code nodes — avoids mutating the tree during traversal.
    const codeNodes = [];
    visit(tree, "code", (node, index, parent) => {
      if (node.lang && parent && index != null) {
        codeNodes.push({ node, index, parent });
      }
    });

    for (const { node, index, parent } of codeNodes) {
      const lang = LANGUAGE_ALIASES[node.lang] ?? node.lang;
      if (SKIP_LANGUAGES.has(lang)) continue;

      try {
        await hl.loadLanguage(lang);

        const highlighted = hl.highlight(
          node.value,
          htmlMultiThemes({
            language: lang,
            themes: { light, dark },
            defaultTheme: "light",
          }),
        );

        // Render through <LumisCodeBlock>, which adds the title bar from
        // title="..." meta and the copy button.
        const attributes = [{ type: "mdxJsxAttribute", name: "html", value: highlighted }];
        const title = node.meta?.match(/title="([^"]*)"/u)?.[1];
        if (title) attributes.push({ type: "mdxJsxAttribute", name: "title", value: title });

        parent.children[index] = {
          type: "mdxJsxFlowElement",
          name: "LumisCodeBlock",
          attributes,
          children: [],
        };
      } catch {
        // Highlighting failed — leave the node as-is for Fumadocs fallback.
      }
    }
  };
}

export default remarkLumis;

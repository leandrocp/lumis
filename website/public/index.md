---
title: Lumis - Syntax Highlighter
description: Syntax highlighting powered by Tree-sitter and Neovim themes, with one API across six runtimes.
---

# Lumis

Lumis is a syntax highlighter powered by Tree-sitter. It provides one model for choosing a language, theme, and formatter across CLI, Rust, Elixir, JavaScript / TypeScript, browsers, and Java.

[Documentation](https://docs.lumis.sh) · [GitHub](https://github.com/leandrocp/lumis) · [Showcase](https://lumis.sh/showcase/) · [Visual comparison](https://lumis.sh/comparison/)

## At a glance

- 6 runtimes with aligned APIs and output
- 110+ compiled Tree-sitter grammars with highlight and injection queries
- 250+ themes generated from Neovim colorschemes
- HTML inline, HTML linked, multi-theme HTML, terminal, and BBCode formatters, plus custom formatters
- Parsers loaded when a document needs them, integrity-checked, and persisted across restarts

## Install

| Runtime | Command or dependency |
| --- | --- |
| CLI | `curl -LsSf https://lumis.sh/install.sh \| sh` |
| Rust | `cargo add lumis` |
| JavaScript / TypeScript | `npm install @lumis-sh/lumis @lumis-sh/themes` |
| Browsers / CDN | `https://esm.sh/@lumis-sh/lumis` |
| Elixir | `{:lumis, "~> 0.7"}` |
| Java | `io.roastedroot:lumis4j:0.0.7` |

## Quick start

```javascript
import { highlight } from "@lumis-sh/lumis";
import { htmlInline } from "@lumis-sh/lumis/formatters";
import javascript from "@lumis-sh/lumis/langs/javascript";
import dracula from "@lumis-sh/themes/dracula";

const html = await highlight(
  "const x = 1",
  htmlInline({ language: javascript, theme: dracula }),
);
```

See the [full quick start](https://docs.lumis.sh) for CLI, Rust, Elixir, browser, and Java examples.

## Highlights

### Interactive playground

Choose a language and any built-in theme to highlight code directly in the browser. The page also includes an inspector that exposes each token's scope, language, byte range, and foreground colour.

### Injected and incomplete languages

Lumis highlights nested languages with their own grammars, including CSS and JavaScript inside HTML, HEEx inside Elixir, and fenced code inside Markdown. Native runtimes load injected languages during the same pass that discovers them. Tree-sitter also lets Lumis highlight incomplete code while it is still streaming.

### Formatters

- `html_inline`: inline styles on every token
- `html_linked`: class names with cacheable stylesheets
- `html_multi_theme`: multiple themes in one render with automatic colour-scheme switching
- `terminal`: ANSI output using the same themes
- `bbcode_scoped`: scoped BBCode colour tags
- Custom formatters built from the highlighted token stream

### Integrations

Lumis integrates with React, react-markdown, markdown-it, Astro, Nuxt, Docusaurus, Vite, VitePress, Ratatui, Nimble Publisher, and Tableau. Browse the [integration guides](https://docs.lumis.sh/integrations/react) or [build a custom formatter](https://docs.lumis.sh/formatters/custom).

### Pre-built parsers

Install one parser, such as `@lumis-sh/wasm-html`, or a preset bundle such as `@lumis-sh/wasm-bundle-full`. Every parser package includes its queries and integrity metadata. Browse the [Lumis packages on npm](https://www.npmjs.com/search?q=keywords:lumis-sh).

## Packages and source

- [Rust crate](https://crates.io/crates/lumis)
- [JavaScript / TypeScript package](https://www.npmjs.com/package/@lumis-sh/lumis)
- [Elixir package](https://hex.pm/packages/lumis)
- [Java package](https://central.sonatype.com/artifact/io.roastedroot/lumis4j)
- [Source code](https://github.com/leandrocp/lumis)

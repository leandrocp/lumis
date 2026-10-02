---
title: Lumis - Syntax Highlighter
description: Syntax highlighting with Tree-sitter and Neovim themes in 7 runtimes, including community-maintained Java and Python packages.
---

# Lumis

Lumis is a syntax highlighter powered by Tree-sitter. It supports 7 runtimes: CLI, Rust, Elixir, JavaScript / TypeScript, browsers, Java, and Python. [Lumis4J](https://github.com/roastedroot/lumis4j) (Java) and [fastpylight](https://github.com/AnswerDotAI/fastpylight) (Python) are community packages with their own APIs and releases.

[Documentation](https://docs.lumis.sh) · [GitHub](https://github.com/leandrocp/lumis) · [Showcase](https://lumis.sh/showcase/) · [Visual comparison](https://lumis.sh/comparison/)

## At a glance

- 7 runtimes: 5 first-party and 2 community-maintained
- 110+ compiled Tree-sitter grammars with highlight and injection queries
- 250+ themes generated from Neovim colorschemes
- HTML inline, HTML linked, multi-theme HTML, terminal, and BBCode formatters, plus custom formatters
- Parsers declared as dependencies, integrity-checked, and persisted across restarts

## Install

| Runtime | Command or dependency |
| --- | --- |
| CLI | `curl -LsSf https://lumis.sh/install.sh \| sh` |
| Rust | `cargo add lumis` |
| JavaScript / TypeScript | `npm install @lumis-sh/lumis @lumis-sh/themes @lumis-sh/wasm-javascript` |
| Browsers / CDN | `https://esm.sh/@lumis-sh/lumis` |
| Elixir | `{:lumis, "~> 0.10"}` plus a parser such as `{:lumis_wasm_elixir, "~> 0.26.0"}` |
| Java (community) | `io.roastedroot:lumis4j:0.0.7` |
| Python (community) | `pip install fastpylight` |

## Quick start

```javascript
import { highlight } from "@lumis-sh/lumis";
import { htmlInline } from "@lumis-sh/lumis/formatters";
import dracula from "@lumis-sh/themes/dracula";
import javascript from "@lumis-sh/wasm-javascript";

const html = await highlight(
  "const x = 1",
  htmlInline({ language: javascript, theme: dracula }),
);
```

The [quick start](https://docs.lumis.sh) has CLI, Rust, Elixir, browser, and Java examples. For fastpylight, see the [Python guide](https://docs.lumis.sh/usage/python).

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

Install one parser, such as `@lumis-sh/wasm-html`, or a preset bundle such as `@lumis-sh/wasm-bundle-full`. Elixir uses the same parsers under Hex names: `lumis_wasm_html`, `lumis_wasm_bundle_full`. Every parser package includes its queries and integrity metadata. Browse the [Lumis packages on npm](https://www.npmjs.com/search?q=keywords:lumis-sh) or [on Hex](https://hex.pm/packages?search=lumis_wasm_).

## Community

### Community runtimes

roastedroot maintains [Lumis4J](https://github.com/roastedroot/lumis4j) for Java. Answer.AI maintains [fastpylight](https://github.com/AnswerDotAI/fastpylight) for Python. Check the [community runtime guides](https://docs.lumis.sh/community#community-runtimes) for supported features and examples.

### Used by

- [Hex.pm](https://hex.pm): Highlights package source.
- [Tuist](https://tuist.dev): Highlights Markdown through MDEx.
- [Oban Pro](https://oban.pro): Highlights website code examples through MDEx.
- [Petal Components](https://petal.build): Highlights HEEx component examples through MDEx.
- [SocratiCode](https://github.com/giancarloerra/SocratiCode): Uses Lumis parsers to analyze HEEx and EEx templates.
- [see](https://github.com/guilhermeprokisch/see): Highlights code and Markdown in the terminal.

[Full project list and integration links](https://docs.lumis.sh/community#used-by).

### Mentions

- [Lumis: Syntax Highlighter powered by Tree-sitter](https://blog.master.dev/lumis-syntax-highlighter-powered-by-tree-sitter/): Chris Coyier, Master.dev.
- [Syntax highlighting in Java, without the pain](https://chicory.dev/blog/syntax-highlight/): Andrea Peruffo, on building Lumis4J.
- [Leandro Pereira on MDEx](https://www.youtube.com/watch?v=IyDNtqlClhU): Elixir Mentor interview covering MDEx, Lumis, and open source.

[All mentions](https://docs.lumis.sh/community#mentions). To add your project or a mention, send a PR to the [Community page](https://github.com/leandrocp/lumis/blob/main/docs/content/community.md).

## Packages and source

- [Rust crate](https://crates.io/crates/lumis)
- [JavaScript / TypeScript package](https://www.npmjs.com/package/@lumis-sh/lumis)
- [Elixir package](https://hex.pm/packages/lumis)
- [Python package (community)](https://pypi.org/project/fastpylight/)
- [Java package (community)](https://central.sonatype.com/artifact/io.roastedroot/lumis4j)
- [Source code](https://github.com/leandrocp/lumis)

---
sidebar_position: 3
slug: /architecture
title: Architecture
description: How Lumis processes source code into highlighted output.
keywords:
  - lumis architecture
  - tree-sitter
  - pipeline
  - crates
---

# Architecture

Lumis has three core components: languages, themes, and formatters. Source code flows through a pipeline that turns it into styled output.

## Pipeline

```text
source code
    |
    v
Tree-sitter parser (language grammar + highlight queries)
    |
    v
highlighted tokens (text + scope + byte range)
    |
    v
theme (scope -> color/style mapping)
    |
    v
formatter (tokens + styles -> HTML / ANSI / BBCode Scoped / custom output)
```

1. Tree-sitter parses the source into a concrete syntax tree
2. Highlight queries map tree nodes to scope names (`keyword`, `function`, `string`, etc.)
3. The theme maps scopes to colors and styles (foreground, background, bold, italic)
4. The formatter turns each token + style into the final output format

## Components

### Languages

110+ Tree-sitter grammars. Each language has:

- a parser (compiled to native code for Rust, WASM for JavaScript, Elixir, and the CLI)
- highlight queries (mostly from [nvim-treesitter](https://github.com/nvim-treesitter/nvim-treesitter))
- injections for nested languages (e.g., CSS and JavaScript inside HTML)

Dynamic runtimes receive parser and queries together in a self-contained,
independently versioned `@lumis-sh/wasm-*` package. A small stable catalog maps
language IDs to package names, and one Tree-sitter-compatible npm range applies
to the whole catalog. The CDN resolves that range; the package supplies current
queries and the exact integrity-pinned parser. Updating one language within the
supported series does not require a runtime package release.

### Themes

250+ themes extracted from Neovim colorschemes. Each theme is a JSON file mapping Tree-sitter scopes to styles. Themes have a `name` and `appearance` (light or dark).

### Formatters

Convert highlighted tokens into output:

| Formatter | Output |
| --- | --- |
| HTML Inline | `<span style="color: #e5c07b;">` |
| HTML Linked | `<span class="l-keyword">` |
| HTML Multi-Themes | `<span style="--lumis-light:#333; --lumis-dark:#ccc;">` |
| Terminal | ANSI escape codes |
| BBCode Scoped | `[keyword-function]main[/keyword-function]` |

## Crate structure

```text
lumis-core        internal: language detection, theme/style logic, the formatters and their builders
lumis             public Rust API, Tree-sitter adapter, re-exports the formatters
lumis-cli         CLI binary
lumis-build       build-time code generation
lumis-wasm-runtime shared Tree-sitter WASM engine, lazy registry, and bounded worker pool
```

## Package layers

- The Elixir package (`packages/elixir/lumis`) uses a small Rustler NIF with a shared Wasmtime engine and loads parser WASM per language.
- The JavaScript runtime package (`packages/javascript/lumis`) uses a native addon over that same Wasmtime runtime on Node, and `web-tree-sitter` in browsers, with the same per-language parser assets either way.
- The integration packages (`packages/javascript/markdown-it-lumis` and `packages/javascript/rehype-lumis`) build on top of that JavaScript runtime for Markdown and HAST pipelines.

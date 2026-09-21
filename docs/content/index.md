---
title: Lumis
description: Lumis is a Tree-sitter syntax highlighter with Neovim themes and one workflow across CLI, Rust, Elixir, JavaScript / TypeScript, Browsers / CDN, and Java.
keywords:
  - lumis
  - syntax highlighting
  - tree-sitter
  - neovim themes
  - rust
  - elixir
  - javascript
  - typescript
  - java
---



Lumis is a syntax highlighter built on [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) and [Neovim themes](https://github.com/topics/neovim-colorscheme).

It keeps the same core workflow across runtimes:

1. [pick a language](/languages)
2. [pick a theme](/themes)
3. [pick a formatter](/formatters)
4. [render to multiple formats](/usage/highlight)

[Install Lumis](/installation) to get started.

## Why Lumis

- [Tree-sitter parsing](/architecture) instead of regex-based tokenization
- [110+ languages](/reference/languages) that [load on demand or preload](/languages), with [injected languages](/recipes/injected-languages-html-css-js) highlighted in the same pass
- [250+ themes](/reference/themes) sourced from Neovim colorschemes, or [one you generate](/recipes/generate-theme-from-neovim-colorscheme)
- 5 formatters: [HTML Inline](/formatters/html-inline), [HTML Linked](/formatters/html-linked), [HTML Multi-Themes](/formatters/html-multi-themes), [Terminal](/formatters/terminal), and [BBCode Scoped](/formatters/bbcode), or [one you write](/formatters/custom)
- [handles incomplete code](/usage/highlight#incomplete-code), useful for streaming
- one API across 6 runtimes: [CLI](/usage/cli), [Rust](/usage/rust), [Elixir](/usage/elixir), [JavaScript / TypeScript](/usage/javascript), [Browsers / CDN](/recipes/browser-cdn), and [Java](/usage/java)

## How it works

- [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) parses code into syntax trees
- [Neovim colorschemes](https://github.com/topics/neovim-colorscheme) supply the color data
- highlight queries come from [`nvim-treesitter`](https://github.com/nvim-treesitter/nvim-treesitter)

[Architecture](/architecture) walks the whole pipeline, from source code to styled output.

---

Every code block on this site is highlighted by Lumis itself, using the [multi-themes formatter](/formatters/html-multi-themes) with `catppuccin_latte` and `catppuccin_frappe` themes for automatic light/dark mode switching.

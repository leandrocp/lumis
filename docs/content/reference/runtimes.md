---
sidebar_position: 1
slug: /reference/runtimes
title: Runtimes
description: Package and API references for Lumis across CLI, Rust, Elixir, JavaScript, Browsers / CDN, and Java.
keywords:
  - lumis
  - docs.rs
  - hexdocs
  - npm
  - java
---

# Runtimes

## Main packages

| Runtime | Package | Reference |
| --- | --- | --- |
| CLI | `@lumis-sh/cli` (`lumis` binary), `lumis-cli` on crates.io | [npm](https://www.npmjs.com/package/@lumis-sh/cli) |
| Rust | `lumis` | [docs.rs](https://docs.rs/lumis) |
| Elixir | `lumis` | [HexDocs](https://hexdocs.pm/lumis) |
| JavaScript | `@lumis-sh/lumis` | [npm](https://www.npmjs.com/package/@lumis-sh/lumis) |
| Browsers / CDN | `@lumis-sh/lumis` | [npm](https://www.npmjs.com/package/@lumis-sh/lumis) |
| Java | `lumis4j` | [GitHub](https://github.com/roastedroot/lumis4j) |

`@lumis-sh/lumis` covers Node.js, Bun, Deno, and browsers. Node, the CLI and
Elixir all run the same Wasmtime highlighting from `lumis-wasm-runtime`, so
identical input produces identical output; browsers use `web-tree-sitter`.

Every dynamic runtime loads exact, integrity-checked parser WASM per language
rather than shipping an all-language binary, downloading what a document turns
out to need and persisting verified assets across process restarts. A host
application can [warm the parser store](/operations/warm-up) at startup without
blocking it, and the standalone CLI can prepare the directory ahead of time.

## Themes

| Runtime | Package | Reference |
| --- | --- | --- |
| JavaScript | `@lumis-sh/themes` | [npm](https://www.npmjs.com/package/@lumis-sh/themes) |

## Integrations

| Runtime | Integration | Package | Reference |
| --- | --- | --- | --- |
| JavaScript | React | `@lumis-sh/react` | [npm](https://www.npmjs.com/package/@lumis-sh/react) |
| JavaScript | Vite HTML | `@lumis-sh/vite` | [npm](https://www.npmjs.com/package/@lumis-sh/vite) |
| JavaScript | markdown-it | `@lumis-sh/markdown-it-lumis` | [npm](https://www.npmjs.com/package/@lumis-sh/markdown-it-lumis) |
| JavaScript | rehype | `@lumis-sh/rehype-lumis` | [npm](https://www.npmjs.com/package/@lumis-sh/rehype-lumis) |

## WASM language packages

Dynamic parser grammars are published as self-contained language packages such
as `@lumis-sh/wasm-rust`, `@lumis-sh/wasm-javascript`, and
`@lumis-sh/wasm-elixir`. Each package keeps its parser, queries, aliases, and
integrity metadata at one version so JavaScript, CLI, and Elixir load the same
language definition.

Preset bundle packages are also available, such as `@lumis-sh/wasm-bundle-web`, `@lumis-sh/wasm-bundle-web-extra`, `@lumis-sh/wasm-bundle-system`, and `@lumis-sh/wasm-bundle-backend`.

- package list: [Languages](/reference/languages)
- helper for npm parser packages in non-Node runtimes: `withWasm()` from `@lumis-sh/lumis`
- helper for npm preset bundles in non-Node runtimes: `withWasmBundle()` from `@lumis-sh/lumis`
- loading and resolver behavior: [WASM and CDN](/advanced/wasm-and-cdn)
- JavaScript runtime notes: [JavaScript Runtime](/usage/javascript)

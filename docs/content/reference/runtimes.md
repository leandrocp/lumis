---
title: Runtimes
description: Seven Lumis runtimes, including community-maintained Java and Python packages.
keywords:
  - lumis
  - docs.rs
  - hexdocs
  - npm
  - typescript
  - java
---



Lumis supports **7 runtimes**. Five are maintained in this repository; the community maintains the Java and Python packages.

## First-party runtimes

| Runtime | Package | Reference |
| --- | --- | --- |
| CLI | `@lumis-sh/cli` (`lumis` binary), `lumis-cli` on crates.io | [npm](https://www.npmjs.com/package/@lumis-sh/cli) |
| Rust | `lumis` | [docs.rs](https://docs.rs/lumis) |
| Elixir | `lumis` | [HexDocs](https://hexdocs.pm/lumis) |
| JavaScript / TypeScript | `@lumis-sh/lumis` | [npm](https://www.npmjs.com/package/@lumis-sh/lumis) |
| Browsers / CDN | `@lumis-sh/lumis` | [npm](https://www.npmjs.com/package/@lumis-sh/lumis) |

`@lumis-sh/lumis` has one JavaScript and TypeScript API with bundled declarations.
It covers Node.js, Bun, Deno, and browsers. Node, the CLI and
Elixir all run the same Wasmtime highlighting from `lumis-wasm-runtime`, so
identical input produces identical output; browsers use `web-tree-sitter`.

Every dynamic runtime loads exact parser WASM per language rather than shipping
an all-language binary. It loads what a document turns out to need, and keeps
its caches across process restarts. A host
application can [preload the languages it needs](/languages) at startup without
blocking it, and the standalone CLI can prepare the directory ahead of time.

## Community-maintained runtimes

These packages use Lumis's highlighting engine and have their own APIs and releases. Check their documentation for supported features and installation instructions.

| Runtime | Package | Maintainer | Guide |
| --- | --- | --- | --- |
| Java | [Lumis4J](https://github.com/roastedroot/lumis4j) | [roastedroot](https://github.com/roastedroot) | [Java](/usage/java) |
| Python | [fastpylight](https://github.com/AnswerDotAI/fastpylight) | [Answer.AI](https://github.com/AnswerDotAI) | [Python](/usage/python) |

## Themes

| Runtime | Package | Reference |
| --- | --- | --- |
| JavaScript / TypeScript | `@lumis-sh/themes` | [npm](https://www.npmjs.com/package/@lumis-sh/themes) |

## Integrations

| Runtime | Integration | Package | Reference |
| --- | --- | --- | --- |
| JavaScript / TypeScript | React | `@lumis-sh/react` | [npm](https://www.npmjs.com/package/@lumis-sh/react) |
| JavaScript / TypeScript | Vite HTML | `@lumis-sh/vite` | [npm](https://www.npmjs.com/package/@lumis-sh/vite) |
| JavaScript / TypeScript | markdown-it | `@lumis-sh/markdown-it-lumis` | [npm](https://www.npmjs.com/package/@lumis-sh/markdown-it-lumis) |
| JavaScript / TypeScript | rehype | `@lumis-sh/rehype-lumis` | [npm](https://www.npmjs.com/package/@lumis-sh/rehype-lumis) |

## WASM language packages

Dynamic parser grammars are published as self-contained language packages such
as `@lumis-sh/wasm-rust`, `@lumis-sh/wasm-javascript`, and
`@lumis-sh/wasm-elixir`. Each package keeps its parser, queries, aliases, and
integrity metadata at one version so the npm package, CLI, and Elixir load the same
language definition.

Preset bundle packages are also available, such as `@lumis-sh/wasm-bundle-web`, `@lumis-sh/wasm-bundle-web-extra`, `@lumis-sh/wasm-bundle-system`, and `@lumis-sh/wasm-bundle-backend`.

- package list: [Languages](/reference/languages)
- each package's default export is its language, so `import rust from "@lumis-sh/wasm-rust"` is all a highlighter needs; a parser that serves more than one language exports each by name
- a bundle package's default export registers its languages lazily
- loading: [Languages](/languages)
- parser sources and resolvers: [JavaScript / TypeScript](/usage/javascript#where-parsers-come-from)
- JavaScript / TypeScript usage: [JavaScript / TypeScript](/usage/javascript)

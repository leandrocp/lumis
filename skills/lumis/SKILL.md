---
name: lumis
description: Add Lumis syntax highlighting to Rust, Elixir, JavaScript, TypeScript, browser, Java, or CLI projects. Use when choosing a Lumis runtime, formatter, theme, language-loading strategy, or integration.
license: MIT
---

# Use Lumis

Use the public Lumis APIs and the official documentation. Do not recreate its parsing or formatting behavior.

## Choose the surface

1. Identify the target runtime: Rust, Elixir, JavaScript or TypeScript, browser, Java, or CLI.
2. Identify the required output: inline HTML, class-based HTML, light/dark HTML, terminal escapes, or scoped BBCode.
3. Read the [installation guide](https://docs.lumis.sh/installation) and the matching runtime guide:
   - [Rust](https://docs.lumis.sh/usage/rust)
   - [Elixir](https://docs.lumis.sh/usage/elixir)
   - [JavaScript and TypeScript](https://docs.lumis.sh/usage/javascript)
   - [Java](https://docs.lumis.sh/usage/java)
   - [CLI](https://docs.lumis.sh/usage/cli)
4. Pick the formatter from the [formatter guide](https://docs.lumis.sh/formatters).

The complete agent-readable documentation is available at [llms.txt](https://docs.lumis.sh/llms.txt), and the documentation MCP server is at `https://docs.lumis.sh/api/mcp`.

## Implement

Follow the same workflow on every runtime:

1. Choose or detect the language. In JavaScript and Elixir, also install its parser package, such as `@lumis-sh/wasm-rust` or `:lumis_wasm_rust`, or a bundle: Lumis loads no parser the project did not install or import. See [Versions](#versions).
2. Choose a theme when the formatter uses one.
3. Configure the formatter.
4. Highlight the source.
5. Render the returned output without altering its escaping or token structure.

For repeated JavaScript highlighting, create and reuse a highlighter instead of rebuilding one per call. In browsers, preload languages injected by the document because browser parser loading is asynchronous. Native runtimes load injected languages on demand.

## Versions

Never copy a version number from memory, an old example, or this file. Look up the current release before adding a dependency:

- Elixir: `mix hex.info lumis` and `mix hex.info lumis_wasm_rust`, or hex.pm
- JavaScript: `npm view @lumis-sh/lumis version` and `npm view @lumis-sh/wasm-rust version`
- Rust: `cargo add lumis`, which picks the current version and lets you enable `lang-*` features
- CLI: the [installation guide](https://docs.lumis.sh/installation)

Keep parser packages (`lumis_wasm_*`, `@lumis-sh/wasm-*`) in the range the installed Lumis supports, and upgrade them together with Lumis. Lumis refuses an out-of-range parser and reports `:incompatible_version` in Elixir (`Lumis.Packages.requirement()` returns the right requirement) or "does not satisfy the supported range" elsewhere.

## When a parser is missing

- **Elixir**: `Lumis.highlight/2` returns `{:ok, html}` as plain text and logs a warning. Invalid options still raise, and `highlight!/2` raises `Lumis.HighlightError` only for formatter failures. To detect a missing parser, call `Lumis.Languages.load/1`, which returns a `Lumis.ParserError` with reason `:not_installed` (add the dependency) or `:parser_missing` (run `mix deps.get` again).
- **JavaScript and TypeScript**: on Node, a missing root language throws (`@lumis-sh/wasm-regex is not installed`); a missing injected language stays plain for that block with a one-time warning. Synchronous `hl.highlight()` and React throw `Language "x" is not loaded` until `createHighlighter({ languages })` or `hl.loadLanguage()` has loaded it. Browsers fetch nothing while highlighting, so load every language first.
- **Rust**: a language whose `lang-*` feature is disabled renders as plain text without an error. Enable the features you need.
- **CLI**: a missing parser is downloaded on demand; an unknown language exits non-zero with `unknown language: <name>`.

Plain text is not an error signal. When highlighting must not silently degrade, load the language explicitly and handle the failure.

## Verify

- Exercise the actual target runtime, not only a source build.
- Test incomplete input when the integration highlights streamed or editor content.
- Test nested languages for HTML, Markdown fences, or other injected-language formats.
- For linked HTML, include the matching Lumis theme CSS.
- For light/dark output, use the multi-theme formatter rather than combining separate renders.

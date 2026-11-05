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

Parser packages (`lumis_wasm_*`, `@lumis-sh/wasm-*`) must stay inside the version range the installed Lumis supports. Lumis refuses a parser outside it and reports `:incompatible_version` (Elixir `Lumis.ParserError`) or "does not satisfy the supported range" (JavaScript, CLI). The range is a `MAJOR.MINOR` series defined by the installed Lumis release, so after upgrading Lumis, upgrade every parser package with it. In Elixir, `Lumis.Packages.requirement()` returns the requirement to put in `mix.exs`, and the error message prints it.

## When a parser is missing

Behavior differs by runtime. Do not assume it is the same everywhere.

- **Elixir**: `Lumis.highlight/2` returns `{:ok, html}` with the source rendered as plain text and logs a warning. No `Lumis.ParserError` ever reaches a `highlight/2` caller. Invalid options still raise. `Lumis.highlight!/2` raises `Lumis.HighlightError` only when the formatter fails, not for a missing parser. `Lumis.highlight_events/3` returns one `:source` event spanning the document. To detect it, call `Lumis.Languages.load/1` (for example at boot), which returns `{:error, %Lumis.ParserError{}}`. Reasons include `:not_installed` (add the `lumis_wasm_*` dependency) and `:parser_missing` (the dependency is present but its `.wasm` is not; run `mix deps.get` again).
- **JavaScript and TypeScript on Node**: the root language loads from the installed package, and a missing one throws, for example `@lumis-sh/wasm-regex is not installed`. A language injected inside a document stays plain for that block, and Lumis warns once naming it. Synchronous `hl.highlight()` throws `Language "x" is not loaded` when the language was not passed to `createHighlighter({ languages })` or `hl.loadLanguage()`; an unknown name throws `Unknown language "x"`. Catch the error or call `hl.loadLanguage()` first to detect it.
- **Browser**: nothing is fetched during highlighting. A language that was not imported and loaded throws `Language "x" is not loaded`, and an injected language that no highlighter on the page loaded stays plain. Load every language the document can name before highlighting.
- **React** (`useLumis`): a language that is not loaded rejects with `Language "x" is not loaded`.
- **Rust**: a language whose `lang-*` feature is disabled is highlighted as plain text without an error. Enable the feature, or `all-languages`, for every language you highlight. Language names that cannot be resolved fall back to detection, then to plain text.
- **CLI**: a missing compatible parser is downloaded on demand, and an unknown language exits non-zero with `unknown language: <name>`. A root language that cannot be downloaded is an error, and only an injected block that cannot be fetched stays plain. Run `lumis languages download <names>` to fetch parsers ahead of time.

A plain-text result is not an error signal. If highlighted output must be guaranteed, load the language explicitly (`Lumis.Languages.load/1`, `hl.loadLanguage()`) and handle its failure.

## Verify

- Exercise the actual target runtime, not only a source build.
- Test incomplete input when the integration highlights streamed or editor content.
- Test nested languages for HTML, Markdown fences, or other injected-language formats.
- For linked HTML, include the matching Lumis theme CSS.
- For light/dark output, use the multi-theme formatter rather than combining separate renders.

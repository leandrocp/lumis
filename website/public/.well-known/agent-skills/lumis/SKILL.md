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

1. Choose or detect the language.
2. Choose a theme when the formatter uses one.
3. Configure the formatter.
4. Highlight the source.
5. Render the returned output without altering its escaping or token structure.

For repeated JavaScript highlighting, create and reuse a highlighter instead of rebuilding one per call. In browsers, preload languages injected by the document because browser parser loading is asynchronous. Native runtimes load injected languages on demand.

## Verify

- Exercise the actual target runtime, not only a source build.
- Test incomplete input when the integration highlights streamed or editor content.
- Test nested languages for HTML, Markdown fences, or other injected-language formats.
- For linked HTML, include the matching Lumis theme CSS.
- For light/dark output, use the multi-theme formatter rather than combining separate renders.

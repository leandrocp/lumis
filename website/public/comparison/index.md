---
title: Lumis - Visual output comparison
description: The same source files highlighted by Lumis, Shiki, highlight.js, and syntect.
---

# Visual output comparison

The [interactive comparison](https://lumis.sh/comparison/) renders the same source files with Lumis, Shiki, highlight.js, and syntect. It provides Catppuccin Latte and Frappé views and preserves each implementation's own public API and closest available Catppuccin theme.

## Documents

- three.js WebGPU compute reduce: HTML with CSS, JSON, and JavaScript injections
- ripgrep searcher: Rust
- Livebook core components: Elixir with HEEx and Markdown injections
- Go `encoding/json`
- Lumis README: Markdown with Bash, Elixir, Java, JavaScript, and Rust injections
- shadcn/ui sidebar: TSX

## Reading the comparison

A token is a span the highlighter gave a colour to. The token count shows how finely an implementation resolved the selected document; it is not a quality score because grammars split punctuation and whitespace differently.

The displayed timing is separate from the visual output. It is the median time to highlight one small Rust file in the [Lumis benchmark suite](https://github.com/leandrocp/lumis/blob/main/benchmarks/README.md), with Lumis represented by its Rust runtime.

The source files, rendering pipeline, exact package versions, themes, and generated outputs are available in the [Lumis repository](https://github.com/leandrocp/lumis).

[Lumis home](https://lumis.sh/) · [Documentation](https://docs.lumis.sh) · [Showcase](https://lumis.sh/showcase/)

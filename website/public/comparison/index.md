---
title: Lumis - Visual output comparison
description: The same source files highlighted by Lumis, highlight.js, Prism, Shiki, speed-highlight, starry-night, Sugar High, syntect, and TanStack Highlight.
---

# Visual output comparison

The [interactive comparison](https://lumis.sh/comparison/) renders the same source files with Lumis, highlight.js, Prism, Shiki, speed-highlight, starry-night, Sugar High, syntect, and TanStack Highlight. It provides Catppuccin Latte and Frappé views and preserves each implementation's own public API and closest available Catppuccin theme. speed-highlight, starry-night, Sugar High, and TanStack Highlight have no Catppuccin theme, so their colours come from the highlight.js port, class for class.

## Documents

- three.js WebGPU: HTML with CSS, JSON, and JavaScript injections
- ripgrep searcher: Rust, which TanStack Highlight does not support
- Livebook core components: Elixir with HEEx and Markdown injections, which speed-highlight, Sugar High, and TanStack Highlight do not support
- Go `encoding/json`
- Lumis README: Markdown with Bash, Elixir, Java, JavaScript, and Rust injections
- shadcn/ui sidebar: TSX, which speed-highlight reads as TypeScript because it has no TSX grammar

## Reading the comparison

A token is a span the highlighter gave a colour to. The token count shows how finely an implementation resolved the selected document; it is not a quality score because grammars split punctuation and whitespace differently.

The displayed timing is the median time the selected implementation takes to highlight the selected document, through the same call that produced the output on screen, with the highlighter already built and its languages loaded. Every timing comes from one run on one machine, and Lumis is represented by its Rust runtime.

The source files, rendering pipeline, exact package versions, themes, and generated outputs are available in the [Lumis repository](https://github.com/leandrocp/lumis).

[Lumis home](https://lumis.sh/) · [Documentation](https://docs.lumis.sh) · [Showcase](https://lumis.sh/showcase/)

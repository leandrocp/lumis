---
title: Lumis - Visual output comparison
description: The same source files highlighted by Lumis, highlight.js, Highlights, Prism, Shiki, speed-highlight, starry-night, Sugar High, syntect, and TanStack Highlight.
---

# Visual output comparison

The [interactive comparison](https://lumis.sh/comparison/) renders the same source files with Lumis, highlight.js, Highlights, Prism, Shiki, speed-highlight, starry-night, Sugar High, syntect, and TanStack Highlight, each through its own public API, in Catppuccin Latte and Frappé. Every Lumis runtime renders each file byte for byte identically, so one Lumis output is shown.

## Documents

- [three.js WebGPU](https://github.com/mrdoob/three.js/blob/6365c1a0af6a32ed45f99712197555fee2f4b24a/examples/webgpu_compute_reduce.html): HTML with CSS, JSON, and JavaScript injections, 1,397 lines
- [ripgrep searcher](https://github.com/BurntSushi/ripgrep/blob/4649aa9700619f94cf9c66876e9549d83420e16c/crates/searcher/src/searcher/mod.rs): Rust with Comment injections, 1,054 lines
- [Livebook core components](https://github.com/livebook-dev/livebook/blob/5133601d9678fb8ef0c87484e5d52eff193813e5/lib/livebook_web/components/core_components.ex): Elixir with HEEx, Markdown, and Comment injections, 1,152 lines; not supported by Sugar High and speed-highlight
- [Go encoding/json](https://github.com/golang/go/blob/3901409b5d0fb7c85a3e6730a59943cc93b2835c/src/encoding/json/encode.go): Go with Comment injections, 1,342 lines
- [Lumis README](https://github.com/leandrocp/lumis/blob/main/README.md): Markdown with Bash, Elixir, Java, JavaScript, and Rust injections, 181 lines
- [shadcn/ui sidebar](https://github.com/shadcn-ui/ui/blob/607e8a9717fe6ff0d374ba74c651012f9c052534/apps/v4/registry/new-york-v4/ui/sidebar.tsx): TSX with Comment injections, 727 lines

The source files, rendering pipeline, themes, and generated outputs are available in the [Lumis repository](https://github.com/leandrocp/lumis).

[Lumis home](https://lumis.sh/) · [Documentation](https://docs.lumis.sh) · [Showcase](https://lumis.sh/showcase/)

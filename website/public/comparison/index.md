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

## Speed and output

Measured 2026-11-06 on Intel(R) Xeon(R) Processor @ 2.60GHz (4 cores, Debian GNU/Linux 12 (bookworm)), Node.js v24.18.0, rustc 1.99.1 (58cf8f9c3 2026-10-13). Each figure is a median. The small file is [shadcn/ui sidebar](https://github.com/shadcn-ui/ui/blob/607e8a9717fe6ff0d374ba74c651012f9c052534/apps/v4/registry/new-york-v4/ui/sidebar.tsx), TSX, 726 lines. The large file is [three.js core build](https://github.com/mrdoob/three.js/blob/6365c1a0af6a32ed45f99712197555fee2f4b24a/build/three.core.js), JavaScript, 60,392 lines.

### Highlight

Time: one highlight of the whole document through the call that rendered the published output, with Lumis as Lumis Rust; building the highlighter and loading its languages are excluded.

| Library | Small: shadcn/ui sidebar (21 KiB) | Large: three.js core build (1.4 MiB) |
| --- | ---: | ---: |
| Lumis Rust | 6.0 ms | 629 ms |
| Lumis JavaScript (Node, Wasmtime) | 8.9 ms | 754 ms |
| highlight.js | 8.6 ms | 467 ms |
| Highlights | 277 µs | 16.2 ms |
| Prism | 10.0 ms | 498 ms |
| Shiki | 40.1 ms | 2.66 s |
| speed-highlight | 2.5 ms | 175 ms |
| starry-night | 40.1 ms | 1.42 s |
| Sugar High | 3.4 ms | 126 ms |
| syntect | 29.8 ms | 1.61 s |
| TanStack Highlight | 2.8 ms | 62.8 ms |

### Setup + first highlight

Time: a fresh process building one library's highlighter, loading the languages the document needs and highlighting it once, timed inside the process; the Lumis data directory already holds its compiled parsers, as it does after a first run.

| Library | Small: shadcn/ui sidebar (21 KiB) | Large: three.js core build (1.4 MiB) |
| --- | ---: | ---: |
| Lumis Rust | 173 ms | 708 ms |
| Lumis JavaScript (Node, Wasmtime) | 72.1 ms | 800 ms |
| highlight.js | 104 ms | 573 ms |
| Highlights | 26.2 ms | 48.3 ms |
| Prism | 253 ms | 877 ms |
| Shiki | 354 ms | 3.22 s |
| speed-highlight | 16.6 ms | 221 ms |
| starry-night | 898 ms | 2.15 s |
| Sugar High | 32.3 ms | 259 ms |
| syntect | 52.1 ms | 1.59 s |
| TanStack Highlight | 42.7 ms | 173 ms |

### Peak RSS

The most memory that fresh process held, its runtime included.

| Library | Small: shadcn/ui sidebar (21 KiB) | Large: three.js core build (1.4 MiB) |
| --- | ---: | ---: |
| Lumis Rust | 8.7 MiB | 91.1 MiB |
| Lumis JavaScript (Node, Wasmtime) | 78.2 MiB | 171.6 MiB |
| highlight.js | 72.9 MiB | 175.4 MiB |
| Highlights | 60.3 MiB | 95.7 MiB |
| Prism | 73.1 MiB | 230.0 MiB |
| Shiki | 138.7 MiB | 386.4 MiB |
| speed-highlight | 52.5 MiB | 116.4 MiB |
| starry-night | 263.5 MiB | 308.3 MiB |
| Sugar High | 65.9 MiB | 158.1 MiB |
| syntect | 12.8 MiB | 71.3 MiB |
| TanStack Highlight | 62.9 MiB | 106.3 MiB |

### Tokens

Spans the highlighter gave a colour to: how finely the file was resolved, not a score.

| Library | Small: shadcn/ui sidebar (21 KiB) | Large: three.js core build (1.4 MiB) |
| --- | ---: | ---: |
| Lumis Rust | 2,422 | 192,011 |
| Lumis JavaScript (Node, Wasmtime) | 2,422 | 192,011 |
| highlight.js | 1,259 | 69,071 |
| Highlights | 2,177 | 191,439 |
| Prism | 2,631 | 177,972 |
| Shiki | 2,474 | 213,728 |
| speed-highlight | 1,487 | 107,956 |
| starry-night | 2,026 | 94,427 |
| Sugar High | 3,228 | 202,958 |
| syntect | 2,946 | 199,271 |
| TanStack Highlight | 838 | 55,122 |

The source files, rendering pipeline, themes, and generated outputs are available in the [Lumis repository](https://github.com/leandrocp/lumis).

[Lumis home](https://lumis.sh/) · [Documentation](https://docs.lumis.sh) · [Showcase](https://lumis.sh/showcase/)

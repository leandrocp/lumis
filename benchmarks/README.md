# Benchmarks

Run all benchmarks and regenerate this file with `mise run -C benchmarks run`.

The timing rows use local workspace source for every Lumis runtime.

**Highlight** is the per-call cost with a prepared highlighter. **Setup** is
what building that highlighter costs once the process is warm. They are
separated because the runtimes cache very differently, and timing them
together compares caching rather than highlighting.

Every Lumis row except Rust loads the same WebAssembly parsers; Rust compiles
its own in and is the floor rather than something anyone installs. What differs
between the two JavaScript rows is which engine runs those parsers and where
the highlight pass happens: **(Node, Wasmtime)** runs them under Wasmtime in a
native addon and walks and formats in Rust, the same code the CLI and Elixir
run; **(web-tree-sitter)** runs them under V8 and walks and formats in
JavaScript. That second one is what browsers use, and what Node falls back to
where no addon is built.

See the [focused before/after report](elixir-runtime.md) for Elixir cold
loading, concurrency, and memory.

## Visual output comparison

Generate the gallery with `mise run -C benchmarks showcase`, then open
[`index.html`](index.html) directly in a browser.

This comparison is separate from timing. Every implementation highlights the
same pinned 1,397-line Three.js HTML file, including injected CSS, JSON, and
JavaScript, in Catppuccin Latte and Frappé. The gallery covers Lumis Rust,
JavaScript Wasm, Elixir, and CLI alongside Shiki, highlight.js, syntect, and
bat. Its preparation step verifies SHA-256 hashes for the fixture and for
syntect's Catppuccin tmThemes before rendering.

It also counts the tokens each implementation coloured in each document, so
the gallery can say how finely a file was resolved next to the output itself.

## Publishing to the website

The deploy workflow builds only `website/` and `docs/`, so it cannot run any
of this. Both sets of numbers are committed, and these refresh them:

```sh
mise run -C benchmarks showcase-publish
mise run -C benchmarks comparison-timings
mise run -C benchmarks comparison-timings-publish
```

`showcase-publish` writes the gallery and its token counts, and CI fails when
the committed bytes stop matching. `comparison-timings` times every library on
every document the gallery renders, and `comparison-timings-publish` writes
those times beside the outputs. No check can verify them because they are
measurements; run them from a quiet machine, or the site will quote whatever
that machine was also doing. The timings below stay in this file.

## Package size

Measured on AMD EPYC 7763 64-Core Processor (x64, linux).
npm rows sum the packed and unpacked sizes of each unique production package.
Native rows compare the raw release artifact with deterministic gzip level 9.
Compare rows within the same artifact class; npm packages, executables, and a NIF
are not interchangeable distribution formats.

| Tool | Measured artifact | Raw / unpacked | Download / gzip -9 |
| --- | --- | ---: | ---: |
| Lumis JavaScript (Node, Wasmtime) (runtime) | npm production closure including the platform addon; parsers load on demand | 21.29 MiB | 7.48 MiB |
| Lumis JavaScript (Node, Wasmtime) (10 languages) | npm production closure including the platform addon, plus benchmark parsers | 33.78 MiB | 8.89 MiB |
| Lumis JavaScript (web-tree-sitter) (runtime) | npm production closure without the platform addon; parsers load on demand | 6.64 MiB | 1.68 MiB |
| Lumis JavaScript (web-tree-sitter) (1 language) | npm production closure plus Rust parser | 9.13 MiB | 1.94 MiB |
| Lumis JavaScript (web-tree-sitter) (10 languages) | npm production closure plus benchmark parsers | 19.13 MiB | 3.10 MiB |
| Shiki | npm production closure | 13.97 MiB | 2.71 MiB |
| highlight.js | npm production closure | 4.39 MiB | 760.98 KiB |
| Lumis Rust | stripped 10-language benchmark executable | 12.33 MiB | 2.14 MiB |
| syntect | stripped default-syntax benchmark executable | 1.40 MiB | 803.36 KiB |
| Lumis CLI | stripped release executable | 16.60 MiB | 6.06 MiB |
| bat | release executable | 6.59 MiB | 3.43 MiB |
| Lumis Elixir | stripped release NIF shared library | 16.44 MiB | 5.98 MiB |

## 1 small file for 1 language

Highlight one small Rust file.

| Tool | Highlight | Setup |
| --- | ---: | ---: |
| highlight.js | 943.390 µs | 199.963 µs |
| Lumis Rust | 1.331 ms | — |
| Lumis Elixir | 2.073 ms | — |
| Lumis JavaScript (Node, Wasmtime) | 2.134 ms | 79.871 ms |
| syntect | 5.132 ms | — |
| Shiki | 6.129 ms | 1.410 ms |
| Lumis JavaScript (web-tree-sitter) | 8.637 ms | 80.821 ms |
| bat | 32.955 ms | — |
| Lumis CLI | 96.956 ms | — |

## 1 big file for 1 language

Highlight one generated 5 MiB Rust file.

| Tool | Highlight | Setup |
| --- | ---: | ---: |
| highlight.js | 1.302 s | 308.956 µs |
| Lumis Rust | 2.939 s | — |
| Lumis CLI | 3.169 s | — |
| Lumis JavaScript (Node, Wasmtime) | 3.461 s | 80.231 ms |
| Lumis Elixir | 3.473 s | — |
| syntect | 4.241 s | — |
| bat | 8.159 s | — |
| Shiki | 12.695 s | 687.793 µs |
| Lumis JavaScript (web-tree-sitter) | 15.787 s | 78.353 ms |

## 10 different files for 1 language

Highlight ten different small Rust files.

| Tool | Highlight | Setup |
| --- | ---: | ---: |
| highlight.js | 2.510 ms | 157.985 µs |
| Lumis Rust | 3.619 ms | — |
| Lumis Elixir | 6.452 ms | — |
| Lumis JavaScript (Node, Wasmtime) | 7.639 ms | 83.313 ms |
| syntect | 8.550 ms | — |
| Shiki | 20.366 ms | 872.277 µs |
| Lumis JavaScript (web-tree-sitter) | 22.737 ms | 76.499 ms |
| bat | 290.670 ms | — |
| Lumis CLI | 949.660 ms | — |

## 10 different files for 10 languages

Highlight ten different small files, each in a different language.

| Tool | Highlight | Setup |
| --- | ---: | ---: |
| Lumis Rust | 2.712 ms | — |
| highlight.js | 2.752 ms | 768.143 µs |
| Lumis Elixir | 5.235 ms | — |
| Lumis JavaScript (Node, Wasmtime) | 6.126 ms | 383.980 ms |
| Lumis JavaScript (web-tree-sitter) | 13.203 ms | 430.438 ms |
| Shiki | 13.284 ms | 23.741 ms |
| syntect | 22.993 ms | — |
| bat | 224.772 ms | — |
| Lumis CLI | 483.599 ms | — |

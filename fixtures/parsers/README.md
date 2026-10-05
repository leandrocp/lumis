# Parsers CI cannot build

A `.wasm` here is a grammar that `mise run wasm-build` produces locally but a CI
runner cannot, so query compilation would otherwise have no parser to check its
queries against, and that language would fail its shard.

The reason is memory. `tree-sitter build --wasm` compiles the generated
`parser.c` through a WASI SDK it fetches itself, and a few grammars need more
than a GitHub runner's 16 GB. Measured peak resident size while building:

| Grammar | Peak RSS | Wall clock |
| --- | --- | --- |
| `tree-sitter-vim` | 18.3 GB | ~12 min |
| `tree-sitter-zsh` | 13.4 GB | ~25 min |
| `tree-sitter-llvm` | 10.5 GB | ~8 min |
| everything else | under 6 GB | seconds |

The other 110 grammars build in seconds and are built in CI. llvm was measured
at 10.5 GB and looked like it would fit; a runner killed it anyway, with exit
143 after five minutes, so the measurement is a guide rather than the rule.

Each file is built from the revision `languages.toml` pins at the time it is
committed, with `mise run wasm-build <parser>`, and copied from
`tmp/wasm/build/`. Rebuild and replace it when that revision changes, the same
as any other pinned artifact.

This list may only shrink. A grammar belongs here only while CI genuinely cannot
build it; delete the file once it can.

Not to be confused with [`../test-parsers`](../test-parsers/README.md), which
holds the grammars the test suites load. CI builds those; it cannot build these.

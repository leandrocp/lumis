# Parser patches

A parser here is pinned to an upstream revision. When Lumis needs a fix before
upstream releases it, the fix lives as a patch in this directory rather than as
edits to the generated sources under
`crates/lumis/vendored_parsers/`, and `[parsers.<name>] patch` in
`crates/lumis/languages.toml` names it.

`mise run langs-fetch-vendored-parsers <name>` and `mise run wasm-build <name>`
both apply it to a fresh checkout before the grammar is generated, so the
native C sources and the WASM artifact come from the same corrected grammar.
The patch also feeds the WASM build id, so editing it rebuilds the artifact.

A revision bump that the patch no longer applies to fails loudly, which is the
point: that is the moment to check whether upstream has taken the fix and the
patch can go.

| patch | parser | upstream |
| --- | --- | --- |
| `tree-sitter-gleam-error-sentinel.patch` | `gleam` | not yet reported |

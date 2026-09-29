# Language-package validation corpus

One `lumis.json` document per file. Every runtime that parses a language package must reach the
**same accept/reject verdict** on every file here.

- `valid/` — must parse and validate.
- `invalid/` — must be rejected. The reason is the filename.

Consumers:

- `crates/lumis-wasm-runtime/tests/language_package_corpus.rs` (Rust; the CLI and the Elixir NIF
  both validate through this crate)
- `packages/javascript/lumis/test/language-package-corpus.test.ts` (Node and browser runtimes)

Both tests assert the corpus size, so a discovery bug that silently finds no fixtures fails.

Every document declares `packageName: "@lumis-sh/wasm-json"`. The corpus covers *document validity*
only. There is no `formatVersion` field and no version gate: the format is additive-only, so
compatibility is decided by shape. Unknown fields are ignored by both runtimes, and a missing
required field is rejected by name.

No runtime reads `definitionHash` or `parser.size` any more. Older releases still require them, so
published packages carry both, and `valid/unread-fields.json` pins that no value in them rejects a
document. `languages` must be a non-empty JSON object, never an array.

The shared raw JSON profile applies before fields are interpreted, including to unknown or
overwritten values:

- The document is UTF-8 JSON without a byte-order mark.
- Every string token contains only Unicode scalar values, with no unpaired surrogate escapes.
- Every number token parses to a finite binary64 value.
- At most 127 arrays or objects may be open at once, matching serde_json's default recursion limit.
- Duplicate object members use the last value, matching JSON.parse and serde_json::Value maps.

`packageName` follows npm's lowercase package-name grammar, and non-null parser provenance fields
are strings. Language IDs and aliases are matched with ASCII case folding, so no two language
entries may claim the same folded name.

Checking that a fetched package's name matches the one that was requested happens a layer up, in
`Registry::fetch_package`, `parse_language_package`, and `parseLanguagePackage`'s
`expectedPackageName` argument.

The explicitly named edge fixtures preserve validator decisions that previously diverged between
Rust and JavaScript. When adding a field to the format, add a fixture here in the same change.

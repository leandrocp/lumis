# Parsers that fail during parsing

`php-0.26.4.wasm` is the parser from the published
`@lumis-sh/wasm-php@0.26.4` tarball. Its SHA-256 is
`78400ce8fbdcb88d39318f910127065f1d7e3c98bb38163fcd67a6c2f4cd9596`.
It loads successfully, then traps on a heredoc or nowdoc. Tests use it to
distinguish parse failure from cancellation without mocking Tree-sitter.

Keep it separate from `test-parsers/`, whose parsers are staged as usable
language packages. The working PHP parser there comes from upstream commit
`8b7d06271bc91573c0e981c391d52191df661dc9`, which evaluates the heredoc stack
pop once before deleting its word. Lumis regenerates the Wasm parser with
Tree-sitter's current headers; its `array_delete` macro evaluates its argument
more than once. The native 0.24.2 crate uses the older headers and does not
trigger this failure.

The broken module can corrupt the browser's shared Wasm memory. Resetting,
deleting, or replacing its parser does not reliably recover it. Run browser
failure tests in a disposable context.

# @lumis-sh/lumis

Syntax Highlighter powered by Tree-sitter and Neovim themes.

JavaScript / TypeScript package for [Lumis](https://lumis.sh). Works in Node.js, Bun, Deno, and browsers, with TypeScript declarations included.

## Features

- **110+ Tree-sitter languages** - Fast, accurate, and updated syntax parsing
- **250+ built-in Neovim themes** - Updated and curated themes from the Neovim community
- **Built-in formatters** - HTML (inline/linked), Terminal (ANSI), Multi-theme (light/dark), BBCode
- **Custom formatters** - Build your own output
- **Language auto-detection** - File extension, shebang, and emacs-mode support
- **Line highlighting** - Mark and style individual lines, with custom HTML wrappers
- **Streaming-friendly** - Handles incomplete code
- **Load parsers on demand** - From the packages you install or import, verified, including injected languages on Node; browsers load those up front

## Install

```sh
npm install @lumis-sh/lumis
npm install @lumis-sh/themes
```

## Usage

```typescript
import { highlight } from '@lumis-sh/lumis'
import { htmlInline } from '@lumis-sh/lumis/formatters'
import javascript from '@lumis-sh/lumis/langs/javascript'
import dracula from '@lumis-sh/themes/dracula'

const html: string = await highlight('const x = 1', htmlInline({ language: javascript, theme: dracula }))
```

`highlight()` shares one process-wide runtime, which is what you want for a
one-off. For repeated calls, `createHighlighter()` loads languages during setup
and highlights synchronously afterwards:

```typescript
import { createHighlighter } from '@lumis-sh/lumis'

const hl = await createHighlighter({ languages: [javascript] })
const html = hl.highlight('const x = 1', htmlInline({ language: javascript, theme: dracula }))
```

Bundles register a whole set at once — `bundles/web`, `web-extra`, `system`,
`backend`, `full` — and each language in one still loads lazily on first use.

## Parsers

Each language import is a handle to an independently released parser package,
such as `@lumis-sh/wasm-javascript`. On Node, install the ones you highlight and
Lumis reads them from `node_modules`. In a browser, import the whole package and
pass it to `withWasm()`; a browser loads nothing else unless you configure a
resolver. Either way the bytes are checked against the package's SHA-256 digest
before use.

```typescript
import { createHighlighter, withWasm } from '@lumis-sh/lumis'
import javascript from '@lumis-sh/lumis/langs/javascript'
import * as javascriptPackage from '@lumis-sh/wasm-javascript'

const hl = await createHighlighter({ languages: [withWasm(javascript, javascriptPackage)] })
```

On Node, a document also loads the languages **injected inside** it during the
same pass, so a Markdown file with a fenced Rust block highlights that block
without Rust being named in your code. Browsers load asynchronously, so load
injected languages up front, including those from a registered bundle.

On Node, warm parsers alongside startup, without putting the compile on the
boot path:

```typescript
import { loadLanguages } from '@lumis-sh/lumis'

await startServer()

// Not awaited. The `.catch()` is required: an unhandled rejection would
// terminate the process.
loadLanguages(['javascript', 'html', 'css']).catch((error) => {
  logger.warn({ error }, 'Lumis warm-up failed; languages load on demand')
})
```

`loadLanguages()` warms the runtime `highlight()` uses and keeps the languages
there, persisting compiled Wasmtime modules on native Node. To compile in a build
step instead, run the same call there, awaited, with `LUMIS_DATA_DIR` set to the
directory the server uses.

## Documentation

Guides for formatters, themes, integrations and recipes are at
[docs.lumis.sh](https://docs.lumis.sh).

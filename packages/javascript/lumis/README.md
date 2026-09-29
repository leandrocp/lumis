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
- **Load parsers on demand** - From the packages you install or import, including injected languages on Node; browsers load those up front

## Install

```sh
npm install @lumis-sh/lumis @lumis-sh/themes
npm install @lumis-sh/wasm-javascript
```

Each language is its own package. Install the ones you highlight, or a bundle
such as `@lumis-sh/wasm-bundle-web`.

## Usage

```typescript
import { highlight } from '@lumis-sh/lumis'
import { htmlInline } from '@lumis-sh/lumis/formatters'
import dracula from '@lumis-sh/themes/dracula'
import javascript from '@lumis-sh/wasm-javascript'

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

A bundle package registers a whole set at once, and each language in it still
loads lazily on first use:

```typescript
import web from '@lumis-sh/wasm-bundle-web'

const hl = await createHighlighter({ languages: [web] })
await hl.loadLanguage(web.typescript)
```

The bundles are `@lumis-sh/wasm-bundle-web`, `-web-extra`, `-system`,
`-backend`, and `-full`.

## Parsers

Each parser is an independently released package, such as
`@lumis-sh/wasm-javascript`, holding the parser, its queries, and its SHA-256
digest. The package's default export is the language, so the same import works
on Node, where Lumis reads the parser from `node_modules`, and in a browser,
where your bundler ships it. A project loads nothing it didn't install or
import unless you configure a resolver, and bytes Lumis fetches are checked
against the digest before use.

Packages are named after the parser, and a parser that serves more than one
language exports each by name:

```typescript
import markdown, { mdx } from '@lumis-sh/wasm-markdown'
import { ejs, erb } from '@lumis-sh/wasm-embedded-template'
```

Importing `markdown` also loads `markdown_inline`, the grammar it depends on.
On Vite 7 and older, list the parser packages you import in
`optimizeDeps.exclude`; Vite 8 needs nothing.

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

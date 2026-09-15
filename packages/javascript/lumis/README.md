# @lumis-sh/lumis

Syntax Highlighter powered by Tree-sitter and Neovim themes.

JavaScript/TypeScript package for [Lumis](https://lumis.sh). Works in Node.js, Bun, Deno, and browsers.

## Features

- **110+ Tree-sitter languages** - Fast, accurate, and updated syntax parsing
- **250+ built-in Neovim themes** - Updated and curated themes from the Neovim community
- **Built-in formatters** - HTML (inline/linked), Terminal (ANSI), Multi-theme (light/dark), BBCode
- **Custom formatters** - Build your own output
- **Language auto-detection** - File extension, shebang, and emacs-mode support
- **Line highlighting** - Mark and style individual lines, with custom HTML wrappers
- **Streaming-friendly** - Handles incomplete code
- **Load parsers on demand** - Verified and cached, including injected languages on Node; browsers load those up front

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

const html = await highlight('const x = 1', htmlInline({ language: javascript, theme: dracula }))
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

Each language import is a handle to an independently released parser package.
Lumis resolves it to an exact version, verifies the bytes against a SHA-256
digest before use, and caches them.

On Node, a document also loads the languages **injected inside** it during the
same pass, so a Markdown file with a fenced Rust block highlights that block
without Rust being named in your code. Browsers load asynchronously, so name
injected languages up front or use a bundle.

Warm parsers alongside startup, without putting the CDN on the boot path:

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
there. `cacheLanguages()` writes the same files but holds nothing, for a build
step preparing a directory another process reads; `lumis languages cache` does
that from the CLI. On native Node both also persist compiled Wasmtime modules.

## Documentation

- [JavaScript runtime](https://lumis.sh/docs/usage/javascript) — runtimes, bundles, language handles
- [WASM and CDN](https://lumis.sh/docs/advanced/wasm-and-cdn) — resolution, caching, custom resolvers
- [Formatters](https://lumis.sh/docs/formatters) — every formatter and its options
- [Custom formatters](https://lumis.sh/docs/formatters/custom)
- [Annotations](https://lumis.sh/docs/formatters/annotations) — compose your own ranges into the event stream
- [Themes](https://lumis.sh/docs/themes) and [CSS theme files](https://lumis.sh/docs/themes/css-files)
- [Integrations](https://lumis.sh/docs/integrations/react) — React, Next.js, Astro, Nuxt, VitePress, rehype, markdown-it
- [Recipes](https://lumis.sh/docs/recipes)

# @lumis-sh/markdown-it-lumis

[markdown-it](https://github.com/markdown-it/markdown-it) plugin for [Lumis](https://lumis.sh) syntax highlighting.

## Install

```sh
npm install @lumis-sh/markdown-it-lumis @lumis-sh/themes \
  @lumis-sh/wasm-javascript @lumis-sh/wasm-typescript @lumis-sh/wasm-json \
  @lumis-sh/wasm-bundle-web
```

## Quick start

```typescript
import MarkdownIt from 'markdown-it'
import markdownItLumis from '@lumis-sh/markdown-it-lumis'
import { htmlInline } from '@lumis-sh/lumis/formatters'
import javascript from '@lumis-sh/wasm-javascript'
import typescript from '@lumis-sh/wasm-typescript'
import json from '@lumis-sh/wasm-json'
import dracula from '@lumis-sh/themes/dracula'

const plugin = await markdownItLumis({
  formatter: (language) => htmlInline({ language, theme: dracula }),
  languages: [javascript, typescript, json],
})

const md = new MarkdownIt()
md.use(plugin)

const html = md.render('```javascript\nconst x = 1\n```')
```

`formatter` receives the language detected from the fenced block and returns a Lumis [Formatter](https://github.com/leandrocp/lumis/tree/main/packages/javascript/lumis#output-formats). Any built-in or custom formatter works.

`languages` lists which languages to load and prepare. Since `md.render()` is synchronous, this has to happen before rendering. Fenced blocks that use a language not in this list won't be highlighted.

## Multiple themes

```typescript
import { htmlMultiThemes } from '@lumis-sh/lumis/formatters'
import githubLight from '@lumis-sh/themes/github_light'
import githubDark from '@lumis-sh/themes/github_dark'

const plugin = await markdownItLumis({
  formatter: (language) => htmlMultiThemes({
    language,
    themes: { light: githubLight, dark: githubDark },
  }),
  languages: [javascript],
})
```

## Using bundles

You can pass a [bundle](https://github.com/leandrocp/lumis/tree/main/packages/javascript/lumis#bundles) to make a group of languages available, then load specific ones by name:

```typescript
import web from '@lumis-sh/wasm-bundle-web'

const plugin = await markdownItLumis({
  formatter: (language) => htmlInline({ language, theme: dracula }),
  languages: [web, 'javascript', 'json'],
})
```

## Bring your own highlighter

If you already have a `Highlighter` instance, use `fromHighlighter()` to skip setup:

```typescript
import { createHighlighter } from '@lumis-sh/lumis'
import { fromHighlighter } from '@lumis-sh/markdown-it-lumis'
import javascript from '@lumis-sh/wasm-javascript'

const hl = await createHighlighter({ languages: [javascript] })
const plugin = fromHighlighter(hl, {
  formatter: (language) => htmlInline({ language, theme: dracula }),
})

const md = new MarkdownIt()
md.use(plugin)
```

## Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `formatter` | `(language: string \| undefined) => Formatter` | (required) | Creates a Lumis formatter for each code block |
| `languages` | `Array` | `[]` | Languages to load and prepare. Accepts Language objects, bundles, or string names (when a bundle is also passed). |

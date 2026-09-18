# @lumis-sh/rehype-lumis

[rehype](https://github.com/rehypejs/rehype) plugin for [Lumis](https://lumis.sh) syntax highlighting.

Finds `<pre><code class="language-*">` elements in the hast tree and replaces them with highlighted output.
Authored classes, styles, ids, ARIA attributes, and data attributes on both elements are preserved.

## Install

```sh
npm install @lumis-sh/rehype-lumis @lumis-sh/themes
```

## Quick start

```typescript
import rehypeParse from 'rehype-parse'
import rehypeStringify from 'rehype-stringify'
import { unified } from 'unified'
import rehypeLumis from '@lumis-sh/rehype-lumis'
import { htmlInline } from '@lumis-sh/lumis/formatters'
import javascript from '@lumis-sh/lumis/langs/javascript'
import dracula from '@lumis-sh/themes/dracula'

const result = await unified()
  .use(rehypeParse, { fragment: true })
  .use(rehypeLumis, {
    formatter: (language) => htmlInline({ language, theme: dracula }),
    languages: [javascript],
  })
  .use(rehypeStringify)
  .process('<pre><code class="language-javascript">const x = 1</code></pre>')

console.log(String(result))
```

`formatter` receives the language detected from the code block and returns a Lumis [Formatter](https://github.com/leandrocp/lumis/tree/main/packages/javascript/lumis#output-formats). Any built-in or custom formatter works.

Each language is downloaded the first time a matching code block is found. Code blocks with unavailable languages are left unchanged.

## Multiple themes

```typescript
import { htmlMultiThemes } from '@lumis-sh/lumis/formatters'
import githubLight from '@lumis-sh/themes/github_light'
import githubDark from '@lumis-sh/themes/github_dark'

unified()
  .use(rehypeLumis, {
    formatter: (language) => htmlMultiThemes({
      language,
      themes: { light: githubLight, dark: githubDark },
    }),
    languages: [javascript],
  })
```

## Using bundles

You can pass a [bundle](https://github.com/leandrocp/lumis/tree/main/packages/javascript/lumis#bundles) to make a group of languages available:

```typescript
import { bundledLanguages } from '@lumis-sh/lumis/bundles/web'

unified()
  .use(rehypeLumis, {
    formatter: (language) => htmlInline({ language, theme: dracula }),
    languages: [bundledLanguages],
  })
```

## Language detection

The plugin reads the language from (in order):

1. `class="language-*"` on the `<code>` element
2. `class="language-*"` on the `<pre>` element
3. `data-language` attribute on the `<pre>` element
4. `language` attribute on the `<pre>` element

If no language is detected, Lumis tries to guess from the content.

## Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `formatter` | `(language: string \| undefined) => Formatter` | (required) | Creates a Lumis formatter for each code block |
| `languages` | `LanguageInput[]` | `[]` | Languages to make available. Accepts Language objects or bundles. |

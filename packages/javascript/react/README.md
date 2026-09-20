# @lumis-sh/react

React integration for [Lumis](https://lumis.sh) syntax highlighting.

Docs: [https://docs.lumis.sh](https://docs.lumis.sh)

Examples:

- [Vite React](./examples/react)
- [Next.js App Router](./examples/next-app-router)

## Install

```bash
npm install @lumis-sh/react @lumis-sh/lumis @lumis-sh/themes react
```

## Usage

### Simple

```tsx
import { CodeBlock } from '@lumis-sh/react'
import { htmlInline } from '@lumis-sh/lumis/formatters'
import githubLight from '@lumis-sh/themes/github_light'

export function Example() {
  return (
    <CodeBlock formatter={htmlInline({ language: 'javascript', theme: githubLight })}>
      {`const x = 1`}
    </CodeBlock>
  )
}
```

### Reuse One Highlighter

```tsx
import { CodeBlock } from '@lumis-sh/react'
import { createHighlighter } from '@lumis-sh/lumis'
import { bundledLanguages } from '@lumis-sh/lumis/bundles/web'
import { htmlInline } from '@lumis-sh/lumis/formatters'
import githubLight from '@lumis-sh/themes/github_light'

const highlighter = createHighlighter({ languages: [bundledLanguages] })

export function Example() {
  return (
    <CodeBlock
      highlighter={highlighter}
      formatter={htmlInline({ language: 'javascript', theme: githubLight })}
    >
      {`const x = 1`}
    </CodeBlock>
  )
}
```

## Hook

Pass `highlighter` when you want to reuse one instance across multiple blocks or load languages during setup.

```tsx
import { useLumis } from '@lumis-sh/react'
import { createHighlighter } from '@lumis-sh/lumis'
import { bundledLanguages } from '@lumis-sh/lumis/bundles/web'
import { htmlInline } from '@lumis-sh/lumis/formatters'
import githubLight from '@lumis-sh/themes/github_light'

const highlighter = createHighlighter({ languages: [bundledLanguages] })

export function Example() {
  const { content, isLoading } = useLumis({
    children: 'const x = 1',
    formatter: htmlInline({ language: 'javascript', theme: githubLight }),
    highlighter,
  })

  if (isLoading) return null
  return content
}
```

## Server rendering

```tsx
import { renderCodeBlock } from '@lumis-sh/react/server'
import { bundledLanguages } from '@lumis-sh/lumis/bundles/web'
import { htmlInline } from '@lumis-sh/lumis/formatters'
import githubLight from '@lumis-sh/themes/github_light'

const node = await renderCodeBlock({
  children: 'const x = 1',
  formatter: htmlInline({ language: bundledLanguages.javascript, theme: githubLight }),
})
```

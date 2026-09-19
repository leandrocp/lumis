# @lumis-sh/vite

Highlights code in Vite HTML entry points with Lumis. The plugin uses Vite's
`transformIndexHtml` hook, so it is specifically for Vite rather than a generic
Rollup plugin.

## Install

```sh
npm install -D @lumis-sh/vite
npm install @lumis-sh/lumis @lumis-sh/themes
```

## Usage

```js
import {defineConfig} from 'vite'
import lumis from '@lumis-sh/vite'
import {htmlMultiThemes} from '@lumis-sh/lumis/formatters'
import javascript from '@lumis-sh/lumis/langs/javascript'
import githubDark from '@lumis-sh/themes/github_dark'
import githubLight from '@lumis-sh/themes/github_light'

export default defineConfig({
  plugins: [
    lumis({
      languages: [javascript],
      formatter: (language) =>
        htmlMultiThemes({
          language,
          themes: {light: githubLight, dark: githubDark},
          defaultTheme: 'light-dark()',
        }),
    }),
  ],
})
```

Write blocks as `<pre><code class="language-javascript">...</code></pre>`, or set
`data-language` on `<pre>`. Authored attributes on both elements are preserved.

See the [Vite integration guide](https://lumis.sh/docs/integrations/vite) for behavior,
language preloading, and a complete example.

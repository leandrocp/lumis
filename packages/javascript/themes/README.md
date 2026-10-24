# @lumis-sh/themes

Neovim themes for [Lumis Syntax Highlighter](https://lumis.sh)

This default pack contains permissively licensed themes and is installed with
`@lumis-sh/lumis`. Copyleft themes require `@lumis-sh/themes-copyleft`; their existing
`@lumis-sh/themes/<name>` imports forward to that package. See the [theme guide](https://docs.lumis.sh/themes).

## Usage

### JS module

```js
import dracula from '@lumis-sh/themes/dracula'
```

### CSS

```html
<link rel="stylesheet" href="https://unpkg.com/@lumis-sh/themes/dist/css/dracula.css">
```

### JSON

```
https://unpkg.com/@lumis-sh/themes/dist/json/dracula.json
```

### Build custom CSS

Use `buildCss(theme, options)` to build a stylesheet you can inline or scope. The CSS builder guide is at [docs.lumis.sh](https://docs.lumis.sh).

## License

The package is MIT. Each theme keeps the license of the Neovim plugin it was extracted from; `dist/NOTICE.md` lists the source, pinned commit, license and copyright notice for every theme.

## Browse

- CSS: https://unpkg.com/browse/@lumis-sh/themes/dist/css/
- JSON: https://unpkg.com/browse/@lumis-sh/themes/dist/json/

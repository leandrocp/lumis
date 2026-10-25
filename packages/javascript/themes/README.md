# @lumis-sh/themes

Neovim themes for [Lumis Syntax Highlighter](https://lumis.sh)

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

The package is MIT. Each theme keeps the license of the Neovim plugin it was extracted from; `dist/NOTICE.md` lists the source, pinned commit and license of every theme, `dist/licenses/` holds each plugin's license file, and each theme records its license in a `license` field.

## Browse

- CSS: https://unpkg.com/browse/@lumis-sh/themes/dist/css/
- JSON: https://unpkg.com/browse/@lumis-sh/themes/dist/json/

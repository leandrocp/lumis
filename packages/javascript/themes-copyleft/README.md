# @lumis-sh/themes-copyleft

Opt-in copyleft themes for [Lumis](https://lumis.sh).

```sh
npm install @lumis-sh/themes-copyleft
```

```js
import modusVivendi from '@lumis-sh/themes-copyleft/modus_vivendi'
```

Pass the theme object to a formatter as usual. Node's `@lumis-sh/lumis/themes`
discovers this installed pack automatically. Browser built-in theme lookup stays
limited to the default pack; import optional themes directly.

Existing `@lumis-sh/themes/<name>` imports also forward to this pack when it is
installed. CSS and JSON are available at `@lumis-sh/themes-copyleft/css/<name>`
and `@lumis-sh/themes-copyleft/json/<name>`.

See the [theme guide](https://docs.lumis.sh/themes). `dist/NOTICE.md` records each
theme's source, revision, license, and bundled license texts.

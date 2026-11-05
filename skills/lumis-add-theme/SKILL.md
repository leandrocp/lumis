---
name: lumis-add-theme
description: Add a built-in theme to the Lumis repository from a permissively licensed Neovim colorscheme. Use when contributing a new theme to leandrocp/lumis, mapping its highlight groups, regenerating THEMES.md and the theme CSS, and opening the pull request.
license: MIT
---

# Add a theme to Lumis

Built-in themes are extracted from Neovim colorscheme plugins. You never write a theme JSON by hand: you add a definition to `themes/themes.lua`, and `mise run themes-gen` runs Neovim, reads the colorscheme's highlight groups, and writes `themes/<name>.json` together with every copy the runtimes ship.

Work from a clone of `leandrocp/lumis` on a topic branch, after `mise run setup`. Extraction also needs Neovim with `vim.pack` (0.12 or later) and `jq` on `PATH`.

## 1. Check the license first

Lumis is MIT and redistributes every built-in theme, so it ships only themes whose plugin is permissively licensed.

1. Open the plugin repository and read its `LICENSE*`, `COPYING*` or `UNLICENSE*` file.
2. Stop if the plugin is copyleft (GPL, LGPL, AGPL, MPL and similar) or has no license file. Lumis removed 27 such themes in [#1874](https://github.com/leandrocp/lumis/pull/1874) and will not take new ones. Tell the user they can still generate the theme for their own use with `lumis themes generate`, which keeps any license.
3. MIT, Apache-2.0, BSD, ISC, Unlicense and similar are fine.

You do not record the license yourself. Extraction copies the plugin's license files into `themes/licenses/<owner>/<repo>/`, and `mise run themes-gen` writes the detected SPDX expression into the theme JSON's `license` field (see [#1880](https://github.com/leandrocp/lumis/pull/1880)). A file that matches no known license is recorded as `NOASSERTION`, and `mise run themes-check-notices` rejects built-in themes with `NONE`, `NOASSERTION` or a copyleft license. Extraction only looks for license files at the plugin's root. `license_dir` changes where they are copied to, not where they are read from, so a plugin whose license lives anywhere else cannot be added as it is: ask upstream to add a root license file.

## 2. Add the definition

Add an entry to `themes/themes.lua`. `name` is the theme id: lowercase with underscores, unique, and the file name of every generated output. `config` must select the variant and load the colorscheme:

```lua
{
    url = "https://github.com/author/theme-repo",
    name = "theme_name",
    config = function()
        vim.o.background = "dark"
        vim.cmd([[colorscheme theme-name]])
    end,
},
```

- Set `vim.o.background` to `"dark"` or `"light"`. It becomes the theme's `appearance`.
- One plugin with several variants gets one entry per variant, each with its own `name` and `config`.
- Add `dependencies = { "https://github.com/..." }` when the colorscheme needs another plugin to load.
- Set any plugin option that affects colors, such as italics, before loading the colorscheme, so extraction does not depend on the environment.

## 3. Map the highlight groups

Extraction reads the Tree-sitter capture groups listed in `themes/extract_theme.lua` (`@keyword`, `@function.call`, `@markup.heading.1` and so on) plus a few UI groups (`Normal`, `CursorLine`, `LineNr`, `CursorLineNr` and the `RainbowDelimiter*` groups), with links resolved. Whatever the colorscheme leaves undefined falls back to Neovim's defaults, which then ship as if the theme had chosen them.

Most modern colorschemes define the `@capture` groups and need nothing extra. Older ones define legacy names such as `TSFuncBuiltin`. Link those to the modern captures inside `config`, after loading the colorscheme, as the `nord` entry does:

```lua
config = function()
    vim.o.background = "dark"
    vim.cmd([[colorscheme nord]])

    -- Nord defines legacy TS groups, not their modern capture names.
    for capture, group in pairs({
        ["@constant.builtin"] = "TSConstBuiltin",
        ["@function.builtin"] = "TSFuncBuiltin",
        ["@variable"] = "TSVariable",
    }) do
        vim.api.nvim_set_hl(0, capture, { link = group })
    end
end,
```

Do not edit `extract_theme.lua`, `highlights.toml` or the generated JSON to make one theme look right.

## 4. Generate

```sh
mise run themes-gen theme_name
mise run css-gen
```

`themes-gen` asks for confirmation, installs the plugin under the repo-local `nvim/data`, writes `themes/theme_name.json` and its license files, syncs the copies in `crates/lumis-core/themes`, regenerates `THEMES.md`, `docs/content/reference/themes.md` and the `NOTICE.md` files, and rebuilds `@lumis-sh/themes`. `css-gen` writes `css/theme_name.css` and its copies in `crates/lumis/css` and the Elixir package.

Check the result:

- `themes/theme_name.json` has the expected `appearance`, a `revision`, and a permissive `license`.
- `highlights` covers the common scopes (`keyword`, `string`, `comment`, `function`, `type`, `variable`) with the colorscheme's colors, not Neovim defaults.
- `THEMES.md` lists the theme with its repository and license.

## 5. Preview

Render samples with the CLI built from the checkout, which embeds the new theme:

```sh
cargo run -p lumis-cli -- themes show theme_name
cargo run -p lumis-cli -- highlight -t theme_name samples/rust.rs
cargo run -p lumis-cli -- highlight -t theme_name samples/markdown.md
```

For HTML, write a page and open it in a browser:

```sh
cargo run -p lumis-cli -- highlight -f html-inline -t theme_name samples/typescript.ts > /tmp/theme_name.html
```

Compare it against a screenshot of the colorscheme in Neovim. If a scope is wrong or missing, fix the mapping in step 3 and generate again.

## 6. Check and open the pull request

These are the checks CI runs for a theme change, from `CONTRIBUTING.md` and `mise.toml`:

- [ ] `mise run themes-check-notices` passes: the license matches its files, the theme is allowed, and `THEMES.md`, the docs reference page and every `NOTICE.md` are current.
- [ ] `mise run test-theme-choice` passes. A new theme can become the closest match for a color; if it does, update `fixtures/theme-choice-cases.json` deliberately and say so in the PR.
- [ ] `cargo test -p lumis-core` passes, including the theme schema test that validates every built-in theme.
- [ ] `mise run fmt-lua -- --check` and `mise run lint-lua` pass for the `themes/themes.lua` edit.
- [ ] `mise run lint` passes, or at least its Lua and theme notice steps when the full run is not possible locally.
- [ ] Every generated file is committed: `themes/`, `themes/licenses/`, `crates/lumis-core/themes/`, `css/`, `crates/lumis/css/`, `packages/elixir/lumis/priv/static/css/`, `packages/javascript/themes/themes/`, `THEMES.md` and `docs/content/reference/themes.md`. Do not edit any of them by hand.
- [ ] No changelog edit and no version bump. Changelogs are generated from Conventional Commits at release time, so the commit subject is the changelog entry: `feat(themes): add <Theme> theme`.

Open the pull request with the plugin URL, its license, the theme ids added, a note on any groups you mapped by hand, and the commands you ran.

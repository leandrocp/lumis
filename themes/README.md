## Themes

All themes are sourced from the Neovim community.

All rights belongs to the original authors and contributors. [`NOTICE.md`](NOTICE.md) lists each theme's source, pinned commit and license, which is detected from the plugin's license files in [`licenses/`](licenses/).

## Generating Themes

Themes are automatically extracted from Neovim theme plugins using `vim.pack`, Neovim's built-in package manager.

**Note**: Requires `nvim` to be installed and available in `$PATH`.

### Using the CLI

Generate a theme JSON from any Git repository containing a Neovim theme:

```bash
# Basic usage - output to stdout
lumis gen-theme --url https://github.com/catppuccin/nvim --colorscheme catppuccin-mocha

# Save to file
lumis gen-theme \
  --url https://github.com/folke/tokyonight.nvim \
  --colorscheme tokyonight \
  -o tokyonight.json

# With custom setup code
lumis gen-theme \
  --url https://github.com/ellisonleao/gruvbox.nvim \
  --colorscheme gruvbox \
  --setup "require('gruvbox').setup({ contrast = 'hard' })" \
  -o gruvbox-hard.json

# Specify light appearance
lumis gen-theme \
  --url https://github.com/projekt0n/github-nvim-theme \
  --colorscheme github_light \
  --appearance light \
  -o github-light.json
```

### Using mise tasks

Regenerate existing themes defined in `themes.lua`:

```bash
# Regenerate all themes
mise run themes-gen

# Regenerate a single theme
mise run themes-gen catppuccin_mocha
```

### How It Works

1. **Plugin Installation**: Uses `vim.pack.add()` to install the theme plugin from the Git repository
2. **Theme Activation**: Runs the theme's config function to activate the colorscheme
3. **Color Extraction**: Extracts colors from 130+ highlight groups (including Tree-sitter groups)
4. **Revision Tracking**: Captures the Git commit SHA using `git rev-parse HEAD`
5. **JSON Output**: Generates a JSON file with the theme name, appearance, revision, and all highlight styles

### Adding New Themes

To add a new theme to the built-in collection:

1. Add the theme definition to `themes.lua`:

```lua
{
    url = "https://github.com/author/theme-repo",
    name = "theme_name",
    config = function()
        vim.o.background = "dark"
        vim.cmd([[colorscheme theme-name]])
    end,
}
```

1. Run `mise run themes-gen theme_name` to generate the JSON file

2. The theme will be automatically included in the next build via `build.rs`

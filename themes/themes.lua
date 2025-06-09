-- Themes definition
--
-- Each definition must follow https://lazy.folke.io/spec
-- and `name` is required and used as the theme identifier,
-- for example the plugin `Shatur/neovim-ayu` has a couple
-- different variants so each one is named accordingly as
-- `name: "ayu_dark"` and so on.
--
-- Note that `name` don't have to match the plugin name,
-- it's used exclusively to name the theme.

return {
	{
    "folke/lazy.nvim",
		name = "neovim_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme default]])
		end,
	},
	{
    "folke/lazy.nvim",
		name = "neovim_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme default]])
		end,
	},
	{
		"Shatur/neovim-ayu",
		name = "ayu_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme ayu-dark]])
		end,
	},
	{
		"Shatur/neovim-ayu",
		name = "ayu_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme ayu-light]])
		end,
	},
	{
		"Shatur/neovim-ayu",
		name = "ayu_mirage",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme ayu-mirage]])
		end,
	},
	{
		"AlexvZyl/nordic.nvim",
		name = "nordic",
		config = function()
			vim.o.background = "dark"
			require("nordic").load()
			vim.cmd([[colorscheme nordic]])
		end,
	},
	{
		"savq/melange-nvim",
		name = "melange_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme melange]])
		end,
	},
	{
		"savq/melange-nvim",
		name = "melange_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme melange]])
		end,
	},
	{
		"bluz71/vim-nightfly-colors",
		name = "nightfly",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme nightfly]])
		end,
	},
	{
		"folke/tokyonight.nvim",
		name = "tokyonight_night",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme tokyonight-night]])
		end,
	},
	{
		"folke/tokyonight.nvim",
		name = "tokyonight_moon",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme tokyonight-moon]])
		end,
	},
	{
		"folke/tokyonight.nvim",
		name = "tokyonight_storm",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme tokyonight-storm]])
		end,
	},
	{
		"folke/tokyonight.nvim",
		name = "tokyonight_day",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme tokyonight-day]])
		end,
	},
	{
		"catppuccin/nvim",
		name = "catppuccin_frappe",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme catppuccin-frappe]])
		end,
	},
	{
		"catppuccin/nvim",
		name = "catppuccin_macchiato",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme catppuccin-macchiato]])
		end,
	},
	{
		"catppuccin/nvim",
		name = "catppuccin_mocha",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme catppuccin-mocha]])
		end,
	},
	{
		"catppuccin/nvim",
		name = "catppuccin_latte",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme catppuccin-latte]])
		end,
	},
	{
		"projekt0n/github-nvim-theme",
		name = "github_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme github_dark]])
		end,
	},
	{
		"projekt0n/github-nvim-theme",
		name = "github_dark_default",
		colorscheme = "github_dark_default",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme github_dark_default]])
		end,
	},
	{
		"projekt0n/github-nvim-theme",
		name = "github_dark_dimmed",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme github_dark_dimmed]])
		end,
	},
	{
		"projekt0n/github-nvim-theme",
		name = "github_dark_high_contrast",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme github_dark_high_contrast]])
		end,
	},
	{
		"projekt0n/github-nvim-theme",
		name = "github_dark_colorblind",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme github_dark_colorblind]])
		end,
	},
	{
		"projekt0n/github-nvim-theme",
		name = "github_dark_tritanopia",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme github_dark_tritanopia]])
		end,
	},
	{
		"projekt0n/github-nvim-theme",
		name = "github_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme github_light]])
		end,
	},
	{
		"projekt0n/github-nvim-theme",
		name = "github_light_default",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme github_light_default]])
		end,
	},
	{
		"projekt0n/github-nvim-theme",
		name = "github_light_high_contrast",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme github_light_high_contrast]])
		end,
	},
	{
		"projekt0n/github-nvim-theme",
		name = "github_light_colorblind",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme github_light_colorblind]])
		end,
	},
	{
		"projekt0n/github-nvim-theme",
		name = "github_light_tritanopia",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme github_light_tritanopia]])
		end,
	},
	{
		"rebelot/kanagawa.nvim",
		name = "kanagawa_wave",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme kanagawa-wave]])
		end,
	},
	{
		"rebelot/kanagawa.nvim",
		name = "kanagawa_dragon",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme kanagawa-dragon]])
		end,
	},
	{
		"rebelot/kanagawa.nvim",
		name = "kanagawa_lotus",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme kanagawa-lotus]])
		end,
	},
	{
		"ellisonleao/gruvbox.nvim",
		name = "gruvbox_dark",
		config = function()
			vim.o.background = "dark"
			require("gruvbox").setup({ contrast = "" })
			vim.cmd([[colorscheme gruvbox]])
		end,
	},
	{
		"ellisonleao/gruvbox.nvim",
		name = "gruvbox_dark_hard",
		config = function()
			vim.o.background = "dark"
			require("gruvbox").setup({ contrast = "hard" })
			vim.cmd([[colorscheme gruvbox]])
		end,
	},
	{
		"ellisonleao/gruvbox.nvim",
		name = "gruvbox_dark_soft",
		config = function()
			vim.o.background = "dark"
			require("gruvbox").setup({ contrast = "soft" })
			vim.cmd([[colorscheme gruvbox]])
		end,
	},
	{
		"ellisonleao/gruvbox.nvim",
		name = "gruvbox_light",
		config = function()
			vim.o.background = "light"
			require("gruvbox").setup({ contrast = "" })
			vim.cmd([[colorscheme gruvbox]])
		end,
	},
	{
		"ellisonleao/gruvbox.nvim",
		name = "gruvbox_light_hard",
		config = function()
			vim.o.background = "light"
			require("gruvbox").setup({ contrast = "hard" })
			vim.cmd([[colorscheme gruvbox]])
		end,
	},
	{
		"ellisonleao/gruvbox.nvim",
		name = "gruvbox_light_soft",
		config = function()
			vim.o.background = "light"
			require("gruvbox").setup({ contrast = "soft" })
			vim.cmd([[colorscheme gruvbox]])
		end,
	},
	{
		"Mofiqul/dracula.nvim",
		name = "dracula",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme dracula]])
		end,
	},
	{
		"Mofiqul/dracula.nvim",
		name = "dracula_soft",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme dracula-soft]])
		end,
	},
	{
		"Mofiqul/vscode.nvim",
		name = "vscode_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme vscode]])
		end,
	},
	{
		"Mofiqul/vscode.nvim",
		name = "vscode_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme vscode]])
		end,
	},
	{
		"maxmx03/solarized.nvim",
		name = "solarized_winter_dark",
		config = function()
			vim.o.background = "dark"
			require("solarized").setup({
				variant = "winter",
				appearance = "dark",
			})
			vim.cmd([[colorscheme solarized]])
		end,
	},
	{
		"maxmx03/solarized.nvim",
		name = "solarized_winter_light",
		config = function()
			vim.o.background = "light"
			require("solarized").setup({
				variant = "winter",
				appearance = "light",
			})
			vim.cmd([[colorscheme solarized]])
		end,
	},
	{
		"maxmx03/solarized.nvim",
		name = "solarized_spring_dark",
		config = function()
			vim.o.background = "dark"
			require("solarized").setup({
				variant = "spring",
				appearance = "dark",
			})
			vim.cmd([[colorscheme solarized]])
		end,
	},
	{
		"maxmx03/solarized.nvim",
		name = "solarized_spring_light",
		config = function()
			vim.o.background = "light"
			require("solarized").setup({
				variant = "spring",
				appearance = "light",
			})
			vim.cmd([[colorscheme solarized]])
		end,
	},
	{
		"maxmx03/solarized.nvim",
		name = "solarized_summer_dark",
		config = function()
			vim.o.background = "dark"
			require("solarized").setup({
				variant = "summer",
				appearance = "dark",
			})
			vim.cmd([[colorscheme solarized]])
		end,
	},
	{
		"maxmx03/solarized.nvim",
		name = "solarized_summer_light",
		config = function()
			vim.o.background = "light"
			require("solarized").setup({
				variant = "summer",
				appearance = "light",
			})
			vim.cmd([[colorscheme solarized]])
		end,
	},
	{
		"maxmx03/solarized.nvim",
		name = "solarized_autumn_dark",
		config = function()
			vim.o.background = "dark"
			require("solarized").setup({
				variant = "autumn",
				appearance = "dark",
			})
			vim.cmd([[colorscheme solarized]])
		end,
	},
	{
		"maxmx03/solarized.nvim",
		name = "solarized_autumn_light",
		config = function()
			vim.o.background = "light"
			require("solarized").setup({
				variant = "autumn",
				appearance = "light",
			})
			vim.cmd([[colorscheme solarized]])
		end,
	},
	{
		"marko-cerovac/material.nvim",
		name = "material_darker",
		config = function()
			vim.o.background = "dark"
			vim.g.material_style = "darker"
			require("material").setup({ async_loading = false })
			vim.cmd([[colorscheme material-darker]])
		end,
	},
	{
		"marko-cerovac/material.nvim",
		name = "material_lighter",
		config = function()
			vim.o.background = "light"
			vim.g.material_style = "lighter"
			require("material").setup({ async_loading = false })
			vim.cmd([[colorscheme material-lighter]])
		end,
	},
	{
		"marko-cerovac/material.nvim",
		name = "material_oceanic",
		config = function()
			vim.o.background = "dark"
			vim.g.material_style = "oceanic"
			require("material").setup({ async_loading = false })
			vim.cmd([[colorscheme material-oceanic]])
		end,
	},
	{
		"marko-cerovac/material.nvim",
		name = "material_palenight",
		config = function()
			vim.o.background = "dark"
			vim.g.material_style = "palenight"
			require("material").setup({ async_loading = false })
			vim.cmd([[colorscheme material-palenight]])
		end,
	},
	{
		"marko-cerovac/material.nvim",
		name = "material_deep_ocean",
		config = function()
			vim.o.background = "dark"
			vim.g.material_style = "deep ocean"
			require("material").setup({ async_loading = false })
			vim.cmd([[colorscheme material-deep-ocean]])
		end,
	},
	{
		"shaunsingh/nord.nvim",
		name = "nord",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme nord]])
		end,
	},
	{
		"navarasu/onedark.nvim",
		name = "onedark_darker",
		config = function()
			vim.o.background = "dark"
			require("onedark").setup({ style = "darker" })
			require("onedark").load()
		end,
	},
	{
		"navarasu/onedark.nvim",
		name = "onedark_cool",
		config = function()
			vim.o.background = "dark"
			require("onedark").setup({ style = "cool" })
			require("onedark").load()
		end,
	},
	{
		"navarasu/onedark.nvim",
		name = "onedark_deep",
		config = function()
			vim.o.background = "dark"
			require("onedark").setup({ style = "deep" })
			require("onedark").load()
		end,
	},
	{
		"navarasu/onedark.nvim",
		name = "onedark_warm",
		config = function()
			vim.o.background = "dark"
			require("onedark").setup({ style = "warn" })
			require("onedark").load()
		end,
	},
	{
		"navarasu/onedark.nvim",
		name = "onedark_warmer",
		config = function()
			vim.o.background = "dark"
			require("onedark").setup({ style = "warmer" })
			require("onedark").load()
		end,
	},
	{
		"navarasu/onedark.nvim",
		name = "onedark_light",
		config = function()
			vim.o.background = "light"
			require("onedark").setup({ style = "light" })
			require("onedark").load()
		end,
	},
	{
		"olimorris/onedarkpro.nvim",
		name = "onedark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme onedark]])
		end,
	},
	{
		"olimorris/onedarkpro.nvim",
		name = "onelight",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme onelight]])
		end,
	},
	{
		"olimorris/onedarkpro.nvim",
		name = "onedarkpro_vivid",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme onedark_vivid]])
		end,
	},
	{
		"olimorris/onedarkpro.nvim",
		name = "onedarkpro_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme onedark_dark]])
		end,
	},
	{
		"EdenEast/nightfox.nvim",
		name = "nightfox",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme nightfox]])
		end,
	},
	{
		"EdenEast/nightfox.nvim",
		name = "dayfox",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme dayfox]])
		end,
	},
	{
		"EdenEast/nightfox.nvim",
		name = "duskfox",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme duskfox]])
		end,
	},
	{
		"EdenEast/nightfox.nvim",
		name = "dawnfox",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme dawnfox]])
		end,
	},
	{
		"EdenEast/nightfox.nvim",
		name = "carbonfox",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme carbonfox]])
		end,
	},
	{
		"EdenEast/nightfox.nvim",
		name = "terafox",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme terafox]])
		end,
	},
	{
		"rose-pine/neovim",
		name = "rosepine_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme rose-pine]])
		end,
	},
	{
		"rose-pine/neovim",
		name = "rosepine_moon",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme rose-pine-moon]])
		end,
	},
	{
		"rose-pine/neovim",
		name = "rosepine_dawn",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme rose-pine-dawn]])
		end,
	},
	{
		"neanias/everforest-nvim",
		name = "everforest_dark",
		config = function()
			vim.o.background = "dark"
			require("everforest").setup({ background = "medium" })
			vim.cmd([[colorscheme everforest]])
		end,
	},
	{
		"neanias/everforest-nvim",
		name = "everforest_light",
		config = function()
			vim.o.background = "light"
			require("everforest").setup({ background = "medium" })
			vim.cmd([[colorscheme everforest]])
		end,
	},
	{
		"sainnhe/edge",
		name = "edge_dark",
		config = function()
			vim.o.background = "dark"
			vim.g.edge_style = "default"
			vim.cmd([[colorscheme edge]])
		end,
	},
	{
		"sainnhe/edge",
		name = "edge_light",
		config = function()
			vim.o.background = "light"
			vim.g.edge_style = "default"
			vim.cmd([[colorscheme edge]])
		end,
	},
	{
		"sainnhe/edge",
		name = "edge_aura",
		config = function()
			vim.o.background = "dark"
			vim.g.edge_style = "aura"
			vim.cmd([[colorscheme edge]])
		end,
	},
	{
		"sainnhe/edge",
		name = "edge_neon",
		config = function()
			vim.o.background = "dark"
			vim.g.edge_style = "neon"
			vim.cmd([[colorscheme edge]])
		end,
	},
	{
		"miikanissi/modus-themes.nvim",
		name = "modus_operandi",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme modus_operandi]])
		end,
	},
	{
		"miikanissi/modus-themes.nvim",
		name = "modus_vivendi",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme modus_vivendi]])
		end,
	},
	{
		"glepnir/zephyr-nvim",
		name = "zephyr_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme zephyr]])
		end,
	},
	{
		"svrana/neosolarized.nvim",
		name = "neosolarized_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme neosolarized]])
		end,
		dependencies = { "tjdevries/colorbuddy.nvim" },
	},
	{
		"svrana/neosolarized.nvim",
		name = "neosolarized_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme neosolarized]])
		end,
		dependencies = { "tjdevries/colorbuddy.nvim" },
	},
	{
		"loctvl842/monokai-pro.nvim",
		name = "monokai_pro_dark",
		config = function()
			vim.o.background = "dark"
			require("monokai-pro").setup({ filter = "pro" })
			vim.cmd([[colorscheme monokai-pro]])
		end,
	},
	{
		"loctvl842/monokai-pro.nvim",
		name = "monokai_pro_machine",
		config = function()
			vim.o.background = "dark"
			require("monokai-pro").setup({ filter = "machine" })
			vim.cmd([[colorscheme monokai-pro]])
		end,
	},
	{
		"loctvl842/monokai-pro.nvim",
		name = "monokai_pro_ristretto",
		config = function()
			vim.o.background = "dark"
			require("monokai-pro").setup({ filter = "ristretto" })
			vim.cmd([[colorscheme monokai-pro]])
		end,
	},
	{
		"loctvl842/monokai-pro.nvim",
		name = "monokai_pro_spectrum",
		config = function()
			vim.o.background = "dark"
			require("monokai-pro").setup({ filter = "spectrum" })
			vim.cmd([[colorscheme monokai-pro]])
		end,
	},
	{
		"ribru17/bamboo.nvim",
		name = "bamboo_light",
		config = function()
			vim.o.background = "light"
			require("bamboo").setup({ style = "light" })
			vim.cmd([[colorscheme bamboo]])
		end,
	},
	{
		"ribru17/bamboo.nvim",
		name = "bamboo_vulgaris",
		config = function()
			vim.o.background = "dark"
			require("bamboo").setup({ style = "vulgaris" })
			vim.cmd([[colorscheme bamboo]])
		end,
	},
	{
		"ribru17/bamboo.nvim",
		name = "bamboo_multiplex",
		config = function()
			vim.o.background = "dark"
			require("bamboo").setup({ style = "multiplex" })
			vim.cmd([[colorscheme bamboo]])
		end,
	},
	{
		"daltonmenezes/aura-theme",
		name = "aura_dark",
		config = function(plugin)
			vim.opt.rtp:append(plugin.dir .. "/packages/neovim")
			vim.o.background = "dark"
			vim.cmd([[colorscheme aura-dark]])
		end,
	},
	{
		"daltonmenezes/aura-theme",
		name = "aura_dark_soft_text",
		config = function(plugin)
			vim.opt.rtp:append(plugin.dir .. "/packages/neovim")
			vim.o.background = "dark"
			vim.cmd([[colorscheme aura-dark-soft-text]])
		end,
	},
	{
		"daltonmenezes/aura-theme",
		name = "aura_soft_dark",
		config = function(plugin)
			vim.opt.rtp:append(plugin.dir .. "/packages/neovim")
			vim.o.background = "dark"
			vim.cmd([[colorscheme aura-soft-dark]])
		end,
	},
	{
		"daltonmenezes/aura-theme",
		name = "aura_soft_dark_soft_text",
		config = function(plugin)
			vim.opt.rtp:append(plugin.dir .. "/packages/neovim")
			vim.o.background = "dark"
			vim.cmd([[colorscheme aura-soft-dark-soft-text]])
		end,
	},
	{
		"bluz71/vim-moonfly-colors",
		name = "moonfly",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme moonfly]])
		end,
	},
	{
		"scottmckendry/cyberdream.nvim",
		name = "cyberdream_dark",
		config = function()
			vim.o.background = "dark"
			require("cyberdream").setup({ variant = "dark" })
			vim.cmd([[colorscheme cyberdream]])
		end,
	},
	{
		"scottmckendry/cyberdream.nvim",
		name = "cyberdream_light",
		config = function()
			vim.o.background = "light"
			require("cyberdream").setup({ variant = "light" })
			vim.cmd([[colorscheme cyberdream-light]])
		end,
	},
	{
		"uloco/bluloco.nvim",
		name = "bluloco_dark",
		config = function()
			vim.o.background = "dark"
			require("bluloco").setup({ style = "dark" })
			vim.cmd([[colorscheme bluloco-dark]])
		end,
		dependencies = { "rktjmp/lush.nvim" },
	},
	{
		"uloco/bluloco.nvim",
		name = "bluloco_light",
		config = function()
			vim.o.background = "light"
			require("bluloco").setup({ style = "light" })
			vim.cmd([[colorscheme bluloco-light]])
		end,
		dependencies = { "rktjmp/lush.nvim" },
	},
	{
		"martinsione/darkplus.nvim",
		name = "darkplus",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme darkplus]])
		end,
	},
	{
		"kepano/flexoki-neovim",
		name = "flexoki_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme flexoki-dark]])
		end,
	},
	{
		"nomis51/nvim-xcode-theme",
		name = "xcode_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme xcodedark]])
		end,
	},
	{
		"nomis51/nvim-xcode-theme",
		name = "xcode_dark_hc",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme xcodedarkhc]])
		end,
	},
	{
		"nomis51/nvim-xcode-theme",
		name = "xcode_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme xcodelight]])
		end,
	},
	{
		"nomis51/nvim-xcode-theme",
		name = "xcode_light_hc",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme xcodelighthc]])
		end,
	},
	{
		"nomis51/nvim-xcode-theme",
		name = "xcode_wwdc",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme xcodewwdc]])
		end,
	},
	{
		"kepano/flexoki-neovim",
		name = "flexoki_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme flexoki-light]])
		end,
	},
}

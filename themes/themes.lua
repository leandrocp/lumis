return {
	{
		name = "neovim_dark",
		colorscheme = "default",
		appearance = "dark",
		plugin = {},
	},
	{
		name = "neovim_light",
		colorscheme = "default",
		appearance = "light",
		plugin = {},
	},
	{
		name = "ayu_dark",
		colorscheme = "ayu-dark",
		appearance = "dark",
		plugin = {
			"Shatur/neovim-ayu",
		},
	},
	{
		name = "ayu_light",
		colorscheme = "ayu-light",
		appearance = "light",
		plugin = {
			"Shatur/neovim-ayu",
		},
	},
	{
		name = "ayu_mirage",
		colorscheme = "ayu-mirage",
		appearance = "dark",
		plugin = {
			"Shatur/neovim-ayu",
		},
	},
	{
		name = "nordic",
		colorscheme = "nordic",
		appearance = "dark",
		plugin = {
			"AlexvZyl/nordic.nvim",
		},
		before = function()
			require("nordic").load()
		end,
	},
	{
		name = "melange_dark",
		colorscheme = "melange",
		appearance = "dark",
		plugin = {
			"savq/melange-nvim",
		},
	},
	{
		name = "melange_light",
		colorscheme = "melange",
		appearance = "light",
		plugin = {
			"savq/melange-nvim",
		},
	},
	{
		name = "nightfly",
		colorscheme = "nightfly",
		appearance = "dark",
		plugin = {
			"bluz71/vim-nightfly-colors",
			name = "nightfly",
		},
	},
	{
		name = "tokyonight_night",
		colorscheme = "tokyonight-night",
		appearance = "dark",
		plugin = {
			"folke/tokyonight.nvim",
		},
	},
	{
		name = "tokyonight_moon",
		colorscheme = "tokyonight-moon",
		appearance = "dark",
		plugin = {
			"folke/tokyonight.nvim",
		},
	},
	{
		name = "tokyonight_storm",
		colorscheme = "tokyonight-storm",
		appearance = "dark",
		plugin = {
			"folke/tokyonight.nvim",
		},
	},
	{
		name = "tokyonight_day",
		colorscheme = "tokyonight-day",
		appearance = "light",
		plugin = {
			"folke/tokyonight.nvim",
		},
	},
	{
		name = "catppuccin_frappe",
		colorscheme = "catppuccin-frappe",
		appearance = "dark",
		plugin = {
			"catppuccin/nvim",
			name = "catppuccin",
		},
	},
	{
		name = "catppuccin_macchiato",
		colorscheme = "catppuccin-macchiato",
		appearance = "dark",
		plugin = {
			"catppuccin/nvim",
			name = "catppuccin",
		},
	},
	{
		name = "catppuccin_mocha",
		colorscheme = "catppuccin-mocha",
		appearance = "dark",
		plugin = {
			"catppuccin/nvim",
			name = "catppuccin",
		},
	},
	{
		name = "catppuccin_latte",
		colorscheme = "catppuccin-latte",
		appearance = "light",
		plugin = {
			"catppuccin/nvim",
			name = "catppuccin",
		},
	},
	{
		name = "github_dark",
		colorscheme = "github_dark",
		appearance = "dark",
		plugin = {
			"projekt0n/github-nvim-theme",
			name = "github-theme",
		},
	},
	{
		name = "github_dark_default",
		colorscheme = "github_dark_default",
		appearance = "dark",
		plugin = {
			"projekt0n/github-nvim-theme",
			name = "github-theme",
		},
	},
	{
		name = "github_dark_dimmed",
		colorscheme = "github_dark_dimmed",
		appearance = "dark",
		plugin = {
			"projekt0n/github-nvim-theme",
			name = "github-theme",
		},
	},
	{
		name = "github_dark_high_contrast",
		colorscheme = "github_dark_high_contrast",
		appearance = "dark",
		plugin = {
			"projekt0n/github-nvim-theme",
			name = "github-theme",
		},
	},
	{
		name = "github_dark_colorblind",
		colorscheme = "github_dark_colorblind",
		appearance = "dark",
		plugin = {
			"projekt0n/github-nvim-theme",
			name = "github-theme",
		},
	},
	{
		name = "github_dark_tritanopia",
		colorscheme = "github_dark_tritanopia",
		appearance = "dark",
		plugin = {
			"projekt0n/github-nvim-theme",
			name = "github-theme",
		},
	},
	{
		name = "github_light",
		colorscheme = "github_light",
		appearance = "light",
		plugin = {
			"projekt0n/github-nvim-theme",
			name = "github-theme",
		},
	},
	{
		name = "github_light_default",
		colorscheme = "github_light_default",
		appearance = "light",
		plugin = {
			"projekt0n/github-nvim-theme",
			name = "github-theme",
		},
	},
	{
		name = "github_light_high_contrast",
		colorscheme = "github_light_high_contrast",
		appearance = "light",
		plugin = {
			"projekt0n/github-nvim-theme",
			name = "github-theme",
		},
	},
	{
		name = "github_light_colorblind",
		colorscheme = "github_light_colorblind",
		appearance = "light",
		plugin = {
			"projekt0n/github-nvim-theme",
			name = "github-theme",
		},
	},
	{
		name = "github_light_tritanopia",
		colorscheme = "github_light_tritanopia",
		appearance = "light",
		plugin = {
			"projekt0n/github-nvim-theme",
			name = "github-theme",
		},
	},
	{
		name = "kanagawa_wave",
		colorscheme = "kanagawa-wave",
		appearance = "dark",
		plugin = {
			"rebelot/kanagawa.nvim",
		},
	},
	{
		name = "kanagawa_dragon",
		colorscheme = "kanagawa-dragon",
		appearance = "dark",
		plugin = {
			"rebelot/kanagawa.nvim",
		},
	},
	{
		name = "kanagawa_lotus",
		colorscheme = "kanagawa-lotus",
		appearance = "light",
		plugin = {
			"rebelot/kanagawa.nvim",
		},
	},
	{
		name = "gruvbox_dark",
		colorscheme = "gruvbox",
		appearance = "dark",
		plugin = {
			"ellisonleao/gruvbox.nvim",
		},
		before = function()
			require("gruvbox").setup({ contrast = "" })
		end,
	},
	{
		name = "gruvbox_dark_hard",
		colorscheme = "gruvbox",
		appearance = "dark",
		plugin = {
			"ellisonleao/gruvbox.nvim",
		},
		before = function()
			require("gruvbox").setup({ contrast = "hard" })
		end,
	},
	{
		name = "gruvbox_dark_soft",
		colorscheme = "gruvbox",
		appearance = "dark",
		plugin = {
			"ellisonleao/gruvbox.nvim",
		},
		before = function()
			require("gruvbox").setup({ contrast = "soft" })
		end,
	},
	{
		name = "gruvbox_light",
		colorscheme = "gruvbox",
		appearance = "light",
		plugin = {
			"ellisonleao/gruvbox.nvim",
		},
		before = function()
			require("gruvbox").setup({ contrast = "" })
		end,
	},
	{
		name = "gruvbox_light_hard",
		colorscheme = "gruvbox",
		appearance = "light",
		plugin = {
			"ellisonleao/gruvbox.nvim",
		},
		before = function()
			require("gruvbox").setup({ contrast = "hard" })
		end,
	},
	{
		name = "gruvbox_light_soft",
		colorscheme = "gruvbox",
		appearance = "light",
		plugin = {
			"ellisonleao/gruvbox.nvim",
		},
		before = function()
			require("gruvbox").setup({ contrast = "soft" })
		end,
	},
	{
		name = "dracula",
		colorscheme = "dracula",
		appearance = "dark",
		plugin = {
			"Mofiqul/dracula.nvim",
		},
	},
	{
		name = "dracula_soft",
		colorscheme = "dracula-soft",
		appearance = "dark",
		plugin = {
			"Mofiqul/dracula.nvim",
		},
	},
	{
		name = "vscode_dark",
		colorscheme = "vscode",
		appearance = "dark",
		plugin = {
			"Mofiqul/vscode.nvim",
		},
	},
	{
		name = "vscode_light",
		colorscheme = "vscode",
		appearance = "light",
		plugin = {
			"Mofiqul/vscode.nvim",
		},
	},
	{
		name = "solarized_winter_dark",
		colorscheme = "solarized",
		appearance = "dark",
		plugin = {
			"maxmx03/solarized.nvim",
		},
		before = function()
			require("solarized").setup({
				variant = "winter",
				appearance = "dark",
			})
		end,
	},
	{
		name = "solarized_winter_light",
		colorscheme = "solarized",
		appearance = "light",
		plugin = {
			"maxmx03/solarized.nvim",
		},
		before = function()
			require("solarized").setup({
				variant = "winter",
				appearance = "light",
			})
		end,
	},
	{
		name = "solarized_spring_dark",
		colorscheme = "solarized",
		appearance = "dark",
		plugin = {
			"maxmx03/solarized.nvim",
		},
		before = function()
			require("solarized").setup({
				variant = "spring",
				appearance = "dark",
			})
		end,
	},
	{
		name = "solarized_spring_light",
		colorscheme = "solarized",
		appearance = "light",
		plugin = {
			"maxmx03/solarized.nvim",
		},
		before = function()
			require("solarized").setup({
				variant = "spring",
				appearance = "light",
			})
		end,
	},
	{
		name = "solarized_summer_dark",
		colorscheme = "solarized",
		appearance = "dark",
		plugin = {
			"maxmx03/solarized.nvim",
		},
		before = function()
			require("solarized").setup({
				variant = "summer",
				appearance = "dark",
			})
		end,
	},
	{
		name = "solarized_summer_light",
		colorscheme = "solarized",
		appearance = "light",
		plugin = {
			"maxmx03/solarized.nvim",
		},
		before = function()
			require("solarized").setup({
				variant = "summer",
				appearance = "light",
			})
		end,
	},
	{
		name = "solarized_autumn_dark",
		colorscheme = "solarized",
		appearance = "dark",
		plugin = {
			"maxmx03/solarized.nvim",
		},
		before = function()
			require("solarized").setup({
				variant = "autumn",
				appearance = "dark",
			})
		end,
	},
	{
		name = "solarized_autumn_light",
		colorscheme = "solarized",
		appearance = "light",
		plugin = {
			"maxmx03/solarized.nvim",
		},
		before = function()
			require("solarized").setup({
				variant = "autumn",
				appearance = "light",
			})
		end,
	},
	{
		name = "material_darker",
		colorscheme = "material-darker",
		appearance = "dark",
		plugin = {
			"marko-cerovac/material.nvim",
		},
		before = function()
			vim.g.material_style = "darker"
			require("material").setup({ async_loading = false })
		end,
	},
	{
		name = "material_lighter",
		colorscheme = "material-lighter",
		appearance = "light",
		plugin = {
			"marko-cerovac/material.nvim",
		},
		before = function()
			vim.g.material_style = "lighter"
			require("material").setup({ async_loading = false })
		end,
	},
	{
		name = "material_oceanic",
		colorscheme = "material-oceanic",
		appearance = "dark",
		plugin = {
			"marko-cerovac/material.nvim",
		},
		before = function()
			vim.g.material_style = "oceanic"
			require("material").setup({ async_loading = false })
		end,
	},
	{
		name = "material_palenight",
		colorscheme = "material-palenight",
		appearance = "dark",
		plugin = {
			"marko-cerovac/material.nvim",
		},
		before = function()
			vim.g.material_style = "palenight"
			require("material").setup({ async_loading = false })
		end,
	},
	{
		name = "material_deep_ocean",
		colorscheme = "material-deep-ocean",
		appearance = "dark",
		plugin = {
			"marko-cerovac/material.nvim",
		},
		before = function()
			vim.g.material_style = "deep ocean"
			require("material").setup({ async_loading = false })
		end,
	},
	{
		name = "nord",
		colorscheme = "nord",
		appearance = "dark",
		plugin = {
			"shaunsingh/nord.nvim",
		},
	},
	{
		name = "onedark_darker",
		colorscheme = "onedark",
		appearance = "dark",
		plugin = {
			"navarasu/onedark.nvim",
		},
		before = function()
			require("onedark").setup({ style = "darker" })
		end,
	},
	-- {
	-- 	name = "onedark_cool",
	-- 	colorscheme = "onedark",
	-- 	appearance = "dark",
	-- 	plugin = {
	-- 		"navarasu/onedark.nvim",
	-- 	},
	-- 	before = function()
	-- 		require("onedark").setup({ style = "cool" })
	-- 	end,
	-- },
	-- {
	-- 	name = "onedark_deep",
	-- 	colorscheme = "onedark",
	-- 	appearance = "dark",
	-- 	plugin = {
	-- 		"navarasu/onedark.nvim",
	-- 	},
	-- 	before = function()
	-- 		require("onedark").setup({ style = "deep" })
	-- 	end,
	-- },
	-- {
	-- 	name = "onedark_warm",
	-- 	colorscheme = "onedark",
	-- 	appearance = "dark",
	-- 	plugin = {
	-- 		"navarasu/onedark.nvim",
	-- 	},
	-- 	before = function()
	-- 		require("onedark").setup({ style = "warm" })
	-- 	end,
	-- },
	-- {
	-- 	name = "onedark_warmer",
	-- 	colorscheme = "onedark",
	-- 	appearance = "dark",
	-- 	plugin = {
	-- 		"navarasu/onedark.nvim",
	-- 	},
	-- 	before = function()
	-- 		require("onedark").setup({ style = "warmer" })
	-- 	end,
	-- },
	-- {
	-- 	name = "onedark_light",
	-- 	colorscheme = "onedark",
	-- 	appearance = "light",
	-- 	plugin = {
	-- 		"navarasu/onedark.nvim",
	-- 	},
	-- 	before = function()
	-- 		require("onedark").setup({ style = "light" })
	-- 	end,
	-- },
	{
		name = "onedark",
		colorscheme = "onedark",
		appearance = "dark",
		plugin = {
			"olimorris/onedarkpro.nvim",
		},
	},
	{
		name = "onelight",
		colorscheme = "onelight",
		appearance = "light",
		plugin = {
			"olimorris/onedarkpro.nvim",
		},
	},
	{
		name = "onedarkpro_vivid",
		colorscheme = "onedark_vivid",
		appearance = "dark",
		plugin = {
			"olimorris/onedarkpro.nvim",
		},
	},
	{
		name = "onedarkpro_dark",
		colorscheme = "onedark_dark",
		appearance = "dark",
		plugin = {
			"olimorris/onedarkpro.nvim",
		},
	},
	{
		name = "nightfox",
		colorscheme = "nightfox",
		appearance = "dark",
		plugin = {
			"EdenEast/nightfox.nvim",
		},
	},
	{
		name = "dayfox",
		colorscheme = "dayfox",
		appearance = "light",
		plugin = {
			"EdenEast/nightfox.nvim",
		},
	},
	{
		name = "duskfox",
		colorscheme = "duskfox",
		appearance = "dark",
		plugin = {
			"EdenEast/nightfox.nvim",
		},
	},
	{
		name = "dawnfox",
		colorscheme = "dawnfox",
		appearance = "light",
		plugin = {
			"EdenEast/nightfox.nvim",
		},
	},
	{
		name = "carbonfox",
		colorscheme = "carbonfox",
		appearance = "dark",
		plugin = {
			"EdenEast/nightfox.nvim",
		},
	},
	{
		name = "terafox",
		colorscheme = "terafox",
		appearance = "dark",
		plugin = {
			"EdenEast/nightfox.nvim",
		},
	},
	{
		name = "rosepine_dark",
		colorscheme = "rose-pine",
		appearance = "dark",
		plugin = {
			"rose-pine/neovim",
		},
	},
	{
		name = "rosepine_moon",
		colorscheme = "rose-pine-moon",
		appearance = "dark",
		plugin = {
			"rose-pine/neovim",
			name = "rose-pine",
		},
	},
	{
		name = "rosepine_dawn",
		colorscheme = "rose-pine-dawn",
		appearance = "light",
		plugin = {
			"rose-pine/neovim",
			name = "rose-pine",
		},
	},
	{
		name = "everforest_dark",
		colorscheme = "everforest",
		appearance = "dark",
		plugin = {
			"neanias/everforest-nvim",
		},
		before = function()
			require("everforest").setup({ background = "medium" })
		end,
	},
	{
		name = "everforest_light",
		colorscheme = "everforest",
		appearance = "light",
		plugin = {
			"neanias/everforest-nvim",
		},
		before = function()
			require("everforest").setup({ background = "medium" })
		end,
	},
	{
		name = "edge_dark",
		colorscheme = "edge",
		appearance = "dark",
		plugin = {
			"sainnhe/edge",
		},
		before = function()
			vim.g.edge_style = "default"
		end,
	},
	{
		name = "edge_light",
		colorscheme = "edge",
		appearance = "light",
		plugin = {
			"sainnhe/edge",
		},
		before = function()
			vim.g.edge_style = "default"
		end,
	},
	{
		name = "edge_aura",
		colorscheme = "edge",
		appearance = "dark",
		plugin = {
			"sainnhe/edge",
		},
		before = function()
			vim.g.edge_style = "aura"
		end,
	},
	{
		name = "edge_neon",
		colorscheme = "edge",
		appearance = "dark",
		plugin = {
			"sainnhe/edge",
		},
		before = function()
			vim.g.edge_style = "neon"
		end,
	},
	{
		name = "modus_operandi",
		colorscheme = "modus_operandi",
		appearance = "light",
		plugin = {
			"miikanissi/modus-themes.nvim",
		},
	},
	{
		name = "modus_vivendi",
		colorscheme = "modus_vivendi",
		appearance = "dark",
		plugin = {
			"miikanissi/modus-themes.nvim",
		},
	},
	{
		name = "zephyr_dark",
		colorscheme = "zephyr",
		appearance = "dark",
		plugin = {
			"glepnir/zephyr-nvim",
		},
	},
	{
		name = "neosolarized_dark",
		colorscheme = "neosolarized",
		appearance = "dark",
		plugin = {
			"svrana/neosolarized.nvim",
			dependencies = { "tjdevries/colorbuddy.nvim" },
		},
	},
	{
		name = "neosolarized_light",
		colorscheme = "neosolarized",
		appearance = "light",
		plugin = {
			"svrana/neosolarized.nvim",
			dependencies = { "tjdevries/colorbuddy.nvim" },
		},
	},
	{
		name = "monokai_pro_dark",
		colorscheme = "monokai-pro",
		appearance = "dark",
		plugin = {
			"loctvl842/monokai-pro.nvim",
		},
		before = function()
			require("monokai-pro").setup({ filter = "pro" })
		end,
	},
	{
		name = "monokai_pro_machine",
		colorscheme = "monokai-pro",
		appearance = "dark",
		plugin = {
			"loctvl842/monokai-pro.nvim",
		},
		before = function()
			require("monokai-pro").setup({ filter = "machine" })
		end,
	},
	{
		name = "monokai_pro_ristretto",
		colorscheme = "monokai-pro",
		appearance = "dark",
		plugin = {
			"loctvl842/monokai-pro.nvim",
		},
		before = function()
			require("monokai-pro").setup({ filter = "ristretto" })
		end,
	},
	{
		name = "monokai_pro_spectrum",
		colorscheme = "monokai-pro",
		appearance = "dark",
		plugin = {
			"loctvl842/monokai-pro.nvim",
		},
		before = function()
			require("monokai-pro").setup({ filter = "spectrum" })
		end,
	},
	{
		name = "bamboo_light",
		colorscheme = "bamboo",
		appearance = "light",
		plugin = {
			"ribru17/bamboo.nvim",
		},
		before = function()
			require("bamboo").setup({ style = "light" })
		end,
	},
	{
		name = "bamboo_vulgaris",
		colorscheme = "bamboo",
		appearance = "dark",
		plugin = {
			"ribru17/bamboo.nvim",
		},
		before = function()
			require("bamboo").setup({ style = "vulgaris" })
		end,
	},
	{
		name = "bamboo_multiplex",
		colorscheme = "bamboo",
		appearance = "dark",
		plugin = {
			"ribru17/bamboo.nvim",
		},
		before = function()
			require("bamboo").setup({ style = "multiplex" })
		end,
	},
	{
		name = "aura_dark",
		colorscheme = "aura-dark",
		appearance = "dark",
		plugin = {
			"daltonmenezes/aura-theme",
			name = "aura",
			config = function(plugin)
				vim.opt.rtp:append(plugin.dir .. "/packages/neovim")
			end,
		},
	},
	{
		name = "aura_dark_soft_text",
		colorscheme = "aura-dark-soft-text",
		appearance = "dark",
		plugin = {
			"daltonmenezes/aura-theme",
			name = "aura",
			config = function(plugin)
				vim.opt.rtp:append(plugin.dir .. "/packages/neovim")
			end,
		},
	},
	{
		name = "aura_soft_dark",
		colorscheme = "aura-soft-dark",
		appearance = "dark",
		plugin = {
			"daltonmenezes/aura-theme",
			name = "aura",
			config = function(plugin)
				vim.opt.rtp:append(plugin.dir .. "/packages/neovim")
			end,
		},
	},
	{
		name = "aura_soft_dark_soft_text",
		colorscheme = "aura-soft-dark-soft-text",
		appearance = "dark",
		plugin = {
			"daltonmenezes/aura-theme",
			name = "aura",
			config = function(plugin)
				vim.opt.rtp:append(plugin.dir .. "/packages/neovim")
			end,
		},
	},
	{
		name = "moonfly",
		colorscheme = "moonfly",
		appearance = "dark",
		plugin = {
			"bluz71/vim-moonfly-colors",
			name = "moonfly",
		},
	},
	{
		name = "cyberdream_dark",
		colorscheme = "cyberdream",
		appearance = "dark",
		plugin = {
			"scottmckendry/cyberdream.nvim",
		},
		before = function()
			require("cyberdream").setup({ variant = "dark" })
		end,
	},
	{
		name = "cyberdream_light",
		colorscheme = "cyberdream-light",
		appearance = "light",
		plugin = {
			"scottmckendry/cyberdream.nvim",
		},
		before = function()
			require("cyberdream").setup({ variant = "light" })
		end,
	},
	{
		name = "bluloco_dark",
		colorscheme = "bluloco-dark",
		appearance = "dark",
		plugin = {
			"uloco/bluloco.nvim",
			dependencies = { "rktjmp/lush.nvim" },
		},
		before = function()
			require("bluloco").setup({ style = "dark" })
		end,
	},
	{
		name = "bluloco_light",
		colorscheme = "bluloco-light",
		appearance = "light",
		plugin = {
			"uloco/bluloco.nvim",
			dependencies = { "rktjmp/lush.nvim" },
		},
		before = function()
			require("bluloco").setup({ style = "light" })
		end,
	},
	{
		name = "darkplus",
		colorscheme = "darkplus",
		appearance = "dark",
		plugin = {
			"martinsione/darkplus.nvim",
		},
	},
	{
		name = "flexoki_dark",
		colorscheme = "flexoki-dark",
		appearance = "dark",
		plugin = {
			"kepano/flexoki-neovim",
			name = "flexoki",
		},
	},
	{
		name = "xcode_dark",
		colorscheme = "xcodedark",
		appearance = "dark",
		plugin = {
			"nomis51/nvim-xcode-theme",
		},
	},
	{
		name = "xcode_dark_hc",
		colorscheme = "xcodedarkhc",
		appearance = "dark",
		plugin = {
			"nomis51/nvim-xcode-theme",
		},
	},
	{
		name = "xcode_light",
		colorscheme = "xcodelight",
		appearance = "light",
		plugin = {
			"nomis51/nvim-xcode-theme",
		},
	},
	{
		name = "xcode_light_hc",
		colorscheme = "xcodelighthc",
		appearance = "light",
		plugin = {
			"nomis51/nvim-xcode-theme",
		},
	},
	{
		name = "xcode_wwdc",
		colorscheme = "xcodewwdc",
		appearance = "dark",
		plugin = {
			"nomis51/nvim-xcode-theme",
		},
	},
	{
		name = "flexoki_light",
		colorscheme = "flexoki-light",
		appearance = "light",
		plugin = {
			"kepano/flexoki-neovim",
			name = "flexoki",
		},
	},
}

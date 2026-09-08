-- Themes definition
--
-- Each definition contains:
-- - url: GitHub repository URL (required)
-- - name: Theme identifier used for file naming (required)
-- - config: Function to set up and activate the theme (required)
-- - dependencies: Optional array of dependency URLs

local function load_material(colorscheme)
	require("material").setup({ async_loading = false })

	local original_set_hl = vim.api.nvim_set_hl
	vim.api.nvim_set_hl = function(namespace_id, name, value)
		local link = type(value) == "table" and value.link or nil

		-- material.nvim defines a lowercase "exception" legacy alias.
		-- Highlight names are case-insensitive, so applying that self-link
		-- clears the concrete Exception style depending on iteration order.
		if type(link) == "string" and string.lower(name) == string.lower(link) then
			return
		end

		return original_set_hl(namespace_id, name, value)
	end

	local success, err = pcall(vim.cmd.colorscheme, colorscheme)
	vim.api.nvim_set_hl = original_set_hl

	if not success then
		error(err)
	end
end

return {
	{
		url = "https://github.com/folke/lazy.nvim",
		name = "neovim_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme default]])
		end,
	},
	{
		url = "https://github.com/folke/lazy.nvim",
		name = "neovim_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme default]])
		end,
	},
	{
		url = "https://github.com/Shatur/neovim-ayu",
		name = "ayu_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme ayu-dark]])
		end,
	},
	{
		url = "https://github.com/Shatur/neovim-ayu",
		name = "ayu_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme ayu-light]])
		end,
	},
	{
		url = "https://github.com/Shatur/neovim-ayu",
		name = "ayu_mirage",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme ayu-mirage]])
		end,
	},
	{
		url = "https://github.com/AlexvZyl/nordic.nvim",
		name = "nordic",
		config = function()
			vim.o.background = "dark"
			require("nordic").load()
			vim.cmd([[colorscheme nordic]])
		end,
	},
	{
		url = "https://github.com/savq/melange-nvim",
		name = "melange_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme melange]])
		end,
	},
	{
		url = "https://github.com/savq/melange-nvim",
		name = "melange_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme melange]])
		end,
	},
	{
		url = "https://github.com/bluz71/vim-nightfly-colors",
		name = "nightfly",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme nightfly]])
		end,
	},
	{
		url = "https://github.com/folke/tokyonight.nvim",
		name = "tokyonight_night",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme tokyonight-night]])
		end,
	},
	{
		url = "https://github.com/folke/tokyonight.nvim",
		name = "tokyonight_moon",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme tokyonight-moon]])
		end,
	},
	{
		url = "https://github.com/folke/tokyonight.nvim",
		name = "tokyonight_storm",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme tokyonight-storm]])
		end,
	},
	{
		url = "https://github.com/folke/tokyonight.nvim",
		name = "tokyonight_day",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme tokyonight-day]])
		end,
	},
	{
		url = "https://github.com/catppuccin/nvim",
		name = "catppuccin_frappe",
		config = function()
			vim.o.background = "dark"
			require("catppuccin").setup({ integrations = { rainbow_delimiters = true } })
			vim.cmd([[colorscheme catppuccin-frappe]])
		end,
	},
	{
		url = "https://github.com/catppuccin/nvim",
		name = "catppuccin_macchiato",
		config = function()
			vim.o.background = "dark"
			require("catppuccin").setup({ integrations = { rainbow_delimiters = true } })
			vim.cmd([[colorscheme catppuccin-macchiato]])
		end,
	},
	{
		url = "https://github.com/catppuccin/nvim",
		name = "catppuccin_mocha",
		config = function()
			vim.o.background = "dark"
			require("catppuccin").setup({ integrations = { rainbow_delimiters = true } })
			vim.cmd([[colorscheme catppuccin-mocha]])
		end,
	},
	{
		url = "https://github.com/catppuccin/nvim",
		name = "catppuccin_latte",
		config = function()
			vim.o.background = "light"
			require("catppuccin").setup({ integrations = { rainbow_delimiters = true } })
			vim.cmd([[colorscheme catppuccin-latte]])
		end,
	},
	{
		url = "https://github.com/projekt0n/github-nvim-theme",
		name = "github_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme github_dark_default]])
		end,
	},
	{
		url = "https://github.com/projekt0n/github-nvim-theme",
		name = "github_dark_dimmed",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme github_dark_dimmed]])
		end,
	},
	{
		url = "https://github.com/projekt0n/github-nvim-theme",
		name = "github_dark_high_contrast",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme github_dark_high_contrast]])
		end,
	},
	{
		url = "https://github.com/projekt0n/github-nvim-theme",
		name = "github_dark_colorblind",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme github_dark_colorblind]])
		end,
	},
	{
		url = "https://github.com/projekt0n/github-nvim-theme",
		name = "github_dark_tritanopia",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme github_dark_tritanopia]])
		end,
	},
	{
		url = "https://github.com/projekt0n/github-nvim-theme",
		name = "github_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme github_light_default]])
		end,
	},
	{
		url = "https://github.com/projekt0n/github-nvim-theme",
		name = "github_light_high_contrast",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme github_light_high_contrast]])
		end,
	},
	{
		url = "https://github.com/projekt0n/github-nvim-theme",
		name = "github_light_colorblind",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme github_light_colorblind]])
		end,
	},
	{
		url = "https://github.com/projekt0n/github-nvim-theme",
		name = "github_light_tritanopia",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme github_light_tritanopia]])
		end,
	},
	{
		url = "https://github.com/rebelot/kanagawa.nvim",
		name = "kanagawa_wave",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme kanagawa-wave]])
		end,
	},
	{
		url = "https://github.com/rebelot/kanagawa.nvim",
		name = "kanagawa_dragon",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme kanagawa-dragon]])
		end,
	},
	{
		url = "https://github.com/rebelot/kanagawa.nvim",
		name = "kanagawa_lotus",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme kanagawa-lotus]])
		end,
	},
	{
		url = "https://github.com/ellisonleao/gruvbox.nvim",
		name = "gruvbox_dark",
		config = function()
			vim.o.background = "dark"
			require("gruvbox").setup({ contrast = "" })
			vim.cmd([[colorscheme gruvbox]])
		end,
	},
	{
		url = "https://github.com/ellisonleao/gruvbox.nvim",
		name = "gruvbox_dark_hard",
		config = function()
			vim.o.background = "dark"
			require("gruvbox").setup({ contrast = "hard" })
			vim.cmd([[colorscheme gruvbox]])
		end,
	},
	{
		url = "https://github.com/ellisonleao/gruvbox.nvim",
		name = "gruvbox_dark_soft",
		config = function()
			vim.o.background = "dark"
			require("gruvbox").setup({ contrast = "soft" })
			vim.cmd([[colorscheme gruvbox]])
		end,
	},
	{
		url = "https://github.com/ellisonleao/gruvbox.nvim",
		name = "gruvbox_light",
		config = function()
			vim.o.background = "light"
			require("gruvbox").setup({ contrast = "" })
			vim.cmd([[colorscheme gruvbox]])
		end,
	},
	{
		url = "https://github.com/ellisonleao/gruvbox.nvim",
		name = "gruvbox_light_hard",
		config = function()
			vim.o.background = "light"
			require("gruvbox").setup({ contrast = "hard" })
			vim.cmd([[colorscheme gruvbox]])
		end,
	},
	{
		url = "https://github.com/ellisonleao/gruvbox.nvim",
		name = "gruvbox_light_soft",
		config = function()
			vim.o.background = "light"
			require("gruvbox").setup({ contrast = "soft" })
			vim.cmd([[colorscheme gruvbox]])
		end,
	},
	{
		url = "https://github.com/Mofiqul/dracula.nvim",
		name = "dracula",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme dracula]])
		end,
	},
	{
		url = "https://github.com/Mofiqul/dracula.nvim",
		name = "dracula_soft",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme dracula-soft]])
		end,
	},
	{
		url = "https://github.com/Mofiqul/vscode.nvim",
		name = "vscode_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme vscode]])
		end,
	},
	{
		url = "https://github.com/Mofiqul/vscode.nvim",
		name = "vscode_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme vscode]])
		end,
	},
	{
		url = "https://github.com/maxmx03/solarized.nvim",
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
		url = "https://github.com/maxmx03/solarized.nvim",
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
		url = "https://github.com/maxmx03/solarized.nvim",
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
		url = "https://github.com/maxmx03/solarized.nvim",
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
		url = "https://github.com/maxmx03/solarized.nvim",
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
		url = "https://github.com/maxmx03/solarized.nvim",
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
		url = "https://github.com/maxmx03/solarized.nvim",
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
		url = "https://github.com/maxmx03/solarized.nvim",
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
		url = "https://github.com/marko-cerovac/material.nvim",
		name = "material_darker",
		config = function()
			vim.o.background = "dark"
			vim.g.material_style = "darker"
			load_material("material-darker")
		end,
	},
	{
		url = "https://github.com/marko-cerovac/material.nvim",
		name = "material_lighter",
		config = function()
			vim.o.background = "light"
			vim.g.material_style = "lighter"
			load_material("material-lighter")
		end,
	},
	{
		url = "https://github.com/marko-cerovac/material.nvim",
		name = "material_oceanic",
		config = function()
			vim.o.background = "dark"
			vim.g.material_style = "oceanic"
			load_material("material-oceanic")
		end,
	},
	{
		url = "https://github.com/marko-cerovac/material.nvim",
		name = "material_palenight",
		config = function()
			vim.o.background = "dark"
			vim.g.material_style = "palenight"
			load_material("material-palenight")
		end,
	},
	{
		url = "https://github.com/marko-cerovac/material.nvim",
		name = "material_deep_ocean",
		config = function()
			vim.o.background = "dark"
			vim.g.material_style = "deep ocean"
			load_material("material-deep-ocean")
		end,
	},
	{
		url = "https://github.com/shaunsingh/nord.nvim",
		name = "nord",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme nord]])
		end,
	},
	{
		url = "https://github.com/navarasu/onedark.nvim",
		name = "onedark_darker",
		config = function()
			vim.o.background = "dark"
			require("onedark").setup({ style = "darker" })
			require("onedark").load()
		end,
	},
	{
		url = "https://github.com/navarasu/onedark.nvim",
		name = "onedark_cool",
		config = function()
			vim.o.background = "dark"
			require("onedark").setup({ style = "cool" })
			require("onedark").load()
		end,
	},
	{
		url = "https://github.com/navarasu/onedark.nvim",
		name = "onedark_deep",
		config = function()
			vim.o.background = "dark"
			require("onedark").setup({ style = "deep" })
			require("onedark").load()
		end,
	},
	{
		url = "https://github.com/navarasu/onedark.nvim",
		name = "onedark_warm",
		config = function()
			vim.o.background = "dark"
			require("onedark").setup({ style = "warm" })
			require("onedark").load()
		end,
	},
	{
		url = "https://github.com/navarasu/onedark.nvim",
		name = "onedark_warmer",
		config = function()
			vim.o.background = "dark"
			require("onedark").setup({ style = "warmer" })
			require("onedark").load()
		end,
	},
	{
		url = "https://github.com/navarasu/onedark.nvim",
		name = "onedark_light",
		config = function()
			vim.o.background = "light"
			require("onedark").setup({ style = "light" })
			require("onedark").load()
		end,
	},
	{
		url = "https://github.com/olimorris/onedarkpro.nvim",
		name = "onedark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme onedark]])
		end,
	},
	{
		url = "https://github.com/olimorris/onedarkpro.nvim",
		name = "onelight",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme onelight]])
		end,
	},
	{
		url = "https://github.com/olimorris/onedarkpro.nvim",
		name = "onedarkpro_vivid",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme onedark_vivid]])
		end,
	},
	{
		url = "https://github.com/olimorris/onedarkpro.nvim",
		name = "onedarkpro_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme onedark_dark]])
		end,
	},
	{
		url = "https://github.com/EdenEast/nightfox.nvim",
		name = "nightfox",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme nightfox]])
		end,
	},
	{
		url = "https://github.com/EdenEast/nightfox.nvim",
		name = "dayfox",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme dayfox]])
		end,
	},
	{
		url = "https://github.com/EdenEast/nightfox.nvim",
		name = "duskfox",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme duskfox]])
		end,
	},
	{
		url = "https://github.com/EdenEast/nightfox.nvim",
		name = "dawnfox",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme dawnfox]])
		end,
	},
	{
		url = "https://github.com/EdenEast/nightfox.nvim",
		name = "carbonfox",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme carbonfox]])
		end,
	},
	{
		url = "https://github.com/EdenEast/nightfox.nvim",
		name = "nordfox",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme nordfox]])
		end,
	},
	{
		url = "https://github.com/EdenEast/nightfox.nvim",
		name = "terafox",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme terafox]])
		end,
	},
	{
		url = "https://github.com/rose-pine/neovim",
		name = "rosepine_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme rose-pine]])
		end,
	},
	{
		url = "https://github.com/rose-pine/neovim",
		name = "rosepine_moon",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme rose-pine-moon]])
		end,
	},
	{
		url = "https://github.com/rose-pine/neovim",
		name = "rosepine_dawn",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme rose-pine-dawn]])
		end,
	},
	{
		url = "https://github.com/neanias/everforest-nvim",
		name = "everforest_dark",
		config = function()
			vim.o.background = "dark"
			require("everforest").setup({ background = "medium" })
			vim.cmd([[colorscheme everforest]])
		end,
	},
	{
		url = "https://github.com/neanias/everforest-nvim",
		name = "everforest_light",
		config = function()
			vim.o.background = "light"
			require("everforest").setup({ background = "medium" })
			vim.cmd([[colorscheme everforest]])
		end,
	},
	{
		url = "https://github.com/sainnhe/edge",
		name = "edge_dark",
		config = function()
			vim.o.background = "dark"
			vim.g.edge_style = "default"
			vim.cmd([[colorscheme edge]])
		end,
	},
	{
		url = "https://github.com/sainnhe/edge",
		name = "edge_light",
		config = function()
			vim.o.background = "light"
			vim.g.edge_style = "default"
			vim.cmd([[colorscheme edge]])
		end,
	},
	{
		url = "https://github.com/sainnhe/edge",
		name = "edge_aura",
		config = function()
			vim.o.background = "dark"
			vim.g.edge_style = "aura"
			vim.cmd([[colorscheme edge]])
		end,
	},
	{
		url = "https://github.com/sainnhe/edge",
		name = "edge_neon",
		config = function()
			vim.o.background = "dark"
			vim.g.edge_style = "neon"
			vim.cmd([[colorscheme edge]])
		end,
	},
	{
		url = "https://github.com/miikanissi/modus-themes.nvim",
		name = "modus_operandi",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme modus_operandi]])
		end,
	},
	{
		url = "https://github.com/miikanissi/modus-themes.nvim",
		name = "modus_vivendi",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme modus_vivendi]])
		end,
	},
	{
		url = "https://github.com/glepnir/zephyr-nvim",
		name = "zephyr_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme zephyr]])
		end,
	},
	{
		url = "https://github.com/svrana/neosolarized.nvim",
		name = "neosolarized",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme neosolarized]])
		end,
		dependencies = { "https://github.com/tjdevries/colorbuddy.nvim" },
	},
	{
		url = "https://github.com/loctvl842/monokai-pro.nvim",
		name = "monokai_pro_dark",
		config = function()
			vim.o.background = "dark"
			require("monokai-pro").setup({ filter = "pro" })
			vim.cmd([[colorscheme monokai-pro]])
		end,
	},
	{
		url = "https://github.com/loctvl842/monokai-pro.nvim",
		name = "monokai_pro_machine",
		config = function()
			vim.o.background = "dark"
			require("monokai-pro").setup({ filter = "machine" })
			vim.cmd([[colorscheme monokai-pro]])
		end,
	},
	{
		url = "https://github.com/loctvl842/monokai-pro.nvim",
		name = "monokai_pro_ristretto",
		config = function()
			vim.o.background = "dark"
			require("monokai-pro").setup({ filter = "ristretto" })
			vim.cmd([[colorscheme monokai-pro]])
		end,
	},
	{
		url = "https://github.com/loctvl842/monokai-pro.nvim",
		name = "monokai_pro_spectrum",
		config = function()
			vim.o.background = "dark"
			require("monokai-pro").setup({ filter = "spectrum" })
			vim.cmd([[colorscheme monokai-pro]])
		end,
	},
	{
		url = "https://github.com/ribru17/bamboo.nvim",
		name = "bamboo_light",
		config = function()
			vim.o.background = "light"
			require("bamboo").setup({ style = "light" })
			vim.cmd([[colorscheme bamboo]])
		end,
	},
	{
		url = "https://github.com/ribru17/bamboo.nvim",
		name = "bamboo_vulgaris",
		config = function()
			vim.o.background = "dark"
			require("bamboo").setup({ style = "vulgaris" })
			vim.cmd([[colorscheme bamboo]])
		end,
	},
	{
		url = "https://github.com/ribru17/bamboo.nvim",
		name = "bamboo_multiplex",
		config = function()
			vim.o.background = "dark"
			require("bamboo").setup({ style = "multiplex" })
			vim.cmd([[colorscheme bamboo]])
		end,
	},
	{
		url = "https://github.com/daltonmenezes/aura-theme",
		name = "aura_dark",
		config = function()
			vim.opt.rtp:append(vim.fn.stdpath("data") .. "/site/pack/core/opt/aura-theme/packages/neovim")
			vim.o.background = "dark"
			vim.cmd([[colorscheme aura-dark]])
		end,
	},
	{
		url = "https://github.com/daltonmenezes/aura-theme",
		name = "aura_dark_soft_text",
		config = function()
			vim.opt.rtp:append(vim.fn.stdpath("data") .. "/site/pack/core/opt/aura-theme/packages/neovim")
			vim.o.background = "dark"
			vim.cmd([[colorscheme aura-dark-soft-text]])
		end,
	},
	{
		url = "https://github.com/daltonmenezes/aura-theme",
		name = "aura_soft_dark",
		config = function()
			vim.opt.rtp:append(vim.fn.stdpath("data") .. "/site/pack/core/opt/aura-theme/packages/neovim")
			vim.o.background = "dark"
			vim.cmd([[colorscheme aura-soft-dark]])
		end,
	},
	{
		url = "https://github.com/daltonmenezes/aura-theme",
		name = "aura_soft_dark_soft_text",
		config = function()
			vim.opt.rtp:append(vim.fn.stdpath("data") .. "/site/pack/core/opt/aura-theme/packages/neovim")
			vim.o.background = "dark"
			vim.cmd([[colorscheme aura-soft-dark-soft-text]])
		end,
	},
	{
		url = "https://github.com/bluz71/vim-moonfly-colors",
		name = "moonfly",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme moonfly]])
		end,
	},
	{
		url = "https://github.com/scottmckendry/cyberdream.nvim",
		name = "cyberdream_dark",
		config = function()
			vim.o.background = "dark"
			require("cyberdream").setup({ variant = "dark" })
			vim.cmd([[colorscheme cyberdream]])
		end,
	},
	{
		url = "https://github.com/scottmckendry/cyberdream.nvim",
		name = "cyberdream_light",
		config = function()
			vim.o.background = "light"
			require("cyberdream").setup({ variant = "light" })
			vim.cmd([[colorscheme cyberdream-light]])
		end,
	},
	{
		url = "https://github.com/uloco/bluloco.nvim",
		name = "bluloco_dark",
		config = function()
			vim.o.background = "dark"
			require("bluloco").setup({ style = "dark" })
			vim.cmd([[colorscheme bluloco-dark]])
		end,
		dependencies = { "https://github.com/rktjmp/lush.nvim" },
	},
	{
		url = "https://github.com/uloco/bluloco.nvim",
		name = "bluloco_light",
		config = function()
			vim.o.background = "light"
			require("bluloco").setup({ style = "light" })
			vim.cmd([[colorscheme bluloco-light]])
		end,
		dependencies = { "https://github.com/rktjmp/lush.nvim" },
	},
	{
		url = "https://github.com/martinsione/darkplus.nvim",
		name = "darkplus",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme darkplus]])
		end,
	},
	{
		url = "https://github.com/kepano/flexoki-neovim",
		name = "flexoki_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme flexoki-dark]])
		end,
	},
	{
		url = "https://github.com/nomis51/nvim-xcode-theme",
		name = "xcode_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme xcodedark]])
		end,
	},
	{
		url = "https://github.com/nomis51/nvim-xcode-theme",
		name = "xcode_dark_hc",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme xcodedarkhc]])
		end,
	},
	{
		url = "https://github.com/nomis51/nvim-xcode-theme",
		name = "xcode_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme xcodelight]])
		end,
	},
	{
		url = "https://github.com/nomis51/nvim-xcode-theme",
		name = "xcode_light_hc",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme xcodelighthc]])
		end,
	},
	{
		url = "https://github.com/nomis51/nvim-xcode-theme",
		name = "xcode_wwdc",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme xcodewwdc]])
		end,
	},
	{
		url = "https://github.com/kepano/flexoki-neovim",
		name = "flexoki_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme flexoki-light]])
		end,
	},
	{
		url = "https://github.com/phha/zenburn.nvim",
		name = "zenburn",
		config = function()
			vim.o.background = "dark"
			require("zenburn").setup()
		end,
	},
	{
		url = "https://github.com/shaunsingh/moonlight.nvim",
		name = "moonlight",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme moonlight]])
		end,
	},
	{
		url = "https://github.com/UtkarshVerma/molokai.nvim",
		name = "molokai",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme molokai]])
		end,
	},
	{
		url = "https://github.com/NLKNguyen/papercolor-theme",
		name = "papercolor_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme PaperColor]])
		end,
	},
	{
		url = "https://github.com/NLKNguyen/papercolor-theme",
		name = "papercolor_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme PaperColor]])
		end,
	},
	{
		url = "https://github.com/cocopon/iceberg.vim",
		name = "iceberg",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme iceberg]])
		end,
	},
	{
		url = "https://github.com/akinsho/horizon.nvim",
		name = "horizon_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme horizon]])
		end,
	},
	{
		url = "https://github.com/akinsho/horizon.nvim",
		name = "horizon_light",
		config = function()
			vim.o.background = "light"
			local data = require("horizon.palette-light")
			data.palette.syntax.apricot = data.palette.syntax.apricot or data.palette.syntax.jaffa
			data.palette.syntax.lavender = data.palette.syntax.lavender or data.palette.syntax.amethyst
			data.palette.syntax.turquoise = data.palette.syntax.turquoise or data.palette.syntax.elm
			require("horizon").setup()
		end,
	},
	{
		url = "https://github.com/srcery-colors/srcery-vim",
		name = "srcery",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme srcery]])
		end,
	},
	{
		url = "https://github.com/tahayvr/matteblack.nvim",
		name = "matte_black",
		config = function()
			vim.o.background = "dark"
			require("matteblack").colorscheme()
		end,
	},
	{
		url = "https://github.com/sainnhe/sonokai",
		name = "sonokai_default",
		config = function()
			vim.o.background = "dark"
			vim.g.sonokai_style = "default"
			vim.cmd([[colorscheme sonokai]])
		end,
	},
	{
		url = "https://github.com/sainnhe/sonokai",
		name = "sonokai_atlantis",
		config = function()
			vim.o.background = "dark"
			vim.g.sonokai_style = "atlantis"
			vim.cmd([[colorscheme sonokai]])
		end,
	},
	{
		url = "https://github.com/sainnhe/sonokai",
		name = "sonokai_andromeda",
		config = function()
			vim.o.background = "dark"
			vim.g.sonokai_style = "andromeda"
			vim.cmd([[colorscheme sonokai]])
		end,
	},
	{
		url = "https://github.com/sainnhe/sonokai",
		name = "sonokai_shusia",
		config = function()
			vim.o.background = "dark"
			vim.g.sonokai_style = "shusia"
			vim.cmd([[colorscheme sonokai]])
		end,
	},
	{
		url = "https://github.com/sainnhe/sonokai",
		name = "sonokai_maia",
		config = function()
			vim.o.background = "dark"
			vim.g.sonokai_style = "maia"
			vim.cmd([[colorscheme sonokai]])
		end,
	},
	{
		url = "https://github.com/sainnhe/sonokai",
		name = "sonokai_espresso",
		config = function()
			vim.o.background = "dark"
			vim.g.sonokai_style = "espresso"
			vim.cmd([[colorscheme sonokai]])
		end,
	},
	{
		url = "https://github.com/sainnhe/gruvbox-material",
		name = "gruvbox_material_dark",
		config = function()
			vim.o.background = "dark"
			vim.g.gruvbox_material_background = "medium"
			vim.cmd([[colorscheme gruvbox-material]])
		end,
	},
	{
		url = "https://github.com/sainnhe/gruvbox-material",
		name = "gruvbox_material_light",
		config = function()
			vim.o.background = "light"
			vim.g.gruvbox_material_background = "medium"
			vim.cmd([[colorscheme gruvbox-material]])
		end,
	},
	{
		url = "https://github.com/oxfist/night-owl.nvim",
		name = "night_owl",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme night-owl]])
		end,
	},
	{
		url = "https://github.com/nyoom-engineering/oxocarbon.nvim",
		name = "oxocarbon_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme oxocarbon]])
		end,
	},
	{
		url = "https://github.com/nyoom-engineering/oxocarbon.nvim",
		name = "oxocarbon_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme oxocarbon]])
		end,
	},
	{
		url = "https://github.com/craftzdog/solarized-osaka.nvim",
		name = "solarized_osaka_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme solarized-osaka]])
		end,
	},
	{
		url = "https://github.com/craftzdog/solarized-osaka.nvim",
		name = "solarized_osaka_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme solarized-osaka]])
		end,
	},
	{
		url = "https://github.com/olivercederborg/poimandres.nvim",
		name = "poimandres",
		config = function()
			vim.o.background = "dark"
			require("poimandres").setup()
			vim.cmd([[colorscheme poimandres]])
		end,
	},
	{
		url = "https://github.com/eldritch-theme/eldritch.nvim",
		name = "eldritch",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme eldritch]])
		end,
	},
	{
		url = "https://github.com/zenbones-theme/zenbones.nvim",
		name = "zenbones_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme zenbones]])
		end,
		dependencies = { "https://github.com/rktjmp/lush.nvim" },
	},
	{
		url = "https://github.com/zenbones-theme/zenbones.nvim",
		name = "zenbones_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme zenbones]])
		end,
		dependencies = { "https://github.com/rktjmp/lush.nvim" },
	},
	{
		url = "https://github.com/zenbones-theme/zenbones.nvim",
		name = "zenwritten_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme zenwritten]])
		end,
		dependencies = { "https://github.com/rktjmp/lush.nvim" },
	},
	{
		url = "https://github.com/zenbones-theme/zenbones.nvim",
		name = "zenwritten_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme zenwritten]])
		end,
		dependencies = { "https://github.com/rktjmp/lush.nvim" },
	},
	{
		url = "https://github.com/zenbones-theme/zenbones.nvim",
		name = "neobones_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme neobones]])
		end,
		dependencies = { "https://github.com/rktjmp/lush.nvim" },
	},
	{
		url = "https://github.com/zenbones-theme/zenbones.nvim",
		name = "neobones_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme neobones]])
		end,
		dependencies = { "https://github.com/rktjmp/lush.nvim" },
	},
	{
		url = "https://github.com/zenbones-theme/zenbones.nvim",
		name = "vimbones",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme vimbones]])
		end,
		dependencies = { "https://github.com/rktjmp/lush.nvim" },
	},
	{
		url = "https://github.com/zenbones-theme/zenbones.nvim",
		name = "rosebones_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme rosebones]])
		end,
		dependencies = { "https://github.com/rktjmp/lush.nvim" },
	},
	{
		url = "https://github.com/zenbones-theme/zenbones.nvim",
		name = "rosebones_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme rosebones]])
		end,
		dependencies = { "https://github.com/rktjmp/lush.nvim" },
	},
	{
		url = "https://github.com/zenbones-theme/zenbones.nvim",
		name = "forestbones_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme forestbones]])
		end,
		dependencies = { "https://github.com/rktjmp/lush.nvim" },
	},
	{
		url = "https://github.com/zenbones-theme/zenbones.nvim",
		name = "forestbones_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme forestbones]])
		end,
		dependencies = { "https://github.com/rktjmp/lush.nvim" },
	},
	{
		url = "https://github.com/zenbones-theme/zenbones.nvim",
		name = "nordbones",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme nordbones]])
		end,
		dependencies = { "https://github.com/rktjmp/lush.nvim" },
	},
	{
		url = "https://github.com/zenbones-theme/zenbones.nvim",
		name = "tokyobones_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme tokyobones]])
		end,
		dependencies = { "https://github.com/rktjmp/lush.nvim" },
	},
	{
		url = "https://github.com/zenbones-theme/zenbones.nvim",
		name = "tokyobones_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme tokyobones]])
		end,
		dependencies = { "https://github.com/rktjmp/lush.nvim" },
	},
	{
		url = "https://github.com/zenbones-theme/zenbones.nvim",
		name = "seoulbones_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme seoulbones]])
		end,
		dependencies = { "https://github.com/rktjmp/lush.nvim" },
	},
	{
		url = "https://github.com/zenbones-theme/zenbones.nvim",
		name = "seoulbones_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme seoulbones]])
		end,
		dependencies = { "https://github.com/rktjmp/lush.nvim" },
	},
	{
		url = "https://github.com/zenbones-theme/zenbones.nvim",
		name = "duckbones",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme duckbones]])
		end,
		dependencies = { "https://github.com/rktjmp/lush.nvim" },
	},
	{
		url = "https://github.com/zenbones-theme/zenbones.nvim",
		name = "zenburned",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme zenburned]])
		end,
		dependencies = { "https://github.com/rktjmp/lush.nvim" },
	},
	{
		url = "https://github.com/zenbones-theme/zenbones.nvim",
		name = "kanagawabones",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme kanagawabones]])
		end,
		dependencies = { "https://github.com/rktjmp/lush.nvim" },
	},
	{
		url = "https://github.com/casedami/neomodern.nvim",
		name = "neomodern_moon_dark",
		config = function()
			vim.o.background = "dark"
			require("neomodern").load("moon")
		end,
	},
	{
		url = "https://github.com/casedami/neomodern.nvim",
		name = "neomodern_moon_light",
		config = function()
			vim.o.background = "light"
			require("neomodern").load("moon")
		end,
	},
	{
		url = "https://github.com/casedami/neomodern.nvim",
		name = "neomodern_iceclimber_dark",
		config = function()
			vim.o.background = "dark"
			require("neomodern").load("iceclimber")
		end,
	},
	{
		url = "https://github.com/casedami/neomodern.nvim",
		name = "neomodern_iceclimber_light",
		config = function()
			vim.o.background = "light"
			require("neomodern").load("iceclimber")
		end,
	},
	{
		url = "https://github.com/casedami/neomodern.nvim",
		name = "neomodern_gyokuro_dark",
		config = function()
			vim.o.background = "dark"
			require("neomodern").load("gyokuro")
		end,
	},
	{
		url = "https://github.com/casedami/neomodern.nvim",
		name = "neomodern_gyokuro_light",
		config = function()
			vim.o.background = "light"
			require("neomodern").load("gyokuro")
		end,
	},
	{
		url = "https://github.com/casedami/neomodern.nvim",
		name = "neomodern_hojicha_dark",
		config = function()
			vim.o.background = "dark"
			require("neomodern").load("hojicha")
		end,
	},
	{
		url = "https://github.com/casedami/neomodern.nvim",
		name = "neomodern_hojicha_light",
		config = function()
			vim.o.background = "light"
			require("neomodern").load("hojicha")
		end,
	},
	{
		url = "https://github.com/casedami/neomodern.nvim",
		name = "neomodern_roseprime_dark",
		config = function()
			vim.o.background = "dark"
			require("neomodern").load("roseprime")
		end,
	},
	{
		url = "https://github.com/casedami/neomodern.nvim",
		name = "neomodern_roseprime_light",
		config = function()
			vim.o.background = "light"
			require("neomodern").load("roseprime")
		end,
	},
	{
		url = "https://github.com/kungfusheep/mfd.nvim",
		name = "mfd_default",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme mfd]])
		end,
	},
	{
		url = "https://github.com/kungfusheep/mfd.nvim",
		name = "mfd_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme mfd-dark]])
		end,
	},
	{
		url = "https://github.com/kungfusheep/mfd.nvim",
		name = "mfd_stealth",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme mfd-stealth]])
		end,
	},
	{
		url = "https://github.com/kungfusheep/mfd.nvim",
		name = "mfd_amber",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme mfd-amber]])
		end,
	},
	{
		url = "https://github.com/kungfusheep/mfd.nvim",
		name = "mfd_mono",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme mfd-mono]])
		end,
	},
	{
		url = "https://github.com/kungfusheep/mfd.nvim",
		name = "mfd_scarlet",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme mfd-scarlet]])
		end,
	},
	{
		url = "https://github.com/kungfusheep/mfd.nvim",
		name = "mfd_paper",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme mfd-paper]])
		end,
	},
	{
		url = "https://github.com/kungfusheep/mfd.nvim",
		name = "mfd_hud",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme mfd-hud]])
		end,
	},
	{
		url = "https://github.com/kungfusheep/mfd.nvim",
		name = "mfd_nvg",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme mfd-nvg]])
		end,
	},
	{
		url = "https://github.com/kungfusheep/mfd.nvim",
		name = "mfd_blackout",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme mfd-blackout]])
		end,
	},
	{
		url = "https://github.com/kungfusheep/mfd.nvim",
		name = "mfd_flir",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme mfd-flir]])
		end,
	},
	{
		url = "https://github.com/kungfusheep/mfd.nvim",
		name = "mfd_flir_bh",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme mfd-flir-bh]])
		end,
	},
	{
		url = "https://github.com/kungfusheep/mfd.nvim",
		name = "mfd_flir_rh",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme mfd-flir-rh]])
		end,
	},
	{
		url = "https://github.com/kungfusheep/mfd.nvim",
		name = "mfd_flir_fusion",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme mfd-flir-fusion]])
		end,
	},
	{
		url = "https://github.com/kungfusheep/mfd.nvim",
		name = "mfd_gbl_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme mfd-gbl-light]])
		end,
	},
	{
		url = "https://github.com/kungfusheep/mfd.nvim",
		name = "mfd_gbl_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme mfd-gbl-dark]])
		end,
	},
	{
		url = "https://github.com/kungfusheep/mfd.nvim",
		name = "mfd_lumon",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme mfd-lumon]])
		end,
	},
	{
		url = "https://github.com/rmehri01/onenord.nvim",
		name = "onenord_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme onenord]])
		end,
	},
	{
		url = "https://github.com/rmehri01/onenord.nvim",
		name = "onenord_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme onenord]])
		end,
	},
	{
		url = "https://github.com/Everblush/nvim",
		name = "everblush",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme everblush]])
		end,
	},
	{
		url = "https://github.com/NTBBloodbath/doom-one.nvim",
		name = "doom_one_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme doom-one]])
		end,
	},
	{
		url = "https://github.com/NTBBloodbath/doom-one.nvim",
		name = "doom_one_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme doom-one]])
		end,
	},
	{
		url = "https://github.com/mellow-theme/mellow.nvim",
		name = "mellow",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme mellow]])
		end,
	},
	{
		url = "https://github.com/vague-theme/vague.nvim",
		name = "vague",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme vague]])
		end,
	},
	{
		url = "https://github.com/Mofiqul/adwaita.nvim",
		name = "adwaita_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme adwaita]])
		end,
	},
	{
		url = "https://github.com/Mofiqul/adwaita.nvim",
		name = "adwaita_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme adwaita]])
		end,
	},
	{
		url = "https://github.com/dgox16/oldworld.nvim",
		name = "oldworld",
		config = function()
			vim.o.background = "dark"
			require("oldworld").setup({ variant = "default" })
			vim.cmd([[colorscheme oldworld]])
		end,
	},
	{
		url = "https://github.com/dgox16/oldworld.nvim",
		name = "oldworld_oled",
		config = function()
			vim.o.background = "dark"
			require("oldworld").setup({ variant = "oled" })
			vim.cmd([[colorscheme oldworld]])
		end,
	},
	{
		url = "https://github.com/dgox16/oldworld.nvim",
		name = "oldworld_cooler",
		config = function()
			vim.o.background = "dark"
			require("oldworld").setup({ variant = "cooler" })
			vim.cmd([[colorscheme oldworld]])
		end,
	},
	{
		url = "https://github.com/maxmx03/fluoromachine.nvim",
		name = "fluoromachine_fluoromachine",
		config = function()
			vim.o.background = "dark"
			require("fluoromachine").setup({ theme = "fluoromachine" })
			vim.cmd([[colorscheme fluoromachine]])
		end,
	},
	{
		url = "https://github.com/maxmx03/fluoromachine.nvim",
		name = "fluoromachine_retrowave",
		config = function()
			vim.o.background = "dark"
			require("fluoromachine").setup({ theme = "retrowave" })
			vim.cmd([[colorscheme fluoromachine]])
		end,
	},
	{
		url = "https://github.com/maxmx03/fluoromachine.nvim",
		name = "fluoromachine_delta",
		config = function()
			vim.o.background = "dark"
			require("fluoromachine").setup({ theme = "delta" })
			vim.cmd([[colorscheme fluoromachine]])
		end,
	},
	{
		url = "https://github.com/luisiacc/gruvbox-baby",
		name = "gruvbox_baby",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme gruvbox-baby]])
		end,
	},
	{
		url = "https://github.com/HoNamDuong/hybrid.nvim",
		name = "hybrid",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme hybrid]])
		end,
	},
	{
		url = "https://github.com/dasupradyumna/midnight.nvim",
		name = "midnight",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme midnight]])
		end,
	},
	{
		url = "https://github.com/xero/miasma.nvim",
		name = "miasma",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme miasma]])
		end,
	},
	{
		url = "https://github.com/Verf/deepwhite.nvim",
		name = "deepwhite",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme deepwhite]])
		end,
	},
	{
		url = "https://github.com/zootedb0t/citruszest.nvim",
		name = "citruszest",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme citruszest]])
		end,
	},
	{
		url = "https://github.com/slugbyte/lackluster.nvim",
		name = "lackluster",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme lackluster]])
		end,
	},
	{
		url = "https://github.com/slugbyte/lackluster.nvim",
		name = "lackluster_hack",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme lackluster-hack]])
		end,
	},
	{
		url = "https://github.com/slugbyte/lackluster.nvim",
		name = "lackluster_mint",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme lackluster-mint]])
		end,
	},
	{
		url = "https://github.com/blazkowolf/gruber-darker.nvim",
		name = "gruber_darker",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme gruber-darker]])
		end,
	},
	{
		url = "https://github.com/JoosepAlviste/palenightfall.nvim",
		name = "palenightfall",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme palenightfall]])
		end,
	},
	{
		url = "https://github.com/embark-theme/vim",
		name = "embark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme embark]])
		end,
	},
	{
		url = "https://github.com/rockyzhang24/arctic.nvim",
		name = "arctic",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme arctic]])
		end,
		dependencies = { "https://github.com/rktjmp/lush.nvim" },
	},
	{
		url = "https://github.com/ramojus/mellifluous.nvim",
		name = "mellifluous_dark",
		config = function()
			vim.o.background = "dark"
			require("mellifluous").setup({ colorset = "mellifluous" })
			vim.cmd([[colorscheme mellifluous]])
		end,
	},
	{
		url = "https://github.com/ramojus/mellifluous.nvim",
		name = "mellifluous_light",
		config = function()
			vim.o.background = "light"
			require("mellifluous").setup({ colorset = "mellifluous" })
			vim.cmd([[colorscheme mellifluous]])
		end,
	},
	{
		url = "https://github.com/ramojus/mellifluous.nvim",
		name = "mellifluous_alduin",
		config = function()
			vim.o.background = "dark"
			require("mellifluous").setup({ colorset = "alduin" })
			vim.cmd([[colorscheme mellifluous]])
		end,
	},
	{
		url = "https://github.com/ramojus/mellifluous.nvim",
		name = "mellifluous_mountain",
		config = function()
			vim.o.background = "dark"
			require("mellifluous").setup({ colorset = "mountain" })
			vim.cmd([[colorscheme mellifluous]])
		end,
	},
	{
		url = "https://github.com/ramojus/mellifluous.nvim",
		name = "mellifluous_tender",
		config = function()
			vim.o.background = "dark"
			require("mellifluous").setup({ colorset = "tender" })
			vim.cmd([[colorscheme mellifluous]])
		end,
	},
	{
		url = "https://github.com/ramojus/mellifluous.nvim",
		name = "mellifluous_kanagawa_dragon",
		config = function()
			vim.o.background = "dark"
			require("mellifluous").setup({ colorset = "kanagawa_dragon" })
			vim.cmd([[colorscheme mellifluous]])
		end,
	},
	{
		url = "https://github.com/AstroNvim/astrotheme",
		name = "astrodark",
		config = function()
			vim.o.background = "dark"
			local astrotheme = require("astrotheme")
			astrotheme.setup({ palette = "astrodark" })
			astrotheme.load("astrodark")
		end,
	},
	{
		url = "https://github.com/AstroNvim/astrotheme",
		name = "astrolight",
		config = function()
			vim.o.background = "light"
			local astrotheme = require("astrotheme")
			astrotheme.setup({ palette = "astrolight" })
			astrotheme.load("astrolight")
		end,
	},
	{
		url = "https://github.com/AstroNvim/astrotheme",
		name = "astromars",
		config = function()
			vim.o.background = "dark"
			local astrotheme = require("astrotheme")
			astrotheme.setup({ palette = "astromars" })
			astrotheme.load("astromars")
		end,
	},
	{
		url = "https://github.com/AstroNvim/astrotheme",
		name = "astrojupiter",
		config = function()
			vim.o.background = "light"
			local astrotheme = require("astrotheme")
			astrotheme.setup({ palette = "astrojupiter" })
			astrotheme.load("astrojupiter")
		end,
	},
	{
		url = "https://github.com/diegoulloao/neofusion.nvim",
		name = "neofusion",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme neofusion]])
		end,
	},
	{
		url = "https://github.com/lalitmee/cobalt2.nvim",
		name = "cobalt2",
		config = function()
			vim.o.background = "dark"
			require("colorbuddy").colorscheme("cobalt2")
		end,
		dependencies = { "https://github.com/tjdevries/colorbuddy.nvim" },
	},
	{
		url = "https://github.com/metalelf0/jellybeans-nvim",
		name = "jellybeans",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme jellybeans-nvim]])
		end,
		dependencies = { "https://github.com/rktjmp/lush.nvim" },
	},
	{
		url = "https://github.com/polirritmico/monokai-nightasty.nvim",
		name = "monokai_nightasty_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme monokai-nightasty]])
		end,
	},
	{
		url = "https://github.com/polirritmico/monokai-nightasty.nvim",
		name = "monokai_nightasty_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme monokai-nightasty]])
		end,
	},
	{
		url = "https://github.com/rafamadriz/neon",
		name = "neon_default",
		config = function()
			vim.o.background = "dark"
			vim.g.neon_style = "default"
			vim.cmd([[colorscheme neon]])
		end,
	},
	{
		url = "https://github.com/rafamadriz/neon",
		name = "neon_doom",
		config = function()
			vim.o.background = "dark"
			vim.g.neon_style = "doom"
			vim.cmd([[colorscheme neon]])
		end,
	},
	{
		url = "https://github.com/rafamadriz/neon",
		name = "neon_dark",
		config = function()
			vim.o.background = "dark"
			vim.g.neon_style = "dark"
			vim.cmd([[colorscheme neon]])
		end,
	},
	{
		url = "https://github.com/rafamadriz/neon",
		name = "neon_light",
		config = function()
			vim.o.background = "light"
			vim.g.neon_style = "light"
			vim.cmd([[colorscheme neon]])
		end,
	},
	{
		url = "https://github.com/0xstepit/flow.nvim",
		name = "flow_dark",
		config = function()
			vim.o.background = "dark"
			require("flow").setup()
			vim.cmd([[colorscheme flow]])
		end,
	},
	{
		url = "https://github.com/0xstepit/flow.nvim",
		name = "flow_light",
		config = function()
			vim.o.background = "light"
			require("flow").setup({
				theme = {
					style = "light",
				},
			})
			vim.cmd([[colorscheme flow]])
		end,
	},
	{
		url = "https://github.com/thesimonho/kanagawa-paper.nvim",
		name = "kanagawa_paper_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme kanagawa-paper]])
		end,
	},
	{
		url = "https://github.com/thesimonho/kanagawa-paper.nvim",
		name = "kanagawa_paper_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme kanagawa-paper]])
		end,
	},
	{
		url = "https://github.com/Tsuzat/NeoSolarized.nvim",
		name = "neosolarized2_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme NeoSolarized]])
		end,
	},
	{
		url = "https://github.com/Tsuzat/NeoSolarized.nvim",
		name = "neosolarized2_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme NeoSolarized]])
		end,
	},
	{
		url = "https://github.com/ofirgall/ofirkai.nvim",
		name = "ofirkai",
		config = function()
			vim.o.background = "dark"
			require("ofirkai").setup({})
		end,
	},
	{
		url = "https://github.com/cpea2506/one_monokai.nvim",
		name = "one_monokai",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme one_monokai]])
		end,
	},
	{
		url = "https://github.com/yorumicolors/yorumi.nvim",
		name = "yorumi",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme yorumi]])
		end,
	},
	{
		url = "https://github.com/ray-x/aurora",
		name = "aurora",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme aurora]])
		end,
	},
	{
		url = "https://github.com/killitar/obscure.nvim",
		name = "obscure",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme obscure]])
		end,
	},
	{
		url = "https://github.com/samharju/synthweave.nvim",
		name = "synthweave",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme synthweave]])
		end,
	},
	{
		url = "https://github.com/samharju/synthweave.nvim",
		name = "synthweave_aqua",
		config = function()
			vim.o.background = "dark"
			require("synthweave").setup({
				palette = {
					cyan = "#7df9ff",
					blue_bright = "#59f3ff",
					green_bright = "#8fffe0",
				},
			})
			require("synthweave").load()
		end,
	},
	{
		url = "https://github.com/aktersnurra/no-clown-fiesta.nvim",
		name = "no_clown_fiesta",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme no-clown-fiesta]])
		end,
	},
	{
		url = "https://github.com/tanvirtin/monokai.nvim",
		name = "monokai",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme monokai]])
		end,
	},
	{
		url = "https://github.com/tanvirtin/monokai.nvim",
		name = "monokai_soda",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme monokai_soda]])
		end,
	},
	{
		url = "https://github.com/tanvirtin/monokai.nvim",
		name = "monokai_pro",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme monokai_pro]])
		end,
	},
	{
		url = "https://github.com/mhartington/oceanic-next",
		name = "oceanic_next",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme OceanicNext]])
		end,
	},
	{
		url = "https://github.com/datsfilipe/vesper.nvim",
		name = "vesper",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme vesper]])
		end,
	},
	{
		url = "https://github.com/tiagovla/tokyodark.nvim",
		name = "tokyodark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme tokyodark]])
		end,
	},
	{
		url = "https://github.com/darkvoid-theme/darkvoid.nvim",
		name = "darkvoid",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme darkvoid]])
		end,
	},
	{
		url = "https://github.com/ThorstenRhau/token",
		name = "token_ultra_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme token-ultra]])
		end,
	},
	{
		url = "https://github.com/ThorstenRhau/token",
		name = "token_ultra_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme token-ultra]])
		end,
	},
	{
		url = "https://github.com/ThorstenRhau/token",
		name = "token_meridian_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme token-meridian]])
		end,
	},
	{
		url = "https://github.com/ThorstenRhau/token",
		name = "token_meridian_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme token-meridian]])
		end,
	},
	{
		url = "https://github.com/ThorstenRhau/token",
		name = "token_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme token]])
		end,
	},
	{
		url = "https://github.com/ThorstenRhau/token",
		name = "token_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme token]])
		end,
	},
	{
		url = "https://github.com/ThorstenRhau/token",
		name = "token_flint_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme token-flint]])
		end,
	},
	{
		url = "https://github.com/ThorstenRhau/token",
		name = "token_flint_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme token-flint]])
		end,
	},
	{
		url = "https://github.com/ThorstenRhau/token",
		name = "token_temper_dark",
		config = function()
			vim.o.background = "dark"
			vim.cmd([[colorscheme token-temper]])
		end,
	},
	{
		url = "https://github.com/ThorstenRhau/token",
		name = "token_temper_light",
		config = function()
			vim.o.background = "light"
			vim.cmd([[colorscheme token-temper]])
		end,
	},
}

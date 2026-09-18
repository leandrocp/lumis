local script_path = debug.getinfo(1, "S").source:sub(2)
local script_dir = vim.fn.fnamemodify(script_path, ":p:h")
local colliding_repo_names = {}
local uv = vim.uv or vim.loop

for _, signal in ipairs({ "sigint", "sigterm" }) do
	local handler = uv.new_signal()
	handler:start(signal, function()
		os.exit(130)
	end)
end

local function parse_repo_parts(repo_url)
	local owner, repo = repo_url:match("github%.com/([^/]+)/([^/]+)$")
	if not owner or not repo then
		return nil, repo_url:match("/([^/]+)$")
	end
	return owner, repo:gsub("%.git$", "")
end

local function plugin_dir_name(repo_url)
	local owner, repo = parse_repo_parts(repo_url)
	if not repo then
		return "unknown"
	end
	if owner and colliding_repo_names[repo:lower()] then
		return owner .. "-" .. repo
	end
	return repo
end

local function table_size(tbl)
	local count = 0
	for _ in pairs(tbl) do
		count = count + 1
	end
	return count
end

local regular_groups = {
	"Normal",
	"Comment",
	"CursorLine",
	"LineNr",
	"CursorLineNr",
	"RainbowDelimiterRed",
	"RainbowDelimiterYellow",
	"RainbowDelimiterBlue",
	"RainbowDelimiterOrange",
	"RainbowDelimiterGreen",
	"RainbowDelimiterViolet",
}

local group_key_map = {
	LineNr = "line_number",
	CursorLineNr = "line_number.highlighted",
	RainbowDelimiterRed = "punctuation.bracket.rainbow.1",
	RainbowDelimiterYellow = "punctuation.bracket.rainbow.2",
	RainbowDelimiterBlue = "punctuation.bracket.rainbow.3",
	RainbowDelimiterOrange = "punctuation.bracket.rainbow.4",
	RainbowDelimiterGreen = "punctuation.bracket.rainbow.5",
	RainbowDelimiterViolet = "punctuation.bracket.rainbow.6",
}

local treesitter_groups = {
	"attribute",
	"attribute.builtin",
	"boolean",
	"character",
	"character.special",
	"charset",
	"comment",
	"comment.documentation",
	"comment.error",
	"comment.hint",
	"comment.note",
	"comment.todo",
	"comment.warning",
	"constant",
	"constant.builtin",
	"constant.macro",
	"constructor",
	"diff.delta",
	"diff.minus",
	"diff.plus",
	"error",
	"function",
	"function.builtin",
	"function.call",
	"function.macro",
	"function.method",
	"function.method.call",
	"import",
	"injection.content",
	"injection.language",
	"keyframes",
	"keyword",
	"keyword.conditional",
	"keyword.conditional.ternary",
	"keyword.coroutine",
	"keyword.debug",
	"keyword.directive",
	"keyword.directive.define",
	"keyword.exception",
	"keyword.export",
	"keyword.function",
	"keyword.import",
	"keyword.modifier",
	"keyword.operator",
	"keyword.repeat",
	"keyword.return",
	"keyword.type",
	"label",
	"markup",
	"markup.environment",
	"markup.environment.name",
	"markup.heading",
	"markup.heading.1",
	"markup.heading.2",
	"markup.heading.3",
	"markup.heading.4",
	"markup.heading.5",
	"markup.heading.6",
	"markup.italic",
	"markup.link",
	"markup.link.label",
	"markup.link.url",
	"markup.list",
	"markup.list.checked",
	"markup.list.unchecked",
	"markup.math",
	"markup.quote",
	"markup.raw",
	"markup.raw.block",
	"markup.strikethrough",
	"markup.strong",
	"markup.underline",
	"media",
	"module",
	"module.builtin",
	"namespace",
	"number",
	"number.float",
	"operator",
	"property",
	"punctuation.bracket",
	"punctuation.delimiter",
	"punctuation.special",
	"string",
	"string.documentation",
	"string.escape",
	"string.regexp",
	"string.special",
	"string.special.path",
	"string.special.symbol",
	"string.special.url",
	"supports",
	"tag",
	"tag.attribute",
	"tag.builtin",
	"tag.delimiter",
	"type",
	"type.builtin",
	"type.definition",
	"variable",
	"variable.builtin",
	"variable.member",
	"variable.parameter",
	"variable.parameter.builtin",
}

-- Language-specific scopes for specialized capture groups
-- Safe to include all supported languages since we only generate if the scope exists and differs from base
local specialized_scopes = {
	-- Already supported
	"bash",
	"c",
	"c_sharp",
	"cpp",
	"css",
	"doc",
	"documentation",
	"elixir",
	"gitcommit",
	"gitignore",
	"html",
	"java",
	"javascript",
	"js",
	"json",
	"lua",
	"markdown",
	"markdown_inline",
	"php",
	"python",
	"regex",
	"ruby",
	"rust",
	"scss",
	"toml",
	"tsx",
	"yaml",
	-- Common languages
	"erlang",
	"fsharp",
	"go",
	"haskell",
	"kotlin",
	"nix",
	"ocaml",
	"scala",
	"sql",
	"swift",
	"typescript",
	"vim",
	"xml",
	"zig",
	-- Web/templating
	"angular",
	"astro",
	"eex",
	"glimmer",
	"heex",
	"jsx",
	"liquid",
	"surface",
	"svelte",
	"vue",
	-- Other supported languages
	"asm",
	"caddy",
	"clojure",
	"cmake",
	"commonlisp",
	"csv",
	"dart",
	"diff",
	"dockerfile",
	"elm",
	"fish",
	"gleam",
	"graphql",
	"hcl",
	"http",
	"iex",
	"latex",
	"llvm",
	"make",
	"objc",
	"perl",
	"powershell",
	"proto",
	"r",
	"typst",
}

-- Helper function to compare two styles for equality
local function styles_equal(a, b)
	if a == nil and b == nil then
		return true
	end
	if a == nil or b == nil then
		return false
	end
	return a.fg == b.fg
		and a.bg == b.bg
		and a.bold == b.bold
		and a.italic == b.italic
		and a.underline == b.underline
		and a.undercurl == b.undercurl
		and a.underdouble == b.underdouble
		and a.underdotted == b.underdotted
		and a.underdashed == b.underdashed
		and a.strikethrough == b.strikethrough
end

-- Helper function to extract style from highlight definition
local function extract_style(hl)
	local style = {}

	if hl.fg then
		style.fg = string.format("#%06x", hl.fg)
	end

	if hl.bg then
		style.bg = string.format("#%06x", hl.bg)
	end

	if hl.bold then
		style.bold = true
	end
	if hl.italic then
		style.italic = true
	end
	if hl.underline then
		style.underline = true
	end
	if hl.undercurl then
		style.undercurl = true
	end
	if hl.underdouble then
		style.underdouble = true
	end
	if hl.underdotted then
		style.underdotted = true
	end
	if hl.underdashed then
		style.underdashed = true
	end
	if hl.strikethrough then
		style.strikethrough = true
	end

	return style
end

local function get_plugin_revision(repo_url)
	local plugin_name = plugin_dir_name(repo_url)
	local plugins = vim.pack.get({ plugin_name }, { offline = true })

	if not plugins[1] or not plugins[1].rev then
		return "unknown"
	end

	return plugins[1].rev
end

local function is_incomplete_plugin_dir(plugin_path)
	if vim.fn.isdirectory(plugin_path) == 0 then
		return false
	end

	for _, entry in ipairs(vim.fn.readdir(plugin_path)) do
		if entry ~= ".git" then
			return false
		end
	end

	return true
end

local function extract_colorscheme_colors(theme)
	local colorscheme_name = vim.g.colors_name
	local appearance = vim.o.background
	local revision = get_plugin_revision(theme.url)

	print(
		string.format(
			"🎨 %s (colorscheme: %s | appearance: %s | revision: %s)\n",
			theme.name,
			colorscheme_name,
			appearance,
			revision
		)
	)

	vim.opt.termguicolors = true

	local all_groups = {}

	for _, group in ipairs(regular_groups) do
		table.insert(all_groups, group)
	end

	for _, group in ipairs(treesitter_groups) do
		table.insert(all_groups, "@" .. group)
	end

	local highlights = {}

	-- Extract base groups
	for _, group in ipairs(all_groups) do
		local hl = vim.api.nvim_get_hl(0, { name = group, link = false })
		local style = extract_style(hl)

		if next(style) ~= nil then
			local key = group_key_map[group] or string.lower(string.gsub(group, "@", ""))
			if key == "cursorline" then
				key = "highlighted"
			end
			highlights[key] = style
		end
	end

	-- Extract specialized (language-specific) groups
	for _, group in ipairs(treesitter_groups) do
		for _, scope in ipairs(specialized_scopes) do
			local specialized_group = "@" .. group .. "." .. scope
			local hl = vim.api.nvim_get_hl(0, { name = specialized_group, link = false })

			-- Only add if the highlight exists (non-empty table)
			if next(hl) ~= nil then
				local base_key = string.lower(group)
				local base_style = highlights[base_key]
				local specialized_style = extract_style(hl)

				-- Only add if it differs from the base style
				if not styles_equal(base_style, specialized_style) and next(specialized_style) ~= nil then
					local key = string.lower(group) .. "." .. scope
					highlights[key] = specialized_style
				end
			end
		end
	end

	local output_file = script_dir .. "/" .. theme.name .. ".json"
	local theme_data = {
		name = theme.name,
		appearance = appearance,
		revision = revision,
		highlights = highlights,
	}

	local json_str = vim.json.encode(theme_data)
	local file = io.open(output_file, "w")
	if file then
		file:write(json_str)
		file:close()
		print("✓ Wrote raw JSON to " .. output_file .. "\n")

		local jq_cmd = [[jq '
      {
        name,
        appearance,
        revision,
        highlights: (.highlights | to_entries | sort_by(.key) | map({
          key: .key,
          value: (
            {
              fg: .value.fg,
              bg: .value.bg,
              bold: .value.bold,
              italic: .value.italic,
              underline: .value.underline,
              undercurl: .value.undercurl,
              underdouble: .value.underdouble,
              underdotted: .value.underdotted,
              underdashed: .value.underdashed,
              strikethrough: .value.strikethrough
            }
          ) | with_entries(select(.value != null))
        }) | from_entries)
      }' ]] .. output_file .. " > " .. output_file .. ".tmp && mv " .. output_file .. ".tmp " .. output_file

		print("Running jq...\n")
		local jq_result = vim.fn.system(jq_cmd)

		if vim.v.shell_error ~= 0 then
			print("❌ jq processing failed (exit code " .. vim.v.shell_error .. "): " .. jq_result .. "\n")
		else
			print("✓ Formatted JSON with jq\n")
		end

		return true
	else
		print(string.format("❌ failed to write to file %s\n", output_file))
		return false
	end
end

local theme_name = arg and arg[1]
if not theme_name then
	print("❌ extract_theme.lua requires a theme name as an argument\n")
	os.exit(1)
end

package.path = table.concat({
	script_dir .. "/?.lua",
	script_dir .. "/?/init.lua",
	package.path,
}, ";")

local themes = require("themes")
local repo_names = {}

for _, theme_def in ipairs(themes) do
	local _, repo = parse_repo_parts(theme_def.url)
	if repo then
		local repo_key = repo:lower()
		repo_names[repo_key] = repo_names[repo_key] or {}
		repo_names[repo_key][theme_def.url] = true
	end
	if theme_def.dependencies then
		for _, dep_url in ipairs(theme_def.dependencies) do
			local _, dep_repo = parse_repo_parts(dep_url)
			if dep_repo then
				local dep_repo_key = dep_repo:lower()
				repo_names[dep_repo_key] = repo_names[dep_repo_key] or {}
				repo_names[dep_repo_key][dep_url] = true
			end
		end
	end
end

for repo, urls in pairs(repo_names) do
	if table_size(urls) > 1 then
		colliding_repo_names[repo:lower()] = true
	end
end

local theme = nil

for _, theme_def in ipairs(themes) do
	if theme_def.name == theme_name then
		theme = theme_def
		break
	end
end

if not theme then
	print(string.format("❌ theme '%s' not found in themes.lua\n", theme_name))
	os.exit(1)
end

local plugins_to_install = {}

if theme.dependencies then
	for _, dep_url in ipairs(theme.dependencies) do
		table.insert(plugins_to_install, { src = dep_url, name = plugin_dir_name(dep_url) })
	end
end

table.insert(plugins_to_install, { src = theme.url, name = plugin_dir_name(theme.url) })

local pack_dir = vim.fn.stdpath("data") .. "/site/pack/core/opt"
local plugin_names = {}

for _, plugin in ipairs(plugins_to_install) do
	table.insert(plugin_names, plugin.name)
	local plugin_path = pack_dir .. "/" .. plugin.name
	if is_incomplete_plugin_dir(plugin_path) then
		vim.pack.del({ plugin.name }, { force = true })
	end
end

print("📦 Installing plugins...\n")
vim.pack.add(plugins_to_install, { load = false, confirm = false })
vim.pack.update(plugin_names, { force = true })

local plugin_name = plugin_dir_name(theme.url)

local success = vim.wait(60000, function()
	local plugin_path = pack_dir .. "/" .. plugin_name
	return vim.fn.isdirectory(plugin_path) == 1
end, 100)

if not success then
	print("❌ Failed to install plugin\n")
	os.exit(1)
end

for _, plugin in ipairs(plugins_to_install) do
	vim.cmd.packadd(plugin.name)
	local plugin_path = pack_dir .. "/" .. plugin.name
	vim.opt.runtimepath:prepend(plugin_path)
	package.path = table.concat({
		plugin_path .. "/lua/?.lua",
		plugin_path .. "/lua/?/init.lua",
		package.path,
	}, ";")
end

local original_pack_add = vim.pack.add
vim.pack.add = function(specs, opts)
	local specs_to_install = {}

	for _, spec in ipairs(specs) do
		local src = type(spec) == "string" and spec or spec.src
		if not src or vim.fn.isdirectory(pack_dir .. "/" .. plugin_dir_name(src)) == 0 then
			table.insert(specs_to_install, spec)
		end
	end

	if #specs_to_install == 0 then
		return {}
	end

	return original_pack_add(specs_to_install, opts)
end

if theme.config then
	local configured, err = pcall(theme.config)
	vim.pack.add = original_pack_add
	if not configured then
		print(string.format("❌ Failed to configure theme '%s'\n", theme.name))
		print(string.format("   Error: %s\n", err))
		print(string.format("   Skipping theme extraction for '%s'\n\n", theme.name))
		os.exit(0)
	end
else
	vim.pack.add = original_pack_add
	print("⚠️  No config function found for theme\n")
end

extract_colorscheme_colors(theme)

vim.cmd("quit!")

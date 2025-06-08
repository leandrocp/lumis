local highlight_groups = {
	"Normal",
	"Comment",
	"@attribute",
	"@attribute.builtin",
	"@boolean",
	"@character",
	"@character.special",
	"@charset",
	"@clicke",
	"@comment",
	"@comment.documentation",
	"@comment.error",
	"@comment.warning",
	"@comment.todo",
	"@comment.note",
	"@constant",
	"@constant.builtin",
	"@constant.macro",
	"@constructor",
	"@diff.minus",
	"@diff.plus",
	"@diff.delta",
	"@error",
	"@function",
	"@function.builtin",
	"@function.call",
	"@function.macro",
	"@function.method",
	"@function.method.call",
	"@import",
	"@injection.content",
	"@injection.language",
	"@keyframes",
	"@keyword",
	"@keyword.conditional",
	"@keyword.conditional.ternary",
	"@keyword.coroutine",
	"@keyword.debug",
	"@keyword.directive",
	"@keyword.directive.define",
	"@keyword.exception",
	"@keyword.function",
	"@keyword.import",
	"@keyword.modifier",
	"@keyword.operator",
	"@keyword.repeat",
	"@keyword.return",
	"@keyword.type",
	"@label",
	"@markup.heading",
	"@markup.heading.1",
	"@markup.heading.2",
	"@markup.heading.3",
	"@markup.heading.4",
	"@markup.heading.5",
	"@markup.heading.6",
	"@markup.italic",
	"@markup.link",
	"@markup.link.label",
	"@markup.link.url",
	"@markup.list",
	"@markup.list.checked",
	"@markup.list.unchecked",
	"@markup.math",
	"@markup.quote",
	"@markup.raw",
	"@markup.raw.block",
	"@markup.strikethrough",
	"@markup.strong",
	"@markup.underline",
	"@media",
	"@module",
	"@module.builtin",
	"@namespace",
	"@number",
	"@number.float",
	"@operator",
	"@property",
	"@punctuation.bracket",
	"@punctuation.delimiter",
	"@punctuation.special",
	"@string",
	"@string.documentation",
	"@string.escape",
	"@string.regexp",
	"@string.special",
	"@string.special.path",
	"@string.special.symbol",
	"@string.special.url",
	"@supports",
	"@tag",
	"@tag.attribute",
	"@tag.builtin",
	"@tag.delimiter",
	"@type",
	"@type.builtin",
	"@type.definition",
	"@variable",
	"@variable.builtin",
	"@variable.member",
	"@variable.parameter",
	"@variable.parameter.builtin",
}

local function extract_colorscheme_colors(theme_def)
	local theme_name = theme_def.name
	local colorscheme_name = theme_def.colorscheme
	local appearance = theme_def.appearance

	print(string.format("%s (colorscheme: %s, appearance: %s)\n", theme_name, colorscheme_name, appearance))

	local preserved_modules = {
		"_G",
		"bit",
		"coroutine",
		"debug",
		"io",
		"lazy",
		"math",
		"os",
		"package",
		"string",
		"table",
		"vim",
		"jit",
	}
	local preserve_list = {}
	for _, mod in ipairs(preserved_modules) do
		preserve_list[mod] = true
	end

	for k in pairs(package.loaded) do
		if not preserve_list[k] then
			package.loaded[k] = nil
		end
	end

	if theme_def.before then
		theme_def.before()
	end

	vim.api.nvim_command("hi clear")
	vim.g.colors_name = nil
	if vim.fn.exists("syntax_on") then
		vim.api.nvim_command("syntax reset")
	end
	vim.opt.termguicolors = true
	vim.o.background = appearance

	local success, err = pcall(vim.cmd, "colorscheme " .. colorscheme_name)
	if not success then
		print(string.format("Error loading colorscheme for %s: %s", theme_name, err))
		return false
	end

	local highlights = {}

	for _, group in ipairs(highlight_groups) do
		local hl = vim.api.nvim_get_hl(0, { name = group, link = false })
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
		if hl.strikethrough then
			style.strikethrough = true
		end

		if next(style) ~= nil then
			highlights[string.lower(string.gsub(group, "@", ""))] = style
		end
	end

	local output_file = theme_name .. ".json"
	local theme_data = {
		name = theme_name,
		appearance = appearance,
		highlights = highlights,
	}

	local json_str = vim.json.encode(theme_data)
	local file = io.open(output_file, "w")
	if file then
		file:write(json_str)
		file:close()

		local jq_cmd = [[jq '
      {
        name,
        appearance,
        highlights: (.highlights | to_entries | sort_by(.key) | map({
          key: .key,
          value: {
		    fg: .value.fg,
            bg: .value.bg,
            bold: .value.bold,
            italic: .value.italic,
            undercurl: .value.undercurl,
            underline: .value.underline,
			strikethrough: .value.strikethrough
          } | with_entries(select(.value != null))
        }) | from_entries)
      }' ]] .. output_file .. " > " .. output_file .. ".tmp && mv " .. output_file .. ".tmp " .. output_file

		local jq_result = vim.fn.system(jq_cmd)

		if vim.v.shell_error ~= 0 then
			print("Warning: jq processing failed: " .. jq_result)
		end

		return true
	else
		print(string.format("Error: Could not write to file %s", output_file))
		return false
	end
end

local theme_name = arg and arg[1]
if not theme_name then
	print("extract_theme.lua requires a theme name as an argument.")
	os.exit(1)
end

local themes = require("themes")
local theme_def = nil

for _, theme in ipairs(themes) do
	if theme.name == theme_name then
		theme_def = theme
		break
	end
end

if not theme_def then
	print(string.format("Theme '%s' not found in themes.lua", theme_name))
	os.exit(1)
end

local plugins = {}
local plugin = vim.deepcopy(theme_def.plugin)
plugin.lazy = false
plugin.priority = 1000
table.insert(plugins, plugin)

require("lazy").setup(plugin, {
	checker = {
		enabled = true,
	},
})

extract_colorscheme_colors(theme_def)

vim.cmd("quit!")

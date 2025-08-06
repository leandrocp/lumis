local regular_groups = {
	"Normal",
	"Comment",
	"CursorLine",
}

local treesitter_groups = {
	"attribute",
	"attribute.builtin",
	"boolean",
	"character",
	"character.special",
	"charset",
	"clicke",
	"comment",
	"comment.documentation",
	"comment.error",
	"comment.hint",
	"comment.note",
	"comment.todo",
	"comment.warning",
	"constant",
	"constant.builtin",
	"constant.java",
	"constant.macro",
	"constructor",
	"constructor.lua",
	"constructor.python",
	"diff.delta",
	"diff.minus",
	"diff.plus",
	"error",
	"function",
	"function.builtin",
	"function.builtin.bash",
	"function.call",
	"function.macro",
	"function.method",
	"function.method.call",
	"function.method.call.php",
	"function.method.php",
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
	"keyword.directive.css",
	"keyword.directive.define",
	"keyword.exception",
	"keyword.export",
	"keyword.function",
	"keyword.import",
	"keyword.import.c",
	"keyword.import.cpp",
	"keyword.modifier",
	"keyword.operator",
	"keyword.repeat",
	"keyword.return",
	"keyword.type",
	"label",
	"label.yaml",
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
	"markup.link.label.html",
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
	"number.css",
	"number.float",
	"operator",
	"property",
	"property.class.css",
	"property.css",
	"property.id.css",
	"property.json",
	"property.scss",
	"property.toml",
	"property.yaml",
	"punctuation.bracket",
	"punctuation.delimiter",
	"punctuation.delimiter.regex",
	"punctuation.special",
	"string",
	"string.documentation",
	"string.escape",
	"string.plain.css",
	"string.regexp",
	"string.special",
	"string.special.path",
	"string.special.symbol",
	"string.special.symbol.ruby",
	"string.special.url",
	"string.special.url.html",
	"supports",
	"tag",
	"tag.attribute",
	"tag.builtin",
	"tag.delimiter",
	"type",
	"type.builtin",
	"type.css",
	"type.definition",
	"type.tag.css",
	"variable",
	"variable.builtin",
	"variable.member",
	"variable.parameter",
	"variable.parameter.bash",
	"variable.parameter.builtin",
}

local function extract_colorscheme_colors(theme)
	local Lock = require("lazy.manage.lock")
	local plugin = Lock.get(theme)

	if not plugin then
		print("❌ failed to find plugin in lock file, make sure plugin spec has a `name` field\n")
		os.exit(1)
	end

	local colorscheme_name = vim.g.colors_name
	local appearance = vim.o.background
	local revision = plugin["commit"]

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

	for _, group in ipairs(all_groups) do
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
			local key = string.lower(string.gsub(group, "@", ""))
			if key == "cursorline" then
				key = "highlighted"
			end
			highlights[key] = style
		end
	end

	local output_file = theme.name .. ".json"
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
              undercurl: .value.undercurl,
              underline: .value.underline,
              strikethrough: .value.strikethrough
            }
          ) | with_entries(select(.value != null))
        }) | from_entries)
      }' ]] .. output_file .. " > " .. output_file .. ".tmp && mv " .. output_file .. ".tmp " .. output_file

		local jq_result = vim.fn.system(jq_cmd)

		if vim.v.shell_error ~= 0 then
			print("❌ jq processing failed: " .. jq_result .. "\n")
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

local themes = require("themes")
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

local plugins = {}
local plugin = vim.deepcopy(theme)
plugin.lazy = false
plugin.priority = 1000
table.insert(plugins, plugin)

require("lazy").setup(plugin, {
	checker = {
		enabled = true,
	},
})

extract_colorscheme_colors(theme)

vim.cmd("quit!")

defmodule Lumis do
  @external_resource "README.md"

  @moduledoc "README.md"
             |> File.read!()
             |> String.split("<!-- MDOC -->")
             |> Enum.fetch!(1)

  require Logger
  alias Lumis.Theme

  @typedoc """
  A language name, filename, or path with extension.

  See `Lumis.available_languages/0` to list all available languages or check out a list of [available languages](https://docs.rs/lumis/latest/lumis/#languages-available).

  ## Examples

      - "elixir"
      - ".ex"
      - "app.ex"
      - "lib/app.ex"

  """
  @type language :: String.t() | nil

  @typedoc """
  Theme used to apply styles on the highlighted source code.

  See `Lumis.available_themes/0` to list all available themes or check out a list of [available themes](https://docs.rs/lumis/latest/lumis/#themes-available).
  """
  @type theme :: String.t() | Lumis.Theme.t() | nil

  @typedoc """
  Highlight lines options for Inline HTML formatter.
  """
  @type html_inline_highlight_lines ::
          %{
            lines: [pos_integer() | Range.t()],
            style: :theme | String.t() | nil,
            class: String.t() | nil
          }
          | nil

  @typedoc """
  Highlight lines options for Linked HTML formatter.
  """
  @type html_linked_highlight_lines ::
          %{
            lines: [pos_integer() | Range.t()],
            class: String.t()
          }
          | nil

  @typedoc """
  Wraps the highlighted code with custom open and close HTML tags.
  """
  @type header ::
          %{
            close_tag: String.t(),
            open_tag: String.t()
          }
          | nil

  @typedoc """
  Options for HTML Multi-Themes formatter.

  The themes are specified as a keyword list where keys are CSS identifiers (atoms)
  and values are theme names (strings) or Theme structs.
  """
  @type html_multi_themes_options ::
          %{
            themes: keyword(theme()),
            default_theme: String.t() | nil,
            css_variable_prefix: String.t() | nil,
            pre_class: String.t() | nil,
            italic: boolean(),
            include_highlights: boolean(),
            highlight_lines: html_inline_highlight_lines() | nil,
            header: header()
          }
          | nil

  @typedoc """
  Highlighter formatter and its options.

  Available formatters: `:html_inline`, `:html_linked`, `:html_multi_themes`, `:terminal`, `:bbcode_scoped`

  * `:html_inline` - generates `<span>` tags with inline styles for each token, for example: `<span style="color: #6eb4bff;">Atom</span>`.
  * `:html_linked` - generates `<span>` tags with `class` representing the token type, for example: `<span class="l-keyword-special">Atom</span>`.
     Must link an external CSS in order to render colors, see more at [HTML Linked](https://hexdocs.pm/lumis/Lumis.html#module-html-linked).
  * `:html_multi_themes` - generates HTML with CSS custom properties (variables) for multiple themes, enabling light/dark mode support.
     Inspired by [Shiki Dual Themes](https://shiki.style/guide/dual-themes).
  * `:terminal` - generates ANSI escape codes for terminal output.
  * `:bbcode_scoped` - generates nested BBCode tags using highlight scope names, for example: `[keyword-elixir]defmodule[/keyword-elixir]`.

  You can either pass the formatter as an atom to use default options or a tuple with the formatter name and options, so both are equivalent:

      # passing only the formatter name like below:
      :html_inline
      # is the same as passing an empty list of options:
      {:html_inline, []}

  ## Available Options:

  * `html_inline`:

      - `:language` (`t:language/0` - default: `nil`) - the language used by the formatter. When omitted, Lumis tries to auto-detect it from the source.
      - `:theme` (`t:theme/0` - default: `nil`) - the theme to apply styles on the highlighted source code.
      - `:pre_class` (`t:String.t/0` - default: `nil`) - the CSS class to append into the wrapping `<pre>` tag.
      - `:italic` (`t:boolean/0` - default: `false`) - enable italic style for the highlighted code.
      - `:include_highlights` (`t:boolean/0` - default: `false`) - include the highlight scope name in a `data-highlight` attribute. Useful for debugging.
      - `:rainbow_brackets` (`t:boolean/0` - default: `false`) - render nested brackets with rainbow bracket scopes.
      - `:highlight_lines` (`t:html_inline_highlight_lines/0` - default: `nil`) - highlight specific lines either using the theme `highlighted` style or with custom CSS styling.
      - `:header` (`t:header/0` - default: `nil`) - wrap the highlighted code with custom open and close HTML tags.

  * `html_linked`:

      - `:language` (`t:language/0` - default: `nil`) - the language used by the formatter. When omitted, Lumis tries to auto-detect it from the source.
      - `:pre_class` (`t:String.t/0` - default: `nil`) - the CSS class to append into the wrapping `<pre>` tag.
      - `:rainbow_brackets` (`t:boolean/0` - default: `false`) - render nested brackets with rainbow bracket scopes.
      - `:highlight_lines` (`t:html_linked_highlight_lines/0` - default: `nil`) - highlight specific lines either using the `l-highlighted` class from themes or with a custom CSS class.
      - `:header` (`t:header/0` - default: `nil`) - wrap the highlighted code with custom open and close HTML tags.

  * `html_multi_themes`:

      - `:language` (`t:language/0` - default: `nil`) - the language used by the formatter. When omitted, Lumis tries to auto-detect it from the source.
      - `:themes` (`keyword(theme())` - required) - keyword list of theme identifiers to theme names/structs. Theme identifiers become CSS class names and CSS variable prefixes. Example: `[light: "github_light", dark: "github_dark"]`.
      - `:default_theme` (`t:String.t/0` - default: `nil`) - controls inline color rendering: specify a theme identifier for inline colors, use `"light-dark()"` for CSS light-dark() function, or `nil` for CSS variables only.
      - `:css_variable_prefix` (`t:String.t/0` - default: `nil`) - CSS variable prefix (defaults to `"--lumis"` if nil). Generates variables like `--lumis-light` (color), `--lumis-light-bg` (background), `--lumis-light-font-style`, etc.
      - `:pre_class` (`t:String.t/0` - default: `nil`) - the CSS class to append into the wrapping `<pre>` tag.
      - `:italic` (`t:boolean/0` - default: `false`) - enable italic style for the highlighted code.
      - `:include_highlights` (`t:boolean/0` - default: `false`) - include the highlight scope name in a `data-highlight` attribute.
      - `:rainbow_brackets` (`t:boolean/0` - default: `false`) - render nested brackets with rainbow bracket scopes.
      - `:highlight_lines` (`t:html_inline_highlight_lines/0` - default: `nil`) - highlight specific lines (same as html_inline).
      - `:header` (`t:header/0` - default: `nil`) - wrap the highlighted code with custom open and close HTML tags.

  * `terminal`:

      - `:language` (`t:language/0` - default: `nil`) - the language used by the formatter. When omitted, Lumis tries to auto-detect it from the source.
      - `:theme` (`t:theme/0` - default: `nil`) - the theme to apply styles on the highlighted source code.
      - `:background` (`:theme | t:String.t/0 | nil` - default: `nil`) - fallback background behavior: `nil` inherits the output background, `:theme` uses the theme's normal background color, and a string uses that color.
      - `:width` (`pos_integer() | nil` - default: `nil`) - pad each rendered terminal line to the given width. This is most useful with `:background`.
      - `:rainbow_brackets` (`t:boolean/0` - default: `false`) - render nested brackets with rainbow bracket scopes.

  * `bbcode_scoped`:

      - `:language` (`t:language/0` - default: `nil`) - available when passed as `{:bbcode_scoped, ...}`.
      - `:rainbow_brackets` (`t:boolean/0` - default: `false`) - render nested brackets with rainbow bracket scopes.

  ## Examples

  ### Inline HTML formatter with default options

      :html_inline

  There is no default theme, so this emits `<span>` tags without any `style` attribute.
  Pass `:theme` to get colors, or use `:html_linked` to style the output with a CSS file.

  ### Inline HTML formatter with custom options

      {:html_inline, theme: "onedark", pre_class: "example-01", include_highlights: true}

  ### HTML Inline: highlight specific lines

      # apply theme's `highlighted` style
      {:html_inline, theme: "onedark", highlight_lines: %{lines: [2..4, 6], style: :theme}}

      # style: :theme is the default
      {:html_inline, theme: "onedark", highlight_lines: %{lines: [1, 2, 3]}}

      # explicitly use theme style
      {:html_inline, theme: "onedark", highlight_lines: %{lines: [1, 2, 3], style: :theme}}

      # overrides default style
      {:html_inline, theme: "onedark", highlight_lines: %{lines: [1, 3..5, 8], style: "background-color: #fff3cd; border-left: 3px solid #ffc107;"}}

      # with only class and no style
      {:html_inline, theme: "onedark", highlight_lines: %{lines: [1, 2, 3], style: nil, class: "transition-colors duration-500 w-full inline-block bg-yellow-500"}}

  ### HTML Linked: highlight specific lines

      # use default `l-highlighted` class (already present in themes)
      {:html_linked, highlight_lines: %{lines: [2..4, 6]}}

      # use custom class
      {:html_linked, highlight_lines: %{lines: [1, 2, 3], class: "error-line"}}

  ### Wrap with custom open and close HTML tags

      header = %{
        open_tag: "<div class=\"code-header\"><span>file: app.ex</span>",
        close_tag: "</div>"
      }
      {:html_inline, theme: "onedark", header: header}

  ### HTML Multi-Themes: Light/Dark mode support

      # Basic dual theme with CSS variables
      {:html_multi_themes, themes: [light: "github_light", dark: "github_dark"]}

      # With light-dark() function for automatic theme switching based on system preference
      {:html_multi_themes,
       themes: [light: "github_light", dark: "github_dark"],
       default_theme: "light-dark()"}

      # With inline colors for default theme and CSS variables for others
      {:html_multi_themes,
       themes: [light: "github_light", dark: "github_dark"],
       default_theme: "light"}

      # Multiple themes with custom prefix
      {:html_multi_themes,
       themes: [light: "github_light", dark: "github_dark", dim: "catppuccin_frappe"],
       css_variable_prefix: "--code"}

      # With Theme structs instead of strings
      light_theme = Lumis.Theme.get("github_light")
      dark_theme = Lumis.Theme.get("github_dark")
      {:html_multi_themes, themes: [light: light_theme, dark: dark_theme]}

  ### Terminal formatter

      :terminal

      {:terminal, theme: "github_light"}

      {:terminal, theme: "dracula", background: :theme, width: 120}

      {:terminal, theme: "dracula", background: "#282a36", width: 120}

  ### BBCode Scoped formatter

      :bbcode_scoped

  Emits highlight scope names as tags, not standard forum-style BBCode like `[b]`, `[color]`, or `[code]`.

  See https://docs.rs/lumis/latest/lumis/enum.FormatterOption.html for more info.
  """
  @type formatter ::
          :html_inline
          | {:html_inline,
             [
               language: language(),
               theme: theme(),
               pre_class: String.t(),
               italic: boolean(),
               include_highlights: boolean(),
               rainbow_brackets: boolean(),
               highlight_lines: html_inline_highlight_lines(),
               header: header()
             ]}
          | :html_linked
          | {:html_linked,
             [
               language: language(),
               pre_class: String.t(),
               rainbow_brackets: boolean(),
               highlight_lines: html_linked_highlight_lines(),
               header: header()
             ]}
          | :html_multi_themes
          | {:html_multi_themes,
             [
               language: language(),
               themes: keyword(theme()),
               default_theme: String.t(),
               css_variable_prefix: String.t(),
               pre_class: String.t(),
               italic: boolean(),
               include_highlights: boolean(),
               rainbow_brackets: boolean(),
               highlight_lines: html_inline_highlight_lines(),
               header: header()
             ]}
          | :terminal
          | {:terminal,
             [
               language: language(),
               theme: theme(),
               background: :theme | String.t() | nil,
               width: pos_integer() | nil,
               rainbow_brackets: boolean()
             ]}
          | :bbcode_scoped
          | {:bbcode_scoped, [language: language(), rainbow_brackets: boolean()]}

  @highlight_options [
    rainbow_brackets: [type: :boolean, default: false]
  ]

  @formatter_schema [
    type: {:custom, Lumis, :formatter_type, []},
    type_spec: quote(do: Lumis.formatter()),
    type_doc: "`t:Lumis.formatter/0`",
    default: {:html_inline, []},
    doc: "Formatter to apply on the highlighted source code. See the type doc for more info."
  ]

  @options_schema [
    language: [
      type: {:or, [:string, nil]},
      type_spec: quote(do: Lumis.language()),
      type_doc: "`t:Lumis.language/0`",
      deprecated:
        "Use the :language option inside the formatter tuple instead, eg: {:html_inline, language: \"elixir\"}"
    ],
    formatter: @formatter_schema,
    theme: [
      type: {:or, [{:struct, Lumis.Theme}, :string, nil]},
      deprecated: "Use :formatter instead."
    ],
    inline_style: [
      type: :boolean,
      deprecated: "Use :formatter instead."
    ],
    pre_class: [
      type: {:or, [:string, nil]},
      deprecated: "Use :formatter instead."
    ]
  ]

  @doc false
  def formatter_schema, do: @formatter_schema

  @doc false
  def options_schema, do: @options_schema

  @doc false
  def formatter_type(formatter)
      when formatter in [
             :html_inline,
             :html_linked,
             :html_multi_themes,
             :bbcode_scoped,
             :terminal
           ] do
    formatter_type({formatter, []})
  end

  def formatter_type({:html_inline, options}) when is_list(options) do
    schema =
      [
        language: [type: {:or, [:string, nil]}, default: nil],
        theme: [type: {:or, [{:struct, Lumis.Theme}, :string, nil]}, default: nil],
        pre_class: [type: {:or, [:string, nil]}, default: nil],
        italic: [type: :boolean, default: false],
        include_highlights: [type: :boolean, default: false],
        highlight_lines: [
          type:
            {:or,
             [
               nil,
               map: [
                 lines: [type: {:list, {:custom, Lumis, :highlight_lines_type, []}}],
                 style: [type: {:or, [:string, {:in, [:theme]}, nil]}, default: :theme],
                 class: [type: {:or, [:string, nil]}, default: nil]
               ]
             ]},
          default: nil
        ],
        header: [
          type:
            {:or,
             [
               nil,
               map: [
                 open_tag: [type: :string],
                 close_tag: [type: :string]
               ]
             ]},
          default: nil
        ]
      ] ++ @highlight_options

    case NimbleOptions.validate(options, schema) do
      {:ok, validated_opts} ->
        case convert_html_inline_options(validated_opts) do
          {:ok, converted_opts} ->
            {:ok, {:html_inline, converted_opts}}

          {:error, error} ->
            {:error, "invalid options given to html_inline: #{error}"}
        end

      {:error, error} ->
        {:error, "invalid options given to html_inline: #{inspect(error)}"}
    end
  end

  def formatter_type({:html_linked, options}) when is_list(options) do
    schema =
      [
        language: [type: {:or, [:string, nil]}, default: nil],
        pre_class: [type: {:or, [:string, nil]}, default: nil],
        highlight_lines: [
          type:
            {:or,
             [
               nil,
               map: [
                 lines: [type: {:list, {:custom, Lumis, :highlight_lines_type, []}}],
                 class: [type: :string, default: "l-highlighted"]
               ]
             ]},
          default: nil
        ],
        header: [
          type:
            {:or,
             [
               nil,
               map: [
                 open_tag: [type: :string],
                 close_tag: [type: :string]
               ]
             ]},
          default: nil
        ]
      ] ++ @highlight_options

    case NimbleOptions.validate(options, schema) do
      {:ok, validated_opts} ->
        case convert_html_linked_options(validated_opts) do
          {:ok, converted_opts} ->
            {:ok, {:html_linked, converted_opts}}

          {:error, error} ->
            {:error, "invalid options given to html_linked: #{error}"}
        end

      {:error, error} ->
        {:error, "invalid options given to html_linked: #{inspect(error)}"}
    end
  end

  def formatter_type({:html_multi_themes, options}) when is_list(options) do
    schema =
      [
        language: [type: {:or, [:string, nil]}, default: nil],
        themes: [
          type: :keyword_list,
          required: true,
          doc:
            "Keyword list of theme identifiers to theme names/structs, e.g., [light: \"github_light\", dark: \"github_dark\"]"
        ],
        default_theme: [
          type: {:or, [:string, nil]},
          default: nil,
          doc:
            "Default theme rendering mode: theme name, \"light-dark()\", or nil for CSS variables only"
        ],
        css_variable_prefix: [
          type: {:or, [:string, nil]},
          default: nil,
          doc: "CSS variable prefix (defaults to \"--lumis\" if nil)"
        ],
        pre_class: [type: {:or, [:string, nil]}, default: nil],
        italic: [type: :boolean, default: false],
        include_highlights: [type: :boolean, default: false],
        highlight_lines: [
          type:
            {:or,
             [
               nil,
               map: [
                 lines: [type: {:list, {:custom, Lumis, :highlight_lines_type, []}}],
                 style: [type: {:or, [:string, {:in, [:theme]}, nil]}, default: :theme],
                 class: [type: {:or, [:string, nil]}, default: nil]
               ]
             ]},
          default: nil
        ],
        header: [
          type:
            {:or,
             [
               nil,
               map: [
                 open_tag: [type: :string],
                 close_tag: [type: :string]
               ]
             ]},
          default: nil
        ]
      ] ++ @highlight_options

    case NimbleOptions.validate(options, schema) do
      {:ok, validated_opts} ->
        case convert_html_multi_themes_options(validated_opts) do
          {:ok, converted_opts} ->
            {:ok, {:html_multi_themes, converted_opts}}

          {:error, error} ->
            {:error, "invalid options given to html_multi_themes: #{error}"}
        end

      {:error, error} ->
        {:error, "invalid options given to html_multi_themes: #{inspect(error)}"}
    end
  end

  def formatter_type({:terminal, options}) when is_list(options) do
    schema =
      [
        language: [type: {:or, [:string, nil]}, default: nil],
        theme: [type: {:or, [{:struct, Lumis.Theme}, :string, nil]}, default: nil],
        background: [type: {:or, [:string, {:in, [:theme]}, nil]}, default: nil],
        width: [type: {:or, [:pos_integer, nil]}, default: nil]
      ] ++ @highlight_options

    case NimbleOptions.validate(options, schema) do
      {:ok, validated_opts} ->
        {:ok, {:terminal, validated_opts}}

      {:error, error} ->
        {:error, "invalid options given to terminal: #{inspect(error)}"}
    end
  end

  def formatter_type({:bbcode_scoped, options}) when is_list(options) do
    case Keyword.keys(options) -- [:language, :rainbow_brackets] do
      [] ->
        {:ok, {:bbcode_scoped, Keyword.merge([language: nil, rainbow_brackets: false], options)}}

      invalid ->
        {:error, "invalid options given to bbcode_scoped: #{inspect(invalid)}"}
    end
  end

  def formatter_type(other) do
    {:error, "invalid formatter option: #{inspect(other)}"}
  end

  @doc false
  defp convert_html_inline_options(opts) do
    with {:ok, opts} <- convert_highlight_lines_inline(opts) do
      convert_header(opts)
    end
  end

  @doc false
  defp convert_html_linked_options(opts) do
    with {:ok, opts} <- convert_highlight_lines_linked(opts) do
      convert_header(opts)
    end
  end

  defp convert_html_multi_themes_options(opts) do
    with {:ok, opts} <- validate_and_convert_themes(opts),
         {:ok, opts} <- convert_highlight_lines_inline(opts) do
      convert_header(opts)
    end
  end

  defp validate_and_convert_themes(opts) do
    case opts[:themes] do
      nil ->
        {:error, "themes option is required for html_multi_themes"}

      [] ->
        {:error, "themes list cannot be empty"}

      themes when is_list(themes) ->
        convert_themes_keyword_list(themes, opts)

      _ ->
        {:error, "themes must be a keyword list"}
    end
  end

  defp convert_themes_keyword_list(themes, opts) do
    themes
    |> Enum.reduce_while({:ok, %{}}, fn {id, theme_value}, {:ok, acc} ->
      theme_id = to_string(id)

      case resolve_theme(theme_value) do
        {:ok, theme_struct} ->
          {:cont, {:ok, Map.put(acc, theme_id, theme_struct)}}

        {:error, reason} ->
          {:halt, {:error, "failed to resolve theme #{inspect(id)}: #{reason}"}}
      end
    end)
    |> case do
      {:ok, themes_map} ->
        {:ok, Keyword.put(opts, :themes, themes_map)}

      {:error, _} = error ->
        error
    end
  end

  defp resolve_theme(%Lumis.Theme{} = theme), do: {:ok, theme}

  defp resolve_theme(theme_name) when is_binary(theme_name) do
    case Lumis.Theme.get(theme_name) do
      nil -> {:error, "theme '#{theme_name}' not found"}
      theme -> {:ok, theme}
    end
  end

  defp resolve_theme(other) do
    {:error, "expected theme name (string) or Lumis.Theme struct, got: #{inspect(other)}"}
  end

  @doc false
  defp convert_highlight_lines_inline(opts) do
    case opts[:highlight_lines] do
      nil ->
        {:ok, opts}

      hl ->
        lines =
          Enum.map(hl[:lines] || [], fn
            %Range{} = range -> {:range, %{start: range.first, end: range.last}}
            n when is_integer(n) -> {:single, n}
          end)

        style =
          case hl[:style] do
            :theme -> :theme
            str when is_binary(str) -> {:style, %{style: str}}
            nil -> nil
            _ -> :theme
          end

        class = hl[:class]

        opts
        |> Keyword.put(:highlight_lines, %Lumis.HtmlInlineHighlightLines{
          lines: lines,
          style: style,
          class: class
        })
        |> then(&{:ok, &1})
    end
  end

  @doc false
  defp convert_highlight_lines_linked(opts) do
    case opts[:highlight_lines] do
      nil ->
        {:ok, opts}

      hl ->
        lines =
          Enum.map(hl[:lines] || [], fn
            %Range{} = range -> {:range, %{start: range.first, end: range.last}}
            n when is_integer(n) -> {:single, n}
          end)

        class = hl[:class] || "l-highlighted"

        opts
        |> Keyword.put(:highlight_lines, %Lumis.HtmlLinkedHighlightLines{
          lines: lines,
          class: class
        })
        |> then(&{:ok, &1})
    end
  end

  @doc false
  defp convert_header(opts) do
    case opts[:header] do
      nil ->
        {:ok, opts}

      %{open_tag: open_tag, close_tag: close_tag} ->
        opts
        |> Keyword.put(:header, %Lumis.HtmlElement{
          open_tag: open_tag,
          close_tag: close_tag
        })
        |> then(&{:ok, &1})

      _ ->
        {:error,
         "invalid value for :header option, must be a map with :open_tag and :close_tag keys"}
    end
  end

  @doc false
  def highlight_lines_type(line) when is_integer(line), do: {:ok, line}

  def highlight_lines_type(%Range{} = range), do: {:ok, range}

  def highlight_lines_type(other),
    do: {:error, "invalid highlight line type: #{inspect(other)}"}

  @typedoc """
  #{NimbleOptions.docs(@options_schema)}

  See each option type for more info.
  """
  @type options() :: [unquote(NimbleOptions.option_typespec(@options_schema))]

  @doc """
  Returns all default options.
  """
  @spec default_options() :: options()
  def default_options, do: validate_options!([])

  @typedoc "What Lumis knows about one language."
  @type language_info :: %{
          id: String.t(),
          name: String.t(),
          aliases: [String.t()],
          extensions: [String.t()],
          globs: [String.t()],
          emacs_modes: [String.t()],
          shebangs: [String.t()]
        }

  @doc """
  Returns every available language and what the catalog knows about it, sorted
  by id.

  ## Example

      iex> Lumis.available_languages() |> Enum.find(&(&1.id == "elixir"))
      %{
        id: "elixir",
        name: "Elixir",
        aliases: [],
        extensions: ["*.ex", "*.exs"],
        globs: ["*.ex", "*.exs"],
        emacs_modes: ["elixir"],
        shebangs: ["elixir"]
      }

  """
  @spec available_languages() :: [language_info()]
  def available_languages, do: Lumis.Native.available_languages()

  @doc """
  Returns the ids of the languages loaded into this VM, sorted.

  The complement of `available_languages/0`: what can be highlighted right now
  without a download. Loading is global to the VM, so this is the same list in
  every process.

  ## Example

      iex> Lumis.Languages.load("elixir")
      iex> Lumis.loaded_languages()
      ["elixir"]

  """
  @spec loaded_languages() :: [id :: String.t()]
  def loaded_languages, do: Lumis.Native.loaded_languages()

  @typedoc "A built-in theme's name and appearance, without its highlight data."
  @type theme_info :: %{name: String.t(), appearance: String.t()}

  @doc """
  Returns every built-in theme's name and appearance, sorted by name.

  Use `Lumis.Theme.get/2` to get the actual theme struct.

  ## Example

      iex> Lumis.available_themes() |> Enum.find(&(&1.name == "github_light"))
      %{name: "github_light", appearance: "light"}

  """
  @spec available_themes() :: [theme_info()]
  def available_themes, do: Lumis.Native.available_themes()

  @deprecated "Use highlight/2 instead"
  def highlight(language, source, options) do
    IO.warn("""
      passing the language in the first argument is deprecated, use the formatter language option instead:

        Lumis.highlight("import Kernel", formatter: {:html_inline, language: "elixir"})

    """)

    {_, options} =
      Keyword.get_and_update(options, :theme, fn
        nil -> {nil, nil}
        current -> {current, String.capitalize(current)}
      end)

    options = put_formatter_language(options, language)

    highlight(source, options)
  end

  @deprecated "Use highlight!/2 instead"
  def highlight!(language, source, options) do
    IO.warn("""
      passing the language in the first argument is deprecated, use the formatter language option instead:

        Lumis.highlight!("import Kernel", formatter: {:html_inline, language: "elixir"})

    """)

    {_, options} =
      Keyword.get_and_update(options, :theme, fn
        nil -> {nil, nil}
        current -> {current, String.capitalize(current)}
      end)

    options = put_formatter_language(options, language)
    highlight!(source, options)
  end

  @doc """
  Highlights `source` code and outputs into a formatted string.

  Returns `{:error, reason}` when the root language cannot be loaded or the
  formatter fails. An injected language that cannot be fetched is not an error:
  that block stays plain and the rest of the document still highlights. Use
  `highlight!/2` to raise instead.

  Invalid *options* still raise, because those are a caller mistake rather than
  a runtime condition.

  ## Options

  See `t:options/0`.

  ## Examples

  Defining the language name:

      iex> Lumis.highlight("Atom.to_string(:elixir)", formatter: {:html_inline, language: "elixir"})
      {
        :ok,
        <pre class="lumis" style="color: #abb2bf; background-color: #282c34;"><code class="language-elixir" translate="no" tabindex="0"><div class="l-line" data-line="1"><span style="color: #e5c07b;">Atom</span><span style="color: #56b6c2;">.</span><span style="color: #61afef;">to_string</span><span style="color: #c678dd;">(</span><span style="color: #e06c75;">:elixir</span><span style="color: #c678dd;">)</span>
        </div></code></pre>
      }

  Guessing the language based on the provided source code:

      iex> Lumis.highlight("#!/usr/bin/env bash\\nID=1")
      {:ok, "<pre class=\"lumis\" ...><code class=\"language-bash\" ...>...</code></pre>"}

  With custom options:

      iex> Lumis.highlight("Atom.to_string(:elixir)", formatter: {:html_inline, language: "example.ex", pre_class: "example-elixir"})
      {:ok, "<pre class=\"lumis example-elixir\" ...><code ...>...</code></pre>"}

  Terminal formatter:

      iex> Lumis.highlight("Atom.to_string(:elixir)", formatter: {:terminal, language: "elixir"})
      {:ok, "\e[0m\e[38;2;229;192;123mAtom\e[0m\e[0m\e[38;2;86;182;194m.\e[0m\e[0m\e[38;2;97;175;239mto_string\e[0m\e[0m\e[38;2;198;120;221m(\e[0m\e[0m\e[38;2;224;108;117m:elixir\e[0m\e[0m\e[38;2;198;120;221m)\e[0m"}

  Highlighting specific lines in HTML Inline formatter:

      iex> code = \"""
      ...> defmodule Example do
      ...>   @lang = :elixir
      ...>   def lang, do: @lang
      ...> end
      ...> \"""
      iex> highlight_lines = %{lines: [2]}
      iex> Lumis.highlight(code, formatter: {:html_inline, language: "elixir", highlight_lines: highlight_lines})
      # Line 2 will be highlighted with the theme's `highlighted` style:
      <div class=\"l-line\" style=\"background-color: #414858;\" data-line=\"2\">...</div>

  Highlighting specific lines in HTML Linked formatter:
      
      iex> code = \"""
      ...> defmodule Example do
      ...>   @lang = :elixir
      ...>   def lang, do: @lang
      ...> end
      ...> \"""
      iex> highlight_lines = %{lines: [2]}
      iex> Lumis.highlight(code, formatter: {:html_linked, language: "elixir", highlight_lines: highlight_lines})
      # Line 2 will contain a `l-highlighted` class:
      <div class=\"l-line l-highlighted\" data-line=\"2\">...

  Wrapping with custom HTML:

      iex> header = %{
      ...>   open_tag: "<figure><span>file: example.exs</span>",
      ...>   close_tag: "</figure>"
      ...> }
      iex> Lumis.highlight("IO.puts('hello')", formatter: {:html_inline, language: "elixir", header: header})
      # Returns: "<div class='code-block' data-lang='elixir'><pre class='lumis'>...</pre></div>"
      {:ok, "<figure><span>file: example.exs</span><pre...><code ...>...</code></pre></figure>"}

  See https://docs.rs/lumis/latest/lumis/fn.highlight.html for more info.

  """
  @spec highlight(String.t(), options()) :: {:ok, String.t()} | {:error, String.t()}
  def highlight(source, options \\ [])

  def highlight(source, options) when is_binary(source) and is_list(options) do
    options =
      options
      |> validate_options!()
      |> rust_options!()

    case Lumis.Native.highlight(source, options) do
      {:error, {:language_not_loaded, language}} ->
        {:error,
         "language #{inspect(language)} could not be loaded. Warm it with " <>
           "`Lumis.Languages.async_load([#{inspect(language)}])` from your " <>
           "application's start/2 if this host has no network access"}

      other ->
        other
    end
  end

  def highlight(language, source)
      when is_binary(language) and is_binary(source) do
    highlight(source, language: language)
  end

  @doc """
  Validates the given options against the options schema.

  This function validates the provided options using NimbleOptions and the defined schema.
  It ensures that all options are valid and properly typed before being passed to the
  highlighting functions.

  ## Examples

      iex> Lumis.validate_options!(formatter: {:html_inline, language: "elixir"})
      [formatter: {:html_inline, [header: nil, highlight_lines: nil, include_highlights: false, italic: false, pre_class: nil, theme: nil, language: "elixir"]}]

      iex> Lumis.validate_options!(formatter: {:html_inline, theme: "dracula"})
      [formatter: {:html_inline, [theme: "dracula", ...]}]

      iex> Lumis.validate_options!(language: :invalid)
      ** (NimbleOptions.ValidationError)

  """
  @spec validate_options!(options()) :: options()
  def validate_options!(options) do
    options
    |> NimbleOptions.validate!(@options_schema)
    |> normalize_formatter_language()
  end

  @doc false
  def rust_options!(options) do
    {formatter, formatter_opts} = options[:formatter]
    {language, formatter_opts} = Keyword.pop(formatter_opts, :language)
    options = Keyword.delete(options, :language)

    {theme, options} = Keyword.pop(options, :theme)
    theme = build_theme(theme || Keyword.get(formatter_opts, :theme))

    {pre_class, options} = Keyword.pop(options, :pre_class)
    pre_class = pre_class || Keyword.get(formatter_opts, :pre_class)

    {inline_style, options} = Keyword.pop(options, :inline_style)

    {formatter, formatter_opts} =
      case inline_style do
        true ->
          {:ok, {_type, default_opts}} = formatter_type(:html_inline)
          {:html_inline, Keyword.merge(default_opts, formatter_opts)}

        false ->
          {:ok, {_type, default_opts}} = formatter_type(:html_linked)
          {:html_linked, Keyword.merge(default_opts, formatter_opts)}

        nil ->
          {formatter, formatter_opts}
      end

    rust_formatter =
      convert_formatter_for_nif(
        formatter,
        Map.merge(Map.new(formatter_opts), %{theme: theme, pre_class: pre_class})
      )

    options
    |> Keyword.put(:language, language)
    |> Keyword.put(:formatter, rust_formatter)
    |> Map.new()
  end

  defp normalize_formatter_language(options) do
    language = Keyword.get(options, :language)
    {formatter, formatter_opts} = Keyword.fetch!(options, :formatter)

    formatter_language = Keyword.get(formatter_opts, :language)
    formatter_opts = Keyword.put(formatter_opts, :language, formatter_language || language)

    Keyword.put(options, :formatter, {formatter, formatter_opts})
  end

  defp put_formatter_language(options, language) do
    {formatter, options} = Keyword.pop(options, :formatter)

    formatter =
      case formatter do
        nil -> {:html_inline, [language: language]}
        {name, opts} when is_list(opts) -> {name, Keyword.put_new(opts, :language, language)}
        name when is_atom(name) -> {name, [language: language]}
      end

    Keyword.put(options, :formatter, formatter)
  end

  @doc false
  def build_theme(theme) do
    cond do
      match?(%Theme{}, theme) ->
        {:theme, theme}

      is_binary(theme) && String.contains?(theme, " ") ->
        Logger.warning("""
        Helix themes are deprecated, use Neovim theme names instead.

        See `Lumis.available_themes/0` for a list of available themes.
        """)

        theme
        |> String.downcase()
        |> String.replace(" ", "")
        |> then(&{:string, &1})

      is_binary(theme) ->
        theme
        |> String.downcase()
        |> then(&{:string, &1})

      :else ->
        nil
    end
  end

  defp convert_formatter_for_nif(:html_inline, opts) do
    opts = convert_theme_for_nif(opts)

    {:html_inline,
     Map.take(opts, [
       :theme,
       :pre_class,
       :italic,
       :include_highlights,
       :rainbow_brackets,
       :highlight_lines,
       :header
     ])}
  end

  defp convert_formatter_for_nif(:html_linked, opts) do
    {:html_linked, Map.take(opts, [:pre_class, :rainbow_brackets, :highlight_lines, :header])}
  end

  defp convert_formatter_for_nif(:terminal, opts) do
    opts = convert_theme_for_nif(opts)

    opts =
      case opts[:background] do
        :theme -> Map.put(opts, :background, :theme)
        color when is_binary(color) -> Map.put(opts, :background, {:string, color})
        nil -> Map.put(opts, :background, nil)
      end

    {:terminal, Map.take(opts, [:theme, :background, :width, :rainbow_brackets])}
  end

  defp convert_formatter_for_nif(:bbcode_scoped, opts) do
    {:bbcode_scoped, Map.take(opts, [:rainbow_brackets])}
  end

  defp convert_formatter_for_nif(:html_multi_themes, opts) do
    {:html_multi_themes,
     Map.take(opts, [
       :themes,
       :default_theme,
       :css_variable_prefix,
       :pre_class,
       :italic,
       :include_highlights,
       :rainbow_brackets,
       :highlight_lines,
       :header
     ])}
  end

  defp convert_theme_for_nif(opts) do
    case opts[:theme] do
      {:theme, %Theme{} = theme} ->
        Map.put(opts, :theme, {:theme, theme})

      {:string, theme_name} when is_binary(theme_name) ->
        Map.put(opts, :theme, {:string, theme_name})

      nil ->
        Map.put(opts, :theme, nil)

      theme_name when is_binary(theme_name) ->
        Map.put(opts, :theme, {:string, theme_name})
    end
  end

  @doc """
  Same as `highlight/2` but raises `Lumis.HighlightError` in case of failure.
  """
  @spec highlight!(String.t(), keyword()) :: String.t()
  def highlight!(source, options \\ [])

  def highlight!(source, options) when is_binary(source) and is_list(options) do
    case highlight(source, options) do
      {:ok, highlighted} -> highlighted
      {:error, error} -> raise Lumis.HighlightError, error: error
    end
  end

  def highlight!(language, source)
      when is_binary(language) and is_binary(source) do
    highlight!(source, language: language)
  end

  @doc """
  Returns the directory parsers are downloaded to and compiled in.

  `Lumis.Application` resolves this at boot from `config :lumis, :data_dir`,
  `LUMIS_DATA_DIR`, or the application's `priv`. A library that embeds Lumis
  reads it here rather than repeating the resolution, so both end up on the
  same store and a parser is fetched and compiled once.

  ## Examples

      Lumis.data_dir()
      #=> "/home/user/.local/share/lumis"

  """
  @spec data_dir() :: String.t()
  def data_dir, do: Lumis.Native.data_dir()
end

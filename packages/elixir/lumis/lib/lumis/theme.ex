defmodule Lumis.Theme do
  @moduledoc """
  A Neovim theme.

  Contains the name, appearance, and a map of highlight `Lumis.Theme.Style`'s.

  Lumis bundles the most popular themes from the Neovim community,
  you can see the full list with `Lumis.available_themes/0` and
  then fetch one of the bundled themes with `Lumis.Theme.get/1`.

  Or check out all the [available themes](https://docs.rs/lumis/latest/lumis/#themes-available).

  ## Example

      %Lumis.Theme{
         name: "github_light",
         appearance: :light,
         revision: "fe70a27afefa6e10db4a59262d31f259f702fd6a",
         highlights: %{
           "function.macro" => %Lumis.Theme.Style{
             fg: "#6639ba",
             bg: nil,
             bold: false,
             italic: false,
             text_decoration: %Lumis.Theme.TextDecoration{
               underline: :solid,
               strikethrough: false
             }
           },
           ...
         }
      }

  """

  @type appearance :: :light | :dark

  @css_options_schema [
    layout: [
      type: :boolean,
      default: true,
      doc:
        "Include the built-in line layout rules. Set to `false` when your stylesheet supplies its own layout."
    ],
    enable_italic: [
      type: :boolean,
      default: true,
      doc: "Whether italic theme styles should be emitted."
    ],
    scope: [
      type: :string,
      default: "",
      doc: "Parent selector prepended to every generated selector."
    ],
    container_selector: [
      type: :string,
      default: ".lumis",
      doc: "Selector used for the container code block rule. Defaults to `.lumis`."
    ],
    container_style: [
      type: {:list, {:tuple, [:string, :string]}},
      default: [],
      doc:
        "Extra `{property, value}` declarations for the container code block rule. A property that matches one that the theme already sets (`color`, `background-color`) replaces that value."
    ]
  ]

  @typedoc "A Neovim theme with name, appearance (:light or :dark), revision, and highlight styles."
  @type t :: %Lumis.Theme{
          name: String.t(),
          appearance: appearance(),
          revision: String.t(),
          highlights: %{String.t() => Lumis.Theme.Style.t()}
        }

  defstruct name: nil, appearance: nil, revision: nil, highlights: %{}

  @doc """
  Builds CSS for a Lumis theme.

  Accepts a bundled theme name or a `Lumis.Theme` struct. Use this with the
  `:html_linked` formatter when you need to embed CSS, scope selectors, or
  customize the container code block rule.

  Pass `layout: false` to generate theme styles without the built-in line layout
  layer. Prebuilt CSS files always include layout.

  ## Options

  #{NimbleOptions.docs(@css_options_schema)}

  ## Examples

      Lumis.Theme.build_css!("github_dark")

  Produces CSS shaped like this:

      /* github_dark
       * revision: ...
       */
      .lumis {
        color: #e6edf3;
        background-color: #0d1117;
      }
      .l-keyword {
        color: #ff7b72;
      }

  Scope the generated stylesheet under a parent selector:

      Lumis.Theme.build_css!("github_dark",
        scope: ~s(html[data-theme="dark"]),
        container_style: [
          {"background-color", "var(--code-background)"},
          {"border-radius", "0.375rem"}
        ]
      )

  That produces selectors like:

      html[data-theme="dark"] .lumis {
        color: #e6edf3;
        background-color: var(--code-background);
        border-radius: 0.375rem;
      }
      html[data-theme="dark"] .l-keyword {
        color: #ff7b72;
      }

  """
  @spec build_css(String.t() | t(), keyword()) :: {:ok, String.t()} | {:error, term()}
  def build_css(theme, options \\ []) when is_list(options) do
    with {:ok, options} <- build_css_options(options) do
      case do_build_css(theme, options) do
        :error -> {:error, :not_found}
        css when is_binary(css) -> {:ok, css}
      end
    end
  end

  @doc """
  Builds CSS for a Lumis theme, raising on errors.

  See `build_css/2` for options.
  """
  @spec build_css!(String.t() | t(), keyword()) :: String.t()
  def build_css!(theme, options \\ []) do
    case build_css(theme, options) do
      {:ok, css} -> css
      {:error, reason} -> raise ArgumentError, "could not build theme CSS: #{inspect(reason)}"
    end
  end

  @doc """
  Get a theme by name.
  """
  @spec get(String.t(), any()) :: Lumis.Theme.t() | nil
  def get(name, default \\ nil) when is_binary(name) do
    case Lumis.Native.get_theme(name) do
      :error -> default
      theme -> theme
    end
  end

  defp do_build_css(name, options) when is_binary(name) do
    Lumis.Native.theme_css_from_name(name, options)
  end

  defp do_build_css(%Lumis.Theme{} = theme, options) do
    Lumis.Native.theme_css_from_theme(theme, options)
  end

  defp build_css_options(options) do
    options
    |> NimbleOptions.validate(@css_options_schema)
    |> case do
      {:ok, options} -> {:ok, Map.new(options)}
      {:error, error} -> {:error, error}
    end
  end

  @doc """
  Load a theme from a JSON file.
  """
  @spec from_file(String.t()) :: {:ok, Lumis.Theme.t()} | {:error, term()}
  def from_file(path) when is_binary(path) do
    case Lumis.Native.build_theme_from_file(path) do
      :error -> {:error, :invalid_theme_file}
      theme -> {:ok, theme}
    end
  end

  @doc """
  Load a theme from a JSON string.
  """
  @spec from_json(String.t()) :: {:ok, Lumis.Theme.t()} | {:error, term()}
  def from_json(json_string) when is_binary(json_string) do
    case Lumis.Native.build_theme_from_json_string(json_string) do
      :error -> {:error, :invalid_json}
      theme -> {:ok, theme}
    end
  end
end

defmodule Lumis.Theme.TextDecoration do
  @moduledoc """
  Text decoration settings for a highlight style.

  Contains the underline style and strikethrough flag.
  """

  @typedoc """
  Text decoration with underline style and strikethrough.

  The underline style can be one of:
  - `nil` - no underline
  - `:solid` - solid underline
  - `:wavy` - wavy/curly underline (undercurl)
  - `:double` - double underline
  - `:dotted` - dotted underline
  - `:dashed` - dashed underline
  """
  @type t :: %Lumis.Theme.TextDecoration{
          underline: nil | :solid | :wavy | :double | :dotted | :dashed,
          strikethrough: boolean()
        }

  defstruct underline: nil, strikethrough: false
end

defmodule Lumis.Theme.Style do
  @moduledoc """
  A highlight style.

  Contains the colors and styles of each highlight of a theme.
  """

  @typedoc "A highlight style with foreground/background colors and text decorations."
  @type t :: %Lumis.Theme.Style{
          fg: nil | String.t(),
          bg: nil | String.t(),
          bold: boolean(),
          italic: boolean(),
          text_decoration: Lumis.Theme.TextDecoration.t()
        }

  defstruct fg: nil,
            bg: nil,
            bold: false,
            italic: false,
            text_decoration: %Lumis.Theme.TextDecoration{}
end

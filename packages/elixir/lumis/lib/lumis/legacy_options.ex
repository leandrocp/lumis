defmodule Lumis.LegacyOptions do
  @moduledoc false

  # The options in the wire format `Lumis.rust_options!/1` returned through
  # Lumis 0.10, for callers that hand them to a NIF of their own: MDEx 0.14.1
  # and earlier, mdex_native 0.2.10 and earlier, and projects that copied
  # MDEx's `Lumis.validate_options!/1 |> Lumis.rust_options!/1`. Lumis itself
  # no longer reads this format; its NIF decodes options as written, with
  # `lumis_core::elixir`. Delete it once those versions are out of use.
  #
  # The body is Lumis 0.10's, with one step added up front: `validate_options!/1`
  # used to convert `highlight_lines`, `header` and `themes` while validating,
  # and no longer does, so `convert_options!/2` does it here.

  require Logger

  alias Lumis.Theme

  # What `Lumis.formatter_type/1` converted while validating, through 0.10.
  defp convert_options!(formatter, opts) do
    result =
      case formatter do
        :html_inline -> convert_html_inline_options(opts)
        :html_linked -> convert_html_linked_options(opts)
        :html_multi_themes -> convert_html_multi_themes_options(opts)
        :terminal -> {:ok, convert_terminal_options(opts)}
        :bbcode_scoped -> {:ok, convert_bbcode_options(opts)}
      end

    case result do
      {:ok, opts} -> opts
      {:error, error} -> raise ArgumentError, "invalid options given to #{formatter}: #{error}"
    end
  end

  defp convert_html_inline_options(opts) do
    with {:ok, opts} <- convert_highlight_lines_inline(opts) do
      convert_header(opts)
    end
  end

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

  defp convert_highlight_lines_inline(opts) do
    case opts[:highlight_lines] do
      nil ->
        {:ok, opts}

      hl ->
        put_inline_highlight_lines(opts, hl)
    end
  end

  defp put_inline_highlight_lines(opts, hl) do
    with {:ok, lines} <- Lumis.LineSpec.encode(hl[:lines] || []) do
      opts
      |> Keyword.put(:highlight_lines, %Lumis.HTMLInlineHighlightLines{
        lines: lines,
        style: inline_highlight_style(hl[:style]),
        class: hl[:class]
      })
      |> then(&{:ok, &1})
    end
  end

  defp inline_highlight_style(:theme), do: :theme
  defp inline_highlight_style(style) when is_binary(style), do: {:style, %{style: style}}
  defp inline_highlight_style(nil), do: nil
  defp inline_highlight_style(_other), do: :theme

  defp convert_highlight_lines_linked(opts) do
    case opts[:highlight_lines] do
      nil ->
        {:ok, opts}

      hl ->
        with {:ok, lines} <- Lumis.LineSpec.encode(hl[:lines] || []) do
          class = hl[:class] || "l-highlighted"

          opts
          |> Keyword.put(:highlight_lines, %Lumis.HTMLLinkedHighlightLines{
            lines: lines,
            class: class
          })
          |> then(&{:ok, &1})
        end
    end
  end

  # A line spec always encodes, so unlike the HTML converters these two cannot
  # fail and hand back the options rather than a result tuple.
  defp convert_terminal_options(opts) do
    case opts[:highlight_lines] do
      nil ->
        opts

      hl ->
        Keyword.put(opts, :highlight_lines, %Lumis.TerminalHighlightLines{
          lines: Lumis.LineSpec.encode!(hl[:lines] || []),
          background: hl[:background]
        })
    end
  end

  defp convert_bbcode_options(opts) do
    case opts[:highlight_lines] do
      nil ->
        opts

      hl ->
        Keyword.put(opts, :highlight_lines, %Lumis.BBCodeHighlightLines{
          lines: Lumis.LineSpec.encode!(hl[:lines] || [])
        })
    end
  end

  defp convert_header(opts) do
    case opts[:header] do
      nil ->
        {:ok, opts}

      %{open_tag: open_tag, close_tag: close_tag} ->
        opts
        |> Keyword.put(:header, %Lumis.HTMLElement{
          open_tag: open_tag,
          close_tag: close_tag
        })
        |> then(&{:ok, &1})

      _ ->
        {:error,
         "invalid value for :header option, must be a map with :open_tag and :close_tag keys"}
    end
  end

  def rust_options!(options) do
    {formatter, formatter_opts} = options[:formatter]
    formatter_opts = convert_options!(formatter, formatter_opts)
    {language, formatter_opts} = Keyword.pop(formatter_opts, :language)

    options =
      options
      |> Keyword.delete(:language)

    {theme, options} = Keyword.pop(options, :theme)
    theme = build_theme(theme || Keyword.get(formatter_opts, :theme))

    {pre_class, options} = Keyword.pop(options, :pre_class)
    pre_class = pre_class || Keyword.get(formatter_opts, :pre_class)

    {inline_style, options} = Keyword.pop(options, :inline_style)

    {formatter, formatter_opts} =
      case inline_style do
        true ->
          {:ok, {_type, default_opts}} = Lumis.formatter_type(:html_inline)
          {:html_inline, Keyword.merge(default_opts, formatter_opts)}

        false ->
          {:ok, {_type, default_opts}} = Lumis.formatter_type(:html_linked)
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
    |> Keyword.update(:budget, budget_for_nif([]), &budget_for_nif/1)
    |> Map.new()
  end

  # The NIF decodes a map, so the nested keyword list crosses as one.
  defp budget_for_nif(budget) do
    %{
      time_limit: Keyword.get(budget, :time_limit),
      match_limit: Keyword.get(budget, :match_limit)
    }
  end

  defp build_theme(theme) do
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
    opts = opts |> convert_theme_for_nif() |> convert_attrs_for_nif()

    {:html_inline,
     Map.take(opts, [
       :structure,
       :theme,
       :pre_class,
       :pre_attrs,
       :code_attrs,
       :italic,
       :include_highlights,
       :highlight_lines,
       :line_numbers,
       :header
     ])}
  end

  defp convert_formatter_for_nif(:html_linked, opts) do
    opts = convert_attrs_for_nif(opts)

    {:html_linked,
     Map.take(opts, [
       :structure,
       :pre_class,
       :pre_attrs,
       :code_attrs,
       :highlight_lines,
       :line_numbers,
       :header
     ])}
  end

  defp convert_formatter_for_nif(:terminal, opts) do
    opts = convert_theme_for_nif(opts)

    opts =
      case opts[:background] do
        :theme -> Map.put(opts, :background, :theme)
        color when is_binary(color) -> Map.put(opts, :background, {:string, color})
        nil -> Map.put(opts, :background, nil)
      end

    {:terminal, Map.take(opts, [:theme, :background, :width, :highlight_lines, :line_numbers])}
  end

  defp convert_formatter_for_nif(:bbcode_scoped, opts) do
    {:bbcode_scoped, Map.take(opts, [:highlight_lines])}
  end

  defp convert_formatter_for_nif(:html_multi_themes, opts) do
    opts = convert_attrs_for_nif(opts)

    {:html_multi_themes,
     Map.take(opts, [
       :structure,
       :themes,
       :default_theme,
       :css_variable_prefix,
       :pre_class,
       :pre_attrs,
       :code_attrs,
       :italic,
       :include_highlights,
       :highlight_lines,
       :line_numbers,
       :header
     ])}
  end

  defp convert_attrs_for_nif(opts) do
    opts
    |> Map.update(:pre_attrs, [], &encode_html_attrs/1)
    |> Map.update(:code_attrs, [], &encode_html_attrs/1)
  end

  defp encode_html_attrs(attrs) do
    Enum.map(attrs, fn {name, value} -> {Atom.to_string(name), value} end)
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
end

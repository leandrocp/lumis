//! Utility module to integrate with Elixir through Rustler.

use crate::formatter::{
    html_inline, html_linked, Formatter, HtmlElement, HtmlInlineBuilder, HtmlLinkedBuilder,
    HtmlMultiThemesBuilder, TerminalBuilder,
};
use crate::{languages::Language, themes};
use rustler::{NifStruct, NifTaggedEnum, NifUnitEnum};
use std::collections::HashMap;

/// Theme appearance enum that maps to Elixir atoms :light and :dark.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, NifUnitEnum)]
pub enum ExAppearance {
    Light,
    #[default]
    Dark,
}

#[derive(Debug, NifTaggedEnum)]
pub enum ExFormatterOption {
    HtmlInline {
        theme: Option<ThemeOrString>,
        pre_class: Option<String>,
        italic: bool,
        include_highlights: bool,
        highlight_lines: Option<ExHtmlInlineHighlightLines>,
        header: Option<ExHtmlElement>,
    },
    HtmlLinked {
        pre_class: Option<String>,
        highlight_lines: Option<ExHtmlLinkedHighlightLines>,
        header: Option<ExHtmlElement>,
    },
    HtmlMultiThemes {
        themes: HashMap<String, ExTheme>,
        default_theme: Option<String>,
        css_variable_prefix: Option<String>,
        pre_class: Option<String>,
        italic: bool,
        include_highlights: bool,
        highlight_lines: Option<ExHtmlInlineHighlightLines>,
        header: Option<ExHtmlElement>,
    },
    Terminal {
        theme: Option<ThemeOrString>,
    },
}

impl Default for ExFormatterOption {
    fn default() -> Self {
        Self::HtmlInline {
            theme: None,
            pre_class: None,
            italic: false,
            include_highlights: false,
            highlight_lines: None,
            header: None,
        }
    }
}

#[derive(Debug, NifTaggedEnum)]
pub enum ThemeOrString {
    Theme(ExTheme),
    String(String),
}

impl Default for ThemeOrString {
    fn default() -> Self {
        Self::String("onedark".to_string())
    }
}

/// Resolves a theme from ThemeOrString, returning None if the theme doesn't exist.
fn resolve_theme(theme_or_string: ThemeOrString) -> Option<themes::Theme> {
    match theme_or_string {
        ThemeOrString::Theme(theme) => Some(theme.into()),
        ThemeOrString::String(name) => themes::get(&name).ok(),
    }
}

/// Converts ExLineSpec to RangeInclusive without intermediate allocation.
#[inline]
fn convert_line_specs(lines: Vec<ExLineSpec>) -> Vec<std::ops::RangeInclusive<usize>> {
    lines
        .into_iter()
        .map(|line_spec| line_spec.to_range_inclusive())
        .collect()
}

/// Converts ExHtmlInlineHighlightLinesStyle to html_inline::HighlightLinesStyle.
#[inline]
fn convert_inline_style(
    style: ExHtmlInlineHighlightLinesStyle,
) -> html_inline::HighlightLinesStyle {
    match style {
        ExHtmlInlineHighlightLinesStyle::Theme => html_inline::HighlightLinesStyle::Theme,
        ExHtmlInlineHighlightLinesStyle::Style { style } => {
            html_inline::HighlightLinesStyle::Style(style)
        }
    }
}

impl ExFormatterOption {
    /// Convert ExFormatterOption to a boxed Formatter trait object.
    ///
    /// # Errors
    ///
    /// Returns an error if the formatter builder fails (should not happen with valid input data).
    pub fn into_formatter(self, language: Language) -> Result<Box<dyn Formatter>, String> {
        match self {
            ExFormatterOption::HtmlInline {
                theme,
                pre_class,
                italic,
                include_highlights,
                highlight_lines,
                header,
            } => {
                let theme = theme.and_then(resolve_theme);

                let highlight_lines = highlight_lines.map(|hl| html_inline::HighlightLines {
                    lines: convert_line_specs(hl.lines),
                    style: hl.style.map(convert_inline_style),
                    class: hl.class,
                });

                let header = header.map(|h| HtmlElement {
                    open_tag: h.open_tag,
                    close_tag: h.close_tag,
                });

                let formatter = HtmlInlineBuilder::new()
                    .lang(language)
                    .theme(theme)
                    .pre_class(pre_class)
                    .italic(italic)
                    .include_highlights(include_highlights)
                    .highlight_lines(highlight_lines)
                    .header(header)
                    .build()
                    .map_err(|e| format!("HtmlInline builder error: {:?}", e))?;

                Ok(Box::new(formatter))
            }
            ExFormatterOption::HtmlLinked {
                pre_class,
                highlight_lines,
                header,
            } => {
                let highlight_lines = highlight_lines.map(|hl| html_linked::HighlightLines {
                    lines: convert_line_specs(hl.lines),
                    class: hl.class,
                });

                let header = header.map(|h| HtmlElement {
                    open_tag: h.open_tag,
                    close_tag: h.close_tag,
                });

                let formatter = HtmlLinkedBuilder::new()
                    .lang(language)
                    .pre_class(pre_class)
                    .highlight_lines(highlight_lines)
                    .header(header)
                    .build()
                    .map_err(|e| format!("HtmlLinked builder error: {:?}", e))?;

                Ok(Box::new(formatter))
            }
            ExFormatterOption::HtmlMultiThemes {
                themes,
                default_theme,
                css_variable_prefix,
                pre_class,
                italic,
                include_highlights,
                highlight_lines,
                header,
            } => {
                let themes_map: HashMap<String, themes::Theme> =
                    themes.into_iter().map(|(k, v)| (k, v.into())).collect();

                let highlight_lines = highlight_lines.map(|hl| html_inline::HighlightLines {
                    lines: convert_line_specs(hl.lines),
                    style: hl.style.map(convert_inline_style),
                    class: hl.class,
                });

                let header = header.map(|h| HtmlElement {
                    open_tag: h.open_tag,
                    close_tag: h.close_tag,
                });

                let mut builder = HtmlMultiThemesBuilder::new();
                builder
                    .lang(language)
                    .themes(themes_map)
                    .css_variable_prefix(css_variable_prefix.as_deref().unwrap_or("--athl"))
                    .pre_class(pre_class)
                    .italic(italic)
                    .include_highlights(include_highlights)
                    .highlight_lines(highlight_lines)
                    .header(header);

                if let Some(dt_str) = default_theme {
                    builder.default_theme(dt_str);
                }

                let formatter = builder
                    .build()
                    .map_err(|e| format!("HtmlMultiThemes builder error: {:?}", e))?;

                Ok(Box::new(formatter))
            }
            ExFormatterOption::Terminal { theme } => {
                let theme = theme.and_then(resolve_theme);

                let formatter = TerminalBuilder::new()
                    .lang(language)
                    .theme(theme)
                    .build()
                    .map_err(|e| format!("Terminal builder error: {:?}", e))?;

                Ok(Box::new(formatter))
            }
        }
    }
}

#[derive(Clone, Debug, Default, NifStruct)]
#[module = "Autumn.Theme"]
pub struct ExTheme {
    pub name: String,
    pub appearance: ExAppearance,
    pub revision: String,
    pub highlights: HashMap<String, ExStyle>,
}

impl From<ExTheme> for themes::Theme {
    fn from(theme: ExTheme) -> Self {
        let appearance = match theme.appearance {
            ExAppearance::Light => themes::Appearance::Light,
            ExAppearance::Dark => themes::Appearance::Dark,
        };
        themes::Theme {
            name: theme.name,
            appearance,
            revision: theme.revision,
            highlights: theme
                .highlights
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        themes::Style {
                            fg: v.fg,
                            bg: v.bg,
                            bold: v.bold,
                            italic: v.italic,
                            text_decoration: v.text_decoration.into(),
                        },
                    )
                })
                .collect(),
        }
    }
}

impl<'a> From<&'a themes::Theme> for ExTheme {
    fn from(theme: &'a themes::Theme) -> Self {
        // Color deduplication: intern color strings to reduce allocations
        // Many styles share the same colors (e.g., "#ff0000" appears in multiple scopes)
        // Pre-allocate for typical themes (10-25 unique colors)
        let mut color_cache: HashMap<&str, String> = HashMap::with_capacity(32);

        let highlights = theme
            .highlights
            .iter()
            .map(|(k, v)| {
                let fg = v.fg.as_ref().map(|color_str| {
                    color_cache
                        .entry(color_str.as_str())
                        .or_insert_with(|| color_str.to_string())
                        .clone()
                });

                let bg = v.bg.as_ref().map(|color_str| {
                    color_cache
                        .entry(color_str.as_str())
                        .or_insert_with(|| color_str.to_string())
                        .clone()
                });

                (
                    k.to_owned(),
                    ExStyle {
                        fg,
                        bg,
                        bold: v.bold,
                        italic: v.italic,
                        text_decoration: v.text_decoration.into(),
                    },
                )
            })
            .collect();

        let appearance = match theme.appearance {
            themes::Appearance::Light => ExAppearance::Light,
            themes::Appearance::Dark => ExAppearance::Dark,
        };

        ExTheme {
            name: theme.name.to_owned(),
            appearance,
            revision: theme.revision.to_owned(),
            highlights,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, NifUnitEnum)]
pub enum ExUnderlineStyle {
    Solid,
    Wavy,
    Double,
    Dotted,
    Dashed,
}

impl From<themes::UnderlineStyle> for Option<ExUnderlineStyle> {
    fn from(style: themes::UnderlineStyle) -> Self {
        match style {
            themes::UnderlineStyle::None => None,
            themes::UnderlineStyle::Solid => Some(ExUnderlineStyle::Solid),
            themes::UnderlineStyle::Wavy => Some(ExUnderlineStyle::Wavy),
            themes::UnderlineStyle::Double => Some(ExUnderlineStyle::Double),
            themes::UnderlineStyle::Dotted => Some(ExUnderlineStyle::Dotted),
            themes::UnderlineStyle::Dashed => Some(ExUnderlineStyle::Dashed),
        }
    }
}

impl From<Option<ExUnderlineStyle>> for themes::UnderlineStyle {
    fn from(style: Option<ExUnderlineStyle>) -> Self {
        match style {
            None => themes::UnderlineStyle::None,
            Some(ExUnderlineStyle::Solid) => themes::UnderlineStyle::Solid,
            Some(ExUnderlineStyle::Wavy) => themes::UnderlineStyle::Wavy,
            Some(ExUnderlineStyle::Double) => themes::UnderlineStyle::Double,
            Some(ExUnderlineStyle::Dotted) => themes::UnderlineStyle::Dotted,
            Some(ExUnderlineStyle::Dashed) => themes::UnderlineStyle::Dashed,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, NifStruct)]
#[module = "Autumn.Theme.TextDecoration"]
pub struct ExTextDecoration {
    pub underline: Option<ExUnderlineStyle>,
    pub strikethrough: bool,
}

impl From<themes::TextDecoration> for ExTextDecoration {
    fn from(td: themes::TextDecoration) -> Self {
        ExTextDecoration {
            underline: td.underline.into(),
            strikethrough: td.strikethrough,
        }
    }
}

impl From<ExTextDecoration> for themes::TextDecoration {
    fn from(td: ExTextDecoration) -> Self {
        themes::TextDecoration {
            underline: td.underline.into(),
            strikethrough: td.strikethrough,
        }
    }
}

#[derive(Clone, Debug, Default, NifStruct)]
#[module = "Autumn.Theme.Style"]
pub struct ExStyle {
    pub fg: Option<String>,
    pub bg: Option<String>,
    pub bold: bool,
    pub italic: bool,
    pub text_decoration: ExTextDecoration,
}

#[derive(Clone, Debug, Default, NifStruct)]
#[module = "Autumn.HtmlElement"]
pub struct ExHtmlElement {
    pub open_tag: String,
    pub close_tag: String,
}

#[derive(Clone, Debug, NifTaggedEnum)]
pub enum ExLineSpec {
    Single(usize),
    Range { start: usize, end: usize },
}

impl ExLineSpec {
    fn to_range_inclusive(&self) -> std::ops::RangeInclusive<usize> {
        match self {
            ExLineSpec::Single(line) => *line..=*line,
            ExLineSpec::Range { start, end } => *start..=*end,
        }
    }

    fn from_range_inclusive(range: std::ops::RangeInclusive<usize>) -> Self {
        let start = *range.start();
        let end = *range.end();
        if start == end {
            ExLineSpec::Single(start)
        } else {
            ExLineSpec::Range { start, end }
        }
    }
}

#[derive(Clone, Debug, Default, NifTaggedEnum)]
pub enum ExHtmlInlineHighlightLinesStyle {
    #[default]
    Theme,
    Style {
        style: String,
    },
}

#[derive(Clone, Debug, Default, NifStruct)]
#[module = "Autumn.HtmlInlineHighlightLines"]
pub struct ExHtmlInlineHighlightLines {
    pub lines: Vec<ExLineSpec>,
    pub style: Option<ExHtmlInlineHighlightLinesStyle>,
    pub class: Option<String>,
}

#[derive(Clone, Debug, Default, NifStruct)]
#[module = "Autumn.HtmlLinkedHighlightLines"]
pub struct ExHtmlLinkedHighlightLines {
    pub lines: Vec<ExLineSpec>,
    pub class: String,
}

impl<'a> From<&'a themes::Style> for ExStyle {
    fn from(style: &'a themes::Style) -> Self {
        ExStyle {
            fg: style.fg.clone(),
            bg: style.bg.clone(),
            bold: style.bold,
            italic: style.italic,
            text_decoration: style.text_decoration.into(),
        }
    }
}

impl From<HtmlElement> for ExHtmlElement {
    fn from(element: HtmlElement) -> Self {
        ExHtmlElement {
            open_tag: element.open_tag,
            close_tag: element.close_tag,
        }
    }
}

impl From<ExHtmlElement> for HtmlElement {
    fn from(element: ExHtmlElement) -> Self {
        HtmlElement {
            open_tag: element.open_tag,
            close_tag: element.close_tag,
        }
    }
}

impl From<html_inline::HighlightLinesStyle> for ExHtmlInlineHighlightLinesStyle {
    fn from(style: html_inline::HighlightLinesStyle) -> Self {
        match style {
            html_inline::HighlightLinesStyle::Theme => ExHtmlInlineHighlightLinesStyle::Theme,
            html_inline::HighlightLinesStyle::Style(s) => {
                ExHtmlInlineHighlightLinesStyle::Style { style: s }
            }
        }
    }
}

impl From<html_inline::HighlightLines> for ExHtmlInlineHighlightLines {
    fn from(highlight_lines: html_inline::HighlightLines) -> Self {
        ExHtmlInlineHighlightLines {
            lines: highlight_lines
                .lines
                .into_iter()
                .map(ExLineSpec::from_range_inclusive)
                .collect(),
            style: highlight_lines.style.map(|s| s.into()),
            class: highlight_lines.class,
        }
    }
}

impl From<html_linked::HighlightLines> for ExHtmlLinkedHighlightLines {
    fn from(highlight_lines: html_linked::HighlightLines) -> Self {
        ExHtmlLinkedHighlightLines {
            lines: highlight_lines
                .lines
                .into_iter()
                .map(ExLineSpec::from_range_inclusive)
                .collect(),
            class: highlight_lines.class,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{highlight, themes};
    use std::collections::HashMap;

    #[cfg(test)]
    use pretty_assertions::assert_str_eq;

    #[test]
    fn test_ex_formatter_option_default() {
        let default_option = ExFormatterOption::default();

        match default_option {
            ExFormatterOption::HtmlInline {
                theme,
                pre_class,
                italic,
                include_highlights,
                highlight_lines,
                header,
            } => {
                assert!(theme.is_none());
                assert!(pre_class.is_none());
                assert!(!italic);
                assert!(!include_highlights);
                assert!(highlight_lines.is_none());
                assert!(header.is_none());
            }
            _ => panic!("Default should be HtmlInline variant"),
        }
    }

    #[test]
    fn test_theme_or_string_conversion() {
        let theme_str = ThemeOrString::String("dracula".to_string());
        match theme_str {
            ThemeOrString::String(name) => assert_eq!(name, "dracula"),
            _ => panic!("Should be String variant"),
        }

        let ex_theme = ExTheme {
            name: "test_theme".to_string(),
            appearance: ExAppearance::Dark,
            revision: "1.0".to_string(),
            highlights: HashMap::new(),
        };
        let theme_obj = ThemeOrString::Theme(ex_theme);
        match theme_obj {
            ThemeOrString::Theme(theme) => {
                assert_eq!(theme.name, "test_theme");
                assert_eq!(theme.appearance, ExAppearance::Dark);
            }
            _ => panic!("Should be Theme variant"),
        }
    }

    #[test]
    fn test_ex_html_element_conversion() {
        let ex_element = ExHtmlElement {
            open_tag: "<div class=\"wrapper\">".to_string(),
            close_tag: "</div>".to_string(),
        };

        let rust_element: HtmlElement = ex_element.clone().into();
        assert_eq!(rust_element.open_tag, "<div class=\"wrapper\">");
        assert_eq!(rust_element.close_tag, "</div>");

        let back_to_ex: ExHtmlElement = rust_element.into();
        assert_eq!(back_to_ex.open_tag, ex_element.open_tag);
        assert_eq!(back_to_ex.close_tag, ex_element.close_tag);
    }

    #[test]
    fn test_ex_html_inline_highlight_lines_conversion() {
        let code = "line 1\nline 2\nline 3\nline 4\nline 5";
        let ex_highlight_lines = ExHtmlInlineHighlightLines {
            lines: vec![
                ExLineSpec::Single(1),
                ExLineSpec::Range { start: 3, end: 5 },
            ],
            style: Some(ExHtmlInlineHighlightLinesStyle::Style {
                style: "background-color: yellow".to_string(),
            }),
            class: None,
        };

        let formatter_option = ExFormatterOption::HtmlInline {
            theme: None,
            pre_class: None,
            italic: false,
            include_highlights: false,
            highlight_lines: Some(ex_highlight_lines),
            header: None,
        };

        let lang = Language::guess(Some("text"), code);
        let _formatter = formatter_option.into_formatter(lang);
    }

    #[test]
    fn test_ex_html_linked_highlight_lines_conversion() {
        let code = "line 1\nline 2\nline 3\nline 4";
        let ex_highlight_lines = ExHtmlLinkedHighlightLines {
            lines: vec![ExLineSpec::Range { start: 2, end: 4 }],
            class: "highlighted".to_string(),
        };

        let formatter_option = ExFormatterOption::HtmlLinked {
            pre_class: None,
            highlight_lines: Some(ex_highlight_lines),
            header: None,
        };

        let lang = Language::guess(Some("text"), code);
        let _formatter = formatter_option.into_formatter(lang);
    }

    #[test]
    fn test_ex_theme_conversion() {
        let mut highlights = HashMap::new();
        highlights.insert(
            "keyword".to_string(),
            ExStyle {
                fg: Some("#ff0000".to_string()),
                bg: None,
                bold: true,
                italic: false,
                text_decoration: ExTextDecoration::default(),
            },
        );

        let ex_theme = ExTheme {
            name: "test_theme".to_string(),
            appearance: ExAppearance::Dark,
            revision: "1.0".to_string(),
            highlights,
        };

        let rust_theme: themes::Theme = ex_theme.into();
        assert_eq!(rust_theme.name, "test_theme");
        assert_eq!(rust_theme.appearance, themes::Appearance::Dark);
        assert_eq!(rust_theme.revision, "1.0");

        let keyword_style = rust_theme.highlights.get("keyword").unwrap();
        assert_eq!(keyword_style.fg, Some("#ff0000".to_string()));
        assert!(keyword_style.bold);
        assert!(!keyword_style.italic);
    }

    #[test]
    fn test_html_inline_with_theme_string() {
        let code = "fn main() { println!(\"Hello\"); }";

        let ex_formatter = ExFormatterOption::HtmlInline {
            theme: Some(ThemeOrString::String("dracula".to_string())),
            pre_class: Some("code-block".to_string()),
            italic: false,
            include_highlights: false,
            highlight_lines: None,
            header: None,
        };

        let lang = Language::guess(Some("rust"), code);
        let formatter = ex_formatter.into_formatter(lang).unwrap();

        let result = highlight(code, formatter);
        let expected = r#"<pre class="athl code-block" style="color: #f8f8f2; background-color: #282a36;"><code class="language-rust" translate="no" tabindex="0"><div class="line" data-line="1"><span style="color: #8be9fd;">fn</span> <span style="color: #50fa7b;">main</span><span style="color: #f8f8f2;">(</span><span style="color: #f8f8f2;">)</span> <span style="color: #f8f8f2;">&lbrace;</span> <span style="color: #bd93f9;">println</span><span style="color: #50fa7b;">!</span><span style="color: #f8f8f2;">(</span><span style="color: #f1fa8c;">&quot;Hello&quot;</span><span style="color: #f8f8f2;">)</span><span style="color: #f8f8f2;">;</span> <span style="color: #f8f8f2;">&rbrace;</span>
</div></code></pre>"#;
        assert_str_eq!(result, expected);
    }

    #[test]
    fn test_html_inline_with_highlight_lines() {
        let code = "line 1\nline 2\nline 3\nline 4";

        let highlight_lines = ExHtmlInlineHighlightLines {
            lines: vec![
                ExLineSpec::Single(1),
                ExLineSpec::Range { start: 3, end: 4 },
            ],
            style: Some(ExHtmlInlineHighlightLinesStyle::Style {
                style: "background-color: yellow".to_string(),
            }),
            class: Some("custom-class".to_string()),
        };

        let ex_formatter = ExFormatterOption::HtmlInline {
            theme: Some(ThemeOrString::String("github_light".to_string())),
            pre_class: None,
            italic: false,
            include_highlights: false,
            highlight_lines: Some(highlight_lines),
            header: None,
        };

        let lang = Language::guess(Some("text"), code);
        let formatter = ex_formatter.into_formatter(lang).unwrap();

        let result = highlight(code, formatter);
        let expected = r#"<pre class="athl" style="color: #1f2328; background-color: #ffffff;"><code class="language-plaintext" translate="no" tabindex="0"><div class="line custom-class" style="background-color: yellow" data-line="1">line 1
</div><div class="line" data-line="2">line 2
</div><div class="line custom-class" style="background-color: yellow" data-line="3">line 3
</div><div class="line custom-class" style="background-color: yellow" data-line="4">line 4
</div></code></pre>"#;
        assert_str_eq!(result, expected);
    }

    #[test]
    fn test_html_inline_with_header() {
        let code = "const x = 42;";

        let header = ExHtmlElement {
            open_tag: "<section class=\"code-wrapper\">".to_string(),
            close_tag: "</section>".to_string(),
        };

        let ex_formatter = ExFormatterOption::HtmlInline {
            theme: None,
            pre_class: None,
            italic: false,
            include_highlights: false,
            highlight_lines: None,
            header: Some(header),
        };

        let lang = Language::guess(Some("javascript"), code);
        let formatter = ex_formatter.into_formatter(lang).unwrap();

        let result = highlight(code, formatter);
        let expected = r#"<section class="code-wrapper"><pre class="athl"><code class="language-javascript" translate="no" tabindex="0"><div class="line" data-line="1"><span >const</span> <span >x</span> <span >=</span> <span >42</span><span >;</span>
</div></code></pre></section>"#;
        assert_str_eq!(result, expected);
    }

    #[test]
    fn test_html_linked_with_all_features() {
        let code = "defmodule Test do\n  def hello, do: :world\nend";

        let highlight_lines = ExHtmlLinkedHighlightLines {
            lines: vec![ExLineSpec::Single(2)],
            class: "custom-hl".to_string(),
        };

        let header = ExHtmlElement {
            open_tag: "<div class=\"elixir-code\">".to_string(),
            close_tag: "</div>".to_string(),
        };

        let ex_formatter = ExFormatterOption::HtmlLinked {
            pre_class: Some("syntax-highlight".to_string()),
            highlight_lines: Some(highlight_lines),
            header: Some(header),
        };

        let lang = Language::guess(Some("elixir"), code);
        let formatter = ex_formatter.into_formatter(lang).unwrap();

        let result = highlight(code, formatter);
        let expected = r#"<div class="elixir-code"><pre class="athl syntax-highlight"><code class="language-elixir" translate="no" tabindex="0"><div class="line" data-line="1"><span class="keyword">defmodule</span> <span class="module">Test</span> <span class="keyword">do</span>
</div><div class="line custom-hl" data-line="2">  <span class="keyword">def</span> <span class="variable">hello</span><span class="punctuation-delimiter">,</span> <span class="string-special-symbol">do: </span><span class="string-special-symbol">:world</span>
</div><div class="line" data-line="3"><span class="keyword">end</span>
</div></code></pre></div>"#;
        assert_str_eq!(result, expected);
    }

    #[test]
    fn test_terminal_formatter_with_theme_string() {
        let code = "puts 'Hello Ruby'";

        let ex_formatter = ExFormatterOption::Terminal {
            theme: Some(ThemeOrString::String("github_dark".to_string())),
        };

        let lang = Language::guess(Some("ruby"), code);
        let formatter = ex_formatter.into_formatter(lang).unwrap();

        let result = highlight(code, formatter);
        let expected = "\x1b[0m\x1b[38;2;210;168;255mputs\x1b[0m \x1b[0m\x1b[38;2;165;214;255m'\x1b[0m\x1b[0m\x1b[38;2;165;214;255mHello Ruby\x1b[0m\x1b[0m\x1b[38;2;165;214;255m'\x1b[0m";
        assert_str_eq!(result, expected);
    }

    #[test]
    fn test_ex_html_inline_highlight_lines_style_theme() {
        let code = "line 1\nline 2\nline 3";
        let highlight_lines = ExHtmlInlineHighlightLines {
            lines: vec![ExLineSpec::Range { start: 1, end: 3 }],
            style: Some(ExHtmlInlineHighlightLinesStyle::Theme),
            class: None,
        };

        let ex_formatter = ExFormatterOption::HtmlInline {
            theme: Some(ThemeOrString::String("catppuccin_mocha".to_string())),
            pre_class: None,
            italic: false,
            include_highlights: false,
            highlight_lines: Some(highlight_lines),
            header: None,
        };

        let lang = Language::guess(Some("text"), code);
        let _formatter = ex_formatter.into_formatter(lang);
    }

    #[test]
    fn test_ex_line_spec_single_and_range() {
        let code = "line 1\nline 2\nline 3\nline 4\nline 5\nline 6\nline 7";
        let highlight_lines = ExHtmlInlineHighlightLines {
            lines: vec![
                ExLineSpec::Single(1),
                ExLineSpec::Range { start: 3, end: 5 },
                ExLineSpec::Single(7),
            ],
            style: Some(ExHtmlInlineHighlightLinesStyle::Theme),
            class: None,
        };

        let formatter_option = ExFormatterOption::HtmlInline {
            theme: Some(ThemeOrString::String("github_light".to_string())),
            pre_class: None,
            italic: false,
            include_highlights: false,
            highlight_lines: Some(highlight_lines),
            header: None,
        };

        let lang = Language::guess(Some("text"), code);
        let _formatter = formatter_option.into_formatter(lang);
    }

    #[test]
    fn test_error_handling_invalid_theme_name() {
        let code = "test code";
        let ex_formatter = ExFormatterOption::HtmlInline {
            theme: Some(ThemeOrString::String("nonexistent_theme".to_string())),
            pre_class: None,
            italic: false,
            include_highlights: false,
            highlight_lines: None,
            header: None,
        };

        let lang = Language::guess(Some("text"), code);
        let _formatter = ex_formatter.into_formatter(lang);
    }

    fn create_test_theme(name: &str, appearance: ExAppearance) -> ExTheme {
        let mut highlights = HashMap::new();
        highlights.insert(
            "keyword".to_string(),
            ExStyle {
                fg: Some("#ff0000".to_string()),
                bg: None,
                bold: true,
                italic: false,
                text_decoration: ExTextDecoration::default(),
            },
        );
        highlights.insert(
            "string".to_string(),
            ExStyle {
                fg: Some("#00ff00".to_string()),
                bg: None,
                bold: false,
                italic: false,
                text_decoration: ExTextDecoration::default(),
            },
        );
        highlights.insert(
            "normal".to_string(),
            ExStyle {
                fg: Some(
                    if appearance == ExAppearance::Light {
                        "#000000"
                    } else {
                        "#ffffff"
                    }
                    .to_string(),
                ),
                bg: Some(
                    if appearance == ExAppearance::Light {
                        "#ffffff"
                    } else {
                        "#000000"
                    }
                    .to_string(),
                ),
                bold: false,
                italic: false,
                text_decoration: ExTextDecoration::default(),
            },
        );

        ExTheme {
            name: name.to_string(),
            appearance,
            revision: "1.0".to_string(),
            highlights,
        }
    }

    #[test]
    fn test_html_multi_themes_basic() {
        let code = "fn main() {}";
        let mut themes = HashMap::new();
        themes.insert(
            "light".to_string(),
            create_test_theme("light", ExAppearance::Light),
        );
        themes.insert(
            "dark".to_string(),
            create_test_theme("dark", ExAppearance::Dark),
        );

        let formatter_option = ExFormatterOption::HtmlMultiThemes {
            themes,
            default_theme: Some("light".to_string()),
            css_variable_prefix: None,
            pre_class: None,
            italic: false,
            include_highlights: false,
            highlight_lines: None,
            header: None,
        };

        let lang = Language::guess(Some("rust"), code);
        let formatter = formatter_option
            .into_formatter(lang)
            .expect("Should create formatter");
        let result = highlight(code, formatter);

        assert!(result.contains("--athl-dark-"));
        assert!(result.contains("color:#"));
    }

    #[test]
    fn test_html_multi_themes_none() {
        let code = "fn main() {}";
        let mut themes = HashMap::new();
        themes.insert(
            "light".to_string(),
            create_test_theme("light", ExAppearance::Light),
        );
        themes.insert(
            "dark".to_string(),
            create_test_theme("dark", ExAppearance::Dark),
        );
        themes.insert(
            "high_contrast".to_string(),
            create_test_theme("high_contrast", ExAppearance::Dark),
        );

        let formatter_option = ExFormatterOption::HtmlMultiThemes {
            themes,
            default_theme: None,
            css_variable_prefix: None,
            pre_class: None,
            italic: false,
            include_highlights: false,
            highlight_lines: None,
            header: None,
        };

        let lang = Language::guess(Some("rust"), code);
        let formatter = formatter_option
            .into_formatter(lang)
            .expect("Should create formatter");
        let result = highlight(code, formatter);

        assert!(result.contains("--athl-light-"));
        assert!(result.contains("--athl-dark-"));
        assert!(result.contains("--athl-high_contrast-"));
        assert!(!result.contains("color:#"));
    }

    #[test]
    fn test_html_multi_themes_light_dark() {
        let code = "fn main() {}";
        let mut themes = HashMap::new();
        themes.insert(
            "light".to_string(),
            create_test_theme("light", ExAppearance::Light),
        );
        themes.insert(
            "dark".to_string(),
            create_test_theme("dark", ExAppearance::Dark),
        );

        let formatter_option = ExFormatterOption::HtmlMultiThemes {
            themes,
            default_theme: Some("light-dark()".to_string()),
            css_variable_prefix: None,
            pre_class: None,
            italic: false,
            include_highlights: false,
            highlight_lines: None,
            header: None,
        };

        let lang = Language::guess(Some("rust"), code);
        let formatter = formatter_option
            .into_formatter(lang)
            .expect("Should create formatter");
        let result = highlight(code, formatter);

        assert!(result.contains("light-dark("));
    }

    #[test]
    fn test_html_multi_themes_custom_prefix() {
        let code = "fn main() {}";
        let mut themes = HashMap::new();
        themes.insert(
            "light".to_string(),
            create_test_theme("light", ExAppearance::Light),
        );
        themes.insert(
            "dark".to_string(),
            create_test_theme("dark", ExAppearance::Dark),
        );

        let formatter_option = ExFormatterOption::HtmlMultiThemes {
            themes,
            default_theme: Some("light".to_string()),
            css_variable_prefix: Some("--custom".to_string()),
            pre_class: None,
            italic: false,
            include_highlights: false,
            highlight_lines: None,
            header: None,
        };

        let lang = Language::guess(Some("rust"), code);
        let formatter = formatter_option
            .into_formatter(lang)
            .expect("Should create formatter");
        let result = highlight(code, formatter);

        assert!(result.contains("--custom-dark-"));
        assert!(!result.contains("--athl-"));
    }

    #[test]
    fn test_html_multi_themes_with_highlight_lines() {
        let code = "line 1\nline 2\nline 3";
        let mut themes = HashMap::new();
        themes.insert(
            "light".to_string(),
            create_test_theme("light", ExAppearance::Light),
        );
        themes.insert(
            "dark".to_string(),
            create_test_theme("dark", ExAppearance::Dark),
        );

        let highlight_lines = ExHtmlInlineHighlightLines {
            lines: vec![ExLineSpec::Range { start: 1, end: 2 }],
            style: Some(ExHtmlInlineHighlightLinesStyle::Theme),
            class: None,
        };

        let formatter_option = ExFormatterOption::HtmlMultiThemes {
            themes,
            default_theme: Some("light".to_string()),
            css_variable_prefix: None,
            pre_class: None,
            italic: false,
            include_highlights: false,
            highlight_lines: Some(highlight_lines),
            header: None,
        };

        let lang = Language::guess(Some("text"), code);
        let formatter = formatter_option
            .into_formatter(lang)
            .expect("Should create formatter");
        let result = highlight(code, formatter);

        assert!(result.contains("<div"));
    }

    #[test]
    fn test_html_multi_themes_with_header() {
        let code = "fn main() {}";
        let mut themes = HashMap::new();
        themes.insert(
            "light".to_string(),
            create_test_theme("light", ExAppearance::Light),
        );
        themes.insert(
            "dark".to_string(),
            create_test_theme("dark", ExAppearance::Dark),
        );

        let header = ExHtmlElement {
            open_tag: "<div class=\"code-wrapper\">".to_string(),
            close_tag: "</div>".to_string(),
        };

        let formatter_option = ExFormatterOption::HtmlMultiThemes {
            themes,
            default_theme: Some("light".to_string()),
            css_variable_prefix: None,
            pre_class: None,
            italic: false,
            include_highlights: false,
            highlight_lines: None,
            header: Some(header),
        };

        let lang = Language::guess(Some("rust"), code);
        let formatter = formatter_option
            .into_formatter(lang)
            .expect("Should create formatter");
        let result = highlight(code, formatter);

        assert!(result.contains("<div class=\"code-wrapper\">"));
        assert!(result.contains("</div>"));
    }

    #[test]
    fn test_html_multi_themes_error_missing_default() {
        let code = "fn main() {}";
        let mut themes = HashMap::new();
        themes.insert(
            "light".to_string(),
            create_test_theme("light", ExAppearance::Light),
        );
        themes.insert(
            "dark".to_string(),
            create_test_theme("dark", ExAppearance::Dark),
        );

        let formatter_option = ExFormatterOption::HtmlMultiThemes {
            themes,
            default_theme: Some("nonexistent".to_string()),
            css_variable_prefix: None,
            pre_class: None,
            italic: false,
            include_highlights: false,
            highlight_lines: None,
            header: None,
        };

        let lang = Language::guess(Some("rust"), code);
        let result = formatter_option.into_formatter(lang);

        assert!(result.is_err());
        if let Err(err_msg) = result {
            assert!(err_msg.contains("Default theme 'nonexistent' not found"));
        }
    }

    #[test]
    fn test_html_multi_themes_error_light_dark_incomplete() {
        let code = "fn main() {}";
        let mut themes = HashMap::new();
        themes.insert(
            "light".to_string(),
            create_test_theme("light", ExAppearance::Light),
        );

        let formatter_option = ExFormatterOption::HtmlMultiThemes {
            themes,
            default_theme: Some("light-dark()".to_string()),
            css_variable_prefix: None,
            pre_class: None,
            italic: false,
            include_highlights: false,
            highlight_lines: None,
            header: None,
        };

        let lang = Language::guess(Some("rust"), code);
        let result = formatter_option.into_formatter(lang);

        assert!(result.is_err());
        if let Err(err_msg) = result {
            assert!(err_msg.contains("requires themes named 'light' and 'dark'"));
        }
    }

    #[test]
    fn test_html_multi_themes_three_themes() {
        let code = "fn main() {}";
        let mut themes = HashMap::new();
        themes.insert(
            "nord".to_string(),
            create_test_theme("nord", ExAppearance::Dark),
        );
        themes.insert(
            "gruvbox".to_string(),
            create_test_theme("gruvbox", ExAppearance::Dark),
        );
        themes.insert(
            "solarized".to_string(),
            create_test_theme("solarized", ExAppearance::Light),
        );

        let formatter_option = ExFormatterOption::HtmlMultiThemes {
            themes,
            default_theme: Some("nord".to_string()),
            css_variable_prefix: None,
            pre_class: None,
            italic: false,
            include_highlights: false,
            highlight_lines: None,
            header: None,
        };

        let lang = Language::guess(Some("rust"), code);
        let formatter = formatter_option
            .into_formatter(lang)
            .expect("Should create formatter");
        let result = highlight(code, formatter);

        assert!(result.contains("--athl-gruvbox-"));
        assert!(result.contains("--athl-solarized-"));
        assert!(result.contains("color:#"));
    }

    #[test]
    fn test_html_multi_themes_font_style_css_variables() {
        let code = "fn main() {}";
        let mut themes = HashMap::new();
        themes.insert(
            "light".to_string(),
            create_test_theme("light", ExAppearance::Light),
        );
        themes.insert(
            "dark".to_string(),
            create_test_theme("dark", ExAppearance::Dark),
        );

        let formatter_option = ExFormatterOption::HtmlMultiThemes {
            themes,
            default_theme: Some("light".to_string()),
            css_variable_prefix: None,
            pre_class: None,
            italic: true,
            include_highlights: false,
            highlight_lines: None,
            header: None,
        };

        let lang = Language::guess(Some("rust"), code);
        let formatter = formatter_option
            .into_formatter(lang)
            .expect("Should create formatter");
        let result = highlight(code, formatter);

        assert!(result.contains("--athl-light-font-style:"));
        assert!(result.contains("--athl-dark-font-style:"));
        assert!(result.contains("--athl-light-font-weight:"));
        assert!(result.contains("--athl-dark-font-weight:"));
        assert!(result.contains("--athl-light-text-decoration:"));
        assert!(result.contains("--athl-dark-text-decoration:"));
    }

    #[test]
    fn test_html_multi_themes_lightdark_font_decorations() {
        let code = "fn main() {}";
        let mut themes = HashMap::new();
        themes.insert(
            "light".to_string(),
            create_test_theme("light", ExAppearance::Light),
        );
        themes.insert(
            "dark".to_string(),
            create_test_theme("dark", ExAppearance::Dark),
        );

        let formatter_option = ExFormatterOption::HtmlMultiThemes {
            themes,
            default_theme: Some("light-dark()".to_string()),
            css_variable_prefix: None,
            pre_class: None,
            italic: true,
            include_highlights: false,
            highlight_lines: None,
            header: None,
        };

        let lang = Language::guess(Some("rust"), code);
        let formatter = formatter_option
            .into_formatter(lang)
            .expect("Should create formatter");
        let result = highlight(code, formatter);

        assert!(result.contains("font-weight: light-dark("));
        assert!(result.contains("font-style: light-dark("));
        assert!(result.contains("text-decoration: light-dark("));
    }

    #[test]
    fn test_html_multi_themes_none_mode_font_decorations() {
        let code = "fn main() {}";
        let mut themes = HashMap::new();
        themes.insert(
            "light".to_string(),
            create_test_theme("light", ExAppearance::Light),
        );
        themes.insert(
            "dark".to_string(),
            create_test_theme("dark", ExAppearance::Dark),
        );

        let formatter_option = ExFormatterOption::HtmlMultiThemes {
            themes,
            default_theme: None,
            css_variable_prefix: None,
            pre_class: None,
            italic: false,
            include_highlights: false,
            highlight_lines: None,
            header: None,
        };

        let lang = Language::guess(Some("rust"), code);
        let formatter = formatter_option
            .into_formatter(lang)
            .expect("Should create formatter");
        let result = highlight(code, formatter);

        assert!(result.contains("--athl-light-font-style:"));
        assert!(result.contains("--athl-dark-font-style:"));
        assert!(result.contains("--athl-light-font-weight:"));
        assert!(result.contains("--athl-dark-font-weight:"));
        assert!(result.contains("--athl-light-text-decoration:"));
        assert!(result.contains("--athl-dark-text-decoration:"));
        assert!(!result.contains("font-style:italic;"));
        assert!(!result.contains("font-weight:bold;"));
    }

    #[test]
    fn test_ex_style_font_decorations_conversion() {
        use themes::{TextDecoration, UnderlineStyle};

        let ex_style = ExStyle {
            fg: Some("#ff0000".to_string()),
            bg: Some("#ffffff".to_string()),
            bold: true,
            italic: true,
            text_decoration: ExTextDecoration {
                underline: Some(ExUnderlineStyle::Solid),
                strikethrough: true,
            },
        };

        let rust_style: themes::Style = themes::Style {
            fg: ex_style.fg.clone(),
            bg: ex_style.bg.clone(),
            bold: ex_style.bold,
            italic: ex_style.italic,
            text_decoration: TextDecoration {
                underline: UnderlineStyle::Solid,
                strikethrough: true,
            },
        };

        assert_eq!(rust_style.fg, Some("#ff0000".to_string()));
        assert_eq!(rust_style.bg, Some("#ffffff".to_string()));
        assert!(rust_style.bold);
        assert!(rust_style.italic);
        assert_eq!(rust_style.text_decoration.underline, UnderlineStyle::Solid);
        assert!(rust_style.text_decoration.strikethrough);

        let back_to_ex: ExStyle = (&rust_style).into();
        assert_eq!(
            back_to_ex.text_decoration.underline,
            ex_style.text_decoration.underline
        );
        assert_eq!(back_to_ex.bold, ex_style.bold);
        assert_eq!(back_to_ex.italic, ex_style.italic);
        assert_eq!(
            back_to_ex.text_decoration.strikethrough,
            ex_style.text_decoration.strikethrough
        );
    }

    #[test]
    fn test_ex_style_undercurl_conversion() {
        use themes::UnderlineStyle;

        let rust_style: themes::Style = themes::Style {
            fg: None,
            bg: None,
            bold: false,
            italic: false,
            text_decoration: themes::TextDecoration {
                underline: UnderlineStyle::Wavy,
                strikethrough: false,
            },
        };

        assert_eq!(rust_style.text_decoration.underline, UnderlineStyle::Wavy);

        let back_to_ex: ExStyle = (&rust_style).into();
        assert_eq!(
            back_to_ex.text_decoration.underline,
            Some(ExUnderlineStyle::Wavy)
        );
        assert!(!back_to_ex.text_decoration.strikethrough);
    }

    #[test]
    fn test_ex_style_no_underline_conversion() {
        use themes::UnderlineStyle;

        let rust_style: themes::Style = themes::Style {
            fg: None,
            bg: None,
            bold: false,
            italic: false,
            text_decoration: themes::TextDecoration {
                underline: UnderlineStyle::None,
                strikethrough: false,
            },
        };

        assert_eq!(rust_style.text_decoration.underline, UnderlineStyle::None);

        let back_to_ex: ExStyle = (&rust_style).into();
        assert_eq!(back_to_ex.text_decoration.underline, None);
        assert!(!back_to_ex.text_decoration.strikethrough);
    }
}

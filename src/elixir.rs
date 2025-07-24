//! Utility module to integrate with Elixir through Rustler.

use crate::formatter::{html_inline, html_linked, HtmlElement};
use crate::{themes, FormatterOption};
use rustler::{NifStruct, NifTaggedEnum};
use std::collections::HashMap;

#[derive(Debug, NifTaggedEnum)]
pub enum ExFormatterOption<'a> {
    HtmlInline {
        theme: Option<ThemeOrString<'a>>,
        pre_class: Option<&'a str>,
        italic: bool,
        include_highlights: bool,
        highlight_lines: Option<ExHtmlInlineHighlightLines>,
        header: Option<ExHtmlElement>,
    },
    HtmlLinked {
        pre_class: Option<&'a str>,
        highlight_lines: Option<ExHtmlLinkedHighlightLines>,
        header: Option<ExHtmlElement>,
    },
    Terminal {
        theme: Option<ThemeOrString<'a>>,
    },
}

impl Default for ExFormatterOption<'_> {
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
pub enum ThemeOrString<'a> {
    Theme(ExTheme),
    String(&'a str),
}

impl Default for ThemeOrString<'_> {
    fn default() -> Self {
        Self::String("onedark")
    }
}

impl<'a> From<ExFormatterOption<'a>> for FormatterOption<'a> {
    fn from(formatter: ExFormatterOption<'a>) -> Self {
        match formatter {
            ExFormatterOption::HtmlInline {
                theme,
                pre_class,
                italic,
                include_highlights,
                highlight_lines,
                header,
            } => {
                let theme = theme.map(|t| match t {
                    ThemeOrString::Theme(theme) => {
                        let theme: themes::Theme = theme.into();
                        let theme = Box::leak(Box::new(theme));
                        &*theme
                    }
                    ThemeOrString::String(name) => themes::get(name).unwrap_or_else(|_| {
                        let theme = Box::leak(Box::new(themes::Theme::default()));
                        &*theme
                    }),
                });

                let highlight_lines = highlight_lines.map(|hl| html_inline::HighlightLines {
                    lines: hl
                        .lines
                        .into_iter()
                        .map(|line_spec| line_spec.to_range_inclusive())
                        .collect(),
                    style: hl.style.map(|s| match s {
                        ExHtmlInlineHighlightLinesStyle::Theme => {
                            html_inline::HighlightLinesStyle::Theme
                        }
                        ExHtmlInlineHighlightLinesStyle::Style { style } => {
                            html_inline::HighlightLinesStyle::Style(style)
                        }
                    }),
                    class: hl.class,
                });

                let header = header.map(|h| HtmlElement {
                    open_tag: h.open_tag,
                    close_tag: h.close_tag,
                });

                FormatterOption::HtmlInline {
                    theme,
                    pre_class,
                    italic,
                    include_highlights,
                    highlight_lines,
                    header,
                }
            }
            ExFormatterOption::HtmlLinked {
                pre_class,
                highlight_lines,
                header,
            } => {
                let highlight_lines = highlight_lines.map(|hl| html_linked::HighlightLines {
                    lines: hl
                        .lines
                        .into_iter()
                        .map(|line_spec| line_spec.to_range_inclusive())
                        .collect(),
                    class: hl.class,
                });

                let header = header.map(|h| HtmlElement {
                    open_tag: h.open_tag,
                    close_tag: h.close_tag,
                });

                FormatterOption::HtmlLinked {
                    pre_class,
                    highlight_lines,
                    header,
                }
            }
            ExFormatterOption::Terminal { theme } => {
                let theme = theme.map(|t| match t {
                    ThemeOrString::Theme(theme) => {
                        let theme: themes::Theme = theme.into();
                        let theme = Box::leak(Box::new(theme));
                        &*theme
                    }
                    ThemeOrString::String(name) => themes::get(name).unwrap_or_else(|_| {
                        let theme = Box::leak(Box::new(themes::Theme::default()));
                        &*theme
                    }),
                });

                FormatterOption::Terminal { theme }
            }
        }
    }
}

#[derive(Debug, Default, NifStruct)]
#[module = "Autumn.Theme"]
pub struct ExTheme {
    pub name: String,
    pub appearance: String,
    pub revision: String,
    pub highlights: HashMap<String, ExStyle>,
}

impl From<ExTheme> for themes::Theme {
    fn from(theme: ExTheme) -> Self {
        themes::Theme {
            name: theme.name,
            appearance: theme.appearance,
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
                            underline: v.underline,
                            bold: v.bold,
                            italic: v.italic,
                            strikethrough: v.strikethrough,
                        },
                    )
                })
                .collect(),
        }
    }
}

impl<'a> From<&'a themes::Theme> for ExTheme {
    fn from(theme: &'a themes::Theme) -> Self {
        ExTheme {
            name: theme.name.to_owned(),
            appearance: theme.appearance.to_owned(),
            revision: theme.revision.to_owned(),
            highlights: theme
                .highlights
                .iter()
                .map(|(k, v)| (k.to_owned(), ExStyle::from(v)))
                .collect(),
        }
    }
}

#[derive(Debug, Default, NifStruct)]
#[module = "Autumn.Theme.Style"]
pub struct ExStyle {
    pub fg: Option<String>,
    pub bg: Option<String>,
    pub underline: bool,
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
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

#[derive(Clone, Debug, NifTaggedEnum)]
pub enum ExHtmlInlineHighlightLinesStyle {
    Theme,
    Style { style: String },
}

impl Default for ExHtmlInlineHighlightLinesStyle {
    fn default() -> Self {
        Self::Theme
    }
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
            underline: style.underline,
            bold: style.bold,
            italic: style.italic,
            strikethrough: style.strikethrough,
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
    use crate::{highlight, themes, Options};
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
        // Test string theme
        let theme_str = ThemeOrString::String("dracula");
        match theme_str {
            ThemeOrString::String(name) => assert_eq!(name, "dracula"),
            _ => panic!("Should be String variant"),
        }

        // Test theme object
        let ex_theme = ExTheme {
            name: "test_theme".to_string(),
            appearance: "dark".to_string(),
            revision: "1.0".to_string(),
            highlights: HashMap::new(),
        };
        let theme_obj = ThemeOrString::Theme(ex_theme);
        match theme_obj {
            ThemeOrString::Theme(theme) => {
                assert_eq!(theme.name, "test_theme");
                assert_eq!(theme.appearance, "dark");
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

        // Convert to Rust type through the From implementation in the formatter conversion
        let formatter_option = ExFormatterOption::HtmlInline {
            theme: None,
            pre_class: None,
            italic: false,
            include_highlights: false,
            highlight_lines: Some(ex_highlight_lines),
            header: None,
        };

        let rust_option: FormatterOption = formatter_option.into();
        match rust_option {
            FormatterOption::HtmlInline {
                highlight_lines, ..
            } => {
                let hl = highlight_lines.unwrap();
                assert_eq!(hl.lines.len(), 2);
                assert_eq!(*hl.lines[0].start(), 1);
                assert_eq!(*hl.lines[0].end(), 1);
                assert_eq!(*hl.lines[1].start(), 3);
                assert_eq!(*hl.lines[1].end(), 5);

                match hl.style {
                    Some(html_inline::HighlightLinesStyle::Style(style)) => {
                        assert_eq!(style, "background-color: yellow");
                    }
                    _ => panic!("Should be Style variant"),
                }
            }
            _ => panic!("Should be HtmlInline variant"),
        }
    }

    #[test]
    fn test_ex_html_linked_highlight_lines_conversion() {
        let ex_highlight_lines = ExHtmlLinkedHighlightLines {
            lines: vec![ExLineSpec::Range { start: 2, end: 4 }],
            class: "highlighted".to_string(),
        };

        let formatter_option = ExFormatterOption::HtmlLinked {
            pre_class: None,
            highlight_lines: Some(ex_highlight_lines),
            header: None,
        };

        let rust_option: FormatterOption = formatter_option.into();
        match rust_option {
            FormatterOption::HtmlLinked {
                highlight_lines, ..
            } => {
                let hl = highlight_lines.unwrap();
                assert_eq!(hl.lines.len(), 1);
                assert_eq!(*hl.lines[0].start(), 2);
                assert_eq!(*hl.lines[0].end(), 4);
                assert_eq!(hl.class, "highlighted");
            }
            _ => panic!("Should be HtmlLinked variant"),
        }
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
                underline: false,
                strikethrough: false,
            },
        );

        let ex_theme = ExTheme {
            name: "test_theme".to_string(),
            appearance: "dark".to_string(),
            revision: "1.0".to_string(),
            highlights,
        };

        let rust_theme: themes::Theme = ex_theme.into();
        assert_eq!(rust_theme.name, "test_theme");
        assert_eq!(rust_theme.appearance, "dark");
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
            theme: Some(ThemeOrString::String("dracula")),
            pre_class: Some("code-block"),
            italic: false,
            include_highlights: false,
            highlight_lines: None,
            header: None,
        };

        let rust_formatter: FormatterOption = ex_formatter.into();
        let options = Options {
            lang_or_file: Some("rust"),
            formatter: rust_formatter,
        };

        let result = highlight(code, options);
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
            theme: Some(ThemeOrString::String("github_light")),
            pre_class: None,
            italic: false,
            include_highlights: false,
            highlight_lines: Some(highlight_lines),
            header: None,
        };

        let rust_formatter: FormatterOption = ex_formatter.into();
        let options = Options {
            lang_or_file: Some("text"),
            formatter: rust_formatter,
        };

        let result = highlight(code, options);
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

        let rust_formatter: FormatterOption = ex_formatter.into();
        let options = Options {
            lang_or_file: Some("javascript"),
            formatter: rust_formatter,
        };

        let result = highlight(code, options);
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
            pre_class: Some("syntax-highlight"),
            highlight_lines: Some(highlight_lines),
            header: Some(header),
        };

        let rust_formatter: FormatterOption = ex_formatter.into();
        let options = Options {
            lang_or_file: Some("elixir"),
            formatter: rust_formatter,
        };

        let result = highlight(code, options);
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
            theme: Some(ThemeOrString::String("github_dark")),
        };

        let rust_formatter: FormatterOption = ex_formatter.into();
        let options = Options {
            lang_or_file: Some("ruby"),
            formatter: rust_formatter,
        };

        let result = highlight(code, options);
        let expected = "\x1b[0m\x1b[38;2;210;168;255mputs\x1b[0m \x1b[0m\x1b[38;2;165;214;255m'\x1b[0m\x1b[0m\x1b[38;2;165;214;255mHello Ruby\x1b[0m\x1b[0m\x1b[38;2;165;214;255m'\x1b[0m";
        assert_str_eq!(result, expected);
    }

    #[test]
    fn test_ex_html_inline_highlight_lines_style_theme() {
        let highlight_lines = ExHtmlInlineHighlightLines {
            lines: vec![ExLineSpec::Range { start: 1, end: 3 }],
            style: Some(ExHtmlInlineHighlightLinesStyle::Theme),
            class: None,
        };

        let ex_formatter = ExFormatterOption::HtmlInline {
            theme: Some(ThemeOrString::String("catppuccin_mocha")),
            pre_class: None,
            italic: false,
            include_highlights: false,
            highlight_lines: Some(highlight_lines),
            header: None,
        };

        let rust_formatter: FormatterOption = ex_formatter.into();
        match rust_formatter {
            FormatterOption::HtmlInline {
                highlight_lines, ..
            } => {
                let hl = highlight_lines.unwrap();
                match hl.style {
                    Some(html_inline::HighlightLinesStyle::Theme) => {
                        // Expected behavior
                    }
                    _ => panic!("Should be Theme variant"),
                }
            }
            _ => panic!("Should be HtmlInline variant"),
        }
    }

    #[test]
    fn test_ex_line_spec_single_and_range() {
        // Test both single lines and ranges
        let highlight_lines = ExHtmlInlineHighlightLines {
            lines: vec![
                ExLineSpec::Single(1),                  // Single line
                ExLineSpec::Range { start: 3, end: 5 }, // Range
                ExLineSpec::Single(7),                  // Another single line
            ],
            style: Some(ExHtmlInlineHighlightLinesStyle::Theme),
            class: None,
        };

        let formatter_option = ExFormatterOption::HtmlInline {
            theme: Some(ThemeOrString::String("github_light")),
            pre_class: None,
            italic: false,
            include_highlights: false,
            highlight_lines: Some(highlight_lines),
            header: None,
        };

        let rust_formatter: FormatterOption = formatter_option.into();
        match rust_formatter {
            FormatterOption::HtmlInline {
                highlight_lines, ..
            } => {
                let hl = highlight_lines.unwrap();
                assert_eq!(hl.lines.len(), 3);

                // First line should be 1..=1 (single line)
                assert_eq!(*hl.lines[0].start(), 1);
                assert_eq!(*hl.lines[0].end(), 1);

                // Second line should be 3..=5 (range)
                assert_eq!(*hl.lines[1].start(), 3);
                assert_eq!(*hl.lines[1].end(), 5);

                // Third line should be 7..=7 (single line)
                assert_eq!(*hl.lines[2].start(), 7);
                assert_eq!(*hl.lines[2].end(), 7);
            }
            _ => panic!("Should be HtmlInline variant"),
        }
    }

    #[test]
    fn test_error_handling_invalid_theme_name() {
        let ex_formatter = ExFormatterOption::HtmlInline {
            theme: Some(ThemeOrString::String("nonexistent_theme")),
            pre_class: None,
            italic: false,
            include_highlights: false,
            highlight_lines: None,
            header: None,
        };

        // This should not panic but fall back to default theme
        let rust_formatter: FormatterOption = ex_formatter.into();
        match rust_formatter {
            FormatterOption::HtmlInline { theme, .. } => {
                assert!(theme.is_some());
                // Should have fallen back to default theme
            }
            _ => panic!("Should be HtmlInline variant"),
        }
    }
}

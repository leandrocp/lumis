#![allow(unused_must_use)]

use super::{Formatter, HtmlFormatter};
use crate::languages::Language;
use crate::themes::Theme;
use std::{
    io::{self, Write},
    ops::RangeInclusive,
};
use tree_sitter_highlight::Highlighter;

/// Configuration for highlighting specific lines in HTML inline output.
///
/// This struct allows you to specify which lines should be highlighted and how
/// they should be styled using either theme-based styling or custom CSS.
///
/// # Examples
///
/// Using theme-based highlighting (requires a theme with 'cursorline' style):
/// ```rust
/// use autumnus::formatter::html_inline::{HighlightLines, HighlightLinesStyle};
///
/// let highlight_lines = HighlightLines {
///     lines: vec![1..=1, 5..=7],  // Highlight lines 1, 5, 6, and 7
///     style: HighlightLinesStyle::Theme,
/// };
/// ```
///
/// Using custom CSS styling:
/// ```rust
/// use autumnus::formatter::html_inline::{HighlightLines, HighlightLinesStyle};
///
/// let highlight_lines = HighlightLines {
///     lines: vec![2..=3],  // Highlight lines 2 and 3
///     style: HighlightLinesStyle::Style("background-color: yellow; border-left: 3px solid red".to_string()),
/// };
/// ```
#[derive(Clone, Debug)]
pub struct HighlightLines {
    /// List of line ranges to highlight.
    ///
    /// Each range is inclusive on both ends. Line numbers are 1-based.
    /// Multiple ranges can overlap and will be merged during rendering.
    pub lines: Vec<RangeInclusive<usize>>,
    /// The styling method to use for highlighted lines.
    pub style: HighlightLinesStyle,
}

/// Defines how highlighted lines should be styled in HTML inline output.
#[derive(Clone, Debug)]
pub enum HighlightLinesStyle {
    /// Use the theme's 'cursorline' style if available.
    ///
    /// This looks for a 'cursorline' style definition in the current theme.
    /// If no theme is provided or the theme doesn't define 'cursorline',
    /// no styling will be applied.
    Theme,
    /// Use a custom CSS style string.
    ///
    /// The provided string will be used directly as the `style` attribute
    /// for highlighted line elements. Should contain valid CSS properties.
    ///
    /// # Example
    /// ```rust
    /// use autumnus::formatter::html_inline::HighlightLinesStyle;
    ///
    /// let style = HighlightLinesStyle::Style(
    ///     "background-color: rgba(255, 255, 0, 0.3); border-left: 2px solid orange".to_string()
    /// );
    /// ```
    Style(String),
}

impl Default for HighlightLines {
    fn default() -> Self {
        Self {
            lines: Vec::new(),
            style: HighlightLinesStyle::Theme,
        }
    }
}

#[derive(Debug)]
pub struct HtmlInline<'a> {
    source: &'a str,
    lang: Language,
    theme: Option<&'a Theme>,
    pre_class: Option<&'a str>,
    italic: bool,
    include_highlights: bool,
    highlight_lines: Option<HighlightLines>,
}

impl<'a> HtmlInline<'a> {
    pub fn new(
        source: &'a str,
        lang: Language,
        theme: Option<&'a Theme>,
        pre_class: Option<&'a str>,
        italic: bool,
        include_highlights: bool,
        highlight_lines: Option<HighlightLines>,
    ) -> Self {
        Self {
            source,
            lang,
            theme,
            pre_class,
            italic,
            include_highlights,
            highlight_lines,
        }
    }

    pub fn builder() -> Self {
        Self::default()
    }

    pub fn with_source(mut self, source: &'a str) -> Self {
        self.source = source;
        self
    }

    pub fn with_lang(mut self, lang: Language) -> Self {
        self.lang = lang;
        self
    }

    pub fn with_theme(mut self, theme: Option<&'a Theme>) -> Self {
        self.theme = theme;
        self
    }

    pub fn with_pre_class(mut self, pre_class: Option<&'a str>) -> Self {
        self.pre_class = pre_class;
        self
    }

    pub fn with_italic(mut self, italic: bool) -> Self {
        self.italic = italic;
        self
    }

    pub fn with_include_highlights(mut self, include_highlights: bool) -> Self {
        self.include_highlights = include_highlights;
        self
    }

    pub fn highlight_lines(
        mut self,
        lines: Vec<RangeInclusive<usize>>,
        style: HighlightLinesStyle,
    ) -> Self {
        self.highlight_lines = Some(HighlightLines { lines, style });
        self
    }
}

impl Default for HtmlInline<'_> {
    fn default() -> Self {
        Self {
            source: "",
            lang: Language::PlainText,
            theme: None,
            pre_class: None,
            italic: false,
            include_highlights: false,
            highlight_lines: None,
        }
    }
}

impl Formatter for HtmlInline<'_> {
    fn highlights(&self, output: &mut dyn Write) -> io::Result<()> {
        let mut highlighter = Highlighter::new();
        let events = highlighter
            .highlight(
                self.lang.config(),
                self.source.as_bytes(),
                None,
                |injected| Some(Language::guess(injected, "").config()),
            )
            .expect("failed to generate highlight events");

        let mut renderer = tree_sitter_highlight::HtmlRenderer::new();

        renderer
            .render(events, self.source.as_bytes(), &move |highlight, output| {
                let scope = crate::constants::HIGHLIGHT_NAMES[highlight.0];

                if self.include_highlights {
                    output.extend("data-highlight=\"".as_bytes());
                    output.extend(scope.as_bytes());
                    output.extend(b"\"");
                }

                if let Some(theme) = self.theme {
                    if let Some(style) = theme.get_style(scope) {
                        if self.include_highlights {
                            output.extend(b" ");
                        }

                        output.extend(b"style=\"");
                        output.extend(style.css(self.italic, " ").as_bytes());
                        output.extend(b"\"");
                    }
                }
            })
            .expect("failed to render highlight events");

        for (i, line) in renderer.lines().enumerate() {
            let line_number = i + 1;
            let highlighted_style = if let Some(ref highlight_lines) = self.highlight_lines {
                if highlight_lines
                    .lines
                    .iter()
                    .any(|range| range.contains(&line_number))
                {
                    match &highlight_lines.style {
                        HighlightLinesStyle::Theme => {
                            if let Some(theme) = self.theme {
                                if let Some(cursorline_style) = theme.get_style("cursorline") {
                                    format!(" style=\"{}\"", cursorline_style.css(self.italic, " "))
                                } else {
                                    String::new()
                                }
                            } else {
                                String::new()
                            }
                        }
                        HighlightLinesStyle::Style(style_string) => {
                            format!(" style=\"{}\"", style_string)
                        }
                    }
                } else {
                    String::new()
                }
            } else {
                String::new()
            };

            write!(
                output,
                "<span class=\"line\"{} data-line=\"{}\">{}</span>",
                highlighted_style,
                line_number,
                line.replace('{', "&lbrace;").replace('}', "&rbrace;")
            )?;
        }
        Ok(())
    }

    fn format(&self, output: &mut dyn Write) -> io::Result<()> {
        let mut buffer = Vec::new();
        self.open_pre_tag(&mut buffer)?;
        self.open_code_tag(&mut buffer)?;
        self.highlights(&mut buffer)?;
        self.closing_tags(&mut buffer)?;
        write!(output, "{}", &String::from_utf8(buffer).unwrap())?;
        Ok(())
    }
}

impl HtmlFormatter for HtmlInline<'_> {
    fn open_pre_tag(&self, output: &mut dyn Write) -> io::Result<()> {
        let class = if let Some(pre_class) = &self.pre_class {
            format!("athl {}", pre_class)
        } else {
            "athl".to_string()
        };

        write!(
            output,
            "<pre class=\"{}\"{}>",
            class,
            &self
                .theme
                .and_then(|theme| theme.pre_style(" "))
                .map(|pre_style| format!(" style=\"{}\"", pre_style))
                .unwrap_or_default(),
        )
    }

    fn open_code_tag(&self, output: &mut dyn Write) -> io::Result<()> {
        write!(
            output,
            "<code class=\"language-{}\" translate=\"no\" tabindex=\"0\">",
            self.lang.id_name()
        )
    }

    fn closing_tags(&self, output: &mut dyn Write) -> io::Result<()> {
        output.write_all(b"</code></pre>")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::themes;

    #[test]
    fn test_do_not_append_pre_style_if_missing_theme_style() {
        let formatter = HtmlInline::default();
        let mut buffer = Vec::new();
        formatter.open_pre_tag(&mut buffer);
        let pre_tag = String::from_utf8(buffer).unwrap();
        assert!(pre_tag.contains("<pre class=\"athl\">"));
    }

    #[test]
    fn test_include_pre_class() {
        let formatter = HtmlInline::new(
            "",
            Language::PlainText,
            None,
            Some("test-pre-class"),
            false,
            false,
            None,
        );
        let mut buffer = Vec::new();
        formatter.open_pre_tag(&mut buffer);
        let pre_tag = String::from_utf8(buffer).unwrap();
        assert!(pre_tag.contains("<pre class=\"athl test-pre-class\">"));
    }

    #[test]
    fn test_include_pre_class_with_theme() {
        let theme = themes::get("github_light").unwrap();
        let formatter = HtmlInline::new(
            "",
            Language::PlainText,
            Some(theme),
            Some("test-pre-class"),
            false,
            false,
            None,
        );
        let mut buffer = Vec::new();
        formatter.open_pre_tag(&mut buffer);
        let pre_tag = String::from_utf8(buffer).unwrap();
        assert!(pre_tag.contains("<pre class=\"athl test-pre-class\" style=\"color: #1f2328; background-color: #ffffff;\">"));
    }

    #[test]
    fn test_builder_pattern() {
        let theme = themes::get("github_light").unwrap();
        let formatter = HtmlInline::default()
            .with_lang(Language::Rust)
            .with_theme(Some(theme))
            .with_pre_class(Some("test-pre-class"))
            .with_italic(true)
            .with_include_highlights(true);

        let mut buffer = Vec::new();
        formatter.open_pre_tag(&mut buffer);
        let pre_tag = String::from_utf8(buffer).unwrap();
        assert!(pre_tag.contains("<pre class=\"athl test-pre-class\" style=\"color: #1f2328; background-color: #ffffff;\">"));
    }

    #[test]
    fn test_highlight_lines_with_theme() {
        let theme = themes::get("github_light").unwrap();
        let highlight_lines = HighlightLines {
            lines: vec![1..=1, 3..=4],
            style: HighlightLinesStyle::Theme,
        };
        let code = "line 1\nline 2\nline 3\nline 4\nline 5";
        let formatter = HtmlInline::new(
            code,
            Language::PlainText,
            Some(theme),
            None,
            false,
            false,
            Some(highlight_lines),
        );

        let mut buffer = Vec::new();
        formatter.format(&mut buffer).unwrap();
        let result = String::from_utf8(buffer).unwrap();

        println!("{}", result);

        assert!(result
            .contains(r#"<span class="line" style="background-color: #e7eaf0;" data-line="1">"#));
        assert!(result.contains(r#"<span class="line" data-line="2">"#));
        assert!(result
            .contains(r#"<span class="line" style="background-color: #e7eaf0;" data-line="3">"#));
        assert!(result
            .contains(r#"<span class="line" style="background-color: #e7eaf0;" data-line="4">"#));
        assert!(result.contains(r#"<span class="line" data-line="5">"#));
    }

    #[test]
    fn test_highlight_lines_with_custom_style() {
        let highlight_lines = HighlightLines {
            lines: vec![1..=1, 3..=4],
            style: HighlightLinesStyle::Style("background-color: yellow".to_string()),
        };
        let code = "line 1\nline 2\nline 3\nline 4\nline 5";
        let formatter = HtmlInline::new(
            code,
            Language::PlainText,
            None,
            None,
            false,
            false,
            Some(highlight_lines),
        );

        let mut buffer = Vec::new();
        formatter.format(&mut buffer).unwrap();
        let result = String::from_utf8(buffer).unwrap();

        assert!(result
            .contains(r#"<span class="line" style="background-color: yellow" data-line="1">"#));
        assert!(result.contains(r#"<span class="line" data-line="2">"#));
        assert!(result
            .contains(r#"<span class="line" style="background-color: yellow" data-line="3">"#));
        assert!(result
            .contains(r#"<span class="line" style="background-color: yellow" data-line="4">"#));
        assert!(result.contains(r#"<span class="line" data-line="5">"#));
    }
}

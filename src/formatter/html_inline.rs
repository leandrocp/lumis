//! HTML formatter with inline CSS styles.
//!
//! This module provides the [`HtmlInline`] formatter that generates HTML output with
//! inline CSS styles for syntax highlighting. It supports themes, line highlighting,
//! and various customization options.
//!
//! See the [formatter](crate::formatter) module for more information and examples.

#![allow(unused_must_use)]

use super::{Formatter, HtmlElement, HtmlFormatter};
use crate::languages::Language;
use crate::themes::Theme;
use derive_builder::Builder;
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
/// Using theme-based highlighting (requires a theme with 'visual' style):
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
    /// Use the theme's 'visual' style if available.
    ///
    /// This looks for a 'visual' style definition in the current theme.
    /// If no theme is provided or the theme doesn't define 'visual',
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

#[derive(Builder, Debug)]
#[builder(default)]
pub struct HtmlInline<'a> {
    source: &'a str,
    lang: Language,
    theme: Option<&'a Theme>,
    pre_class: Option<&'a str>,
    italic: bool,
    include_highlights: bool,
    highlight_lines: Option<HighlightLines>,
    header: Option<HtmlElement>,
}

impl<'a> HtmlInlineBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a> HtmlInline<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        source: &'a str,
        lang: Language,
        theme: Option<&'a Theme>,
        pre_class: Option<&'a str>,
        italic: bool,
        include_highlights: bool,
        highlight_lines: Option<HighlightLines>,
        header: Option<HtmlElement>,
    ) -> Self {
        Self {
            source,
            lang,
            theme,
            pre_class,
            italic,
            include_highlights,
            highlight_lines,
            header,
        }
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
            header: None,
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
                                if let Some(visual_style) = theme.get_style("visual") {
                                    format!(" style=\"{}\"", visual_style.css(self.italic, " "))
                                } else {
                                    String::new()
                                }
                            } else {
                                String::new()
                            }
                        }
                        HighlightLinesStyle::Style(style_string) => {
                            format!(" style=\"{style_string}\"")
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

        if let Some(ref header) = self.header {
            write!(buffer, "{}", header.open_tag)?;
        }

        self.open_pre_tag(&mut buffer)?;
        self.open_code_tag(&mut buffer)?;
        self.highlights(&mut buffer)?;
        self.closing_tags(&mut buffer)?;

        if let Some(ref header) = self.header {
            write!(buffer, "{}", header.close_tag)?;
        }

        write!(output, "{}", &String::from_utf8(buffer).unwrap())?;
        Ok(())
    }
}

impl HtmlFormatter for HtmlInline<'_> {
    fn open_pre_tag(&self, output: &mut dyn Write) -> io::Result<()> {
        let class = if let Some(pre_class) = &self.pre_class {
            format!("athl {pre_class}")
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
                .map(|pre_style| format!(" style=\"{pre_style}\""))
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
    use crate::formatter::HtmlInlineBuilder;
    use crate::themes;

    #[test]
    fn test_no_attrs() {
        let formatter = HtmlInline::new(
            "@lang :rust",
            Language::Elixir,
            None,
            None,
            false,
            false,
            None,
            None,
        );
        let mut buffer = Vec::new();
        formatter.format(&mut buffer);
        let result = String::from_utf8(buffer).unwrap();
        let expected = r#"<pre class="athl"><code class="language-elixir" translate="no" tabindex="0"><span class="line" data-line="1"><span ><span >@<span ><span >lang <span >:rust</span></span></span></span></span>
</span></code></pre>"#;
        assert_eq!(result, expected)
    }

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
        let formatter = HtmlInlineBuilder::new()
            .source("")
            .lang(Language::Rust)
            .theme(Some(theme))
            .pre_class(Some("test-pre-class"))
            .italic(true)
            .include_highlights(true)
            .build()
            .unwrap();

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
            None,
        );

        let mut buffer = Vec::new();
        formatter.format(&mut buffer).unwrap();
        let result = String::from_utf8(buffer).unwrap();

        println!("{result}");

        assert!(result
            .contains(r#"<span class="line" style="background-color: #dae9f9;" data-line="1">"#));
        assert!(result.contains(r#"<span class="line" data-line="2">"#));
        assert!(result
            .contains(r#"<span class="line" style="background-color: #dae9f9;" data-line="3">"#));
        assert!(result
            .contains(r#"<span class="line" style="background-color: #dae9f9;" data-line="4">"#));
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
            None,
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

    #[test]
    fn test_header_wrapping() {
        let header = HtmlElement {
            open_tag: "<div class=\"code-wrapper\">".to_string(),
            close_tag: "</div>".to_string(),
        };
        let code = "line 1\nline 2";
        let formatter = HtmlInline::new(
            code,
            Language::PlainText,
            None,
            None,
            false,
            false,
            None,
            Some(header),
        );

        let mut buffer = Vec::new();
        formatter.format(&mut buffer).unwrap();
        let result = String::from_utf8(buffer).unwrap();

        assert!(result.starts_with("<div class=\"code-wrapper\">"));
        assert!(result.ends_with("</div>"));
        assert!(result.contains("<pre class=\"athl\">")); // Ensure the pre tag is inside
    }

    #[test]
    fn test_header_with_complex_structure() {
        let header = HtmlElement {
            open_tag: "<section class=\"highlight\" data-lang=\"rust\">".to_string(),
            close_tag: "</section>".to_string(),
        };
        let code = "fn main() { }";
        let formatter = HtmlInline::new(
            code,
            Language::Rust,
            None,
            Some("custom-class"),
            false,
            false,
            None,
            Some(header),
        );

        let mut buffer = Vec::new();
        formatter.format(&mut buffer).unwrap();
        let result = String::from_utf8(buffer).unwrap();

        assert!(result.starts_with("<section class=\"highlight\" data-lang=\"rust\">"));
        assert!(result.ends_with("</section>"));
        assert!(result.contains("<pre class=\"athl custom-class\">"));
        assert!(result.contains("<code class=\"language-rust\""));
    }
}

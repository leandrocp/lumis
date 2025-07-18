//! HTML formatter with linked CSS classes.
//!
//! This module provides the [`HtmlLinked`] formatter that generates HTML output with
//! CSS classes for syntax highlighting. Requires external CSS files for styling.
//! Supports line highlighting and custom CSS classes.
//!
//! See the [formatter](crate::formatter) module for more information and examples.

#![allow(unused_must_use)]

use super::{Formatter, HtmlElement, HtmlFormatter};
use crate::constants::CLASSES;
use crate::languages::Language;
use derive_builder::Builder;
use std::{
    io::{self, Write},
    ops::RangeInclusive,
};
use tree_sitter_highlight::Highlighter;

/// Configuration for highlighting specific lines in HTML linked output.
///
/// This struct allows you to specify which lines should be highlighted using
/// CSS classes. The highlighting is applied by adding the specified class
/// to the line elements, allowing for flexible styling via external CSS.
///
/// # Examples
///
/// Basic usage with default "visual" class:
/// ```rust
/// use autumnus::formatter::html_linked::HighlightLines;
///
/// let highlight_lines = HighlightLines {
///     lines: vec![1..=1, 5..=7],  // Highlight lines 1, 5, 6, and 7
///     class: "visual".to_string(),
/// };
/// ```
///
/// Using a custom CSS class:
/// ```rust
/// use autumnus::formatter::html_linked::HighlightLines;
///
/// let highlight_lines = HighlightLines {
///     lines: vec![2..=3],  // Highlight lines 2 and 3
///     class: "highlighted-line".to_string(),
/// };
/// ```
///
/// The resulting HTML will include the class in line elements:
/// ```html
/// <span class="line highlighted-line" data-line="2">...</span>
/// <span class="line highlighted-line" data-line="3">...</span>
/// ```
#[derive(Clone, Debug)]
pub struct HighlightLines {
    /// List of line ranges to highlight.
    ///
    /// Each range is inclusive on both ends. Line numbers are 1-based.
    /// Multiple ranges can overlap and will be merged during rendering.
    pub lines: Vec<RangeInclusive<usize>>,
    /// The CSS class name to add to highlighted line elements.
    ///
    /// This class will be added to the existing "line" class for highlighted lines,
    /// resulting in elements like `<span class="line your-class-name" data-line="N">`.
    /// You can then style this class in your CSS to achieve the desired highlighting effect.
    pub class: String,
}

impl Default for HighlightLines {
    fn default() -> Self {
        Self {
            lines: Vec::new(),
            class: "visual".to_string(),
        }
    }
}

#[derive(Builder, Debug)]
#[builder(default)]
pub struct HtmlLinked<'a> {
    source: &'a str,
    lang: Language,
    pre_class: Option<&'a str>,
    highlight_lines: Option<HighlightLines>,
    header: Option<HtmlElement>,
}

impl<'a> HtmlLinkedBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a> HtmlLinked<'a> {
    pub fn new(
        source: &'a str,
        lang: Language,
        pre_class: Option<&'a str>,
        highlight_lines: Option<HighlightLines>,
        header: Option<HtmlElement>,
    ) -> Self {
        Self {
            source,
            lang,
            pre_class,
            highlight_lines,
            header,
        }
    }
}

impl Default for HtmlLinked<'_> {
    fn default() -> Self {
        Self {
            source: "",
            lang: Language::PlainText,
            pre_class: None,
            highlight_lines: None,
            header: None,
        }
    }
}

impl Formatter for HtmlLinked<'_> {
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
                let class = CLASSES[highlight.0];

                output.extend(b"class=\"");
                output.extend(class.as_bytes());
                output.extend(b"\"");
            })
            .expect("failed to render highlight events");

        for (i, line) in renderer.lines().enumerate() {
            let line_number = i + 1;
            let highlighted_class = if let Some(ref highlight_lines) = self.highlight_lines {
                if highlight_lines
                    .lines
                    .iter()
                    .any(|range| range.contains(&line_number))
                {
                    format!(" {}", highlight_lines.class)
                } else {
                    String::new()
                }
            } else {
                String::new()
            };

            write!(
                output,
                "<span class=\"line{}\" data-line=\"{}\">{}</span>",
                highlighted_class,
                line_number,
                line.replace('{', "&lbrace;").replace('}', "&rbrace;")
            );
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

impl HtmlFormatter for HtmlLinked<'_> {
    fn open_pre_tag(&self, output: &mut dyn Write) -> io::Result<()> {
        let class = if let Some(pre_class) = self.pre_class {
            format!("athl {pre_class}")
        } else {
            "athl".to_string()
        };

        write!(output, "<pre class=\"{class}\">")
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
    use crate::formatter::HtmlLinkedBuilder;

    #[test]
    fn test_no_attrs() {
        let formatter = HtmlLinked::new("@lang :rust", Language::Elixir, None, None, None);
        let mut buffer = Vec::new();
        formatter.format(&mut buffer);
        let result = String::from_utf8(buffer).unwrap();
        let expected = r#"<pre class="athl"><code class="language-elixir" translate="no" tabindex="0"><span class="line" data-line="1"><span class="operator"><span class="constant">@<span class="function-call"><span class="constant">lang <span class="string-special-symbol">:rust</span></span></span></span></span>
</span></code></pre>"#;
        assert_eq!(result, expected)
    }

    #[test]
    fn test_include_pre_class() {
        let formatter =
            HtmlLinked::new("", Language::PlainText, Some("test-pre-class"), None, None);
        let mut buffer = Vec::new();
        formatter.open_pre_tag(&mut buffer);
        let pre_tag = String::from_utf8(buffer).unwrap();
        assert!(pre_tag.contains("<pre class=\"athl test-pre-class\">"));
    }

    #[test]
    fn test_code_tag_with_language() {
        let formatter = HtmlLinked::new("", Language::Rust, None, None, None);
        let mut buffer = Vec::new();
        formatter.open_code_tag(&mut buffer);
        let code_tag = String::from_utf8(buffer).unwrap();
        assert!(code_tag.contains("<code class=\"language-rust\" translate=\"no\" tabindex=\"0\">"));
    }

    #[test]
    fn test_builder_pattern() {
        let formatter = HtmlLinkedBuilder::new()
            .source("")
            .lang(Language::Rust)
            .pre_class(Some("test-pre-class"))
            .build()
            .unwrap();

        let mut buffer = Vec::new();
        formatter.open_pre_tag(&mut buffer);
        let pre_tag = String::from_utf8(buffer).unwrap();
        assert!(pre_tag.contains("<pre class=\"athl test-pre-class\">"));

        let mut buffer = Vec::new();
        formatter.open_code_tag(&mut buffer);
        let code_tag = String::from_utf8(buffer).unwrap();
        assert!(code_tag.contains("<code class=\"language-rust\" translate=\"no\" tabindex=\"0\">"));
    }

    #[test]
    fn test_highlight_lines_functionality() {
        let code = "line 1\nline 2\nline 3\nline 4\nline 5";
        let highlight_lines = HighlightLines {
            lines: vec![1..=1, 3..=4],
            class: "highlighted".to_string(),
        };
        let formatter =
            HtmlLinked::new(code, Language::PlainText, None, Some(highlight_lines), None);

        let mut buffer = Vec::new();
        formatter.format(&mut buffer).unwrap();
        let result = String::from_utf8(buffer).unwrap();

        assert!(result.contains("class=\"line highlighted\" data-line=\"1\""));
        assert!(result.contains("class=\"line\" data-line=\"2\""));
        assert!(result.contains("class=\"line highlighted\" data-line=\"3\""));
        assert!(result.contains("class=\"line highlighted\" data-line=\"4\""));
        assert!(result.contains("class=\"line\" data-line=\"5\""));
    }

    #[test]
    fn test_header_wrapping() {
        let header = HtmlElement {
            open_tag: "<div class=\"code-wrapper\">".to_string(),
            close_tag: "</div>".to_string(),
        };
        let code = "line 1\nline 2";
        let formatter = HtmlLinked::new(code, Language::PlainText, None, None, Some(header));

        let mut buffer = Vec::new();
        formatter.format(&mut buffer).unwrap();
        let result = String::from_utf8(buffer).unwrap();

        assert!(result.starts_with("<div class=\"code-wrapper\">"));
        assert!(result.ends_with("</div>"));
        assert!(result.contains("<pre class=\"athl\">")); // Ensure the pre tag is inside
    }

    #[test]
    fn test_header_with_highlight_lines() {
        let header = HtmlElement {
            open_tag: "<section class=\"code-section\">".to_string(),
            close_tag: "</section>".to_string(),
        };
        let highlight_lines = HighlightLines {
            lines: vec![1..=1],
            class: "highlighted".to_string(),
        };
        let code = "line 1\nline 2";
        let formatter = HtmlLinked::new(
            code,
            Language::PlainText,
            Some("custom-pre"),
            Some(highlight_lines),
            Some(header),
        );

        let mut buffer = Vec::new();
        formatter.format(&mut buffer).unwrap();
        let result = String::from_utf8(buffer).unwrap();

        assert!(result.starts_with("<section class=\"code-section\">"));
        assert!(result.ends_with("</section>"));
        assert!(result.contains("<pre class=\"athl custom-pre\">"));
        assert!(result.contains("class=\"line highlighted\" data-line=\"1\""));
        assert!(result.contains("class=\"line\" data-line=\"2\""));
    }
}

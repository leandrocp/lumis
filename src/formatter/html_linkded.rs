#![allow(unused_must_use)]

use super::{Formatter, HtmlFormatter};
use crate::constants::CLASSES;
use crate::languages::Language;
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
/// Basic usage with default "cursorline" class:
/// ```rust
/// use autumnus::formatter::html_linkded::HighlightLines;
///
/// let highlight_lines = HighlightLines {
///     lines: vec![1..=1, 5..=7],  // Highlight lines 1, 5, 6, and 7
///     class: "cursorline".to_string(),
/// };
/// ```
///
/// Using a custom CSS class:
/// ```rust
/// use autumnus::formatter::html_linkded::HighlightLines;
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
            class: "cursorline".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct HtmlLinked<'a> {
    source: &'a str,
    lang: Language,
    pre_class: Option<&'a str>,
    highlight_lines: Option<HighlightLines>,
}

impl<'a> HtmlLinked<'a> {
    pub fn new(
        source: &'a str,
        lang: Language,
        pre_class: Option<&'a str>,
        highlight_lines: Option<HighlightLines>,
    ) -> Self {
        Self {
            source,
            lang,
            pre_class,
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

    pub fn with_pre_class(mut self, pre_class: Option<&'a str>) -> Self {
        self.pre_class = pre_class;
        self
    }

    pub fn highlight_lines(mut self, lines: Vec<RangeInclusive<usize>>, class: String) -> Self {
        self.highlight_lines = Some(HighlightLines { lines, class });
        self
    }
}

impl Default for HtmlLinked<'_> {
    fn default() -> Self {
        Self {
            source: "",
            lang: Language::PlainText,
            pre_class: None,
            highlight_lines: None,
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
        self.open_pre_tag(&mut buffer)?;
        self.open_code_tag(&mut buffer)?;
        self.highlights(&mut buffer)?;
        self.closing_tags(&mut buffer)?;
        write!(output, "{}", &String::from_utf8(buffer).unwrap())?;
        Ok(())
    }
}

impl HtmlFormatter for HtmlLinked<'_> {
    fn open_pre_tag(&self, output: &mut dyn Write) -> io::Result<()> {
        let class = if let Some(pre_class) = self.pre_class {
            format!("athl {}", pre_class)
        } else {
            "athl".to_string()
        };

        write!(output, "<pre class=\"{}\">", class)
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

    #[test]
    fn test_include_pre_class() {
        let formatter = HtmlLinked::new("", Language::PlainText, Some("test-pre-class"), None);
        let mut buffer = Vec::new();
        formatter.open_pre_tag(&mut buffer);
        let pre_tag = String::from_utf8(buffer).unwrap();
        assert!(pre_tag.contains("<pre class=\"athl test-pre-class\">"));
    }

    #[test]
    fn test_code_tag_with_language() {
        let formatter = HtmlLinked::new("", Language::Rust, None, None);
        let mut buffer = Vec::new();
        formatter.open_code_tag(&mut buffer);
        let code_tag = String::from_utf8(buffer).unwrap();
        assert!(code_tag.contains("<code class=\"language-rust\" translate=\"no\" tabindex=\"0\">"));
    }

    #[test]
    fn test_builder_pattern() {
        let formatter = HtmlLinked::default()
            .with_lang(Language::Rust)
            .with_pre_class(Some("test-pre-class"));

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
        let formatter = HtmlLinked::new(code, Language::PlainText, None, Some(highlight_lines));

        let mut buffer = Vec::new();
        formatter.format(&mut buffer).unwrap();
        let result = String::from_utf8(buffer).unwrap();

        assert!(result.contains("class=\"line highlighted\" data-line=\"1\""));
        assert!(result.contains("class=\"line\" data-line=\"2\""));
        assert!(result.contains("class=\"line highlighted\" data-line=\"3\""));
        assert!(result.contains("class=\"line highlighted\" data-line=\"4\""));
        assert!(result.contains("class=\"line\" data-line=\"5\""));
    }
}

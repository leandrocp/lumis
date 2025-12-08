//! HTML formatter with linked CSS classes.
//!
//! This module provides the [`HtmlLinked`] formatter that generates HTML output with
//! CSS classes for syntax highlighting. Requires external CSS files for styling.
//! Supports line highlighting and custom CSS classes.
//!
//! # Example Output
//!
//! For the Rust code `fn main() { println!("Hello"); }`, the formatter generates
//! HTML with CSS classes like:
//!
//! ```html
//! <pre class="athl"><code class="language-rust" translate="no" tabindex="0"><div class="line" data-line="1"><span class="keyword-function">fn</span> <span class="function">main</span><span class="punctuation-bracket">(</span><span class="punctuation-bracket">)</span> <span class="punctuation-bracket">&lbrace;</span> <span class="keyword-exception">println</span><span class="function-macro">!</span><span class="punctuation-bracket">(</span><span class="string">&quot;Hello&quot;</span><span class="punctuation-bracket">)</span><span class="punctuation-delimiter">;</span> <span class="punctuation-bracket">&rbrace;</span></div></code></pre>
//! ```
//!
//! See the [formatter](crate::formatter) module for more information and examples.

#![allow(unused_must_use)]

use super::{Formatter, HtmlElement};
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
/// With default "highlighted" class:
/// ```rust
/// use autumnus::formatter::html_linked::HighlightLines;
///
/// let highlight_lines = HighlightLines {
///     lines: vec![1..=1, 5..=7],
///     ..Default::default()
/// };
/// ```
/// The resulting HTML will look like:
/// ```html
/// <div class="line highlighted" data-line="2">...</div>
/// ```
///
/// Using a custom CSS class:
/// ```rust
/// use autumnus::formatter::html_linked::HighlightLines;
///
/// let highlight_lines = HighlightLines {
///     lines: vec![2..=3],
///     class: "transition-colors duration-500 w-full inline-block bg-yellow-500".to_string(),
/// };
/// ```
///
/// The resulting HTML will include the classes in line elements:
/// ```html
/// <div class="line transition-colors duration-500 w-full inline-block bg-yellow-500" data-line="2">...</div>
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
    /// Highlighted lines will have both "highlighted" and this custom class added to the existing "line" class,
    /// resulting in elements like `<div class="line highlighted your-class-name" data-line="N">`.
    /// You can then style these classes in your CSS to achieve the desired highlighting effect.
    ///
    /// Note that themes include a `highlighted` class for convenience,
    /// which contains the colors from the theme's "CursorLine" highlight from Neovim.
    ///
    /// Defaults to `"highlighted"`.
    /// ```rust
    /// use autumnus::formatter::html_linked::HighlightLines;
    ///
    /// let highlight_lines = HighlightLines {
    ///     lines: vec![1..=2],
    ///     class: "highlighted".to_string(),
    /// };
    /// ```
    pub class: String,
}

impl Default for HighlightLines {
    fn default() -> Self {
        Self {
            lines: Vec::new(),
            class: "highlighted".to_string(),
        }
    }
}

/// HTML formatter with CSS classes.
///
/// Generates HTML with CSS classes instead of inline styles. Requires external CSS files.
/// Use this for better performance and smaller HTML when serving multiple code blocks.
/// Use [`HtmlLinkedBuilder`] to create instances.
///
/// # When to use
///
/// - Multiple code blocks sharing the same theme (smaller HTML output)
/// - Need to customize or override styles with CSS
/// - Prefer separation of content and styling
///
/// # Example
///
/// ```rust
/// use autumnus::{HtmlLinkedBuilder, languages::Language, formatter::Formatter};
/// use std::io::Write;
///
/// let code = "print('Hello')";
///
/// let formatter = HtmlLinkedBuilder::new()
///     .lang(Language::Python)
///     .pre_class(Some("my-code"))
///     .build()
///     .unwrap();
///
/// let mut output = Vec::new();
/// formatter.format(code, &mut output).unwrap();
/// // Remember to include the corresponding CSS file for your theme
/// ```
#[derive(Builder, Debug)]
#[builder(default)]
pub struct HtmlLinked<'a> {
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
        lang: Language,
        pre_class: Option<&'a str>,
        highlight_lines: Option<HighlightLines>,
        header: Option<HtmlElement>,
    ) -> Self {
        Self {
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
            lang: Language::PlainText,
            pre_class: None,
            highlight_lines: None,
            header: None,
        }
    }
}

impl Formatter for HtmlLinked<'_> {
    fn format(&self, source: &str, output: &mut dyn Write) -> io::Result<()> {
        let mut buffer = Vec::new();

        if let Some(ref header) = self.header {
            write!(buffer, "{}", header.open_tag)?;
        }

        crate::formatter::html::open_pre_tag(&mut buffer, self.pre_class, None)?;
        crate::formatter::html::open_code_tag(&mut buffer, &self.lang)?;

        let mut highlighter = Highlighter::new();
        let events = highlighter
            .highlight(self.lang.config(), source.as_bytes(), None, |injected| {
                Some(Language::guess(Some(injected), "").config())
            })
            .map_err(io::Error::other)?;

        let mut renderer = tree_sitter_highlight::HtmlRenderer::new();

        renderer
            .render(events, source.as_bytes(), &move |highlight, output| {
                let class = CLASSES[highlight.0];

                output.extend(b"class=\"");
                output.extend(class.as_bytes());
                output.extend(b"\"");
            })
            .map_err(io::Error::other)?;

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

            let line_with_braces = line.replace('{', "&lbrace;").replace('}', "&rbrace;");

            write!(
                &mut buffer,
                "<div class=\"line{}\" data-line=\"{}\">{}</div>",
                highlighted_class, line_number, line_with_braces
            );
        }

        crate::formatter::html::closing_tags(&mut buffer)?;

        if let Some(ref header) = self.header {
            write!(buffer, "{}", header.close_tag)?;
        }

        write!(output, "{}", &String::from_utf8_lossy(&buffer))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formatter::HtmlLinkedBuilder;

    #[cfg(test)]
    use pretty_assertions::assert_str_eq;

    #[test]
    fn test_no_attrs() {
        let code = "@lang :rust";
        let formatter = HtmlLinked::new(Language::Elixir, None, None, None);
        let mut buffer = Vec::new();
        formatter.format(code, &mut buffer);
        let result = String::from_utf8(buffer).unwrap();
        let expected = r#"<pre class="athl"><code class="language-elixir" translate="no" tabindex="0"><div class="line" data-line="1"><span class="operator"><span class="constant">@<span class="function-call"><span class="constant">lang <span class="string-special-symbol">:rust</span></span></span></span></span>
</div></code></pre>"#;
        assert_eq!(result, expected)
    }

    #[test]
    fn test_include_pre_class() {
        let formatter = HtmlLinked::new(Language::PlainText, Some("test-pre-class"), None, None);
        let mut buffer = Vec::new();
        crate::formatter::html::open_pre_tag(&mut buffer, formatter.pre_class, None).unwrap();
        let result = String::from_utf8(buffer).unwrap();
        let expected = r#"<pre class="athl test-pre-class">"#;
        assert_str_eq!(result, expected);
    }

    #[test]
    fn test_code_tag_with_language() {
        let formatter = HtmlLinked::new(Language::Rust, None, None, None);
        let mut buffer = Vec::new();
        crate::formatter::html::open_code_tag(&mut buffer, &formatter.lang).unwrap();
        let result = String::from_utf8(buffer).unwrap();
        let expected = r#"<code class="language-rust" translate="no" tabindex="0">"#;
        assert_str_eq!(result, expected);
    }

    #[test]
    fn test_builder_pattern() {
        let formatter = HtmlLinkedBuilder::new()
            .lang(Language::Rust)
            .pre_class(Some("test-pre-class"))
            .build()
            .unwrap();

        let mut buffer = Vec::new();
        crate::formatter::html::open_pre_tag(&mut buffer, formatter.pre_class, None).unwrap();
        let pre_result = String::from_utf8(buffer).unwrap();
        let pre_expected = r#"<pre class="athl test-pre-class">"#;
        assert_str_eq!(pre_result, pre_expected);

        let mut buffer = Vec::new();
        crate::formatter::html::open_code_tag(&mut buffer, &formatter.lang).unwrap();
        let code_result = String::from_utf8(buffer).unwrap();
        let code_expected = r#"<code class="language-rust" translate="no" tabindex="0">"#;
        assert_str_eq!(code_result, code_expected);
    }

    #[test]
    fn test_default_highlight_lines() {
        let code = "line 1\nline 2\nline 3";
        let highlight_lines = HighlightLines {
            lines: vec![2..=2],
            ..Default::default()
        };

        let formatter = HtmlLinked::new(Language::PlainText, None, Some(highlight_lines), None);

        let mut buffer = Vec::new();
        formatter.format(code, &mut buffer).unwrap();
        let result = String::from_utf8(buffer).unwrap();

        let expected = r#"<pre class="athl"><code class="language-plaintext" translate="no" tabindex="0"><div class="line" data-line="1">line 1
</div><div class="line highlighted" data-line="2">line 2
</div><div class="line" data-line="3">line 3
</div></code></pre>"#;
        assert_str_eq!(result, expected);
    }

    #[test]
    fn test_highlight_lines() {
        let code = "line 1\nline 2\nline 3\nline 4\nline 5";
        let highlight_lines = HighlightLines {
            lines: vec![1..=1, 3..=4],
            class: "custom-hl".to_string(),
        };
        let formatter = HtmlLinked::new(Language::PlainText, None, Some(highlight_lines), None);

        let mut buffer = Vec::new();
        formatter.format(code, &mut buffer).unwrap();
        let result = String::from_utf8(buffer).unwrap();

        let expected = r#"<pre class="athl"><code class="language-plaintext" translate="no" tabindex="0"><div class="line custom-hl" data-line="1">line 1
</div><div class="line" data-line="2">line 2
</div><div class="line custom-hl" data-line="3">line 3
</div><div class="line custom-hl" data-line="4">line 4
</div><div class="line" data-line="5">line 5
</div></code></pre>"#;
        assert_str_eq!(result, expected);
    }

    #[test]
    fn test_header_wrapping() {
        let header = HtmlElement {
            open_tag: "<div class=\"code-wrapper\">".to_string(),
            close_tag: "</div>".to_string(),
        };
        let code = "line 1\nline 2";
        let formatter = HtmlLinked::new(Language::PlainText, None, None, Some(header));

        let mut buffer = Vec::new();
        formatter.format(code, &mut buffer).unwrap();
        let result = String::from_utf8(buffer).unwrap();

        let expected = r#"<div class="code-wrapper"><pre class="athl"><code class="language-plaintext" translate="no" tabindex="0"><div class="line" data-line="1">line 1
</div><div class="line" data-line="2">line 2
</div></code></pre></div>"#;
        assert_str_eq!(result, expected);
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
            Language::PlainText,
            Some("custom-pre"),
            Some(highlight_lines),
            Some(header),
        );

        let mut buffer = Vec::new();
        formatter.format(code, &mut buffer).unwrap();
        let result = String::from_utf8(buffer).unwrap();

        let expected = r#"<section class="code-section"><pre class="athl custom-pre"><code class="language-plaintext" translate="no" tabindex="0"><div class="line highlighted" data-line="1">line 1
</div><div class="line" data-line="2">line 2
</div></code></pre></section>"#;
        assert_str_eq!(result, expected);
    }
}

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
//! <pre class="lumis"><code class="language-rust" translate="no" tabindex="0"><div class="l-line" data-line="1"><span class="l-keyword-function">fn</span> <span class="l-function">main</span><span class="l-punctuation-bracket">(</span><span class="l-punctuation-bracket">)</span> <span class="l-punctuation-bracket">{</span> <span class="l-keyword-exception">println</span><span class="l-function-macro">!</span><span class="l-punctuation-bracket">(</span><span class="l-string">&quot;Hello&quot;</span><span class="l-punctuation-bracket">)</span><span class="l-punctuation-delimiter">;</span> <span class="l-punctuation-bracket">}</span></div></code></pre>
//! ```
//!
//! See the [formatter](crate::formatter) module for more information and examples.

use super::{Formatter, HtmlElement};
use crate::languages::Language;
use derive_builder::Builder;
use lumis_core::formatter::Formatter as _;
use std::{
    io::{self, Write},
    ops::RangeInclusive,
};

/// Configuration for highlighting specific lines in HTML linked output.
///
/// This struct allows you to specify which lines should be highlighted using
/// CSS classes. The highlighting is applied by adding the specified class
/// to the line elements, allowing for flexible styling via external CSS.
///
/// # Examples
///
/// With default "l-highlighted" class:
/// ```rust
/// use lumis::formatters::html_linked::HighlightLines;
///
/// let highlight_lines = HighlightLines {
///     lines: vec![1..=1, 5..=7],
///     ..Default::default()
/// };
/// ```
/// The resulting HTML will look like:
/// ```html
/// <div class="l-line l-highlighted" data-line="2">...</div>
/// ```
///
/// Using a custom CSS class:
/// ```rust
/// use lumis::formatters::html_linked::HighlightLines;
///
/// let highlight_lines = HighlightLines {
///     lines: vec![2..=3],
///     class: "transition-colors duration-500 w-full inline-block bg-yellow-500".to_string(),
/// };
/// ```
///
/// The resulting HTML will include the classes in line elements:
/// ```html
/// <div class="l-line transition-colors duration-500 w-full inline-block bg-yellow-500" data-line="2">...</div>
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
    /// Highlighted lines will have both "l-highlighted" and this custom class added to the existing "l-line" class,
    /// resulting in elements like `<div class="l-line l-highlighted your-class-name" data-line="N">`.
    /// You can then style these classes in your CSS to achieve the desired highlighting effect.
    ///
    /// Note that themes include a `l-highlighted` class for convenience,
    /// which contains the colors from the theme's "`CursorLine`" highlight from Neovim.
    ///
    /// Defaults to `"l-highlighted"`.
    /// ```rust
    /// use lumis::formatters::html_linked::HighlightLines;
    ///
    /// let highlight_lines = HighlightLines {
    ///     lines: vec![1..=2],
    ///     class: "l-highlighted".to_string(),
    /// };
    /// ```
    pub class: String,
}

impl Default for HighlightLines {
    fn default() -> Self {
        Self {
            lines: Vec::new(),
            class: "l-highlighted".to_string(),
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
/// use lumis::{HtmlLinkedBuilder, languages::Language, formatters::Formatter};
/// use std::io::Write;
///
/// let code = "print('Hello')";
///
/// let formatter = HtmlLinkedBuilder::new()
///     .language(Language::Python)
///     .pre_class(Some("my-code".to_string()))
///     .build()
///     .unwrap();
///
/// let mut output = Vec::new();
/// lumis::write_highlight(&mut output, code, formatter).unwrap();
/// // Remember to include the corresponding CSS file for your theme
/// ```
#[derive(Builder, Clone, Debug)]
#[builder(default)]
pub struct HtmlLinked {
    #[builder(setter(custom))]
    language: Language,
    pre_class: Option<String>,
    highlight_lines: Option<HighlightLines>,
    header: Option<HtmlElement>,
}

impl HtmlLinkedBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn language(&mut self, language: Language) -> &mut Self {
        self.language = Some(language);
        self
    }

    #[deprecated(note = "use `.language(...)` instead")]
    pub fn lang(&mut self, language: Language) -> &mut Self {
        self.language(language)
    }
}

impl HtmlLinked {
    pub fn new(
        language: Language,
        pre_class: Option<String>,
        highlight_lines: Option<HighlightLines>,
        header: Option<HtmlElement>,
    ) -> Self {
        Self {
            language,
            pre_class,
            highlight_lines,
            header,
        }
    }
}

impl Default for HtmlLinked {
    fn default() -> Self {
        Self {
            language: Language::PlainText,
            pre_class: None,
            highlight_lines: None,
            header: None,
        }
    }
}

impl<T> Formatter<T> for HtmlLinked {
    fn language(&self) -> Language {
        self.language
    }

    fn render(
        &self,
        source: &str,
        events: &[lumis_core::events::HighlightEvent<'_, T>],
        output: &mut dyn Write,
    ) -> io::Result<()> {
        let core_formatter = lumis_core::formatter::html_linked::HtmlLinked::new(
            self.language,
            self.pre_class.clone(),
            self.highlight_lines.clone().map(map_highlight_lines),
            self.header.clone(),
        );

        core_formatter.render(source, events, output)
    }
}

fn map_highlight_lines(
    highlight_lines: HighlightLines,
) -> lumis_core::formatter::html_linked::HighlightLines {
    lumis_core::formatter::html_linked::HighlightLines {
        lines: highlight_lines.lines,
        class: highlight_lines.class,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formatters::HtmlLinkedBuilder;

    #[cfg(test)]
    use pretty_assertions::assert_str_eq;

    #[test]
    fn test_no_attrs() {
        let code = "@lang :rust";
        let formatter = HtmlLinked::new(Language::Elixir, None, None, None);
        let mut buffer = Vec::new();
        crate::write_highlight(&mut buffer, code, formatter).unwrap();
        let result = String::from_utf8(buffer).unwrap();
        let expected = r#"<pre class="lumis"><code class="language-elixir" translate="no" tabindex="0"><div class="l-line" data-line="1"><span class="l-operator"><span class="l-constant">@<span class="l-function-call"><span class="l-constant">lang <span class="l-string-special-symbol">:rust</span></span></span></span></span>
</div></code></pre>"#;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_include_pre_class() {
        let formatter = HtmlLinked::new(
            Language::PlainText,
            Some("test-pre-class".to_string()),
            None,
            None,
        );
        let mut buffer = Vec::new();
        crate::formatters::html::open_pre_tag(&mut buffer, formatter.pre_class.as_deref(), None)
            .unwrap();
        let result = String::from_utf8(buffer).unwrap();
        let expected = r#"<pre class="lumis test-pre-class">"#;
        assert_str_eq!(result, expected);
    }

    #[test]
    fn test_code_tag_with_language() {
        let formatter = HtmlLinked::new(Language::Rust, None, None, None);
        let mut buffer = Vec::new();
        crate::formatters::html::open_code_tag(&mut buffer, &formatter.language).unwrap();
        let result = String::from_utf8(buffer).unwrap();
        let expected = r#"<code class="language-rust" translate="no" tabindex="0">"#;
        assert_str_eq!(result, expected);
    }

    #[test]
    fn test_builder_pattern() {
        let formatter = HtmlLinkedBuilder::new()
            .language(Language::Rust)
            .pre_class(Some("test-pre-class".to_string()))
            .build()
            .unwrap();

        let mut buffer = Vec::new();
        crate::formatters::html::open_pre_tag(&mut buffer, formatter.pre_class.as_deref(), None)
            .unwrap();
        let pre_result = String::from_utf8(buffer).unwrap();
        let pre_expected = r#"<pre class="lumis test-pre-class">"#;
        assert_str_eq!(pre_result, pre_expected);

        let mut buffer = Vec::new();
        crate::formatters::html::open_code_tag(&mut buffer, &formatter.language).unwrap();
        let code_result = String::from_utf8(buffer).unwrap();
        let code_expected = r#"<code class="language-rust" translate="no" tabindex="0">"#;
        assert_str_eq!(code_result, code_expected);
    }

    #[test]
    #[allow(clippy::single_range_in_vec_init)]
    fn test_default_highlight_lines() {
        let code = "line 1\nline 2\nline 3";
        let highlight_lines = HighlightLines {
            lines: vec![2..=2],
            ..Default::default()
        };

        let formatter = HtmlLinked::new(Language::PlainText, None, Some(highlight_lines), None);

        let mut buffer = Vec::new();
        crate::write_highlight(&mut buffer, code, formatter).unwrap();
        let result = String::from_utf8(buffer).unwrap();

        let expected = r#"<pre class="lumis"><code class="language-plaintext" translate="no" tabindex="0"><div class="l-line" data-line="1">line 1
</div><div class="l-line l-highlighted" data-line="2">line 2
</div><div class="l-line" data-line="3">line 3
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
        crate::write_highlight(&mut buffer, code, formatter).unwrap();
        let result = String::from_utf8(buffer).unwrap();

        let expected = r#"<pre class="lumis"><code class="language-plaintext" translate="no" tabindex="0"><div class="l-line custom-hl" data-line="1">line 1
</div><div class="l-line" data-line="2">line 2
</div><div class="l-line custom-hl" data-line="3">line 3
</div><div class="l-line custom-hl" data-line="4">line 4
</div><div class="l-line" data-line="5">line 5
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
        crate::write_highlight(&mut buffer, code, formatter).unwrap();
        let result = String::from_utf8(buffer).unwrap();

        let expected = r#"<div class="code-wrapper"><pre class="lumis"><code class="language-plaintext" translate="no" tabindex="0"><div class="l-line" data-line="1">line 1
</div><div class="l-line" data-line="2">line 2
</div></code></pre></div>"#;
        assert_str_eq!(result, expected);
    }

    #[test]
    #[allow(clippy::single_range_in_vec_init)]
    fn test_header_with_highlight_lines() {
        let header = HtmlElement {
            open_tag: "<section class=\"code-section\">".to_string(),
            close_tag: "</section>".to_string(),
        };
        let highlight_lines = HighlightLines {
            lines: vec![1..=1],
            class: "l-highlighted".to_string(),
        };
        let code = "line 1\nline 2";
        let formatter = HtmlLinked::new(
            Language::PlainText,
            Some("custom-pre".to_string()),
            Some(highlight_lines),
            Some(header),
        );

        let mut buffer = Vec::new();
        crate::write_highlight(&mut buffer, code, formatter).unwrap();
        let result = String::from_utf8(buffer).unwrap();

        let expected = r#"<section class="code-section"><pre class="lumis custom-pre"><code class="language-plaintext" translate="no" tabindex="0"><div class="l-line l-highlighted" data-line="1">line 1
</div><div class="l-line" data-line="2">line 2
</div></code></pre></section>"#;
        assert_str_eq!(result, expected);
    }
}

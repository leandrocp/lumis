//! HTML formatter with linked CSS classes.
//!
//! This module re-exports the [`HtmlLinked`] formatter that generates HTML output with
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
//! # Example
//!
//! ```rust
//! use lumis::{HtmlLinkedBuilder, languages::Language};
//!
//! let code = "print('Hello')";
//!
//! let formatter = HtmlLinkedBuilder::new()
//!     .language(Language::Python)
//!     .pre_class(Some("my-code".to_string()))
//!     .build()
//!     .unwrap();
//!
//! let mut output = Vec::new();
//! lumis::write_highlight(&mut output, code, formatter).unwrap();
//! // Remember to include the corresponding CSS file for your theme
//! ```
//!
//! # Line Highlighting
//!
//! [`HighlightLines`] picks the lines to highlight and names the CSS class applied to
//! them. Highlighted lines carry both `l-line` and that class, so styling them is a
//! matter of writing CSS. Themes ship an `l-highlighted` class for convenience, built
//! from the theme's `CursorLine` highlight in Neovim.
//!
//! With the default `l-highlighted` class:
//! ```rust
//! use lumis::formatters::html_linked::HighlightLines;
//!
//! let highlight_lines = HighlightLines {
//!     lines: vec![1..=1, 5..=7],
//!     ..Default::default()
//! };
//! ```
//! The resulting HTML will look like:
//! ```html
//! <div class="l-line l-highlighted" data-line="2">...</div>
//! ```
//!
//! Using a custom CSS class:
//! ```rust
//! use lumis::formatters::html_linked::HighlightLines;
//!
//! let highlight_lines = HighlightLines {
//!     lines: vec![2..=3],
//!     class: "transition-colors duration-500 w-full inline-block bg-yellow-500".to_string(),
//! };
//! ```
//!
//! The resulting HTML will include the classes in line elements:
//! ```html
//! <div class="l-line transition-colors duration-500 w-full inline-block bg-yellow-500" data-line="2">...</div>
//! ```
//!
//! See the [formatter](crate::formatter) module for more information and examples.

pub use lumis_core::formatter::html_linked::{
    HighlightLines, HtmlLinked, HtmlLinkedBuilder, HtmlLinkedBuilderError,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formatters::html::HtmlElement;
    use crate::languages::Language;
    use pretty_assertions::assert_str_eq;

    #[test]
    fn test_no_attrs() {
        let code = "@lang :rust";
        let formatter = HtmlLinked::new(Language::Elixir, None, None, None);
        let result = crate::highlight(code, formatter);
        let expected = r#"<pre class="lumis"><code class="language-elixir" translate="no" tabindex="0"><div class="l-line" data-line="1"><span class="l-operator"><span class="l-constant">@<span class="l-function-call"><span class="l-constant">lang <span class="l-string-special-symbol">:rust</span></span></span></span></span>
</div></code></pre>"#;
        assert_str_eq!(result, expected);
    }

    #[test]
    fn test_include_pre_class() {
        let formatter = HtmlLinked::new(
            Language::PlainText,
            Some("test-pre-class".to_string()),
            None,
            None,
        );
        let result = crate::highlight("", formatter);
        assert!(
            result.starts_with(r#"<pre class="lumis test-pre-class">"#),
            "{result}"
        );
    }

    #[test]
    fn test_code_tag_with_language() {
        let formatter = HtmlLinked::new(Language::Rust, None, None, None);
        let result = crate::highlight("", formatter);
        assert!(
            result.contains(r#"<code class="language-rust" translate="no" tabindex="0">"#),
            "{result}"
        );
    }

    #[test]
    fn test_builder_pattern() {
        let formatter = HtmlLinkedBuilder::new()
            .language(Language::Rust)
            .pre_class(Some("test-pre-class".to_string()))
            .build()
            .unwrap();

        let result = crate::highlight("", formatter);
        assert!(
            result.starts_with(r#"<pre class="lumis test-pre-class">"#),
            "{result}"
        );
        assert!(
            result.contains(r#"<code class="language-rust" translate="no" tabindex="0">"#),
            "{result}"
        );
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

        let result = crate::highlight(code, formatter);

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

        let result = crate::highlight(code, formatter);

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

        let result = crate::highlight(code, formatter);

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

        let result = crate::highlight(code, formatter);

        let expected = r#"<section class="code-section"><pre class="lumis custom-pre"><code class="language-plaintext" translate="no" tabindex="0"><div class="l-line l-highlighted" data-line="1">line 1
</div><div class="l-line" data-line="2">line 2
</div></code></pre></section>"#;
        assert_str_eq!(result, expected);
    }
}

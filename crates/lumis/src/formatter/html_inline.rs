//! HTML formatter with inline CSS styles.
//!
//! This module re-exports the [`HtmlInline`] formatter that generates HTML output with
//! inline CSS styles for syntax highlighting. It supports themes, line highlighting,
//! and various customization options.
//!
//! # Example Output
//!
//! For the Rust code `fn main() { println!("Hello"); }` with the dracula theme,
//! the formatter generates self-contained HTML like:
//!
//! ```html
//! <pre class="lumis" style="color: #f8f8f2; background-color: #282a36;"><code class="language-rust" translate="no" tabindex="0"><div class="l-line" data-line="1"><span style="color: #8be9fd;">fn</span> <span style="color: #50fa7b;">main</span><span style="color: #f8f8f2;">(</span><span style="color: #f8f8f2;">)</span> <span style="color: #f8f8f2;">{</span> <span style="color: #bd93f9;">println</span><span style="color: #50fa7b;">!</span><span style="color: #f8f8f2;">(</span><span style="color: #f1fa8c;">&quot;Hello&quot;</span><span style="color: #f8f8f2;">)</span><span style="color: #f8f8f2;">;</span> <span style="color: #f8f8f2;">}</span></div></code></pre>
//! ```
//!
//! # Example
//!
//! ```rust
//! use lumis::{HtmlInlineBuilder, languages::Language, themes};
//!
//! let code = "const x = 42;";
//! let theme = themes::get("github_dark").unwrap();
//!
//! let formatter = HtmlInlineBuilder::new()
//!     .language(Language::JavaScript)
//!     .theme(Some(theme))
//!     .pre_class(Some("code-block".to_string()))
//!     .build()
//!     .unwrap();
//!
//! let mut output = Vec::new();
//! lumis::write_highlight(&mut output, code, formatter).unwrap();
//! ```
//!
//! # Line Highlighting
//!
//! [`HighlightLines`] picks the lines to highlight and [`HighlightLinesStyle`] decides
//! how they are styled.
//!
//! Using theme-based highlighting (requires a theme with a `highlighted` style):
//! ```rust
//! use lumis::formatters::html_inline::{HighlightLines, HighlightLinesStyle};
//!
//! let highlight_lines = HighlightLines {
//!     lines: vec![1..=1, 5..=7],
//!     style: Some(HighlightLinesStyle::Theme),
//!     class: None,
//! };
//! ```
//!
//! The resulting HTML will include the theme style for highlighted lines:
//! ```html
//! <div class="l-line" style="background-color: #dae9f9;" data-line="1">fn main() {</div>
//! ```
//!
//! Using both style and class:
//! ```rust
//! use lumis::formatters::html_inline::{HighlightLines, HighlightLinesStyle};
//!
//! let highlight_lines = HighlightLines {
//!     lines: vec![2..=3],
//!     style: Some(HighlightLinesStyle::Theme),
//!     class: Some("w-full inline-block bg-yellow-500".to_string()),
//! };
//! ```
//!
//! The resulting HTML will look like:
//! ```html
//! <div class="l-line w-full inline-block bg-yellow-500" style="background-color: #dae9f9;" data-line="3">    let x = 42;</div>
//! ```
//!
//! Or replace the theme style with CSS of your own:
//! ```rust
//! use lumis::formatters::html_inline::{HighlightLines, HighlightLinesStyle};
//!
//! let highlight_lines = HighlightLines {
//!     lines: vec![2..=3],
//!     style: Some(HighlightLinesStyle::Style("background-color: yellow; border-left: 3px solid red".to_string())),
//!     class: None,
//! };
//! ```
//!
//! See the [formatter](crate::formatter) module for more information and examples.

pub use lumis_core::formatter::html_inline::{
    HighlightLines, HighlightLinesStyle, HtmlInline, HtmlInlineBuilder, HtmlInlineBuilderError,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formatters::html::HtmlElement;
    use crate::languages::Language;
    use crate::themes;
    use pretty_assertions::assert_str_eq;

    #[test]
    fn test_no_attrs() {
        let code = "@lang :rust";
        let formatter = HtmlInline::new(Language::Elixir, None, None, false, false, None, None);
        let result = crate::highlight(code, formatter);
        let expected = r#"<pre class="lumis"><code class="language-elixir" translate="no" tabindex="0"><div class="l-line" data-line="1"><span><span>@<span><span>lang <span>:rust</span></span></span></span></span>
</div></code></pre>"#;
        assert_str_eq!(result, expected);
    }

    #[test]
    fn test_unstyled_scope_opens_a_bare_span() {
        let theme = themes::get("dracula").unwrap();
        let code = "# *italic*";
        let formatter = HtmlInline::new(
            Language::Markdown,
            Some(theme),
            None,
            false,
            false,
            None,
            None,
        );
        let result = crate::highlight(code, formatter);

        assert!(
            result.contains("<span>*italic*</span>"),
            "a scope dracula does not define must open a bare `<span>`: {result}"
        );
        assert!(
            !result.contains("<span >"),
            "an attribute-less span must not carry a trailing space: {result}"
        );
    }

    #[test]
    fn test_do_not_append_pre_style_if_missing_theme_style() {
        let formatter = HtmlInline::default();
        let result = crate::highlight("", formatter);
        assert!(result.starts_with(r#"<pre class="lumis">"#), "{result}");
    }

    #[test]
    fn test_include_pre_class() {
        let formatter = HtmlInline::new(
            Language::PlainText,
            None,
            Some("test-pre-class".to_string()),
            false,
            false,
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
    fn test_include_pre_class_with_theme() {
        let theme = themes::get("github_light").unwrap();
        let formatter = HtmlInline::new(
            Language::PlainText,
            Some(theme),
            Some("test-pre-class".to_string()),
            false,
            false,
            None,
            None,
        );
        let result = crate::highlight("", formatter);
        assert!(
            result.starts_with(
                r#"<pre class="lumis test-pre-class" style="color: #1f2328; background-color: #ffffff;">"#
            ),
            "{result}"
        );
    }

    #[test]
    fn test_builder_pattern() {
        let theme = themes::get("github_light").unwrap();
        let formatter = HtmlInlineBuilder::new()
            .language(Language::Rust)
            .theme(Some(theme))
            .pre_class(Some("test-pre-class".to_string()))
            .italic(true)
            .include_highlights(true)
            .build()
            .unwrap();

        let result = crate::highlight("fn main() {}", formatter);
        assert!(
            result.starts_with(
                r#"<pre class="lumis test-pre-class" style="color: #1f2328; background-color: #ffffff;">"#
            ),
            "{result}"
        );
        assert!(
            result.contains(r#"<code class="language-rust""#),
            "{result}"
        );
        assert!(result.contains("data-highlight="), "{result}");
    }

    #[test]
    fn test_highlight_lines_with_theme() {
        let theme = themes::get("github_light").unwrap();
        let highlight_lines = HighlightLines {
            lines: vec![1..=1, 3..=4],
            style: Some(HighlightLinesStyle::Theme),
            class: None,
        };
        let code = "line 1\nline 2\nline 3\nline 4\nline 5";
        let formatter = HtmlInline::new(
            Language::PlainText,
            Some(theme),
            None,
            false,
            false,
            Some(highlight_lines),
            None,
        );

        let result = crate::highlight(code, formatter);

        let expected = r#"<pre class="lumis" style="color: #1f2328; background-color: #ffffff;"><code class="language-plaintext" translate="no" tabindex="0"><div class="l-line" style="background-color: #e7eaf0;" data-line="1">line 1
</div><div class="l-line" data-line="2">line 2
</div><div class="l-line" style="background-color: #e7eaf0;" data-line="3">line 3
</div><div class="l-line" style="background-color: #e7eaf0;" data-line="4">line 4
</div><div class="l-line" data-line="5">line 5
</div></code></pre>"#;
        assert_str_eq!(result, expected);
    }

    #[test]
    fn test_highlight_lines_with_custom_style() {
        let highlight_lines = HighlightLines {
            lines: vec![1..=1, 3..=4],
            style: Some(HighlightLinesStyle::Style(
                "background-color: yellow".to_string(),
            )),
            class: None,
        };
        let code = "line 1\nline 2\nline 3\nline 4\nline 5";
        let formatter = HtmlInline::new(
            Language::PlainText,
            None,
            None,
            false,
            false,
            Some(highlight_lines),
            None,
        );

        let result = crate::highlight(code, formatter);

        let expected = r#"<pre class="lumis"><code class="language-plaintext" translate="no" tabindex="0"><div class="l-line" style="background-color: yellow" data-line="1">line 1
</div><div class="l-line" data-line="2">line 2
</div><div class="l-line" style="background-color: yellow" data-line="3">line 3
</div><div class="l-line" style="background-color: yellow" data-line="4">line 4
</div><div class="l-line" data-line="5">line 5
</div></code></pre>"#;
        assert_str_eq!(result, expected);
    }

    #[test]
    fn test_highlight_lines_with_custom_class() {
        let highlight_lines = HighlightLines {
            lines: vec![1..=1, 3..=3],
            style: Some(HighlightLinesStyle::Style(
                "background-color: yellow".to_string(),
            )),
            class: Some("custom-highlight".to_string()),
        };
        let code = "line 1\nline 2\nline 3\nline 4";
        let formatter = HtmlInline::new(
            Language::PlainText,
            None,
            None,
            false,
            false,
            Some(highlight_lines),
            None,
        );

        let result = crate::highlight(code, formatter);

        let expected = r#"<pre class="lumis"><code class="language-plaintext" translate="no" tabindex="0"><div class="l-line custom-highlight" style="background-color: yellow" data-line="1">line 1
</div><div class="l-line" data-line="2">line 2
</div><div class="l-line custom-highlight" style="background-color: yellow" data-line="3">line 3
</div><div class="l-line" data-line="4">line 4
</div></code></pre>"#;
        assert_str_eq!(result, expected);
    }

    #[test]
    fn test_highlight_lines_with_custom_class_and_no_style() {
        let highlight_lines = HighlightLines {
            lines: vec![1..=1, 3..=3],
            style: None,
            class: Some("custom-highlight".to_string()),
        };
        let code = "fn main() {\n    println!(\"Hello, world!\");\n    let x = 42;\n}";
        let formatter = HtmlInline::new(
            Language::Rust,
            None,
            None,
            false,
            false,
            Some(highlight_lines),
            None,
        );

        let result = crate::highlight(code, formatter);

        let expected = r#"<pre class="lumis"><code class="language-rust" translate="no" tabindex="0"><div class="l-line custom-highlight" data-line="1"><span>fn</span> <span>main</span><span>(</span><span>)</span> <span>{</span>
</div><div class="l-line" data-line="2">    <span>println</span><span>!</span><span>(</span><span>&quot;Hello, world!&quot;</span><span>)</span><span>;</span>
</div><div class="l-line custom-highlight" data-line="3">    <span>let</span> <span>x</span> <span>=</span> <span>42</span><span>;</span>
</div><div class="l-line" data-line="4"><span>}</span>
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
        let formatter = HtmlInline::new(
            Language::PlainText,
            None,
            None,
            false,
            false,
            None,
            Some(header),
        );

        let result = crate::highlight(code, formatter);

        let expected = r#"<div class="code-wrapper"><pre class="lumis"><code class="language-plaintext" translate="no" tabindex="0"><div class="l-line" data-line="1">line 1
</div><div class="l-line" data-line="2">line 2
</div></code></pre></div>"#;
        assert_str_eq!(result, expected);
    }

    #[test]
    fn test_header_with_complex_structure() {
        let header = HtmlElement {
            open_tag: "<section class=\"highlight\" data-lang=\"rust\">".to_string(),
            close_tag: "</section>".to_string(),
        };
        let code = "fn main() { }";
        let formatter = HtmlInline::new(
            Language::Rust,
            None,
            Some("custom-class".to_string()),
            false,
            false,
            None,
            Some(header),
        );

        let result = crate::highlight(code, formatter);

        let expected = r#"<section class="highlight" data-lang="rust"><pre class="lumis custom-class"><code class="language-rust" translate="no" tabindex="0"><div class="l-line" data-line="1"><span>fn</span> <span>main</span><span>(</span><span>)</span> <span>{</span> <span>}</span>
</div></code></pre></section>"#;
        assert_str_eq!(result, expected);
    }
}

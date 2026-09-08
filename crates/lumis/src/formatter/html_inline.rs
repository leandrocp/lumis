//! HTML formatter with inline CSS styles.
//!
//! This module provides the [`HtmlInline`] formatter that generates HTML output with
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
//! See the [formatter](crate::formatter) module for more information and examples.

use super::{Formatter, HtmlElement};
use crate::languages::Language;
use crate::themes::Theme;
use derive_builder::Builder;
use lumis_core::formatter::Formatter as _;
use std::{
    io::{self, Write},
    ops::RangeInclusive,
};

/// Configuration for highlighting specific lines in HTML inline output.
///
/// This struct allows you to specify which lines should be highlighted and how
/// they should be styled using either theme-based styling or custom CSS.
///
/// # Examples
///
/// Using theme-based highlighting (requires a theme with 'highlighted' style):
/// ```rust
/// use lumis::formatters::html_inline::{HighlightLines, HighlightLinesStyle};
///
/// let highlight_lines = HighlightLines {
///     lines: vec![1..=1, 5..=7],
///     style: Some(HighlightLinesStyle::Theme),
///     class: None,
/// };
/// ```
///
// The resulting HTML will include the theme style for highlighted lines:
/// ```html
/// <div class="l-line" style="background-color: #dae9f9;" data-line="1">fn main() {</div>
/// ```
///
/// Using both style and class:
/// ```rust
/// use lumis::formatters::html_inline::{HighlightLines, HighlightLinesStyle};
///
/// let highlight_lines = HighlightLines {
///     lines: vec![2..=3],
///     style: Some(HighlightLinesStyle::Theme),
///     class: Some("w-full inline-block bg-yellow-500".to_string()),
/// };
/// ```
///
/// The resulting HTML will look like:
/// ```html
/// <div class="l-line w-full inline-block bg-yellow-500" style="background-color: #dae9f9;" data-line="3">    let x = 42;</div>
/// ```
///
/// Or disable either one of them:
/// ```rust
/// use lumis::formatters::html_inline::{HighlightLines, HighlightLinesStyle};
///
/// let highlight_lines = HighlightLines {
///     lines: vec![2..=3],
///     style: Some(HighlightLinesStyle::Style("background-color: yellow; border-left: 3px solid red".to_string())),
///     class: None,
/// };
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HighlightLines {
    /// List of line ranges to highlight.
    ///
    /// Each range is inclusive on both ends. Line numbers are 1-based.
    /// Multiple ranges can overlap and will be merged during rendering.
    pub lines: Vec<RangeInclusive<usize>>,
    /// The styling method to use for highlighted lines.
    pub style: Option<HighlightLinesStyle>,
    /// Optional CSS class to add to highlighted lines.
    pub class: Option<String>,
}

/// Defines how highlighted lines should be styled in HTML inline output.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HighlightLinesStyle {
    /// Use the theme's 'highlighted' style if available.
    ///
    /// This looks for a 'highlighted' style definition in the current theme.
    /// If no theme is provided or the theme doesn't define 'highlighted',
    /// no styling will be applied.
    Theme,
    /// Use a custom CSS style string.
    ///
    /// The provided string will be used directly as the `style` attribute
    /// for highlighted line elements. Should contain valid CSS properties.
    ///
    /// # Example
    /// ```rust
    /// use lumis::formatters::html_inline::HighlightLinesStyle;
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
            style: Some(HighlightLinesStyle::Theme),
            class: None,
        }
    }
}

/// HTML formatter with inline CSS styles.
///
/// Generates self-contained HTML with styles embedded directly in elements.
/// Use this when you need standalone HTML without external stylesheets.
/// Use [`HtmlInlineBuilder`] to create instances.
///
/// # When to use
///
/// - Need standalone HTML with no external dependencies
/// - Embedding code snippets in emails or restricted environments
/// - Quick prototyping without CSS setup
///
/// # Example
///
/// ```rust
/// use lumis::{HtmlInlineBuilder, languages::Language, themes, formatters::Formatter};
/// use std::io::Write;
///
/// let code = "const x = 42;";
/// let theme = themes::get("github_dark").unwrap();
///
/// let formatter = HtmlInlineBuilder::new()
///     .language(Language::JavaScript)
///     .theme(Some(theme))
///     .pre_class(Some("code-block".to_string()))
///     .build()
///     .unwrap();
///
/// let mut output = Vec::new();
/// lumis::write_highlight(&mut output, code, formatter).unwrap();
/// ```
#[derive(Builder, Clone, Debug)]
#[builder(default)]
pub struct HtmlInline {
    #[builder(setter(custom))]
    language: Language,
    theme: Option<Theme>,
    pre_class: Option<String>,
    italic: bool,
    include_highlights: bool,
    highlight_lines: Option<HighlightLines>,
    header: Option<HtmlElement>,
}

impl HtmlInlineBuilder {
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

impl HtmlInline {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        language: Language,
        theme: Option<Theme>,
        pre_class: Option<String>,
        italic: bool,
        include_highlights: bool,
        highlight_lines: Option<HighlightLines>,
        header: Option<HtmlElement>,
    ) -> Self {
        Self {
            language,
            theme,
            pre_class,
            italic,
            include_highlights,
            highlight_lines,
            header,
        }
    }
}

impl Default for HtmlInline {
    fn default() -> Self {
        Self {
            language: Language::PlainText,
            theme: None,
            pre_class: None,
            italic: false,
            include_highlights: false,
            highlight_lines: None,
            header: None,
        }
    }
}

impl<T> Formatter<T> for HtmlInline {
    fn language(&self) -> Language {
        self.language
    }

    fn render(
        &self,
        source: &str,
        events: &[lumis_core::events::HighlightEvent<'_, T>],
        output: &mut dyn Write,
    ) -> io::Result<()> {
        let core_formatter = lumis_core::formatter::html_inline::HtmlInline::new(
            self.language,
            self.theme.clone(),
            self.pre_class.clone(),
            self.italic,
            self.include_highlights,
            self.highlight_lines
                .clone()
                .map(super::map_inline_highlight_lines),
            self.header.clone(),
        );

        core_formatter.render(source, events, output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formatters::HtmlInlineBuilder;
    use crate::themes;

    #[cfg(test)]
    use pretty_assertions::assert_str_eq;

    #[test]
    fn test_no_attrs() {
        let code = "@lang :rust";
        let formatter = HtmlInline::new(Language::Elixir, None, None, false, false, None, None);
        let mut buffer = Vec::new();
        crate::write_highlight(&mut buffer, code, formatter).unwrap();
        let result = String::from_utf8(buffer).unwrap();
        let expected = r#"<pre class="lumis"><code class="language-elixir" translate="no" tabindex="0"><div class="l-line" data-line="1"><span><span>@<span><span>lang <span>:rust</span></span></span></span></span>
</div></code></pre>"#;
        assert_eq!(result, expected);
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
        let mut buffer = Vec::new();
        crate::write_highlight(&mut buffer, code, formatter).unwrap();
        let result = String::from_utf8(buffer).unwrap();

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
        let mut buffer = Vec::new();
        crate::formatters::html::open_pre_tag(
            &mut buffer,
            formatter.pre_class.as_deref(),
            formatter.theme.as_ref(),
        )
        .unwrap();
        let pre_tag = String::from_utf8(buffer).unwrap();
        assert!(pre_tag.contains("<pre class=\"lumis\">"));
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
        let mut buffer = Vec::new();
        crate::formatters::html::open_pre_tag(
            &mut buffer,
            formatter.pre_class.as_deref(),
            formatter.theme.as_ref(),
        )
        .unwrap();
        let pre_tag = String::from_utf8(buffer).unwrap();
        assert!(pre_tag.contains("<pre class=\"lumis test-pre-class\">"));
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
        let mut buffer = Vec::new();
        crate::formatters::html::open_pre_tag(
            &mut buffer,
            formatter.pre_class.as_deref(),
            formatter.theme.as_ref(),
        )
        .unwrap();
        let pre_tag = String::from_utf8(buffer).unwrap();
        assert!(pre_tag.contains("<pre class=\"lumis test-pre-class\" style=\"color: #1f2328; background-color: #ffffff;\">"));
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

        let mut buffer = Vec::new();
        crate::formatters::html::open_pre_tag(
            &mut buffer,
            formatter.pre_class.as_deref(),
            formatter.theme.as_ref(),
        )
        .unwrap();
        let pre_tag = String::from_utf8(buffer).unwrap();
        assert!(pre_tag.contains("<pre class=\"lumis test-pre-class\" style=\"color: #1f2328; background-color: #ffffff;\">"));
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

        let mut buffer = Vec::new();
        crate::write_highlight(&mut buffer, code, formatter).unwrap();
        let result = String::from_utf8(buffer).unwrap();

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

        let mut buffer = Vec::new();
        crate::write_highlight(&mut buffer, code, formatter).unwrap();
        let result = String::from_utf8(buffer).unwrap();

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

        let mut buffer = Vec::new();
        crate::write_highlight(&mut buffer, code, formatter).unwrap();
        let result = String::from_utf8(buffer).unwrap();

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

        let mut buffer = Vec::new();
        crate::write_highlight(&mut buffer, code, formatter).unwrap();
        let result = String::from_utf8(buffer).unwrap();

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

        let mut buffer = Vec::new();
        crate::write_highlight(&mut buffer, code, formatter).unwrap();
        let result = String::from_utf8(buffer).unwrap();

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

        let mut buffer = Vec::new();
        crate::write_highlight(&mut buffer, code, formatter).unwrap();
        let result = String::from_utf8(buffer).unwrap();

        let expected = r#"<section class="highlight" data-lang="rust"><pre class="lumis custom-class"><code class="language-rust" translate="no" tabindex="0"><div class="l-line" data-line="1"><span>fn</span> <span>main</span><span>(</span><span>)</span> <span>{</span> <span>}</span>
</div></code></pre></section>"#;
        assert_str_eq!(result, expected);
    }
}

//! HTML generation helpers for creating custom HTML formatters.
//!
//! This module provides utilities to make it easy to create custom HTML formatters
//! without dealing with tree-sitter internals directly.
//!
//! See also:
//! - [`Formatter`](crate::formatters::Formatter) trait documentation for a complete example
//! - [`crates/lumis/examples/custom_html_formatter.rs`](https://github.com/leandrocp/lumis/blob/main/crates/lumis/examples/custom_html_formatter.rs)

use crate::languages::Language;
use crate::themes::Theme;
pub use lumis_core::formatter::HtmlElement;
use std::io::{self, Write};
use std::ops::RangeInclusive;

/// Generate an HTML `<span>` element with inline CSS styles.
///
/// This is useful for creating inline-styled HTML output similar to the
/// built-in `HtmlInline` formatter.
///
/// # Arguments
///
/// * `text` - The text content to wrap
/// * `scope` - The tree-sitter scope name (e.g., "keyword")
/// * `language` - Optional language for specialized scope lookup
/// * `theme` - Optional theme for style lookup
/// * `italic` - Whether to include italic styles
/// * `include_highlights` - Whether to include `data-highlight` attribute
///
/// # Example
///
/// ```rust
/// use lumis::{html, languages::Language, themes};
///
/// let theme = themes::get("dracula").ok();
/// let span = html::span_inline("fn", Some(Language::Rust), "keyword", theme.as_ref(), false, true);
/// assert_eq!(span, r#"<span data-highlight="keyword" style="color: #ff79c6;">fn</span>"#);
/// ```
///
/// A scope that resolves to no attributes still gets a bare `<span>`:
///
/// ```rust
/// use lumis::html;
///
/// assert_eq!(html::span_inline("fn", None, "keyword", None, false, false), "<span>fn</span>");
/// ```
pub fn span_inline(
    text: &str,
    language: Option<Language>,
    scope: &str,
    theme: Option<&Theme>,
    italic: bool,
    include_highlights: bool,
) -> String {
    lumis_core::formatter::html::span_inline(
        text,
        language,
        scope,
        theme,
        italic,
        include_highlights,
    )
}

/// Generate HTML attributes for a span with inline CSS styles.
///
/// Returns only the attributes string (without the span tags), useful when you
/// need more control over the HTML structure.
///
/// # Example
///
/// ```rust
/// use lumis::{html, languages::Language, themes};
///
/// let theme = themes::get("dracula").ok();
/// let attrs = html::span_inline_attrs(Some(Language::Rust), "keyword", theme.as_ref(), false, true);
/// assert_eq!(attrs, r#"data-highlight="keyword" style="color: #ff79c6;""#);
/// ```
pub fn span_inline_attrs(
    language: Option<Language>,
    scope: &str,
    theme: Option<&Theme>,
    italic: bool,
    include_highlights: bool,
) -> String {
    lumis_core::formatter::html::span_inline_attrs(
        language,
        scope,
        theme,
        italic,
        include_highlights,
    )
}

/// Generate an HTML `<span>` element with CSS class.
///
/// This is useful for creating class-based HTML output similar to the
/// built-in `HtmlLinked` formatter.
///
/// # Arguments
///
/// * `text` - The text content to wrap
/// * `scope` - The tree-sitter scope to map to a CSS class
///
/// # Example
///
/// ```rust
/// use lumis::html;
///
/// let span = html::span_linked("fn span", "keyword.function");
/// assert_eq!(span, r#"<span class="l-keyword-function">fn span</span>"#);
/// ```
pub fn span_linked(text: &str, scope: &str) -> String {
    lumis_core::formatter::html::span_linked(text, scope)
}

/// Generate HTML attributes for a span with CSS class.
///
/// Returns only the attributes string (without the span tags), useful when you
/// need more control over the HTML structure.
///
/// # Example
///
/// ```rust
/// use lumis::html;
///
/// let attrs = html::span_linked_attrs("keyword.function");
/// assert_eq!(attrs, r#"class="l-keyword-function""#);
/// ```
pub fn span_linked_attrs(scope: &str) -> String {
    lumis_core::formatter::html::span_linked_attrs(scope)
}

/// Sanitize a theme name for use in CSS variable names.
///
/// Converts non-alphanumeric characters (except `-` and `_`) to `-`.
///
/// # Example
///
/// ```rust
/// use lumis::html;
///
/// assert_eq!(html::sanitize_theme_name("github-dark"), "github-dark");
/// assert_eq!(html::sanitize_theme_name("my theme"), "my-theme");
/// ```
pub fn sanitize_theme_name(name: &str) -> String {
    lumis_core::formatter::html::sanitize_theme_name(name)
}

use crate::themes::TextDecoration;

/// Get the CSS text-decoration value from a `TextDecoration` struct.
///
/// # Example
///
/// ```rust
/// use lumis::{html, themes::{TextDecoration, UnderlineStyle}};
///
/// let none = TextDecoration::default();
/// assert_eq!(html::text_decoration(&none), "none");
///
/// let underline = TextDecoration { underline: UnderlineStyle::Solid, strikethrough: false };
/// assert_eq!(html::text_decoration(&underline), "underline");
///
/// let wavy = TextDecoration { underline: UnderlineStyle::Wavy, strikethrough: false };
/// assert_eq!(html::text_decoration(&wavy), "underline wavy");
///
/// let strike = TextDecoration { underline: UnderlineStyle::None, strikethrough: true };
/// assert_eq!(html::text_decoration(&strike), "line-through");
///
/// let both = TextDecoration { underline: UnderlineStyle::Solid, strikethrough: true };
/// assert_eq!(html::text_decoration(&both), "underline line-through");
/// ```
pub fn text_decoration(td: &TextDecoration) -> &'static str {
    lumis_core::formatter::html::text_decoration(td)
}

/// Generate HTML attributes for a span with CSS variables for multiple themes.
///
/// Returns only the attributes string (without the span tags), useful when you
/// need more control over the HTML structure.
///
/// # Example
///
/// ```rust
/// use lumis::{html, themes};
/// use std::collections::HashMap;
///
/// let mut theme_map = HashMap::new();
/// theme_map.insert("dark".to_string(), themes::get("dracula").unwrap());
///
/// let attrs = html::span_multi_themes_attrs("keyword", None, &theme_map, None, "--hl", false, false);
/// assert_eq!(attrs, r#"style="--hl-dark:#ff79c6; --hl-dark-font-style:normal; --hl-dark-font-weight:normal; --hl-dark-text-decoration:none;""#);
/// ```
pub fn span_multi_themes_attrs(
    scope: &str,
    language: Option<Language>,
    themes: &std::collections::HashMap<String, Theme>,
    default_theme: Option<&str>,
    css_variable_prefix: &str,
    italic: bool,
    include_highlights: bool,
) -> String {
    lumis_core::formatter::html::span_multi_themes_attrs(
        scope,
        language,
        themes,
        default_theme,
        css_variable_prefix,
        italic,
        include_highlights,
    )
}

/// Generate an HTML `<span>` element with CSS variables for multiple themes.
///
/// This is useful for creating multi-theme HTML output similar to the
/// built-in `HtmlMultiThemes` formatter. Each theme gets CSS variables
/// for color, font-style, font-weight, and text-decoration.
///
/// # Arguments
///
/// * `text` - The text content to wrap
/// * `scope` - The tree-sitter scope name (e.g., "keyword", "string")
/// * `language` - Optional language for specialized scope lookup
/// * `themes` - Map of theme name to Theme
/// * `default_theme` - Optional name of the default theme (gets inline styles)
/// * `css_variable_prefix` - CSS variable prefix (e.g., "--lumis")
/// * `italic` - Whether to enable italic styling
/// * `include_highlights` - Whether to include data-highlight attribute
///
/// # Example
///
/// ```rust
/// use lumis::{html, themes};
/// use std::collections::HashMap;
///
/// let mut theme_map = HashMap::new();
/// theme_map.insert("dark".to_string(), themes::get("dracula").unwrap());
///
/// let span = html::span_multi_themes(
///     "fn",
///     "keyword",
///     None,
///     &theme_map,
///     None,
///     "--hl",
///     false,
///     false,
/// );
/// assert_eq!(span, r#"<span style="--hl-dark:#ff79c6; --hl-dark-font-style:normal; --hl-dark-font-weight:normal; --hl-dark-text-decoration:none;">fn</span>"#);
///
/// // With data-highlight attribute
/// let span = html::span_multi_themes("fn", "keyword", None, &theme_map, None, "--hl", false, true);
/// assert_eq!(span, r#"<span data-highlight="keyword" style="--hl-dark:#ff79c6; --hl-dark-font-style:normal; --hl-dark-font-weight:normal; --hl-dark-text-decoration:none;">fn</span>"#);
/// ```
#[allow(clippy::too_many_arguments)]
pub fn span_multi_themes(
    text: &str,
    scope: &str,
    language: Option<Language>,
    themes: &std::collections::HashMap<String, Theme>,
    default_theme: Option<&str>,
    css_variable_prefix: &str,
    italic: bool,
    include_highlights: bool,
) -> String {
    lumis_core::formatter::html::span_multi_themes(
        text,
        scope,
        language,
        themes,
        default_theme,
        css_variable_prefix,
        italic,
        include_highlights,
    )
}

/// Escape text for safe HTML output.
///
/// Escapes the following characters:
/// - `&` → `&amp;`
/// - `<` → `&lt;`
/// - `>` → `&gt;`
/// - `"` → `&quot;`
/// - `'` → `&#39;`
///
/// # Example
///
/// ```rust
/// use lumis::html;
///
/// assert_eq!(html::escape("<script>"), "&lt;script&gt;");
/// assert_eq!(html::escape("{code}"), "{code}");
/// ```
pub fn escape(text: &str) -> String {
    lumis_core::formatter::html::escape(text)
}

/// Escape a value for use inside a double-quoted HTML attribute.
///
/// The helpers in this module already escape every attribute they build, so this
/// is for custom formatters that assemble their own tags. The escape set matches
/// [`escape`].
///
/// # Example
///
/// ```rust
/// use lumis::html;
///
/// assert_eq!(html::escape_attr(r#"x"><script>"#), "x&quot;&gt;&lt;script&gt;");
/// assert_eq!(html::escape_attr("font-family: 'Fira Code'"), "font-family: &#39;Fira Code&#39;");
/// ```
pub fn escape_attr(value: &str) -> String {
    lumis_core::formatter::html::escape_attr(value)
}

/// Escape braces for framework compatibility.
///
/// Replaces `{` with `&lbrace;` and `}` with `&rbrace;`. This is useful
/// when rendering code inside template systems that use braces for interpolation
/// (like Handlebars, Liquid, Jinja, Phoenix templates, etc.).
///
/// # Example
///
/// ```rust
/// use lumis::html;
///
/// assert_eq!(html::escape_braces("fn main() { }"), "fn main() &lbrace; &rbrace;");
/// ```
pub fn escape_braces(text: &str) -> String {
    lumis_core::formatter::html::escape_braces(text)
}

/// Wrap content in a line div with optional class and style attributes.
///
/// Creates a `<div class="l-line..." data-line="N">content</div>` element
/// with optional additional CSS classes and inline styles.
///
/// # Arguments
///
/// * `line_number` - The 1-based line number
/// * `content` - The HTML content for the line
/// * `class_suffix` - Optional additional CSS classes (e.g., " highlighted custom-class")
/// * `style` - Optional inline style attribute content
///
/// # Example
///
/// ```rust
/// use lumis::html;
///
/// let line = html::wrap_line(1, "content", Some(" highlighted"), Some("background: yellow"));
/// assert_eq!(line, r#"<div class="l-line highlighted" style="background: yellow" data-line="1">content</div>"#);
/// ```
pub fn wrap_line(
    line_number: usize,
    content: &str,
    class_suffix: Option<&str>,
    style: Option<&str>,
) -> String {
    lumis_core::formatter::html::wrap_line(line_number, content, class_suffix, style)
}

/// Map tree-sitter scope to CSS class name.
///
/// Converts scope names to their corresponding CSS class names using the
/// CLASSES constant. This maintains the full scope hierarchy specificity.
///
/// # Example
///
/// ```rust
/// use lumis::html;
///
/// assert_eq!(html::scope_to_class("string"), "l-string");
/// assert_eq!(html::scope_to_class("function.method.call"), "l-function-method-call");
/// ```
pub fn scope_to_class(scope: &str) -> String {
    lumis_core::formatter::html::scope_to_class(scope)
}

/// Generate an opening `<pre>` tag with optional class and theme styles.
///
/// Creates the opening `<pre>` tag with the base "lumis" class, an optional custom class,
/// and optional theme styling for background and foreground colors.
///
/// # Arguments
///
/// * `output` - Writer to send the tag to
/// * `pre_class` - Optional additional CSS class to append
/// * `theme` - Optional theme for extracting pre tag styles
///
/// # Example
///
/// ```rust
/// use lumis::html;
///
/// let mut output = Vec::new();
/// html::open_pre_tag(&mut output, Some("my-code"), None).unwrap();
/// assert_eq!(String::from_utf8(output).unwrap(), r#"<pre class="lumis my-code">"#);
/// ```
pub fn open_pre_tag(
    output: &mut dyn Write,
    pre_class: Option<&str>,
    theme: Option<&Theme>,
) -> io::Result<()> {
    lumis_core::formatter::html::open_pre_tag(output, pre_class, theme)
}

/// Generate an opening `<pre>` tag with classes and styles for multiple themes.
///
/// Creates the opening `<pre>` tag used by `HtmlMultiThemes`, including the
/// base `lumis lumis-themes` classes, one class per theme name, optional custom
/// classes, and the theme foreground/background styles.
///
/// # Arguments
///
/// * `output` - Writer to send the tag to
/// * `pre_class` - Optional additional CSS class to append
/// * `themes` - Map of theme name to theme
/// * `default_theme` - Optional default theme name, or `light-dark()`
/// * `css_variable_prefix` - CSS variable prefix, e.g. `--lumis`
///
/// # Example
///
/// ```rust
/// use lumis::{html, themes};
/// use std::collections::HashMap;
///
/// let mut themes = HashMap::new();
/// themes.insert("light".to_string(), themes::get("github_light").unwrap());
/// themes.insert("dark".to_string(), themes::get("github_dark").unwrap());
///
/// let mut output = Vec::new();
/// html::open_multi_themes_pre_tag(
///     &mut output,
///     Some("code-block"),
///     &themes,
///     Some("light-dark()"),
///     "--lumis",
/// )
/// .unwrap();
///
/// let html = String::from_utf8(output).unwrap();
/// assert!(html.starts_with(r#"<pre class="lumis lumis-themes code-block"#));
/// ```
pub fn open_multi_themes_pre_tag(
    output: &mut dyn Write,
    pre_class: Option<&str>,
    themes: &std::collections::HashMap<String, Theme>,
    default_theme: Option<&str>,
    css_variable_prefix: &str,
) -> io::Result<()> {
    lumis_core::formatter::html::open_multi_themes_pre_tag(
        output,
        pre_class,
        themes,
        default_theme,
        css_variable_prefix,
    )
}

/// Generate an opening `<code>` tag with language class.
///
/// Creates the opening `<code>` tag with the language class, translate="no",
/// and tabindex="0" attributes.
///
/// # Arguments
///
/// * `output` - Writer to send the tag to
/// * `lang` - The programming language for the code class
///
/// # Example
///
/// ```rust
/// use lumis::{html, languages::Language};
///
/// let mut output = Vec::new();
/// html::open_code_tag(&mut output, &Language::Rust).unwrap();
/// assert_eq!(String::from_utf8(output).unwrap(), r#"<code class="language-rust" translate="no" tabindex="0">"#);
/// ```
pub fn open_code_tag(output: &mut dyn Write, lang: &Language) -> io::Result<()> {
    lumis_core::formatter::html::open_code_tag(output, lang)
}

/// Generate closing `</code>` tag.
///
/// # Arguments
///
/// * `output` - Writer to send the tag to
///
/// # Example
///
/// ```rust
/// use lumis::html;
///
/// let mut output = Vec::new();
/// html::close_code_tag(&mut output).unwrap();
/// assert_eq!(String::from_utf8(output).unwrap(), "</code>");
/// ```
pub fn close_code_tag(output: &mut dyn Write) -> io::Result<()> {
    lumis_core::formatter::html::close_code_tag(output)
}

/// Generate closing `</pre>` tag.
///
/// # Arguments
///
/// * `output` - Writer to send the tag to
///
/// # Example
///
/// ```rust
/// use lumis::html;
///
/// let mut output = Vec::new();
/// html::close_pre_tag(&mut output).unwrap();
/// assert_eq!(String::from_utf8(output).unwrap(), "</pre>");
/// ```
pub fn close_pre_tag(output: &mut dyn Write) -> io::Result<()> {
    lumis_core::formatter::html::close_pre_tag(output)
}

/// Generate closing `</code></pre>` tags.
///
/// Outputs the closing tags for the code and pre elements.
///
/// # Arguments
///
/// * `output` - Writer to send the tags to
///
/// # Example
///
/// ```rust
/// use lumis::html;
///
/// let mut output = Vec::new();
/// html::closing_tags(&mut output).unwrap();
/// assert_eq!(String::from_utf8(output).unwrap(), "</code></pre>");
/// ```
pub fn closing_tags(output: &mut dyn Write) -> io::Result<()> {
    lumis_core::formatter::html::closing_tags(output)
}

/// Generate an opening `<span>` tag carrying `attrs`, or a bare one when empty.
///
/// A scope a theme styles in no way still opens a `<span>`, so every one pairs
/// with the `</span>` an end event writes.
///
/// # Example
///
/// ```rust
/// use lumis::html;
///
/// assert_eq!(html::open_span(&html::span_linked_attrs("keyword")), r#"<span class="l-keyword">"#);
/// assert_eq!(html::open_span(""), "<span>");
/// ```
pub fn open_span(attrs: &str) -> String {
    lumis_core::formatter::html::open_span(attrs)
}

/// Whether `line_number` falls inside any of `lines`.
///
/// Lines are 1-based, matching the `data-line` attribute [`wrap_line`] writes.
///
/// # Example
///
/// ```rust
/// use lumis::html;
///
/// assert!(html::line_is_highlighted(&[1..=1, 3..=5], 4));
/// assert!(!html::line_is_highlighted(&[1..=1, 3..=5], 2));
/// ```
pub fn line_is_highlighted(lines: &[RangeInclusive<usize>], line_number: usize) -> bool {
    lumis_core::formatter::html::line_is_highlighted(lines, line_number)
}

/// The CSS class a highlighted line carries, or `None` when the line is not highlighted.
///
/// `class` wins over `default_class`, so a formatter can offer a caller-supplied
/// class over its own.
///
/// # Example
///
/// ```rust
/// use lumis::html;
///
/// let lines = [1..=1, 3..=5];
/// assert_eq!(html::highlight_line_class(&lines, 4, None, Some("l-highlighted")), Some("l-highlighted"));
/// assert_eq!(html::highlight_line_class(&lines, 4, Some("active"), Some("l-highlighted")), Some("active"));
/// assert_eq!(html::highlight_line_class(&lines, 2, Some("active"), None), None);
/// ```
pub fn highlight_line_class<'a>(
    lines: &[RangeInclusive<usize>],
    line_number: usize,
    class: Option<&'a str>,
    default_class: Option<&'a str>,
) -> Option<&'a str> {
    lumis_core::formatter::html::highlight_line_class(lines, line_number, class, default_class)
}

/// Split a fragment across lines, appending to the current line and starting new ones at `\n`.
///
/// The line buffer [`render_lines_from_events`] fills, exposed for a formatter
/// that fills its own.
///
/// # Example
///
/// ```rust
/// use lumis::html;
///
/// let mut lines = vec![String::from("hello")];
/// html::append_fragment(&mut lines, " world\nnew line");
/// assert_eq!(lines, ["hello world", "new line"]);
/// ```
pub fn append_fragment(lines: &mut Vec<String>, fragment: &str) {
    lumis_core::formatter::html::append_fragment(lines, fragment);
}

/// Render highlight events into HTML lines, reopening active spans at line boundaries.
///
/// A span that crosses a newline is closed at the end of one line and reopened at
/// the start of the next, so every line's tags nest on their own. `span_attrs`
/// receives a scope index into [`highlights::HIGHLIGHT_NAMES`](crate::highlights::HIGHLIGHT_NAMES)
/// and the language of the block the event came from.
///
/// # Example
///
/// ```rust
/// use lumis::{events::HighlightEvent, highlights::HIGHLIGHT_NAMES, html};
///
/// let source = "a\nb";
/// let keyword = HIGHLIGHT_NAMES.iter().position(|&s| s == "keyword").unwrap();
/// let events: Vec<HighlightEvent<'_>> = vec![
///     HighlightEvent::Start { scope_index: keyword, language: "rust".to_string() },
///     HighlightEvent::Source { start: 0, end: source.len() },
///     HighlightEvent::End,
/// ];
///
/// let lines = html::render_lines_from_events(source, &events, |scope_index, _language| {
///     html::span_linked_attrs(HIGHLIGHT_NAMES[scope_index])
/// });
///
/// assert_eq!(
///     lines,
///     [
///         r#"<span class="l-keyword">a</span>"#,
///         r#"<span class="l-keyword">b</span>"#,
///     ]
/// );
/// ```
pub fn render_lines_from_events<T, F>(
    source: &str,
    events: &[lumis_core::events::HighlightEvent<'_, T>],
    span_attrs: F,
) -> Vec<String>
where
    F: Fn(usize, &str) -> String,
{
    lumis_core::formatter::html::render_lines_from_events(source, events, span_attrs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_str_eq;

    #[test]
    fn test_escape_all_entities() {
        assert_eq!(escape("&<>\"'{}"), "&amp;&lt;&gt;&quot;&#39;{}");
    }

    #[test]
    fn test_escape_preserves_normal_text() {
        assert_eq!(escape("hello world"), "hello world");
    }

    #[test]
    fn test_escape_mixed_content() {
        assert_eq!(
            escape("fn main() { println!(\"<html>\"); }"),
            "fn main() { println!(&quot;&lt;html&gt;&quot;); }"
        );
    }

    #[test]
    fn test_escape_empty_string() {
        assert_eq!(escape(""), "");
    }

    #[test]
    fn test_escape_braces_only() {
        assert_eq!(escape_braces("fn() {}"), "fn() &lbrace;&rbrace;");
    }

    #[test]
    fn test_escape_braces_preserves_other_chars() {
        assert_eq!(
            escape_braces("fn main() { let x = 42; }"),
            "fn main() &lbrace; let x = 42; &rbrace;"
        );
    }

    #[test]
    fn test_escape_braces_no_braces() {
        assert_eq!(escape_braces("hello world"), "hello world");
    }

    #[test]
    fn test_escape_braces_empty_string() {
        assert_eq!(escape_braces(""), "");
    }

    #[test]
    fn test_scope_to_class_keyword_conditional() {
        assert_eq!(
            scope_to_class("keyword.conditional"),
            "l-keyword-conditional"
        );
    }

    #[test]
    fn test_scope_to_class_string_escape() {
        assert_eq!(scope_to_class("string.escape"), "l-string-escape");
    }

    #[test]
    fn test_scope_to_class_function_method_call() {
        assert_eq!(
            scope_to_class("function.method.call"),
            "l-function-method-call"
        );
    }

    #[test]
    fn test_scope_to_class_comment_documentation() {
        assert_eq!(
            scope_to_class("comment.documentation"),
            "l-comment-documentation"
        );
    }

    #[test]
    fn test_scope_to_class_unknown_scope() {
        assert_eq!(scope_to_class("unknown.scope.name"), "l-text");
    }

    #[test]
    fn test_scope_to_class_simple_scope() {
        assert_eq!(scope_to_class("keyword"), "l-keyword");
    }

    #[test]
    fn test_wrap_line_simple() {
        let result = wrap_line(1, "content", None, None);
        assert_str_eq!(result, r#"<div class="l-line" data-line="1">content</div>"#);
    }

    #[test]
    fn test_wrap_line_with_class() {
        let result = wrap_line(5, "highlighted content", Some(" highlighted"), None);
        assert_str_eq!(
            result,
            r#"<div class="l-line highlighted" data-line="5">highlighted content</div>"#
        );
    }

    #[test]
    fn test_wrap_line_with_style() {
        let result = wrap_line(3, "styled", None, Some("color: red;"));
        assert_str_eq!(
            result,
            r#"<div class="l-line" style="color: red;" data-line="3">styled</div>"#
        );
    }

    #[test]
    fn test_wrap_line_with_class_and_style() {
        let result = wrap_line(
            10,
            "both",
            Some(" custom-class"),
            Some("background: yellow;"),
        );
        assert_str_eq!(
            result,
            r#"<div class="l-line custom-class" style="background: yellow;" data-line="10">both</div>"#
        );
    }

    #[test]
    fn test_wrap_line_empty_content() {
        let result = wrap_line(1, "", None, None);
        assert_str_eq!(result, r#"<div class="l-line" data-line="1"></div>"#);
    }

    #[test]
    fn test_span_inline_with_theme_and_scope() {
        let theme = crate::themes::get("dracula").unwrap();
        let result = span_inline(
            "fn",
            Some(Language::Rust),
            "keyword",
            Some(&theme),
            false,
            true,
        );
        assert_str_eq!(
            result,
            r#"<span data-highlight="keyword" style="color: #ff79c6;">fn</span>"#
        );
    }

    #[test]
    fn test_span_inline_no_theme() {
        let result = span_inline("text", None, "text", None, false, false);
        assert_str_eq!(result, "<span>text</span>");
    }

    #[test]
    fn test_span_multi_themes_no_theme() {
        let result = span_multi_themes(
            "<b>",
            "text",
            None,
            &std::collections::HashMap::new(),
            None,
            "--lumis",
            false,
            false,
        );
        assert_str_eq!(result, "<span>&lt;b&gt;</span>");
    }

    #[test]
    fn test_span_linked() {
        let result = span_linked("fn", "keyword.function");
        assert_str_eq!(result, r#"<span class="l-keyword-function">fn</span>"#);
    }
}

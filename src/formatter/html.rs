//! HTML generation helpers for creating custom HTML formatters.
//!
//! This module provides utilities to make it easy to create custom HTML formatters
//! without dealing with tree-sitter internals directly.
//!
//! # Example: Simple HTML Formatter
//!
//! ```rust
//! use autumnus::{html, languages::Language, themes};
//! use std::io::Write;
//!
//! let code = "fn main() {}";
//! let theme = themes::get("dracula").ok();
//! let lang = Language::Rust;
//!
//! let mut output = Vec::new();
//! writeln!(&mut output, "<pre><code>").unwrap();
//!
//! for (style, text, _range, scope) in html::highlight_iter_with_scopes(code, lang, theme).unwrap() {
//!     let span = html::span_inline(text, &style, Some(scope));
//!     write!(&mut output, "{}", span).unwrap();
//! }
//!
//! writeln!(&mut output, "</code></pre>").unwrap();
//! ```
//!
//! See also:
//! - [`Formatter`](crate::formatter::Formatter) trait documentation
//! - [`examples/custom_html_formatter.rs`](https://github.com/leandrocp/autumnus/blob/main/examples/custom_html_formatter.rs)

use crate::constants::HIGHLIGHT_NAMES;
use crate::highlight::Style;
use crate::languages::Language;
use crate::themes::Theme;
use crate::vendor::tree_sitter_highlight::{HighlightEvent, Highlighter as TSHighlighter};
use std::io::{self, Write};
use std::ops::Range;
use std::sync::Arc;

/// Iterator over highlighted tokens with scope names.
///
/// Returns tuples of `(Arc<Style>, text, byte_range, scope_name)` for each token.
pub struct HighlightIteratorWithScopes<'a> {
    segments: Vec<(Arc<Style>, &'a str, Range<usize>, &'static str)>,
    index: usize,
}

impl<'a> Iterator for HighlightIteratorWithScopes<'a> {
    type Item = (Arc<Style>, &'a str, Range<usize>, &'static str);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.segments.len() {
            let result = self.segments[self.index].clone();
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

/// Create an iterator over highlighted tokens with scope names.
///
/// This is similar to [`crate::highlight::highlight_iter`] but also provides
/// the tree-sitter scope name for each token (e.g., "keyword", "string.quoted").
///
/// # Arguments
///
/// * `source` - The source code to highlight
/// * `language` - The language to use for parsing
/// * `theme` - Optional theme for styling
///
/// # Returns
///
/// An iterator yielding `(Arc<Style>, &str, Range<usize>, &'static str)` tuples:
/// - `Arc<Style>` - Color and font styling information (shared reference)
/// - `&str` - The token text
/// - `Range<usize>` - Byte range in source
/// - `&'static str` - Scope name (e.g., "keyword", "string")
///
/// # Example
///
/// ```rust
/// use autumnus::{html, languages::Language, themes};
///
/// let code = "let x = 42;";
/// let theme = themes::get("dracula").ok();
///
/// for (style, text, range, scope) in html::highlight_iter_with_scopes(code, Language::Rust, theme).unwrap() {
///     println!("{} (scope: {}, color: {:?})", text, scope, style.fg);
/// }
/// ```
pub fn highlight_iter_with_scopes(
    source: &str,
    language: Language,
    theme: Option<Theme>,
) -> Result<HighlightIteratorWithScopes<'_>, String> {
    let mut highlighter = TSHighlighter::new();
    let events = highlighter
        .highlight(language.config(), source.as_bytes(), None, |injected| {
            Some(Language::guess(Some(injected), "").config())
        })
        .map_err(|e| format!("Failed to highlight: {}", e))?;

    let mut segments = Vec::new();
    let mut current_style = Arc::new(Style::default());
    let mut current_scope = "";

    for event in events {
        let event = event.map_err(|e| format!("Failed to get highlight event: {:?}", e))?;

        match event {
            HighlightEvent::HighlightStart {
                highlight,
                language,
            } => {
                let scope = HIGHLIGHT_NAMES[highlight.0];
                let specialized_scope = format!("{}.{}", scope, language);
                current_scope = scope;

                current_style = if let Some(ref theme) = theme {
                    Arc::new(
                        theme
                            .get_style(&specialized_scope)
                            .map(Style::from_theme_style)
                            .unwrap_or_default(),
                    )
                } else {
                    Arc::new(Style::default())
                };
            }
            HighlightEvent::Source { start, end } => {
                let text = &source[start..end];
                if !text.is_empty() {
                    segments.push((Arc::clone(&current_style), text, start..end, current_scope));
                }
            }
            HighlightEvent::HighlightEnd => {
                current_style = Arc::new(Style::default());
                current_scope = "";
            }
        }
    }

    Ok(HighlightIteratorWithScopes { segments, index: 0 })
}

/// Generate an HTML `<span>` element with inline CSS styles.
///
/// This is useful for creating inline-styled HTML output similar to the
/// built-in `HtmlInline` formatter.
///
/// # Arguments
///
/// * `text` - The text content to wrap
/// * `style` - The styling to apply (colors, bold, italic, etc.)
/// * `scope` - Optional scope name to include as `data-highlight` attribute
///
/// # Example
///
/// ```rust
/// use autumnus::{html, highlight::Style};
///
/// let style = Style {
///     fg: Some("#ff79c6".to_string()),
///     bold: true,
///     ..Default::default()
/// };
///
/// let span = html::span_inline("keyword", &style, Some("keyword"));
/// // Returns: <span data-highlight="keyword" style="color: #ff79c6; font-weight: bold;">keyword</span>
/// ```
pub fn span_inline(text: &str, style: &Style, scope: Option<&str>) -> String {
    let escaped = escape(text);
    let mut attrs = String::new();

    if let Some(scope_name) = scope {
        attrs.push_str(&format!(" data-highlight=\"{}\"", scope_name));
    }

    if style.fg.is_some() || style.bg.is_some() || style.bold || style.italic {
        attrs.push_str(" style=\"");
        let mut styles = Vec::new();

        if let Some(fg) = &style.fg {
            styles.push(format!("color: {};", fg));
        }
        if let Some(bg) = &style.bg {
            styles.push(format!("background-color: {};", bg));
        }
        if style.bold {
            styles.push("font-weight: bold;".to_string());
        }
        if style.italic {
            styles.push("font-style: italic;".to_string());
        }
        if style.underline {
            styles.push("text-decoration: underline;".to_string());
        }
        if style.strikethrough {
            styles.push("text-decoration: line-through;".to_string());
        }

        attrs.push_str(&styles.join(" "));
        attrs.push('"');
    }

    if attrs.is_empty() {
        escaped
    } else {
        format!("<span{}>{}</span>", attrs, escaped)
    }
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
/// use autumnus::html;
///
/// let span = html::span_linked("keyword", "keyword.control");
/// // Returns: <span class="keyword">keyword</span>
/// ```
pub fn span_linked(text: &str, scope: &str) -> String {
    let escaped = escape(text);
    let class = scope_to_class(scope);
    format!("<span class=\"{}\">{}</span>", class, escaped)
}

/// Escape text for safe HTML output.
///
/// Escapes the following characters:
/// - `&` → `&amp;`
/// - `<` → `&lt;`
/// - `>` → `&gt;`
/// - `"` → `&quot;`
/// - `'` → `&#39;`
/// - `{` → `&lbrace;` (for framework compatibility)
/// - `}` → `&rbrace;` (for framework compatibility)
///
/// # Example
///
/// ```rust
/// use autumnus::html;
///
/// assert_eq!(html::escape("<script>"), "&lt;script&gt;");
/// assert_eq!(html::escape("{code}"), "&lbrace;code&rbrace;");
/// ```
pub fn escape(text: &str) -> String {
    let mut buf = String::with_capacity(text.len() + text.len() / 10);

    for c in text.chars() {
        match c {
            '&' => buf.push_str("&amp;"),
            '<' => buf.push_str("&lt;"),
            '>' => buf.push_str("&gt;"),
            '"' => buf.push_str("&quot;"),
            '\'' => buf.push_str("&#39;"),
            '{' => buf.push_str("&lbrace;"),
            '}' => buf.push_str("&rbrace;"),
            _ => buf.push(c),
        }
    }

    buf
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
/// use autumnus::html;
///
/// assert_eq!(html::escape_braces("fn main() { }"), "fn main() &lbrace; &rbrace;");
/// ```
pub fn escape_braces(text: &str) -> String {
    text.replace('{', "&lbrace;").replace('}', "&rbrace;")
}

/// Wrap content in a line div with optional class and style attributes.
///
/// Creates a `<div class="line..." data-line="N">content</div>` element
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
/// use autumnus::html;
///
/// let line = html::wrap_line(1, "content", Some(" highlighted"), Some("background: yellow"));
/// // Returns: <div class="line highlighted" style="background: yellow" data-line="1">content</div>
/// ```
pub fn wrap_line(
    line_number: usize,
    content: &str,
    class_suffix: Option<&str>,
    style: Option<&str>,
) -> String {
    let class_attr = if let Some(suffix) = class_suffix {
        format!("line{}", suffix)
    } else {
        "line".to_string()
    };

    let style_attr = if let Some(s) = style {
        format!(" style=\"{}\"", s)
    } else {
        String::new()
    };

    format!(
        "<div class=\"{}\"{}data-line=\"{}\">{}</div>",
        class_attr,
        if style.is_some() {
            format!("{} ", style_attr)
        } else {
            " ".to_string()
        },
        line_number,
        content
    )
}

/// Map tree-sitter scope to CSS class name.
///
/// Converts scope names to their corresponding CSS class names using the
/// CLASSES constant. This maintains the full scope hierarchy specificity.
///
/// # Example
///
/// ```rust
/// use autumnus::html;
///
/// assert_eq!(html::scope_to_class("keyword.conditional"), "keyword-conditional");
/// assert_eq!(html::scope_to_class("string.escape"), "string-escape");
/// assert_eq!(html::scope_to_class("function.method.call"), "function-method-call");
/// ```
pub fn scope_to_class(scope: &str) -> &str {
    crate::constants::HIGHLIGHT_NAMES
        .iter()
        .position(|&s| s == scope)
        .and_then(|idx| crate::constants::CLASSES.get(idx))
        .copied()
        .unwrap_or("text")
}

/// Wrap HTML content into line-wrapped divs.
///
/// Takes HTML content and wraps it in `<div class="line">` elements,
/// one per line of code.
///
/// # Example
///
/// ```rust
/// use autumnus::html;
///
/// let html = "line1\nline2\nline3";
/// let lines = html::wrap_in_lines(html);
///
/// assert_eq!(lines.len(), 3);
/// ```
// TODO: remove wrap_in_line in favor of wrap_line
pub fn wrap_in_lines(html: &str) -> Vec<String> {
    html.lines()
        .enumerate()
        .map(|(i, line)| {
            let line_number = i + 1;
            format!(
                "<div class=\"line\" data-line=\"{}\">{}\n</div>",
                line_number, line
            )
        })
        .collect()
}

/// Generate an opening `<pre>` tag with optional class and theme styles.
///
/// Creates the opening `<pre>` tag with the base "athl" class, an optional custom class,
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
/// use autumnus::{html, themes};
/// use std::io::Write;
///
/// let mut output = Vec::new();
/// let theme = themes::get("dracula").ok();
///
/// html::open_pre_tag(&mut output, Some("my-code"), theme.as_ref()).unwrap();
/// // Outputs: <pre class="athl my-code" style="...theme colors...">
/// ```
pub fn open_pre_tag(
    output: &mut dyn Write,
    pre_class: Option<&str>,
    theme: Option<&Theme>,
) -> io::Result<()> {
    let class = if let Some(pre_class) = pre_class {
        format!("athl {pre_class}")
    } else {
        "athl".to_string()
    };

    write!(
        output,
        "<pre class=\"{}\"{}>",
        class,
        theme
            .and_then(|theme| theme.pre_style(" "))
            .map(|pre_style| format!(" style=\"{pre_style}\""))
            .unwrap_or_default(),
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
/// use autumnus::{html, languages::Language};
/// use std::io::Write;
///
/// let mut output = Vec::new();
/// html::open_code_tag(&mut output, &Language::Rust).unwrap();
/// // Outputs: <code class="language-rust" translate="no" tabindex="0">
/// ```
pub fn open_code_tag(output: &mut dyn Write, lang: &Language) -> io::Result<()> {
    write!(
        output,
        "<code class=\"language-{}\" translate=\"no\" tabindex=\"0\">",
        lang.id_name()
    )
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
/// use autumnus::html;
/// use std::io::Write;
///
/// let mut output = Vec::new();
/// html::closing_tags(&mut output).unwrap();
/// // Outputs: </code></pre>
/// ```
pub fn closing_tags(output: &mut dyn Write) -> io::Result<()> {
    output.write_all(b"</code></pre>")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_all_entities() {
        assert_eq!(
            escape("&<>\"'{}"),
            "&amp;&lt;&gt;&quot;&#39;&lbrace;&rbrace;"
        );
    }

    #[test]
    fn test_escape_preserves_normal_text() {
        assert_eq!(escape("hello world"), "hello world");
    }

    #[test]
    fn test_escape_mixed_content() {
        assert_eq!(
            escape("fn main() { println!(\"<html>\"); }"),
            "fn main() &lbrace; println!(&quot;&lt;html&gt;&quot;); &rbrace;"
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
        assert_eq!(scope_to_class("keyword.conditional"), "keyword-conditional");
    }

    #[test]
    fn test_scope_to_class_string_escape() {
        assert_eq!(scope_to_class("string.escape"), "string-escape");
    }

    #[test]
    fn test_scope_to_class_function_method_call() {
        assert_eq!(
            scope_to_class("function.method.call"),
            "function-method-call"
        );
    }

    #[test]
    fn test_scope_to_class_comment_documentation() {
        assert_eq!(
            scope_to_class("comment.documentation"),
            "comment-documentation"
        );
    }

    #[test]
    fn test_scope_to_class_unknown_scope() {
        assert_eq!(scope_to_class("unknown.scope.name"), "text");
    }

    #[test]
    fn test_scope_to_class_simple_scope() {
        assert_eq!(scope_to_class("keyword"), "keyword");
    }

    #[test]
    fn test_wrap_line_simple() {
        let result = wrap_line(1, "content", None, None);
        assert_eq!(result, "<div class=\"line\" data-line=\"1\">content</div>");
    }

    #[test]
    fn test_wrap_line_with_class() {
        let result = wrap_line(5, "highlighted content", Some(" highlighted"), None);
        assert_eq!(
            result,
            "<div class=\"line highlighted\" data-line=\"5\">highlighted content</div>"
        );
    }

    #[test]
    fn test_wrap_line_with_style() {
        let result = wrap_line(3, "styled", None, Some("color: red;"));
        assert_eq!(
            result,
            "<div class=\"line\" style=\"color: red;\" data-line=\"3\">styled</div>"
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
        assert_eq!(
            result,
            "<div class=\"line custom-class\" style=\"background: yellow;\" data-line=\"10\">both</div>"
        );
    }

    #[test]
    fn test_wrap_line_empty_content() {
        let result = wrap_line(1, "", None, None);
        assert_eq!(result, "<div class=\"line\" data-line=\"1\"></div>");
    }

    #[test]
    fn test_span_inline_with_style_and_scope() {
        let style = Style {
            fg: Some("#ff79c6".to_string()),
            bold: true,
            ..Default::default()
        };
        let result = span_inline("keyword", &style, Some("keyword"));
        assert_eq!(
            result,
            "<span data-highlight=\"keyword\" style=\"color: #ff79c6; font-weight: bold;\">keyword</span>"
        );
    }

    #[test]
    fn test_span_inline_no_style() {
        let style = Style::default();
        let result = span_inline("text", &style, None);
        assert_eq!(result, "text");
    }

    #[test]
    fn test_span_linked() {
        let result = span_linked("fn", "keyword.function");
        assert_eq!(result, "<span class=\"keyword-function\">fn</span>");
    }
}

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
use std::io::{self, Write};
use std::ops::Range;
use std::sync::Arc;
use tree_sitter_highlight::{HighlightEvent, Highlighter as TSHighlighter};

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
            HighlightEvent::HighlightStart(idx) => {
                let scope = HIGHLIGHT_NAMES[idx.0];
                current_scope = scope;

                current_style = if let Some(ref theme) = theme {
                    Arc::new(
                        theme
                            .get_style(scope)
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
/// - `{` → `&lbrace;` (for framework compatibility)
/// - `}` → `&rbrace;` (for framework compatibility)
/// - `<` → `&lt;`
/// - `>` → `&gt;`
/// - `&` → `&amp;`
/// - `"` → `&quot;`
/// - `'` → `&#39;`
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
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
        .replace('{', "&lbrace;")
        .replace('}', "&rbrace;")
}

/// Map tree-sitter scope to CSS class name.
///
/// Converts scope names like "keyword.control" to CSS-friendly class names
/// like "keyword".
///
/// # Example
///
/// ```rust
/// use autumnus::html;
///
/// assert_eq!(html::scope_to_class("keyword.control"), "keyword");
/// assert_eq!(html::scope_to_class("string.quoted"), "string");
/// ```
pub fn scope_to_class(scope: &str) -> &str {
    // Map to first segment of scope
    scope.split('.').next().unwrap_or("text")
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
pub fn wrap_in_lines(html: &str) -> Vec<String> {
    html.lines()
        .enumerate()
        .map(|(i, line)| {
            let line_number = i + 1;
            format!(
                "<div class=\"line\" data-line=\"{}\">{}</div>",
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

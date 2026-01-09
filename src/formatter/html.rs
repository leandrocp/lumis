//! HTML generation helpers for creating custom HTML formatters.
//!
//! This module provides utilities to make it easy to create custom HTML formatters
//! without dealing with tree-sitter internals directly.
//!
//! # Example: Simple HTML Formatter
//!
//! ```rust
//! use autumnus::{highlight::highlight_iter, html, languages::Language, themes};
//! use std::io::Write;
//!
//! let code = "fn main() {}";
//! let theme = themes::get("dracula").unwrap();
//! let lang = Language::Rust;
//!
//! let mut output = Vec::new();
//! html::open_pre_tag(&mut output, None, Some(&theme)).unwrap();
//! html::open_code_tag(&mut output, &lang).unwrap();
//!
//! highlight_iter(code, lang, Some(theme.clone()), |text, _range, scope, _style| {
//!     let span = html::span_inline(text, scope, Some(lang), Some(&theme), false, false);
//!     write!(&mut output, "{}", span)
//! }).unwrap();
//!
//! html::closing_tags(&mut output).unwrap();
//! ```
//!
//! See also:
//! - [`Formatter`](crate::formatter::Formatter) trait documentation
//! - [`examples/custom_html_formatter.rs`](https://github.com/leandrocp/autumnus/blob/main/examples/custom_html_formatter.rs)

use crate::languages::Language;
use crate::themes::Theme;
use std::io::{self, Write};

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
/// use autumnus::{html, languages::Language, themes};
///
/// let theme = themes::get("dracula").ok();
/// let span = html::span_inline("fn", "keyword", Some(Language::Rust), theme.as_ref(), false, true);
/// assert_eq!(span, r#"<span data-highlight="keyword" style="color: #ff79c6;">fn</span>"#);
/// ```
pub fn span_inline(
    text: &str,
    scope: &str,
    language: Option<Language>,
    theme: Option<&Theme>,
    italic: bool,
    include_highlights: bool,
) -> String {
    let escaped = escape(text);
    let attrs = span_inline_attrs(scope, language, theme, italic, include_highlights);

    if attrs.is_empty() {
        escaped
    } else {
        format!("<span {}>{}</span>", attrs, escaped)
    }
}

/// Generate HTML attributes for a span with inline CSS styles.
///
/// Returns only the attributes string (without the span tags), useful when you
/// need more control over the HTML structure.
///
/// # Example
///
/// ```rust
/// use autumnus::{html, languages::Language, themes};
///
/// let theme = themes::get("dracula").ok();
/// let attrs = html::span_inline_attrs("keyword", Some(Language::Rust), theme.as_ref(), false, true);
/// assert_eq!(attrs, r#"data-highlight="keyword" style="color: #ff79c6;""#);
/// ```
pub fn span_inline_attrs(
    scope: &str,
    language: Option<Language>,
    theme: Option<&Theme>,
    italic: bool,
    include_highlights: bool,
) -> String {
    let mut attrs = String::new();

    if include_highlights {
        attrs.push_str(&format!("data-highlight=\"{}\"", scope));
    }

    if let Some(theme) = theme {
        let specialized_scope = if let Some(lang) = language {
            format!("{}.{}", scope, lang.id_name())
        } else {
            scope.to_string()
        };

        if let Some(style) = theme.get_style(&specialized_scope) {
            let has_decoration = style.text_decoration.underline != UnderlineStyle::None
                || style.text_decoration.strikethrough;
            if include_highlights
                && (style.fg.is_some()
                    || style.bg.is_some()
                    || style.bold
                    || (italic && style.italic)
                    || has_decoration)
            {
                attrs.push(' ');
            }

            let css = style.css(italic, " ");
            if !css.is_empty() {
                attrs.push_str(&format!("style=\"{}\"", css));
            }
        }
    }

    attrs
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
/// let span = html::span_linked("fn span", "keyword.function");
/// assert_eq!(span, r#"<span class="keyword-function">fn span</span>"#);
/// ```
pub fn span_linked(text: &str, scope: &str) -> String {
    let escaped = escape(text);
    let class = scope_to_class(scope);
    format!("<span class=\"{}\">{}</span>", class, escaped)
}

/// Generate HTML attributes for a span with CSS class.
///
/// Returns only the attributes string (without the span tags), useful when you
/// need more control over the HTML structure.
///
/// # Example
///
/// ```rust
/// use autumnus::html;
///
/// let attrs = html::span_linked_attrs("keyword.function");
/// assert_eq!(attrs, r#"class="keyword-function""#);
/// ```
pub fn span_linked_attrs(scope: &str) -> String {
    let class = scope_to_class(scope);
    format!("class=\"{}\"", class)
}

/// Sanitize a theme name for use in CSS variable names.
///
/// Converts non-alphanumeric characters (except `-` and `_`) to `-`.
///
/// # Example
///
/// ```rust
/// use autumnus::html;
///
/// assert_eq!(html::sanitize_theme_name("github-dark"), "github-dark");
/// assert_eq!(html::sanitize_theme_name("my theme"), "my-theme");
/// ```
pub fn sanitize_theme_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect()
}

use crate::themes::{TextDecoration, UnderlineStyle};

/// Get the CSS text-decoration value from a TextDecoration struct.
///
/// # Example
///
/// ```rust
/// use autumnus::{html, themes::{TextDecoration, UnderlineStyle}};
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
    match (td.underline, td.strikethrough) {
        (UnderlineStyle::None, false) => "none",
        (UnderlineStyle::None, true) => "line-through",
        (UnderlineStyle::Solid, false) => "underline",
        (UnderlineStyle::Solid, true) => "underline line-through",
        (UnderlineStyle::Wavy, false) => "underline wavy",
        (UnderlineStyle::Wavy, true) => "underline wavy line-through",
        (UnderlineStyle::Double, false) => "underline double",
        (UnderlineStyle::Double, true) => "underline double line-through",
        (UnderlineStyle::Dotted, false) => "underline dotted",
        (UnderlineStyle::Dotted, true) => "underline dotted line-through",
        (UnderlineStyle::Dashed, false) => "underline dashed",
        (UnderlineStyle::Dashed, true) => "underline dashed line-through",
    }
}

/// Generate HTML attributes for a span with CSS variables for multiple themes.
///
/// Returns only the attributes string (without the span tags), useful when you
/// need more control over the HTML structure.
///
/// # Example
///
/// ```rust
/// use autumnus::{html, themes};
/// use std::collections::HashMap;
///
/// let mut theme_map = HashMap::new();
/// theme_map.insert("dark".to_string(), themes::get("dracula").unwrap());
///
/// let attrs = html::span_multi_themes_attrs("keyword", None, &theme_map, None, "--hl", false, false);
/// assert_eq!(attrs, r#"style="--hl-dark: #ff79c6; --hl-dark-font-style: normal; --hl-dark-font-weight: normal; --hl-dark-text-decoration: none;""#);
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
    if themes.is_empty() {
        return String::new();
    }

    let specialized_scope = if let Some(lang) = language {
        format!("{}.{}", scope, lang.id_name())
    } else {
        scope.to_string()
    };

    let mut inline_styles = Vec::new();
    let mut css_vars = Vec::new();

    if let Some(default_name) = default_theme {
        if default_name == "light-dark()" {
            if let (Some(light_theme), Some(dark_theme)) = (themes.get("light"), themes.get("dark"))
            {
                if let (Some(light_style), Some(dark_style)) = (
                    light_theme.get_style(&specialized_scope),
                    dark_theme.get_style(&specialized_scope),
                ) {
                    if let (Some(light_fg), Some(dark_fg)) = (&light_style.fg, &dark_style.fg) {
                        inline_styles
                            .push(format!("color: light-dark({}, {});", light_fg, dark_fg));
                    }
                    if let (Some(light_bg), Some(dark_bg)) = (&light_style.bg, &dark_style.bg) {
                        inline_styles.push(format!(
                            "background-color: light-dark({}, {});",
                            light_bg, dark_bg
                        ));
                    }
                    let light_weight = if light_style.bold { "bold" } else { "normal" };
                    let dark_weight = if dark_style.bold { "bold" } else { "normal" };
                    inline_styles.push(format!(
                        "font-weight: light-dark({}, {});",
                        light_weight, dark_weight
                    ));
                    if italic {
                        let light_style_val = if light_style.italic {
                            "italic"
                        } else {
                            "normal"
                        };
                        let dark_style_val = if dark_style.italic {
                            "italic"
                        } else {
                            "normal"
                        };
                        inline_styles.push(format!(
                            "font-style: light-dark({}, {});",
                            light_style_val, dark_style_val
                        ));
                    }
                    let light_decoration = text_decoration(&light_style.text_decoration);
                    let dark_decoration = text_decoration(&dark_style.text_decoration);
                    inline_styles.push(format!(
                        "text-decoration: light-dark({}, {});",
                        light_decoration, dark_decoration
                    ));
                }
            }
        } else if let Some(default_theme_obj) = themes.get(default_name) {
            if let Some(style) = default_theme_obj.get_style(&specialized_scope) {
                if let Some(fg) = &style.fg {
                    inline_styles.push(format!("color:{};", fg));
                }
                if let Some(bg) = &style.bg {
                    inline_styles.push(format!("background-color:{};", bg));
                }
                if style.bold {
                    inline_styles.push("font-weight:bold;".to_string());
                }
                if italic && style.italic {
                    inline_styles.push("font-style:italic;".to_string());
                }
                let td_css = text_decoration(&style.text_decoration);
                if td_css != "none" {
                    inline_styles.push(format!("text-decoration:{};", td_css));
                }

                let sanitized = sanitize_theme_name(default_name);
                let font_style = if style.italic { "italic" } else { "normal" };
                css_vars.push(format!(
                    "{}-{}-font-style:{};",
                    css_variable_prefix, sanitized, font_style
                ));

                let font_weight = if style.bold { "bold" } else { "normal" };
                css_vars.push(format!(
                    "{}-{}-font-weight:{};",
                    css_variable_prefix, sanitized, font_weight
                ));

                let text_dec = text_decoration(&style.text_decoration);
                css_vars.push(format!(
                    "{}-{}-text-decoration:{};",
                    css_variable_prefix, sanitized, text_dec
                ));
            }

            for (theme_name, theme) in themes.iter() {
                if theme_name != default_name {
                    if let Some(style) = theme.get_style(&specialized_scope) {
                        let sanitized = sanitize_theme_name(theme_name);

                        if let Some(fg) = &style.fg {
                            css_vars.push(format!("{}-{}:{};", css_variable_prefix, sanitized, fg));
                        }
                        if let Some(bg) = &style.bg {
                            css_vars
                                .push(format!("{}-{}-bg:{};", css_variable_prefix, sanitized, bg));
                        }

                        let font_style = if style.italic { "italic" } else { "normal" };
                        css_vars.push(format!(
                            "{}-{}-font-style:{};",
                            css_variable_prefix, sanitized, font_style
                        ));

                        let font_weight = if style.bold { "bold" } else { "normal" };
                        css_vars.push(format!(
                            "{}-{}-font-weight:{};",
                            css_variable_prefix, sanitized, font_weight
                        ));

                        let text_dec = text_decoration(&style.text_decoration);
                        css_vars.push(format!(
                            "{}-{}-text-decoration:{};",
                            css_variable_prefix, sanitized, text_dec
                        ));
                    }
                }
            }
        }
    } else {
        for (theme_name, theme) in themes.iter() {
            if let Some(style) = theme.get_style(&specialized_scope) {
                let sanitized = sanitize_theme_name(theme_name);

                if let Some(fg) = &style.fg {
                    css_vars.push(format!("{}-{}: {};", css_variable_prefix, sanitized, fg));
                }
                if let Some(bg) = &style.bg {
                    css_vars.push(format!("{}-{}-bg: {};", css_variable_prefix, sanitized, bg));
                }

                let font_style = if style.italic { "italic" } else { "normal" };
                css_vars.push(format!(
                    "{}-{}-font-style: {};",
                    css_variable_prefix, sanitized, font_style
                ));

                let font_weight = if style.bold { "bold" } else { "normal" };
                css_vars.push(format!(
                    "{}-{}-font-weight: {};",
                    css_variable_prefix, sanitized, font_weight
                ));

                let text_dec = text_decoration(&style.text_decoration);
                css_vars.push(format!(
                    "{}-{}-text-decoration: {};",
                    css_variable_prefix, sanitized, text_dec
                ));
            }
        }
    }

    if inline_styles.is_empty() && css_vars.is_empty() {
        return String::new();
    }

    let mut attrs = String::new();
    if include_highlights {
        attrs.push_str(&format!("data-highlight=\"{}\" ", scope));
    }

    attrs.push_str("style=\"");
    if !inline_styles.is_empty() {
        attrs.push_str(&inline_styles.join(" "));
    }
    if !css_vars.is_empty() {
        if !inline_styles.is_empty() {
            attrs.push(' ');
        }
        attrs.push_str(&css_vars.join(" "));
    }
    attrs.push('"');

    attrs
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
/// * `css_variable_prefix` - CSS variable prefix (e.g., "--athl")
/// * `italic` - Whether to enable italic styling
/// * `include_highlights` - Whether to include data-highlight attribute
///
/// # Example
///
/// ```rust
/// use autumnus::{html, themes};
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
/// assert_eq!(span, r#"<span style="--hl-dark: #ff79c6; --hl-dark-font-style: normal; --hl-dark-font-weight: normal; --hl-dark-text-decoration: none;">fn</span>"#);
///
/// // With data-highlight attribute
/// let span = html::span_multi_themes("fn", "keyword", None, &theme_map, None, "--hl", false, true);
/// assert_eq!(span, r#"<span data-highlight="keyword" style="--hl-dark: #ff79c6; --hl-dark-font-style: normal; --hl-dark-font-weight: normal; --hl-dark-text-decoration: none;">fn</span>"#);
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
    let escaped = escape(text);

    if themes.is_empty() {
        return escaped;
    }

    let attrs = span_multi_themes_attrs(
        scope,
        language,
        themes,
        default_theme,
        css_variable_prefix,
        italic,
        include_highlights,
    );

    if attrs.is_empty() {
        escaped
    } else {
        format!("<span {}>{}</span>", attrs, escaped)
    }
}

/// Escape text for safe HTML output.
///
/// Escapes the following characters:
/// - `&` → `&amp;`
/// - `<` → `&lt;`
/// - `>` → `&gt;`
/// - `"` → `&quot;`
/// - `'` → `&#39;`
/// - `{` → `&lbrace;`
/// - `}` → `&rbrace;`
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
/// assert_eq!(line, r#"<div class="line highlighted" style="background: yellow" data-line="1">content</div>"#);
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
/// assert_eq!(html::scope_to_class("string"), "string");
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
/// use autumnus::html;
///
/// let mut output = Vec::new();
/// html::open_pre_tag(&mut output, Some("my-code"), None).unwrap();
/// assert_eq!(String::from_utf8(output).unwrap(), r#"<pre class="athl my-code">"#);
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
///
/// let mut output = Vec::new();
/// html::open_code_tag(&mut output, &Language::Rust).unwrap();
/// assert_eq!(String::from_utf8(output).unwrap(), r#"<code class="language-rust" translate="no" tabindex="0">"#);
/// ```
pub fn open_code_tag(output: &mut dyn Write, lang: &Language) -> io::Result<()> {
    write!(
        output,
        "<code class=\"language-{}\" translate=\"no\" tabindex=\"0\">",
        lang.id_name()
    )
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
/// use autumnus::html;
///
/// let mut output = Vec::new();
/// html::close_code_tag(&mut output).unwrap();
/// assert_eq!(String::from_utf8(output).unwrap(), "</code>");
/// ```
pub fn close_code_tag(output: &mut dyn Write) -> io::Result<()> {
    output.write_all(b"</code>")
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
/// use autumnus::html;
///
/// let mut output = Vec::new();
/// html::close_pre_tag(&mut output).unwrap();
/// assert_eq!(String::from_utf8(output).unwrap(), "</pre>");
/// ```
pub fn close_pre_tag(output: &mut dyn Write) -> io::Result<()> {
    output.write_all(b"</pre>")
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
///
/// let mut output = Vec::new();
/// html::closing_tags(&mut output).unwrap();
/// assert_eq!(String::from_utf8(output).unwrap(), "</code></pre>");
/// ```
pub fn closing_tags(output: &mut dyn Write) -> io::Result<()> {
    close_code_tag(output)?;
    close_pre_tag(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_str_eq;

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
        assert_str_eq!(result, r#"<div class="line" data-line="1">content</div>"#);
    }

    #[test]
    fn test_wrap_line_with_class() {
        let result = wrap_line(5, "highlighted content", Some(" highlighted"), None);
        assert_str_eq!(
            result,
            r#"<div class="line highlighted" data-line="5">highlighted content</div>"#
        );
    }

    #[test]
    fn test_wrap_line_with_style() {
        let result = wrap_line(3, "styled", None, Some("color: red;"));
        assert_str_eq!(
            result,
            r#"<div class="line" style="color: red;" data-line="3">styled</div>"#
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
            r#"<div class="line custom-class" style="background: yellow;" data-line="10">both</div>"#
        );
    }

    #[test]
    fn test_wrap_line_empty_content() {
        let result = wrap_line(1, "", None, None);
        assert_str_eq!(result, r#"<div class="line" data-line="1"></div>"#);
    }

    #[test]
    fn test_span_inline_with_theme_and_scope() {
        let theme = crate::themes::get("dracula").unwrap();
        let result = span_inline(
            "fn",
            "keyword",
            Some(Language::Rust),
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
        let result = span_inline("text", "text", None, None, false, false);
        assert_str_eq!(result, "text");
    }

    #[test]
    fn test_span_linked() {
        let result = span_linked("fn", "keyword.function");
        assert_str_eq!(result, r#"<span class="keyword-function">fn</span>"#);
    }
}

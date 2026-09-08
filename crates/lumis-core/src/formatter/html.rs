//! HTML generation helpers for creating custom HTML formatters.
//!
//! These helpers work with language names as strings, making them independent of tree-sitter.

use crate::languages::Language;
use crate::themes::{Style, TextDecoration, Theme, UnderlineStyle};
use std::fmt::Write as _;
use std::io::{self, Write};

/// Generate HTML attributes for a span with inline CSS styles.
pub fn span_inline_attrs(
    language: Option<Language>,
    scope: &str,
    theme: Option<&Theme>,
    italic: bool,
    include_highlights: bool,
) -> String {
    let mut attrs = String::new();

    if include_highlights {
        let _ = write!(attrs, "data-highlight=\"{scope}\"");
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
                let _ = write!(attrs, "style=\"{css}\"");
            }
        }
    }

    attrs
}

/// Generate an HTML `<span>` element with inline CSS styles.
pub fn span_inline(
    text: &str,
    language: Option<Language>,
    scope: &str,
    theme: Option<&Theme>,
    italic: bool,
    include_highlights: bool,
) -> String {
    let escaped = escape(text);
    let attrs = span_inline_attrs(language, scope, theme, italic, include_highlights);

    format!("{}{}</span>", open_span(&attrs), escaped)
}

/// Generate HTML attributes for a span with CSS class.
pub fn span_linked_attrs(scope: &str) -> String {
    let class = scope_to_class(scope);
    format!("class=\"{class}\"")
}

/// Generate an HTML `<span>` element with CSS class.
pub fn span_linked(text: &str, scope: &str) -> String {
    let escaped = escape(text);
    let class = scope_to_class(scope);
    format!("<span class=\"{class}\">{escaped}</span>")
}

/// Sanitize a theme name for use in CSS variable names.
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

/// Get the CSS text-decoration value from a `TextDecoration` struct.
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
/// Theme names in a stable order.
///
/// The map is unordered, so without this the emitted custom properties come out
/// in a different order on every process. `<pre class="...">` already sorts.
fn sorted_theme_names(themes: &std::collections::HashMap<String, Theme>) -> Vec<&str> {
    let mut names: Vec<&str> = themes.keys().map(String::as_str).collect();
    names.sort_unstable();
    names
}

/// The default theme's colors are already inline, so only the style properties a
/// sibling theme could override are emitted as variables.
fn push_default_theme_css_vars(
    css_vars: &mut Vec<String>,
    css_variable_prefix: &str,
    theme_name: &str,
    style: &Style,
) {
    let sanitized = sanitize_theme_name(theme_name);

    let font_style = if style.italic { "italic" } else { "normal" };
    css_vars.push(format!(
        "{css_variable_prefix}-{sanitized}-font-style:{font_style};"
    ));

    let font_weight = if style.bold { "bold" } else { "normal" };
    css_vars.push(format!(
        "{css_variable_prefix}-{sanitized}-font-weight:{font_weight};"
    ));

    css_vars.push(format!(
        "{}-{}-text-decoration:{};",
        css_variable_prefix,
        sanitized,
        text_decoration(&style.text_decoration)
    ));
}

fn push_theme_css_vars(
    css_vars: &mut Vec<String>,
    css_variable_prefix: &str,
    theme_name: &str,
    style: &Style,
) {
    let sanitized = sanitize_theme_name(theme_name);

    if let Some(fg) = &style.fg {
        css_vars.push(format!("{css_variable_prefix}-{sanitized}:{fg};"));
    }
    if let Some(bg) = &style.bg {
        css_vars.push(format!("{css_variable_prefix}-{sanitized}-bg:{bg};"));
    }

    let font_style = if style.italic { "italic" } else { "normal" };
    css_vars.push(format!(
        "{css_variable_prefix}-{sanitized}-font-style:{font_style};"
    ));

    let font_weight = if style.bold { "bold" } else { "normal" };
    css_vars.push(format!(
        "{css_variable_prefix}-{sanitized}-font-weight:{font_weight};"
    ));

    css_vars.push(format!(
        "{}-{}-text-decoration:{};",
        css_variable_prefix,
        sanitized,
        text_decoration(&style.text_decoration)
    ));
}

fn push_light_dark_inline_styles(
    inline_styles: &mut Vec<String>,
    light_style: &Style,
    dark_style: &Style,
    italic: bool,
) {
    if let (Some(light_fg), Some(dark_fg)) = (&light_style.fg, &dark_style.fg) {
        inline_styles.push(format!("color: light-dark({light_fg}, {dark_fg});"));
    }
    if let (Some(light_bg), Some(dark_bg)) = (&light_style.bg, &dark_style.bg) {
        inline_styles.push(format!(
            "background-color: light-dark({light_bg}, {dark_bg});"
        ));
    }

    let light_weight = if light_style.bold { "bold" } else { "normal" };
    let dark_weight = if dark_style.bold { "bold" } else { "normal" };
    inline_styles.push(format!(
        "font-weight: light-dark({light_weight}, {dark_weight});"
    ));

    if italic {
        let light_value = if light_style.italic {
            "italic"
        } else {
            "normal"
        };
        let dark_value = if dark_style.italic {
            "italic"
        } else {
            "normal"
        };
        inline_styles.push(format!(
            "font-style: light-dark({light_value}, {dark_value});"
        ));
    }

    let light_decoration = text_decoration(&light_style.text_decoration);
    let dark_decoration = text_decoration(&dark_style.text_decoration);
    inline_styles.push(format!(
        "text-decoration: light-dark({light_decoration}, {dark_decoration});"
    ));
}

fn push_default_inline_styles(inline_styles: &mut Vec<String>, style: &Style, italic: bool) {
    if let Some(fg) = &style.fg {
        inline_styles.push(format!("color:{fg};"));
    }
    if let Some(bg) = &style.bg {
        inline_styles.push(format!("background-color:{bg};"));
    }
    if style.bold {
        inline_styles.push("font-weight:bold;".to_string());
    }
    if italic && style.italic {
        inline_styles.push("font-style:italic;".to_string());
    }

    let decoration = text_decoration(&style.text_decoration);
    if decoration != "none" {
        inline_styles.push(format!("text-decoration:{decoration};"));
    }
}

fn push_other_theme_css_vars(
    css_vars: &mut Vec<String>,
    themes: &std::collections::HashMap<String, Theme>,
    specialized_scope: &str,
    css_variable_prefix: &str,
    skipped_theme: Option<&str>,
) {
    for theme_name in sorted_theme_names(themes) {
        if skipped_theme == Some(theme_name) {
            continue;
        }
        if let Some(style) = themes[theme_name].get_style(specialized_scope) {
            push_theme_css_vars(css_vars, css_variable_prefix, theme_name, style);
        }
    }
}

#[allow(clippy::too_many_arguments)]
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

    match default_theme {
        Some("light-dark()") => {
            push_light_dark_styles(&mut inline_styles, themes, &specialized_scope, italic);
        }
        Some(default_name) => push_named_default_styles(
            &mut inline_styles,
            &mut css_vars,
            themes,
            &specialized_scope,
            css_variable_prefix,
            default_name,
            italic,
        ),
        None => push_other_theme_css_vars(
            &mut css_vars,
            themes,
            &specialized_scope,
            css_variable_prefix,
            None,
        ),
    }

    if inline_styles.is_empty() && css_vars.is_empty() {
        return String::new();
    }

    render_style_attrs(&inline_styles, &css_vars, scope, include_highlights)
}

/// `light-dark()` takes its inline styles from the two themes named `light` and
/// `dark`, and contributes no CSS variables.
fn push_light_dark_styles(
    inline_styles: &mut Vec<String>,
    themes: &std::collections::HashMap<String, Theme>,
    specialized_scope: &str,
    italic: bool,
) {
    let (Some(light_theme), Some(dark_theme)) = (themes.get("light"), themes.get("dark")) else {
        return;
    };
    let (Some(light_style), Some(dark_style)) = (
        light_theme.get_style(specialized_scope),
        dark_theme.get_style(specialized_scope),
    ) else {
        return;
    };

    push_light_dark_inline_styles(inline_styles, light_style, dark_style, italic);
}

/// A named default theme is written inline, and every other theme becomes a CSS
/// variable.
fn push_named_default_styles(
    inline_styles: &mut Vec<String>,
    css_vars: &mut Vec<String>,
    themes: &std::collections::HashMap<String, Theme>,
    specialized_scope: &str,
    css_variable_prefix: &str,
    default_name: &str,
    italic: bool,
) {
    let Some(default_theme_obj) = themes.get(default_name) else {
        return;
    };

    if let Some(style) = default_theme_obj.get_style(specialized_scope) {
        push_default_inline_styles(inline_styles, style, italic);
        push_default_theme_css_vars(css_vars, css_variable_prefix, default_name, style);
    }

    push_other_theme_css_vars(
        css_vars,
        themes,
        specialized_scope,
        css_variable_prefix,
        Some(default_name),
    );
}

fn render_style_attrs(
    inline_styles: &[String],
    css_vars: &[String],
    scope: &str,
    include_highlights: bool,
) -> String {
    let mut attrs = String::new();
    if include_highlights {
        let _ = write!(attrs, "data-highlight=\"{scope}\" ");
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

    let attrs = span_multi_themes_attrs(
        scope,
        language,
        themes,
        default_theme,
        css_variable_prefix,
        italic,
        include_highlights,
    );

    format!("{}{}</span>", open_span(&attrs), escaped)
}

/// Escape text for safe HTML output.
pub fn escape(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut buf = String::with_capacity(text.len() + text.len() / 10);
    let mut last = 0;

    for (i, &b) in bytes.iter().enumerate() {
        let replacement = match b {
            b'&' => "&amp;",
            b'<' => "&lt;",
            b'>' => "&gt;",
            b'"' => "&quot;",
            b'\'' => "&#39;",
            _ => continue,
        };
        buf.push_str(&text[last..i]);
        buf.push_str(replacement);
        last = i + 1;
    }

    if last == 0 {
        return text.to_string();
    }

    buf.push_str(&text[last..]);
    buf
}

/// Escape braces for framework compatibility.
pub fn escape_braces(text: &str) -> String {
    text.replace('{', "&lbrace;").replace('}', "&rbrace;")
}

/// Wrap content in a line div with optional class and style attributes.
pub fn wrap_line(
    line_number: usize,
    content: &str,
    class_suffix: Option<&str>,
    style: Option<&str>,
) -> String {
    let class_attr = match class_suffix {
        Some(suffix) => format!("l-line{suffix}"),
        None => "l-line".to_string(),
    };

    match style {
        Some(s) => format!(
            "<div class=\"{class_attr}\" style=\"{s}\" data-line=\"{line_number}\">{content}</div>"
        ),
        None => format!("<div class=\"{class_attr}\" data-line=\"{line_number}\">{content}</div>"),
    }
}

/// Map tree-sitter scope to CSS class name.
pub fn scope_to_class(scope: &str) -> String {
    crate::highlights::HIGHLIGHT_NAMES
        .iter()
        .position(|&s| s == scope)
        .and_then(|idx| crate::highlights::CLASSES.get(idx))
        .map_or_else(|| "l-text".to_string(), |class| format!("l-{class}"))
}

/// Generate an opening `<pre>` tag with optional class and theme styles.
pub fn open_pre_tag(
    output: &mut dyn Write,
    pre_class: Option<&str>,
    theme: Option<&Theme>,
) -> io::Result<()> {
    let class = if let Some(pre_class) = pre_class {
        format!("lumis {pre_class}")
    } else {
        "lumis".to_string()
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

/// Generate an opening `<pre>` tag with classes and styles for multiple themes.
pub fn open_multi_themes_pre_tag(
    output: &mut dyn Write,
    pre_class: Option<&str>,
    themes: &std::collections::HashMap<String, Theme>,
    default_theme: Option<&str>,
    css_variable_prefix: &str,
) -> io::Result<()> {
    let classes = multi_themes_pre_classes(pre_class, themes);
    let style = multi_themes_pre_style(themes, default_theme, css_variable_prefix);

    write!(output, "<pre class=\"{classes}\"")?;
    if !style.is_empty() {
        write!(output, " style=\"{style}\"")?;
    }
    write!(output, ">")
}

fn multi_themes_pre_classes(
    pre_class: Option<&str>,
    themes: &std::collections::HashMap<String, Theme>,
) -> String {
    let mut classes = vec!["lumis".to_string(), "lumis-themes".to_string()];

    if let Some(pre_class) = pre_class {
        classes.push(pre_class.to_string());
    }

    classes.extend(sorted_theme_names(themes).into_iter().map(str::to_string));

    classes.join(" ")
}

fn push_normal_theme_vars(
    styles: &mut Vec<String>,
    css_variable_prefix: &str,
    theme_name: &str,
    theme: &Theme,
) {
    let sanitized = sanitize_theme_name(theme_name);
    if let Some(fg) = theme.fg() {
        styles.push(format!("{css_variable_prefix}-{sanitized}:{fg};"));
    }
    if let Some(bg) = theme.bg() {
        styles.push(format!("{css_variable_prefix}-{sanitized}-bg:{bg};"));
    }
}

fn multi_themes_pre_style(
    themes: &std::collections::HashMap<String, Theme>,
    default_theme: Option<&str>,
    css_variable_prefix: &str,
) -> String {
    let mut styles = Vec::new();

    match default_theme {
        Some("light-dark()") => {
            if let (Some(light), Some(dark)) = (themes.get("light"), themes.get("dark")) {
                let light_fg = light.fg().unwrap_or("#000000");
                let light_bg = light.bg().unwrap_or("#ffffff");
                let dark_fg = dark.fg().unwrap_or("#ffffff");
                let dark_bg = dark.bg().unwrap_or("#000000");

                styles.push(format!("color: light-dark({light_fg}, {dark_fg});"));
                styles.push(format!(
                    "background-color: light-dark({light_bg}, {dark_bg});"
                ));
            }
        }
        Some(default_name) => {
            if let Some(default_theme) = themes.get(default_name) {
                if let Some(fg) = default_theme.fg() {
                    styles.push(format!("color:{fg};"));
                }
                if let Some(bg) = default_theme.bg() {
                    styles.push(format!("background-color:{bg};"));
                }
            }

            for theme_name in sorted_theme_names(themes) {
                if theme_name != default_name {
                    push_normal_theme_vars(
                        &mut styles,
                        css_variable_prefix,
                        theme_name,
                        &themes[theme_name],
                    );
                }
            }
        }
        None => {
            for theme_name in sorted_theme_names(themes) {
                push_normal_theme_vars(
                    &mut styles,
                    css_variable_prefix,
                    theme_name,
                    &themes[theme_name],
                );
            }
        }
    }

    styles.join(" ")
}

/// Generate an opening `<code>` tag with language class.
pub fn open_code_tag(output: &mut dyn Write, lang: &Language) -> io::Result<()> {
    write!(
        output,
        "<code class=\"language-{}\" translate=\"no\" tabindex=\"0\">",
        lang.id_name()
    )
}

/// Generate closing `</code>` tag.
pub fn close_code_tag(output: &mut dyn Write) -> io::Result<()> {
    output.write_all(b"</code>")
}

/// Generate closing `</pre>` tag.
pub fn close_pre_tag(output: &mut dyn Write) -> io::Result<()> {
    output.write_all(b"</pre>")
}

/// Generate closing `</code></pre>` tags.
pub fn closing_tags(output: &mut dyn Write) -> io::Result<()> {
    close_code_tag(output)?;
    close_pre_tag(output)
}

/// Split a fragment across lines, appending to the current line and starting new ones at `\n`.
pub fn append_fragment(lines: &mut Vec<String>, fragment: &str) {
    let mut parts = fragment.split('\n').peekable();

    while let Some(part) = parts.next() {
        if let Some(current) = lines.last_mut() {
            current.push_str(part);
        }

        if parts.peek().is_some() {
            lines.push(String::new());
        }
    }
}

/// Escape text for use in HTML span content.
pub fn escape_fragment(text: &str) -> String {
    escape(text)
}

fn open_span(attrs: &str) -> String {
    if attrs.is_empty() {
        "<span>".to_string()
    } else {
        format!("<span {attrs}>")
    }
}

/// Render highlight events into HTML lines, reopening active spans at line boundaries.
pub fn render_lines_from_events<T, F>(
    source: &str,
    events: &[crate::events::HighlightEvent<'_, T>],
    span_attrs: F,
) -> Vec<String>
where
    F: Fn(usize, &str) -> String,
{
    let mut lines = vec![String::new()];
    let mut stack: Vec<(usize, String)> = Vec::new();

    for event in events {
        match event {
            crate::events::HighlightEvent::Start {
                scope_index,
                language,
            } => {
                let attrs = span_attrs(*scope_index, language);
                append_fragment(&mut lines, &open_span(&attrs));
                stack.push((*scope_index, language.clone()));
            }
            crate::events::HighlightEvent::End => {
                if stack.pop().is_some() {
                    append_fragment(&mut lines, "</span>");
                }
            }
            crate::events::HighlightEvent::Source { start, end } => {
                let s = (*start).min(source.len());
                let e = (*end).min(source.len()).max(s);
                render_source_event(&mut lines, &source[s..e], &stack, &span_attrs);
            }
            crate::events::HighlightEvent::AnnotationStart { .. }
            | crate::events::HighlightEvent::AnnotationEnd => {}
        }
    }

    while stack.pop().is_some() {
        append_fragment(&mut lines, "</span>");
    }

    lines
}

fn render_source_event<F>(
    lines: &mut Vec<String>,
    text: &str,
    stack: &[(usize, String)],
    span_attrs: &F,
) where
    F: Fn(usize, &str) -> String,
{
    let mut remaining = text;

    loop {
        if let Some(newline_index) = remaining.find('\n') {
            let fragment = &remaining[..newline_index];
            append_fragment(lines, &escape_fragment(fragment));
            close_open_spans(lines, stack.len());
            lines.push(String::new());
            reopen_spans(lines, stack, span_attrs);
            remaining = &remaining[newline_index + 1..];
        } else {
            append_fragment(lines, &escape_fragment(remaining));
            break;
        }
    }
}

fn close_open_spans(lines: &mut Vec<String>, len: usize) {
    for _ in 0..len {
        append_fragment(lines, "</span>");
    }
}

fn reopen_spans<F>(lines: &mut Vec<String>, stack: &[(usize, String)], span_attrs: &F)
where
    F: Fn(usize, &str) -> String,
{
    for (scope_index, language) in stack {
        let attrs = span_attrs(*scope_index, language);
        append_fragment(lines, &open_span(&attrs));
    }
}

/// Render highlight events into HTML lines, calling `attribute_callback` for each highlight span.
///
/// This is a simplified version of the vendored `HtmlRenderer` that works with
/// pre-computed `HighlightEvent` slices instead of tree-sitter iterators.
pub fn render_events<T, F>(
    source: &str,
    events: &[crate::events::HighlightEvent<'_, T>],
    attribute_callback: &F,
) -> (Vec<u8>, Vec<u32>)
where
    F: Fn(usize, &str, &mut Vec<u8>),
{
    let source = source.as_bytes();
    let mut html = Vec::new();
    let mut line_offsets = vec![0u32];
    let mut highlight_stack: Vec<(usize, String)> = Vec::new();

    for event in events {
        match event {
            crate::events::HighlightEvent::Start {
                scope_index,
                language,
            } => {
                let mut attrs = Vec::new();
                attribute_callback(*scope_index, language, &mut attrs);
                if attrs.is_empty() {
                    html.extend_from_slice(b"<span>");
                } else {
                    html.extend_from_slice(b"<span ");
                    html.extend_from_slice(&attrs);
                    html.push(b'>');
                }
                highlight_stack.push((*scope_index, language.clone()));
            }
            crate::events::HighlightEvent::End => {
                html.extend_from_slice(b"</span>");
                highlight_stack.pop();
            }
            crate::events::HighlightEvent::Source { start, end } => {
                let s = (*start).min(source.len());
                let e = (*end).min(source.len()).max(s);
                let text = &source[s..e];
                // Process character by character, tracking line boundaries
                for &byte in text {
                    if byte == b'\n' {
                        html.push(b'\n');
                        // Close all open spans for line boundary
                        // (they'll be reopened on the next line by the caller
                        // if needed — but in this simplified model we just track offsets)
                        line_offsets.push(html.len() as u32);
                    } else {
                        // HTML-escape individual characters
                        match byte {
                            b'&' => html.extend_from_slice(b"&amp;"),
                            b'<' => html.extend_from_slice(b"&lt;"),
                            b'>' => html.extend_from_slice(b"&gt;"),
                            b'"' => html.extend_from_slice(b"&quot;"),
                            b'\'' => html.extend_from_slice(b"&#39;"),
                            _ => html.push(byte),
                        }
                    }
                }
            }
            crate::events::HighlightEvent::AnnotationStart { .. }
            | crate::events::HighlightEvent::AnnotationEnd => {}
        }
    }

    (html, line_offsets)
}

/// Iterator over rendered HTML lines.
pub fn lines_from_offsets<'a>(
    html: &'a [u8],
    line_offsets: &'a [u32],
) -> impl Iterator<Item = &'a str> + 'a {
    line_offsets
        .windows(2)
        .map(move |w| {
            let start = w[0] as usize;
            let end = w[1] as usize;
            std::str::from_utf8(&html[start..end]).unwrap_or("")
        })
        .chain(std::iter::once({
            let last_offset = *line_offsets.last().unwrap_or(&0) as usize;
            std::str::from_utf8(&html[last_offset..]).unwrap_or("")
        }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_str_eq;

    fn attr_value<'a>(tag: &'a str, attr: &str) -> &'a str {
        let marker = format!(r#"{attr}=""#);
        let start = tag.find(&marker).expect("missing attribute") + marker.len();
        let end = tag[start..].find('"').expect("unterminated attribute");
        &tag[start..start + end]
    }

    fn assert_classes(tag: &str, expected: &[&str]) {
        let classes: std::collections::HashSet<&str> =
            attr_value(tag, "class").split(' ').collect();

        assert_eq!(classes.len(), expected.len());
        for class in expected {
            assert!(classes.contains(class), "missing class {class:?} in {tag}");
        }
    }

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
    fn test_span_inline_without_attributes() {
        assert_eq!(
            span_inline("<b>", None, "string", None, false, false),
            "<span>&lt;b&gt;</span>"
        );
    }

    #[test]
    fn test_span_multi_themes_without_attributes() {
        assert_eq!(
            span_multi_themes(
                "<b>",
                "string",
                None,
                &std::collections::HashMap::new(),
                None,
                "--lumis",
                false,
                false,
            ),
            "<span>&lt;b&gt;</span>"
        );
    }

    #[test]
    fn test_escape_braces_only() {
        assert_eq!(escape_braces("fn() {}"), "fn() &lbrace;&rbrace;");
    }

    #[test]
    fn test_scope_to_class_keyword_conditional() {
        assert_eq!(
            scope_to_class("keyword.conditional"),
            "l-keyword-conditional"
        );
    }

    #[test]
    fn test_scope_to_class_unknown_scope() {
        assert_eq!(scope_to_class("unknown.scope.name"), "l-text");
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
    fn test_open_multi_themes_pre_tag_with_default_theme() {
        let mut themes = std::collections::HashMap::new();
        themes.insert(
            "light".to_string(),
            crate::themes::get("github_light").unwrap(),
        );

        let mut output = Vec::new();
        open_multi_themes_pre_tag(
            &mut output,
            Some("custom-pre"),
            &themes,
            Some("light"),
            "--lumis",
        )
        .unwrap();

        let html = String::from_utf8(output).unwrap();

        assert_classes(&html, &["lumis", "lumis-themes", "custom-pre", "light"]);
        assert_eq!(
            attr_value(&html, "style"),
            "color:#1f2328; background-color:#ffffff;"
        );
    }

    #[test]
    fn test_open_multi_themes_pre_tag_with_light_dark() {
        let mut themes = std::collections::HashMap::new();
        themes.insert(
            "light".to_string(),
            crate::themes::get("catppuccin_latte").unwrap(),
        );
        themes.insert(
            "dark".to_string(),
            crate::themes::get("catppuccin_mocha").unwrap(),
        );

        let mut output = Vec::new();
        open_multi_themes_pre_tag(
            &mut output,
            Some("custom-pre"),
            &themes,
            Some("light-dark()"),
            "--lumis",
        )
        .unwrap();

        let html = String::from_utf8(output).unwrap();

        assert_classes(
            &html,
            &["lumis", "lumis-themes", "custom-pre", "light", "dark"],
        );
        assert_eq!(
            attr_value(&html, "style"),
            "color: light-dark(#4c4f69, #cdd6f4); background-color: light-dark(#eff1f5, #1e1e2e);"
        );
    }
}

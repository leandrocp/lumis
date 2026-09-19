//! HTML generation helpers for creating custom HTML formatters.
//!
//! These helpers work with language names as strings, making them independent of tree-sitter.

use crate::decorations::{
    compose_line_decorations, rainbow_scope_index, Decoration, LineSelection,
};
use crate::events::HighlightEvent;
use crate::languages::Language;
use crate::themes::{Style, TextDecoration, Theme, UnderlineStyle};
use std::fmt::Write as _;
use std::io::{self, Write};
use std::ops::RangeInclusive;

#[doc(hidden)]
pub use crate::decorations::SteppedLineRange;

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
        let _ = write!(attrs, "data-highlight=\"{}\"", escape_attr(scope));
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
                let _ = write!(attrs, "style=\"{}\"", escape_attr(&css));
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
        let _ = write!(attrs, "data-highlight=\"{}\" ", escape_attr(scope));
    }

    let mut style = String::new();
    if !inline_styles.is_empty() {
        style.push_str(&inline_styles.join(" "));
    }
    if !css_vars.is_empty() {
        if !inline_styles.is_empty() {
            style.push(' ');
        }
        style.push_str(&css_vars.join(" "));
    }

    let _ = write!(attrs, "style=\"{}\"", escape_attr(&style));

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

/// The HTML entity a byte has to be written as, if any.
const fn html_entity(byte: u8) -> Option<&'static str> {
    match byte {
        b'&' => Some("&amp;"),
        b'<' => Some("&lt;"),
        b'>' => Some("&gt;"),
        b'"' => Some("&quot;"),
        b'\'' => Some("&#39;"),
        _ => None,
    }
}

/// Escape text for safe HTML output.
pub fn escape(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut buf = String::with_capacity(text.len() + text.len() / 10);
    let mut last = 0;

    for (i, &b) in bytes.iter().enumerate() {
        let Some(replacement) = html_entity(b) else {
            continue;
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

/// Escape text straight into `output`, without building a `String` first.
///
/// [`escape`] allocates once per call, which on a highlighted document is once
/// per token. The built-in formatters write to a buffer they already own, so
/// they take this instead.
pub(crate) fn write_escaped(output: &mut dyn Write, text: &str) -> io::Result<()> {
    let mut last = 0;

    for (index, &byte) in text.as_bytes().iter().enumerate() {
        let Some(replacement) = html_entity(byte) else {
            continue;
        };
        output.write_all(&text.as_bytes()[last..index])?;
        output.write_all(replacement.as_bytes())?;
        last = index + 1;
    }

    output.write_all(&text.as_bytes()[last..])
}

/// Escape a value for use inside a double-quoted HTML attribute.
///
/// Attribute values reach the formatters from two places outside the binary: a
/// caller-supplied class or style (`pre_class`, `highlight_lines`), and a theme
/// loaded with [`crate::themes::from_json`]. Neither is a generated constant, so
/// both are escaped here rather than interpolated raw.
///
/// The escape set is the same as [`escape`], which is what `escapeAttr` in
/// `packages/javascript/lumis/src/formatter/html.ts` covers, so the two runtimes
/// answer the same for the same input. Escaping is safe for CSS in an attribute
/// because the HTML parser decodes the entity before the CSS parser sees it, so
/// `font-family: 'Fira Code'` survives the round trip.
pub fn escape_attr(value: &str) -> String {
    escape(value)
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
    let mut line = Vec::with_capacity(content.len() + 48);
    let _ = LineTag::new(class_suffix, style, false, false, None).write(&mut line, line_number);
    line.extend_from_slice(content.as_bytes());
    line.extend_from_slice(b"</div>");

    String::from_utf8(line).expect("the tag and its content are both UTF-8")
}

/// The class the gutter element carries, for a stylesheet to hang a column off.
///
/// The number is written out rather than left to `content: attr(data-line)`
/// because a formatter that cannot reach a stylesheet — `terminal` — has to show
/// the same thing, and because generated content is not in the document a reader
/// can inspect. It is `aria-hidden`, so a screen reader is not read a number
/// before every line.
///
/// Crate-visible rather than private so the CSS builder writes the same two
/// names this module does, instead of deriving its own from the scope.
pub(crate) const LINE_NUMBER_CLASS: &str = "l-line-number";
pub(crate) const HIGHLIGHTED_LINE_NUMBER_CLASS: &str = "l-line-number-highlighted";

/// Everything in a line's opening tag that does not change from line to line.
///
/// A document's lines differ only in their number and in whether they are
/// highlighted, so the class and style attributes — which cost an escape each —
/// are assembled once rather than once per line.
struct LineTag {
    open: String,
    gutter_open: Option<String>,
}

impl LineTag {
    fn new(
        class_suffix: Option<&str>,
        style: Option<&str>,
        numbered: bool,
        highlighted: bool,
        gutter_attrs: Option<&str>,
    ) -> Self {
        let mut open = String::from("<div class=\"");
        match class_suffix {
            Some(suffix) => open.push_str(&escape_attr(&format!("l-line{suffix}"))),
            None => open.push_str("l-line"),
        }
        open.push('"');

        if let Some(style) = style {
            let _ = write!(open, " style=\"{}\"", escape_attr(style));
        }

        open.push_str(" data-line=\"");
        let gutter_open = numbered.then(|| {
            let mut gutter = format!("<span class=\"{LINE_NUMBER_CLASS}");
            if highlighted {
                let _ = write!(gutter, " {HIGHLIGHTED_LINE_NUMBER_CLASS}");
            }
            gutter.push('"');
            if let Some(attrs) = gutter_attrs.filter(|attrs| !attrs.is_empty()) {
                gutter.push(' ');
                gutter.push_str(attrs);
            }
            gutter.push_str(" aria-hidden=\"true\">");
            gutter
        });

        Self { open, gutter_open }
    }

    fn write(&self, output: &mut dyn Write, line_number: usize) -> io::Result<()> {
        output.write_all(self.open.as_bytes())?;
        write!(output, "{line_number}\">")?;

        if let Some(gutter_open) = &self.gutter_open {
            write!(output, "{gutter_open}{line_number}</span>")?;
        }

        Ok(())
    }
}

/// Whether `line_number` falls inside any of `lines`.
///
/// Lines are 1-based, matching the `data-line` attribute [`wrap_line`] writes.
pub fn line_is_highlighted(lines: &[RangeInclusive<usize>], line_number: usize) -> bool {
    lines.iter().any(|range| range.contains(&line_number))
}

/// The CSS class a highlighted line carries, or `None` when the line is not highlighted.
///
/// `class` wins over `default_class`, so a formatter can offer a caller-supplied
/// class over its own.
pub fn highlight_line_class<'a>(
    lines: &[RangeInclusive<usize>],
    line_number: usize,
    class: Option<&'a str>,
    default_class: Option<&'a str>,
) -> Option<&'a str> {
    if line_is_highlighted(lines, line_number) {
        class.or(default_class)
    } else {
        None
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

/// What an HTML attribute carries.
///
/// HTML writes an attribute two ways, `name="value"` and a bare `name` for the
/// boolean ones such as `hidden` or `inert`, and both have to survive a merge.
/// [`AttrValue::Absent`] is the third state that merging needs: it is how an
/// authored attribute removes one Lumis generated, so `translate` can be taken
/// off `<code>` rather than only overwritten.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AttrValue {
    /// Rendered as `name="value"`, with the value escaped.
    Value(String),
    /// Rendered as a bare `name`.
    Present,
    /// Not rendered, and removes a generated attribute of the same name.
    Absent,
}

impl AttrValue {
    /// The string this renders as, or `None` when it renders bare or not at all.
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Value(value) => Some(value),
            Self::Present | Self::Absent => None,
        }
    }
}

impl From<String> for AttrValue {
    fn from(value: String) -> Self {
        Self::Value(value)
    }
}

impl From<&str> for AttrValue {
    fn from(value: &str) -> Self {
        Self::Value(value.to_string())
    }
}

impl From<bool> for AttrValue {
    fn from(value: bool) -> Self {
        if value {
            Self::Present
        } else {
            Self::Absent
        }
    }
}

/// Ordered HTML attribute name/value pairs.
///
/// Values stay unescaped until the opening tag is rendered. Keeping the
/// structured form lets callers merge attributes without parsing HTML and lets
/// Lumis escape every value exactly once.
pub type HtmlAttrs = Vec<(String, AttrValue)>;

/// Whether `name` is a name HTML can carry, per the attribute-name production.
///
/// Escaping a name is not an option, which is why this exists: a space needs no
/// escaping and splits one name into two attributes, so `x onclick=alert(1)`
/// would render an event handler no matter how the value was treated.
#[must_use]
pub fn is_valid_attr_name(name: &str) -> bool {
    !name.is_empty()
        && !name.chars().any(|character| {
            character.is_whitespace()
                || character.is_control()
                || matches!(character, '"' | '\'' | '>' | '/' | '=')
        })
}

fn merge_classes(current: &str, additional: &str) -> String {
    let mut classes: Vec<&str> = Vec::new();

    for candidate in current
        .split_ascii_whitespace()
        .chain(additional.split_ascii_whitespace())
    {
        if !classes.contains(&candidate) {
            classes.push(candidate);
        }
    }

    classes.join(" ")
}

fn append_style(current: &str, additional: &str) -> String {
    let current = current.trim();
    let additional = additional.trim();

    match (current.is_empty(), additional.is_empty()) {
        (true, true) => String::new(),
        (true, false) => additional.to_string(),
        (false, true) => current.to_string(),
        (false, false) if current.ends_with(';') => format!("{current} {additional}"),
        (false, false) => format!("{current}; {additional}"),
    }
}

/// `class` unions, `style` appends, and every other name replaces.
fn merged_value(name: &str, current: Option<&AttrValue>, authored: &AttrValue) -> AttrValue {
    let Some(authored) = authored.as_str() else {
        return AttrValue::Present;
    };
    let current = current.and_then(AttrValue::as_str).unwrap_or_default();

    if name.eq_ignore_ascii_case("class") {
        AttrValue::Value(merge_classes(current, authored))
    } else if name.eq_ignore_ascii_case("style") {
        AttrValue::Value(append_style(current, authored))
    } else {
        AttrValue::Value(authored.to_string())
    }
}

fn merge_attrs(mut generated: HtmlAttrs, authored: &[(String, AttrValue)]) -> HtmlAttrs {
    for (name, value) in authored {
        let existing = generated
            .iter()
            .position(|(candidate, _)| candidate.eq_ignore_ascii_case(name));

        if matches!(value, AttrValue::Absent) {
            if let Some(index) = existing {
                generated.remove(index);
            }
            continue;
        }

        let merged = merged_value(name, existing.map(|index| &generated[index].1), value);

        match existing {
            Some(index) => generated[index].1 = merged,
            // An authored `class=""` on a tag that generated none adds nothing.
            None if merged.as_str().is_some_and(str::is_empty) => {}
            None => generated.push((name.clone(), merged)),
        }
    }

    generated
}

/// Generate an opening tag from attributes, escaping every value.
///
/// This is what the `*_attrs` helpers are built for: merge their result with
/// your own attributes, then write the whole thing here rather than assembling
/// the string and remembering to escape it.
///
/// # Errors
///
/// Returns [`io::ErrorKind::InvalidInput`] for an attribute name HTML cannot
/// carry, because rendering one would let it break out of the tag, and the
/// underlying error if `output` fails.
pub fn open_tag(output: &mut dyn Write, name: &str, attrs: &HtmlAttrs) -> io::Result<()> {
    write!(output, "<{name}")?;

    for (attr_name, value) in attrs {
        if !is_valid_attr_name(attr_name) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("invalid HTML attribute name: {attr_name:?}"),
            ));
        }

        match value {
            AttrValue::Value(value) => write!(output, " {attr_name}=\"{}\"", escape_attr(value))?,
            AttrValue::Present => write!(output, " {attr_name}")?,
            AttrValue::Absent => {}
        }
    }

    output.write_all(b">")
}

/// Build attributes for the `<pre>` tag used by inline and linked HTML.
///
/// Generated values come first. Authored `class` values are unioned with the
/// `lumis` and `pre_class` classes, authored `style` is appended to the theme
/// style, and every other authored value replaces a generated default.
pub fn pre_attrs(
    pre_class: Option<&str>,
    theme: Option<&Theme>,
    attrs: &[(String, AttrValue)],
) -> HtmlAttrs {
    let class = pre_class.map_or_else(|| "lumis".to_string(), |value| format!("lumis {value}"));
    let mut generated = vec![("class".to_string(), AttrValue::Value(class))];
    if let Some(style) = theme.and_then(|theme| theme.pre_style(" ")) {
        generated.push(("style".to_string(), AttrValue::Value(style)));
    }

    merge_attrs(generated, attrs)
}

pub(crate) fn write_pre_tag(
    output: &mut dyn Write,
    pre_class: Option<&str>,
    theme: Option<&Theme>,
    attrs: &[(String, AttrValue)],
) -> io::Result<()> {
    open_tag(output, "pre", &pre_attrs(pre_class, theme, attrs))
}

/// Generate an opening `<pre>` tag with optional class and theme styles.
pub fn open_pre_tag(
    output: &mut dyn Write,
    pre_class: Option<&str>,
    theme: Option<&Theme>,
) -> io::Result<()> {
    write_pre_tag(output, pre_class, theme, &[])
}

/// Build attributes for the multi-theme `<pre>` tag.
pub fn multi_themes_pre_attrs(
    pre_class: Option<&str>,
    themes: &std::collections::HashMap<String, Theme>,
    default_theme: Option<&str>,
    css_variable_prefix: &str,
    attrs: &[(String, AttrValue)],
) -> HtmlAttrs {
    let mut generated = vec![(
        "class".to_string(),
        AttrValue::Value(multi_themes_pre_classes(pre_class, themes)),
    )];
    let style = multi_themes_pre_style(themes, default_theme, css_variable_prefix);
    if !style.is_empty() {
        generated.push(("style".to_string(), AttrValue::Value(style)));
    }

    merge_attrs(generated, attrs)
}

pub(crate) fn write_multi_themes_pre_tag(
    output: &mut dyn Write,
    pre_class: Option<&str>,
    themes: &std::collections::HashMap<String, Theme>,
    default_theme: Option<&str>,
    css_variable_prefix: &str,
    attrs: &[(String, AttrValue)],
) -> io::Result<()> {
    open_tag(
        output,
        "pre",
        &multi_themes_pre_attrs(pre_class, themes, default_theme, css_variable_prefix, attrs),
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
    write_multi_themes_pre_tag(
        output,
        pre_class,
        themes,
        default_theme,
        css_variable_prefix,
        &[],
    )
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

    // A `pre_class` naming one of the themes would otherwise appear twice.
    merge_classes(&classes.join(" "), "")
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

/// Build attributes for the `<code>` tag used by every HTML formatter.
pub fn code_attrs(lang: &Language, attrs: &[(String, AttrValue)]) -> HtmlAttrs {
    merge_attrs(
        vec![
            (
                "class".to_string(),
                AttrValue::Value(format!("language-{}", lang.id_name())),
            ),
            ("translate".to_string(), AttrValue::Value("no".to_string())),
            ("tabindex".to_string(), AttrValue::Value("0".to_string())),
        ],
        attrs,
    )
}

pub(crate) fn write_code_tag(
    output: &mut dyn Write,
    lang: Language,
    attrs: &[(String, AttrValue)],
) -> io::Result<()> {
    open_tag(output, "code", &code_attrs(&lang, attrs))
}

/// Generate an opening `<code>` tag with language class.
pub fn open_code_tag(output: &mut dyn Write, lang: &Language) -> io::Result<()> {
    write_code_tag(output, *lang, &[])
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
#[deprecated(note = "use `escape(...)` instead")]
pub fn escape_fragment(text: &str) -> String {
    escape(text)
}

/// Generate an opening `<span>` tag carrying `attrs`, or a bare one when empty.
///
/// A scope a theme styles in no way still opens a `<span>`, so every one pairs
/// with the `</span>` an end event writes.
pub fn open_span(attrs: &str) -> String {
    if attrs.is_empty() {
        "<span>".to_string()
    } else {
        format!("<span {attrs}>")
    }
}

/// Render highlight events into HTML lines, reopening active spans at line boundaries.
///
/// Each line carries the exact `\n` or `\r\n` that ended it in `source`, after
/// any closing span tags. An unterminated final line has no terminator.
pub fn render_lines_from_events<T, F>(
    source: &str,
    events: &[HighlightEvent<'_, T>],
    span_attrs: F,
) -> Vec<String>
where
    F: Fn(usize, &str) -> String,
{
    let mut lines = Vec::new();
    let mut line = String::new();
    let decoration_language = events
        .iter()
        .find_map(HighlightEvent::language)
        .unwrap_or_default();

    write_line_events(
        &compose_line_decorations(source, events, &LineSelection::default()),
        source,
        decoration_language,
        |fragment| match fragment {
            LineFragment::OpenLine { .. } => {}
            LineFragment::Close(ending) => {
                line.push_str(ending);
                lines.push(std::mem::take(&mut line));
            }
            LineFragment::Text(text) => line.push_str(&escape(text)),
            LineFragment::SpanOpen(scope_index, language) => {
                line.push_str(&open_span(&span_attrs(scope_index, language)));
            }
            LineFragment::SpanClose => line.push_str("</span>"),
        },
    );

    lines
}

/// One step of a line-decorated event stream, with its text already sliced.
pub(crate) enum LineFragment<'a> {
    /// A line begins.
    OpenLine { number: usize, highlighted: bool },
    /// The current line ends with the source terminator, when it had one.
    Close(&'a str),
    /// Unescaped source text, never spanning a line boundary.
    Text(&'a str),
    /// A syntax or built-in decoration scope begins.
    SpanOpen(usize, &'a str),
    /// The innermost syntax or built-in decoration scope ends.
    SpanClose,
}

/// Walk a line-decorated stream, handing each step to `on_fragment`.
///
/// A source line's `\n` or `\r\n` is held until
/// [`Close`](LineFragment::Close), after all syntax spans have closed. This
/// keeps the terminator outside syntax markup while preserving the source
/// exactly. Caller annotations are skipped, which is what a built-in formatter
/// does with data it has never seen.
pub(crate) fn write_line_events<'a, T, F>(
    events: &'a [HighlightEvent<'_, T>],
    source: &'a str,
    decoration_language: &'a str,
    mut on_fragment: F,
) where
    F: FnMut(LineFragment<'a>),
{
    let mut ending = "";
    let mut decorations = Vec::new();

    for event in events {
        match event {
            HighlightEvent::DecorationStart { decoration } => {
                decorations.push(*decoration);
                match decoration {
                    Decoration::Line {
                        number,
                        highlighted,
                    } => {
                        ending = "";
                        on_fragment(LineFragment::OpenLine {
                            number: *number,
                            highlighted: *highlighted,
                        });
                    }
                    Decoration::RainbowBracket { depth } => on_fragment(LineFragment::SpanOpen(
                        rainbow_scope_index(*depth),
                        decoration_language,
                    )),
                }
            }
            HighlightEvent::DecorationEnd => match decorations.pop() {
                Some(Decoration::Line { .. }) => on_fragment(LineFragment::Close(ending)),
                Some(Decoration::RainbowBracket { .. }) => on_fragment(LineFragment::SpanClose),
                None => {}
            },
            HighlightEvent::Start {
                scope_index,
                language,
            } => on_fragment(LineFragment::SpanOpen(*scope_index, language)),
            HighlightEvent::End => on_fragment(LineFragment::SpanClose),
            HighlightEvent::Source { start, end } => {
                let text = source_slice(source, *start, *end);
                let (text, source_ending) = split_line_ending(text);
                ending = source_ending;
                on_fragment(LineFragment::Text(text));
            }
            HighlightEvent::AnnotationStart { .. } | HighlightEvent::AnnotationEnd => {}
        }
    }
}

fn split_line_ending(text: &str) -> (&str, &str) {
    let Some(content) = text.strip_suffix('\n') else {
        return (text, "");
    };

    match content.strip_suffix('\r') {
        Some(content) => (content, "\r\n"),
        None => (content, "\n"),
    }
}

/// How one render's lines are numbered, marked and styled.
///
/// The three HTML formatters differ only in what a highlighted line carries, so
/// this is what they hand [`write_html_lines`] to make the walk itself shared.
pub(crate) struct HtmlLines<'a> {
    /// Root language used by Lumis-owned syntax-like decorations.
    pub language: Language,
    /// Which lines the caller asked to highlight.
    pub selection: &'a LineSelection,
    /// Whether each line opens with a gutter carrying its number.
    pub numbered: bool,
    /// Extra attributes carried by a regular line-number gutter.
    pub line_number_attrs: Option<&'a str>,
    /// Extra attributes carried by a highlighted line-number gutter.
    pub highlighted_line_number_attrs: Option<&'a str>,
    /// The class suffix a highlighted line's `<div>` carries.
    pub highlighted_class: Option<&'a str>,
    /// The inline style a highlighted line's `<div>` carries.
    pub highlighted_style: Option<&'a str>,
}

/// Write a line-decorated stream as the `<div class="l-line">` blocks every
/// built-in HTML formatter emits.
///
/// [`write_line_events`] supplies the source terminator with each closing line,
/// and this writer places it immediately before `</div>` without inventing one.
///
/// Nothing here is recomputed per line. The two line tags are assembled once,
/// and a scope's attributes are resolved the first time it is seen rather than
/// again every time a line boundary reopens it — which on a long document is
/// once per open scope per line.
pub(crate) fn write_html_lines<T>(
    output: &mut dyn Write,
    source: &str,
    events: &[HighlightEvent<'_, T>],
    lines: &HtmlLines<'_>,
    span_attrs: &dyn Fn(usize, &str) -> String,
) -> io::Result<()> {
    let plain_tag = LineTag::new(None, None, lines.numbered, false, lines.line_number_attrs);
    let highlighted_tag = LineTag::new(
        lines.highlighted_class,
        lines.highlighted_style,
        lines.numbered,
        true,
        lines.highlighted_line_number_attrs,
    );
    let composed = compose_line_decorations(source, events, lines.selection);
    let mut attrs: std::collections::HashMap<(usize, &str), String> =
        std::collections::HashMap::new();
    let mut result = Ok(());

    write_line_events(&composed, source, lines.language.id_name(), |fragment| {
        if result.is_err() {
            return;
        }
        result = match fragment {
            LineFragment::OpenLine {
                number,
                highlighted,
            } => {
                let tag = if highlighted {
                    &highlighted_tag
                } else {
                    &plain_tag
                };
                tag.write(output, number)
            }
            LineFragment::Close(ending) => output
                .write_all(ending.as_bytes())
                .and_then(|()| output.write_all(b"</div>")),
            LineFragment::Text(text) => write_escaped(output, text),
            LineFragment::SpanOpen(scope_index, language) => {
                let attrs = attrs
                    .entry((scope_index, language))
                    .or_insert_with(|| span_attrs(scope_index, language));
                if attrs.is_empty() {
                    output.write_all(b"<span>")
                } else {
                    write!(output, "<span {attrs}>")
                }
            }
            LineFragment::SpanClose => output.write_all(b"</span>"),
        };
    });

    result
}

/// The largest slice of `source` fully inside `start..end`.
///
/// A formatter can build its own events rather than replaying the ones Lumis
/// handed it, so these offsets are caller data. Out of range is clamped, and an
/// offset landing inside a multi-byte character moves to the boundary that keeps
/// the slice smaller, because `&source[start..end]` would otherwise panic on a
/// range that split one. Reversed offsets give an empty slice.
fn source_slice(source: &str, start: usize, end: usize) -> &str {
    let start = ceil_char_boundary(source, start.min(source.len()));
    let end = floor_char_boundary(source, end.min(source.len())).max(start);

    &source[start..end]
}

fn floor_char_boundary(source: &str, mut index: usize) -> usize {
    while index > 0 && !source.is_char_boundary(index) {
        index -= 1;
    }
    index
}

fn ceil_char_boundary(source: &str, mut index: usize) -> usize {
    while index < source.len() && !source.is_char_boundary(index) {
        index += 1;
    }
    index
}

/// Render highlight events into HTML lines, calling `attribute_callback` for each highlight span.
///
/// This is a simplified version of the vendored `HtmlRenderer` that works with
/// pre-computed `HighlightEvent` slices instead of tree-sitter iterators.
///
/// Deprecated: it records line offsets without closing and reopening the spans
/// that cross a line boundary, so slicing the buffer at them yields lines whose
/// tags do not nest. [`render_lines_from_events`] does the same job correctly.
#[deprecated(note = "use `render_lines_from_events(...)` instead")]
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
            | crate::events::HighlightEvent::AnnotationEnd
            | crate::events::HighlightEvent::DecorationStart { .. }
            | crate::events::HighlightEvent::DecorationEnd => {}
        }
    }

    (html, line_offsets)
}

/// Iterator over rendered HTML lines.
///
/// Deprecated: only useful for slicing what [`render_events`] returns.
#[deprecated(note = "use `render_lines_from_events(...)` instead")]
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

    fn html_lines<T>(
        source: &str,
        events: &[HighlightEvent<'_, T>],
        selection: &LineSelection,
        numbered: bool,
        highlighted_class: Option<&str>,
    ) -> String {
        let mut output = Vec::new();
        let lines = HtmlLines {
            language: Language::PlainText,
            selection,
            numbered,
            line_number_attrs: None,
            highlighted_line_number_attrs: None,
            highlighted_class,
            highlighted_style: None,
        };
        write_html_lines(&mut output, source, events, &lines, &|_, _| String::new()).unwrap();

        String::from_utf8(output).expect("the formatter writes UTF-8")
    }

    #[test]
    fn test_escape_all_entities() {
        assert_eq!(escape("&<>\"'{}"), "&amp;&lt;&gt;&quot;&#39;{}");
    }

    /// A formatter can build its own events, so a `Source` range is caller data
    /// and `&source[start..end]` used to panic on one that split a character.
    #[test]
    fn test_source_slice_never_panics_on_caller_offsets() {
        let source = "éx";

        assert_eq!(source_slice(source, 0, 1), "", "end splits 'é'");
        assert_eq!(source_slice(source, 1, 2), "", "start splits 'é'");
        assert_eq!(source_slice(source, 1, 3), "x", "start splits 'é'");
        assert_eq!(source_slice(source, 0, 2), "é");
        assert_eq!(source_slice(source, 0, 3), "éx");
        assert_eq!(source_slice(source, 0, 99), "éx", "end past the source");
        assert_eq!(source_slice(source, 99, 99), "", "start past the source");
        assert_eq!(source_slice(source, 3, 0), "", "reversed");
    }

    #[test]
    fn test_render_lines_from_events_survives_a_split_character() {
        let events: [crate::events::HighlightEvent<'_>; 1] =
            [crate::events::HighlightEvent::Source { start: 0, end: 1 }];

        assert_eq!(
            render_lines_from_events("é", &events, |_, _| String::new()),
            [""]
        );
    }

    /// Lines are the only thing the helper and the built-in formatters split on,
    /// so both have to agree about where a line ends.
    #[test]
    fn render_lines_from_events_and_the_html_pass_split_alike() {
        for source in ["", "one", "one\n", "one\ntwo", "one\r\ntwo", "one\rtwo"] {
            let events: [HighlightEvent<'_, ()>; 1] = [HighlightEvent::Source {
                start: 0,
                end: source.len(),
            }];

            let lines = render_lines_from_events(source, &events, |_, _| String::new());
            let html = html_lines(source, &events, &LineSelection::default(), false, None);

            assert_str_eq!(
                html,
                lines
                    .iter()
                    .enumerate()
                    .map(|(index, line)| wrap_line(index + 1, line, None, None))
                    .collect::<String>()
            );
        }
    }

    #[test]
    fn html_lines_write_source_endings_after_the_syntax_spans() {
        let source = "a\r\nb";
        let events: [HighlightEvent<'_, ()>; 3] = [
            HighlightEvent::Start {
                scope_index: 0,
                language: "text".to_string(),
            },
            HighlightEvent::Source {
                start: 0,
                end: source.len(),
            },
            HighlightEvent::End,
        ];
        let mut html = Vec::new();

        write_html_lines(
            &mut html,
            source,
            &events,
            &HtmlLines {
                language: Language::PlainText,
                selection: &LineSelection::default(),
                numbered: false,
                line_number_attrs: None,
                highlighted_line_number_attrs: None,
                highlighted_class: None,
                highlighted_style: None,
            },
            &|_, _| "class=\"scope\"".to_string(),
        )
        .unwrap();

        assert_str_eq!(
            String::from_utf8(html).unwrap(),
            "<div class=\"l-line\" data-line=\"1\"><span class=\"scope\">a</span>\r\n</div><div class=\"l-line\" data-line=\"2\"><span class=\"scope\">b</span></div>"
        );
    }

    /// The gutter carries the same number `data-line` does, because both read it
    /// off the line's decoration.
    #[test]
    fn a_gutter_carries_the_number_data_line_does() {
        let source = "one\ntwo";
        let events = [HighlightEvent::<()>::Source {
            start: 0,
            end: source.len(),
        }];
        let selection = LineSelection::new(std::slice::from_ref(&(2..=2)), &[]);

        let html = html_lines(source, &events, &selection, true, Some(" l-highlighted"));

        assert_str_eq!(
            html,
            concat!(
                r#"<div class="l-line" data-line="1">"#,
                r#"<span class="l-line-number" aria-hidden="true">1</span>one"#,
                "\n</div>",
                r#"<div class="l-line l-highlighted" data-line="2">"#,
                r#"<span class="l-line-number l-line-number-highlighted" aria-hidden="true">2</span>two"#,
                // The last line is unterminated in the source, so it carries no
                // terminator here either.
                "</div>",
            )
        );
    }

    #[test]
    fn gutters_carry_their_theme_attributes() {
        let source = "one\ntwo";
        let events = [HighlightEvent::<()>::Source {
            start: 0,
            end: source.len(),
        }];
        let selection = LineSelection::new(std::slice::from_ref(&(2..=2)), &[]);
        let mut output = Vec::new();
        let lines = HtmlLines {
            language: Language::PlainText,
            selection: &selection,
            numbered: true,
            line_number_attrs: Some(r#"style="color:#111111;""#),
            highlighted_line_number_attrs: Some(r#"style="color:#eeeeee;""#),
            highlighted_class: None,
            highlighted_style: None,
        };

        write_html_lines(&mut output, source, &events, &lines, &|_, _| String::new()).unwrap();

        let html = String::from_utf8(output).unwrap();
        assert!(html.contains(
            r#"<span class="l-line-number" style="color:#111111;" aria-hidden="true">1</span>"#
        ));
        assert!(html.contains(
            r#"<span class="l-line-number l-line-number-highlighted" style="color:#eeeeee;" aria-hidden="true">2</span>"#
        ));
    }

    /// Without the option nothing is added, which is what keeps every existing
    /// fixture byte for byte what it was.
    #[test]
    fn no_gutter_without_line_numbers() {
        let events = [HighlightEvent::<()>::Source { start: 0, end: 1 }];

        let html = html_lines("a", &events, &LineSelection::default(), false, None);

        assert_str_eq!(html, "<div class=\"l-line\" data-line=\"1\">a</div>");
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

    /// A theme is data, so a colour can carry a quote out of a JSON file and
    /// close the attribute it is written into.
    fn quote_bearing_theme() -> Theme {
        crate::themes::from_json(
            r##"{
              "name": "quote-bearing",
              "appearance": "dark",
              "revision": "test",
              "highlights": {
                "normal": { "fg": "red\" onmouseover=\"alert(1)", "bg": "#000000" },
                "keyword": { "fg": "red\" onmouseover=\"alert(1)" }
              }
            }"##,
        )
        .unwrap()
    }

    #[test]
    fn test_escape_attr_matches_escape() {
        assert_eq!(escape_attr("&<>\"'"), "&amp;&lt;&gt;&quot;&#39;");
    }

    #[test]
    fn test_escape_attr_keeps_quoted_css_readable() {
        assert_eq!(
            escape_attr("font-family: 'Fira Code';"),
            "font-family: &#39;Fira Code&#39;;"
        );
    }

    #[test]
    fn test_open_pre_tag_escapes_caller_class() {
        let mut output = Vec::new();
        open_pre_tag(&mut output, Some(r#"x"><script>"#), None).unwrap();

        assert_str_eq!(
            String::from_utf8(output).unwrap(),
            r#"<pre class="lumis x&quot;&gt;&lt;script&gt;">"#
        );
    }

    #[test]
    fn test_open_pre_tag_escapes_theme_colors() {
        let mut output = Vec::new();
        open_pre_tag(&mut output, None, Some(&quote_bearing_theme())).unwrap();

        assert_str_eq!(
            String::from_utf8(output).unwrap(),
            r#"<pre class="lumis" style="color: red&quot; onmouseover=&quot;alert(1); background-color: #000000;">"#
        );
    }

    #[test]
    fn pre_attributes_union_classes_append_styles_and_escape_values() {
        let theme = crate::themes::get("dracula").unwrap();
        let attrs = vec![
            ("class".to_string(), "shorthand authored".into()),
            ("style".to_string(), "outline: 1px solid red".into()),
            ("id".to_string(), r#"sample" onmouseover="alert(1)"#.into()),
        ];
        let mut output = Vec::new();

        write_pre_tag(&mut output, Some("shorthand"), Some(&theme), &attrs).unwrap();

        assert_str_eq!(
            String::from_utf8(output).unwrap(),
            concat!(
                r#"<pre class="lumis shorthand authored" "#,
                r#"style="color: #f8f8f2; background-color: #282a36; outline: 1px solid red" "#,
                r#"id="sample&quot; onmouseover=&quot;alert(1)">"#,
            )
        );
    }

    #[test]
    fn code_attributes_union_classes_and_override_defaults() {
        let attrs = vec![
            ("class".to_string(), "copyable language-plaintext".into()),
            ("translate".to_string(), "yes".into()),
            ("tabindex".to_string(), "-1".into()),
            ("data-copy".to_string(), "button".into()),
        ];
        let mut output = Vec::new();

        write_code_tag(&mut output, Language::PlainText, &attrs).unwrap();

        assert_str_eq!(
            String::from_utf8(output).unwrap(),
            concat!(
                r#"<code class="language-plaintext copyable" "#,
                r#"translate="yes" tabindex="-1" data-copy="button">"#,
            )
        );
    }

    #[test]
    fn boolean_attributes_render_bare_and_false_removes_a_default() {
        let attrs = vec![
            ("inert".to_string(), true.into()),
            ("translate".to_string(), false.into()),
        ];
        let mut output = Vec::new();

        write_code_tag(&mut output, Language::PlainText, &attrs).unwrap();

        assert_str_eq!(
            String::from_utf8(output).unwrap(),
            r#"<code class="language-plaintext" tabindex="0" inert>"#
        );
    }

    #[test]
    fn an_attribute_name_cannot_break_out_of_the_tag() {
        for name in [
            r#"x" onclick="alert(1)"#,
            "x onclick=alert(1)",
            "x=y",
            "x/y",
            "x>y",
            "",
        ] {
            let attrs = vec![(name.to_string(), "y".into())];
            let error = open_tag(&mut Vec::new(), "pre", &attrs).expect_err("the name is rejected");

            assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
        }

        let valid = vec![("data-copy".to_string(), "y".into())];
        assert!(open_tag(&mut Vec::new(), "pre", &valid).is_ok());
    }

    #[test]
    fn a_pre_class_naming_a_theme_is_not_repeated() {
        let mut themes = std::collections::HashMap::new();
        themes.insert("dark".to_string(), crate::themes::get("dracula").unwrap());

        let attrs = multi_themes_pre_attrs(Some("dark"), &themes, None, "--lumis", &[]);

        assert_eq!(
            attrs[0].1,
            AttrValue::Value("lumis lumis-themes dark".to_string())
        );
    }

    #[test]
    fn test_open_multi_themes_pre_tag_escapes_caller_class() {
        let mut themes = std::collections::HashMap::new();
        themes.insert("dark".to_string(), quote_bearing_theme());

        let mut output = Vec::new();
        open_multi_themes_pre_tag(
            &mut output,
            Some(r#"x"><script>"#),
            &themes,
            Some("dark"),
            "--lumis",
        )
        .unwrap();

        let html = String::from_utf8(output).unwrap();

        assert!(
            html.starts_with(r#"<pre class="lumis lumis-themes x&quot;&gt;&lt;script&gt; dark""#),
            "unescaped class in {html}"
        );
        assert!(!html.contains("<script>"), "unescaped markup in {html}");
    }

    #[test]
    fn test_wrap_line_escapes_caller_class_and_style() {
        let result = wrap_line(
            1,
            "content",
            Some(r#" x"><script>"#),
            Some(r#"color: red" onmouseover="alert(1)"#),
        );

        assert_str_eq!(
            result,
            r#"<div class="l-line x&quot;&gt;&lt;script&gt;" style="color: red&quot; onmouseover=&quot;alert(1)" data-line="1">content</div>"#
        );
    }

    #[test]
    fn test_span_inline_escapes_theme_colors() {
        assert_str_eq!(
            span_inline(
                "fn",
                None,
                "keyword",
                Some(&quote_bearing_theme()),
                false,
                false
            ),
            r#"<span style="color: red&quot; onmouseover=&quot;alert(1);">fn</span>"#
        );
    }

    #[test]
    fn test_span_multi_themes_escapes_theme_colors() {
        let mut themes = std::collections::HashMap::new();
        themes.insert("dark".to_string(), quote_bearing_theme());

        let result = span_multi_themes(
            "fn", "keyword", None, &themes, None, "--lumis", false, false,
        );

        assert_str_eq!(
            result,
            r#"<span style="--lumis-dark:red&quot; onmouseover=&quot;alert(1); --lumis-dark-font-style:normal; --lumis-dark-font-weight:normal; --lumis-dark-text-decoration:none;">fn</span>"#
        );
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

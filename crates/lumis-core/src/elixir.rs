//! Lumis options as Elixir writes them.
//!
//! The `:lumis` NIF and `mdex_native` both take Lumis options from the BEAM.
//! These decoders read them in the shape a caller writes,
//! `{:html_inline, theme: "dracula", highlight_lines: %{lines: [2..4]}}`, and
//! fill in the defaults `Lumis` documents. Neither NIF needs an Elixir step to
//! convert options first, so the two cannot drift apart.
//!
//! `Lumis` still validates options in Elixir, where `NimbleOptions` documents
//! them and reports errors before anything crosses into Rust. A caller that
//! skips that step gets the same checks here: an unknown option, or a value of
//! the wrong type, raises an `ArgumentError` that names it.

use crate::formatter::html::{is_valid_attr_name, AttrValue, HtmlAttrs, HtmlStructure};
use crate::formatter::{
    bbcode, html::SteppedLineRange, html_inline, html_linked, terminal, BBCodeScopedBuilder,
    Formatter, HtmlElement, HtmlInlineBuilder, HtmlLinkedBuilder, HtmlMultiThemesBuilder,
    TerminalBackground, TerminalBuilder,
};
use crate::{languages::Language, themes};
use rustler::types::map::MapIterator;
use rustler::{
    Atom, Decoder, Encoder, Env, Error, NifMap, NifResult, NifStruct, NifUnitEnum, NifUntaggedEnum,
    Term,
};
use std::collections::HashMap;
use std::fmt::Write;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, NifUnitEnum)]
pub enum ExAppearance {
    Light,
    #[default]
    Dark,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ExHtmlStructure {
    #[default]
    Block,
    Inline,
}

impl From<ExHtmlStructure> for HtmlStructure {
    fn from(structure: ExHtmlStructure) -> Self {
        match structure {
            ExHtmlStructure::Block => Self::Block,
            ExHtmlStructure::Inline => Self::Inline,
        }
    }
}

/// An attribute value as Elixir spells it: a string, or `true`/`false` for the
/// boolean form and for removing one of Lumis's own attributes.
///
/// `bool` comes first because Rustler tries the variants in order and `true` is
/// an atom, not a binary.
#[derive(Clone, Debug, NifUntaggedEnum)]
pub enum ExAttrValue {
    Flag(bool),
    Value(String),
}

impl From<ExAttrValue> for AttrValue {
    fn from(value: ExAttrValue) -> Self {
        match value {
            ExAttrValue::Flag(flag) => Self::from(flag),
            ExAttrValue::Value(value) => Self::Value(value),
        }
    }
}

impl From<AttrValue> for ExAttrValue {
    fn from(value: AttrValue) -> Self {
        match value {
            AttrValue::Value(value) => Self::Value(value),
            AttrValue::Present => Self::Flag(true),
            AttrValue::Absent => Self::Flag(false),
        }
    }
}

/// Decode the attribute pairs an Elixir keyword list arrives as.
pub fn attr_values(attrs: Vec<(String, ExAttrValue)>) -> HtmlAttrs {
    attrs
        .into_iter()
        .map(|(name, value)| (name, value.into()))
        .collect()
}

/// Encode attribute pairs back into what Elixir reads as a keyword list.
pub fn ex_attr_values(attrs: HtmlAttrs) -> Vec<(String, ExAttrValue)> {
    attrs
        .into_iter()
        .map(|(name, value)| (name, value.into()))
        .collect()
}

#[derive(Clone, Debug)]
pub enum ExFormatterOption {
    HtmlInline {
        structure: ExHtmlStructure,
        theme: Option<ThemeOrString>,
        pre_class: Option<String>,
        pre_attrs: Vec<(String, ExAttrValue)>,
        code_attrs: Vec<(String, ExAttrValue)>,
        italic: bool,
        include_highlights: bool,
        highlight_lines: Option<ExHtmlInlineHighlightLines>,
        line_numbers: bool,
        header: Option<ExHtmlElement>,
    },
    HtmlLinked {
        structure: ExHtmlStructure,
        pre_class: Option<String>,
        pre_attrs: Vec<(String, ExAttrValue)>,
        code_attrs: Vec<(String, ExAttrValue)>,
        highlight_lines: Option<ExHtmlLinkedHighlightLines>,
        line_numbers: bool,
        header: Option<ExHtmlElement>,
    },
    HtmlMultiThemes {
        structure: ExHtmlStructure,
        themes: HashMap<String, ExTheme>,
        default_theme: Option<String>,
        css_variable_prefix: Option<String>,
        pre_class: Option<String>,
        pre_attrs: Vec<(String, ExAttrValue)>,
        code_attrs: Vec<(String, ExAttrValue)>,
        italic: bool,
        include_highlights: bool,
        highlight_lines: Option<ExHtmlInlineHighlightLines>,
        line_numbers: bool,
        header: Option<ExHtmlElement>,
    },
    Terminal {
        theme: Option<ThemeOrString>,
        background: Option<ExTerminalBackground>,
        width: Option<usize>,
        highlight_lines: Option<ExTerminalHighlightLines>,
        line_numbers: bool,
    },
    BbcodeScoped {
        highlight_lines: Option<ExBBCodeHighlightLines>,
    },
}

#[derive(Clone, Debug)]
pub enum ExTerminalBackground {
    Theme,
    String(String),
}

impl Default for ExFormatterOption {
    fn default() -> Self {
        Self::HtmlInline {
            structure: ExHtmlStructure::Block,
            theme: None,
            pre_class: None,
            pre_attrs: Vec::new(),
            code_attrs: Vec::new(),
            italic: false,
            include_highlights: false,
            highlight_lines: None,
            line_numbers: false,
            header: None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum ThemeOrString {
    Theme(ExTheme),
    String(String),
}

impl Default for ThemeOrString {
    fn default() -> Self {
        Self::String("onedark".to_string())
    }
}

fn resolve_theme(theme_or_string: ThemeOrString) -> Option<themes::Theme> {
    match theme_or_string {
        ThemeOrString::Theme(theme) => Some(theme.into()),
        ThemeOrString::String(name) => themes::get(&name).ok(),
    }
}

#[inline]
fn convert_line_specs(
    lines: Vec<ExLineSpec>,
) -> (Vec<std::ops::RangeInclusive<usize>>, Vec<SteppedLineRange>) {
    let mut contiguous = Vec::new();
    let mut stepped = Vec::new();

    for line in lines {
        match line {
            ExLineSpec::Single(line) => contiguous.push(line..=line),
            ExLineSpec::Range {
                start,
                end,
                step: 1,
            } if start <= end => contiguous.push(start..=end),
            ExLineSpec::Range {
                start,
                end,
                step: -1,
            } if start >= end => contiguous.push(end..=start),
            line @ ExLineSpec::Range { .. } => {
                if let Some(range) = line.to_stepped_line_range() {
                    stepped.push(range);
                }
            }
        }
    }

    (contiguous, stepped)
}

pub fn line_specs_contain(lines: &[ExLineSpec], line_number: usize) -> bool {
    lines.iter().any(|line| {
        line.to_stepped_line_range()
            .is_some_and(|range| range.contains(line_number))
    })
}

#[inline]
fn convert_inline_style(
    style: ExHtmlInlineHighlightLinesStyle,
) -> html_inline::HighlightLinesStyle {
    match style {
        ExHtmlInlineHighlightLinesStyle::Theme => html_inline::HighlightLinesStyle::Theme,
        ExHtmlInlineHighlightLinesStyle::Style { style } => {
            html_inline::HighlightLinesStyle::Style(style)
        }
    }
}

fn convert_inline_highlight_lines(
    highlight: ExHtmlInlineHighlightLines,
) -> (html_inline::HighlightLines, Vec<SteppedLineRange>) {
    let (lines, stepped) = convert_line_specs(highlight.lines);
    let highlight = html_inline::HighlightLines {
        lines,
        style: highlight.style.map(convert_inline_style),
        class: highlight.class,
    };
    (highlight, stepped)
}

fn convert_terminal_highlight_lines(
    highlight: ExTerminalHighlightLines,
) -> (terminal::HighlightLines, Vec<SteppedLineRange>) {
    let (lines, stepped) = convert_line_specs(highlight.lines);
    let highlight = terminal::HighlightLines {
        lines,
        background: highlight.background,
    };
    (highlight, stepped)
}

fn convert_bbcode_highlight_lines(
    highlight: ExBBCodeHighlightLines,
) -> (bbcode::HighlightLines, Vec<SteppedLineRange>) {
    let (lines, stepped) = convert_line_specs(highlight.lines);
    (bbcode::HighlightLines { lines }, stepped)
}

fn convert_linked_highlight_lines(
    highlight: ExHtmlLinkedHighlightLines,
) -> (html_linked::HighlightLines, Vec<SteppedLineRange>) {
    let (lines, stepped) = convert_line_specs(highlight.lines);
    let highlight = html_linked::HighlightLines {
        lines,
        class: highlight.class,
    };
    (highlight, stepped)
}

fn split_highlight_lines<T>(
    converted: Option<(T, Vec<SteppedLineRange>)>,
) -> (Option<T>, Vec<SteppedLineRange>) {
    converted.map_or((None, Vec::new()), |(highlight, stepped)| {
        (Some(highlight), stepped)
    })
}

impl ExFormatterOption {
    pub fn into_formatter<T>(self, language: Language) -> Result<Box<dyn Formatter<T>>, String> {
        match self {
            ExFormatterOption::HtmlInline {
                structure,
                theme,
                pre_class,
                pre_attrs,
                code_attrs,
                italic,
                include_highlights,
                highlight_lines,
                line_numbers,
                header,
            } => {
                let theme = theme.and_then(resolve_theme);

                let (highlight_lines, stepped_highlight_lines) =
                    split_highlight_lines(highlight_lines.map(convert_inline_highlight_lines));

                let header = header.map(|h| HtmlElement {
                    open_tag: h.open_tag,
                    close_tag: h.close_tag,
                });

                let mut formatter = HtmlInlineBuilder::new()
                    .language(language)
                    .structure(structure.into())
                    .theme(theme)
                    .pre_class(pre_class)
                    .pre_attrs(attr_values(pre_attrs))
                    .code_attrs(attr_values(code_attrs))
                    .italic(italic)
                    .include_highlights(include_highlights)
                    .highlight_lines(highlight_lines)
                    .line_numbers(line_numbers)
                    .header(header)
                    .build()
                    .map_err(|e| format!("HtmlInline builder error: {e:?}"))?;
                formatter.set_stepped_highlight_lines(stepped_highlight_lines);

                Ok(Box::new(formatter))
            }
            ExFormatterOption::HtmlLinked {
                structure,
                pre_class,
                pre_attrs,
                code_attrs,
                highlight_lines,
                line_numbers,
                header,
            } => {
                let (highlight_lines, stepped_highlight_lines) =
                    split_highlight_lines(highlight_lines.map(convert_linked_highlight_lines));

                let header = header.map(|h| HtmlElement {
                    open_tag: h.open_tag,
                    close_tag: h.close_tag,
                });

                let mut formatter = HtmlLinkedBuilder::new()
                    .language(language)
                    .structure(structure.into())
                    .pre_class(pre_class)
                    .pre_attrs(attr_values(pre_attrs))
                    .code_attrs(attr_values(code_attrs))
                    .highlight_lines(highlight_lines)
                    .line_numbers(line_numbers)
                    .header(header)
                    .build()
                    .map_err(|e| format!("HtmlLinked builder error: {e:?}"))?;
                formatter.set_stepped_highlight_lines(stepped_highlight_lines);

                Ok(Box::new(formatter))
            }
            ExFormatterOption::HtmlMultiThemes {
                structure,
                themes,
                default_theme,
                css_variable_prefix,
                pre_class,
                pre_attrs,
                code_attrs,
                italic,
                include_highlights,
                highlight_lines,
                line_numbers,
                header,
            } => {
                let themes_map: HashMap<String, themes::Theme> =
                    themes.into_iter().map(|(k, v)| (k, v.into())).collect();

                let (highlight_lines, stepped_highlight_lines) =
                    split_highlight_lines(highlight_lines.map(convert_inline_highlight_lines));

                let header = header.map(|h| HtmlElement {
                    open_tag: h.open_tag,
                    close_tag: h.close_tag,
                });

                let mut builder = HtmlMultiThemesBuilder::new();
                builder
                    .language(language)
                    .structure(structure.into())
                    .themes(themes_map)
                    .css_variable_prefix(css_variable_prefix.as_deref().unwrap_or("--lumis"))
                    .pre_class(pre_class)
                    .pre_attrs(attr_values(pre_attrs))
                    .code_attrs(attr_values(code_attrs))
                    .italic(italic)
                    .include_highlights(include_highlights)
                    .highlight_lines(highlight_lines)
                    .line_numbers(line_numbers)
                    .header(header);

                if let Some(dt_str) = default_theme {
                    builder.default_theme(dt_str);
                }

                let mut formatter = builder
                    .build()
                    .map_err(|e| format!("HtmlMultiThemes builder error: {e:?}"))?;
                formatter.set_stepped_highlight_lines(stepped_highlight_lines);

                Ok(Box::new(formatter))
            }
            ExFormatterOption::Terminal {
                theme,
                background,
                width,
                highlight_lines,
                line_numbers,
            } => {
                let theme = theme.and_then(resolve_theme);
                let background = match background {
                    Some(ExTerminalBackground::Theme) => TerminalBackground::Theme,
                    Some(ExTerminalBackground::String(color)) => TerminalBackground::Color(color),
                    None => TerminalBackground::Inherit,
                };
                let (highlight_lines, stepped_highlight_lines) =
                    split_highlight_lines(highlight_lines.map(convert_terminal_highlight_lines));

                let mut formatter = TerminalBuilder::new()
                    .language(language)
                    .theme(theme)
                    .background(background)
                    .width(width)
                    .highlight_lines(highlight_lines)
                    .line_numbers(line_numbers)
                    .build()
                    .map_err(|e| format!("Terminal builder error: {e:?}"))?;
                formatter.set_stepped_highlight_lines(stepped_highlight_lines);

                Ok(Box::new(formatter))
            }
            ExFormatterOption::BbcodeScoped { highlight_lines } => {
                let (highlight_lines, stepped_highlight_lines) =
                    split_highlight_lines(highlight_lines.map(convert_bbcode_highlight_lines));

                let mut formatter = BBCodeScopedBuilder::new()
                    .language(language)
                    .highlight_lines(highlight_lines)
                    .build()
                    .map_err(|e| format!("BBCode scoped builder error: {e:?}"))?;
                formatter.set_stepped_highlight_lines(stepped_highlight_lines);

                Ok(Box::new(formatter))
            }
        }
    }
}

#[derive(Clone, Debug, Default, NifStruct)]
#[module = "Lumis.Theme"]
pub struct ExTheme {
    pub name: String,
    pub appearance: ExAppearance,
    pub revision: String,
    pub highlights: HashMap<String, ExStyle>,
}

impl From<ExTheme> for themes::Theme {
    fn from(theme: ExTheme) -> Self {
        let appearance = match theme.appearance {
            ExAppearance::Light => themes::Appearance::Light,
            ExAppearance::Dark => themes::Appearance::Dark,
        };
        themes::Theme {
            name: theme.name,
            appearance,
            revision: theme.revision,
            highlights: theme
                .highlights
                .into_iter()
                .map(|(name, style)| (name, style.into()))
                .collect(),
        }
    }
}

impl<'a> From<&'a themes::Theme> for ExTheme {
    fn from(theme: &'a themes::Theme) -> Self {
        let mut color_cache: HashMap<&str, String> = HashMap::with_capacity(32);

        let highlights = theme
            .highlights
            .iter()
            .map(|(k, v)| {
                let fg = v.fg.as_ref().map(|color_str| {
                    color_cache
                        .entry(color_str.as_str())
                        .or_insert_with(|| color_str.clone())
                        .clone()
                });

                let bg = v.bg.as_ref().map(|color_str| {
                    color_cache
                        .entry(color_str.as_str())
                        .or_insert_with(|| color_str.clone())
                        .clone()
                });

                (
                    k.to_owned(),
                    ExStyle {
                        fg,
                        bg,
                        bold: v.bold,
                        italic: v.italic,
                        text_decoration: v.text_decoration.into(),
                    },
                )
            })
            .collect();

        let appearance = match theme.appearance {
            themes::Appearance::Light => ExAppearance::Light,
            themes::Appearance::Dark => ExAppearance::Dark,
        };

        ExTheme {
            name: theme.name.clone(),
            appearance,
            revision: theme.revision.clone(),
            highlights,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, NifUnitEnum)]
pub enum ExUnderlineStyle {
    Solid,
    Wavy,
    Double,
    Dotted,
    Dashed,
}

impl ExUnderlineStyle {
    fn from_theme(style: themes::UnderlineStyle) -> Option<Self> {
        match style {
            themes::UnderlineStyle::None => None,
            themes::UnderlineStyle::Solid => Some(ExUnderlineStyle::Solid),
            themes::UnderlineStyle::Wavy => Some(ExUnderlineStyle::Wavy),
            themes::UnderlineStyle::Double => Some(ExUnderlineStyle::Double),
            themes::UnderlineStyle::Dotted => Some(ExUnderlineStyle::Dotted),
            themes::UnderlineStyle::Dashed => Some(ExUnderlineStyle::Dashed),
        }
    }

    fn to_theme(opt: Option<Self>) -> themes::UnderlineStyle {
        match opt {
            None => themes::UnderlineStyle::None,
            Some(ExUnderlineStyle::Solid) => themes::UnderlineStyle::Solid,
            Some(ExUnderlineStyle::Wavy) => themes::UnderlineStyle::Wavy,
            Some(ExUnderlineStyle::Double) => themes::UnderlineStyle::Double,
            Some(ExUnderlineStyle::Dotted) => themes::UnderlineStyle::Dotted,
            Some(ExUnderlineStyle::Dashed) => themes::UnderlineStyle::Dashed,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, NifStruct)]
#[module = "Lumis.Theme.TextDecoration"]
pub struct ExTextDecoration {
    pub underline: Option<ExUnderlineStyle>,
    pub strikethrough: bool,
}

impl From<themes::TextDecoration> for ExTextDecoration {
    fn from(td: themes::TextDecoration) -> Self {
        ExTextDecoration {
            underline: ExUnderlineStyle::from_theme(td.underline),
            strikethrough: td.strikethrough,
        }
    }
}

impl From<ExTextDecoration> for themes::TextDecoration {
    fn from(td: ExTextDecoration) -> Self {
        themes::TextDecoration {
            underline: ExUnderlineStyle::to_theme(td.underline),
            strikethrough: td.strikethrough,
        }
    }
}

#[derive(Clone, Debug, Default, NifStruct)]
#[module = "Lumis.Theme.Style"]
pub struct ExStyle {
    pub fg: Option<String>,
    pub bg: Option<String>,
    pub bold: bool,
    pub italic: bool,
    pub text_decoration: ExTextDecoration,
}

impl From<ExStyle> for themes::Style {
    fn from(style: ExStyle) -> Self {
        themes::Style {
            fg: style.fg,
            bg: style.bg,
            bold: style.bold,
            italic: style.italic,
            text_decoration: style.text_decoration.into(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ExHtmlElement {
    pub open_tag: String,
    pub close_tag: String,
}

/// A line to highlight: a line number, or an Elixir `Range` with its direction
/// and step. Rust keeps a range as the arithmetic progression it is and clips
/// it to the rendered document; it is never expanded into one entry per line.
#[derive(Clone, Debug)]
pub enum ExLineSpec {
    Single(usize),
    Range {
        start: usize,
        end: usize,
        step: isize,
    },
}

impl ExLineSpec {
    fn to_stepped_line_range(&self) -> Option<SteppedLineRange> {
        match self {
            ExLineSpec::Single(line) => SteppedLineRange::new(*line, *line, 1),
            ExLineSpec::Range { start, end, step } => SteppedLineRange::new(*start, *end, *step),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum ExHtmlInlineHighlightLinesStyle {
    #[default]
    Theme,
    Style {
        style: String,
    },
}

#[derive(Clone, Debug, Default)]
pub struct ExHtmlInlineHighlightLines {
    pub lines: Vec<ExLineSpec>,
    pub style: Option<ExHtmlInlineHighlightLinesStyle>,
    pub class: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct ExHtmlLinkedHighlightLines {
    pub lines: Vec<ExLineSpec>,
    pub class: String,
}

#[derive(Clone, Debug, Default)]
pub struct ExTerminalHighlightLines {
    pub lines: Vec<ExLineSpec>,
    pub background: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct ExBBCodeHighlightLines {
    pub lines: Vec<ExLineSpec>,
}

#[derive(Clone, Debug, NifMap)]
pub struct ExCssOptions {
    pub layout: bool,
    pub enable_italic: bool,
    pub scope: String,
    pub container_selector: String,
    pub container_style: Vec<(String, String)>,
}

impl<'a> From<&'a themes::Style> for ExStyle {
    fn from(style: &'a themes::Style) -> Self {
        ExStyle {
            fg: style.fg.clone(),
            bg: style.bg.clone(),
            bold: style.bold,
            italic: style.italic,
            text_decoration: style.text_decoration.into(),
        }
    }
}

impl From<HtmlElement> for ExHtmlElement {
    fn from(element: HtmlElement) -> Self {
        ExHtmlElement {
            open_tag: element.open_tag,
            close_tag: element.close_tag,
        }
    }
}

impl From<ExHtmlElement> for HtmlElement {
    fn from(element: ExHtmlElement) -> Self {
        HtmlElement {
            open_tag: element.open_tag,
            close_tag: element.close_tag,
        }
    }
}

// Decoding.
//
// Each decoder reports what it expected and what it got. The formatter decoder
// prefixes that with the formatter's name, and `Decoder` raises the result as
// `ArgumentError`, the exception a bad option raises everywhere else in `Lumis`.

type Decoded<T> = Result<T, String>;

impl<'a> Decoder<'a> for ExFormatterOption {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        Self::from_term(term).map_err(argument_error)
    }
}

impl<'a> Decoder<'a> for ExLineSpec {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        line_spec(term).map_err(argument_error)
    }
}

impl ExFormatterOption {
    /// Decodes a built-in formatter as Elixir writes it: `:html_inline`, or
    /// `{:html_inline, options}` with the options as a keyword list or a map.
    ///
    /// `:language` is checked and left out. [`ExLumisOptions`] reads it, and a
    /// caller that decides the language itself, from a code fence say, ignores
    /// it.
    ///
    /// # Errors
    ///
    /// Returns a message naming the formatter and the option that failed, for
    /// an unknown formatter or option and for a value of the wrong type.
    pub fn from_term(term: Term<'_>) -> Result<Self, String> {
        formatter_with_language(term).map(|(formatter, _language)| formatter)
    }
}

/// The formatter and the `:language` given in its options.
fn formatter_with_language(term: Term<'_>) -> Decoded<(ExFormatterOption, Option<String>)> {
    let (name, options) = formatter_parts(term)?;
    if !FORMATTERS.contains(&name.as_str()) {
        return Err(format!(
            "unknown formatter :{name}{}, expected one of {}",
            suggestion(&name, FORMATTERS, |name| format!(":{name}")),
            atom_list(&FORMATTERS)
        ));
    }

    let decoded = Fields::of(options).and_then(|mut fields| {
        let language = fields.optional("language", string)?;

        let formatter = match name.as_str() {
            "html_inline" => html_inline(&mut fields)?,
            "html_linked" => html_linked(&mut fields)?,
            "html_multi_themes" => html_multi_themes(&mut fields)?,
            "terminal" => terminal(&mut fields)?,
            "bbcode_scoped" => ExFormatterOption::BbcodeScoped {
                highlight_lines: fields.optional("highlight_lines", bbcode_highlight_lines)?,
            },
            _ => unreachable!("checked against FORMATTERS above"),
        };

        fields.finish().map(|()| (formatter, language))
    });

    decoded.map_err(|error| format!("invalid options given to {name}: {error}"))
}

/// Lumis's options as `Lumis.highlight/2` takes them, the formatter and the
/// options around it, as a keyword list or a map.
///
/// The deprecated top-level `:theme`, `:pre_class` and `:inline_style` are
/// applied to the formatter here, so a caller never handles them. The
/// `:annotations` are left as terms: `Lumis` normalizes them before the NIF
/// decodes them, and a caller that does not annotate ignores them.
#[derive(Debug, Default)]
pub struct ExLumisOptions<'a> {
    pub formatter: ExFormatterOption,
    /// The formatter's `:language`, else the deprecated top-level one.
    pub language: Option<String>,
    pub rainbow_brackets: bool,
    pub annotations: Vec<Term<'a>>,
    /// `:budget`'s `:time_limit`, in milliseconds. `nil` selects the default.
    pub time_limit: Option<u64>,
    /// `:budget`'s `:match_limit`. `nil` selects the default.
    pub match_limit: Option<u32>,
}

impl<'a> Decoder<'a> for ExLumisOptions<'a> {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        Self::from_term(term).map_err(argument_error)
    }
}

impl<'a> ExLumisOptions<'a> {
    /// # Errors
    ///
    /// Returns a message naming the option that failed, for an unknown option
    /// and for a value of the wrong type.
    pub fn from_term(term: Term<'a>) -> Result<Self, String> {
        let mut fields = Fields::of(Some(term))?;
        let (formatter, formatter_language) = fields
            .optional("formatter", formatter_with_language)?
            .unwrap_or_default();
        let language = fields.optional("language", string)?;
        let deprecated_theme = fields.optional("theme", theme)?;
        let deprecated_pre_class = fields.optional("pre_class", string)?;
        let inline_style = fields.optional("inline_style", boolean)?;
        let rainbow_brackets = fields.flag("rainbow_brackets")?;
        let annotations = fields.optional("annotations", list)?.unwrap_or_default();
        let (time_limit, match_limit) = fields.optional("budget", budget)?.unwrap_or_default();
        fields.finish()?;

        Ok(Self {
            formatter: formatter.with_deprecated(
                inline_style,
                deprecated_theme,
                deprecated_pre_class,
            ),
            language: formatter_language.or(language),
            rainbow_brackets,
            annotations,
            time_limit,
            match_limit,
        })
    }
}

/// Every setting a built-in formatter can carry, for moving one formatter's
/// settings onto another.
#[derive(Default)]
struct Settings {
    structure: ExHtmlStructure,
    theme: Option<ThemeOrString>,
    pre_class: Option<String>,
    pre_attrs: Vec<(String, ExAttrValue)>,
    code_attrs: Vec<(String, ExAttrValue)>,
    italic: bool,
    include_highlights: bool,
    highlight_lines: Option<ExHtmlInlineHighlightLines>,
    line_numbers: bool,
    header: Option<ExHtmlElement>,
}

impl ExFormatterOption {
    /// Applies `Lumis`'s deprecated top-level options as it always has.
    /// `inline_style` picks `:html_inline` or `:html_linked`, keeping the
    /// settings that formatter takes. Then `theme` and `pre_class` replace the
    /// formatter's own, where it takes them.
    fn with_deprecated(
        self,
        inline_style: Option<bool>,
        theme: Option<ThemeOrString>,
        pre_class: Option<String>,
    ) -> Self {
        let mut formatter = match (inline_style, self) {
            (Some(true), formatter @ Self::HtmlInline { .. })
            | (Some(false), formatter @ Self::HtmlLinked { .. })
            | (None, formatter) => formatter,
            (Some(true), formatter) => formatter.settings().into_html_inline(),
            (Some(false), formatter) => formatter.settings().into_html_linked(),
        };

        if let Some(new_theme) = theme {
            if let Self::HtmlInline { theme, .. } | Self::Terminal { theme, .. } = &mut formatter {
                *theme = Some(new_theme);
            }
        }

        if let Some(new_pre_class) = pre_class {
            if let Self::HtmlInline { pre_class, .. }
            | Self::HtmlLinked { pre_class, .. }
            | Self::HtmlMultiThemes { pre_class, .. } = &mut formatter
            {
                *pre_class = Some(new_pre_class);
            }
        }

        formatter
    }

    fn settings(self) -> Settings {
        // Lines carried over from a formatter without a line style take the
        // inline formatter's default, `:theme`.
        let lines = |lines, class| ExHtmlInlineHighlightLines {
            lines,
            style: Some(ExHtmlInlineHighlightLinesStyle::Theme),
            class,
        };

        match self {
            Self::HtmlInline {
                structure,
                theme,
                pre_class,
                pre_attrs,
                code_attrs,
                italic,
                include_highlights,
                highlight_lines,
                line_numbers,
                header,
            } => Settings {
                structure,
                theme,
                pre_class,
                pre_attrs,
                code_attrs,
                italic,
                include_highlights,
                highlight_lines,
                line_numbers,
                header,
            },
            Self::HtmlLinked {
                structure,
                pre_class,
                pre_attrs,
                code_attrs,
                highlight_lines,
                line_numbers,
                header,
            } => Settings {
                structure,
                pre_class,
                pre_attrs,
                code_attrs,
                highlight_lines: highlight_lines
                    .map(|highlight| lines(highlight.lines, Some(highlight.class))),
                line_numbers,
                header,
                ..Settings::default()
            },
            Self::HtmlMultiThemes {
                structure,
                pre_class,
                pre_attrs,
                code_attrs,
                italic,
                include_highlights,
                highlight_lines,
                line_numbers,
                header,
                ..
            } => Settings {
                structure,
                pre_class,
                pre_attrs,
                code_attrs,
                italic,
                include_highlights,
                highlight_lines,
                line_numbers,
                header,
                ..Settings::default()
            },
            Self::Terminal {
                theme,
                highlight_lines,
                line_numbers,
                ..
            } => Settings {
                theme,
                highlight_lines: highlight_lines.map(|highlight| lines(highlight.lines, None)),
                line_numbers,
                ..Settings::default()
            },
            Self::BbcodeScoped { highlight_lines } => Settings {
                highlight_lines: highlight_lines.map(|highlight| lines(highlight.lines, None)),
                ..Settings::default()
            },
        }
    }
}

impl Settings {
    fn into_html_inline(self) -> ExFormatterOption {
        ExFormatterOption::HtmlInline {
            structure: self.structure,
            theme: self.theme,
            pre_class: self.pre_class,
            pre_attrs: self.pre_attrs,
            code_attrs: self.code_attrs,
            italic: self.italic,
            include_highlights: self.include_highlights,
            highlight_lines: self.highlight_lines,
            line_numbers: self.line_numbers,
            header: self.header,
        }
    }

    fn into_html_linked(self) -> ExFormatterOption {
        ExFormatterOption::HtmlLinked {
            structure: self.structure,
            pre_class: self.pre_class,
            pre_attrs: self.pre_attrs,
            code_attrs: self.code_attrs,
            highlight_lines: self
                .highlight_lines
                .map(|highlight| ExHtmlLinkedHighlightLines {
                    lines: highlight.lines,
                    class: highlight
                        .class
                        .unwrap_or_else(|| "l-highlighted".to_string()),
                }),
            line_numbers: self.line_numbers,
            header: self.header,
        }
    }
}

const FORMATTERS: [&str; 5] = [
    "html_inline",
    "html_linked",
    "html_multi_themes",
    "terminal",
    "bbcode_scoped",
];

fn atom_list(names: &[&str]) -> String {
    let atoms: Vec<String> = names.iter().map(|name| format!(":{name}")).collect();
    format!("[{}]", atoms.join(", "))
}

fn formatter_parts(term: Term<'_>) -> Decoded<(String, Option<Term<'_>>)> {
    if let Some(name) = atom_name(term) {
        return Ok((name, None));
    }

    if let Ok((name, options)) = term.decode::<(Term<'_>, Term<'_>)>() {
        if let Some(name) = atom_name(name) {
            return Ok((name, Some(options)));
        }
    }

    Err(format!(
        "expected a formatter such as :html_inline or {{:html_inline, options}}, got: {}",
        inspect(term)
    ))
}

fn html_inline(fields: &mut Fields<'_>) -> Decoded<ExFormatterOption> {
    Ok(ExFormatterOption::HtmlInline {
        structure: fields.optional("structure", structure)?.unwrap_or_default(),
        theme: fields.optional("theme", theme)?,
        pre_class: fields.optional("pre_class", string)?,
        pre_attrs: fields.optional("pre_attrs", attrs)?.unwrap_or_default(),
        code_attrs: fields.optional("code_attrs", attrs)?.unwrap_or_default(),
        italic: fields.flag("italic")?,
        include_highlights: fields.flag("include_highlights")?,
        highlight_lines: fields.optional("highlight_lines", inline_highlight_lines)?,
        line_numbers: fields.flag("line_numbers")?,
        header: fields.optional("header", header)?,
    })
}

fn html_linked(fields: &mut Fields<'_>) -> Decoded<ExFormatterOption> {
    Ok(ExFormatterOption::HtmlLinked {
        structure: fields.optional("structure", structure)?.unwrap_or_default(),
        pre_class: fields.optional("pre_class", string)?,
        pre_attrs: fields.optional("pre_attrs", attrs)?.unwrap_or_default(),
        code_attrs: fields.optional("code_attrs", attrs)?.unwrap_or_default(),
        highlight_lines: fields.optional("highlight_lines", linked_highlight_lines)?,
        line_numbers: fields.flag("line_numbers")?,
        header: fields.optional("header", header)?,
    })
}

fn html_multi_themes(fields: &mut Fields<'_>) -> Decoded<ExFormatterOption> {
    let themes = fields
        .optional("themes", named_themes)?
        .ok_or_else(|| "required :themes option not found".to_string())?;

    Ok(ExFormatterOption::HtmlMultiThemes {
        structure: fields.optional("structure", structure)?.unwrap_or_default(),
        themes,
        default_theme: fields.optional("default_theme", string)?,
        css_variable_prefix: fields.optional("css_variable_prefix", string)?,
        pre_class: fields.optional("pre_class", string)?,
        pre_attrs: fields.optional("pre_attrs", attrs)?.unwrap_or_default(),
        code_attrs: fields.optional("code_attrs", attrs)?.unwrap_or_default(),
        italic: fields.flag("italic")?,
        include_highlights: fields.flag("include_highlights")?,
        highlight_lines: fields.optional("highlight_lines", inline_highlight_lines)?,
        line_numbers: fields.flag("line_numbers")?,
        header: fields.optional("header", header)?,
    })
}

fn terminal(fields: &mut Fields<'_>) -> Decoded<ExFormatterOption> {
    Ok(ExFormatterOption::Terminal {
        theme: fields.optional("theme", theme)?,
        background: fields.optional("background", terminal_background)?,
        width: fields.optional("width", positive_integer)?,
        highlight_lines: fields.optional("highlight_lines", terminal_highlight_lines)?,
        line_numbers: fields.flag("line_numbers")?,
    })
}

/// Options given as a keyword list or a map, read one key at a time. Whatever
/// is left once a decoder is done is an option it does not know.
struct Fields<'a> {
    entries: Vec<(String, Term<'a>)>,
    known: Vec<&'static str>,
}

impl<'a> Fields<'a> {
    fn of(term: Option<Term<'a>>) -> Decoded<Self> {
        let Some(term) = term else {
            return Ok(Self {
                entries: Vec::new(),
                known: Vec::new(),
            });
        };

        let pairs: Vec<(Term<'a>, Term<'a>)> = match MapIterator::new(term) {
            Some(map) => map.collect(),
            None => term
                .decode()
                .map_err(|_| format!("expected a keyword list or a map, got: {}", inspect(term)))?,
        };

        let entries = pairs
            .into_iter()
            .map(|(key, value)| match atom_name(key) {
                Some(name) => Ok((name, value)),
                None => Err(format!("expected an atom key, got: {}", inspect(key))),
            })
            .collect::<Decoded<_>>()?;

        Ok(Self {
            entries,
            known: Vec::new(),
        })
    }

    /// The value of `key`, if given. A repeated key reads as its first value,
    /// like `Keyword.get/2`.
    fn take(&mut self, key: &'static str) -> Option<Term<'a>> {
        self.known.push(key);
        let value = self
            .entries
            .iter()
            .find(|(name, _)| name == key)
            .map(|(_, value)| *value);
        self.entries.retain(|(name, _)| name != key);
        value
    }

    /// `key` decoded with `decode`, or `None` when it is absent or `nil`.
    fn optional<T>(
        &mut self,
        key: &'static str,
        decode: impl FnOnce(Term<'a>) -> Decoded<T>,
    ) -> Decoded<Option<T>> {
        match self.take(key) {
            Some(term) if !is_nil(term) => decode(term)
                .map(Some)
                .map_err(|error| format!("invalid value for :{key} option: {error}")),
            _ => Ok(None),
        }
    }

    /// A boolean `key`, `false` when absent.
    fn flag(&mut self, key: &'static str) -> Decoded<bool> {
        self.take(key).map_or(Ok(false), |term| {
            boolean(term).map_err(|error| format!("invalid value for :{key} option: {error}"))
        })
    }

    fn finish(self) -> Decoded<()> {
        match self.entries.first() {
            Some((key, _)) => Err(format!(
                "unknown option {}{}, valid options are: {}",
                atom(key),
                suggestion(key, self.known.iter().copied(), |name| format!(":{name}")),
                atom_list(&self.known)
            )),
            None => Ok(()),
        }
    }
}

fn atom_name(term: Term<'_>) -> Option<String> {
    if term.is_atom() {
        term.atom_to_string().ok()
    } else {
        None
    }
}

/// `term` as Elixir's `inspect/1` would print it, close enough for an error
/// message: a caller wrote `"yes"`, not `<<"yes">>`. Deep or long values are
/// cut short.
fn inspect(term: Term<'_>) -> String {
    const LIMIT: usize = 200;

    let mut out = String::new();
    write_term(&mut out, term, 0);
    if out.len() > LIMIT {
        let mut end = LIMIT;
        while !out.is_char_boundary(end) {
            end -= 1;
        }
        out.truncate(end);
        out.push_str("...");
    }
    out
}

fn write_term(out: &mut String, term: Term<'_>, depth: usize) {
    if depth > 4 {
        out.push_str("...");
    } else if term.is_tuple() {
        let elements = rustler::types::tuple::get_tuple(term).unwrap_or_default();
        out.push('{');
        write_sequence(out, elements.into_iter(), depth);
        out.push('}');
    } else if let Ok(elements) = term.decode::<Vec<Term<'_>>>() {
        write_list(out, elements, depth);
    } else if let Some(entries) = MapIterator::new(term) {
        write_map(out, entries.collect(), depth);
    } else {
        write_scalar(out, term);
    }
}

fn write_scalar(out: &mut String, term: Term<'_>) {
    let written = if let Some(name) = atom_name(term) {
        out.push_str(&atom(&name));
        Ok(())
    } else if let Some(string) = term
        .is_binary()
        .then(|| term.decode::<String>().ok())
        .flatten()
    {
        write!(out, "{string:?}")
    } else if let Ok(integer) = term.decode::<i64>() {
        write!(out, "{integer}")
    } else if let Ok(float) = term.decode::<f64>() {
        write!(out, "{float:?}")
    } else {
        write!(out, "{term:?}")
    };
    written.expect("writing to a String cannot fail");
}

fn write_list(out: &mut String, elements: Vec<Term<'_>>, depth: usize) {
    out.push('[');
    if is_keyword(&elements) {
        write_pairs(out, elements.into_iter().filter_map(pair), depth);
    } else {
        write_sequence(out, elements.into_iter(), depth);
    }
    out.push(']');
}

/// `%{...}`, or `%Module{...}` for a struct.
fn write_map<'a>(out: &mut String, mut entries: Vec<(Term<'a>, Term<'a>)>, depth: usize) {
    let module = entries
        .iter()
        .position(|(key, _)| atom_name(*key).as_deref() == Some("__struct__"))
        .map(|index| entries.remove(index).1)
        .and_then(atom_name);

    out.push('%');
    if let Some(module) = module {
        out.push_str(&atom(&module));
    }
    out.push('{');
    write_pairs(out, entries.into_iter(), depth);
    out.push('}');
}

fn write_sequence<'a>(out: &mut String, elements: impl Iterator<Item = Term<'a>>, depth: usize) {
    for (index, element) in elements.enumerate() {
        if index > 0 {
            out.push_str(", ");
        }
        write_term(out, element, depth + 1);
    }
}

/// `key: value` for an atom key, `key => value` for anything else.
fn write_pairs<'a>(
    out: &mut String,
    pairs: impl Iterator<Item = (Term<'a>, Term<'a>)>,
    depth: usize,
) {
    for (index, (key, value)) in pairs.enumerate() {
        if index > 0 {
            out.push_str(", ");
        }
        if let Some(key) = keyword_key(key) {
            out.push_str(&key);
            out.push_str(": ");
        } else {
            write_term(out, key, depth + 1);
            out.push_str(" => ");
        }
        write_term(out, value, depth + 1);
    }
}

fn pair(term: Term<'_>) -> Option<(Term<'_>, Term<'_>)> {
    term.decode().ok()
}

fn is_keyword(elements: &[Term<'_>]) -> bool {
    !elements.is_empty()
        && elements
            .iter()
            .all(|element| pair(*element).is_some_and(|(key, _)| keyword_key(key).is_some()))
}

/// How an atom key reads in `[key: value]`: `id`, or `"data id"` when quoted.
/// `nil`, `true`, `false` and modules have no such form.
fn keyword_key(key: Term<'_>) -> Option<String> {
    atom_name(key)
        .map(|name| atom(&name))
        .and_then(|quoted| quoted.strip_prefix(':').map(str::to_string))
}

/// An atom as Elixir writes it: `nil`, `true` and `false` bare, a module as its
/// alias, `:name` when the name needs no quotes and `:"a name"` when it does.
fn atom(name: &str) -> String {
    if matches!(name, "nil" | "true" | "false") {
        return name.to_string();
    }

    if let Some(alias) = name.strip_prefix("Elixir.") {
        return alias.to_string();
    }

    let mut chars = name.chars();
    let identifier = chars
        .next()
        .is_some_and(|first| first.is_ascii_lowercase() || first == '_')
        && name
            .trim_end_matches(['?', '!'])
            .chars()
            .all(|char| char.is_ascii_alphanumeric() || char == '_' || char == '@');

    if identifier {
        format!(":{name}")
    } else {
        format!(":{name:?}")
    }
}

fn is_nil(term: Term<'_>) -> bool {
    atom_name(term).as_deref() == Some("nil")
}

/// Whether `term` is a struct of `module`, spelled as Elixir names it, such as
/// `Elixir.Range`.
fn is_struct(term: Term<'_>, module: &str) -> bool {
    term.is_map()
        && Atom::from_str(term.get_env(), "__struct__")
            .and_then(|key| term.map_get(key))
            .ok()
            .and_then(atom_name)
            .is_some_and(|name| name == module)
}

fn struct_field<'a>(term: Term<'a>, name: &str) -> Decoded<Term<'a>> {
    Atom::from_str(term.get_env(), name)
        .and_then(|key| term.map_get(key))
        .map_err(|_| format!("missing :{name} in {}", inspect(term)))
}

fn string(term: Term<'_>) -> Decoded<String> {
    if term.is_binary() {
        term.decode()
            .map_err(|_| format!("expected a UTF-8 string, got: {}", inspect(term)))
    } else {
        Err(format!("expected a string, got: {}", inspect(term)))
    }
}

fn boolean(term: Term<'_>) -> Decoded<bool> {
    term.decode()
        .map_err(|_| format!("expected a boolean, got: {}", inspect(term)))
}

fn list(term: Term<'_>) -> Decoded<Vec<Term<'_>>> {
    term.decode()
        .map_err(|_| format!("expected a list, got: {}", inspect(term)))
}

/// `:budget`'s `{time_limit, match_limit}`.
fn budget(term: Term<'_>) -> Decoded<(Option<u64>, Option<u32>)> {
    let mut fields = Fields::of(Some(term))?;
    let time_limit = fields.optional("time_limit", |term| {
        term.decode::<u64>().map_err(|_| {
            format!(
                "expected a non-negative integer of milliseconds, got: {}",
                inspect(term)
            )
        })
    })?;
    let match_limit = fields.optional("match_limit", |term| match term.decode::<u32>() {
        Ok(limit) if (1..=65_536).contains(&limit) => Ok(limit),
        _ => Err(format!(
            "expected an integer in 1..65536, got: {}",
            inspect(term)
        )),
    })?;
    fields.finish()?;

    Ok((time_limit, match_limit))
}

fn positive_integer(term: Term<'_>) -> Decoded<usize> {
    match term.decode::<usize>() {
        Ok(value) if value > 0 => Ok(value),
        _ => Err(format!(
            "expected a positive integer, got: {}",
            inspect(term)
        )),
    }
}

fn structure(term: Term<'_>) -> Decoded<ExHtmlStructure> {
    match atom_name(term).as_deref() {
        Some("block") => Ok(ExHtmlStructure::Block),
        Some("inline") => Ok(ExHtmlStructure::Inline),
        _ => Err(format!(
            "expected :block or :inline, got: {}",
            inspect(term)
        )),
    }
}

/// A theme name or a `%Lumis.Theme{}`. Names are matched case-insensitively,
/// and a Helix-style name with spaces still finds its Neovim spelling.
fn theme(term: Term<'_>) -> Decoded<ThemeOrString> {
    if term.is_binary() {
        let name = string(term)?;
        return Ok(ThemeOrString::String(name.to_lowercase().replace(' ', "")));
    }

    if is_struct(term, "Elixir.Lumis.Theme") {
        return term
            .decode()
            .map(ThemeOrString::Theme)
            .map_err(|_| format!("expected a valid %Lumis.Theme{{}}, got: {}", inspect(term)));
    }

    Err(format!(
        "expected a theme name or a %Lumis.Theme{{}}, got: {}",
        inspect(term)
    ))
}

/// The `:themes` of `html_multi_themes`: a keyword list of identifiers to theme
/// names or `%Lumis.Theme{}` structs. A name has to match a built-in theme
/// exactly.
fn named_themes(term: Term<'_>) -> Decoded<HashMap<String, ExTheme>> {
    let fields = Fields::of(Some(term))?;
    if fields.entries.is_empty() {
        return Err("themes list cannot be empty".to_string());
    }

    fields
        .entries
        .into_iter()
        .map(|(id, value)| {
            let theme = if value.is_binary() {
                let name = string(value)?;
                themes::get(&name)
                    .map(|theme| ExTheme::from(&theme))
                    .map_err(|_| {
                        format!(
                            "failed to resolve theme :{id}: theme '{name}' not found{}",
                            suggestion(
                                &name,
                                themes::available_themes().map(|theme| theme.name.as_str()),
                                |name| format!("'{name}'"),
                            )
                        )
                    })?
            } else if is_struct(value, "Elixir.Lumis.Theme") {
                value
                    .decode()
                    .map_err(|_| format!("expected a valid %Lumis.Theme{{}}, got: {}", inspect(value)))?
            } else {
                return Err(format!("failed to resolve theme :{id}: expected a theme name or a %Lumis.Theme{{}}, got: {}", inspect(value)));
            };
            Ok((id, theme))
        })
        .collect()
}

/// HTML attributes as a keyword list of names to strings or booleans.
fn attrs(term: Term<'_>) -> Decoded<Vec<(String, ExAttrValue)>> {
    let pairs: Vec<(Term<'_>, Term<'_>)> = term.decode().map_err(|_| {
        format!(
            "expected a keyword list of HTML attributes, got: {}",
            inspect(term)
        )
    })?;

    pairs
        .into_iter()
        .map(|(name, value)| {
            let name = atom_name(name).ok_or_else(|| {
                format!("expected an atom attribute name, got: {}", inspect(name))
            })?;
            if !is_valid_attr_name(&name) {
                return Err(format!(
                    "`{name}` is not a name HTML can carry on an attribute"
                ));
            }
            let value = value.decode().map_err(|_| {
                format!(
                    "expected a string or a boolean for :{name}, got: {}",
                    inspect(value)
                )
            })?;
            Ok((name, value))
        })
        .collect()
}

fn header(term: Term<'_>) -> Decoded<ExHtmlElement> {
    let mut fields = Fields::of(Some(term))?;
    let open_tag = fields.optional("open_tag", string)?;
    let close_tag = fields.optional("close_tag", string)?;
    fields.finish()?;

    Ok(ExHtmlElement {
        open_tag: open_tag.ok_or("required :open_tag option not found")?,
        close_tag: close_tag.ok_or("required :close_tag option not found")?,
    })
}

fn line_spec(term: Term<'_>) -> Decoded<ExLineSpec> {
    if term.is_integer() {
        return line_number(term).map(ExLineSpec::Single);
    }

    if is_struct(term, "Elixir.Range") {
        let step = struct_field(term, "step")?;
        return Ok(ExLineSpec::Range {
            start: line_number(struct_field(term, "first")?)?,
            end: line_number(struct_field(term, "last")?)?,
            step: step
                .decode()
                .map_err(|_| format!("expected an integer step, got: {}", inspect(step)))?,
        });
    }

    Err(format!(
        "expected a line number or a range, got: {}",
        inspect(term)
    ))
}

fn line_number(term: Term<'_>) -> Decoded<usize> {
    term.decode().map_err(|_| {
        format!(
            "expected a non-negative line number, got: {}",
            inspect(term)
        )
    })
}

fn lines(fields: &mut Fields<'_>) -> Decoded<Vec<ExLineSpec>> {
    let lines = fields.optional("lines", |term| {
        let specs: Vec<Term<'_>> = term
            .decode()
            .map_err(|_| format!("expected a list, got: {}", inspect(term)))?;
        specs.into_iter().map(line_spec).collect()
    })?;
    Ok(lines.unwrap_or_default())
}

fn inline_highlight_lines(term: Term<'_>) -> Decoded<ExHtmlInlineHighlightLines> {
    let mut fields = Fields::of(Some(term))?;
    let lines = lines(&mut fields)?;

    // `:theme` unless given; an explicit `nil` asks for no style at all.
    let style = match fields.take("style") {
        None => Some(ExHtmlInlineHighlightLinesStyle::Theme),
        Some(style) if is_nil(style) => None,
        Some(style) if atom_name(style).as_deref() == Some("theme") => {
            Some(ExHtmlInlineHighlightLinesStyle::Theme)
        }
        Some(style) => Some(ExHtmlInlineHighlightLinesStyle::Style {
            style: string(style).map_err(|_| {
                format!(
                    "invalid value for :style option: expected :theme, a string, or nil, got: {}",
                    inspect(style)
                )
            })?,
        }),
    };
    let class = fields.optional("class", string)?;
    fields.finish()?;

    Ok(ExHtmlInlineHighlightLines {
        lines,
        style,
        class,
    })
}

fn linked_highlight_lines(term: Term<'_>) -> Decoded<ExHtmlLinkedHighlightLines> {
    let mut fields = Fields::of(Some(term))?;
    let lines = lines(&mut fields)?;
    let class = fields
        .optional("class", string)?
        .unwrap_or_else(|| "l-highlighted".to_string());
    fields.finish()?;

    Ok(ExHtmlLinkedHighlightLines { lines, class })
}

fn terminal_highlight_lines(term: Term<'_>) -> Decoded<ExTerminalHighlightLines> {
    let mut fields = Fields::of(Some(term))?;
    let lines = lines(&mut fields)?;
    let background = fields.optional("background", string)?;
    fields.finish()?;

    Ok(ExTerminalHighlightLines { lines, background })
}

fn bbcode_highlight_lines(term: Term<'_>) -> Decoded<ExBBCodeHighlightLines> {
    let mut fields = Fields::of(Some(term))?;
    let lines = lines(&mut fields)?;
    fields.finish()?;

    Ok(ExBBCodeHighlightLines { lines })
}

fn terminal_background(term: Term<'_>) -> Decoded<ExTerminalBackground> {
    if atom_name(term).as_deref() == Some("theme") {
        return Ok(ExTerminalBackground::Theme);
    }

    string(term)
        .map(ExTerminalBackground::String)
        .map_err(|_| format!("expected :theme or a color string, got: {}", inspect(term)))
}

/// Raised as Elixir's `ArgumentError`.
struct ArgumentError(String);

impl Encoder for ArgumentError {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let atom = |name| Atom::from_str(env, name).expect("a short atom name is valid");

        Term::map_new(env)
            .map_put(atom("__struct__"), atom("Elixir.ArgumentError"))
            .and_then(|map| map.map_put(atom("__exception__"), true))
            .and_then(|map| map.map_put(atom("message"), self.0.as_str()))
            .expect("a new map accepts new keys")
    }
}

/// ` (did you mean X?)` for the candidate closest to `given`, or nothing when
/// none is close. The threshold is the one Elixir's own suggestions use.
fn suggestion<'n>(
    given: &str,
    candidates: impl IntoIterator<Item = &'n str>,
    show: impl Fn(&str) -> String,
) -> String {
    candidates
        .into_iter()
        .map(|candidate| (strsim::jaro_winkler(given, candidate), candidate))
        .filter(|(similarity, _)| *similarity >= 0.8)
        .max_by(|(left, _), (right, _)| left.total_cmp(right))
        .map(|(_, candidate)| format!(" (did you mean {}?)", show(candidate)))
        .unwrap_or_default()
}

/// Raises `message` as Elixir's `ArgumentError`, for a caller that decodes
/// with [`ExLumisOptions::from_term`] and adds where the options came from.
pub fn argument_error(message: String) -> Error {
    Error::RaiseTerm(Box::new(ArgumentError(message)))
}

#[cfg(test)]
mod tests {
    use super::{convert_line_specs, ExLineSpec};

    #[test]
    fn partitions_contiguous_and_genuinely_stepped_line_specs() {
        let (contiguous, stepped) = convert_line_specs(vec![
            ExLineSpec::Single(2),
            ExLineSpec::Range {
                start: 3,
                end: 5,
                step: 1,
            },
            ExLineSpec::Range {
                start: 9,
                end: 7,
                step: -1,
            },
            ExLineSpec::Range {
                start: 1,
                end: 9,
                step: 2,
            },
        ]);

        assert_eq!(contiguous, [2..=2, 3..=5, 7..=9]);
        assert_eq!(stepped.len(), 1);
        assert!(stepped[0].contains(7));
        assert!(!stepped[0].contains(8));
    }
}

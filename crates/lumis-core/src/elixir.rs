//! Elixir-facing decoders for the Lumis option surface.
//!
//! Both `lumis_nif` and `mdex_native_nif` decode the same option maps from the
//! BEAM. One copy lives here so the wire format cannot drift between them.

use crate::formatter::{
    html_inline, html_linked, BBCodeScopedBuilder, Formatter, HtmlElement, HtmlInlineBuilder,
    HtmlLinkedBuilder, HtmlMultiThemesBuilder, TerminalBackground, TerminalBuilder,
};
use crate::{languages::Language, themes};
use rustler::{Encoder, NifMap, NifStruct, NifTaggedEnum, NifUnitEnum};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, NifUnitEnum)]
pub enum ExAppearance {
    Light,
    #[default]
    Dark,
}

#[derive(Clone, Debug, NifTaggedEnum)]
pub enum ExFormatterOption {
    HtmlInline {
        theme: Option<ThemeOrString>,
        pre_class: Option<String>,
        italic: bool,
        include_highlights: bool,
        rainbow_brackets: bool,
        highlight_lines: Option<ExHtmlInlineHighlightLines>,
        header: Option<ExHtmlElement>,
    },
    HtmlLinked {
        pre_class: Option<String>,
        rainbow_brackets: bool,
        highlight_lines: Option<ExHtmlLinkedHighlightLines>,
        header: Option<ExHtmlElement>,
    },
    HtmlMultiThemes {
        themes: HashMap<String, ExTheme>,
        default_theme: Option<String>,
        css_variable_prefix: Option<String>,
        pre_class: Option<String>,
        italic: bool,
        include_highlights: bool,
        rainbow_brackets: bool,
        highlight_lines: Option<ExHtmlInlineHighlightLines>,
        header: Option<ExHtmlElement>,
    },
    Terminal {
        theme: Option<ThemeOrString>,
        background: Option<ExTerminalBackground>,
        width: Option<usize>,
        rainbow_brackets: bool,
    },
    BbcodeScoped {
        rainbow_brackets: bool,
    },
}

#[derive(Clone, Debug, NifTaggedEnum)]
pub enum ExTerminalBackground {
    Theme,
    String(String),
}

impl Default for ExFormatterOption {
    fn default() -> Self {
        Self::HtmlInline {
            theme: None,
            pre_class: None,
            italic: false,
            include_highlights: false,
            rainbow_brackets: false,
            highlight_lines: None,
            header: None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum ThemeOrString {
    Theme(ExTheme),
    String(String),
}

/// Also accepts a bare name, which is how `mdex_native` has always spelled a
/// theme in `{:html_inline, theme: "onedark"}`. The tagged forms `{:string,
/// name}` and `{:theme, theme}` keep working.
impl<'a> rustler::Decoder<'a> for ThemeOrString {
    fn decode(term: rustler::Term<'a>) -> rustler::NifResult<Self> {
        if let Ok(name) = String::decode(term) {
            return Ok(Self::String(name));
        }

        let (kind, value): (rustler::Atom, rustler::Term<'a>) = term.decode()?;
        let env = term.get_env();

        if kind == rustler::Atom::from_str(env, "string")? {
            return Ok(Self::String(value.decode()?));
        }

        if kind == rustler::Atom::from_str(env, "theme")? {
            return Ok(Self::Theme(value.decode()?));
        }

        Err(rustler::Error::BadArg)
    }
}

impl rustler::Encoder for ThemeOrString {
    fn encode<'a>(&self, env: rustler::Env<'a>) -> rustler::Term<'a> {
        match self {
            Self::Theme(theme) => {
                (rustler::Atom::from_str(env, "theme").unwrap(), theme).encode(env)
            }
            Self::String(name) => {
                (rustler::Atom::from_str(env, "string").unwrap(), name).encode(env)
            }
        }
    }
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
fn convert_line_specs(lines: Vec<ExLineSpec>) -> Vec<std::ops::RangeInclusive<usize>> {
    lines
        .into_iter()
        .map(|line_spec| line_spec.to_range_inclusive())
        .collect()
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

impl ExFormatterOption {
    pub fn into_formatter(self, language: Language) -> Result<(Box<dyn Formatter>, bool), String> {
        match self {
            ExFormatterOption::HtmlInline {
                theme,
                pre_class,
                italic,
                include_highlights,
                rainbow_brackets,
                highlight_lines,
                header,
            } => {
                let theme = theme.and_then(resolve_theme);

                let highlight_lines = highlight_lines.map(|hl| html_inline::HighlightLines {
                    lines: convert_line_specs(hl.lines),
                    style: hl.style.map(convert_inline_style),
                    class: hl.class,
                });

                let header = header.map(|h| HtmlElement {
                    open_tag: h.open_tag,
                    close_tag: h.close_tag,
                });

                let formatter = HtmlInlineBuilder::new()
                    .language(language)
                    .theme(theme)
                    .pre_class(pre_class)
                    .italic(italic)
                    .include_highlights(include_highlights)
                    .highlight_lines(highlight_lines)
                    .header(header)
                    .build()
                    .map_err(|e| format!("HtmlInline builder error: {e:?}"))?;

                Ok((Box::new(formatter), rainbow_brackets))
            }
            ExFormatterOption::HtmlLinked {
                pre_class,
                rainbow_brackets,
                highlight_lines,
                header,
            } => {
                let highlight_lines = highlight_lines.map(|hl| html_linked::HighlightLines {
                    lines: convert_line_specs(hl.lines),
                    class: hl.class,
                });

                let header = header.map(|h| HtmlElement {
                    open_tag: h.open_tag,
                    close_tag: h.close_tag,
                });

                let formatter = HtmlLinkedBuilder::new()
                    .language(language)
                    .pre_class(pre_class)
                    .highlight_lines(highlight_lines)
                    .header(header)
                    .build()
                    .map_err(|e| format!("HtmlLinked builder error: {e:?}"))?;

                Ok((Box::new(formatter), rainbow_brackets))
            }
            ExFormatterOption::HtmlMultiThemes {
                themes,
                default_theme,
                css_variable_prefix,
                pre_class,
                italic,
                include_highlights,
                rainbow_brackets,
                highlight_lines,
                header,
            } => {
                let themes_map: HashMap<String, themes::Theme> =
                    themes.into_iter().map(|(k, v)| (k, v.into())).collect();

                let highlight_lines = highlight_lines.map(|hl| html_inline::HighlightLines {
                    lines: convert_line_specs(hl.lines),
                    style: hl.style.map(convert_inline_style),
                    class: hl.class,
                });

                let header = header.map(|h| HtmlElement {
                    open_tag: h.open_tag,
                    close_tag: h.close_tag,
                });

                let mut builder = HtmlMultiThemesBuilder::new();
                builder
                    .language(language)
                    .themes(themes_map)
                    .css_variable_prefix(css_variable_prefix.as_deref().unwrap_or("--lumis"))
                    .pre_class(pre_class)
                    .italic(italic)
                    .include_highlights(include_highlights)
                    .highlight_lines(highlight_lines)
                    .header(header);

                if let Some(dt_str) = default_theme {
                    builder.default_theme(dt_str);
                }

                let formatter = builder
                    .build()
                    .map_err(|e| format!("HtmlMultiThemes builder error: {e:?}"))?;

                Ok((Box::new(formatter), rainbow_brackets))
            }
            ExFormatterOption::Terminal {
                theme,
                background,
                width,
                rainbow_brackets,
            } => {
                let theme = theme.and_then(resolve_theme);
                let background = match background {
                    Some(ExTerminalBackground::Theme) => TerminalBackground::Theme,
                    Some(ExTerminalBackground::String(color)) => TerminalBackground::Color(color),
                    None => TerminalBackground::Inherit,
                };

                let formatter = TerminalBuilder::new()
                    .language(language)
                    .theme(theme)
                    .background(background)
                    .width(width)
                    .build()
                    .map_err(|e| format!("Terminal builder error: {e:?}"))?;

                Ok((Box::new(formatter), rainbow_brackets))
            }
            ExFormatterOption::BbcodeScoped { rainbow_brackets } => {
                let formatter = BBCodeScopedBuilder::new()
                    .language(language)
                    .build()
                    .map_err(|e| format!("BBCode scoped builder error: {e:?}"))?;

                Ok((Box::new(formatter), rainbow_brackets))
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
                .map(|(k, v)| {
                    (
                        k,
                        themes::Style {
                            fg: v.fg,
                            bg: v.bg,
                            bold: v.bold,
                            italic: v.italic,
                            text_decoration: v.text_decoration.into(),
                        },
                    )
                })
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

#[derive(Clone, Debug, Default, NifStruct)]
#[module = "Lumis.HtmlElement"]
pub struct ExHtmlElement {
    pub open_tag: String,
    pub close_tag: String,
}

#[derive(Clone, Debug, NifTaggedEnum)]
pub enum ExLineSpec {
    Single(usize),
    Range { start: usize, end: usize },
}

impl ExLineSpec {
    fn to_range_inclusive(&self) -> std::ops::RangeInclusive<usize> {
        match self {
            ExLineSpec::Single(line) => *line..=*line,
            ExLineSpec::Range { start, end } => *start..=*end,
        }
    }

    #[allow(dead_code)]
    fn from_range_inclusive(range: std::ops::RangeInclusive<usize>) -> Self {
        let start = *range.start();
        let end = *range.end();
        if start == end {
            ExLineSpec::Single(start)
        } else {
            ExLineSpec::Range { start, end }
        }
    }
}

#[derive(Clone, Debug, Default, NifTaggedEnum)]
pub enum ExHtmlInlineHighlightLinesStyle {
    #[default]
    Theme,
    Style {
        style: String,
    },
}

#[derive(Clone, Debug, Default, NifStruct)]
#[module = "Lumis.HtmlInlineHighlightLines"]
pub struct ExHtmlInlineHighlightLines {
    pub lines: Vec<ExLineSpec>,
    pub style: Option<ExHtmlInlineHighlightLinesStyle>,
    pub class: Option<String>,
}

#[derive(Clone, Debug, Default, NifStruct)]
#[module = "Lumis.HtmlLinkedHighlightLines"]
pub struct ExHtmlLinkedHighlightLines {
    pub lines: Vec<ExLineSpec>,
    pub class: String,
}

#[derive(Clone, Debug, NifMap)]
pub struct ExCssOptions {
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

impl From<html_inline::HighlightLinesStyle> for ExHtmlInlineHighlightLinesStyle {
    fn from(style: html_inline::HighlightLinesStyle) -> Self {
        match style {
            html_inline::HighlightLinesStyle::Theme => ExHtmlInlineHighlightLinesStyle::Theme,
            html_inline::HighlightLinesStyle::Style(s) => {
                ExHtmlInlineHighlightLinesStyle::Style { style: s }
            }
        }
    }
}

impl From<html_inline::HighlightLines> for ExHtmlInlineHighlightLines {
    fn from(highlight_lines: html_inline::HighlightLines) -> Self {
        ExHtmlInlineHighlightLines {
            lines: highlight_lines
                .lines
                .into_iter()
                .map(ExLineSpec::from_range_inclusive)
                .collect(),
            style: highlight_lines.style.map(std::convert::Into::into),
            class: highlight_lines.class,
        }
    }
}

impl From<html_linked::HighlightLines> for ExHtmlLinkedHighlightLines {
    fn from(highlight_lines: html_linked::HighlightLines) -> Self {
        ExHtmlLinkedHighlightLines {
            lines: highlight_lines
                .lines
                .into_iter()
                .map(ExLineSpec::from_range_inclusive)
                .collect(),
            class: highlight_lines.class,
        }
    }
}

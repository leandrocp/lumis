use lumis_core::formatter::{
    html::SteppedLineRange, html_inline, html_linked, BBCodeScopedBuilder, Formatter, HtmlElement,
    HtmlInlineBuilder, HtmlLinkedBuilder, HtmlMultiThemesBuilder, TerminalBackground,
    TerminalBuilder,
};
use lumis_core::{languages::Language, themes};
use rustler::{NifMap, NifStruct, NifTaggedEnum, NifUnitEnum};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, NifUnitEnum)]
pub enum ExAppearance {
    Light,
    #[default]
    Dark,
}

#[derive(Debug, NifTaggedEnum)]
pub enum ExFormatterOption {
    HtmlInline {
        theme: Option<ThemeOrString>,
        pre_class: Option<String>,
        italic: bool,
        include_highlights: bool,
        highlight_lines: Option<ExHtmlInlineHighlightLines>,
        header: Option<ExHtmlElement>,
    },
    HtmlLinked {
        pre_class: Option<String>,
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
        highlight_lines: Option<ExHtmlInlineHighlightLines>,
        header: Option<ExHtmlElement>,
    },
    Terminal {
        theme: Option<ThemeOrString>,
        background: Option<ExTerminalBackground>,
        width: Option<usize>,
    },
    BbcodeScoped {},
}

#[derive(Debug, NifTaggedEnum)]
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
            highlight_lines: None,
            header: None,
        }
    }
}

#[derive(Debug, NifTaggedEnum)]
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

pub(crate) fn line_specs_contain(lines: &[ExLineSpec], line_number: usize) -> bool {
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
                theme,
                pre_class,
                italic,
                include_highlights,
                highlight_lines,
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
                    .theme(theme)
                    .pre_class(pre_class)
                    .italic(italic)
                    .include_highlights(include_highlights)
                    .highlight_lines(highlight_lines)
                    .header(header)
                    .build()
                    .map_err(|e| format!("HtmlInline builder error: {e:?}"))?;
                formatter.set_stepped_highlight_lines(stepped_highlight_lines);

                Ok(Box::new(formatter))
            }
            ExFormatterOption::HtmlLinked {
                pre_class,
                highlight_lines,
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
                    .pre_class(pre_class)
                    .highlight_lines(highlight_lines)
                    .header(header)
                    .build()
                    .map_err(|e| format!("HtmlLinked builder error: {e:?}"))?;
                formatter.set_stepped_highlight_lines(stepped_highlight_lines);

                Ok(Box::new(formatter))
            }
            ExFormatterOption::HtmlMultiThemes {
                themes,
                default_theme,
                css_variable_prefix,
                pre_class,
                italic,
                include_highlights,
                highlight_lines,
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

                Ok(Box::new(formatter))
            }
            ExFormatterOption::BbcodeScoped {} => {
                let formatter = BBCodeScopedBuilder::new()
                    .language(language)
                    .build()
                    .map_err(|e| format!("BBCode scoped builder error: {e:?}"))?;

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

#[derive(Clone, Debug, Default, NifStruct)]
#[module = "Lumis.HTMLElement"]
pub struct ExHtmlElement {
    pub open_tag: String,
    pub close_tag: String,
}

#[derive(Clone, Debug, NifTaggedEnum)]
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

    #[allow(dead_code)]
    fn from_range_inclusive(range: std::ops::RangeInclusive<usize>) -> Self {
        let start = *range.start();
        let end = *range.end();
        if start == end {
            ExLineSpec::Single(start)
        } else {
            ExLineSpec::Range {
                start,
                end,
                step: 1,
            }
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
#[module = "Lumis.HTMLInlineHighlightLines"]
pub struct ExHtmlInlineHighlightLines {
    pub lines: Vec<ExLineSpec>,
    pub style: Option<ExHtmlInlineHighlightLinesStyle>,
    pub class: Option<String>,
}

#[derive(Clone, Debug, Default, NifStruct)]
#[module = "Lumis.HTMLLinkedHighlightLines"]
pub struct ExHtmlLinkedHighlightLines {
    pub lines: Vec<ExLineSpec>,
    pub class: String,
}

#[derive(Clone, Debug, NifMap)]
pub(crate) struct ExCssOptions {
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

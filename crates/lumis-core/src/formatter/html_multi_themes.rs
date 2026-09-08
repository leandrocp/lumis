//! HTML formatter with multiple theme support using CSS variables.
//!
//! Works with pre-computed highlight events from any source.

use super::{Formatter, HtmlElement};
use crate::events::HighlightEvent;
use crate::formatter::html_inline::HighlightLines;
use crate::languages::Language;
use crate::themes::Theme;
use derive_builder::Builder;
use std::collections::HashMap;
use std::io::{self, Write};
use std::str::FromStr;

/// Configuration for which theme to use as the default (inline styles).
#[derive(Clone, Debug)]
pub enum DefaultTheme {
    /// Use a specific named theme as the default (e.g., "light", "dark")
    Theme(String),
    /// Use CSS `light-dark()` function (requires light and dark themes)
    LightDark,
}

impl FromStr for DefaultTheme {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "light-dark()" => DefaultTheme::LightDark,
            theme_name => DefaultTheme::Theme(theme_name.to_string()),
        })
    }
}

/// HTML formatter with multiple theme support.
#[derive(Builder, Clone, Debug)]
#[builder(default, build_fn(skip))]
pub struct HtmlMultiThemes {
    #[builder(setter(custom))]
    language: Language,
    themes: HashMap<String, Theme>,
    #[builder(setter(custom))]
    default_theme: Option<DefaultTheme>,
    #[builder(setter(into))]
    css_variable_prefix: String,
    pre_class: Option<String>,
    italic: bool,
    include_highlights: bool,
    highlight_lines: Option<HighlightLines>,
    header: Option<HtmlElement>,
}

impl HtmlMultiThemesBuilder {
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

    pub fn default_theme<T: Into<DefaultThemeArg>>(&mut self, value: T) -> &mut Self {
        self.default_theme = Some(value.into().into_enum());
        self
    }

    pub fn build(&mut self) -> Result<HtmlMultiThemes, String> {
        let result = HtmlMultiThemes {
            language: self.language.take().unwrap_or(Language::PlainText),
            themes: self.themes.take().unwrap_or_default(),
            default_theme: self.default_theme.take().flatten(),
            css_variable_prefix: self
                .css_variable_prefix
                .take()
                .unwrap_or_else(|| "--lumis".to_string()),
            pre_class: self.pre_class.take().flatten(),
            italic: self.italic.take().unwrap_or(false),
            include_highlights: self.include_highlights.take().unwrap_or(false),
            highlight_lines: self.highlight_lines.take().flatten(),
            header: self.header.take().flatten(),
        };

        if result.themes.is_empty() {
            return Err("At least one theme is required".to_string());
        }

        match &result.default_theme {
            Some(DefaultTheme::Theme(name)) if !result.themes.contains_key(name) => {
                return Err(format!("Default theme '{name}' not found in themes map"));
            }
            Some(DefaultTheme::LightDark)
                if !result.themes.contains_key("light") || !result.themes.contains_key("dark") =>
            {
                return Err("LightDark mode requires themes named 'light' and 'dark'".to_string());
            }
            _ => {}
        }

        Ok(result)
    }
}

/// Argument type for the `default_theme` builder method.
#[doc(hidden)]
pub enum DefaultThemeArg {
    String(String),
    Bool(bool),
}

impl DefaultThemeArg {
    fn into_enum(self) -> Option<DefaultTheme> {
        match self {
            DefaultThemeArg::String(s) => Some(s.parse().unwrap()),
            DefaultThemeArg::Bool(false) => None,
            DefaultThemeArg::Bool(true) => Some(DefaultTheme::Theme("light".to_string())),
        }
    }
}

impl From<&str> for DefaultThemeArg {
    fn from(s: &str) -> Self {
        DefaultThemeArg::String(s.to_string())
    }
}

impl From<String> for DefaultThemeArg {
    fn from(s: String) -> Self {
        DefaultThemeArg::String(s)
    }
}

impl From<bool> for DefaultThemeArg {
    fn from(b: bool) -> Self {
        DefaultThemeArg::Bool(b)
    }
}

impl Default for HtmlMultiThemes {
    fn default() -> Self {
        Self {
            language: Language::PlainText,
            themes: HashMap::new(),
            default_theme: None,
            css_variable_prefix: "--lumis".to_string(),
            pre_class: None,
            italic: false,
            include_highlights: false,
            highlight_lines: None,
            header: None,
        }
    }
}

impl HtmlMultiThemes {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        language: Language,
        themes: HashMap<String, Theme>,
        default_theme: Option<DefaultTheme>,
        css_variable_prefix: String,
        pre_class: Option<String>,
        italic: bool,
        include_highlights: bool,
        highlight_lines: Option<HighlightLines>,
        header: Option<HtmlElement>,
    ) -> Self {
        Self {
            language,
            themes,
            default_theme,
            css_variable_prefix,
            pre_class,
            italic,
            include_highlights,
            highlight_lines,
            header,
        }
    }

    fn open_pre_tag(&self, output: &mut dyn Write) -> io::Result<()> {
        crate::formatter::html::open_multi_themes_pre_tag(
            output,
            self.pre_class.as_deref(),
            &self.themes,
            self.default_theme_name(),
            &self.css_variable_prefix,
        )
    }

    fn default_theme_name(&self) -> Option<&str> {
        match &self.default_theme {
            Some(DefaultTheme::Theme(name)) => Some(name.as_str()),
            Some(DefaultTheme::LightDark) => Some("light-dark()"),
            None => None,
        }
    }

    fn get_line_attrs(&self, line_number: usize) -> (Option<String>, Option<String>) {
        let is_highlighted = self
            .highlight_lines
            .as_ref()
            .is_some_and(|hl| hl.lines.iter().any(|r| r.contains(&line_number)));

        if !is_highlighted {
            return (None, None);
        }

        let class_suffix = self
            .highlight_lines
            .as_ref()
            .and_then(|hl| hl.class.as_ref())
            .map(|c| format!(" {c}"));

        let style = self.get_highlight_style();

        (class_suffix, style)
    }

    fn get_highlight_style(&self) -> Option<String> {
        use crate::formatter::html_inline::HighlightLinesStyle;

        let highlight_lines = self.highlight_lines.as_ref()?;

        match &highlight_lines.style {
            Some(HighlightLinesStyle::Theme) => match &self.default_theme {
                Some(DefaultTheme::Theme(default_name)) => {
                    let theme = self.themes.get(default_name)?;
                    let highlighted_style = theme.get_style("highlighted")?;
                    Some(highlighted_style.css(self.italic, " "))
                }
                Some(DefaultTheme::LightDark) => self.light_dark_highlight_style(),
                None => None,
            },
            Some(HighlightLinesStyle::Style(style_string)) => Some(style_string.clone()),
            None => None,
        }
    }

    fn light_dark_highlight_style(&self) -> Option<String> {
        let light = self.themes.get("light")?.get_style("highlighted")?;
        let dark = self.themes.get("dark")?.get_style("highlighted")?;
        let light_bg = light.bg.as_ref()?;
        let dark_bg = dark.bg.as_ref()?;

        Some(format!(
            "background-color: light-dark({light_bg}, {dark_bg});"
        ))
    }

    fn span_attrs_from_index(&self, scope_index: usize, language: &str) -> String {
        let scope = crate::highlights::HIGHLIGHT_NAMES
            .get(scope_index)
            .copied()
            .unwrap_or("");
        let lang = language.parse::<Language>().ok();
        crate::formatter::html::span_multi_themes_attrs(
            scope,
            lang,
            &self.themes,
            self.default_theme_name(),
            &self.css_variable_prefix,
            self.italic,
            self.include_highlights,
        )
    }
}

impl<T> Formatter<T> for HtmlMultiThemes {
    fn render(
        &self,
        source: &str,
        events: &[HighlightEvent<'_, T>],
        output: &mut dyn Write,
    ) -> io::Result<()> {
        let mut buffer = Vec::new();

        if let Some(ref header) = self.header {
            write!(buffer, "{}", header.open_tag)?;
        }

        self.open_pre_tag(&mut buffer)?;
        crate::formatter::html::open_code_tag(&mut buffer, &self.language)?;

        let lines = crate::formatter::html::render_lines_from_events(
            source,
            events,
            |scope_index, language| self.span_attrs_from_index(scope_index, language),
        );

        for (i, line) in lines.iter().enumerate() {
            let line_number = i + 1;
            let line_with_newline = format!("{line}\n");
            let (class_suffix, style) = self.get_line_attrs(line_number);
            let wrapped = crate::formatter::html::wrap_line(
                line_number,
                &line_with_newline,
                class_suffix.as_deref(),
                style.as_deref(),
            );
            write!(&mut buffer, "{wrapped}")?;
        }

        crate::formatter::html::closing_tags(&mut buffer)?;

        if let Some(ref header) = self.header {
            write!(buffer, "{}", header.close_tag)?;
        }

        output.write_all(&buffer)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn render_uses_multi_themes_pre_tag_helper() {
        let mut themes = HashMap::new();
        themes.insert(
            "light".to_string(),
            crate::themes::get("catppuccin_latte").unwrap(),
        );
        themes.insert(
            "dark".to_string(),
            crate::themes::get("catppuccin_mocha").unwrap(),
        );

        let formatter = HtmlMultiThemes::new(
            Language::PlainText,
            themes,
            Some(DefaultTheme::LightDark),
            "--lumis".to_string(),
            Some("custom-pre".to_string()),
            false,
            false,
            None,
            None,
        );
        let mut output = Vec::new();
        let events: [HighlightEvent<'_, ()>; 0] = [];

        formatter.render("", &events, &mut output).unwrap();

        let html = String::from_utf8(output).unwrap();
        let pre_tag = html.split_once('>').expect("missing pre tag").0;

        assert_classes(
            pre_tag,
            &["lumis", "lumis-themes", "custom-pre", "light", "dark"],
        );
        assert_eq!(
            attr_value(pre_tag, "style"),
            "color: light-dark(#4c4f69, #cdd6f4); background-color: light-dark(#eff1f5, #1e1e2e);"
        );
    }

    fn render_highlighted_line(
        default_theme: DefaultTheme,
        light_background: Option<&str>,
        dark_background: Option<&str>,
    ) -> String {
        use crate::formatter::html_inline::{HighlightLines, HighlightLinesStyle};

        let mut themes = HashMap::new();
        let mut light = crate::themes::get("catppuccin_latte").unwrap();
        let light_highlighted = light.highlights.get_mut("highlighted").unwrap();
        light_highlighted.fg = Some("#111111".to_string());
        light_highlighted.bg = light_background.map(str::to_string);
        themes.insert("light".to_string(), light);

        let mut dark = crate::themes::get("catppuccin_mocha").unwrap();
        let dark_highlighted = dark.highlights.get_mut("highlighted").unwrap();
        dark_highlighted.fg = Some("#eeeeee".to_string());
        dark_highlighted.bg = dark_background.map(str::to_string);
        themes.insert("dark".to_string(), dark);

        let formatter = HtmlMultiThemes::new(
            Language::PlainText,
            themes,
            Some(default_theme),
            "--lumis".to_string(),
            None,
            false,
            false,
            Some(HighlightLines {
                lines: std::iter::once(1..=1).collect(),
                style: Some(HighlightLinesStyle::Theme),
                class: None,
            }),
            None,
        );
        let mut output = Vec::new();

        Formatter::<()>::render(&formatter, "one\ntwo", &[], &mut output).unwrap();

        let html = String::from_utf8(output).unwrap();
        let line_tag = html
            .split_once("<div ")
            .expect("missing first line")
            .1
            .split_once('>')
            .expect("unterminated line tag")
            .0;
        assert!(
            line_tag.contains(r#"data-line="1""#),
            "wrong line: {line_tag}"
        );

        line_tag.to_string()
    }

    #[test]
    fn theme_highlight_lines_follow_the_colour_scheme() {
        let line_tag =
            render_highlighted_line(DefaultTheme::LightDark, Some("#e9ebf1"), Some("#2a2b3c"));
        assert_eq!(
            attr_value(&line_tag, "style"),
            "background-color: light-dark(#e9ebf1, #2a2b3c);"
        );
    }

    #[test]
    fn light_dark_theme_highlight_lines_require_a_light_background() {
        let line_tag = render_highlighted_line(DefaultTheme::LightDark, None, Some("#2a2b3c"));

        assert!(
            !line_tag.contains(" style="),
            "unexpected style: {line_tag}"
        );
    }

    #[test]
    fn light_dark_theme_highlight_lines_require_a_dark_background() {
        let line_tag = render_highlighted_line(DefaultTheme::LightDark, Some("#e9ebf1"), None);

        assert!(
            !line_tag.contains(" style="),
            "unexpected style: {line_tag}"
        );
    }

    #[test]
    fn theme_highlight_lines_use_the_named_default_theme() {
        let line_tag = render_highlighted_line(
            DefaultTheme::Theme("dark".to_string()),
            Some("#e9ebf1"),
            Some("#2a2b3c"),
        );
        assert_eq!(
            attr_value(&line_tag, "style"),
            "color: #eeeeee; background-color: #2a2b3c;"
        );
    }
}

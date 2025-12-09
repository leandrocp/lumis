//! HTML formatter with multiple theme support using CSS variables.
//!
//! Generates HTML with inline styles for a default theme and CSS variables for alternate themes.
//! Theme switching happens via CSS without JavaScript or re-rendering.
//!
//! Inspired by [Shiki's dual-themes pattern](https://shiki.style/guide/dual-themes).
//!
//! # Usage
//!
//! ```rust
//! use autumnus::{HtmlMultiThemesBuilder, languages::Language, themes, formatter::Formatter};
//! use std::collections::HashMap;
//!
//! let mut theme_map = HashMap::new();
//! theme_map.insert("light".to_string(), themes::get("github_light").unwrap());
//! theme_map.insert("dark".to_string(), themes::get("github_dark").unwrap());
//!
//! let formatter = HtmlMultiThemesBuilder::new()
//!     .lang(Language::Rust)
//!     .themes(theme_map)
//!     .default_theme("light")
//!     .build()
//!     .unwrap();
//!
//! let mut output = Vec::new();
//! formatter.format("fn main() {}", &mut output).unwrap();
//! ```
//!
//! # How It Works
//!
//! Generated HTML includes inline colors for the default theme and CSS variables for alternates:
//!
//! ```html
//! <span style="color:#d73a49; --athl-dark:#ff7b72;">keyword</span>
//! ```
//!
//! # CSS You Must Provide
//!
//! Like Shiki, NO CSS is injected. You must provide CSS to activate theme switching.
//!
//! **Option 1: OS Preference (automatic dark mode)**
//! ```css
//! @media (prefers-color-scheme: dark) {
//!   .athl-themes, .athl-themes span {
//!     color: var(--athl-dark) !important;
//!     background-color: var(--athl-dark-bg) !important;
//!   }
//! }
//! ```
//!
//! **Option 2: Manual switching with `data-theme` attribute**
//! ```css
//! html[data-theme="dark"] .athl-themes,
//! html[data-theme="dark"] .athl-themes span {
//!   color: var(--athl-dark) !important;
//!   background-color: var(--athl-dark-bg) !important;
//! }
//! ```
//!
//! **Option 3: Class-based switching**
//! ```css
//! html.dark .athl-themes, html.dark .athl-themes span {
//!   color: var(--athl-dark) !important;
//!   background-color: var(--athl-dark-bg) !important;
//! }
//! ```
//!
//! **Option 4: CSS `light-dark()` function (modern browsers)**
//!
//! For browsers that support the [CSS `light-dark()` function](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Values/color_value/light-dark), you can use a more elegant approach:
//!
//! ```rust
//! use autumnus::{HtmlMultiThemesBuilder, languages::Language, themes, formatter::Formatter};
//! use std::collections::HashMap;
//!
//! let mut theme_map = HashMap::new();
//! theme_map.insert("light".to_string(), themes::get("github_light").unwrap());
//! theme_map.insert("dark".to_string(), themes::get("github_dark").unwrap());
//!
//! let formatter = HtmlMultiThemesBuilder::new()
//!     .lang(Language::Rust)
//!     .themes(theme_map)
//!     .default_theme("light-dark()")
//!     .build()
//!     .unwrap();
//!
//! let mut output = Vec::new();
//! formatter.format("fn main() {}", &mut output).unwrap();
//! ```
//!
//! This generates HTML using the native `light-dark()` CSS function:
//!
//! ```html
//! <span style="color: light-dark(#d73a49, #ff7b72);">keyword</span>
//! ```
//!
//! The browser automatically selects the appropriate color based on `color-scheme` or
//! `prefers-color-scheme` without any additional CSS required.
//!
//! **Note**: Requires themes named exactly "light" and "dark". Only works in browsers
//! supporting the CSS `light-dark()` function (Chrome 123+, Safari 17.5+, Firefox 120+).
//!
//! See [html_multi_themesi.rs](https://github.com/leandrocp/autumnus/blob/main/examples/html_multi_themesi.rs)
//! and [html_multi_themesi.html](https://github.com/leandrocp/autumnus/blob/main/examples/html_multi_themesi.html) for a demo.
//!

use super::{Formatter, HtmlElement};
use crate::formatter::html_inline::HighlightLines;
use crate::languages::Language;
use crate::themes::Theme;
use derive_builder::Builder;
use std::collections::HashMap;
use std::io::{self, Write};
use std::str::FromStr;
use tree_sitter_highlight::Highlighter;

/// Configuration for which theme to use as the default (inline styles).
///
/// The default theme's colors are rendered as direct inline styles (e.g., `color:#d73a49`),
/// while other themes are defined as CSS variables (e.g., `--athl-dark:#ff7b72`).
#[derive(Clone, Debug)]
pub enum DefaultTheme {
    /// Use a specific named theme as the default (e.g., "light", "dark")
    Theme(String),
    /// No default theme - all themes are CSS variables only
    None,
    /// Use CSS `light-dark()` function (requires light and dark themes)
    ///
    /// Generates inline styles using the CSS `light-dark(light-color, dark-color)` function.
    /// The browser automatically switches between colors based on color-scheme preference.
    /// Requires themes named exactly "light" and "dark" in the themes map.
    LightDark,
}

impl FromStr for DefaultTheme {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "light-dark()" => DefaultTheme::LightDark,
            "false" | "none" => DefaultTheme::None,
            theme_name => DefaultTheme::Theme(theme_name.to_string()),
        })
    }
}

/// HTML formatter with multiple theme support.
///
/// This formatter generates HTML with inline CSS styles for a default theme and CSS variables
/// for alternate themes. Create instances using [`HtmlMultiThemesBuilder`].
///
/// # Examples
///
/// ```rust
/// use autumnus::{HtmlMultiThemesBuilder, languages::Language, themes, formatter::Formatter};
/// use std::collections::HashMap;
///
/// let mut themes = HashMap::new();
/// themes.insert("light".to_string(), themes::get("github_light").unwrap());
/// themes.insert("dark".to_string(), themes::get("github_dark").unwrap());
///
/// let formatter = HtmlMultiThemesBuilder::new()
///     .lang(Language::Rust)
///     .themes(themes)
///     .default_theme("light")
///     .build()
///     .unwrap();
/// ```
#[derive(Builder, Debug)]
#[builder(default, build_fn(skip))]
pub struct HtmlMultiThemes<'a> {
    lang: Language,
    themes: HashMap<String, Theme>,
    #[builder(setter(custom))]
    default_theme: DefaultTheme,
    #[builder(setter(into))]
    css_variable_prefix: String,
    pre_class: Option<&'a str>,
    italic: bool,
    include_highlights: bool,
    highlight_lines: Option<HighlightLines>,
    header: Option<HtmlElement>,
}

/// Builder for creating [`HtmlMultiThemes`] formatters.
///
/// Provides a type-safe API for configuring multiple theme support.
///
/// # Examples
///
/// ```rust
/// use autumnus::{HtmlMultiThemesBuilder, languages::Language, themes};
/// use std::collections::HashMap;
///
/// let mut themes = HashMap::new();
/// themes.insert("light".to_string(), themes::get("github_light").unwrap());
/// themes.insert("dark".to_string(), themes::get("github_dark").unwrap());
///
/// let formatter = HtmlMultiThemesBuilder::new()
///     .lang(Language::Rust)
///     .themes(themes)
///     .default_theme("light")
///     .css_variable_prefix("--my-app")
///     .build()
///     .unwrap();
/// ```
impl<'a> HtmlMultiThemesBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn default_theme<T: Into<DefaultThemeArg>>(&mut self, value: T) -> &mut Self {
        self.default_theme = Some(value.into().into_enum());
        self
    }

    pub fn build(&mut self) -> Result<HtmlMultiThemes<'a>, String> {
        let mut result = HtmlMultiThemes {
            lang: self.lang.take().unwrap_or(Language::PlainText),
            themes: self.themes.take().unwrap_or_default(),
            default_theme: self.default_theme.take().unwrap_or(DefaultTheme::None),
            css_variable_prefix: self
                .css_variable_prefix
                .take()
                .unwrap_or_else(|| "--athl".to_string()),
            pre_class: self.pre_class.take().flatten(),
            italic: self.italic.take().unwrap_or(false),
            include_highlights: self.include_highlights.take().unwrap_or(false),
            highlight_lines: self.highlight_lines.take().flatten(),
            header: self.header.take().flatten(),
        };

        if result.themes.is_empty() {
            return Err("At least one theme is required".to_string());
        }

        if matches!(result.default_theme, DefaultTheme::None) {
            let first_theme_name = result.themes.keys().next().unwrap().clone();
            result.default_theme = DefaultTheme::Theme(first_theme_name);
        }

        match &result.default_theme {
            DefaultTheme::Theme(name) => {
                if !result.themes.contains_key(name) {
                    return Err(format!("Default theme '{}' not found in themes map", name));
                }
            }
            DefaultTheme::LightDark => {
                if !result.themes.contains_key("light") || !result.themes.contains_key("dark") {
                    return Err(
                        "LightDark mode requires themes named 'light' and 'dark'".to_string()
                    );
                }
            }
            DefaultTheme::None => {}
        }

        Ok(result)
    }
}

/// Argument type for the `default_theme` builder method.
///
/// Accepts either a string theme name or a boolean value.
/// This is an internal type used for ergonomic API design.
#[doc(hidden)]
pub enum DefaultThemeArg {
    String(String),
    Bool(bool),
}

impl DefaultThemeArg {
    fn into_enum(self) -> DefaultTheme {
        match self {
            DefaultThemeArg::String(s) => s.parse().unwrap(),
            DefaultThemeArg::Bool(false) => DefaultTheme::None,
            DefaultThemeArg::Bool(true) => DefaultTheme::Theme("light".to_string()),
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

impl Default for HtmlMultiThemes<'_> {
    fn default() -> Self {
        Self {
            lang: Language::PlainText,
            themes: HashMap::new(),
            default_theme: DefaultTheme::None,
            css_variable_prefix: "--athl".to_string(),
            pre_class: None,
            italic: false,
            include_highlights: false,
            highlight_lines: None,
            header: None,
        }
    }
}

impl<'a> HtmlMultiThemes<'a> {
    fn sanitize_theme_name(name: &str) -> String {
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

    fn generate_pre_classes(&self) -> String {
        let mut classes = vec!["athl".to_string(), "athl-themes".to_string()];

        if let Some(pre_class) = self.pre_class {
            classes.push(pre_class.to_string());
        }

        for theme_name in self.themes.keys() {
            classes.push(theme_name.clone());
        }

        classes.join(" ")
    }

    fn generate_pre_style(&self) -> io::Result<String> {
        let mut styles = Vec::new();

        match &self.default_theme {
            DefaultTheme::Theme(default_name) => {
                if let Some(default_theme) = self.themes.get(default_name) {
                    if let Some(fg) = default_theme.fg() {
                        styles.push(format!("color:{};", fg));
                    }
                    if let Some(bg) = default_theme.bg() {
                        styles.push(format!("background-color:{};", bg));
                    }
                }

                for (theme_name, theme) in &self.themes {
                    if theme_name != default_name {
                        let sanitized = Self::sanitize_theme_name(theme_name);
                        if let Some(fg) = theme.fg() {
                            styles.push(format!(
                                "{}-{}:{};",
                                self.css_variable_prefix, sanitized, fg
                            ));
                        }
                        if let Some(bg) = theme.bg() {
                            styles.push(format!(
                                "{}-{}-bg:{};",
                                self.css_variable_prefix, sanitized, bg
                            ));
                        }
                    }
                }
            }
            DefaultTheme::None => {
                for (theme_name, theme) in &self.themes {
                    let sanitized = Self::sanitize_theme_name(theme_name);
                    if let Some(fg) = theme.fg() {
                        styles.push(format!(
                            "{}-{}: {};",
                            self.css_variable_prefix, sanitized, fg
                        ));
                    }
                    if let Some(bg) = theme.bg() {
                        styles.push(format!(
                            "{}-{}-bg: {};",
                            self.css_variable_prefix, sanitized, bg
                        ));
                    }
                }
            }
            DefaultTheme::LightDark => {
                if let (Some(light), Some(dark)) =
                    (self.themes.get("light"), self.themes.get("dark"))
                {
                    let light_fg = light.fg().unwrap_or_else(|| "#000000".to_string());
                    let light_bg = light.bg().unwrap_or_else(|| "#ffffff".to_string());
                    let dark_fg = dark.fg().unwrap_or_else(|| "#ffffff".to_string());
                    let dark_bg = dark.bg().unwrap_or_else(|| "#000000".to_string());

                    styles.push(format!("color: light-dark({}, {});", light_fg, dark_fg));
                    styles.push(format!(
                        "background-color: light-dark({}, {});",
                        light_bg, dark_bg
                    ));
                }
            }
        }

        Ok(styles.join(" "))
    }

    fn open_pre_tag(&self, output: &mut dyn Write) -> io::Result<()> {
        let classes = self.generate_pre_classes();
        let style = self.generate_pre_style()?;

        write!(output, "<pre class=\"{}\"", classes)?;
        if !style.is_empty() {
            write!(output, " style=\"{}\"", style)?;
        }
        write!(output, ">")
    }

    fn render_token_style(&self, scope: &str, output: &mut Vec<u8>) {
        let mut inline_styles = Vec::new();
        let mut css_vars = Vec::new();

        match &self.default_theme {
            DefaultTheme::Theme(default_name) => {
                if let Some(default_theme) = self.themes.get(default_name) {
                    if let Some(style) = default_theme.get_style(scope) {
                        if let Some(fg) = &style.fg {
                            inline_styles.push(format!("color:{};", fg));
                        }
                        if let Some(bg) = &style.bg {
                            inline_styles.push(format!("background-color:{};", bg));
                        }
                        if style.bold {
                            inline_styles.push("font-weight:bold;".to_string());
                        }
                        if self.italic && style.italic {
                            inline_styles.push("font-style:italic;".to_string());
                        }
                        if style.underline || style.strikethrough {
                            let mut decorations = Vec::new();
                            if style.underline {
                                decorations.push("underline");
                            }
                            if style.strikethrough {
                                decorations.push("line-through");
                            }
                            inline_styles
                                .push(format!("text-decoration:{};", decorations.join(" ")));
                        }
                    }
                }

                for (theme_name, theme) in &self.themes {
                    if theme_name != default_name {
                        if let Some(style) = theme.get_style(scope) {
                            let sanitized = Self::sanitize_theme_name(theme_name);

                            if let Some(fg) = &style.fg {
                                css_vars.push(format!(
                                    "{}-{}:{};",
                                    self.css_variable_prefix, sanitized, fg
                                ));
                            }
                            if let Some(bg) = &style.bg {
                                css_vars.push(format!(
                                    "{}-{}-bg:{};",
                                    self.css_variable_prefix, sanitized, bg
                                ));
                            }
                        }
                    }
                }
            }
            DefaultTheme::None => {
                for (theme_name, theme) in &self.themes {
                    if let Some(style) = theme.get_style(scope) {
                        let sanitized = Self::sanitize_theme_name(theme_name);

                        if let Some(fg) = &style.fg {
                            css_vars.push(format!(
                                "{}-{}: {};",
                                self.css_variable_prefix, sanitized, fg
                            ));
                        }
                        if let Some(bg) = &style.bg {
                            css_vars.push(format!(
                                "{}-{}-bg: {};",
                                self.css_variable_prefix, sanitized, bg
                            ));
                        }
                    }
                }
            }
            DefaultTheme::LightDark => {
                if let (Some(light), Some(dark)) =
                    (self.themes.get("light"), self.themes.get("dark"))
                {
                    if let (Some(light_style), Some(dark_style)) =
                        (light.get_style(scope), dark.get_style(scope))
                    {
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
                        if light_style.bold || dark_style.bold {
                            let light_weight = if light_style.bold { "bold" } else { "normal" };
                            let dark_weight = if dark_style.bold { "bold" } else { "normal" };
                            inline_styles.push(format!(
                                "font-weight: light-dark({}, {});",
                                light_weight, dark_weight
                            ));
                        }
                        if self.italic && (light_style.italic || dark_style.italic) {
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
                    }
                }
            }
        }

        if !inline_styles.is_empty() || !css_vars.is_empty() {
            if self.include_highlights {
                output.extend(b" ");
            }
            output.extend(b"style=\"");

            if !inline_styles.is_empty() {
                output.extend(inline_styles.join(" ").as_bytes());
            }
            if !css_vars.is_empty() {
                if !inline_styles.is_empty() {
                    output.extend(b" ");
                }
                output.extend(css_vars.join(" ").as_bytes());
            }

            output.extend(b"\"");
        }
    }

    fn write_line(
        &self,
        output: &mut dyn Write,
        line_number: usize,
        content: &str,
    ) -> io::Result<()> {
        let is_highlighted = self
            .highlight_lines
            .as_ref()
            .is_some_and(|hl| hl.lines.iter().any(|r| r.contains(&line_number)));

        write!(output, "<div class=\"line")?;

        if is_highlighted {
            if let Some(class) = self
                .highlight_lines
                .as_ref()
                .and_then(|hl| hl.class.as_ref())
            {
                write!(output, " {}", class)?;
            }
        }

        write!(output, "\"")?;

        if is_highlighted {
            if let Some(style_str) = self.get_highlight_style() {
                write!(output, " style=\"{}\"", style_str)?;
            }
        }

        write!(output, " data-line=\"{}\">{}</div>", line_number, content)
    }

    fn get_highlight_style(&self) -> Option<String> {
        use crate::formatter::html_inline::HighlightLinesStyle;

        let highlight_lines = self.highlight_lines.as_ref()?;

        match &highlight_lines.style {
            Some(HighlightLinesStyle::Theme) => {
                if let DefaultTheme::Theme(default_name) = &self.default_theme {
                    let theme = self.themes.get(default_name)?;
                    let highlighted_style = theme.get_style("highlighted")?;
                    Some(highlighted_style.css(self.italic, " "))
                } else {
                    None
                }
            }
            Some(HighlightLinesStyle::Style(style_string)) => Some(style_string.clone()),
            None => None,
        }
    }
}

impl Formatter for HtmlMultiThemes<'_> {
    fn format(&self, source: &str, output: &mut dyn Write) -> io::Result<()> {
        let mut buffer = Vec::new();

        if let Some(ref header) = self.header {
            write!(buffer, "{}", header.open_tag)?;
        }

        self.open_pre_tag(&mut buffer)?;
        crate::formatter::html::open_code_tag(&mut buffer, &self.lang)?;

        let mut highlighter = Highlighter::new();
        let events = highlighter
            .highlight(self.lang.config(), source.as_bytes(), None, |injected| {
                Some(Language::guess(Some(injected), "").config())
            })
            .map_err(io::Error::other)?;

        let mut renderer = tree_sitter_highlight::HtmlRenderer::new();

        renderer
            .render(events, source.as_bytes(), &move |highlight, output| {
                let scope = crate::constants::HIGHLIGHT_NAMES[highlight.0];

                if self.include_highlights {
                    output.extend("data-highlight=\"".as_bytes());
                    output.extend(scope.as_bytes());
                    output.extend(b"\"");
                }

                self.render_token_style(scope, output);
            })
            .map_err(io::Error::other)?;

        for (i, line) in renderer.lines().enumerate() {
            let line_number = i + 1;
            let line_with_braces = line.replace('{', "&lbrace;").replace('}', "&rbrace;");
            self.write_line(&mut buffer, line_number, &line_with_braces)?;
        }

        crate::formatter::html::closing_tags(&mut buffer)?;

        if let Some(ref header) = self.header {
            write!(buffer, "{}", header.close_tag)?;
        }

        write!(output, "{}", &String::from_utf8_lossy(&buffer))?;
        Ok(())
    }
}

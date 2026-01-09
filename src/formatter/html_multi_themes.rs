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
//! Generated HTML includes inline colors and font styles for the default theme, plus CSS
//! variables for all themes (including font styles):
//!
//! ```html
//! <span style="color:#d73a49; font-weight:bold; --athl-light:#d73a49; --athl-light-font-weight:bold; --athl-dark:#ff7b72; --athl-dark-font-weight:normal;">keyword</span>
//! ```
//!
//! **Note**: Multi-theme formatter generates a larger HTML payload due to CSS variables for
//! each theme. If you only need a single theme, use [`HtmlInline`](crate::formatter::HtmlInline) instead.
//!
//! # CSS You Must Provide
//!
//! Like Shiki, NO CSS is injected. You must provide CSS to activate theme switching.
//!
//! **Option 1: OS Preference (automatic dark mode)**
//! ```css
//! @media (prefers-color-scheme: dark) {
//!   .athl,
//!   .athl span {
//!     color: var(--athl-dark) !important;
//!     background-color: var(--athl-dark-bg) !important;
//!     font-style: var(--athl-dark-font-style) !important;
//!     font-weight: var(--athl-dark-font-weight) !important;
//!     text-decoration: var(--athl-dark-text-decoration) !important;
//!   }
//! }
//! ```
//!
//! **Option 2: Manual switching with `data-theme` attribute**
//! ```css
//! html[data-theme="dark"] .athl,
//! html[data-theme="dark"] .athl span {
//!   color: var(--athl-dark) !important;
//!   background-color: var(--athl-dark-bg) !important;
//!   font-style: var(--athl-dark-font-style) !important;
//!   font-weight: var(--athl-dark-font-weight) !important;
//!   text-decoration: var(--athl-dark-text-decoration) !important;
//! }
//! ```
//!
//! **Option 3: Class-based switching**
//! ```css
//! html.dark .athl,
//! html.dark .athl span {
//!   color: var(--athl-dark) !important;
//!   background-color: var(--athl-dark-bg) !important;
//!   /* Optional, if you also want font styles */
//!   font-style: var(--athl-dark-font-style) !important;
//!   font-weight: var(--athl-dark-font-weight) !important;
//!   text-decoration: var(--athl-dark-text-decoration) !important;
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
use crate::vendor::tree_sitter_highlight::{Highlighter, HtmlRenderer};
use derive_builder::Builder;
use std::collections::HashMap;
use std::io::{self, Write};
use std::str::FromStr;

/// Configuration for which theme to use as the default (inline styles).
///
/// The default theme's colors are rendered as direct inline styles (e.g., `color:#d73a49`),
/// while other themes are defined as CSS variables (e.g., `--athl-dark:#ff7b72`).
///
/// Use `Option<DefaultTheme>` where `None` means no default theme (all CSS variables only).
#[derive(Clone, Debug)]
pub enum DefaultTheme {
    /// Use a specific named theme as the default (e.g., "light", "dark")
    Theme(String),
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
#[derive(Builder, Clone, Debug)]
#[builder(default, build_fn(skip))]
pub struct HtmlMultiThemes {
    lang: Language,
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
impl HtmlMultiThemesBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn default_theme<T: Into<DefaultThemeArg>>(&mut self, value: T) -> &mut Self {
        self.default_theme = Some(value.into().into_enum());
        self
    }

    pub fn build(&mut self) -> Result<HtmlMultiThemes, String> {
        let result = HtmlMultiThemes {
            lang: self.lang.take().unwrap_or(Language::PlainText),
            themes: self.themes.take().unwrap_or_default(),
            default_theme: self.default_theme.take().flatten(),
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

        match &result.default_theme {
            Some(DefaultTheme::Theme(name)) => {
                if !result.themes.contains_key(name) {
                    return Err(format!("Default theme '{}' not found in themes map", name));
                }
            }
            Some(DefaultTheme::LightDark) => {
                if !result.themes.contains_key("light") || !result.themes.contains_key("dark") {
                    return Err(
                        "LightDark mode requires themes named 'light' and 'dark'".to_string()
                    );
                }
            }
            None => {
                // No default theme - all themes are CSS variables only
            }
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
            lang: Language::PlainText,
            themes: HashMap::new(),
            default_theme: None,
            css_variable_prefix: "--athl".to_string(),
            pre_class: None,
            italic: false,
            include_highlights: false,
            highlight_lines: None,
            header: None,
        }
    }
}

impl HtmlMultiThemes {
    fn generate_pre_classes(&self) -> String {
        let mut classes = vec!["athl".to_string(), "athl-themes".to_string()];

        if let Some(ref pre_class) = self.pre_class {
            classes.push(pre_class.clone());
        }

        for theme_name in self.themes.keys() {
            classes.push(theme_name.clone());
        }

        classes.join(" ")
    }

    fn generate_pre_style(&self) -> io::Result<String> {
        let mut styles = Vec::new();

        match &self.default_theme {
            Some(DefaultTheme::Theme(default_name)) => {
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
                        let sanitized = crate::formatter::html::sanitize_theme_name(theme_name);
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
            Some(DefaultTheme::LightDark) => {
                if let (Some(light), Some(dark)) =
                    (self.themes.get("light"), self.themes.get("dark"))
                {
                    let light_fg = light.fg().unwrap_or("#000000");
                    let light_bg = light.bg().unwrap_or("#ffffff");
                    let dark_fg = dark.fg().unwrap_or("#ffffff");
                    let dark_bg = dark.bg().unwrap_or("#000000");

                    styles.push(format!("color: light-dark({}, {});", light_fg, dark_fg));
                    styles.push(format!(
                        "background-color: light-dark({}, {});",
                        light_bg, dark_bg
                    ));
                }
            }
            None => {
                for (theme_name, theme) in &self.themes {
                    let sanitized = crate::formatter::html::sanitize_theme_name(theme_name);
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
            .map(|c| format!(" {}", c));

        let style = self.get_highlight_style();

        (class_suffix, style)
    }

    fn get_highlight_style(&self) -> Option<String> {
        use crate::formatter::html_inline::HighlightLinesStyle;

        let highlight_lines = self.highlight_lines.as_ref()?;

        match &highlight_lines.style {
            Some(HighlightLinesStyle::Theme) => {
                if let Some(DefaultTheme::Theme(default_name)) = &self.default_theme {
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

impl Formatter for HtmlMultiThemes {
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

        let mut renderer = HtmlRenderer::new();

        renderer
            .render(
                events,
                source.as_bytes(),
                &move |highlight, language, output| {
                    let scope = crate::constants::HIGHLIGHT_NAMES[highlight.0];
                    let lang = Language::guess(Some(language), "");
                    let default_theme_str = match &self.default_theme {
                        Some(DefaultTheme::Theme(name)) => Some(name.as_str()),
                        Some(DefaultTheme::LightDark) => Some("light-dark()"),
                        None => None,
                    };
                    let attrs = crate::formatter::html::span_multi_themes_attrs(
                        scope,
                        Some(lang),
                        &self.themes,
                        default_theme_str,
                        &self.css_variable_prefix,
                        self.italic,
                        self.include_highlights,
                    );
                    output.extend(attrs.as_bytes());
                },
            )
            .map_err(io::Error::other)?;

        for (i, line) in renderer.lines().enumerate() {
            let line_number = i + 1;
            let line_with_braces = crate::formatter::html::escape_braces(line);
            let (class_suffix, style) = self.get_line_attrs(line_number);
            let wrapped = crate::formatter::html::wrap_line(
                line_number,
                &line_with_braces,
                class_suffix.as_deref(),
                style.as_deref(),
            );
            write!(&mut buffer, "{}", wrapped)?;
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

    #[test]
    fn test_text_decoration() {
        use crate::formatter::html::text_decoration;
        use crate::themes::{TextDecoration, UnderlineStyle};

        let none = TextDecoration::default();
        assert_eq!(text_decoration(&none), "none");

        let underline = TextDecoration {
            underline: UnderlineStyle::Solid,
            strikethrough: false,
        };
        assert_eq!(text_decoration(&underline), "underline");

        let wavy = TextDecoration {
            underline: UnderlineStyle::Wavy,
            strikethrough: false,
        };
        assert_eq!(text_decoration(&wavy), "underline wavy");

        let strike = TextDecoration {
            underline: UnderlineStyle::None,
            strikethrough: true,
        };
        assert_eq!(text_decoration(&strike), "line-through");

        let both = TextDecoration {
            underline: UnderlineStyle::Solid,
            strikethrough: true,
        };
        assert_eq!(text_decoration(&both), "underline line-through");

        let wavy_strike = TextDecoration {
            underline: UnderlineStyle::Wavy,
            strikethrough: true,
        };
        assert_eq!(text_decoration(&wavy_strike), "underline wavy line-through");
    }

    #[test]
    fn test_theme_mode_generates_font_css_variables() {
        let mut themes = HashMap::new();
        themes.insert(
            "light".to_string(),
            crate::themes::get("github_light").unwrap(),
        );
        themes.insert(
            "dark".to_string(),
            crate::themes::get("github_dark").unwrap(),
        );

        let formatter = HtmlMultiThemesBuilder::new()
            .lang(Language::Rust)
            .themes(themes)
            .default_theme("light")
            .italic(true)
            .build()
            .unwrap();

        let source = "fn main() {}";
        let mut output = Vec::new();
        formatter.format(source, &mut output).unwrap();
        let html = String::from_utf8(output).unwrap();

        assert!(html.contains("--athl-light-font-style:"));
        assert!(html.contains("--athl-dark-font-style:"));
        assert!(html.contains("--athl-light-font-weight:"));
        assert!(html.contains("--athl-dark-font-weight:"));
        assert!(html.contains("--athl-light-text-decoration:"));
        assert!(html.contains("--athl-dark-text-decoration:"));
    }

    #[test]
    fn test_lightdark_mode_includes_text_decoration() {
        let mut themes = HashMap::new();
        themes.insert(
            "light".to_string(),
            crate::themes::get("github_light").unwrap(),
        );
        themes.insert(
            "dark".to_string(),
            crate::themes::get("github_dark").unwrap(),
        );

        let formatter = HtmlMultiThemesBuilder::new()
            .lang(Language::Rust)
            .themes(themes)
            .default_theme("light-dark()")
            .italic(true)
            .build()
            .unwrap();

        let source = "fn main() {}";
        let mut output = Vec::new();
        formatter.format(source, &mut output).unwrap();
        let html = String::from_utf8(output).unwrap();

        assert!(html.contains("font-weight: light-dark("));
        assert!(html.contains("font-style: light-dark("));
        assert!(html.contains("text-decoration: light-dark("));
    }

    #[test]
    fn test_lightdark_mode_always_outputs_font_weight() {
        let mut themes = HashMap::new();
        themes.insert(
            "light".to_string(),
            crate::themes::get("github_light").unwrap(),
        );
        themes.insert(
            "dark".to_string(),
            crate::themes::get("github_dark").unwrap(),
        );

        let formatter = HtmlMultiThemesBuilder::new()
            .lang(Language::Rust)
            .themes(themes)
            .default_theme("light-dark()")
            .build()
            .unwrap();

        let source = "// comment";
        let mut output = Vec::new();
        formatter.format(source, &mut output).unwrap();
        let html = String::from_utf8(output).unwrap();

        assert!(html.contains("font-weight: light-dark(normal, normal)"));
    }

    #[test]
    fn test_none_mode_generates_font_css_variables() {
        let mut themes = HashMap::new();
        themes.insert(
            "light".to_string(),
            crate::themes::get("github_light").unwrap(),
        );
        themes.insert(
            "dark".to_string(),
            crate::themes::get("github_dark").unwrap(),
        );

        let formatter = HtmlMultiThemesBuilder::new()
            .lang(Language::Rust)
            .themes(themes)
            .build()
            .unwrap();

        let source = "fn main() {}";
        let mut output = Vec::new();
        formatter.format(source, &mut output).unwrap();
        let html = String::from_utf8(output).unwrap();

        assert!(html.contains("--athl-light-font-style:"));
        assert!(html.contains("--athl-dark-font-style:"));
        assert!(html.contains("--athl-light-font-weight:"));
        assert!(html.contains("--athl-dark-font-weight:"));
        assert!(html.contains("--athl-light-text-decoration:"));
        assert!(html.contains("--athl-dark-text-decoration:"));
        assert!(!html.contains("font-style:italic;"));
        assert!(!html.contains("font-weight:bold;"));
    }

    #[test]
    fn test_font_style_values_are_correct() {
        let mut themes = HashMap::new();
        themes.insert(
            "light".to_string(),
            crate::themes::get("github_light").unwrap(),
        );
        themes.insert(
            "dark".to_string(),
            crate::themes::get("github_dark").unwrap(),
        );

        let formatter = HtmlMultiThemesBuilder::new()
            .lang(Language::Rust)
            .themes(themes)
            .default_theme("light")
            .italic(true)
            .build()
            .unwrap();

        let source = "fn main() {}";
        let mut output = Vec::new();
        formatter.format(source, &mut output).unwrap();
        let html = String::from_utf8(output).unwrap();

        assert!(
            html.contains("--athl-light-font-style:normal")
                || html.contains("--athl-dark-font-style:normal")
        );
        assert!(
            html.contains("--athl-light-font-weight:normal")
                || html.contains("--athl-dark-font-weight:normal")
        );
        assert!(
            html.contains("--athl-light-text-decoration:none")
                || html.contains("--athl-dark-text-decoration:none")
        );
    }

    #[test]
    fn test_italic_flag_respects_lightdark_mode() {
        let mut themes = HashMap::new();
        themes.insert(
            "light".to_string(),
            crate::themes::get("github_light").unwrap(),
        );
        themes.insert(
            "dark".to_string(),
            crate::themes::get("github_dark").unwrap(),
        );

        let formatter = HtmlMultiThemesBuilder::new()
            .lang(Language::Rust)
            .themes(themes.clone())
            .default_theme("light-dark()")
            .italic(false)
            .build()
            .unwrap();

        let source = "// comment";
        let mut output = Vec::new();
        formatter.format(source, &mut output).unwrap();
        let html = String::from_utf8(output).unwrap();

        assert!(!html.contains("font-style: light-dark("));

        let formatter = HtmlMultiThemesBuilder::new()
            .lang(Language::Rust)
            .themes(themes)
            .default_theme("light-dark()")
            .italic(true)
            .build()
            .unwrap();

        let mut output = Vec::new();
        formatter.format(source, &mut output).unwrap();
        let html = String::from_utf8(output).unwrap();

        assert!(html.contains("font-style: light-dark("));
    }
}

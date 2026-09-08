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
//! use lumis::{HtmlMultiThemesBuilder, languages::Language, themes, formatters::Formatter};
//! use std::collections::HashMap;
//!
//! let mut theme_map = HashMap::new();
//! theme_map.insert("light".to_string(), themes::get("github_light").unwrap());
//! theme_map.insert("dark".to_string(), themes::get("github_dark").unwrap());
//!
//! let formatter = HtmlMultiThemesBuilder::new()
//!     .language(Language::Rust)
//!     .themes(theme_map)
//!     .default_theme("light")
//!     .build()
//!     .unwrap();
//!
//! let mut output = Vec::new();
//! lumis::write_highlight(&mut output, "fn main() {}", formatter).unwrap();
//! ```
//!
//! # How It Works
//!
//! Generated HTML includes inline colors and font styles for the default theme, plus CSS
//! variables for all themes (including font styles):
//!
//! ```html
//! <span style="color:#d73a49; font-weight:bold; --lumis-light:#d73a49; --lumis-light-font-weight:bold; --lumis-dark:#ff7b72; --lumis-dark-font-weight:normal;">keyword</span>
//! ```
//!
//! **Note**: Multi-theme formatter generates a larger HTML payload due to CSS variables for
//! each theme. If you only need a single theme, use [`HtmlInline`](crate::formatters::HtmlInline) instead.
//!
//! # CSS You Must Provide
//!
//! Like Shiki, NO CSS is injected. You must provide CSS to activate theme switching.
//!
//! **Option 1: OS Preference (automatic dark mode)**
//! ```css
//! @media (prefers-color-scheme: dark) {
//!   .lumis,
//!   .lumis span {
//!     color: var(--lumis-dark) !important;
//!     background-color: var(--lumis-dark-bg) !important;
//!     font-style: var(--lumis-dark-font-style) !important;
//!     font-weight: var(--lumis-dark-font-weight) !important;
//!     text-decoration: var(--lumis-dark-text-decoration) !important;
//!   }
//! }
//! ```
//!
//! **Option 2: Manual switching with `data-theme` attribute**
//! ```css
//! html[data-theme="dark"] .lumis,
//! html[data-theme="dark"] .lumis span {
//!   color: var(--lumis-dark) !important;
//!   background-color: var(--lumis-dark-bg) !important;
//!   font-style: var(--lumis-dark-font-style) !important;
//!   font-weight: var(--lumis-dark-font-weight) !important;
//!   text-decoration: var(--lumis-dark-text-decoration) !important;
//! }
//! ```
//!
//! **Option 3: Class-based switching**
//! ```css
//! html.dark .lumis,
//! html.dark .lumis span {
//!   color: var(--lumis-dark) !important;
//!   background-color: var(--lumis-dark-bg) !important;
//!   /* Optional, if you also want font styles */
//!   font-style: var(--lumis-dark-font-style) !important;
//!   font-weight: var(--lumis-dark-font-weight) !important;
//!   text-decoration: var(--lumis-dark-text-decoration) !important;
//! }
//! ```
//!
//! **Option 4: CSS `light-dark()` function (modern browsers)**
//!
//! For browsers that support the [CSS `light-dark()` function](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Values/color_value/light-dark), you can use a more elegant approach:
//!
//! ```rust
//! use lumis::{HtmlMultiThemesBuilder, languages::Language, themes, formatters::Formatter};
//! use std::collections::HashMap;
//!
//! let mut theme_map = HashMap::new();
//! theme_map.insert("light".to_string(), themes::get("github_light").unwrap());
//! theme_map.insert("dark".to_string(), themes::get("github_dark").unwrap());
//!
//! let formatter = HtmlMultiThemesBuilder::new()
//!     .language(Language::Rust)
//!     .themes(theme_map)
//!     .default_theme("light-dark()")
//!     .build()
//!     .unwrap();
//!
//! let mut output = Vec::new();
//! lumis::write_highlight(&mut output, "fn main() {}", formatter).unwrap();
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
//! See [html_multi_themes_light_dark.rs](https://github.com/leandrocp/lumis/blob/main/crates/lumis/examples/html_multi_themes_light_dark.rs),
//! [html_multi_themes_manual.rs](https://github.com/leandrocp/lumis/blob/main/crates/lumis/examples/html_multi_themes_manual.rs),
//! and [html_multi_themes_vars.rs](https://github.com/leandrocp/lumis/blob/main/crates/lumis/examples/html_multi_themes_vars.rs)
//! for end-to-end demos.
//!

use super::{Formatter, HtmlElement};
use crate::formatters::html_inline::HighlightLines;
use crate::languages::Language;
use crate::themes::Theme;
use derive_builder::Builder;
use lumis_core::formatter::Formatter as _;
use std::collections::HashMap;
use std::io::{self, Write};
use std::str::FromStr;

/// Configuration for which theme to use as the default (inline styles).
///
/// The default theme's colors are rendered as direct inline styles (e.g., `color:#d73a49`),
/// while other themes are defined as CSS variables (e.g., `--lumis-dark:#ff7b72`).
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
/// use lumis::{HtmlMultiThemesBuilder, languages::Language, themes, formatters::Formatter};
/// use std::collections::HashMap;
///
/// let mut themes = HashMap::new();
/// themes.insert("light".to_string(), themes::get("github_light").unwrap());
/// themes.insert("dark".to_string(), themes::get("github_dark").unwrap());
///
/// let formatter = HtmlMultiThemesBuilder::new()
///     .language(Language::Rust)
///     .themes(themes)
///     .default_theme("light")
///     .build()
///     .unwrap();
/// ```
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

/// Builder for creating [`HtmlMultiThemes`] formatters.
///
/// Provides a type-safe API for configuring multiple theme support.
///
/// # Examples
///
/// ```rust
/// use lumis::{HtmlMultiThemesBuilder, languages::Language, themes};
/// use std::collections::HashMap;
///
/// let mut themes = HashMap::new();
/// themes.insert("light".to_string(), themes::get("github_light").unwrap());
/// themes.insert("dark".to_string(), themes::get("github_dark").unwrap());
///
/// let formatter = HtmlMultiThemesBuilder::new()
///     .language(Language::Rust)
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
            // No default theme leaves every theme as CSS variables only.
            _ => {}
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

impl<T> Formatter<T> for HtmlMultiThemes {
    fn language(&self) -> Language {
        self.language
    }

    fn render(
        &self,
        source: &str,
        events: &[lumis_core::events::HighlightEvent<'_, T>],
        output: &mut dyn Write,
    ) -> io::Result<()> {
        let core_formatter = lumis_core::formatter::html_multi_themes::HtmlMultiThemes::new(
            self.language,
            self.themes.clone(),
            self.default_theme.clone().map(map_default_theme),
            self.css_variable_prefix.clone(),
            self.pre_class.clone(),
            self.italic,
            self.include_highlights,
            self.highlight_lines
                .clone()
                .map(super::map_inline_highlight_lines),
            self.header.clone(),
        );

        core_formatter.render(source, events, output)
    }
}

fn map_default_theme(
    default_theme: DefaultTheme,
) -> lumis_core::formatter::html_multi_themes::DefaultTheme {
    match default_theme {
        DefaultTheme::Theme(name) => {
            lumis_core::formatter::html_multi_themes::DefaultTheme::Theme(name)
        }
        DefaultTheme::LightDark => {
            lumis_core::formatter::html_multi_themes::DefaultTheme::LightDark
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_decoration() {
        use crate::formatters::html::text_decoration;
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
            .language(Language::Rust)
            .themes(themes)
            .default_theme("light")
            .italic(true)
            .build()
            .unwrap();

        let source = "fn main() {}";
        let mut output = Vec::new();
        crate::write_highlight(&mut output, source, formatter).unwrap();
        let html = String::from_utf8(output).unwrap();

        assert!(html.contains("--lumis-light-font-style:"));
        assert!(html.contains("--lumis-dark-font-style:"));
        assert!(html.contains("--lumis-light-font-weight:"));
        assert!(html.contains("--lumis-dark-font-weight:"));
        assert!(html.contains("--lumis-light-text-decoration:"));
        assert!(html.contains("--lumis-dark-text-decoration:"));
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
            .language(Language::Rust)
            .themes(themes)
            .default_theme("light-dark()")
            .italic(true)
            .build()
            .unwrap();

        let source = "fn main() {}";
        let mut output = Vec::new();
        crate::write_highlight(&mut output, source, formatter).unwrap();
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
            .language(Language::Rust)
            .themes(themes)
            .default_theme("light-dark()")
            .build()
            .unwrap();

        let source = "// comment";
        let mut output = Vec::new();
        crate::write_highlight(&mut output, source, formatter).unwrap();
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
            .language(Language::Rust)
            .themes(themes)
            .build()
            .unwrap();

        let source = "fn main() {}";
        let mut output = Vec::new();
        crate::write_highlight(&mut output, source, formatter).unwrap();
        let html = String::from_utf8(output).unwrap();

        assert!(html.contains("--lumis-light-font-style:"));
        assert!(html.contains("--lumis-dark-font-style:"));
        assert!(html.contains("--lumis-light-font-weight:"));
        assert!(html.contains("--lumis-dark-font-weight:"));
        assert!(html.contains("--lumis-light-text-decoration:"));
        assert!(html.contains("--lumis-dark-text-decoration:"));
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
            .language(Language::Rust)
            .themes(themes)
            .default_theme("light")
            .italic(true)
            .build()
            .unwrap();

        let source = "fn main() {}";
        let mut output = Vec::new();
        crate::write_highlight(&mut output, source, formatter).unwrap();
        let html = String::from_utf8(output).unwrap();

        assert!(
            html.contains("--lumis-light-font-style:normal")
                || html.contains("--lumis-dark-font-style:normal")
        );
        assert!(
            html.contains("--lumis-light-font-weight:normal")
                || html.contains("--lumis-dark-font-weight:normal")
        );
        assert!(
            html.contains("--lumis-light-text-decoration:none")
                || html.contains("--lumis-dark-text-decoration:none")
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
            .language(Language::Rust)
            .themes(themes.clone())
            .default_theme("light-dark()")
            .italic(false)
            .build()
            .unwrap();

        let source = "// comment";
        let mut output = Vec::new();
        crate::write_highlight(&mut output, source, formatter).unwrap();
        let html = String::from_utf8(output).unwrap();

        assert!(!html.contains("font-style: light-dark("));

        let formatter = HtmlMultiThemesBuilder::new()
            .language(Language::Rust)
            .themes(themes)
            .default_theme("light-dark()")
            .italic(true)
            .build()
            .unwrap();

        let mut output = Vec::new();
        crate::write_highlight(&mut output, source, formatter).unwrap();
        let html = String::from_utf8(output).unwrap();

        assert!(html.contains("font-style: light-dark("));
    }
}

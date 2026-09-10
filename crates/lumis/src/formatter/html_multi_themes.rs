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

pub use lumis_core::formatter::html_multi_themes::{
    DefaultTheme, DefaultThemeArg, HtmlMultiThemes, HtmlMultiThemesBuilder,
    HtmlMultiThemesBuilderError,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::languages::Language;
    use std::collections::HashMap;

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

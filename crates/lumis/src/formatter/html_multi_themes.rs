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
//! <span style="color: light-dark(#d73a49, #ff7b72); font-weight: bold;">keyword</span>
//! <span style="color: light-dark(#6a737d, #8b949e); --lumis-dark-font-style:normal; --lumis-light-font-style:italic;">comment</span>
//! ```
//!
//! The browser selects the color based on `color-scheme` or `prefers-color-scheme`,
//! with no additional CSS required.
//!
//! That covers `color` and `background-color`, the only properties `light-dark()` is
//! defined over. `font-weight`, `font-style` and `text-decoration` follow a value the
//! two themes share as an ordinary declaration, which needs no stylesheet and no
//! switching. A value they disagree on is a `--lumis-light-*` and a `--lumis-dark-*`
//! variable instead, and nothing inline, since only a rule of your own can switch it:
//!
//! ```css
//! .lumis span {
//!   font-style: var(--lumis-light-font-style);
//!   font-weight: var(--lumis-light-font-weight);
//!   text-decoration: var(--lumis-light-text-decoration);
//! }
//!
//! @media (prefers-color-scheme: dark) {
//!   .lumis span {
//!     font-style: var(--lumis-dark-font-style);
//!     font-weight: var(--lumis-dark-font-weight);
//!     text-decoration: var(--lumis-dark-text-decoration);
//!   }
//! }
//! ```
//!
//! No `!important`, and none is wanted: a disputed property is left out of the style
//! attribute, so these rules have nothing inline to outrank, and they stay where your
//! own `print` or `forced-colors` rules can still beat them. On a token whose themes
//! agreed, the inline declaration outranks them and the shared value stands. On one
//! neither theme styled, the variable was never set, the declaration drops out, and
//! whatever your page says is what renders.
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

    fn lightdark_html(light: &str, dark: &str, source: &str, italic: bool) -> String {
        let mut themes = HashMap::new();
        themes.insert("light".to_string(), crate::themes::get(light).unwrap());
        themes.insert("dark".to_string(), crate::themes::get(dark).unwrap());

        let formatter = HtmlMultiThemesBuilder::new()
            .language(Language::Rust)
            .themes(themes)
            .default_theme("light-dark()")
            .italic(italic)
            .build()
            .unwrap();

        let mut output = Vec::new();
        crate::write_highlight(&mut output, source, formatter).unwrap();
        String::from_utf8(output).unwrap()
    }

    #[test]
    fn test_lightdark_mode_keeps_light_dark_to_the_colors() {
        // `\n` is a `string.escape`, which both GitHub themes render bold.
        let html = lightdark_html("github_light", "github_dark", r#"let s = "a\nb";"#, true);

        assert!(html.contains("color: light-dark("));
        assert!(html.contains("font-weight: bold;"));
        assert!(
            !html.contains("font-weight: light-dark("),
            "font-weight is not a color: {html}"
        );
        assert!(
            !html.contains("font-style: light-dark("),
            "font-style is not a color: {html}"
        );
        assert!(
            !html.contains("text-decoration: light-dark("),
            "text-decoration is not a color: {html}"
        );
    }

    #[test]
    fn test_lightdark_mode_omits_font_properties_neither_theme_sets() {
        let html = lightdark_html("github_light", "github_dark", "// comment", true);

        assert!(!html.contains("font-weight"), "{html}");
        assert!(!html.contains("font-style"), "{html}");
        assert!(!html.contains("text-decoration"), "{html}");
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
        // Both Catppuccin themes italicise comments.
        let source = "// comment";

        let html = lightdark_html("catppuccin_latte", "catppuccin_mocha", source, false);
        assert!(!html.contains("font-style"), "{html}");

        let html = lightdark_html("catppuccin_latte", "catppuccin_mocha", source, true);
        assert!(html.contains("font-style: italic;"), "{html}");
    }
}

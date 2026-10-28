//! Theme system for syntax highlighting.
//!
//! This module provides access to Neovim-based color themes for syntax highlighting.
//! Themes define colors and styling for different syntax elements like keywords,
//! strings, comments, etc. The themes are extracted from popular Neovim colorschemes
//! and converted to a format suitable for syntax highlighting.
//!
//! # Available Themes
//!
//! The theme system includes 120+ themes covering light and dark variants from
//! popular colorschemes like Dracula, Catppuccin, GitHub, Gruvbox, and many more.
//! See the main library documentation for the complete list.
//!
//! # Basic Usage
//!
//! ```rust
//! use lumis::themes::{self, Theme};
//! use std::str::FromStr;
//!
//! // Get a theme by name
//! let theme = themes::get("dracula").expect("Theme not found");
//! println!("Theme: {} ({})", theme.name, theme.appearance);
//!
//! // Parse from string
//! let theme: Theme = "catppuccin_mocha".parse().expect("Theme not found");
//! println!("Theme: {}", theme.name);
//!
//! // Using FromStr
//! let theme = Theme::from_str("github_light").expect("Theme not found");
//!
//! // List all available themes
//! let all_themes: Vec<_> = themes::available_themes().collect();
//! println!("Found {} themes", all_themes.len());
//! ```
//!
//! # Integration with Formatters
//!
//! Themes are primarily used with HTML inline and Terminal formatters
//! to provide syntax highlighting colors:
//!
//! ```rust
//! use lumis::{highlight, HtmlInlineBuilder, languages::Language, themes};
//!
//! let code = "fn main() { println!(\"Hello\"); }";
//! let theme = themes::get("catppuccin_mocha").unwrap();
//!
//! let formatter = HtmlInlineBuilder::new()
//!     .language(Language::Rust)
//!     .theme(Some(theme))
//!     .build()
//!     .unwrap();
//!
//! let highlighted = highlight(code, formatter);
//! ```
//!
//! # Theme Structure
//!
//! Each theme contains:
//! - **Metadata**: Name, appearance (light/dark), revision info
//! - **Color definitions**: Foreground/background colors, font styles
//! - **Scope mappings**: Which colors apply to which syntax elements
//!
//! # Custom Themes
//!
//! To generate a JSON theme from a colorscheme outside the bundled collection,
//! see [Generate a theme from a Neovim colorscheme](https://docs.lumis.sh/recipes/generate-theme-from-neovim-colorscheme),
//! including source-license and redistribution considerations.
//!
//! Create custom themes by loading from JSON files or building programmatically:
//!
//! ```rust,no_run
//! use lumis::themes;
//!
//! // Load from a JSON file
//! let theme = themes::from_file("my_theme.json").unwrap();
//!
//! // Or parse from a JSON string
//! let json = r#"{"name": "my_theme", "appearance": "dark", ...}"#;
//! let theme = themes::from_json(json).unwrap();
//! ```
//!
//! See [custom_theme.rs](https://github.com/leandrocp/lumis/blob/main/crates/lumis/examples/custom_theme.rs)
//! for a complete example of building themes programmatically.

pub use lumis_core::themes::*;

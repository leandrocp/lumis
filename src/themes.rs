//! Theme system for syntax highlighting.
//!
//! This module provides access to Neovim-based color themes for syntax highlighting.
//! Themes define colors and styling for different syntax elements like keywords,
//! strings, comments, etc. The themes are extracted from popular Neovim colorschemes
//! and converted to a format suitable for syntax highlighting.
//!
//! # Available Themes
//!
//! The theme system includes 100+ themes covering light and dark variants from
//! popular colorschemes like Dracula, Catppuccin, GitHub, Gruvbox, and many more.
//! See the main library documentation for the complete list.
//!
//! # Basic Usage
//!
//! ```rust
//! use autumnus::themes;
//!
//! // Get a theme by name
//! let theme = themes::get("dracula").expect("Theme not found");
//! println!("Theme: {} ({})", theme.name, theme.appearance);
//!
//! // List all available themes
//! let all_themes = themes::available_themes();
//! println!("Found {} themes", all_themes.len());
//! ```
//!
//! # Integration with Formatters
//!
//! Themes are primarily used with [`crate::FormatterOption::HtmlInline`] and
//! [`crate::FormatterOption::Terminal`] to provide syntax highlighting colors:
//!
//! ```rust
//! use autumnus::{highlight, Options, FormatterOption, themes};
//!
//! let code = "fn main() { println!(\"Hello\"); }";
//! let theme = themes::get("catppuccin_mocha").unwrap();
//!
//! let options = Options {
//!     lang_or_file: Some("rust"),
//!     formatter: FormatterOption::HtmlInline {
//!         theme: Some(theme),
//!         pre_class: None,
//!         italic: false,
//!         include_highlights: false,
//!         highlight_lines: None,
//!     },
//! };
//!
//! let highlighted = highlight(code, options);
//! ```
//!
//! # Theme Structure
//!
//! Each theme contains:
//! - **Metadata**: Name, appearance (light/dark), revision info
//! - **Color definitions**: Foreground/background colors, font styles
//! - **Scope mappings**: Which colors apply to which syntax elements
//!
//! # External Theme Files
//!
//! You can also load themes from JSON files using [`from_file`] or [`from_json`]
//! for custom colorschemes or theme development.

#![allow(unused_must_use)]

use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, path::Path};

/// Error type for theme operations
#[derive(Debug)]
pub enum ThemeError {
    /// Theme not found
    NotFound(String),
    /// Invalid theme JSON
    InvalidJson(String),
    /// Theme file not found
    FileNotFound(String),
    /// Theme file read error
    FileReadError(String),
}

impl std::fmt::Display for ThemeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeError::NotFound(name) => write!(f, "Theme '{}' not found", name),
            ThemeError::InvalidJson(msg) => write!(f, "Invalid theme JSON: {}", msg),
            ThemeError::FileNotFound(path) => write!(f, "Theme file not found: {}", path),
            ThemeError::FileReadError(msg) => write!(f, "Failed to read theme file: {}", msg),
        }
    }
}

impl std::error::Error for ThemeError {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
/// A theme for syntax highlighting.
///
/// A theme consists of a name, appearance (light/dark), revision (commit) and a collection of highlight styles
/// mapped to their scope names.
///
/// # Examples
///
/// Loading a theme by name:
///
/// ```
/// use autumnus::themes;
///
/// let theme = themes::get("github_light").expect("Theme not found");
/// assert_eq!(theme.appearance, "light");
/// ```
///
/// Loading a theme from a JSON file:
///
/// ```
/// use autumnus::themes;
/// use std::path::Path;
///
/// let theme = themes::from_file(Path::new("themes/dracula.json")).unwrap();
/// ```
///
/// Creating a theme programmatically:
///
/// ```
/// use autumnus::themes::{Theme, Style};
/// use std::collections::BTreeMap;
///
/// let mut highlights = BTreeMap::new();
/// highlights.insert("keyword".to_string(), Style {
///     fg: Some("#ff79c6".to_string()),
///     bold: true,
///     ..Default::default()
/// });
///
/// let theme = Theme::new(
///     "my_theme".to_string(),
///     "dark".to_string(),
///     "3e976b4".to_string(),
///     highlights
/// );
/// ```
pub struct Theme {
    /// The name of the theme.
    pub name: String,
    /// The appearance of the theme ("light" or "dark").
    pub appearance: String,
    /// The commit of the theme plugin
    pub revision: String,
    /// A map of highlight scope names to their styles.
    pub highlights: BTreeMap<String, Style>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
/// A style for syntax highlighting.
///
/// A style defines the visual appearance of a highlight scope, including colors,
/// font weight, and text decoration.
///
/// # Examples
///
/// Creating a style with foreground color and bold text:
///
/// ```
/// use autumnus::themes::Style;
///
/// let style = Style {
///     fg: Some("#ff79c6".to_string()),
///     bold: true,
///     ..Default::default()
/// };
/// ```
///
/// Creating a style with background color and italic text:
///
/// ```
/// use autumnus::themes::Style;
///
/// let style = Style {
///     bg: Some("#282a36".to_string()),
///     italic: true,
///     ..Default::default()
/// };
/// ```
///
/// Creating a style with text decoration:
///
/// ```
/// use autumnus::themes::Style;
///
/// let style = Style {
///     underline: true,
///     strikethrough: true,
///     ..Default::default()
/// };
/// ```
pub struct Style {
    /// The foreground color in hex format (e.g., "#ff79c6").
    #[serde(default)]
    pub fg: Option<String>,
    /// The background color in hex format (e.g., "#282a36").
    #[serde(default)]
    pub bg: Option<String>,
    /// Whether to underline the text.
    #[serde(default)]
    pub underline: bool,
    /// Whether to make the text bold.
    #[serde(default)]
    pub bold: bool,
    /// Whether to make the text italic.
    #[serde(default)]
    pub italic: bool,
    /// Whether to strikethrough the text.
    #[serde(default)]
    pub strikethrough: bool,
}

include!(concat!(env!("OUT_DIR"), "/theme_data.rs"));

/// Load a theme from a JSON file.
///
/// This function reads a theme definition from a JSON file and parses it into a [`Theme`] struct.
/// The JSON file should contain theme metadata (name, appearance, revision) and highlight style
/// definitions for various syntax scopes.
///
/// # Arguments
///
/// * `path` - Path to the JSON theme file
///
/// # Returns
///
/// * `Ok(Theme)` - Successfully loaded and parsed theme
/// * `Err(ThemeError)` - File not found, read error, or invalid JSON format
///
/// # JSON Format
///
/// Theme files should follow this structure:
///
/// ```json
/// {
///   "name": "my_custom_theme",
///   "appearance": "dark",
///   "revision": "v1.0.0",
///   "highlights": {
///     "keyword": { "fg": "#ff79c6", "bold": true },
///     "string": { "fg": "#f1fa8c" },
///     "comment": { "fg": "#6272a4", "italic": true }
///   }
/// }
/// ```
///
/// # Examples
///
/// ## Loading a theme file
///
/// ```rust,no_run
/// use autumnus::themes;
/// use std::path::Path;
///
/// // Load theme from file
/// let theme = themes::from_file("themes/my_theme.json")
///     .expect("Failed to load theme");
///
/// println!("Loaded theme: {} ({})", theme.name, theme.appearance);
/// ```
///
/// ## Error handling
///
/// ```rust,no_run
/// use autumnus::themes::{self, ThemeError};
///
/// match themes::from_file("nonexistent.json") {
///     Ok(theme) => println!("Theme loaded: {}", theme.name),
///     Err(ThemeError::FileNotFound(path)) => {
///         eprintln!("Theme file not found: {}", path);
///     },
///     Err(ThemeError::InvalidJson(msg)) => {
///         eprintln!("Invalid theme JSON: {}", msg);
///     },
///     Err(err) => eprintln!("Other error: {}", err),
/// }
/// ```
pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Theme, ThemeError> {
    let path = path.as_ref();
    let json = fs::read_to_string(path).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            ThemeError::FileNotFound(path.display().to_string())
        } else {
            ThemeError::FileReadError(e.to_string())
        }
    })?;

    from_json(&json).map_err(|e| ThemeError::InvalidJson(e.to_string()))
}

/// Parse a theme from a JSON string.
///
/// This function parses a JSON string containing theme definition data and creates
/// a [`Theme`] struct. The JSON must contain required fields (name, appearance, revision)
/// and highlight style definitions.
///
/// # Arguments
///
/// * `json` - JSON string containing theme definition
///
/// # Returns
///
/// * `Ok(Theme)` - Successfully parsed theme
/// * `Err(Box<dyn Error>)` - JSON parse error or validation failure
///
/// # Validation
///
/// The function validates that required fields are present and non-empty:
/// - `name` - Theme identifier
/// - `appearance` - Either "light" or "dark"
/// - `revision` - Version or commit hash
/// - `highlights` - Map of syntax scopes to styles
///
/// # Examples
///
/// ## Basic theme parsing
///
/// ```rust
/// use autumnus::themes;
///
/// let json = r##"{
///     "name": "my_theme",
///     "appearance": "dark",
///     "revision": "v1.0.0",
///     "highlights": {
///         "keyword": { "fg": "#ff79c6", "bold": true },
///         "string": { "fg": "#f1fa8c" },
///         "comment": { "fg": "#6272a4", "italic": true }
///     }
/// }"##;
///
/// let theme = themes::from_json(json).expect("Failed to parse theme");
/// assert_eq!(theme.name, "my_theme");
/// assert_eq!(theme.appearance, "dark");
/// ```
///
/// ## Error handling
///
/// ```rust
/// use autumnus::themes;
///
/// // Invalid JSON
/// let invalid_json = r#"{ invalid json }"#;
/// assert!(themes::from_json(invalid_json).is_err());
///
/// // Missing required fields
/// let incomplete_json = r#"{ "name": "test" }"#;
/// assert!(themes::from_json(incomplete_json).is_err());
/// ```
///
/// ## Runtime theme creation
///
/// ```rust
/// use autumnus::themes;
/// use serde_json::json;
///
/// // Create theme JSON programmatically
/// let theme_data = json!({
///     "name": "runtime_theme",
///     "appearance": "light",
///     "revision": "generated",
///     "highlights": {
///         "keyword": { "fg": "#0000ff", "bold": true },
///         "string": { "fg": "#008000" }
///     }
/// });
///
/// let theme = themes::from_json(&theme_data.to_string())
///     .expect("Failed to create theme");
/// ```
pub fn from_json(json: &str) -> Result<Theme, Box<dyn std::error::Error>> {
    let theme: Theme = serde_json::from_str(json)?;

    // Validate required fields
    if theme.name.is_empty() {
        return Err("Theme name cannot be empty".into());
    }
    if theme.appearance.is_empty() {
        return Err("Theme appearance cannot be empty".into());
    }
    if theme.revision.is_empty() {
        return Err("Theme revision cannot be empty".into());
    }

    Ok(theme)
}

impl Theme {
    pub fn new(
        name: String,
        appearance: String,
        revision: String,
        highlights: BTreeMap<String, Style>,
    ) -> Self {
        Theme {
            name,
            appearance,
            revision,
            highlights,
        }
    }

    pub fn css(&self, enable_italic: bool) -> String {
        let mut rules = Vec::new();

        rules.push(format!(
            "/* {}\n * revision: {}\n */\n\npre.athl",
            self.name, self.revision
        ));

        if let Some(pre_style) = &self.pre_style("\n  ") {
            rules.push(format!(" {{\n  {}\n}}\n", pre_style));
        } else {
            rules.push(" {}\n".to_string());
        }

        for (scope, style) in &self.highlights {
            let style_css = style.css(enable_italic, "\n  ");

            if !style_css.is_empty() {
                rules.push(format!(
                    ".{} {{\n  {}\n}}\n",
                    scope.replace('.', "-"),
                    style_css
                ))
            };
        }

        rules.join("")
    }

    pub fn get_style(&self, scope: &str) -> Option<&Style> {
        match self.highlights.get(scope) {
            Some(syntax) => Some(syntax),
            None => {
                if scope.contains('.') {
                    let split: Vec<&str> = scope.split('.').collect();
                    let joined = split[0..split.len() - 1].join(".");
                    self.get_style(joined.as_str())
                } else {
                    None
                }
            }
        }
    }

    pub fn fg(&self) -> Option<String> {
        if let Some(style) = self.get_style("normal") {
            style.fg.clone()
        } else {
            None
        }
    }

    pub fn bg(&self) -> Option<String> {
        if let Some(style) = self.get_style("normal") {
            style.bg.clone()
        } else {
            None
        }
    }

    pub fn pre_style(&self, separator: &str) -> Option<String> {
        let mut rules = Vec::new();

        if let Some(fg) = &self.fg() {
            rules.push(format!("color: {};", fg));
        }

        if let Some(bg) = &self.bg() {
            rules.push(format!("background-color: {};", bg));
        }

        if rules.is_empty() {
            None
        } else {
            Some(rules.join(separator))
        }
    }
}

impl Style {
    pub fn css(&self, enable_italic: bool, separator: &str) -> String {
        let mut rules = Vec::new();

        if let Some(fg) = &self.fg {
            rules.push(format!("color: {};", fg))
        };

        if let Some(bg) = &self.bg {
            rules.push(format!("background-color: {};", bg))
        };

        if self.bold {
            rules.push("font-weight: bold;".to_string())
        }

        if enable_italic && self.italic {
            rules.push("font-style: italic;".to_string())
        };

        match (self.underline, self.strikethrough) {
            (true, true) => rules.push("text-decoration: underline line-through;".to_string()),
            (true, false) => rules.push("text-decoration: underline;".to_string()),
            (false, true) => rules.push("text-decoration: line-through;".to_string()),
            (false, false) => (),
        };

        rules.join(separator)
    }
}

/// Get a list of all built-in themes.
///
/// This function returns a vector containing references to all themes bundled
/// with the library. These themes are compiled into the binary and are always
/// available without external files.
///
/// # Returns
///
/// A vector of theme references sorted alphabetically by name.
///
/// # Examples
///
/// ## List all themes
///
/// ```rust
/// use autumnus::themes;
///
/// let all_themes = themes::available_themes();
/// println!("Available themes: {}", all_themes.len());
///
/// for theme in &all_themes {
///     println!("- {} ({})", theme.name, theme.appearance);
/// }
/// ```
///
/// ## Filter themes by appearance
///
/// ```rust
/// use autumnus::themes;
///
/// let all_themes = themes::available_themes();
///
/// let dark_themes: Vec<_> = all_themes
///     .iter()
///     .filter(|theme| theme.appearance == "dark")
///     .collect();
///
/// let light_themes: Vec<_> = all_themes
///     .iter()
///     .filter(|theme| theme.appearance == "light")
///     .collect();
///
/// println!("Dark themes: {}, Light themes: {}",
///          dark_themes.len(), light_themes.len());
/// ```
///
/// ## Find themes by name pattern
///
/// ```rust
/// use autumnus::themes;
///
/// let all_themes = themes::available_themes();
///
/// // Find all Catppuccin variants
/// let catppuccin_themes: Vec<_> = all_themes
///     .iter()
///     .filter(|theme| theme.name.starts_with("catppuccin"))
///     .collect();
///
/// // Find GitHub themes
/// let github_themes: Vec<_> = all_themes
///     .iter()
///     .filter(|theme| theme.name.contains("github"))
///     .collect();
/// ```
///
/// ## Build a theme selector
///
/// ```rust
/// use autumnus::themes;
///
/// let themes = themes::available_themes();
/// let mut theme_names: Vec<&str> = themes.iter().map(|t| t.name.as_str()).collect();
/// theme_names.sort();
///
/// println!("Theme selector options:");
/// for (i, name) in theme_names.iter().enumerate() {
///     println!("{}. {}", i + 1, name);
/// }
/// ```
pub fn available_themes() -> Vec<&'static Theme> {
    ALL_THEMES.iter().copied().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_available_themes() {
        let themes = available_themes();

        assert!(!themes.is_empty());

        let dracula = themes.iter().find(|t| t.name == "dracula").unwrap();
        assert_eq!(dracula.name, "dracula");
        assert_eq!(dracula.appearance, "dark");

        let github_light = themes.iter().find(|t| t.name == "github_light").unwrap();
        assert_eq!(github_light.name, "github_light");
        assert_eq!(github_light.appearance, "light");

        for theme in themes {
            assert!(!theme.name.is_empty());
            assert!(!theme.appearance.is_empty());
            assert!(theme.appearance == "light" || theme.appearance == "dark");
        }
    }

    #[test]
    fn test_load_all_themes() {
        for theme in ALL_THEMES.iter() {
            assert!(!theme.name.is_empty());
        }

        assert_eq!(ALL_THEMES.len(), 109);
    }

    #[test]
    fn test_get_by_name() {
        let theme = get("github_light").expect("Theme not found");
        assert_eq!(theme.name, "github_light");

        let err = get("non_existent_theme");
        assert!(err.is_err());
    }

    #[test]
    fn test_from_json() {
        let json = r#"{"name": "test", "appearance": "dark", "revision": "3e976b4", "highlights": {"keyword": {"fg": "blue"}}}"#;
        let theme = from_json(json).unwrap();

        assert_eq!(theme.name, "test");

        assert_eq!(
            theme.get_style("keyword"),
            Some(&Style {
                fg: Some("blue".to_string()),
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_from_file() {
        let path = Path::new("themes/catppuccin_frappe.json");
        let theme = from_file(path).unwrap();

        assert_eq!(theme.name, "catppuccin_frappe");

        assert_eq!(
            theme.get_style("tag.attribute"),
            Some(&Style {
                fg: Some("#81c8be".to_string()),
                italic: true,
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_style_css() {
        let style = Style {
            fg: Some("blue".to_string()),
            underline: true,
            italic: true,
            ..Default::default()
        };

        assert_eq!(
            style.css(true, " "),
            "color: blue; font-style: italic; text-decoration: underline;"
        );
    }

    #[test]
    fn test_theme_css() {
        let json = r#"{"name": "test", "appearance": "dark", "revision": "3e976b4", "highlights": {"normal": {"fg": "red", "bg": "green"}, "keyword": {"fg": "blue", "italic": true}, "tag.attribute": {"bg": "gray", "bold": true}}}"#;
        let theme = from_json(json).unwrap();

        let expected = r#"/* test
 * revision: 3e976b4
 */

pre.athl {
  color: red;
  background-color: green;
}
.keyword {
  color: blue;
  font-style: italic;
}
.normal {
  color: red;
  background-color: green;
}
.tag-attribute {
  background-color: gray;
  font-weight: bold;
}
"#;

        assert_eq!(theme.css(true), expected);
    }
}

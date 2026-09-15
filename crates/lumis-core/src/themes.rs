//! Theme system for syntax highlighting.
//!
//! This module provides access to Neovim-based color themes for syntax highlighting.
//! Themes define colors and styling for different syntax elements like keywords,
//! strings, comments, etc.

use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, path::Path, str::FromStr};

use derive_builder::Builder;

/// Error type for theme operations.
#[derive(Debug, Clone)]
#[non_exhaustive]
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
            ThemeError::NotFound(name) => write!(f, "theme '{name}' not found"),
            ThemeError::InvalidJson(msg) => write!(f, "invalid theme json: {msg}"),
            ThemeError::FileNotFound(path) => write!(f, "theme file not found: {path}"),
            ThemeError::FileReadError(msg) => write!(f, "failed to read theme file: {msg}"),
        }
    }
}

impl std::error::Error for ThemeError {}

/// Error type returned when parsing a theme from a string fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThemeParseError(String);

impl std::fmt::Display for ThemeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown theme: {}", self.0)
    }
}

impl std::error::Error for ThemeParseError {}

impl From<std::io::Error> for ThemeError {
    fn from(err: std::io::Error) -> Self {
        if err.kind() == std::io::ErrorKind::NotFound {
            ThemeError::FileNotFound(err.to_string())
        } else {
            ThemeError::FileReadError(err.to_string())
        }
    }
}

impl From<serde_json::Error> for ThemeError {
    fn from(err: serde_json::Error) -> Self {
        ThemeError::InvalidJson(err.to_string())
    }
}

/// Underline style for text decoration.
///
/// Corresponds to Neovim's underline variants:
/// - `Solid` - standard underline (`underline` in Neovim)
/// - `Wavy` - wavy/curly underline (`undercurl` in Neovim)
/// - `Double` - double underline (`underdouble` in Neovim)
/// - `Dotted` - dotted underline (`underdotted` in Neovim)
/// - `Dashed` - dashed underline (`underdashed` in Neovim)
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UnderlineStyle {
    #[default]
    None,
    Solid,
    Wavy,
    Double,
    Dotted,
    Dashed,
}

/// Text decoration combining underline style and strikethrough.
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct TextDecoration {
    /// The underline style to apply.
    #[serde(default)]
    pub underline: UnderlineStyle,
    /// Whether to apply strikethrough.
    #[serde(default)]
    pub strikethrough: bool,
}

/// The visual appearance of a theme.
///
/// Themes are categorized as either light (dark text on light background) or
/// dark (light text on dark background).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Appearance {
    /// Light theme (dark text on light background)
    Light,
    /// Dark theme (light text on dark background)
    #[default]
    Dark,
}

impl std::fmt::Display for Appearance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Appearance::Light => write!(f, "light"),
            Appearance::Dark => write!(f, "dark"),
        }
    }
}

/// A theme for syntax highlighting.
///
/// A theme consists of a name, appearance (light/dark), revision (commit) and a collection
/// of highlight styles mapped to their scope names.
///
/// # Examples
///
/// Loading a theme by name:
///
/// ```
/// use lumis_core::themes::{self, Theme, Appearance};
///
/// let theme = themes::get("github_light").expect("Theme not found");
/// assert_eq!(theme.appearance, Appearance::Light);
///
/// let theme: Theme = "dracula".parse().expect("Theme not found");
/// assert_eq!(theme.name, "dracula");
/// ```
///
/// Creating a theme programmatically:
///
/// ```
/// use lumis_core::themes::{Theme, Style, Appearance};
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
///     Appearance::Dark,
///     "3e976b4".to_string(),
///     highlights
/// );
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Theme {
    /// The name of the theme.
    pub name: String,
    /// The appearance of the theme (light or dark).
    pub appearance: Appearance,
    /// The commit of the theme plugin
    pub revision: String,
    /// A map of highlight scope names to their styles.
    pub highlights: BTreeMap<String, Style>,
}

/// A style for syntax highlighting.
///
/// Defines the visual appearance of a highlight scope, including colors,
/// font weight, and text decoration.
///
/// # Examples
///
/// ```
/// use lumis_core::themes::{Style, TextDecoration, UnderlineStyle};
///
/// let style = Style {
///     fg: Some("#ff79c6".to_string()),
///     bold: true,
///     ..Default::default()
/// };
///
/// let style = Style {
///     text_decoration: TextDecoration {
///         underline: UnderlineStyle::Wavy,
///         strikethrough: true,
///     },
///     ..Default::default()
/// };
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Style {
    /// The foreground color in hex format (e.g., "#ff79c6").
    pub fg: Option<String>,
    /// The background color in hex format (e.g., "#282a36").
    pub bg: Option<String>,
    /// Whether to make the text bold.
    pub bold: bool,
    /// Whether to make the text italic.
    pub italic: bool,
    /// Text decoration (underline style and strikethrough).
    pub text_decoration: TextDecoration,
}

/// Deserializes underline from either a boolean (`true` = solid) or a string (`"solid"`, `"wavy"`, etc.).
fn deserialize_underline_field<'de, D>(deserializer: D) -> Result<UnderlineStyle, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de;

    struct UnderlineVisitor;

    impl de::Visitor<'_> for UnderlineVisitor {
        type Value = UnderlineStyle;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("a boolean or underline style string")
        }

        fn visit_bool<E: de::Error>(self, v: bool) -> Result<UnderlineStyle, E> {
            Ok(if v {
                UnderlineStyle::Solid
            } else {
                UnderlineStyle::None
            })
        }

        fn visit_str<E: de::Error>(self, v: &str) -> Result<UnderlineStyle, E> {
            match v {
                "solid" => Ok(UnderlineStyle::Solid),
                "wavy" | "undercurl" => Ok(UnderlineStyle::Wavy),
                "double" => Ok(UnderlineStyle::Double),
                "dotted" => Ok(UnderlineStyle::Dotted),
                "dashed" => Ok(UnderlineStyle::Dashed),
                "none" => Ok(UnderlineStyle::None),
                _ => Err(de::Error::unknown_variant(
                    v,
                    &["solid", "wavy", "double", "dotted", "dashed", "none"],
                )),
            }
        }
    }

    deserializer.deserialize_any(UnderlineVisitor)
}

/// Helper struct for deserializing Style from JSON.
///
/// Accepts two formats:
/// - Old format: separate bool fields (`underline: true`, `undercurl: true`, etc.)
/// - New format: single `underline` field with string value (`"solid"`, `"wavy"`, etc.)
#[derive(Deserialize)]
struct StyleHelper {
    #[serde(default)]
    fg: Option<String>,
    #[serde(default)]
    bg: Option<String>,
    #[serde(default)]
    bold: bool,
    #[serde(default)]
    italic: bool,
    #[serde(default, deserialize_with = "deserialize_underline_field")]
    underline: UnderlineStyle,
    #[serde(default)]
    undercurl: bool,
    #[serde(default)]
    underdouble: bool,
    #[serde(default)]
    underdotted: bool,
    #[serde(default)]
    underdashed: bool,
    #[serde(default)]
    strikethrough: bool,
}

impl<'de> Deserialize<'de> for Style {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let helper = StyleHelper::deserialize(deserializer)?;

        // Old-format bool fields take precedence if set, otherwise use the underline field
        let underline = if helper.undercurl {
            UnderlineStyle::Wavy
        } else if helper.underdouble {
            UnderlineStyle::Double
        } else if helper.underdotted {
            UnderlineStyle::Dotted
        } else if helper.underdashed {
            UnderlineStyle::Dashed
        } else {
            helper.underline
        };

        Ok(Style {
            fg: helper.fg,
            bg: helper.bg,
            bold: helper.bold,
            italic: helper.italic,
            text_decoration: TextDecoration {
                underline,
                strikethrough: helper.strikethrough,
            },
        })
    }
}

fn serialized_style_field_count(style: &Style) -> usize {
    usize::from(style.fg.is_some())
        + usize::from(style.bg.is_some())
        + usize::from(style.bold)
        + usize::from(style.italic)
        + usize::from(style.text_decoration.underline != UnderlineStyle::None)
        + usize::from(style.text_decoration.strikethrough)
}

impl Serialize for Style {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Style", serialized_style_field_count(self))?;

        if let Some(fg) = &self.fg {
            state.serialize_field("fg", fg)?;
        }
        if let Some(bg) = &self.bg {
            state.serialize_field("bg", bg)?;
        }
        if self.bold {
            state.serialize_field("bold", &true)?;
        }
        if self.italic {
            state.serialize_field("italic", &true)?;
        }
        match self.text_decoration.underline {
            UnderlineStyle::None => {}
            UnderlineStyle::Solid => state.serialize_field("underline", &true)?,
            UnderlineStyle::Wavy => state.serialize_field("undercurl", &true)?,
            UnderlineStyle::Double => state.serialize_field("underdouble", &true)?,
            UnderlineStyle::Dotted => state.serialize_field("underdotted", &true)?,
            UnderlineStyle::Dashed => state.serialize_field("underdashed", &true)?,
        }
        if self.text_decoration.strikethrough {
            state.serialize_field("strikethrough", &true)?;
        }

        state.end()
    }
}

/// Load a theme from a JSON file.
///
/// Reads a theme definition from a JSON file and parses it into a [`Theme`].
/// The JSON file should contain theme metadata (name, appearance, revision) and
/// highlight style definitions for various syntax scopes.
///
/// # Errors
///
/// Returns [`ThemeError::FileNotFound`] if the file doesn't exist,
/// [`ThemeError::FileReadError`] if the file can't be read,
/// or [`ThemeError::InvalidJson`] if the JSON is malformed.
///
/// # Examples
///
/// ```rust,no_run
/// use lumis_core::themes;
///
/// let theme = themes::from_file("themes/my_theme.json")
///     .expect("Failed to load theme");
/// println!("Loaded theme: {} ({})", theme.name, theme.appearance);
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

    from_json(&json)
}

/// Parse a theme from a JSON string.
///
/// Parses a JSON string containing a theme definition and creates a [`Theme`].
/// The JSON must contain required fields (name, appearance, revision) and
/// highlight style definitions.
///
/// # Errors
///
/// Returns [`ThemeError::InvalidJson`] if the JSON is malformed or if
/// required fields (name, revision) are empty.
///
/// # Examples
///
/// ```rust
/// use lumis_core::themes;
///
/// let json = r##"{
///     "name": "my_theme",
///     "appearance": "dark",
///     "revision": "v1.0.0",
///     "highlights": {
///         "keyword": { "fg": "#ff79c6", "bold": true },
///         "string": { "fg": "#f1fa8c" }
///     }
/// }"##;
///
/// let theme = themes::from_json(json).expect("Failed to parse theme");
/// assert_eq!(theme.name, "my_theme");
/// ```
pub fn from_json(json: &str) -> Result<Theme, ThemeError> {
    let theme: Theme = serde_json::from_str(json)?;

    // Validate required fields
    if theme.name.is_empty() {
        return Err(ThemeError::InvalidJson("theme name cannot be empty".into()));
    }
    if theme.revision.is_empty() {
        return Err(ThemeError::InvalidJson(
            "theme revision cannot be empty".into(),
        ));
    }

    Ok(theme)
}

impl FromStr for Theme {
    type Err = ThemeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        get(s).map_err(|_| ThemeParseError(s.to_string()))
    }
}

static RAINBOW_BRACKET_FALLBACKS: std::sync::LazyLock<BTreeMap<&'static str, Style>> =
    std::sync::LazyLock::new(|| {
        BTreeMap::from([
            (
                "punctuation.bracket.rainbow.1",
                Style {
                    fg: Some("#e06c75".to_string()),
                    ..Default::default()
                },
            ),
            (
                "punctuation.bracket.rainbow.2",
                Style {
                    fg: Some("#e5c07b".to_string()),
                    ..Default::default()
                },
            ),
            (
                "punctuation.bracket.rainbow.3",
                Style {
                    fg: Some("#61afef".to_string()),
                    ..Default::default()
                },
            ),
            (
                "punctuation.bracket.rainbow.4",
                Style {
                    fg: Some("#d19a66".to_string()),
                    ..Default::default()
                },
            ),
            (
                "punctuation.bracket.rainbow.5",
                Style {
                    fg: Some("#98c379".to_string()),
                    ..Default::default()
                },
            ),
            (
                "punctuation.bracket.rainbow.6",
                Style {
                    fg: Some("#c678dd".to_string()),
                    ..Default::default()
                },
            ),
        ])
    });

impl Theme {
    pub fn new(
        name: String,
        appearance: Appearance,
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

    #[deprecated(note = "use themes::CssBuilder instead")]
    pub fn css(&self, enable_italic: bool) -> String {
        CssBuilder::new(self).enable_italic(enable_italic).build()
    }

    /// Get style for a scope.
    ///
    /// Implements Neovim's treesitter-highlight-groups scope hierarchy walk-up:
    /// tries exact match first, then walks up parent scopes
    /// (e.g., "markup.heading.2.markdown" -> "markup.heading.2" -> "markup.heading").
    ///
    /// For language-specific styles, callers should construct the full scope themselves
    /// (e.g., "comment.lua" instead of "comment"). If the specialized scope doesn't exist,
    /// it automatically falls back to parent scopes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lumis_core::themes;
    ///
    /// let theme = themes::get("catppuccin_mocha").unwrap();
    ///
    /// // Request specialized scope - falls back to parent if not found
    /// let style = theme.get_style("comment.lua");
    ///
    /// // Request generic scope
    /// let style = theme.get_style("keyword");
    /// ```
    pub fn get_style(&self, scope: &str) -> Option<&Style> {
        let mut current = scope;
        loop {
            if let Some(style) = self.highlights.get(current) {
                return Some(style);
            }
            if let Some(style) = RAINBOW_BRACKET_FALLBACKS.get(current) {
                return Some(style);
            }
            match current.rsplit_once('.') {
                Some((parent, _)) => current = parent,
                None => return None,
            }
        }
    }

    pub fn fg(&self) -> Option<&str> {
        self.get_style("normal").and_then(|s| s.fg.as_deref())
    }

    pub fn bg(&self) -> Option<&str> {
        self.get_style("normal").and_then(|s| s.bg.as_deref())
    }

    pub fn pre_style(&self, separator: &str) -> Option<String> {
        let mut rules = Vec::new();

        if let Some(fg) = self.fg() {
            rules.push(format!("color: {fg};"));
        }

        if let Some(bg) = self.bg() {
            rules.push(format!("background-color: {bg};"));
        }

        if rules.is_empty() {
            None
        } else {
            Some(rules.join(separator))
        }
    }
}

/// Builder for a theme CSS stylesheet.
///
/// Most applications should use the bundled CSS files, but the builder is useful when CSS needs to
/// be embedded, scoped, or customized.
///
/// # Examples
///
/// ```rust
/// use lumis_core::themes::{self, CssBuilder};
///
/// let theme = themes::get("github_dark").unwrap();
///
/// let css = CssBuilder::new(&theme)
///     .container_selector(".lumis")
///     .container_style([("background-color", "var(--code-background)"), ("border-radius", "0.375rem")])
///     .build();
///
/// assert!(css.contains(".l-keyword"));
/// ```
#[derive(Builder, Clone, Debug)]
#[builder(
    vis = "pub",
    custom_constructor,
    create_empty = "empty",
    build_fn(private, name = "fallible_build")
)]
struct Css<'a> {
    #[builder(setter(custom))]
    theme: &'a Theme,
    /// Whether italic theme styles should be emitted. Defaults to `true`.
    #[builder(default = "true")]
    enable_italic: bool,
    /// Parent selector prepended to every generated selector. Defaults to `""`.
    #[builder(default, setter(into))]
    scope: String,
    /// Selector used for the container code block rule. Defaults to `.lumis`.
    #[builder(default = "\".lumis\".to_string()", setter(into))]
    container_selector: String,
    /// Extra declarations for the container code block rule, set via [`CssBuilder::container_style`].
    #[builder(default, setter(custom))]
    container_style: Vec<(String, String)>,
}

impl<'a> CssBuilder<'a> {
    /// Create a CSS builder for `theme`.
    pub fn new(theme: &'a Theme) -> Self {
        CssBuilder {
            theme: Some(theme),
            ..CssBuilder::empty()
        }
    }

    /// Set extra declarations appended to the container code block rule.
    ///
    /// Each item is a `(property, value)` pair, for example `("padding", "1rem")`.
    pub fn container_style<K, V>(
        &mut self,
        declarations: impl IntoIterator<Item = (K, V)>,
    ) -> &mut Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.container_style = Some(
            declarations
                .into_iter()
                .map(|(property, value)| (property.into(), value.into()))
                .collect(),
        );
        self
    }

    /// Build the CSS stylesheet.
    #[must_use]
    pub fn build(&self) -> String {
        // `fallible_build` only errors when a required field is unset. `new` always sets `theme`,
        // the sole required field, so this never fails unless a future required field is added
        // without updating `new` (guarded by `test_css_builder_new_sets_required_fields`).
        self.fallible_build()
            .expect("CssBuilder::new initializes the required theme field")
            .render()
    }
}

impl Css<'_> {
    fn render(&self) -> String {
        let mut rules = Vec::new();

        rules.push(format!(
            "/* {}\n * revision: {}\n */\n{}",
            self.theme.name,
            self.theme.revision,
            self.scoped_selector(&self.container_selector),
        ));

        let container_style = self.container_style("\n  ");

        if container_style.is_empty() {
            rules.push(" {}\n".to_string());
        } else {
            rules.push(format!(" {{\n  {container_style}\n}}\n"));
        }

        for (scope, style) in &self.theme.highlights {
            // `normal` defines the code block's inherited colors, already emitted in the container rule
            // above and inherited by all text. It is never applied as a token class in highlighted
            // output, so emitting a `.normal` rule here would be dead CSS.
            if scope == "normal" {
                continue;
            }

            let style_css = style.css(self.enable_italic, "\n  ");

            if !style_css.is_empty() {
                rules.push(format!(
                    "{}.l-{} {{\n  {}\n}}\n",
                    self.scope_prefix(),
                    scope.replace('.', "-"),
                    style_css
                ));
            }
        }

        rules.join("")
    }

    fn scope_prefix(&self) -> String {
        if self.scope.is_empty() {
            String::new()
        } else {
            format!("{} ", self.scope)
        }
    }

    fn scoped_selector(&self, selector: &str) -> String {
        if self.scope.is_empty() {
            selector.to_string()
        } else {
            format!("{} {}", self.scope, selector)
        }
    }

    fn container_style(&self, separator: &str) -> String {
        // Start from the theme's container declarations, then merge `container_style` over them: a rule whose
        // property already exists replaces that value in place, otherwise it is appended. This keeps
        // a single declaration per property (e.g. overriding `background-color` does not duplicate
        // the theme's).
        let mut decls: Vec<(String, String)> = Vec::new();

        if let Some(fg) = self.theme.fg() {
            decls.push(("color".to_string(), fg.to_string()));
        }
        if let Some(bg) = self.theme.bg() {
            decls.push(("background-color".to_string(), bg.to_string()));
        }

        for (property, value) in &self.container_style {
            if let Some(existing) = decls.iter_mut().find(|(p, _)| p == property) {
                existing.1.clone_from(value);
            } else {
                decls.push((property.clone(), value.clone()));
            }
        }

        decls
            .iter()
            .map(|(property, value)| format!("{property}: {value};"))
            .collect::<Vec<_>>()
            .join(separator)
    }
}

impl Style {
    pub fn css(&self, enable_italic: bool, separator: &str) -> String {
        let mut rules = Vec::new();

        if let Some(fg) = &self.fg {
            rules.push(format!("color: {fg};"));
        }

        if let Some(bg) = &self.bg {
            rules.push(format!("background-color: {bg};"));
        }

        if self.bold {
            rules.push("font-weight: bold;".to_string());
        }

        if enable_italic && self.italic {
            rules.push("font-style: italic;".to_string());
        }

        let td = &self.text_decoration;
        let underline_css = match td.underline {
            UnderlineStyle::None => None,
            UnderlineStyle::Solid => Some("underline"),
            UnderlineStyle::Wavy => Some("underline wavy"),
            UnderlineStyle::Double => Some("underline double"),
            UnderlineStyle::Dotted => Some("underline dotted"),
            UnderlineStyle::Dashed => Some("underline dashed"),
        };

        match (underline_css, td.strikethrough) {
            (Some(u), true) => rules.push(format!("text-decoration: {u} line-through;")),
            (Some(u), false) => rules.push(format!("text-decoration: {u};")),
            (None, true) => rules.push("text-decoration: line-through;".to_string()),
            (None, false) => (),
        }

        rules.join(separator)
    }
}

include!(concat!(env!("OUT_DIR"), "/theme_data.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_available_themes() {
        let themes: Vec<_> = available_themes().collect();

        assert!(!themes.is_empty(), "no themes are compiled in");

        let dracula = themes.iter().find(|t| t.name == "dracula").unwrap();
        assert_eq!(dracula.name, "dracula");
        assert_eq!(dracula.appearance, Appearance::Dark);

        let github_light = themes.iter().find(|t| t.name == "github_light").unwrap();
        assert_eq!(github_light.name, "github_light");
        assert_eq!(github_light.appearance, Appearance::Light);

        for theme in themes {
            assert!(!theme.name.is_empty(), "a theme has no name");
            assert!(theme.appearance == Appearance::Light || theme.appearance == Appearance::Dark);
        }
    }

    #[test]
    fn available_themes_is_sorted_by_name() {
        let names: Vec<&str> = available_themes()
            .map(|theme| theme.name.as_str())
            .collect();

        assert!(
            names.len() > 200,
            "expected the full theme corpus, got {}",
            names.len()
        );

        let mut sorted = names.clone();
        sorted.sort_unstable();
        assert_eq!(
            names, sorted,
            "available_themes() must not depend on filesystem order"
        );
    }

    #[test]
    fn test_load_all_themes() {
        for theme in ALL_THEMES.iter() {
            assert!(!theme.name.is_empty(), "a theme has no name");
        }

        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let expected_count = std::fs::read_dir(manifest_dir.join("themes"))
            .unwrap()
            .filter_map(std::result::Result::ok)
            .filter(|entry| entry.path().extension().and_then(|s| s.to_str()) == Some("json"))
            .count();

        assert_eq!(ALL_THEMES.len(), expected_count);
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
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let path = manifest_dir.join("themes/catppuccin_frappe.json");
        let theme = from_file(&path).unwrap();

        assert_eq!(theme.name, "catppuccin_frappe");

        assert!(theme.get_style("tag.attribute").is_some());
    }

    #[test]
    fn test_style_css() {
        let style = Style {
            fg: Some("blue".to_string()),
            italic: true,
            text_decoration: TextDecoration {
                underline: UnderlineStyle::Solid,
                strikethrough: false,
            },
            ..Default::default()
        };

        assert_eq!(
            style.css(true, " "),
            "color: blue; font-style: italic; text-decoration: underline;"
        );
    }

    #[test]
    fn test_css_builder_default_stylesheet() {
        let json = r#"{"name": "test", "appearance": "dark", "revision": "3e976b4", "highlights": {"normal": {"fg": "red", "bg": "green"}, "keyword": {"fg": "blue", "italic": true}, "tag.attribute": {"bg": "gray", "bold": true}}}"#;
        let theme = from_json(json).unwrap();

        let expected = r"/* test
 * revision: 3e976b4
 */
.lumis {
  color: red;
  background-color: green;
}
.l-keyword {
  color: blue;
  font-style: italic;
}
.l-tag-attribute {
  background-color: gray;
  font-weight: bold;
}
";

        assert_eq!(
            CssBuilder::new(&theme).enable_italic(true).build(),
            expected
        );
    }

    #[test]
    fn test_css_builder_can_disable_italic() {
        let json = r#"{"name": "test", "appearance": "dark", "revision": "3e976b4", "highlights": {"keyword": {"fg": "blue", "italic": true}}}"#;
        let theme = from_json(json).unwrap();

        let css = CssBuilder::new(&theme).enable_italic(false).build();

        assert!(css.contains(".l-keyword {\n  color: blue;\n}"));
        assert!(!css.contains("font-style: italic;"));
    }

    #[test]
    fn test_css_builder_scopes_selectors_and_container_style() {
        let json = r#"{"name": "test", "appearance": "dark", "revision": "3e976b4", "highlights": {"normal": {"fg": "red", "bg": "green"}, "keyword": {"fg": "blue"}}}"#;
        let theme = from_json(json).unwrap();

        let expected = r#"/* test
 * revision: 3e976b4
 */
html[data-theme="dark"] .lumis {
  color: red;
  background-color: var(--color-grey-900);
  border-radius: 0.375rem;
}
html[data-theme="dark"] .l-keyword {
  color: blue;
}
"#;

        assert_eq!(
            CssBuilder::new(&theme)
                .scope("html[data-theme=\"dark\"]")
                .container_selector(".lumis")
                .container_style([
                    ("background-color", "var(--color-grey-900)"),
                    ("border-radius", "0.375rem"),
                ])
                .build(),
            expected
        );
    }

    #[test]
    fn test_css_builder_emits_italic_by_default() {
        let json = r#"{"name": "test", "appearance": "dark", "revision": "abc", "highlights": {"keyword": {"fg": "blue", "italic": true}}}"#;
        let theme = from_json(json).unwrap();

        let expected = r"/* test
 * revision: abc
 */
.lumis {}
.l-keyword {
  color: blue;
  font-style: italic;
}
";

        // No `enable_italic` call: the default must be `true`.
        assert_eq!(CssBuilder::new(&theme).build(), expected);
    }

    #[test]
    fn test_css_builder_empty_theme_renders_empty_base_rule() {
        let json = r#"{"name": "test", "appearance": "dark", "revision": "abc", "highlights": {}}"#;
        let theme = from_json(json).unwrap();

        let expected = r"/* test
 * revision: abc
 */
.lumis {}
";

        assert_eq!(CssBuilder::new(&theme).build(), expected);
    }

    #[test]
    fn test_css_builder_container_rule_keeps_theme_background() {
        let json = r#"{"name": "test", "appearance": "dark", "revision": "abc", "highlights": {"normal": {"fg": "red", "bg": "green"}, "keyword": {"fg": "blue"}}}"#;
        let theme = from_json(json).unwrap();

        let expected = r"/* test
 * revision: abc
 */
.lumis {
  color: red;
  background-color: green;
  padding: 1rem;
}
.l-keyword {
  color: blue;
}
";

        assert_eq!(
            CssBuilder::new(&theme)
                .container_style([("padding", "1rem")])
                .build(),
            expected
        );
    }

    #[test]
    fn test_css_builder_container_rule_adds_background_when_theme_lacks_bg() {
        let json = r#"{"name": "test", "appearance": "dark", "revision": "abc", "highlights": {"normal": {"fg": "red"}}}"#;
        let theme = from_json(json).unwrap();

        let expected = r"/* test
 * revision: abc
 */
.lumis {
  color: red;
  background-color: #000;
}
";

        assert_eq!(
            CssBuilder::new(&theme)
                .container_style([("background-color", "#000")])
                .build(),
            expected
        );
    }

    #[test]
    fn test_css_builder_container_rule_overrides_theme_background() {
        let json = r#"{"name": "test", "appearance": "dark", "revision": "abc", "highlights": {"normal": {"fg": "red", "bg": "green"}}}"#;
        let theme = from_json(json).unwrap();

        let expected = r"/* test
 * revision: abc
 */
.lumis {
  color: red;
  background-color: #000;
}
";

        // The theme's `background-color: green` is replaced in place, not duplicated.
        let css = CssBuilder::new(&theme)
            .container_style([("background-color", "#000")])
            .build();
        assert_eq!(css, expected);
        assert!(!css.contains("green"));
    }

    #[test]
    fn test_css_builder_omits_rules_that_render_no_declarations() {
        // `comment` only carries italic, so disabling italic leaves it with nothing to emit, while
        // `function` keeps its color and must still be present.
        let json = r#"{"name": "test", "appearance": "dark", "revision": "abc", "highlights": {"function": {"fg": "red"}, "comment": {"italic": true}}}"#;
        let theme = from_json(json).unwrap();

        let css = CssBuilder::new(&theme).enable_italic(false).build();

        assert!(css.contains(".l-function {\n  color: red;\n}"));
        assert!(!css.contains(".l-comment"));
    }

    #[test]
    fn test_css_builder_container_style_preserves_insertion_order() {
        let json = r#"{"name": "test", "appearance": "dark", "revision": "abc", "highlights": {"normal": {"fg": "red"}}}"#;
        let theme = from_json(json).unwrap();

        let expected = r"/* test
 * revision: abc
 */
.lumis {
  color: red;
  border-radius: 0.375rem;
  padding: 1rem;
  overflow-x: auto;
}
";

        assert_eq!(
            CssBuilder::new(&theme)
                .container_style([
                    ("border-radius", "0.375rem"),
                    ("padding", "1rem"),
                    ("overflow-x", "auto"),
                ])
                .build(),
            expected
        );
    }

    #[test]
    fn test_css_builder_scope() {
        let json = r#"{"name": "test", "appearance": "dark", "revision": "abc", "highlights": {"normal": {"fg": "red"}, "keyword": {"fg": "blue"}}}"#;
        let theme = from_json(json).unwrap();

        let expected = r"/* test
 * revision: abc
 */
.app .lumis {
  color: red;
}
.app .l-keyword {
  color: blue;
}
";

        assert_eq!(CssBuilder::new(&theme).scope(".app").build(), expected);
    }

    #[test]
    fn test_css_builder_renders_text_decorations() {
        let json = r#"{"name": "test", "appearance": "dark", "revision": "abc", "highlights": {"keyword": {"fg": "blue", "bold": true, "italic": true, "underline": "double", "strikethrough": true}}}"#;
        let theme = from_json(json).unwrap();

        let expected = r"/* test
 * revision: abc
 */
.lumis {}
.l-keyword {
  color: blue;
  font-weight: bold;
  font-style: italic;
  text-decoration: underline double line-through;
}
";

        assert_eq!(CssBuilder::new(&theme).build(), expected);
    }

    #[test]
    fn test_css_builder_nested_scope_dots_become_dashes() {
        let json = r#"{"name": "test", "appearance": "dark", "revision": "abc", "highlights": {"markup.heading.1.markdown": {"fg": "red"}}}"#;
        let theme = from_json(json).unwrap();

        let css = CssBuilder::new(&theme).build();

        assert!(css.contains(".l-markup-heading-1-markdown {\n  color: red;\n}"));
    }

    #[test]
    fn test_css_builder_build_does_not_consume_and_setters_mutate_in_place() {
        let json = r#"{"name": "test", "appearance": "dark", "revision": "abc", "highlights": {"keyword": {"fg": "blue", "italic": true}}}"#;
        let theme = from_json(json).unwrap();

        let mut builder = CssBuilder::new(&theme);

        let first = builder.build();
        let second = builder.build();
        assert_eq!(first, second, "build(&self) must be repeatable");
        assert!(first.contains("font-style: italic;"));

        builder.enable_italic(false);
        let third = builder.build();
        assert!(
            !third.contains("font-style: italic;"),
            "mutating a setter must be reflected on the next build"
        );
    }

    #[test]
    fn test_css_builder_new_sets_required_fields() {
        // `build()` calls the private `fallible_build().expect(...)`. That expect can only panic if
        // a required field is left unset. This guards the invariant that `new` initializes every
        // required field, so a future required field added without updating `new` fails here.
        let json = r#"{"name": "test", "appearance": "dark", "revision": "abc", "highlights": {}}"#;
        let theme = from_json(json).unwrap();

        let _ = CssBuilder::new(&theme).build();
    }

    #[test]
    fn test_css_builder_default_matches_bundled_css_files() {
        // The bundled `crates/lumis/css/<theme>.css` files are generated by the `dev` tool using a
        // default builder. A default `CssBuilder` must reproduce them byte-for-byte, which proves
        // it emits the canonical, valid stylesheet for every bundled theme with no customization.
        let css_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("lumis")
            .join("css");

        for theme in available_themes() {
            let css_path = css_dir.join(format!("{}.css", theme.name));
            let expected = std::fs::read_to_string(&css_path)
                .unwrap_or_else(|e| panic!("missing bundled CSS for {}: {e}", theme.name));
            let expected = expected.replace("\r\n", "\n");

            assert_eq!(
                CssBuilder::new(theme).build(),
                expected,
                "default CssBuilder output drifted from {}",
                css_path.display()
            );
        }
    }

    #[test]
    fn test_get_style_specialized() {
        let json = r##"{
            "name": "test",
            "appearance": "dark",
            "revision": "test",
            "highlights": {
                "comment": {"fg": "#666666"},
                "comment.lua": {"fg": "#888888", "italic": true},
                "markup.heading.2": {"fg": "#ff0000", "bold": true},
                "markup.heading.2.markdown": {"fg": "#00ff00", "bold": true}
            }
        }"##;
        let theme = from_json(json).unwrap();

        let lua_comment = theme.get_style("comment.lua");
        assert_eq!(
            lua_comment,
            Some(&Style {
                fg: Some("#888888".to_string()),
                italic: true,
                ..Default::default()
            })
        );

        let md_heading = theme.get_style("markup.heading.2.markdown");
        assert_eq!(
            md_heading,
            Some(&Style {
                fg: Some("#00ff00".to_string()),
                bold: true,
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_get_style_fallback_to_generic() {
        let json = r##"{
            "name": "test",
            "appearance": "dark",
            "revision": "test",
            "highlights": {
                "comment": {"fg": "#666666"},
                "markup.heading.2": {"fg": "#ff0000", "bold": true}
            }
        }"##;
        let theme = from_json(json).unwrap();

        let rust_comment = theme.get_style("comment.rust");
        assert_eq!(
            rust_comment,
            Some(&Style {
                fg: Some("#666666".to_string()),
                ..Default::default()
            })
        );

        let md_heading = theme.get_style("markup.heading.2.markdown");
        assert_eq!(
            md_heading,
            Some(&Style {
                fg: Some("#ff0000".to_string()),
                bold: true,
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_get_style_fallback_to_parent_scope() {
        let json = r##"{
            "name": "test",
            "appearance": "dark",
            "revision": "test",
            "highlights": {
                "markup.heading": {"fg": "#ff0000", "bold": true}
            }
        }"##;
        let theme = from_json(json).unwrap();

        let md_heading = theme.get_style("markup.heading.2.markdown");
        assert_eq!(
            md_heading,
            Some(&Style {
                fg: Some("#ff0000".to_string()),
                bold: true,
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_get_style_no_match() {
        let json = r##"{
            "name": "test",
            "appearance": "dark",
            "revision": "test",
            "highlights": {
                "keyword": {"fg": "#ff0000"}
            }
        }"##;
        let theme = from_json(json).unwrap();

        let result = theme.get_style("comment.rust");
        assert!(result.is_none());
    }

    #[test]
    fn test_get_style_prefers_specialized_over_generic() {
        let json = r##"{
            "name": "test",
            "appearance": "dark",
            "revision": "test",
            "highlights": {
                "string.special.symbol": {"fg": "#aaaaaa"},
                "string.special.symbol.ruby": {"fg": "#bbbbbb", "bold": true},
                "string.special.symbol.elixir": {"fg": "#cccccc", "italic": true}
            }
        }"##;
        let theme = from_json(json).unwrap();

        let ruby_symbol = theme.get_style("string.special.symbol.ruby");
        assert_eq!(
            ruby_symbol,
            Some(&Style {
                fg: Some("#bbbbbb".to_string()),
                bold: true,
                ..Default::default()
            })
        );

        let elixir_symbol = theme.get_style("string.special.symbol.elixir");
        assert_eq!(
            elixir_symbol,
            Some(&Style {
                fg: Some("#cccccc".to_string()),
                italic: true,
                ..Default::default()
            })
        );

        let python_symbol = theme.get_style("string.special.symbol.python");
        assert_eq!(
            python_symbol,
            Some(&Style {
                fg: Some("#aaaaaa".to_string()),
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_get_style_real_world_scenario() {
        let json = r##"{
            "name": "test",
            "appearance": "dark",
            "revision": "test",
            "highlights": {
                "markup.heading": {"fg": "#ff6b6b"},
                "markup.heading.1": {"fg": "#ff0000", "bold": true},
                "markup.heading.2": {"fg": "#00ff00", "bold": true},
                "markup.heading.2.markdown": {"fg": "#4ecdc4", "bold": true, "underline": true},
                "string.special.symbol": {"fg": "#ffeaa7"},
                "string.special.symbol.elixir": {"fg": "#fab1a0", "italic": true}
            }
        }"##;
        let theme = from_json(json).unwrap();

        let markdown_h2 = theme.get_style("markup.heading.2.markdown");
        assert_eq!(
            markdown_h2,
            Some(&Style {
                fg: Some("#4ecdc4".to_string()),
                bold: true,
                text_decoration: TextDecoration {
                    underline: UnderlineStyle::Solid,
                    strikethrough: false,
                },
                ..Default::default()
            })
        );

        let elixir_atom = theme.get_style("string.special.symbol.elixir");
        assert_eq!(
            elixir_atom,
            Some(&Style {
                fg: Some("#fab1a0".to_string()),
                italic: true,
                ..Default::default()
            })
        );

        let rust_symbol = theme.get_style("string.special.symbol.rust");
        assert_eq!(
            rust_symbol,
            Some(&Style {
                fg: Some("#ffeaa7".to_string()),
                ..Default::default()
            })
        );
    }
}

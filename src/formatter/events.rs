//! Helper utilities for building custom formatters.
//!
//! This module provides low-level and high-level APIs for creating custom formatters.
//! It exposes the tree-sitter highlight event stream and provides ergonomic utilities
//! for processing syntax highlighting tokens.
//!
//! # Two Levels of APIs
//!
//! ## Low-Level API: Direct Event Access
//!
//! For maximum control, you can work directly with tree-sitter [`HighlightEvent`] stream:
//!
//! ```rust
//! use autumnus::formatter::events::highlight_events;
//! use autumnus::languages::Language;
//! use tree_sitter_highlight::HighlightEvent;
//!
//! let source = "fn main() {}";
//! let lang = Language::Rust;
//!
//! for event in highlight_events(source, lang) {
//!     match event {
//!         HighlightEvent::HighlightStart(idx) => {
//!             println!("Start highlight scope {}", idx.0);
//!         }
//!         HighlightEvent::Source { start, end } => {
//!             println!("Source text: {:?}", &source[start..end]);
//!         }
//!         HighlightEvent::HighlightEnd => {
//!             println!("End highlight scope");
//!         }
//!     }
//! }
//! ```
//!
//! ## High-Level API: Token Iteration
//!
//! For easier use, iterate over pre-processed tokens with text and style:
//!
//! ```rust
//! use autumnus::formatter::events::iter_tokens;
//! use autumnus::languages::Language;
//! use autumnus::themes;
//!
//! let source = "fn main() { println!(\"Hello\"); }";
//! let lang = Language::Rust;
//! let theme = themes::get("dracula");
//!
//! for token in iter_tokens(source, lang, theme.ok()) {
//!     println!(
//!         "Text: {:?}, Scope: {}, Style: {:?}",
//!         token.text,
//!         token.scope,
//!         token.style
//!     );
//! }
//! ```

use crate::{constants::HIGHLIGHT_NAMES, languages::Language, themes::{Style, Theme}};
use std::borrow::Cow;
use tree_sitter_highlight::{HighlightEvent, Highlighter};

/// Returns an iterator over highlight events for the given source code.
///
/// This is the low-level API for creating custom formatters. It gives you direct
/// access to the tree-sitter highlight event stream, allowing you to process
/// syntax highlighting at the most fundamental level.
///
/// # Arguments
///
/// * `source` - The source code to highlight
/// * `lang` - The programming language of the source code
///
/// # Returns
///
/// An iterator that yields [`HighlightEvent`] items. Each event represents:
/// - `HighlightStart(Highlight)` - Beginning of a syntax scope (e.g., keyword, string)
/// - `Source { start, end }` - A span of source text with byte offsets
/// - `HighlightEnd` - End of the current syntax scope
///
/// # Event Processing Pattern
///
/// Process events using a stack-based approach to track nested scopes:
///
/// ```rust
/// use autumnus::formatter::events::highlight_events;
/// use autumnus::languages::Language;
/// use tree_sitter_highlight::HighlightEvent;
///
/// let source = "const x = 42;";
/// let mut scope_stack: Vec<usize> = Vec::new();
///
/// for event in highlight_events(source, Language::JavaScript) {
///     match event {
///         HighlightEvent::HighlightStart(highlight) => {
///             scope_stack.push(highlight.0);
///             // Apply styling for this scope
///         }
///         HighlightEvent::Source { start, end } => {
///             let text = &source[start..end];
///             // Render text with current scope's style
///         }
///         HighlightEvent::HighlightEnd => {
///             scope_stack.pop();
///             // Restore previous scope's style
///         }
///     }
/// }
/// ```
///
/// # Panics
///
/// Panics if tree-sitter fails to generate highlight events. This is rare and
/// typically indicates an internal tree-sitter error or corrupted language grammar.
///
/// # See Also
///
/// - [`iter_tokens`] - Higher-level API that provides pre-processed tokens
/// - [`scope_name`] - Convert highlight index to scope name string
/// - [`theme_style_for_scope`] - Look up theme style for a scope
pub fn highlight_events(
    source: &str,
    lang: Language,
) -> impl Iterator<Item = HighlightEvent> + '_ {
    // Collect events into a Vec to avoid borrowing issues
    let mut highlighter = Highlighter::new();
    let events: Vec<_> = highlighter
        .highlight(
            lang.config(),
            source.as_bytes(),
            None,
            |injected| Some(Language::guess(injected, "").config()),
        )
        .expect("failed to generate highlight events")
        .map(|event| event.expect("failed to get highlight event"))
        .collect();

    events.into_iter()
}

/// Converts a highlight index to its corresponding scope name.
///
/// Tree-sitter uses numeric indices to represent syntax scopes. This function
/// maps those indices to human-readable scope names like "keyword", "string",
/// "function", etc.
///
/// # Arguments
///
/// * `highlight_idx` - The highlight index from [`HighlightEvent::HighlightStart`]
///
/// # Returns
///
/// The scope name as a string slice (e.g., "keyword", "string", "comment").
///
/// # Examples
///
/// ```rust
/// use autumnus::formatter::events::{highlight_events, scope_name};
/// use autumnus::languages::Language;
/// use tree_sitter_highlight::HighlightEvent;
///
/// let source = "fn test() {}";
/// for event in highlight_events(source, Language::Rust) {
///     if let HighlightEvent::HighlightStart(highlight) = event {
///         let name = scope_name(highlight.0);
///         println!("Scope: {}", name); // e.g., "keyword", "function"
///     }
/// }
/// ```
///
/// # Panics
///
/// Panics if the highlight index is out of bounds. This should never happen
/// with indices from tree-sitter's highlight events.
pub fn scope_name(highlight_idx: usize) -> &'static str {
    HIGHLIGHT_NAMES[highlight_idx]
}

/// Looks up the theme style for a given scope name.
///
/// # Arguments
///
/// * `theme` - The theme to query (if `None`, returns `None`)
/// * `scope` - The scope name (e.g., "keyword", "string", "function")
///
/// # Returns
///
/// The [`Style`] for the scope if found, or `None` if:
/// - No theme was provided
/// - The theme doesn't define styling for this scope
///
/// # Examples
///
/// ```rust
/// use autumnus::formatter::events::theme_style_for_scope;
/// use autumnus::themes;
///
/// let theme = themes::get("dracula");
/// if let Some(style) = theme_style_for_scope(theme.ok(), "keyword") {
///     println!("Keyword color: {:?}", style.fg);
///     println!("Bold: {}", style.bold);
///     println!("Italic: {}", style.italic);
/// }
/// ```
pub fn theme_style_for_scope<'a>(theme: Option<&'a Theme>, scope: &str) -> Option<&'a Style> {
    theme.and_then(|t| t.get_style(scope))
}

/// A syntax token with its text content and associated style information.
///
/// This struct represents a single semantic unit of source code (like a keyword,
/// string, or identifier) along with its scope name and optional theme styling.
///
/// # Fields
///
/// * `text` - The source text of this token
/// * `scope` - The highlight scope name (e.g., "keyword", "string", "comment")
/// * `style` - The theme style if a theme is provided and defines this scope
///
/// # Examples
///
/// Process tokens and format them as HTML:
///
/// ```rust
/// use autumnus::formatter::events::iter_tokens;
/// use autumnus::languages::Language;
/// use autumnus::themes;
///
/// let source = "fn main() {}";
/// let theme = themes::get("dracula");
///
/// for token in iter_tokens(source, Language::Rust, theme.ok()) {
///     if let Some(style) = token.style {
///         if let Some(fg) = &style.fg {
///             print!("<span style=\"color: {}\">{}</span>", fg, token.text);
///         } else {
///             print!("{}", token.text);
///         }
///     } else {
///         print!("{}", token.text);
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct HighlightToken<'a> {
    /// The source text of this token.
    pub text: Cow<'a, str>,
    /// The highlight scope name (e.g., "keyword", "string", "function").
    pub scope: Cow<'static, str>,
    /// The theme style for this scope, if available.
    pub style: Option<&'a Style>,
}

/// Returns an iterator over high-level syntax tokens.
///
/// This is the high-level API for creating custom formatters. It processes the
/// raw highlight event stream and yields ergonomic [`HighlightToken`] structs
/// that combine text, scope names, and styling information.
///
/// # Arguments
///
/// * `source` - The source code to highlight
/// * `lang` - The programming language of the source code
/// * `theme` - Optional theme for style lookups
///
/// # Returns
///
/// An iterator that yields [`HighlightToken`] items, each representing a
/// syntactically meaningful piece of text with its scope and style.
///
/// # Examples
///
/// ## Basic usage with theme
///
/// ```rust
/// use autumnus::formatter::events::iter_tokens;
/// use autumnus::languages::Language;
/// use autumnus::themes;
///
/// let source = "const x = 42;";
/// let theme = themes::get("catppuccin_mocha");
///
/// for token in iter_tokens(source, Language::JavaScript, theme.ok()) {
///     println!("{}: {} (scope: {})",
///         if token.style.is_some() { "Styled" } else { "Plain" },
///         token.text,
///         token.scope
///     );
/// }
/// ```
///
/// ## Building a custom Markdown formatter
///
/// ```rust
/// use autumnus::formatter::events::iter_tokens;
/// use autumnus::languages::Language;
///
/// let source = "// This is a comment\nfn main() {}";
/// let mut markdown = String::new();
///
/// for token in iter_tokens(source, Language::Rust, None) {
///     match token.scope.as_ref() {
///         "comment" => {
///             markdown.push_str(&format!("*{}*", token.text));
///         }
///         "keyword" => {
///             markdown.push_str(&format!("**{}**", token.text));
///         }
///         _ => {
///             markdown.push_str(&token.text);
///         }
///     }
/// }
///
/// println!("{}", markdown);
/// ```
///
/// ## Building a JSON formatter
///
/// ```rust
/// use autumnus::formatter::events::iter_tokens;
/// use autumnus::languages::Language;
/// use autumnus::themes;
///
/// let source = "print('hello')";
/// let theme = themes::get("dracula");
///
/// let tokens: Vec<_> = iter_tokens(source, Language::Python, theme.ok())
///     .map(|token| {
///         serde_json::json!({
///             "text": token.text,
///             "scope": token.scope,
///             "color": token.style.and_then(|s| s.fg.as_ref()),
///             "bold": token.style.map(|s| s.bold).unwrap_or(false),
///         })
///     })
///     .collect();
///
/// let json = serde_json::to_string_pretty(&tokens).unwrap();
/// println!("{}", json);
/// ```
pub fn iter_tokens<'a>(
    source: &'a str,
    lang: Language,
    theme: Option<&'a Theme>,
) -> impl Iterator<Item = HighlightToken<'a>> + 'a {
    let mut current_scope: Option<&'static str> = None;
    let mut current_style: Option<&'a Style> = None;

    highlight_events(source, lang).filter_map(move |event| match event {
        HighlightEvent::HighlightStart(highlight) => {
            let scope = scope_name(highlight.0);
            let style = theme_style_for_scope(theme, scope);
            current_scope = Some(scope);
            current_style = style;
            None
        }
        HighlightEvent::Source { start, end } => {
            let text = &source[start..end];
            let scope = current_scope.unwrap_or("text");
            Some(HighlightToken {
                text: Cow::Borrowed(text),
                scope: Cow::Borrowed(scope),
                style: current_style,
            })
        }
        HighlightEvent::HighlightEnd => {
            current_scope = None;
            current_style = None;
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_events_basic() {
        let source = "fn main() {}";
        let events: Vec<_> = highlight_events(source, Language::Rust).collect();
        assert!(!events.is_empty(), "Should generate highlight events");
    }

    #[test]
    fn test_scope_name() {
        let name = scope_name(0);
        assert!(!name.is_empty(), "Scope name should not be empty");
    }

    #[test]
    fn test_theme_style_for_scope() {
        let result = theme_style_for_scope(None, "keyword");
        assert!(result.is_none(), "Should return None when no theme");
    }

    #[test]
    fn test_iter_tokens_basic() {
        let source = "fn test() {}";
        let tokens: Vec<_> = iter_tokens(source, Language::Rust, None).collect();
        assert!(!tokens.is_empty(), "Should generate tokens");

        // Should have at least a keyword token
        let has_keyword = tokens.iter().any(|t| t.scope.contains("keyword"));
        assert!(has_keyword, "Should identify keyword tokens");
    }

    #[test]
    fn test_iter_tokens_preserves_text() {
        let source = "const x = 42;";
        let reconstructed: String = iter_tokens(source, Language::JavaScript, None)
            .map(|t| t.text.into_owned())
            .collect();
        assert_eq!(source, reconstructed, "Token text should reconstruct source");
    }
}

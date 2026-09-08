//! `BBCode` formatter for syntax highlighting.
//!
//! This module provides the [`BBCodeScoped`] formatter that generates `BBCode` output with
//! highlight scope names as tags (e.g., `[keyword-function]text[/keyword-function]`).
//!
//! It does not emit standard forum-style `BBCode` such as `[b]`, `[color]`, or `[code]`.
//!
//! # Example Output
//!
//! For the Rust code `fn main() {}`, the formatter generates:
//!
//! ```text
//! [keyword-function]fn[/keyword-function] [function]main[/function][punctuation-bracket]([/punctuation-bracket][punctuation-bracket])[/punctuation-bracket] [punctuation-bracket]{[/punctuation-bracket][punctuation-bracket]}[/punctuation-bracket]
//! ```
//!
//! See the [formatter](crate::formatter) module for more information and examples.

use super::Formatter;
use crate::languages::Language;
use derive_builder::Builder;
use lumis_core::formatter::Formatter as _;
use std::io::{self, Write};

/// `BBCode` formatter for syntax highlighting using highlight scope names as tags.
///
/// Generates `BBCode` output using scope-based tags derived from tree-sitter scope names.
/// Dots in scope names are converted to hyphens (e.g., `keyword.function` becomes
/// `[keyword-function]...[/keyword-function]`).
/// It does not emit standard forum-style `BBCode` tags.
///
/// Use [`BBCodeScopedBuilder`] to create instances.
///
/// # Example
///
/// ```rust,ignore
/// use lumis::{BBCodeScopedBuilder, languages::Language, formatters::Formatter};
///
/// let code = "fn main() { println!(\"Hello\"); }";
///
/// let formatter = BBCodeScopedBuilder::new()
///     .language(Language::Rust)
///     .build()
///     .unwrap();
///
/// let mut output = Vec::new();
/// lumis::write_highlight(&mut output, code, formatter).unwrap();
/// let bbcode = String::from_utf8(output).unwrap();
/// ```
#[derive(Builder, Clone, Debug)]
#[builder(default)]
pub struct BBCodeScoped {
    #[builder(setter(custom))]
    language: Language,
}

impl BBCodeScopedBuilder {
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
}

impl BBCodeScoped {
    pub fn new(language: Language) -> Self {
        Self { language }
    }
}

impl Default for BBCodeScoped {
    fn default() -> Self {
        Self {
            language: Language::PlainText,
        }
    }
}

impl<T> Formatter<T> for BBCodeScoped {
    fn language(&self) -> Language {
        self.language
    }

    fn render(
        &self,
        source: &str,
        events: &[lumis_core::events::HighlightEvent<'_, T>],
        output: &mut dyn Write,
    ) -> io::Result<()> {
        let core_formatter = lumis_core::formatter::bbcode::BBCodeScoped::new(self.language);
        core_formatter.render(source, events, output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_attrs() {
        let code = "@lang :rust";
        let formatter = BBCodeScoped::new(Language::Elixir);
        let mut buffer = Vec::new();
        crate::write_highlight(&mut buffer, code, formatter).unwrap();
        let result = String::from_utf8(buffer).unwrap();

        assert!(result.contains('@'));
        assert!(result.contains("lang"));
        assert!(result.contains(":rust"));
    }

    #[test]
    fn test_plain_text() {
        let code = "hello world";
        let formatter = BBCodeScoped::new(Language::PlainText);
        let mut buffer = Vec::new();
        crate::write_highlight(&mut buffer, code, formatter).unwrap();
        let result = String::from_utf8(buffer).unwrap();

        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_builder_pattern() {
        let formatter = BBCodeScopedBuilder::new()
            .language(Language::Rust)
            .build()
            .unwrap();

        let code = "fn main() {}";
        let mut buffer = Vec::new();
        crate::write_highlight(&mut buffer, code, formatter).unwrap();
        let result = String::from_utf8(buffer).unwrap();

        assert!(result.contains("fn"));
        assert!(result.contains("main"));
    }
}

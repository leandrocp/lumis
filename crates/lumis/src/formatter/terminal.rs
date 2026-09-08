//! Terminal formatter with ANSI color codes.
//!
//! This module provides the [`Terminal`] formatter that generates terminal output with
//! ANSI color codes for syntax highlighting. Supports themes and automatic color
//! mapping from theme definitions to terminal colors.
//!
//! # Example Output
//!
//! For the Rust code `fn main() { println!("Hello"); }` with a theme applied,
//! the formatter generates ANSI-colored terminal output like:
//!
//! ```text
//! [0m[38;2;139;233;253mfn[0m [0m[38;2;80;250;123mmain[0m[0m[38;2;248;248;242m([0m[0m[38;2;248;248;242m)[0m [0m[38;2;248;248;242m{[0m [0m[38;2;189;147;249mprintln[0m[0m[38;2;80;250;123m![0m[0m[38;2;248;248;242m([0m[0m[38;2;241;250;140m"Hello"[0m[0m[38;2;248;248;242m)[0m[0m[38;2;248;248;242m;[0m [0m[38;2;248;248;242m}[0m
//! ```
//!
//! See the [formatter](crate::formatter) module for more information and examples.

use super::Formatter;
use crate::languages::Language;
use crate::themes::Theme;
use derive_builder::Builder;
pub use lumis_core::formatter::terminal::Background;
use lumis_core::formatter::Formatter as _;
use std::io::{self, Write};

/// Terminal formatter for syntax highlighting with ANSI color codes.
///
/// Generates terminal output with ANSI escape sequences. Use [`TerminalBuilder`] to create instances.
///
/// # Example
///
/// ```rust
/// use lumis::{TerminalBuilder, languages::Language, themes, formatters::Formatter};
/// use std::io::Write;
///
/// let code = "fn main() { println!(\"Hello\"); }";
/// let theme = themes::get("dracula").unwrap();
///
/// let formatter = TerminalBuilder::new()
///     .language(Language::Rust)
///     .theme(Some(theme))
///     .build()
///     .unwrap();
///
/// let mut output = Vec::new();
/// lumis::write_highlight(&mut output, code, formatter).unwrap();
/// println!("{}", String::from_utf8(output).unwrap());
/// ```
#[derive(Builder, Clone, Debug)]
#[builder(default)]
pub struct Terminal {
    #[builder(setter(custom))]
    language: Language,
    theme: Option<Theme>,
    background: Background,
    width: Option<usize>,
}

impl TerminalBuilder {
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

impl Terminal {
    pub fn new(
        language: Language,
        theme: Option<Theme>,
        background: Background,
        width: Option<usize>,
    ) -> Self {
        Self {
            language,
            theme,
            background,
            width,
        }
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Self {
            language: Language::PlainText,
            theme: None,
            background: Background::Inherit,
            width: None,
        }
    }
}

impl<T> Formatter<T> for Terminal {
    fn language(&self) -> Language {
        self.language
    }

    fn render(
        &self,
        source: &str,
        events: &[lumis_core::events::HighlightEvent<'_, T>],
        output: &mut dyn Write,
    ) -> io::Result<()> {
        let core_formatter = lumis_core::formatter::terminal::Terminal::new(
            self.language,
            self.theme.clone(),
            self.background.clone(),
            self.width,
        );
        core_formatter.render(source, events, output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_attrs() {
        let code = "@lang :rust";
        let formatter = Terminal::new(Language::Elixir, None, Background::Inherit, None);
        let mut buffer = Vec::new();
        crate::write_highlight(&mut buffer, code, formatter).unwrap();
        let result = String::from_utf8_lossy(&buffer);

        assert!(result.contains('@'));
        assert!(result.contains("lang"));
        assert!(result.contains(":rust"));
        // Without a theme, some tokens may not have styling, so just check the text is there
    }
}

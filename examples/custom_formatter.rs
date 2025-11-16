//! Creating a custom formatter
//!
//! This example demonstrates how to implement a custom formatter by implementing
//! the Formatter trait. Here we create a token metadata formatter that explicitly
//! shows what data is available from `highlight_iter()`.
//!
//! # How It Works
//!
//! 1. Implement the `Formatter` trait for your struct
//! 2. Use `highlight_iter()` to get styled tokens as `(Style, text, range)` tuples
//! 3. Access some the available data:
//!    - `Style` - colors (fg/bg)
//!    - `text` - the actual source code text
//!    - `range` - byte positions (start..end)
//!
//! # Output Format
//!
//! ```text
//! token (fg:color bg:color pos:start..end)
//! ```
//!
//! # Example Output
//!
//! For JavaScript code `const greeting = "Hello";`:
//!
//! ```text
//! const (fg:#ff79c6 bg:none pos:0..5)
//!   (fg:none bg:none pos:5..6)
//! greeting (fg:#f8f8f2 bg:none pos:6..14)
//!   (fg:none bg:none pos:14..15)
//! = (fg:#ff79c6 bg:none pos:15..16)
//! ```

use autumnus::{
    formatter::Formatter, highlight::highlight_iter, languages::Language, themes, write_highlight,
    Options,
};
use std::io::{self, Write};

/// A custom formatter that outputs token metadata to show available data
struct TokenMetadataFormatter {
    language: Language,
    theme: Option<autumnus::themes::Theme>,
}

impl TokenMetadataFormatter {
    fn new(language: Language, theme: Option<autumnus::themes::Theme>) -> Self {
        Self { language, theme }
    }
}

impl Formatter for TokenMetadataFormatter {
    fn format(&self, source: &str, output: &mut dyn Write) -> io::Result<()> {
        // Use highlight_iter() to get styled tokens
        // Returns an iterator of (Style, &str, Range<usize>) tuples
        let iter =
            highlight_iter(source, self.language, self.theme.clone()).map_err(io::Error::other)?;

        for (style, text, range) in iter {
            writeln!(
                output,
                "{} (fg:{} bg:{} pos:{}..{})",
                text.escape_debug(),
                style.fg.as_deref().unwrap_or("none"),
                style.bg.as_deref().unwrap_or("none"),
                range.start,
                range.end,
            )?;
        }

        Ok(())
    }
}

fn main() {
    let code = r#"const greeting = "Hello, World!";
console.log(greeting);"#;

    let theme = themes::get("dracula").ok();
    let lang = Language::guess(Some("javascript"), code);

    let formatter = TokenMetadataFormatter::new(lang, theme);

    let options = Options {
        language: Some("javascript"),
        formatter: Box::new(formatter),
    };

    write_highlight(&mut io::stdout(), code, options).expect("Failed to write output");
}

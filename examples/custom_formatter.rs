//! Creating a custom formatter
//!
//! This example demonstrates how to implement a custom formatter by implementing
//! the Formatter trait. Here we create a token metadata formatter that explicitly
//! shows what data is available from `highlight_iter()`.

use autumnus::{
    formatter::Formatter, highlight::highlight_iter, languages::Language, themes, write_highlight,
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
        // Returns an iterator of (Style, &str, Range<usize>, scope) tuples
        let iter =
            highlight_iter(source, self.language, self.theme.clone()).map_err(io::Error::other)?;

        for (style, text, range, scope) in iter {
            writeln!(
                output,
                "{} (scope:{} fg:{} bg:{} pos:{}..{})",
                text.escape_debug(),
                scope,
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

    write_highlight(&mut io::stdout(), code, formatter).expect("Failed to write output");
}

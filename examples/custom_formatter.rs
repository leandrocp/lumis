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
        // Use highlight_iter() with a callback to process styled tokens
        // The callback receives (text, range, scope, style) for each token
        highlight_iter(
            source,
            self.language,
            self.theme.clone(),
            |text, range, scope, style| {
                writeln!(
                    output,
                    "{} (pos:{}..{} scope:{} fg:{} bg:{})",
                    text.escape_debug(),
                    range.start,
                    range.end,
                    scope,
                    style.fg.as_deref().unwrap_or("none"),
                    style.bg.as_deref().unwrap_or("none"),
                )
            },
        )
        .map_err(io::Error::other)
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

//! Creating a custom formatter
//!
//! This example demonstrates how to implement a custom formatter
//! by implementing the Formatter trait. This allows you to create
//! output in any format you need (JSON, Markdown, XML, etc.)
//!
//! # Output
//!
//! ```json
//! {
//!   "language": "JavaScript",
//!   "tokens": [
//!     {
//!       "text": "const",
//!       "start": 0,
//!       "end": 5,
//!       "fg": "#ff79c6",
//!       "bold": false,
//!       "italic": false
//!     },
//!     {
//!       "text": " ",
//!       "start": 5,
//!       "end": 6,
//!       "bold": false,
//!       "italic": false
//!     },
//!     {
//!       "text": "greeting",
//!       "start": 6,
//!       "end": 14,
//!       "fg": "#f8f8f2",
//!       "bold": false,
//!       "italic": false
//!     },
//!     ...
//!   ]
//! }
//! ```

use autumnus::{
    formatter::Formatter, highlight::highlight_iter, languages::Language, themes, write_highlight,
    Options,
};
use std::io::{self, Write};

/// A custom formatter that outputs JSON with token information
struct JsonFormatter<'a> {
    source: &'a str,
    language: Language,
    theme: Option<autumnus::themes::Theme>,
}

impl<'a> JsonFormatter<'a> {
    fn new(source: &'a str, language: Language, theme: Option<autumnus::themes::Theme>) -> Self {
        Self {
            source,
            language,
            theme,
        }
    }
}

impl<'a> Formatter for JsonFormatter<'a> {
    fn format(&self, output: &mut dyn Write) -> io::Result<()> {
        writeln!(output, "{{")?;
        writeln!(output, r#"  "language": "{:?}","#, self.language)?;
        writeln!(output, r#"  "tokens": ["#)?;

        let iter = highlight_iter(self.source, self.language, self.theme.clone())
            .map_err(io::Error::other)?;

        let tokens: Vec<_> = iter.collect();
        for (i, (style, text, range)) in tokens.iter().enumerate() {
            let is_last = i == tokens.len() - 1;
            let comma = if is_last { "" } else { "," };

            // Escape the text for JSON
            let escaped_text = text
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', "\\n")
                .replace('\r', "\\r")
                .replace('\t', "\\t");

            writeln!(output, "    {{")?;
            writeln!(output, r#"      "text": "{}","#, escaped_text)?;
            writeln!(output, r#"      "start": {},"#, range.start)?;
            writeln!(output, r#"      "end": {},"#, range.end)?;

            if let Some(fg) = &style.fg {
                writeln!(output, r#"      "fg": "{}","#, fg)?;
            }
            if let Some(bg) = &style.bg {
                writeln!(output, r#"      "bg": "{}","#, bg)?;
            }

            writeln!(output, r#"      "bold": {},"#, style.bold)?;
            writeln!(output, r#"      "italic": {}"#, style.italic)?;
            writeln!(output, "    }}{}", comma)?;
        }

        writeln!(output, "  ]")?;
        writeln!(output, "}}")?;

        Ok(())
    }

    fn highlights(&self, _output: &mut dyn Write) -> io::Result<()> {
        // Not used for this formatter
        Ok(())
    }
}

fn main() {
    let code = r#"const greeting = "Hello, World!";
console.log(greeting);"#;

    let theme = themes::get("dracula").ok();
    let lang = Language::guess(Some("javascript"), code);

    // Create our custom formatter
    let formatter = JsonFormatter::new(code, lang, theme);

    let options = Options {
        language: Some("javascript"),
        formatter: Box::new(formatter),
    };

    // Use write_highlight to write to stdout
    write_highlight(&mut io::stdout(), options).expect("Failed to write output");
}

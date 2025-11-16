//! Custom terminal formatter using public helper functions.
//!
//! This example demonstrates how to create a custom terminal formatter using only
//! the public APIs from the `ansi` module, without needing to interact with
//! tree-sitter or termcolor internals directly.
//!
//! # Output
//!
//! The formatter adds line numbers to the terminal output with gray coloring.
//!
//! # Example Output
//!
//! ```text
//!   1 │ const greeting = "Hello, World!";
//!   2 │ console.log(greeting);
//! ```

use autumnus::{ansi, formatter::Formatter, languages::Language, themes, write_highlight, Options};
use std::io::{self, Write};

/// A custom terminal formatter that adds line numbers
struct LineNumberedTerminal {
    language: Language,
    theme: Option<autumnus::themes::Theme>,
}

impl LineNumberedTerminal {
    fn new(language: Language, theme: Option<autumnus::themes::Theme>) -> Self {
        Self { language, theme }
    }
}

impl Formatter for LineNumberedTerminal {
    fn format(&self, source: &str, output: &mut dyn Write) -> io::Result<()> {
        let iter = ansi::highlight_iter_with_ansi(source, self.language, self.theme.clone())
            .map_err(io::Error::other)?;

        let mut line_num = 1;
        let mut at_line_start = true;

        for (ansi_text, _range) in iter {
            if at_line_start {
                // Add line number in gray using ANSI helpers
                let gray_fg = ansi::rgb_to_ansi(128, 128, 128, false);
                write!(output, "{}{:3} │ {}", gray_fg, line_num, ansi::ANSI_RESET)?;
                at_line_start = false;
            }

            write!(output, "{}", ansi_text)?;

            if ansi_text.contains('\n') {
                line_num += ansi_text.matches('\n').count();
                at_line_start = true;
            }
        }

        Ok(())
    }
}

fn main() {
    let code = r#"const greeting = "Hello, World!";
console.log(greeting);"#;

    let theme = themes::get("dracula").ok();
    let lang = Language::guess(Some("javascript"), code);

    let formatter = LineNumberedTerminal::new(lang, theme);

    let options = Options {
        language: Some("javascript"),
        formatter: Box::new(formatter),
    };

    write_highlight(&mut io::stdout(), code, options).expect("Failed to write output");
}

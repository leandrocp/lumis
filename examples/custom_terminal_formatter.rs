//! Custom terminal formatter using public helper functions.
//!
//! This example demonstrates how to create a custom terminal formatter using only
//! the public APIs from the `ansi` module, without needing to interact with
//! tree-sitter or termcolor internals directly.
//!
//! # Output
//!
//! The formatter creates a minimal bat-style terminal output with:
//! - File header with horizontal divider lines
//! - Line numbers with gray coloring
//! - Syntax highlighted code content
//!
//! # Example Output
//!
//! ```text
//! ────────────────────────────────────────────────────────────────────────────────
//! File: src/index.html
//! ────────────────────────────────────────────────────────────────────────────────
//!   1 │ const greeting = "Hello, World!";
//!   2 │ console.log(greeting);
//! ```

use autumnus::{ansi, formatter::Formatter, languages::Language, themes, write_highlight, Options};
use std::io::{self, Write};

const HORIZONTAL_LINE: char = '─';

/// A custom terminal formatter that creates bat-style output with file header and line numbers
struct LineNumberedTerminal {
    language: Language,
    theme: Option<autumnus::themes::Theme>,
    filename: Option<String>,
    term_width: usize,
}

impl LineNumberedTerminal {
    fn new(language: Language, theme: Option<autumnus::themes::Theme>, filename: Option<String>) -> Self {
        Self {
            language,
            theme,
            filename,
            term_width: 80, // Default terminal width
        }
    }

    /// Print a horizontal line across the terminal width
    fn print_horizontal_line(&self, output: &mut dyn Write) -> io::Result<()> {
        let gray_fg = ansi::rgb_to_ansi(128, 128, 128, false);
        writeln!(
            output,
            "{}{}{}",
            gray_fg,
            HORIZONTAL_LINE.to_string().repeat(self.term_width),
            ansi::ANSI_RESET
        )
    }

    /// Print the file header with horizontal lines (bat-style)
    fn print_header(&self, output: &mut dyn Write) -> io::Result<()> {
        if let Some(ref filename) = self.filename {
            let gray_fg = ansi::rgb_to_ansi(128, 128, 128, false);

            // Top horizontal line
            self.print_horizontal_line(output)?;

            // File label
            writeln!(output, "{}File: {}{}", gray_fg, filename, ansi::ANSI_RESET)?;

            // Bottom horizontal line
            self.print_horizontal_line(output)?;
        }
        Ok(())
    }
}

impl Formatter for LineNumberedTerminal {
    fn format(&self, source: &str, output: &mut dyn Write) -> io::Result<()> {
        // Print bat-style header with filename
        self.print_header(output)?;

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

    let formatter = LineNumberedTerminal::new(lang, theme, Some("src/index.html".to_string()));

    let options = Options {
        language: Some("javascript"),
        formatter: Box::new(formatter),
    };

    write_highlight(&mut io::stdout(), code, options).expect("Failed to write output");
}

//! Custom terminal formatter using public helper functions.
//!
//! This example demonstrates how to create a custom terminal formatter using only
//! the public APIs from the `ansi` module, without needing to interact with
//! tree-sitter or termcolor internals directly.

use lumis::{
    ansi, events::HighlightEvent, formatters::Formatter, languages::Language, themes,
    write_highlight,
};
use std::io::{self, Write};

const HORIZONTAL_LINE: char = '─';

/// A custom terminal formatter that creates bat-style output with file header and line numbers
struct LineNumberedTerminal {
    language: Language,
    theme: Option<lumis::themes::Theme>,
    filename: Option<String>,
    term_width: usize,
}

impl LineNumberedTerminal {
    fn new(
        language: Language,
        theme: Option<lumis::themes::Theme>,
        filename: Option<String>,
    ) -> Self {
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
    fn language(&self) -> Language {
        self.language
    }

    fn render(
        &self,
        source: &str,
        events: &[HighlightEvent<'_>],
        output: &mut dyn Write,
    ) -> io::Result<()> {
        // Print bat-style header with filename
        self.print_header(output)?;

        let mut line_num = 1;
        let mut at_line_start = true;
        let mut scopes = Vec::new();

        for event in events {
            match event {
                HighlightEvent::Start {
                    scope_index,
                    language,
                } => scopes.push((*scope_index, language.as_str())),
                HighlightEvent::End => {
                    scopes.pop();
                }
                HighlightEvent::Source { start, end } => {
                    let text = &source[*start..*end];
                    let style = scopes.last().and_then(|(scope_index, language)| {
                        let theme = self.theme.as_ref()?;
                        let scope = lumis::highlights::HIGHLIGHT_NAMES[*scope_index];
                        let specialized_scope = format!("{scope}.{language}");
                        theme
                            .get_style(&specialized_scope)
                            .or_else(|| theme.get_style(scope))
                    });
                    let ansi_text =
                        style.map_or_else(|| text.to_owned(), |style| ansi::paint(text, style));

                    if at_line_start {
                        // Add line number in gray using ANSI helpers
                        let gray_fg = ansi::rgb_to_ansi(128, 128, 128, false);
                        let reset = ansi::ANSI_RESET;
                        write!(output, "{gray_fg}{line_num:3} │ {reset}")?;
                        at_line_start = false;
                    }

                    write!(output, "{ansi_text}")?;

                    if ansi_text.contains('\n') {
                        line_num += ansi_text.matches('\n').count();
                        at_line_start = true;
                    }
                }
                // Anything this formatter does not render, it skips.
                _ => {}
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

    write_highlight(&mut io::stdout(), code, formatter).expect("Failed to write output");
}

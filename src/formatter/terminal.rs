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

#![allow(unused_must_use)]

use super::Formatter;
use crate::{languages::Language, themes::Theme};
use derive_builder::Builder;
use std::io::{self, Write};
use termcolor::{BufferWriter, ColorChoice, ColorSpec, WriteColor};
use tree_sitter_highlight::{HighlightEvent, Highlighter};

#[derive(Builder, Debug)]
#[builder(default)]
pub struct Terminal<'a> {
    source: &'a str,
    lang: Language,
    theme: Option<&'a Theme>,
}

impl<'a> TerminalBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a> Terminal<'a> {
    pub fn new(source: &'a str, lang: Language, theme: Option<&'a Theme>) -> Self {
        Self {
            source,
            lang,
            theme,
        }
    }
}

impl Default for Terminal<'_> {
    fn default() -> Self {
        Self {
            source: "",
            lang: Language::PlainText,
            theme: None,
        }
    }
}

impl Formatter for Terminal<'_> {
    fn highlights(&self, output: &mut dyn Write) -> io::Result<()> {
        let mut highlighter = Highlighter::new();
        let events = highlighter
            .highlight(
                self.lang.config(),
                self.source.as_bytes(),
                None,
                |injected| Some(Language::guess(Some(injected), "").config()),
            )
            .expect("failed to generate highlight events");

        let writer = BufferWriter::stdout(ColorChoice::Always);
        let mut buffer = writer.buffer();

        for event in events {
            let event = event.expect("failed to get highlight event");

            match event {
                HighlightEvent::HighlightStart(idx) => {
                    let scope = crate::constants::HIGHLIGHT_NAMES[idx.0];

                    let hex = &self
                        .theme
                        .and_then(|theme| theme.get_style(scope))
                        .and_then(|style| style.fg.as_deref())
                        // not completely blank so it's still visible in light terminals
                        .unwrap_or("#eeeeee")
                        .trim_start_matches('#');

                    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
                    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
                    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();

                    buffer
                        .set_color(ColorSpec::new().set_fg(Some(termcolor::Color::Rgb(r, g, b))))?;
                }
                HighlightEvent::Source { start, end } => {
                    let text = self
                        .source
                        .get(start..end)
                        .expect("failed to get source bounds");

                    write!(buffer, "{text}")?;
                }
                HighlightEvent::HighlightEnd => {
                    buffer.reset()?;
                }
            }
        }

        output.write_all(buffer.as_slice())?;
        Ok(())
    }

    fn format(&self, output: &mut dyn Write) -> io::Result<()> {
        self.highlights(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    use pretty_assertions::assert_str_eq;

    #[test]
    fn test_no_attrs() {
        let formatter = Terminal::new("@lang :rust", Language::Elixir, None);
        let mut buffer = Vec::new();
        formatter.format(&mut buffer);
        let result = String::from_utf8(buffer).unwrap();
        let expected = "\u{1b}[0m\u{1b}[38;2;238;238;238m\u{1b}[0m\u{1b}[38;2;238;238;238m@\u{1b}[0m\u{1b}[38;2;238;238;238m\u{1b}[0m\u{1b}[38;2;238;238;238mlang \u{1b}[0m\u{1b}[38;2;238;238;238m:rust\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[0m\u{1b}[0m";
        assert_str_eq!(result, expected)
    }
}

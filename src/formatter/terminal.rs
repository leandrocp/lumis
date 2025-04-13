#![allow(unused_must_use)]

use super::Formatter;
use crate::{languages::Language, themes::Theme};
use std::io::{self, Write};
use termcolor::{BufferWriter, ColorChoice, ColorSpec, WriteColor};
use tree_sitter_highlight::{HighlightEvent, Highlighter};

#[derive(Debug)]
pub struct Terminal<'a> {
    source: &'a str,
    lang: Language,
    theme: Option<&'a Theme>,
}

impl<'a> Terminal<'a> {
    pub fn new(source: &'a str, lang: Language, theme: Option<&'a Theme>) -> Self {
        Self {
            source,
            lang,
            theme,
        }
    }

    pub fn with_source(mut self, source: &'a str) -> Self {
        self.source = source;
        self
    }

    pub fn with_lang(mut self, lang: Language) -> Self {
        self.lang = lang;
        self
    }

    pub fn with_theme(mut self, theme: Option<&'a Theme>) -> Self {
        self.theme = theme;
        self
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
                |injected| Some(Language::guess(injected, "").config()),
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

                    write!(buffer, "{}", text)?;
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

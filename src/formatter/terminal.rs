#![allow(unused_must_use)]

use super::Formatter;
use crate::{constants::HIGHLIGHT_NAMES, themes::Theme};
use std::cell::RefCell;
use std::io::Write;
use termcolor::{ColorSpec, WriteColor};
use tree_sitter_highlight::{Error, HighlightEvent};

pub struct Terminal<'a> {
    buffer: RefCell<termcolor::Buffer>,
    theme: Option<&'a Theme>,
}

impl<'a> Terminal<'a> {
    pub fn new(theme: Option<&'a Theme>) -> Self {
        Self {
            buffer: RefCell::new(termcolor::Buffer::ansi()),
            theme,
        }
    }
}

impl Formatter for Terminal<'_> {
    fn write<W>(
        &self,
        _writer: &mut W,
        source: &str,
        events: impl Iterator<Item = Result<HighlightEvent, Error>>,
    ) where
        W: std::fmt::Write,
    {
        for event in events {
            let event = event.expect("todo");

            match event {
                HighlightEvent::HighlightStart(idx) => {
                    let scope = HIGHLIGHT_NAMES[idx.0];

                    let hex: &str = self
                        .theme
                        .as_ref()
                        .and_then(|theme| theme.get_style(scope))
                        .and_then(|style| style.fg.as_deref())
                        // not completely blank so it's still visible in light terminals
                        .unwrap_or("#eeeeee")
                        .trim_start_matches('#');

                    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
                    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
                    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();

                    self.buffer
                        .borrow_mut()
                        .set_color(ColorSpec::new().set_fg(Some(termcolor::Color::Rgb(r, g, b))));
                }
                HighlightEvent::Source { start, end } => {
                    let text = source.get(start..end).expect("failed to get source bounds");
                    self.buffer.borrow_mut().write_all(text.as_bytes());
                }
                HighlightEvent::HighlightEnd => {
                    self.buffer.borrow_mut().reset();
                }
            }
        }
    }

    fn finish<W>(&self, writer: &mut W, _: &str)
    where
        W: std::fmt::Write,
    {
        let output = String::from_utf8(self.buffer.borrow_mut().clone().into_inner()).unwrap();
        let _ = writer.write_str(output.as_str());
    }
}

impl Default for Terminal<'_> {
    fn default() -> Self {
        Self {
            buffer: RefCell::new(termcolor::Buffer::ansi()),
            theme: None,
        }
    }
}

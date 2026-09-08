//! Formatter implementations for generating syntax highlighted output.
//!
//! This module provides formatters for rendering syntax highlighted code from
//! pre-computed highlight events. The formatters are independent of tree-sitter
//! and work with [`HighlightEvent`] streams.
//!
//! Available formatters:
//! - [`html_inline`] - HTML with inline CSS styles
//! - [`html_multi_themes`] - HTML with multiple theme support
//! - [`html_linked`] - HTML with CSS classes
//! - [`terminal`] - ANSI color codes for terminal output
//! - [`bbcode`] - `BBCode` scoped output using highlight scope names as tags

use crate::events::HighlightEvent;
use std::io::{self, Write};

pub mod ansi;
pub mod html;

pub mod html_inline;
pub use html_inline::{HtmlInline, HtmlInlineBuilder};

pub mod html_multi_themes;
pub use html_multi_themes::{HtmlMultiThemes, HtmlMultiThemesBuilder};

pub mod html_linked;
pub use html_linked::{HtmlLinked, HtmlLinkedBuilder};

pub mod terminal;
pub use terminal::{Background as TerminalBackground, Terminal, TerminalBuilder};

pub mod bbcode;
pub use bbcode::{BBCodeScoped, BBCodeScopedBuilder};

/// Configuration for wrapping the formatted output with custom HTML elements.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HtmlElement {
    /// The opening HTML tag that will be placed before the formatted code.
    pub open_tag: String,
    /// The closing HTML tag that will be placed after the formatted code.
    pub close_tag: String,
}

/// Trait for implementing custom syntax highlighting formatters.
///
/// Formatters in lumis-core work with pre-computed highlight events,
/// making them independent of tree-sitter.
pub trait Formatter<T = ()>: Send + Sync {
    /// Format source code using pre-computed highlight events.
    ///
    /// # Arguments
    ///
    /// * `source` - The source code to format
    /// * `events` - Pre-computed highlight events from tree-sitter or any other source
    /// * `output` - Writer to send formatted output to
    fn render(
        &self,
        source: &str,
        events: &[HighlightEvent<'_, T>],
        output: &mut dyn Write,
    ) -> io::Result<()>;
}

impl<T> Formatter<T> for Box<dyn Formatter<T>> {
    fn render(
        &self,
        source: &str,
        events: &[HighlightEvent<'_, T>],
        output: &mut dyn Write,
    ) -> io::Result<()> {
        (**self).render(source, events, output)
    }
}

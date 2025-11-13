//! Example of creating a custom formatter using the low-level event API.
//!
//! This demonstrates the advanced approach when you need maximum control over
//! event processing. Focus is on understanding how HighlightEvents work.
//!
//! Run with: `cargo run --example custom_formatter_low_level`

use autumnus::{
    formatter::{events, Formatter},
    highlight, languages::Language, Options,
};
use std::io::{self, Write};
use tree_sitter_highlight::HighlightEvent;

/// A custom formatter that adds explicit scope markers around each token.
///
/// This shows how to work directly with HighlightEvents to build your own
/// token processing logic. Useful when you need full control over the event
/// stream or want to build your own abstraction layer.
///
/// Output format: `[scope:text]`
/// Example: `[keyword:fn] [text: ][function:main][punctuation.bracket:(]`
pub struct ScopeMarkerFormatter<'a> {
    source: &'a str,
    lang: Language,
}

impl<'a> Formatter for ScopeMarkerFormatter<'a> {
    fn format(&self, output: &mut dyn Write) -> io::Result<()> {
        // Track the current scope as we process events
        let mut current_scope = "text";

        // Iterate over raw tree-sitter highlight events
        for event in events::highlight_events(self.source, self.lang) {
            match event {
                // HighlightStart: A new syntax scope begins (e.g., entering a keyword)
                HighlightEvent::HighlightStart(highlight) => {
                    // Convert the numeric highlight index to a scope name
                    current_scope = events::scope_name(highlight.0);
                }

                // Source: Actual source code text with byte offsets
                HighlightEvent::Source { start, end } => {
                    // Extract the text from the source using byte offsets
                    let text = &self.source[start..end];

                    // Output with explicit scope markers for visibility
                    write!(output, "[{}:{}]", current_scope, text)?;
                }

                // HighlightEnd: The current syntax scope ends
                HighlightEvent::HighlightEnd => {
                    // Reset to default scope
                    current_scope = "text";
                }
            }
        }
        Ok(())
    }

    fn highlights(&self, output: &mut dyn Write) -> io::Result<()> {
        self.format(output)
    }
}

fn main() {
    let rust_code = "fn greet(name: &str) { println!(\"Hello, {}!\", name); }";

    println!("Shows how HighlightEvents map to source code tokens.\n");
    println!("Input: {}\n", rust_code);

    let marker_formatter = ScopeMarkerFormatter {
        source: rust_code,
        lang: Language::Rust,
    };

    let result = highlight(
        rust_code,
        Options {
            lang_or_file: Some("rust"),
            formatter: Box::new(marker_formatter),
        },
    );

    println!("Output (with scope markers):");
    println!("{}\n", result);

    println!("Key Concepts:");
    println!("  • HighlightStart(idx) - Scope begins (e.g., keyword starts)");
    println!("  • Source{{start, end}} - Text with byte offsets");
    println!("  • HighlightEnd        - Current scope ends");
    println!();
    println!("Events are nested - use a stack to track scope hierarchy.");
    println!("Use events::scope_name() to convert indices to names.");
}

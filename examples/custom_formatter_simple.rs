//! Example of creating a custom Markdown formatter using the high-level token API.
//!
//! This demonstrates the recommended approach for most custom formatters:
//! using `events::iter_tokens()` for ergonomic access to syntax tokens.
//!
//! Run with: `cargo run --example custom_formatter_simple`

use autumnus::{
    formatter::{events, Formatter},
    highlight, languages::Language, Options,
};
use std::io::{self, Write};

/// A custom formatter that generates Markdown-style formatting.
///
/// - Comments become *italic*
/// - Keywords become **bold**
/// - Strings become `code`
/// - Functions become __underlined__
pub struct MarkdownFormatter<'a> {
    source: &'a str,
    lang: Language,
}

impl<'a> Formatter for MarkdownFormatter<'a> {
    fn format(&self, output: &mut dyn Write) -> io::Result<()> {
        // Use the high-level iter_tokens API for clean token iteration
        for token in events::iter_tokens(self.source, self.lang, None) {
            // Match on scope names to apply Markdown formatting
            match token.scope.as_ref() {
                scope if scope.contains("comment") => {
                    write!(output, "*{}*", token.text)?;
                }
                scope if scope.contains("keyword") => {
                    write!(output, "**{}**", token.text)?;
                }
                scope if scope.contains("string") => {
                    write!(output, "`{}`", token.text)?;
                }
                scope if scope.contains("function") => {
                    write!(output, "__{}__", token.text)?;
                }
                _ => write!(output, "{}", token.text)?,
            }
        }
        Ok(())
    }

    fn highlights(&self, output: &mut dyn Write) -> io::Result<()> {
        self.format(output)
    }
}

fn main() {
    let rust_code = r#"// Calculate factorial
fn factorial(n: u32) -> u32 {
    match n {
        0 => 1,
        _ => n * factorial(n - 1),
    }
}"#;

    println!("Input Rust code:");
    println!("{}\n", rust_code);

    let formatter = MarkdownFormatter {
        source: rust_code,
        lang: Language::Rust,
    };

    let result = highlight(
        rust_code,
        Options {
            lang_or_file: Some("rust"),
            formatter: Box::new(formatter),
        },
    );

    println!("Markdown-formatted output:");
    println!("{}", result);
}

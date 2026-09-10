//! `BBCode` formatter for syntax highlighting.
//!
//! This module re-exports the [`BBCodeScoped`] formatter that generates `BBCode` output with
//! highlight scope names as tags. A tag is the scope and the language it was matched
//! in, with the dots turned into hyphens: `keyword.function` in Rust becomes
//! `[keyword-function-rust]...[/keyword-function-rust]`.
//!
//! It does not emit standard forum-style `BBCode` such as `[b]`, `[color]`, or `[code]`.
//!
//! # Example Output
//!
//! For the Rust code `fn main() {}`, the formatter generates:
//!
//! ```text
//! [keyword-function-rust]fn[/keyword-function-rust] [function-rust]main[/function-rust][punctuation-bracket-rust]([/punctuation-bracket-rust][punctuation-bracket-rust])[/punctuation-bracket-rust] [punctuation-bracket-rust]{[/punctuation-bracket-rust][punctuation-bracket-rust]}[/punctuation-bracket-rust]
//! ```
//!
//! # Example
//!
//! ```rust
//! use lumis::{BBCodeScopedBuilder, languages::Language};
//!
//! let code = "fn main() { println!(\"Hello\"); }";
//!
//! let formatter = BBCodeScopedBuilder::new()
//!     .language(Language::Rust)
//!     .build()
//!     .unwrap();
//!
//! let mut output = Vec::new();
//! lumis::write_highlight(&mut output, code, formatter).unwrap();
//! let bbcode = String::from_utf8(output).unwrap();
//!
//! assert!(bbcode.starts_with("[keyword-function-rust]fn[/keyword-function-rust]"));
//! ```
//!
//! See the [formatter](crate::formatter) module for more information and examples.

pub use lumis_core::formatter::bbcode::{
    BBCodeScoped, BBCodeScopedBuilder, BBCodeScopedBuilderError,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::languages::Language;

    #[test]
    fn test_no_attrs() {
        let code = "@lang :rust";
        let formatter = BBCodeScoped::new(Language::Elixir);
        let result = crate::highlight(code, formatter);

        assert!(result.contains('@'));
        assert!(result.contains("lang"));
        assert!(result.contains(":rust"));
    }

    #[test]
    fn test_plain_text() {
        let code = "hello world";
        let formatter = BBCodeScoped::new(Language::PlainText);
        let result = crate::highlight(code, formatter);

        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_builder_pattern() {
        let formatter = BBCodeScopedBuilder::new()
            .language(Language::Rust)
            .build()
            .unwrap();

        let code = "fn main() {}";
        let result = crate::highlight(code, formatter);

        assert!(result.contains("fn"));
        assert!(result.contains("main"));
    }
}

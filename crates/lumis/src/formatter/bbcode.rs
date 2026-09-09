//! `BBCode` formatter for syntax highlighting.
//!
//! This module re-exports the [`BBCodeScoped`] formatter that generates `BBCode` output with
//! highlight scope names as tags (e.g., `[keyword-function]text[/keyword-function]`).
//! Dots in scope names are converted to hyphens (e.g., `keyword.function` becomes
//! `[keyword-function]...[/keyword-function]`).
//!
//! It does not emit standard forum-style `BBCode` such as `[b]`, `[color]`, or `[code]`.
//!
//! # Example Output
//!
//! For the Rust code `fn main() {}`, the formatter generates:
//!
//! ```text
//! [keyword-function]fn[/keyword-function] [function]main[/function][punctuation-bracket]([/punctuation-bracket][punctuation-bracket])[/punctuation-bracket] [punctuation-bracket]{[/punctuation-bracket][punctuation-bracket]}[/punctuation-bracket]
//! ```
//!
//! # Example
//!
//! ```rust,ignore
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

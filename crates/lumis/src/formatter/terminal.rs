//! Terminal formatter with ANSI color codes.
//!
//! This module re-exports the [`Terminal`] formatter that generates terminal output with
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
//! # Example
//!
//! ```rust
//! use lumis::{TerminalBuilder, languages::Language, themes};
//!
//! let code = "fn main() { println!(\"Hello\"); }";
//! let theme = themes::get("dracula").unwrap();
//!
//! let formatter = TerminalBuilder::new()
//!     .language(Language::Rust)
//!     .theme(Some(theme))
//!     .build()
//!     .unwrap();
//!
//! let mut output = Vec::new();
//! lumis::write_highlight(&mut output, code, formatter).unwrap();
//! println!("{}", String::from_utf8(output).unwrap());
//! ```
//!
//! See the [formatter](crate::formatter) module for more information and examples.

pub use lumis_core::formatter::terminal::{
    Background, Terminal, TerminalBuilder, TerminalBuilderError,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::languages::Language;

    #[test]
    fn test_no_attrs() {
        let code = "@lang :rust";
        let formatter = Terminal::new(Language::Elixir, None, Background::Inherit, None);
        let result = crate::highlight(code, formatter);

        assert!(result.contains('@'));
        assert!(result.contains("lang"));
        assert!(result.contains(":rust"));
        // Without a theme, some tokens may not have styling, so just check the text is there
    }
}

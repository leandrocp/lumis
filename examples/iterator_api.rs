//! Using the low-level iterator API
//!
//! This example demonstrates:
//! - Using highlight_iter for streaming/lazy highlighting
//! - Accessing position information for each token
//! - Processing tokens individually for custom output
//!
//! # Output
//!
//! ```text
//! Tokens with position information:
//!
//! 0..2: "fn" (fg: Some("#cf222e"), bold: false, italic: false)
//! 2..3: " " (fg: None, bold: false, italic: false)
//! 3..6: "add" (fg: Some("#6639ba"), bold: false, italic: false)
//! 6..7: "(" (fg: Some("#1f2328"), bold: false, italic: false)
//! 7..8: "a" (fg: Some("#1f2328"), bold: false, italic: false)
//! 8..9: ":" (fg: Some("#1f2328"), bold: false, italic: false)
//! 9..10: " " (fg: None, bold: false, italic: false)
//! 10..13: "i32" (fg: Some("#cf222e"), bold: false, italic: false)
//! 13..14: "," (fg: Some("#1f2328"), bold: false, italic: false)
//! 14..15: " " (fg: None, bold: false, italic: false)
//! 15..16: "b" (fg: Some("#1f2328"), bold: false, italic: false)
//! 16..17: ":" (fg: Some("#1f2328"), bold: false, italic: false)
//! 17..18: " " (fg: None, bold: false, italic: false)
//! 18..21: "i32" (fg: Some("#cf222e"), bold: false, italic: false)
//! 21..22: ")" (fg: Some("#1f2328"), bold: false, italic: false)
//! 22..23: " " (fg: None, bold: false, italic: false)
//! 23..25: "->" (fg: Some("#1f2328"), bold: false, italic: false)
//! 25..26: " " (fg: None, bold: false, italic: false)
//! 26..29: "i32" (fg: Some("#cf222e"), bold: false, italic: false)
//! 29..30: " " (fg: None, bold: false, italic: false)
//! 30..31: "{" (fg: Some("#1f2328"), bold: false, italic: false)
//! 31..36: "\n    " (fg: None, bold: false, italic: false)
//! 36..37: "a" (fg: Some("#1f2328"), bold: false, italic: false)
//! 37..38: " " (fg: None, bold: false, italic: false)
//! 38..39: "+" (fg: Some("#0550ae"), bold: false, italic: false)
//! 39..40: " " (fg: None, bold: false, italic: false)
//! 40..41: "b" (fg: Some("#1f2328"), bold: false, italic: false)
//! 41..42: "\n" (fg: None, bold: false, italic: false)
//! 42..43: "}" (fg: Some("#1f2328"), bold: false, italic: false)
//! ```

use autumnus::{highlight::highlight_iter, languages::Language, themes};

fn main() {
    let code = r#"fn add(a: i32, b: i32) -> i32 {
    a + b
}"#;

    let theme = themes::get("github_light").ok();
    let lang = Language::guess(Some("rust"), code);

    // Use the iterator API for streaming access
    let iter = highlight_iter(code, lang, theme).expect("Failed to create iterator");

    println!("Tokens with position information:\n");

    for (style, text, range) in iter {
        println!(
            "{}..{}: {:?} (fg: {:?}, bold: {}, italic: {})",
            range.start, range.end, text, style.fg, style.bold, style.italic
        );
    }
}

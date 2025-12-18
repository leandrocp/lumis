//! Using the highlight_iter function for streaming/lazy highlighting
//!
//! This example demonstrates:
//! - Using `highlight_iter()` for streaming/lazy highlighting
//! - Accessing position (byte range) and scope info for each token
//! - Processing tokens individually for custom output

use autumnus::{highlight::highlight_iter, languages::Language, themes};

fn main() {
    let code = r#"fn add(a: i32, b: i32) -> i32 {
    a + b
}"#;

    let theme = themes::get("github_light").ok();
    let lang = Language::guess(Some("rust"), code);

    let iter = highlight_iter(code, lang, theme).expect("Failed to create iterator");

    println!("Tokens with position information:\n");

    for (style, text, range, scope) in iter {
        println!(
            "{}..{}: {:?} (scope: {}, fg: {:?}, bold: {}, italic: {})",
            range.start, range.end, text, scope, style.fg, style.bold, style.italic
        );
    }
}

//! Using the highlight_iter function for streaming highlighting
//!
//! This example demonstrates:
//! - Using `highlight_iter()` with a callback for streaming highlighting
//! - Accessing position (byte range) and scope info for each token
//! - Processing tokens individually for custom output

use autumnus::{highlight::highlight_iter, languages::Language, themes};

fn main() {
    let code = r#"fn add(a: i32, b: i32) -> i32 {
    a + b
}"#;

    let theme = themes::get("github_light").ok();
    let lang = Language::guess(Some("rust"), code);

    println!("Tokens with position information:\n");

    highlight_iter(code, lang, theme, |text, range, scope, style| {
        println!(
            "{}..{}: {:?} (scope: {}, fg: {:?}, bold: {}, italic: {})",
            range.start, range.end, text, scope, style.fg, style.bold, style.italic
        );
        Ok::<_, std::io::Error>(())
    })
    .expect("Failed to highlight");
}

//! Auto-detecting language from file extension or source content
//!
//! This example demonstrates using `Language::guess()` to detect the
//! programming language from a file extension or source code content.

use autumnus::{highlight, languages::Language, HtmlInlineBuilder};

fn main() {
    let code = r#"#!/usr/bin/env python3
def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n-1) + fibonacci(n-2)

print(fibonacci(10))
"#;

    // Detect language from file path/extension or source content.
    // Pass the file path as the first argument (can be just filename).
    // The source code is used for shebang detection if no file extension matches.
    let lang = Language::guess(None, code);

    let formatter = HtmlInlineBuilder::new()
        .lang(lang)
        .build()
        .expect("Failed to build formatter");

    let html = highlight(code, formatter);

    println!("{}", html);
}

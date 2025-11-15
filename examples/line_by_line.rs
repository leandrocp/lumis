//! Processing code line by line with the Highlighter API
//!
//! This example demonstrates:
//! - Using the stateful Highlighter for repeated highlighting operations
//! - Processing source code without tree-sitter internals
//! - Accessing styled segments
//!
//! # Output
//!
//! ```text
//! Highlighted code with 79 segments:
//!
//! Color #cba6f7: "SELECT"
//! " "
//! Color #f9e2af: "users"
//! Color #9399b2: "."
//! Color #b4befe: "name"
//! Color #9399b2: ","
//! " "
//! Color #f9e2af: "COUNT"
//! Color #9399b2: "("
//! Color #f9e2af: "posts"
//! Color #9399b2: "."
//! Color #b4befe: "id"
//! Color #9399b2: ")"
//! " "
//! Color #cba6f7: "as"
//! " "
//! Color #cdd6f4: "post_count"
//! ...
//! ```

use autumnus::{highlight::Highlighter, languages::Language, themes};

fn main() {
    let code = r#"SELECT users.name, COUNT(posts.id) as post_count
FROM users
LEFT JOIN posts ON users.id = posts.user_id
WHERE users.active = true
GROUP BY users.id
HAVING COUNT(posts.id) > 5
ORDER BY post_count DESC;"#;

    let theme = themes::get("catppuccin_mocha").ok();

    let mut highlighter = Highlighter::new(Language::SQL, theme);

    let segments = highlighter
        .highlight(code)
        .expect("Failed to highlight code");

    println!("Highlighted code with {} segments:\n", segments.len());

    for (style, text) in segments {
        if let Some(fg) = &style.fg {
            print!("Color {}: ", fg);
        }
        if style.bold {
            print!("[BOLD] ");
        }
        if style.italic {
            print!("[ITALIC] ");
        }
        println!("{:?}", text);
    }
}

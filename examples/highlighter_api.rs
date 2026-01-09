//! Using the Highlighter struct for stateful highlighting
//!
//! This example demonstrates using `Highlighter` to get a vector of styled
//! segments `(Style, &str)` for custom processing.

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

    let highlighter = Highlighter::new(Language::SQL, theme);

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

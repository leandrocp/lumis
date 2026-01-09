//! Syntax highlighting with a custom theme
//!
//! This example demonstrates:
//! - Building a custom theme from scratch
//! - Defining custom colors for different syntax elements
//! - Using the custom theme with the highlighter

use autumnus::{
    highlight,
    languages::Language,
    themes::{Appearance, Style, Theme},
    HtmlInlineBuilder,
};
use std::collections::BTreeMap;

fn main() {
    let code = r#"fn main() {
    println!("Hello from Rust!");
    let numbers = vec![1, 2, 3, 4, 5];
    for n in numbers.iter() {
        println!("{}", n);
    }
}"#;

    // Build a custom theme with your own colors
    let mut highlights = BTreeMap::new();

    // Define colors for different syntax elements
    highlights.insert(
        "keyword".to_string(),
        Style {
            fg: Some("#ff6b9d".to_string()), // Pink for keywords
            bg: None,
            bold: true,
            italic: false,
            ..Default::default()
        },
    );

    highlights.insert(
        "function".to_string(),
        Style {
            fg: Some("#c792ea".to_string()), // Purple for functions
            bg: None,
            bold: false,
            italic: false,
            ..Default::default()
        },
    );

    highlights.insert(
        "string".to_string(),
        Style {
            fg: Some("#a5ff90".to_string()), // Green for strings
            bg: None,
            bold: false,
            italic: true,
            ..Default::default()
        },
    );

    highlights.insert(
        "number".to_string(),
        Style {
            fg: Some("#ffc799".to_string()), // Orange for numbers
            bg: None,
            bold: false,
            italic: false,
            ..Default::default()
        },
    );

    highlights.insert(
        "comment".to_string(),
        Style {
            fg: Some("#7f8c98".to_string()), // Gray for comments
            bg: None,
            bold: false,
            italic: true,
            ..Default::default()
        },
    );

    let custom_theme = Theme {
        name: "my_custom_theme".to_string(),
        appearance: Appearance::Dark,
        revision: "1.0".to_string(),
        highlights,
    };

    let lang = Language::guess(Some("rust"), code);

    let formatter = HtmlInlineBuilder::new()
        .lang(lang)
        .theme(Some(custom_theme))
        .pre_class(Some("my-code-block".to_string()))
        .italic(true) // Enable italic rendering for elements marked as italic
        .build()
        .expect("Failed to build formatter");

    let html = highlight(code, formatter);
    println!("{}", html);
}

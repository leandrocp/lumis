//! Syntax highlighting with a custom theme
//!
//! This example demonstrates:
//! - Building a custom theme from scratch
//! - Defining custom colors for different syntax elements
//! - Using the custom theme with the highlighter
//!
//! # Output
//!
//! ```html
//! <pre class="athl my-code-block"><code class="language-rust" translate="no" tabindex="0">
//! <div class="line" data-line="1"><span style="color: #ff6b9d; font-weight: bold;">fn</span> <span style="color: #c792ea;">main</span><span >(</span><span >)</span> <span >&lbrace;</span></div>
//! <div class="line" data-line="2">    <span style="color: #ff6b9d; font-weight: bold;">println</span><span style="color: #c792ea;">!</span><span >(</span><span style="color: #a5ff90; font-style: italic;">&quot;Hello from Rust!&quot;</span><span >)</span><span >;</span></div>
//! <div class="line" data-line="3">    <span style="color: #ff6b9d; font-weight: bold;">let</span> <span >numbers</span> <span >=</span> <span style="color: #ff6b9d; font-weight: bold;">vec</span><span style="color: #c792ea;">!</span><span >[</span><span style="color: #ffc799;">1</span><span >,</span> <span style="color: #ffc799;">2</span><span >,</span> <span style="color: #ffc799;">3</span><span >,</span> <span style="color: #ffc799;">4</span><span >,</span> <span style="color: #ffc799;">5</span><span >]</span><span >;</span></div>
//! <div class="line" data-line="4">    <span style="color: #ff6b9d; font-weight: bold;">for</span> <span >n</span> <span style="color: #ff6b9d; font-weight: bold;">in</span> <span >numbers</span><span >.</span><span style="color: #c792ea;">iter</span><span >(</span><span >)</span> <span >&lbrace;</span></div>
//! <div class="line" data-line="5">        <span style="color: #ff6b9d; font-weight: bold;">println</span><span style="color: #c792ea;">!</span><span >(</span><span style="color: #a5ff90; font-style: italic;">&quot;&lbrace;&rbrace;&quot;</span><span >,</span> <span >n</span><span >)</span><span >;</span></div>
//! <div class="line" data-line="6">    <span >&rbrace;</span></div>
//! <div class="line" data-line="7"><span >&rbrace;</span></div>
//! </code></pre>
//! ```

use autumnus::{
    highlight,
    languages::Language,
    themes::{Style, Theme},
    HtmlInlineBuilder, Options,
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
            underline: false,
            strikethrough: false,
        },
    );

    highlights.insert(
        "function".to_string(),
        Style {
            fg: Some("#c792ea".to_string()), // Purple for functions
            bg: None,
            bold: false,
            italic: false,
            underline: false,
            strikethrough: false,
        },
    );

    highlights.insert(
        "string".to_string(),
        Style {
            fg: Some("#a5ff90".to_string()), // Green for strings
            bg: None,
            bold: false,
            italic: true,
            underline: false,
            strikethrough: false,
        },
    );

    highlights.insert(
        "number".to_string(),
        Style {
            fg: Some("#ffc799".to_string()), // Orange for numbers
            bg: None,
            bold: false,
            italic: false,
            underline: false,
            strikethrough: false,
        },
    );

    highlights.insert(
        "comment".to_string(),
        Style {
            fg: Some("#7f8c98".to_string()), // Gray for comments
            bg: None,
            bold: false,
            italic: true,
            underline: false,
            strikethrough: false,
        },
    );

    let custom_theme = Theme {
        name: "my_custom_theme".to_string(),
        appearance: "dark".to_string(),
        revision: "1.0".to_string(),
        highlights,
    };

    let lang = Language::guess(Some("rust"), code);

    let formatter = HtmlInlineBuilder::new()
        .lang(lang)
        .theme(Some(custom_theme))
        .pre_class(Some("my-code-block"))
        .italic(true) // Enable italic rendering for elements marked as italic
        .build()
        .expect("Failed to build formatter");

    let options = Options {
        language: Some("rust"),
        formatter: Box::new(formatter),
    };

    let html = highlight(code, options);
    println!("{}", html);
}

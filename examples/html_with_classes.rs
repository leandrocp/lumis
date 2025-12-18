//! HTML output with CSS classes instead of inline styles
//!
//! This example demonstrates:
//! - Using HtmlLinkedBuilder for CSS class-based output
//! - Generating separate CSS that can be linked in your HTML
//! - Highlighting specific lines with custom classes

use autumnus::{highlight, languages::Language, themes, HtmlLinkedBuilder};

fn main() {
    let code = r#"<template>
  <div class="user-profile">
    <h1>{{ user.name }}</h1>
    <p>{{ user.email }}</p>
  </div>
</template>

<script>
export default {
  data() {
    return {
      user: { name: 'Alice', email: 'alice@example.com' }
    }
  }
}
</script>"#;

    let lang = Language::guess(Some("vue"), code);

    let formatter = HtmlLinkedBuilder::new()
        .lang(lang)
        .pre_class(Some("code-block".to_string()))
        .build()
        .expect("Failed to build formatter");

    let html = highlight(code, formatter);

    let theme = themes::get("github_light").expect("github_light theme should be available");
    let css = theme.css(true);

    println!("<!-- Include this CSS in your HTML -->");
    println!("<style>\n{}\n</style>\n", css);

    println!("<!-- And use this HTML markup -->");
    println!("{}", html);
}

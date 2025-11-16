//! HTML output with CSS classes instead of inline styles
//!
//! This example demonstrates:
//! - Using HtmlLinkedBuilder for CSS class-based output
//! - Generating separate CSS that can be linked in your HTML
//! - Highlighting specific lines with custom classes
//!
//! # Output
//!
//! First, the CSS is generated:
//!
//! ```css
//! /* github_light
//!  * revision: c106c9472154d6b2c74b74565616b877ae8ed31d
//!  */
//!
//! pre.athl {
//!   color: #1f2328;
//!   background-color: #ffffff;
//! }
//! .attribute {
//!   color: #0550ae;
//! }
//! .comment {
//!   color: #57606a;
//! }
//! .keyword {
//!   color: #cf222e;
//! }
//! ...
//! ```
//!
//! Then the HTML markup with CSS classes:
//!
//! ```html
//! <pre class="athl code-block"><code class="language-vue" translate="no" tabindex="0">
//! <div class="line" data-line="1"><span class="tag">&lt;template&gt;</span></div>
//! <div class="line" data-line="2">  <span class="tag">&lt;div</span> <span class="attribute">class</span><span class="tag">=</span><span class="string">&quot;user-profile&quot;</span><span class="tag">&gt;</span></div>
//! ...
//! </code></pre>
//! ```

use autumnus::{highlight, languages::Language, themes, HtmlLinkedBuilder, Options};

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
        .pre_class(Some("code-block"))
        .build()
        .expect("Failed to build formatter");

    let options = Options {
        language: Some("vue"),
        formatter: Box::new(formatter),
    };

    let html = highlight(code, options);

    let theme = themes::get("github_light").expect("github_light theme should be available");
    let css = theme.css(true);

    println!("<!-- Include this CSS in your HTML -->");
    println!("<style>\n{}\n</style>\n", css);

    println!("<!-- And use this HTML markup -->");
    println!("{}", html);
}

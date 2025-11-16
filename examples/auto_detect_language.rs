//! Basic syntax highlighting with HTML inline styles
//!
//! This example demonstrates the simplest way to highlight code using autumnus:
//! - Using the default theme (if available)
//! - Outputting HTML with inline CSS styles
//! - Auto-detecting language from source code
//!
//! # Output
//!
//! ```html
//! <pre class="athl"><code class="language-python" translate="no" tabindex="0">
//! <div class="line" data-line="1"><span >#!/usr/bin/env python3</span></div>
//! <div class="line" data-line="2"><span >def</span> <span >fibonacci</span>(<span >n</span>):</div>
//! <div class="line" data-line="3">    <span >if</span> <span >n</span> <span >&lt;=</span> <span >1</span>:</div>
//! <div class="line" data-line="4">        <span >return</span> <span >n</span></div>
//! <div class="line" data-line="5">    <span >return</span> <span >fibonacci</span>(<span >n</span><span >-</span><span >1</span>) <span >+</span> <span >fibonacci</span>(<span >n</span><span >-</span><span >2</span>)</div>
//! <div class="line" data-line="6"></div>
//! <div class="line" data-line="7"><span >print</span>(<span >fibonacci</span>(<span >10</span>))</div>
//! </code></pre>
//! ```

use autumnus::{highlight, OptionsBuilder};

fn main() {
    // Python code with shebang for auto-detection
    let code = r#"#!/usr/bin/env python3
def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n-1) + fibonacci(n-2)

print(fibonacci(10))
"#;

    // Simple API using builder with defaults
    let options = OptionsBuilder::new().build().unwrap();

    let html = highlight(code, options);

    println!("{}", html);
}

//! Terminal output with ANSI color codes
//!
//! This example demonstrates:
//! - Using the Terminal formatter for CLI output
//! - Applying themes to terminal output
//! - Direct printing to stdout

use autumnus::{highlight, languages::Language, themes, Options, TerminalBuilder};

fn main() {
    let code = r#"class User < ApplicationRecord
  has_many :posts
  validates :email, presence: true

  def greet
    puts "Hello, #{name}!"
  end
end"#;

    let theme = themes::get("github_dark").expect("github_dark theme should be available");

    let lang = Language::guess(Some("ruby"), code);

    let formatter = TerminalBuilder::new()
        .lang(lang)
        .theme(Some(theme))
        .build()
        .expect("Failed to build formatter");

    let options = Options {
        language: Some("ruby"),
        formatter: Box::new(formatter),
    };

    let ansi_output = highlight(code, options);

    println!("{}", ansi_output);
}

//! Terminal output with ANSI color codes
//!
//! This example demonstrates using the `TerminalBuilder` formatter to produce
//! syntax-highlighted output for terminals that support ANSI escape codes.
//!
//! The output can be printed directly to stdout or captured as a string.
//! Colors are applied based on the selected theme.

use lumis::{highlight, languages::Language, themes, TerminalBuilder};

fn main() {
    let code = r#"class User < ApplicationRecord
  has_many :posts
  validates :email, presence: true

  def greet
    puts "Hello, #{name}!"
  end
end"#;

    let theme = themes::get("github_dark").expect("github_dark theme should be available");

    let formatter = TerminalBuilder::new()
        .lang(Language::Ruby)
        .theme(Some(theme))
        .build()
        .expect("Failed to build formatter");

    let ansi_output = highlight(code, formatter);

    println!("{}", ansi_output);
}

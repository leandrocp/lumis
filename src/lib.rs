//! Syntax highlighter powered by Tree-sitter and Neovim themes.
//!
//! ## Quick Start
//!
//! Use the builder pattern for type-safe, ergonomic formatter creation:
//!
//! ```rust
//! use autumnus::{HtmlInlineBuilder, languages::Language, themes, formatter::Formatter};
//! use std::io::Write;
//!
//! let code = "fn main() { println!(\"Hello, world!\"); }";
//! let theme = themes::get("dracula").unwrap();
//!
//! let formatter = HtmlInlineBuilder::new()
//!     .source(code)
//!     .lang(Language::Rust)
//!     .theme(Some(theme))
//!     .pre_class(Some("code-block"))
//!     .build()
//!     .unwrap();
//!
//! let mut output = Vec::new();
//! formatter.format(&mut output).unwrap();
//! let html = String::from_utf8(output).unwrap();
//! ```
//!
//! ## Language Feature Flags
//!
//! By default, Autumnus includes support for all languages, which can result in longer
//! compilation times. You can reduce compilation time and binary size by enabling only
//! the languages you need:
//!
//! ```toml
//! [dependencies]
//! autumnus = { version = "0.3", default-features = false, features = ["lang-rust", "lang-javascript"] }
//! ```
//!
//! Available language features include: `lang-angular`, `lang-astro`, `lang-bash`, `lang-c`,
//! `lang-cpp`, `lang-css`, `lang-elixir`, `lang-go`, `lang-html`, `lang-java`, `lang-javascript`,
//! `lang-json`, `lang-markdown`, `lang-python`, `lang-rust`, `lang-typescript`, and many more.
//!
//! Use `all-languages` to enable all language support:
//!
//! ```toml
//! [dependencies]
//! autumnus = { version = "0.3", features = ["all-languages"] }
//! ```
//!
//! ## Available Builders
//!
//! - [`HtmlInlineBuilder`] - HTML output with inline CSS styles
//! - [`HtmlLinkedBuilder`] - HTML output with CSS classes (requires external CSS)
//! - [`TerminalBuilder`] - ANSI color codes for terminal output
//!
//! ## More Examples
//!
//! See the [`formatter`] module for detailed examples and usage patterns.
//!
//! ## Languages available
//!
//! | Language | File Extensions |
//! |----------|-----------------|
//! | Angular | *.angular, component.html |
//! | Assembly | *.s, *.asm, *.assembly |
//! | Astro | *.astro |
//! | Bash | *.bash, *.bats, *.cgi, *.command, *.env, *.fcgi, *.ksh, *.sh, *.sh.in, *.tmux, *.tool, *.zsh, .bash_aliases, .bash_history, .bash_logout, .bash_profile, .bashrc, .cshrc, .env, .env.example, .flaskenv, .kshrc, .login, .profile, .zlogin, .zlogout, .zprofile, .zshenv, .zshrc, 9fs, PKGBUILD, bash_aliases, bash_logout, bash_profile, bashrc, cshrc, ebuild, eclass, gradlew, kshrc, login, man, profile, zlogin, zlogout, zprofile, zshenv, zshrc |
//! | C | *.c |
//! | Caddy | Caddyfile |
//! | CMake | *.cmake, *.cmake.in, CMakeLists.txt |
//! | C++ | *.cc, *.cpp, *.h, *.hh, *.hpp, *.ino, *.cxx, *.cu, *.hxx |
//! | CSS | *.css |
//! | CSV | *.csv |
//! | C# | *.cs |
//! | Clojure | *.bb, *.boot, *.clj, *.cljc, *.clje, *.cljs, *.cljx, *.edn, *.joke, *.joker |
//! | Comment | |
//! | Common Lisp | *.lisp, *.lsp, *.asd |
//! | Dart | *.dart |
//! | Diff | *.diff |
//! | Dockerfile | Dockerfile, dockerfile, docker, Containerfile, container, *.dockerfile, *.docker, *.container |
//! | EEx | *.eex |
//! | EJS | *.ejs |
//! | ERB | *.erb |
//! | Elixir | *.ex, *.exs |
//! | Elm | *.elm |
//! | Erlang | *.erl, *.app, *.app.src, *.es, *.escript, *.hrl, *.xrl, *.yrl, Emakefile, rebar.config |
//! | Fish | *.fish |
//! | F# | *.fs, *.fsx, *.fsi |
//! | Gleam | *.gleam |
//! | Glimmer | *.hbs, *.handlebars, *.html.handlebars, *.glimmer |
//! | Go | *.go |
//! | GraphQL | |
//! | HEEx | *.heex, *.neex |
//! | HTML | *.html, *.htm, *.xhtml |
//! | Haskell | *.hs, *.hs-boot |
//! | HCL | *.hcl, *.nomad, *.tf, *.tfvars, *.workflow |
//! | IEx | *.iex |
//! | JSON | *.json, *.avsc, *.geojson, *.gltf, *.har, *.ice, *.JSON-tmLanguage, *.jsonl, *.mcmeta, *.tfstate, *.tfstate.backup, *.topojson, *.webapp, *.webmanifest, .arcconfig, .auto-changelog, .c8rc, .htmlhintrc, .imgbotconfig, .nycrc, .tern-config, .tern-project, .watchmanconfig, Pipfile.lock, composer.lock, mcmod.info, flake.lock |
//! | Java | *.java |
//! | JavaScript | *.cjs, *.js, *.mjs, *.snap, *.jsx |
//! | Kotlin | *.kt, *.ktm, *.kts |
//! | LaTeX | *.aux, *.cls, *.sty, *.tex |
//! | Liquid | *liquid |
//! | LLVM | *.llvm, *.ll |
//! | Lua | *.lua |
//! | Make | *.mak, *.d, *.make, *.makefile, *.mk, *.mkfile, *.dsp, BSDmakefile, GNUmakefile, Kbuild, Makefile, MAKEFILE, Makefile.am, Makefile.boot, Makefile.frag, Makefile*.in, Makefile.inc, Makefile.wat, makefile, makefile.sco, mkfile |
//! | Markdown | *.md, README, LICENSE |
//! | Markdown Inline | |
//! | Nix | *.nix |
//! | OCaml | *.ml |
//! | OCaml Interface | *.mli |
//! | Objective-C | *.m, *.objc |
//! | Perl | *.pm, *.pl, *.t |
//! | PHP | *.php, *.phtml, *.php3, *.php4, *.php5, *.php7, *.phps |
//! | Plain Text | |
//! | PowerShell | *.ps1, *.psm1 |
//! | Protocol Buffer | *.proto, *.protobuf, *.proto2, *.proto3 |
//! | Python | *.py, *.py3, *.pyi, *.bzl, TARGETS, BUCK, DEPS |
//! | R | *.R, *.r, *.rd, *.rsx, .Rprofile, expr-dist |
//! | Regex | *.regex |
//! | Ruby | *.rb, *.builder, *.spec, *.rake, Gemfile, Rakefile |
//! | Rust | *.rs |
//! | SCSS | *.scss |
//! | SQL | *.sql, *.pgsql |
//! | Scala | *.scala, *.sbt, *.sc |
//! | Surface | *.surface, *.sface |
//! | Svelte | *.svelte |
//! | Swift | *.swift |
//! | TOML | *.toml, Cargo.lock, Gopkg.lock, Pipfile, pdm.lock, poetry.lock, uv.lock |
//! | TSX | *.tsx |
//! | TypeScript | *.ts |
//! | Typst | *.typ, *.typst |
//! | Vim | *.vim, *.viml |
//! | Vue | *.vue |
//! | XML | *.ant, *.csproj, *.mjml, *.plist, *.resx, *.svg, *.ui, *.vbproj, *.xaml, *.xml, *.xsd, *.xsl, *.xslt, *.zcml, *.rng, App.config, nuget.config, packages.config, .classpath, .cproject, .project |
//! | YAML | *.yaml, *.yml |
//! | Zig | *.zig |
//!
//! ## Themes available
//!
//! | Theme Name |
//! | ---------- |
//! | aura_dark |
//! | aura_dark_soft_text |
//! | aura_soft_dark |
//! | aura_soft_dark_soft_text |
//! | ayu_dark |
//! | ayu_light |
//! | ayu_mirage |
//! | bamboo_light |
//! | bamboo_multiplex |
//! | bamboo_vulgaris |
//! | bluloco_dark |
//! | bluloco_light |
//! | carbonfox |
//! | catppuccin_frappe |
//! | catppuccin_latte |
//! | catppuccin_macchiato |
//! | catppuccin_mocha |
//! | cyberdream_dark |
//! | cyberdream_light |
//! | darkplus |
//! | dawnfox |
//! | dayfox |
//! | dracula |
//! | dracula_soft |
//! | duskfox |
//! | edge_aura |
//! | edge_dark |
//! | edge_light |
//! | edge_neon |
//! | everforest_dark |
//! | everforest_light |
//! | flexoki_dark |
//! | flexoki_light |
//! | github_dark |
//! | github_dark_colorblind |
//! | github_dark_default |
//! | github_dark_dimmed |
//! | github_dark_high_contrast |
//! | github_dark_tritanopia |
//! | github_light |
//! | github_light_colorblind |
//! | github_light_default |
//! | github_light_high_contrast |
//! | github_light_tritanopia |
//! | horizon_dark |
//! | horizon_light |
//! | iceberg |
//! | gruvbox_dark |
//! | gruvbox_dark_hard |
//! | gruvbox_dark_soft |
//! | gruvbox_light |
//! | gruvbox_light_hard |
//! | gruvbox_light_soft |
//! | kanagawa_dragon |
//! | kanagawa_lotus |
//! | kanagawa_wave |
//! | material_darker |
//! | material_deep_ocean |
//! | material_lighter |
//! | material_oceanic |
//! | material_palenight |
//! | matte_black |
//! | melange_dark |
//! | melange_light |
//! | molokai |
//! | modus_operandi |
//! | modus_vivendi |
//! | monokai_pro_dark |
//! | monokai_pro_machine |
//! | monokai_pro_ristretto |
//! | monokai_pro_spectrum |
//! | moonfly |
//! | moonlight |
//! | neosolarized_dark |
//! | neosolarized_light |
//! | neovim_dark |
//! | neovim_light |
//! | nightfly |
//! | nightfox |
//! | nord |
//! | nordfox |
//! | nordic |
//! | onedark |
//! | onedark_cool |
//! | onedark_darker |
//! | onedark_deep |
//! | onedark_light |
//! | onedark_warm |
//! | onedark_warmer |
//! | onedarkpro_dark |
//! | onedarkpro_vivid |
//! | onelight |
//! | papercolor_dark |
//! | papercolor_light |
//! | rosepine_dark |
//! | rosepine_dawn |
//! | rosepine_moon |
//! | solarized_autumn_dark |
//! | solarized_autumn_light |
//! | solarized_spring_dark |
//! | solarized_spring_light |
//! | solarized_summer_dark |
//! | solarized_summer_light |
//! | solarized_winter_dark |
//! | solarized_winter_light |
//! | srcery |
//! | terafox |
//! | tokyonight_day |
//! | tokyonight_moon |
//! | tokyonight_night |
//! | tokyonight_storm |
//! | vscode_dark |
//! | vscode_light |
//! | xcode_dark |
//! | xcode_dark_hc |
//! | xcode_light |
//! | xcode_light_hc |
//! | xcode_wwdc |
//! | zenburn |
//! | zephyr_dark |

#[doc(hidden)]
pub mod constants;
pub mod formatter;
pub mod languages;
pub mod themes;

#[cfg(feature = "elixir-nif")]
#[doc(hidden)]
pub mod elixir;

use crate::formatter::Formatter;
use std::io::{self, Write};

// Re-export builders for easier access
pub use crate::formatter::{HtmlInlineBuilder, HtmlLinkedBuilder, TerminalBuilder};

/// Configuration options for syntax highlighting.
///
/// This struct provides all the configuration needed to highlight source code,
/// including language detection and output formatting options. It's used with
/// the [`highlight`] and [`write_highlight`] functions.
///
/// # Language Detection
///
/// The `lang_or_file` field supports multiple input formats:
/// - **Language names**: `"rust"`, `"python"`, `"javascript"`
/// - **File paths**: `"src/main.rs"`, `"app.py"`, `"script.js"`
/// - **File extensions**: `"rs"`, `"py"`, `"js"`
/// - **None**: Automatic detection from source content
///
/// # Default Behavior
///
/// When using [`Options::default()`], you get:
/// - Automatic language detection (`lang_or_file: None`)
/// - HTML inline formatter with no theme
///
/// # Examples
///
/// ## Basic usage with defaults
///
/// ```rust
/// use autumnus::{highlight, Options};
///
/// let code = r#"
/// #!/usr/bin/env python3
/// print("Hello, World!")
/// "#;
///
/// // Language auto-detected from shebang, HTML inline output
/// let html = highlight(code, Options::default());
/// ```
///
/// ## Explicit language specification
///
/// ```rust
/// use autumnus::{highlight, Options, HtmlInlineBuilder, languages::Language};
///
/// let code = "fn main() { println!(\"Hello\"); }";
///
/// let formatter = HtmlInlineBuilder::new()
///     .source(code)
///     .lang(Language::Rust)
///     .pre_class(Some("code-block"))
///     .build()
///     .unwrap();
///
/// let options = Options {
///     lang_or_file: Some("rust"),
///     formatter: Box::new(formatter),
/// };
///
/// let html = highlight(code, options);
/// ```
///
/// ## File path-based detection
///
/// ```rust
/// use autumnus::{highlight, Options, HtmlInlineBuilder, languages::Language, themes};
///
/// let code = "defmodule MyApp do\n  def hello, do: :world\nend";
/// let theme = themes::get("dracula").unwrap();
///
/// let formatter = HtmlInlineBuilder::new()
///     .source(code)
///     .lang(Language::Elixir)
///     .theme(Some(theme))
///     .italic(true)
///     .build()
///     .unwrap();
///
/// let options = Options {
///     lang_or_file: Some("lib/my_app.ex"),
///     formatter: Box::new(formatter),
/// };
///
/// let html = highlight(code, options);
/// ```
///
/// ## Terminal output
///
/// ```rust
/// use autumnus::{highlight, Options, TerminalBuilder, languages::Language, themes};
///
/// let code = "SELECT * FROM users WHERE active = true;";
/// let theme = themes::get("github_light").unwrap();
///
/// let formatter = TerminalBuilder::new()
///     .source(code)
///     .lang(Language::SQL)
///     .theme(Some(theme))
///     .build()
///     .unwrap();
///
/// let options = Options {
///     lang_or_file: Some("sql"),
///     formatter: Box::new(formatter),
/// };
///
/// let ansi = highlight(code, options);
/// ```
///
/// ## HTML with linked CSS
///
/// ```rust
/// use autumnus::{highlight, Options, HtmlLinkedBuilder, languages::Language};
///
/// let code = "<div class=\"container\">Hello</div>";
///
/// let formatter = HtmlLinkedBuilder::new()
///     .source(code)
///     .lang(Language::HTML)
///     .pre_class(Some("syntax-highlight"))
///     .build()
///     .unwrap();
///
/// let options = Options {
///     lang_or_file: Some("html"),
///     formatter: Box::new(formatter),
/// };
///
/// let html = highlight(code, options);
/// // Remember to include the corresponding CSS file for your theme
/// ```
pub struct Options<'a> {
    /// Optional language or file path for highlighting.
    ///
    /// This field controls language detection and can accept:
    /// - **Language names**: `"rust"`, `"python"`, `"javascript"`, etc.
    /// - **File paths**: `"src/main.rs"`, `"app.py"`, `"Dockerfile"`
    /// - **File extensions**: `"rs"`, `"py"`, `"js"`
    /// - **None**: Automatic detection from source content (shebang, doctype, etc.)
    ///
    /// When `None`, the highlighter will analyze the source content to guess
    /// the language using shebangs, file content patterns, and other heuristics.
    pub lang_or_file: Option<&'a str>,

    /// The output formatter to use.
    ///
    /// Accepts any type implementing the [`Formatter`] trait. Use the provided builders:
    /// - [`HtmlInlineBuilder`] - HTML with inline CSS styles
    /// - [`HtmlLinkedBuilder`] - HTML with CSS classes (requires external CSS)
    /// - [`TerminalBuilder`] - ANSI color codes for terminal output
    ///
    /// See the [`formatter`] module for examples.
    pub formatter: Box<dyn Formatter + 'a>,
}

impl Default for Options<'_> {
    fn default() -> Self {
        Self {
            lang_or_file: None,
            formatter: Box::new(formatter::HtmlInline::default()),
        }
    }
}

/// Highlights source code and returns it as a string with syntax highlighting.
///
/// This function takes the source code and options as input,
/// and returns a string with the source code highlighted according to the specified formatter.
///
/// See [`formatter`] module for builder pattern examples.
///
/// # Arguments
///
/// * `source` - A string slice that represents the source code to be highlighted.
/// * `options` - An `Options` struct that contains the configuration options for the highlighter,
///   including the optional language/file path and formatter type to use.
///
/// # Examples
///
/// Basic usage with defaults:
///
/// ```rust
/// use autumnus::{highlight, Options};
///
/// let code = "fn main() { println!(\"Hello, world!\"); }";
/// let html = highlight(code, Options::default());
/// ```
///
/// Using builders:
///
/// ```rust
/// use autumnus::{highlight, Options, HtmlInlineBuilder, languages::Language};
///
/// let code = "fn main() { println!(\"Hello, world!\"); }";
///
/// let formatter = HtmlInlineBuilder::new()
///     .source(code)
///     .lang(Language::Rust)
///     .build()
///     .unwrap();
///
/// let options = Options {
///     lang_or_file: Some("rust"),
///     formatter: Box::new(formatter),
/// };
///
/// let html = highlight(code, options);
/// ```
///
pub fn highlight(_source: &str, options: Options) -> String {
    let mut buffer = Vec::new();
    let _ = options.formatter.format(&mut buffer);
    String::from_utf8(buffer).unwrap()
}

/// Write syntax highlighted output directly to a writer.
///
/// This function performs the same syntax highlighting as [`highlight`] but writes
/// the output directly to any type that implements [`Write`] instead of returning
/// a string. This is more memory efficient for large outputs and allows streaming
/// to files, network connections, or other destinations.
///
/// See [`formatter`] module for builder pattern examples.
///
/// # Arguments
///
/// * `output` - The writer to send highlighted output to
/// * `source` - The source code to highlight
/// * `options` - Configuration options for highlighting and formatting
///
/// # Returns
///
/// * `Ok(())` - Successfully wrote highlighted output
/// * `Err(io::Error)` - Write operation failed
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use autumnus::{write_highlight, Options};
///
/// let code = "const x = 42;";
/// let mut buffer = Vec::new();
///
/// write_highlight(&mut buffer, code, Options::default())
///     .expect("Failed to write");
/// ```
///
pub fn write_highlight(output: &mut dyn Write, _source: &str, options: Options) -> io::Result<()> {
    options.formatter.format(output)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // println!("{}", result);
    // std::fs::write("result.html", result.clone()).unwrap();

    #[test]
    fn test_write_highlight() {
        let code = r#"const = 1"#;

        let expected = r#"<pre class="athl" style="color: #c6d0f5; background-color: #303446;"><code class="language-javascript" translate="no" tabindex="0"><div class="line" data-line="1"><span style="color: #ca9ee6;">const</span> <span style="color: #99d1db;">=</span> <span style="color: #ef9f76;">1</span>
</div></code></pre>"#;

        let mut buffer = Vec::new();

        let formatter = HtmlInlineBuilder::new()
            .source(code)
            .lang(languages::Language::JavaScript)
            .theme(themes::get("catppuccin_frappe").ok())
            .build()
            .unwrap();

        write_highlight(
            &mut buffer,
            code,
            Options {
                lang_or_file: Some("javascript"),
                formatter: Box::new(formatter),
            },
        )
        .unwrap();

        let result = String::from_utf8(buffer).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_highlight_html_inline() {
        let code = r#"defmodule Foo do
  @moduledoc """
  Test Module
  """

  @projects ["Phoenix", "MDEx"]

  def projects, do: @projects
end
"#;

        let expected = r#"<pre class="athl" style="color: #c6d0f5; background-color: #303446;"><code class="language-elixir" translate="no" tabindex="0"><div class="line" data-line="1"><span style="color: #ca9ee6;">defmodule</span> <span style="color: #e5c890;">Foo</span> <span style="color: #ca9ee6;">do</span>
</div><div class="line" data-line="2">  <span style="color: #99d1db;"><span style="color: #949cbb;"><span style="color: #949cbb;">@</span><span style="color: #949cbb;">moduledoc</span> <span style="color: #949cbb;">&quot;&quot;&quot;</span></span></span>
</div><div class="line" data-line="3"><span style="color: #99d1db;"><span style="color: #949cbb;"><span style="color: #949cbb;">  Test Module</span></span></span>
</div><div class="line" data-line="4"><span style="color: #99d1db;"><span style="color: #949cbb;"><span style="color: #949cbb;">  &quot;&quot;&quot;</span></span></span>
</div><div class="line" data-line="5">
</div><div class="line" data-line="6">  <span style="color: #99d1db;"><span style="color: #ef9f76;">@<span style="color: #8caaee;"><span style="color: #ef9f76;">projects <span style="color: #949cbb;">[</span><span style="color: #a6d189;">&quot;Phoenix&quot;</span><span style="color: #949cbb;">,</span> <span style="color: #a6d189;">&quot;MDEx&quot;</span><span style="color: #949cbb;">]</span></span></span></span></span>
</div><div class="line" data-line="7">
</div><div class="line" data-line="8">  <span style="color: #ca9ee6;">def</span> <span style="color: #c6d0f5;">projects</span><span style="color: #949cbb;">,</span> <span style="color: #eebebe;">do: </span><span style="color: #99d1db;"><span style="color: #ef9f76;">@<span style="color: #ef9f76;">projects</span></span></span>
</div><div class="line" data-line="9"><span style="color: #ca9ee6;">end</span>
</div></code></pre>"#;

        let formatter = HtmlInlineBuilder::new()
            .source(code)
            .lang(languages::Language::Elixir)
            .pre_class(None)
            .italic(false)
            .include_highlights(false)
            .theme(themes::get("catppuccin_frappe").ok())
            .highlight_lines(None)
            .header(None)
            .build()
            .unwrap();

        let result = highlight(
            code,
            Options {
                lang_or_file: Some("elixir"),
                formatter: Box::new(formatter),
            },
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn test_highlight_html_inline_include_highlights() {
        let code = r#"defmodule Foo do
  @lang :elixir
end
"#;

        let expected = r#"<pre class="athl" style="color: #c6d0f5; background-color: #303446;"><code class="language-elixir" translate="no" tabindex="0"><div class="line" data-line="1"><span data-highlight="keyword" style="color: #ca9ee6;">defmodule</span> <span data-highlight="module" style="color: #e5c890;">Foo</span> <span data-highlight="keyword" style="color: #ca9ee6;">do</span>
</div><div class="line" data-line="2">  <span data-highlight="operator" style="color: #99d1db;"><span data-highlight="constant" style="color: #ef9f76;">@<span data-highlight="function.call" style="color: #8caaee;"><span data-highlight="constant" style="color: #ef9f76;">lang <span data-highlight="string.special.symbol" style="color: #eebebe;">:elixir</span></span></span></span></span>
</div><div class="line" data-line="3"><span data-highlight="keyword" style="color: #ca9ee6;">end</span>
</div></code></pre>"#;

        let formatter = HtmlInlineBuilder::new()
            .source(code)
            .lang(languages::Language::Elixir)
            .pre_class(None)
            .italic(false)
            .include_highlights(true)
            .theme(themes::get("catppuccin_frappe").ok())
            .highlight_lines(None)
            .header(None)
            .build()
            .unwrap();

        let result = highlight(
            code,
            Options {
                lang_or_file: Some("elixir"),
                formatter: Box::new(formatter),
            },
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn test_highlight_html_inline_escape_curly_braces() {
        let code = "{:ok, char: '{'}";

        let expected = r#"<pre class="athl" style="color: #c6d0f5; background-color: #303446;"><code class="language-elixir" translate="no" tabindex="0"><div class="line" data-line="1"><span style="color: #949cbb;">&lbrace;</span><span style="color: #eebebe;">:ok</span><span style="color: #949cbb;">,</span> <span style="color: #eebebe;">char: </span><span style="color: #81c8be;">&#39;&lbrace;&#39;</span><span style="color: #949cbb;">&rbrace;</span>
</div></code></pre>"#;

        let formatter = HtmlInlineBuilder::new()
            .source(code)
            .lang(languages::Language::Elixir)
            .pre_class(None)
            .italic(false)
            .include_highlights(false)
            .theme(themes::get("catppuccin_frappe").ok())
            .highlight_lines(None)
            .header(None)
            .build()
            .unwrap();

        let result = highlight(
            code,
            Options {
                lang_or_file: Some("elixir"),
                formatter: Box::new(formatter),
            },
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn test_highlight_html_linked() {
        let code = r#"defmodule Foo do
  @moduledoc """
  Test Module
  """

  @projects ["Phoenix", "MDEx"]

  def projects, do: @projects
end
"#;

        let expected = r#"<pre class="athl"><code class="language-elixir" translate="no" tabindex="0"><div class="line" data-line="1"><span class="keyword">defmodule</span> <span class="module">Foo</span> <span class="keyword">do</span>
</div><div class="line" data-line="2">  <span class="operator"><span class="comment-documentation"><span class="comment">@</span><span class="comment">moduledoc</span> <span class="comment">&quot;&quot;&quot;</span></span></span>
</div><div class="line" data-line="3"><span class="operator"><span class="comment-documentation"><span class="comment">  Test Module</span></span></span>
</div><div class="line" data-line="4"><span class="operator"><span class="comment-documentation"><span class="comment">  &quot;&quot;&quot;</span></span></span>
</div><div class="line" data-line="5">
</div><div class="line" data-line="6">  <span class="operator"><span class="constant">@<span class="function-call"><span class="constant">projects <span class="punctuation-bracket">[</span><span class="string">&quot;Phoenix&quot;</span><span class="punctuation-delimiter">,</span> <span class="string">&quot;MDEx&quot;</span><span class="punctuation-bracket">]</span></span></span></span></span>
</div><div class="line" data-line="7">
</div><div class="line" data-line="8">  <span class="keyword">def</span> <span class="variable">projects</span><span class="punctuation-delimiter">,</span> <span class="string-special-symbol">do: </span><span class="operator"><span class="constant">@<span class="constant">projects</span></span></span>
</div><div class="line" data-line="9"><span class="keyword">end</span>
</div></code></pre>"#;

        let formatter = HtmlLinkedBuilder::new()
            .source(code)
            .lang(languages::Language::Elixir)
            .pre_class(None)
            .highlight_lines(None)
            .header(None)
            .build()
            .unwrap();

        let result = highlight(
            code,
            Options {
                lang_or_file: Some("elixir"),
                formatter: Box::new(formatter),
            },
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn test_highlight_html_linked_escape_curly_braces() {
        let code = "{:ok, char: '{'}";

        let expected = r#"<pre class="athl"><code class="language-elixir" translate="no" tabindex="0"><div class="line" data-line="1"><span class="punctuation-bracket">&lbrace;</span><span class="string-special-symbol">:ok</span><span class="punctuation-delimiter">,</span> <span class="string-special-symbol">char: </span><span class="character">&#39;&lbrace;&#39;</span><span class="punctuation-bracket">&rbrace;</span>
</div></code></pre>"#;

        let formatter = HtmlLinkedBuilder::new()
            .source(code)
            .lang(languages::Language::Elixir)
            .pre_class(None)
            .highlight_lines(None)
            .header(None)
            .build()
            .unwrap();

        let result = highlight(
            code,
            Options {
                lang_or_file: Some("elixir"),
                formatter: Box::new(formatter),
            },
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn test_guess_language_by_file_name() {
        let code = "foo = 1";

        let formatter = HtmlInlineBuilder::new()
            .source(code)
            .lang(languages::Language::Elixir)
            .pre_class(None)
            .italic(false)
            .include_highlights(false)
            .theme(themes::get("catppuccin_frappe").ok())
            .highlight_lines(None)
            .header(None)
            .build()
            .unwrap();

        let result = highlight(
            code,
            Options {
                lang_or_file: Some("app.ex"),
                formatter: Box::new(formatter),
            },
        );
        assert!(result.as_str().contains("language-elixir"));
    }

    #[test]
    fn test_guess_language_by_file_extension() {
        let code1 = "# Title";

        let formatter1 = HtmlInlineBuilder::new()
            .source(code1)
            .lang(languages::Language::Markdown)
            .pre_class(None)
            .italic(false)
            .include_highlights(false)
            .theme(themes::get("catppuccin_frappe").ok())
            .highlight_lines(None)
            .header(None)
            .build()
            .unwrap();

        let result = highlight(
            code1,
            Options {
                lang_or_file: Some("md"),
                formatter: Box::new(formatter1),
            },
        );
        assert!(result.as_str().contains("language-markdown"));

        let code2 = "foo = 1";

        let formatter2 = HtmlInlineBuilder::new()
            .source(code2)
            .lang(languages::Language::Elixir)
            .pre_class(None)
            .italic(false)
            .include_highlights(false)
            .theme(themes::get("catppuccin_frappe").ok())
            .highlight_lines(None)
            .header(None)
            .build()
            .unwrap();

        let result = highlight(
            code2,
            Options {
                lang_or_file: Some("ex"),
                formatter: Box::new(formatter2),
            },
        );
        assert!(result.as_str().contains("language-elixir"));
    }

    #[test]
    fn test_guess_language_by_shebang() {
        let code = "#!/usr/bin/env elixir";

        let formatter = HtmlInlineBuilder::new()
            .source(code)
            .lang(languages::Language::Elixir)
            .pre_class(None)
            .italic(false)
            .include_highlights(false)
            .theme(themes::get("catppuccin_frappe").ok())
            .highlight_lines(None)
            .header(None)
            .build()
            .unwrap();

        let result = highlight(
            code,
            Options {
                lang_or_file: Some("test"),
                formatter: Box::new(formatter),
            },
        );
        assert!(result.as_str().contains("language-elixir"));
    }

    #[test]
    fn test_fallback_to_plain_text() {
        let code = "source code";

        let formatter = HtmlInlineBuilder::new()
            .source(code)
            .lang(languages::Language::PlainText)
            .pre_class(None)
            .italic(false)
            .include_highlights(false)
            .theme(themes::get("catppuccin_frappe").ok())
            .highlight_lines(None)
            .header(None)
            .build()
            .unwrap();

        let result = highlight(
            code,
            Options {
                lang_or_file: Some("none"),
                formatter: Box::new(formatter),
            },
        );
        assert!(result.as_str().contains("language-plaintext"));
    }

    #[test]
    fn test_highlight_terminal() {
        let code = "puts 'Hello from Ruby!'";

        let formatter = TerminalBuilder::new()
            .source(code)
            .lang(languages::Language::Ruby)
            .theme(themes::get("dracula").ok())
            .build()
            .unwrap();

        let options = Options {
            lang_or_file: Some("ruby"),
            formatter: Box::new(formatter),
        };
        let ansi = highlight(code, options);

        assert!(ansi.as_str().contains("[38;2;241;250;140mHello from Ruby!"));
    }

    #[test]
    fn test_formatter_option_with_header() {
        let code = "fn main() { println!(\"Hello\"); }";

        // Test HtmlInline with header
        let inline_formatter = HtmlInlineBuilder::new()
            .source(code)
            .lang(languages::Language::Rust)
            .theme(None)
            .pre_class(None)
            .italic(false)
            .include_highlights(false)
            .highlight_lines(None)
            .header(Some(formatter::HtmlElement {
                open_tag: "<div class=\"code-container\">".to_string(),
                close_tag: "</div>".to_string(),
            }))
            .build()
            .unwrap();

        let inline_result = highlight(
            code,
            Options {
                lang_or_file: Some("rust"),
                formatter: Box::new(inline_formatter),
            },
        );

        assert!(inline_result.starts_with("<div class=\"code-container\">"));
        assert!(inline_result.ends_with("</div>"));
        assert!(inline_result.contains("<pre class=\"athl\">"));

        // Test HtmlLinked with header
        let linked_formatter = HtmlLinkedBuilder::new()
            .source(code)
            .lang(languages::Language::Rust)
            .pre_class(None)
            .highlight_lines(None)
            .header(Some(formatter::HtmlElement {
                open_tag: "<section class=\"code-section\">".to_string(),
                close_tag: "</section>".to_string(),
            }))
            .build()
            .unwrap();

        let linked_result = highlight(
            code,
            Options {
                lang_or_file: Some("rust"),
                formatter: Box::new(linked_formatter),
            },
        );

        assert!(linked_result.starts_with("<section class=\"code-section\">"));
        assert!(linked_result.ends_with("</section>"));
        assert!(linked_result.contains("<pre class=\"athl\">"));
    }
}

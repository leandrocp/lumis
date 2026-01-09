//! Syntax highlighter powered by Tree-sitter and Neovim themes.
//!
//! ## Quick Start
//!
//! Highlight code in three steps: pick a formatter, configure it, format your code.
//!
//! ```rust
//! use autumnus::{HtmlInlineBuilder, languages::Language, themes, formatter::Formatter};
//!
//! let code = "fn main() { println!(\"Hello, world!\"); }";
//! let theme = themes::get("dracula").unwrap();
//!
//! let formatter = HtmlInlineBuilder::new()
//!     .lang(Language::Rust)
//!     .theme(Some(theme))
//!     .build()
//!     .unwrap();
//!
//! let mut output = Vec::new();
//! formatter.format(code, &mut output).unwrap();
//! let html = String::from_utf8(output).unwrap();
//! ```
//!
//! ### Alternative: Using `highlight()` and `write_highlight()`
//!
//! ```rust
//! use autumnus::{highlight, HtmlInlineBuilder, languages::Language, themes};
//!
//! let code = "print('Hello')";
//! let theme = themes::get("dracula").unwrap();
//!
//! let formatter = HtmlInlineBuilder::new()
//!     .lang(Language::Python)
//!     .theme(Some(theme))
//!     .build()
//!     .unwrap();
//!
//! let html = highlight(code, formatter);
//! ```
//!
//! For large outputs, use `write_highlight()` to stream directly to a writer:
//!
//! ```rust,no_run
//! use autumnus::{write_highlight, TerminalBuilder, languages::Language};
//! use std::fs::File;
//!
//! # let code = "x = 1";
//! # let formatter = TerminalBuilder::new().lang(Language::Python).build().unwrap();
//! let mut file = File::create("output.txt").unwrap();
//!
//! write_highlight(&mut file, code, formatter).unwrap();
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
//! autumnus = { version = "0.7", default-features = false, features = ["lang-rust", "lang-javascript"] }
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
//! autumnus = { version = "0.7", features = ["all-languages"] }
//! ```
//!
//! ## Formatters
//!
//! | Formatter | Output | Use When |
//! |-----------|--------|----------|
//! | [`HtmlInlineBuilder`] | HTML with inline styles | Need standalone HTML, email, no external CSS |
//! | [`HtmlMultiThemesBuilder`] | HTML (inline) with multiple themes | Support light/dark mode, theme switching |
//! | [`HtmlLinkedBuilder`] | HTML with CSS classes | Multiple code blocks, custom styling |
//! | [`TerminalBuilder`] | ANSI escape codes | CLI tools, terminal output |
//!
//! See the [`formatter`] module for advanced features like line highlighting and custom formatters.
//!
//! ## Themes
//!
//! 120+ themes from popular Neovim colorschemes. Use with HTML inline and terminal formatters.
//!
//! ```rust
//! use autumnus::themes;
//!
//! // Get a theme by name
//! let theme = themes::get("dracula").unwrap();
//!
//! // Or parse from string
//! let theme: themes::Theme = "catppuccin_mocha".parse().unwrap();
//! ```
//!
//! See the [`themes`] module for loading custom themes from JSON files.
//! Available themes are listed below.
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

pub mod constants;
pub mod formatter;
pub mod highlight;
pub mod languages;
pub mod themes;
pub mod vendor;

// Re-export helper modules from formatter for convenience
pub use formatter::ansi;
pub use formatter::html;

#[cfg(feature = "elixir-nif")]
#[doc(hidden)]
pub mod elixir;

use crate::formatter::Formatter;
use std::io::{self, Write};

// Re-export builders for easier access
pub use crate::formatter::{
    HtmlInlineBuilder, HtmlLinkedBuilder, HtmlMultiThemesBuilder, TerminalBuilder,
};

/// Highlights source code and returns it as a string.
///
/// This is a convenience wrapper that calls the formatter and returns the result as a String.
/// For streaming to files or other writers, use [`write_highlight()`] instead.
///
/// # Arguments
///
/// * `source` - The source code to highlight.
/// * `formatter` - A configured formatter (e.g., from [`HtmlInlineBuilder`], [`TerminalBuilder`]).
///
/// # Panics
///
/// Panics if the formatter fails to format the source code or produces invalid UTF-8 output.
/// For fallible formatting, use [`write_highlight()`] instead.
///
/// # Examples
///
/// ```rust
/// use autumnus::{highlight, HtmlInlineBuilder, languages::Language};
///
/// let code = r#"fn main() { println!("Hello!"); }"#;
///
/// let formatter = HtmlInlineBuilder::new()
///     .lang(Language::Rust)
///     .build()
///     .unwrap();
///
/// let html = highlight(code, formatter);
/// ```
pub fn highlight<F: Formatter>(source: &str, formatter: F) -> String {
    let mut buffer = Vec::new();
    formatter
        .format(source, &mut buffer)
        .expect("formatter failed to format source code");
    String::from_utf8(buffer).expect("formatter produced invalid UTF-8")
}

/// Write syntax highlighted output directly to a writer.
///
/// This function writes highlighted output directly to any [`Write`] implementation,
/// which is more memory efficient for large outputs than [`highlight()`].
///
/// # Arguments
///
/// * `output` - The writer to send highlighted output to.
/// * `source` - The source code to highlight.
/// * `formatter` - A configured formatter.
///
/// # Examples
///
/// ```rust,no_run
/// use autumnus::{write_highlight, HtmlInlineBuilder, languages::Language};
/// use std::fs::File;
///
/// let code = "fn main() { }";
/// let formatter = HtmlInlineBuilder::new()
///     .lang(Language::Rust)
///     .build()
///     .unwrap();
///
/// let mut file = File::create("output.html")?;
/// write_highlight(&mut file, code, formatter)?;
/// # Ok::<(), std::io::Error>(())
/// ```
pub fn write_highlight<F: Formatter>(
    output: &mut dyn Write,
    source: &str,
    formatter: F,
) -> io::Result<()> {
    formatter.format(source, output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::languages::Language;

    // println!("{}", result);
    // std::fs::write("result.html", result.clone()).unwrap();

    #[test]
    fn test_write_highlight() {
        let code = r#"const = 1"#;

        let expected = r#"<pre class="athl" style="color: #c6d0f5; background-color: #303446;"><code class="language-javascript" translate="no" tabindex="0"><div class="line" data-line="1"><span style="color: #ca9ee6;">const</span> <span style="color: #99d1db;">=</span> <span style="color: #ef9f76;">1</span>
</div></code></pre>"#;

        let mut buffer = Vec::new();

        let formatter = HtmlInlineBuilder::default()
            .lang(Language::JavaScript)
            .theme(themes::get("catppuccin_frappe").ok())
            .build()
            .unwrap();

        write_highlight(&mut buffer, code, formatter).unwrap();

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

        let formatter = HtmlInlineBuilder::default()
            .lang(Language::Elixir)
            .theme(themes::get("catppuccin_frappe").ok())
            .build()
            .unwrap();

        let result = highlight(code, formatter);

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

        let formatter = HtmlInlineBuilder::default()
            .lang(Language::Elixir)
            .include_highlights(true)
            .theme(themes::get("catppuccin_frappe").ok())
            .build()
            .unwrap();

        let result = highlight(code, formatter);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_highlight_html_inline_escape_curly_braces() {
        let code = "{:ok, char: '{'}";
        let expected = r#"<pre class="athl" style="color: #c6d0f5; background-color: #303446;"><code class="language-elixir" translate="no" tabindex="0"><div class="line" data-line="1"><span style="color: #949cbb;">&lbrace;</span><span style="color: #eebebe;">:ok</span><span style="color: #949cbb;">,</span> <span style="color: #eebebe;">char: </span><span style="color: #81c8be;">&#39;&lbrace;&#39;</span><span style="color: #949cbb;">&rbrace;</span>
</div></code></pre>"#;

        let formatter = HtmlInlineBuilder::default()
            .lang(Language::Elixir)
            .theme(themes::get("catppuccin_frappe").ok())
            .build()
            .unwrap();

        let result = highlight(code, formatter);

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

        let formatter = HtmlLinkedBuilder::default()
            .lang(Language::Elixir)
            .build()
            .unwrap();

        let result = highlight(code, formatter);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_highlight_html_linked_escape_curly_braces() {
        let code = "{:ok, char: '{'}";
        let expected = r#"<pre class="athl"><code class="language-elixir" translate="no" tabindex="0"><div class="line" data-line="1"><span class="punctuation-bracket">&lbrace;</span><span class="string-special-symbol">:ok</span><span class="punctuation-delimiter">,</span> <span class="string-special-symbol">char: </span><span class="character">&#39;&lbrace;&#39;</span><span class="punctuation-bracket">&rbrace;</span>
</div></code></pre>"#;

        let formatter = HtmlLinkedBuilder::default()
            .lang(Language::Elixir)
            .build()
            .unwrap();

        let result = highlight(code, formatter);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_guess_language_by_file_name() {
        let code = "foo = 1";
        let formatter = HtmlInlineBuilder::default()
            .lang(Language::Elixir)
            .theme(themes::get("catppuccin_frappe").ok())
            .build()
            .unwrap();

        let result = highlight(code, formatter);
        assert!(result.contains("language-elixir"));
    }

    #[test]
    fn test_guess_language_by_file_extension() {
        let code1 = "# Title";
        let formatter1 = HtmlInlineBuilder::default()
            .lang(Language::Markdown)
            .theme(themes::get("catppuccin_frappe").ok())
            .build()
            .unwrap();

        let result = highlight(code1, formatter1);
        assert!(result.contains("language-markdown"));

        let code2 = "foo = 1";
        let formatter2 = HtmlInlineBuilder::default()
            .lang(Language::Elixir)
            .theme(themes::get("catppuccin_frappe").ok())
            .build()
            .unwrap();

        let result = highlight(code2, formatter2);
        assert!(result.contains("language-elixir"));
    }

    #[test]
    fn test_guess_language_by_shebang() {
        let code = "#!/usr/bin/env elixir";
        let formatter = HtmlInlineBuilder::default()
            .lang(Language::Elixir)
            .theme(themes::get("catppuccin_frappe").ok())
            .build()
            .unwrap();

        let result = highlight(code, formatter);
        assert!(result.contains("language-elixir"));
    }

    #[test]
    fn test_fallback_to_plain_text() {
        let code = "source code";
        let formatter = HtmlInlineBuilder::default()
            .lang(Language::PlainText)
            .theme(themes::get("catppuccin_frappe").ok())
            .build()
            .unwrap();

        let result = highlight(code, formatter);
        assert!(result.contains("language-plaintext"));
    }

    #[test]
    fn test_highlight_terminal() {
        let code = "puts 'Hello from Ruby!'";
        let formatter = TerminalBuilder::default()
            .lang(Language::Ruby)
            .theme(themes::get("dracula").ok())
            .build()
            .unwrap();

        let ansi = highlight(code, formatter);

        assert!(ansi.contains("[38;2;241;250;140mHello from Ruby!"));
    }

    #[test]
    fn test_formatter_option_with_header() {
        let code = "fn main() { println!(\"Hello\"); }";

        // Test HtmlInline with header
        let inline_formatter = HtmlInlineBuilder::default()
            .lang(Language::Rust)
            .header(Some(formatter::HtmlElement {
                open_tag: "<div class=\"code-container\">".to_string(),
                close_tag: "</div>".to_string(),
            }))
            .build()
            .unwrap();

        let inline_result = highlight(code, inline_formatter);

        assert!(inline_result.starts_with("<div class=\"code-container\">"));
        assert!(inline_result.ends_with("</div>"));
        assert!(inline_result.contains("<pre class=\"athl\">"));

        // Test HtmlLinked with header
        let linked_formatter = HtmlLinkedBuilder::default()
            .lang(Language::Rust)
            .header(Some(formatter::HtmlElement {
                open_tag: "<section class=\"code-section\">".to_string(),
                close_tag: "</section>".to_string(),
            }))
            .build()
            .unwrap();

        let linked_result = highlight(code, linked_formatter);

        assert!(linked_result.starts_with("<section class=\"code-section\">"));
        assert!(linked_result.ends_with("</section>"));
        assert!(linked_result.contains("<pre class=\"athl\">"));
    }
}

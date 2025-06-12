//! Syntax highlighter powered by Tree-sitter and Neovim themes.
//!
//! ## Examples
//!
//! Basic usage with default options (HTML inline styles):
//!
//! ```rust
//! use autumnus::{highlight, Options};
//!
//! let code = r#"
//!     function greet(name) {
//!         console.log(`Hello ${name}!`);
//!     }
//! "#;
//!
//! let html = highlight(code, Options {
//!         lang_or_file: Some("javascript"),
//!         ..Options::default()
//!     }
//! );
//! ```
//!
//! Using a specific theme:
//!
//! ```rust
//! use autumnus::{highlight, Options, themes, FormatterOption};
//!
//! let code = "SELECT * FROM users WHERE active = true;";
//! let html = highlight(
//!     code,
//!     Options {
//!         lang_or_file: Some("sql"),
//!         formatter: FormatterOption::HtmlInline {
//!             theme: themes::get("dracula").ok(),
//!             pre_class: None,
//!             italic: false,
//!             include_highlights: false,
//!             highlight_lines: None,
//!         },
//!     }
//! );
//! ```
//!
//! Highlighting with file path detection:
//!
//! ```rust
//! use autumnus::{highlight, Options};
//!
//! let code = r#"
//!     defmodule MyApp do
//!       def hello, do: :world
//!     end
//! "#;
//! // Language will be automatically detected as Elixir from the .ex extension
//! let html = highlight(
//!     code,
//!     Options {
//!         lang_or_file: Some("app.ex"),
//!         ..Options::default()
//!     }
//! );
//! ```
//!
//! Guess language by source content:
//!
//! ```rust
//! use autumnus::{highlight, Options};
//!
//! let code = r#"
//!     #!/usr/bin/env bash
//!
//!     echo "Hello, world!"
//! "#;
//! // Language will be automatically detected as Bash from the shebang line
//! let html = highlight(code, Options::default());
//! ```
//!
//! Terminal output with ANSI colors:
//!
//! ```rust
//! use autumnus::{highlight, Options, themes, FormatterOption};
//!
//! let code = "puts 'Hello from Ruby!'";
//! let ansi = highlight(
//!     code,
//!     Options {
//!         lang_or_file: Some("ruby"),
//!         formatter: FormatterOption::Terminal {
//!             theme: themes::get("github_light").ok(),
//!         },
//!     }
//! );
//! ```
//!
//! ## Languages available
//!
//! | Language | File Extensions |
//! |----------|-----------------|
//! | Angular | *.angular, component.html |
//! | Astro | *.astro |
//! | Bash | *.bash, *.bats, *.cgi, *.command, *.env, *.fcgi, *.ksh, *.sh, *.sh.in, *.tmux, *.tool, *.zsh, .bash_aliases, .bash_history, .bash_logout, .bash_profile, .bashrc, .cshrc, .env, .env.example, .flaskenv, .kshrc, .login, .profile, .zlogin, .zlogout, .zprofile, .zshenv, .zshrc, 9fs, PKGBUILD, bash_aliases, bash_logout, bash_profile, bashrc, cshrc, ebuild, eclass, gradlew, kshrc, login, man, profile, zlogin, zlogout, zprofile, zshenv, zshrc |
//! | C | *.c |
//! | CMake | *.cmake, *.cmake.in, CMakeLists.txt |
//! | C++ | *.cc, *.cpp, *.h, *.hh, *.hpp, *.ino, *.cxx, *.cu, *.hxx |
//! | CSS | *.css |
//! | CSV | *.csv |
//! | C# | *.cs |
//! | Clojure | *.bb, *.boot, *.clj, *.cljc, *.clje, *.cljs, *.cljx, *.edn, *.joke, *.joker |
//! | Comment | |
//! | Common Lisp | *.lisp, *.lsp, *.asd |
//! | Diff | *.diff |
//! | Dockerfile | Dockerfile, dockerfile, docker, Containerfile, container, *.dockerfile, *.docker, *.container |
//! | EEx | *.eex |
//! | EJS | *.ejs |
//! | ERB | *.erb |
//! | Elixir | *.ex, *.exs |
//! | Elm | *.elm |
//! | Erlang | *.erl, *.app, *.app.src, *.es, *.escript, *.hrl, *.xrl, *.yrl, Emakefile, rebar.config |
//! | F# | *.fs, *.fsx, *.fsi |
//! | Gleam | *.gleam |
//! | Glimmer | *.hbs, *.handlebars, *.html.handlebars, *.glimmer |
//! | Go | *.go |
//! | GraphQL | |
//! | HEEx | *.heex, *.neex |
//! | HTML | *.html, *.htm, *.xhtml |
//! | Haskell | *.hs, *.hs-boot |
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
//! | Vim | *.vim, *.viml |
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
//! | melange_dark |
//! | melange_light |
//! | modus_operandi |
//! | modus_vivendi |
//! | monokai_pro_dark |
//! | monokai_pro_machine |
//! | monokai_pro_ristretto |
//! | monokai_pro_spectrum |
//! | moonfly |
//! | neosolarized_dark |
//! | neosolarized_light |
//! | neovim_dark |
//! | neovim_light |
//! | nightfly |
//! | nightfox |
//! | nord |
//! | nordic |
//! | onedark |
//! | onedark_darker |
//! | onedarkpro_dark |
//! | onedarkpro_vivid |
//! | onelight |
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
//! | zephyr_dark |

#[doc(hidden)]
pub mod constants;
pub mod formatter;
pub mod languages;
pub mod themes;

#[cfg(feature = "elixir")]
#[doc(hidden)]
pub mod elixir;

use crate::formatter::FormatterBuilder;
use crate::languages::Language;
use crate::themes::Theme;
use std::io::{self, Write};

/// The type of formatter to use for syntax highlighting.
///
/// Defaults to `HtmlInline` with no additional `pre_class`, no italics, and no highlight scope names.
#[derive(Clone, Debug)]
pub enum FormatterOption<'a> {
    /// HTML output with inline styles.
    HtmlInline {
        /// Theme to use for highlighting.
        theme: Option<&'a Theme>,
        /// Class to add to the `<pre>` tag.
        pre_class: Option<&'a str>,
        /// Whether to use italics for highlighting.
        italic: bool,
        /// Whether to include the original highlight scope name in a `data` attribute.
        /// Useful for debugging.
        include_highlights: bool,
        highlight_lines: Option<formatter::html_inline::HighlightLines>,
    },
    /// HTML output with linked styles.
    ///
    /// When using this formatter, CSS files for all themes are available in the `css/` directory.
    /// You need to include the corresponding CSS file for your chosen theme:
    ///
    /// ```html
    /// <link rel="stylesheet" href="css/dracula.css">
    /// ```
    HtmlLinked {
        /// Class to add to the `<pre>` tag.
        pre_class: Option<&'a str>,
        highlight_lines: Option<formatter::html_linkded::HighlightLines>,
    },
    /// Terminal output with ANSI colors.
    Terminal {
        /// Theme to use for highlighting.
        theme: Option<&'a Theme>,
    },
}

impl Default for FormatterOption<'_> {
    fn default() -> Self {
        Self::HtmlInline {
            theme: None,
            pre_class: None,
            italic: false,
            include_highlights: false,
            highlight_lines: None,
        }
    }
}

/// Options for the highlighter.
#[derive(Debug)]
pub struct Options<'a> {
    /// Optional language or file path to use for highlighting.
    /// If not provided, the language will be guessed based on the source content.
    ///
    /// # Examples
    ///
    /// ```
    /// use autumnus::{Options, highlight, FormatterOption};
    ///
    /// let options = Options {
    ///     lang_or_file: Some("rust"),
    ///     formatter: FormatterOption::HtmlInline {
    ///         pre_class: None,
    ///         italic: false,
    ///         include_highlights: false,
    ///         theme: None,
    ///         highlight_lines: None,
    ///     },
    /// };
    ///
    /// let code = r#"fn main() { println!("Hello"); }"#;
    /// let html = highlight(code, options);
    /// ```
    pub lang_or_file: Option<&'a str>,

    /// The type of formatter to use for output.
    ///
    /// # Examples
    ///
    /// ```
    /// use autumnus::{Options, FormatterOption, highlight};
    ///
    /// let code = "puts 'Hello from Ruby!'";
    /// let ansi = highlight(
    ///     code,
    ///     Options {
    ///         lang_or_file: Some("ruby"),
    ///         formatter: FormatterOption::Terminal {
    ///             theme: None,
    ///         },
    ///     }
    /// );
    /// ```
    pub formatter: FormatterOption<'a>,
}

impl Default for Options<'_> {
    fn default() -> Self {
        Self {
            lang_or_file: None,
            formatter: FormatterOption::HtmlInline {
                pre_class: None,
                italic: false,
                include_highlights: false,
                theme: None,
                highlight_lines: None,
            },
        }
    }
}

/// Highlights source code and returns it as a string with syntax highlighting.
///
/// This function takes the source code and options as input,
/// and returns a string with the source code highlighted according to the specified formatter.
///
/// # Arguments
///
/// * `source` - A string slice that represents the source code to be highlighted.
/// * `options` - An `Options` struct that contains the configuration options for the highlighter,
///   including the optional language/file path and formatter type to use.
///
/// # Examples
///
/// Basic usage with HTML inline styles (default):
///
/// ```rust
/// use autumnus::highlight;
/// use autumnus::Options;
/// use autumnus::FormatterOption;
///
/// let code = r#"
/// fn main() {
///     println!("Hello, world!");
/// }
/// "#;
///
/// let html = highlight(
///     code,
///     Options {
///         lang_or_file: Some("rust"),
///         formatter: FormatterOption::HtmlInline {
///             pre_class: None,
///             italic: false,
///             include_highlights: false,
///             theme: None,
///             highlight_lines: None,
///         },
///     }
/// );
/// ```
///
/// Output with HTML inline styles (default):
/// ```html
/// <pre class="athl" style="color: #c6d0f5; background-color: #303446;">
///   <code class="language-rust" translate="no" tabindex="0">
///     <span class="line" data-line="1">
///       <span style="color: #ca9ee6;">fn</span> <span style="color: #8caaee;">main</span>() {
///     </span>
///     <span class="line" data-line="2">
///       <span style="color: #8caaee;">println!</span>(<span style="color: #a6d189;">"Hello, world!"</span>);
///     </span>
///     <span class="line" data-line="3">}</span>
///   </code>
/// </pre>
/// ```
///
/// Using HTML with linked styles:
///
/// ```rust
/// use autumnus::highlight;
/// use autumnus::Options;
/// use autumnus::FormatterOption;
///
/// let code = r#"
/// fn main() {
///     println!("Hello, world!");
/// }
/// "#;
///
/// let html = highlight(
///     code,
///     Options {
///         lang_or_file: Some("rust"),
///         formatter: FormatterOption::HtmlLinked {
///             pre_class: Some("my-code-block"),
///             highlight_lines: None,
///         },
///     }
/// );
/// ```
///
/// Output with HTML linked styles:
/// ```html
/// <pre class="athl my-code-block">
///   <code class="language-rust" translate="no" tabindex="0">
///     <span class="line" data-line="1">
///       <span class="keyword-function">fn</span> <span class="function">main</span>() {
///     </span>
///     <span class="line" data-line="2">
///       <span class="function-macro">println!</span>(<span class="string">"Hello, world!"</span>);
///     </span>
///     <span class="line" data-line="3">}</span>
///   </code>
/// </pre>
/// ```
///
/// When using `FormatterOption::HtmlLinked`, you need to include the corresponding CSS file for your chosen theme.
/// CSS files for all themes are available in the `css/` directory:
///
/// ```html
/// <link rel="stylesheet" href="css/dracula.css">
/// ```
///
/// Using terminal output:
///
/// ```rust
/// use autumnus::highlight;
/// use autumnus::Options;
/// use autumnus::FormatterOption;
///
/// let code = r#"
/// fn main() {
///     println!("Hello, world!");
/// }
/// "#;
///
/// let ansi = highlight(
///     code,
///     Options {
///         lang_or_file: Some("rust"),
///         formatter: FormatterOption::Terminal {
///             theme: None,
///         },
///     }
/// );
/// ```
///
/// Output with ANSI terminal colors:
/// ```text
/// [38;2;202;158;230mfn[0m [38;2;140;170;238mmain[0m() {
///     [38;2;140;170;238mprintln![0m([38;2;166;209;137m"Hello, world!"[0m);
/// }
/// ```
pub fn highlight(source: &str, options: Options) -> String {
    let lang = Language::guess(options.lang_or_file.unwrap_or(""), source);
    let formatter = FormatterBuilder::new()
        .with_source(source)
        .with_lang(lang)
        .with_formatter(options.formatter)
        .build();
    let mut buffer = Vec::new();
    let _ = formatter.format(&mut buffer);
    String::from_utf8(buffer).unwrap()
}

pub fn write_highlight(output: &mut dyn Write, source: &str, options: Options) -> io::Result<()> {
    let lang = Language::guess(options.lang_or_file.unwrap_or(""), source);
    let formatter = FormatterBuilder::new()
        .with_source(source)
        .with_lang(lang)
        .with_formatter(options.formatter)
        .build();
    formatter.format(output)?;
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

        let expected = r#"<pre class="athl" style="color: #c6d0f5; background-color: #303446;"><code class="language-javascript" translate="no" tabindex="0"><span class="line" data-line="1"><span style="color: #ca9ee6;">const</span> <span style="color: #99d1db;">=</span> <span style="color: #ef9f76;">1</span>
</span></code></pre>"#;

        let mut buffer = Vec::new();

        write_highlight(
            &mut buffer,
            code,
            Options {
                lang_or_file: Some("javascript"),
                formatter: FormatterOption::HtmlInline {
                    pre_class: None,
                    italic: false,
                    include_highlights: false,
                    theme: themes::get("catppuccin_frappe").ok(),
                    highlight_lines: None,
                },
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

        let expected = r#"<pre class="athl" style="color: #c6d0f5; background-color: #303446;"><code class="language-elixir" translate="no" tabindex="0"><span class="line" data-line="1"><span style="color: #ca9ee6;">defmodule</span> <span style="color: #babbf1;">Foo</span> <span style="color: #ca9ee6;">do</span>
</span><span class="line" data-line="2">  <span style="color: #99d1db;"><span style="color: #949cbb;"><span style="color: #949cbb;">@</span><span style="color: #949cbb;">moduledoc</span> <span style="color: #949cbb;">&quot;&quot;&quot;</span></span></span>
</span><span class="line" data-line="3"><span style="color: #99d1db;"><span style="color: #949cbb;"><span style="color: #949cbb;">  Test Module</span></span></span>
</span><span class="line" data-line="4"><span style="color: #99d1db;"><span style="color: #949cbb;"><span style="color: #949cbb;">  &quot;&quot;&quot;</span></span></span>
</span><span class="line" data-line="5">
</span><span class="line" data-line="6">  <span style="color: #99d1db;"><span style="color: #ef9f76;">@<span style="color: #8caaee;"><span style="color: #ef9f76;">projects <span style="color: #949cbb;">[</span><span style="color: #a6d189;">&quot;Phoenix&quot;</span><span style="color: #949cbb;">,</span> <span style="color: #a6d189;">&quot;MDEx&quot;</span><span style="color: #949cbb;">]</span></span></span></span></span>
</span><span class="line" data-line="7">
</span><span class="line" data-line="8">  <span style="color: #ca9ee6;">def</span> <span style="color: #c6d0f5;">projects</span><span style="color: #949cbb;">,</span> <span style="color: #eebebe;">do: </span><span style="color: #99d1db;"><span style="color: #ef9f76;">@<span style="color: #ef9f76;">projects</span></span></span>
</span><span class="line" data-line="9"><span style="color: #ca9ee6;">end</span>
</span></code></pre>"#;

        let result = highlight(
            code,
            Options {
                lang_or_file: Some("elixir"),
                formatter: FormatterOption::HtmlInline {
                    pre_class: None,
                    italic: false,
                    include_highlights: false,
                    theme: themes::get("catppuccin_frappe").ok(),
                    highlight_lines: None,
                },
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

        let expected = r#"<pre class="athl" style="color: #c6d0f5; background-color: #303446;"><code class="language-elixir" translate="no" tabindex="0"><span class="line" data-line="1"><span data-highlight="keyword" style="color: #ca9ee6;">defmodule</span> <span data-highlight="module" style="color: #babbf1;">Foo</span> <span data-highlight="keyword" style="color: #ca9ee6;">do</span>
</span><span class="line" data-line="2">  <span data-highlight="operator" style="color: #99d1db;"><span data-highlight="constant" style="color: #ef9f76;">@<span data-highlight="function.call" style="color: #8caaee;"><span data-highlight="constant" style="color: #ef9f76;">lang <span data-highlight="string.special.symbol" style="color: #eebebe;">:elixir</span></span></span></span></span>
</span><span class="line" data-line="3"><span data-highlight="keyword" style="color: #ca9ee6;">end</span>
</span></code></pre>"#;

        let result = highlight(
            code,
            Options {
                lang_or_file: Some("elixir"),
                formatter: FormatterOption::HtmlInline {
                    pre_class: None,
                    italic: false,
                    include_highlights: true,
                    theme: themes::get("catppuccin_frappe").ok(),
                    highlight_lines: None,
                },
            },
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn test_highlight_html_inline_escape_curly_braces() {
        let expected = r#"<pre class="athl" style="color: #c6d0f5; background-color: #303446;"><code class="language-elixir" translate="no" tabindex="0"><span class="line" data-line="1"><span style="color: #949cbb;">&lbrace;</span><span style="color: #eebebe;">:ok</span><span style="color: #949cbb;">,</span> <span style="color: #eebebe;">char: </span><span style="color: #81c8be;">&#39;&lbrace;&#39;</span><span style="color: #949cbb;">&rbrace;</span>
</span></code></pre>"#;

        let result = highlight(
            "{:ok, char: '{'}",
            Options {
                lang_or_file: Some("elixir"),
                formatter: FormatterOption::HtmlInline {
                    pre_class: None,
                    italic: false,
                    include_highlights: false,
                    theme: themes::get("catppuccin_frappe").ok(),
                    highlight_lines: None,
                },
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

        let expected = r#"<pre class="athl"><code class="language-elixir" translate="no" tabindex="0"><span class="line" data-line="1"><span class="keyword">defmodule</span> <span class="module">Foo</span> <span class="keyword">do</span>
</span><span class="line" data-line="2">  <span class="operator"><span class="comment-documentation"><span class="comment">@</span><span class="comment">moduledoc</span> <span class="comment">&quot;&quot;&quot;</span></span></span>
</span><span class="line" data-line="3"><span class="operator"><span class="comment-documentation"><span class="comment">  Test Module</span></span></span>
</span><span class="line" data-line="4"><span class="operator"><span class="comment-documentation"><span class="comment">  &quot;&quot;&quot;</span></span></span>
</span><span class="line" data-line="5">
</span><span class="line" data-line="6">  <span class="operator"><span class="constant">@<span class="function-call"><span class="constant">projects <span class="punctuation-bracket">[</span><span class="string">&quot;Phoenix&quot;</span><span class="punctuation-delimiter">,</span> <span class="string">&quot;MDEx&quot;</span><span class="punctuation-bracket">]</span></span></span></span></span>
</span><span class="line" data-line="7">
</span><span class="line" data-line="8">  <span class="keyword">def</span> <span class="variable">projects</span><span class="punctuation-delimiter">,</span> <span class="string-special-symbol">do: </span><span class="operator"><span class="constant">@<span class="constant">projects</span></span></span>
</span><span class="line" data-line="9"><span class="keyword">end</span>
</span></code></pre>"#;

        let result = highlight(
            code,
            Options {
                lang_or_file: Some("elixir"),
                formatter: FormatterOption::HtmlLinked {
                    pre_class: None,
                    highlight_lines: None,
                },
            },
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn test_highlight_html_linked_escape_curly_braces() {
        let expected = r#"<pre class="athl"><code class="language-elixir" translate="no" tabindex="0"><span class="line" data-line="1"><span class="punctuation-bracket">&lbrace;</span><span class="string-special-symbol">:ok</span><span class="punctuation-delimiter">,</span> <span class="string-special-symbol">char: </span><span class="character">&#39;&lbrace;&#39;</span><span class="punctuation-bracket">&rbrace;</span>
</span></code></pre>"#;

        let result = highlight(
            "{:ok, char: '{'}",
            Options {
                lang_or_file: Some("elixir"),
                formatter: FormatterOption::HtmlLinked {
                    pre_class: None,
                    highlight_lines: None,
                },
            },
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn test_guess_language_by_file_name() {
        let result = highlight(
            "foo = 1",
            Options {
                lang_or_file: Some("app.ex"),
                formatter: FormatterOption::HtmlInline {
                    pre_class: None,
                    italic: false,
                    include_highlights: false,
                    theme: themes::get("catppuccin_frappe").ok(),
                    highlight_lines: None,
                },
            },
        );
        assert!(result.as_str().contains("language-elixir"));
    }

    #[test]
    fn test_guess_language_by_file_extension() {
        let result = highlight(
            "# Title",
            Options {
                lang_or_file: Some("md"),
                formatter: FormatterOption::HtmlInline {
                    pre_class: None,
                    italic: false,
                    include_highlights: false,
                    theme: themes::get("catppuccin_frappe").ok(),
                    highlight_lines: None,
                },
            },
        );
        assert!(result.as_str().contains("language-markdown"));

        let result = highlight(
            "foo = 1",
            Options {
                lang_or_file: Some("ex"),
                formatter: FormatterOption::HtmlInline {
                    pre_class: None,
                    italic: false,
                    include_highlights: false,
                    theme: themes::get("catppuccin_frappe").ok(),
                    highlight_lines: None,
                },
            },
        );
        assert!(result.as_str().contains("language-elixir"));
    }

    #[test]
    fn test_guess_language_by_shebang() {
        let result = highlight(
            "#!/usr/bin/env elixir",
            Options {
                lang_or_file: Some("test"),
                formatter: FormatterOption::HtmlInline {
                    pre_class: None,
                    italic: false,
                    include_highlights: false,
                    theme: themes::get("catppuccin_frappe").ok(),
                    highlight_lines: None,
                },
            },
        );
        assert!(result.as_str().contains("language-elixir"));
    }

    #[test]
    fn test_fallback_to_plain_text() {
        let result = highlight(
            "source code",
            Options {
                lang_or_file: Some("none"),
                formatter: FormatterOption::HtmlInline {
                    pre_class: None,
                    italic: false,
                    include_highlights: false,
                    theme: themes::get("catppuccin_frappe").ok(),
                    highlight_lines: None,
                },
            },
        );
        assert!(result.as_str().contains("language-plaintext"));
    }

    #[test]
    fn test_highlight_terminal() {
        let options = Options {
            lang_or_file: Some("ruby"),
            formatter: FormatterOption::Terminal {
                theme: themes::get("dracula").ok(),
            },
        };
        let code = "puts 'Hello from Ruby!'";
        let ansi = highlight(code, options);

        assert!(ansi.as_str().contains("[38;2;241;250;140mHello from Ruby!"));
    }
}

//! Syntax highlighter powered by Tree-sitter and Neovim themes.
//!
//! ## Quick Start
//!
//! Highlight code in three steps: pick a formatter, configure it, format your code.
//!
//! ```rust
//! use lumis::{HtmlInlineBuilder, languages::Language, themes, formatters::Formatter};
//!
//! let code = "fn main() { println!(\"Hello, world!\"); }";
//! let theme = themes::get("dracula").unwrap();
//!
//! let formatter = HtmlInlineBuilder::new()
//!     .language(Language::Rust)
//!     .theme(Some(theme))
//!     .build()
//!     .unwrap();
//!
//! let mut output = Vec::new();
//! lumis::write_highlight(&mut output, code, formatter).unwrap();
//! let html = String::from_utf8(output).unwrap();
//! ```
//!
//! ### Alternative: Using `highlight()` and `write_highlight()`
//!
//! ```rust
//! use lumis::{highlight, HtmlInlineBuilder, languages::Language, themes};
//!
//! let code = "print('Hello')";
//! let theme = themes::get("dracula").unwrap();
//!
//! let formatter = HtmlInlineBuilder::new()
//!     .language(Language::Python)
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
//! use lumis::{write_highlight, TerminalBuilder, languages::Language};
//! use std::fs::File;
//!
//! # let code = "x = 1";
//! # let formatter = TerminalBuilder::new().language(Language::Python).build().unwrap();
//! let mut file = File::create("output.txt").unwrap();
//!
//! write_highlight(&mut file, code, formatter).unwrap();
//! ```
//!
//! ## Language Feature Flags
//!
//! By default, Lumis includes support for all languages, which can result in longer
//! compilation times. You can reduce compilation time and binary size by enabling only
//! the languages you need:
//!
//! ```toml
//! [dependencies]
//! lumis = { version = "0.1", default-features = false, features = ["lang-rust", "lang-javascript", "lang-bundle-web"] }
//! ```
//!
//! Available features include per-language flags like `lang-rust`, `lang-javascript`,
//! `lang-html`, and bundle flags like `lang-bundle-web`, `lang-bundle-web-extra`,
//! `lang-bundle-system`, `lang-bundle-backend`, and `lang-bundle-full`.
//!
//! Use `all-languages` to enable all language support:
//!
//! ```toml
//! [dependencies]
//! lumis = { version = "0.1", features = ["all-languages"] }
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
//! | [`BBCodeScopedBuilder`] | `BBCode` with highlight scope tags | Scope-aware `BBCode` parsers, BBCode-based platforms |
//!
//! See the [`formatters`] module for advanced features like line highlighting and custom formatters.
//!
//! ## Themes
//!
//! 120+ themes from popular Neovim colorschemes. Use with HTML inline and terminal formatters.
//!
//! ```rust
//! use lumis::themes;
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
//! | Bash | *.bash, *.bats, *.cgi, *.command, *.env, *.fcgi, *.ksh, *.sh, *.sh.in, *.tmux, *.tool, *.zsh, .`bash_aliases`, .`bash_history`, .`bash_logout`, .`bash_profile`, .bashrc, .cshrc, .env, .env.example, .flaskenv, .kshrc, .login, .profile, .zlogin, .zlogout, .zprofile, .zshenv, .zshrc, 9fs, PKGBUILD, `bash_aliases`, `bash_logout`, `bash_profile`, bashrc, cshrc, ebuild, eclass, gradlew, kshrc, login, man, profile, zlogin, zlogout, zprofile, zshenv, zshrc |
//! | C | *.c |
//! | Caddy | Caddyfile |
//! | `CMake` | *.cmake, *.cmake.in, CMakeLists.txt |
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
//! | `EEx` | *.eex |
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
//! | `HEEx` | *.heex, *.neex |
//! | HTML | *.html, *.htm, *.xhtml |
//! | Haskell | *.hs, *.hs-boot |
//! | HCL | *.hcl, *.nomad, *.tf, *.tfvars, *.workflow |
//! | `IEx` | *.iex |
//! | JSON | *.json, *.avsc, *.geojson, *.gltf, *.har, *.ice, *.JSON-tmLanguage, *.jsonl, *.mcmeta, *.tfstate, *.tfstate.backup, *.topojson, *.webapp, *.webmanifest, .arcconfig, .auto-changelog, .c8rc, .htmlhintrc, .imgbotconfig, .nycrc, .tern-config, .tern-project, .watchmanconfig, Pipfile.lock, composer.lock, mcmod.info, flake.lock |
//! | Java | *.java |
//! | JavaScript | *.cjs, *.js, *.mjs, *.snap, *.jsx |
//! | Kotlin | *.kt, *.ktm, *.kts |
//! | LaTeX | *.aux, *.cls, *.sty, *.tex |
//! | Liquid | *liquid |
//! | LLVM | *.llvm, *.ll |
//! | Lua | *.lua |
//! | Make | *.mak, *.d, *.make, *.makefile, *.mk, *.mkfile, *.dsp, `BSDmakefile`, `GNUmakefile`, Kbuild, Makefile, MAKEFILE, Makefile.am, Makefile.boot, Makefile.frag, Makefile*.in, Makefile.inc, Makefile.wat, makefile, makefile.sco, mkfile |
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
//! | neosolarized |
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

#[path = "formatter/mod.rs"]
pub mod formatters;
#[deprecated(note = "use `formatters` instead")]
pub use formatters as formatter;
pub mod highlight;
pub mod languages;
pub mod themes;

/// Caller-provided semantic ranges for formatter event streams.
///
/// Annotations use UTF-8 offsets or zero-based line and byte-column
/// positions, and keep caller-owned data typed:
///
/// ```rust
/// use lumis::{HighlightOptions, Annotation};
///
/// #[derive(Debug)]
/// struct Change {
///     id: u64,
/// }
///
/// let annotations = [
///     Annotation::new(4..9, Change { id: 7 })?,
/// ];
/// let options = HighlightOptions::new().annotations(&annotations);
///
/// # let _ = options;
/// # Ok::<(), lumis::annotations::AnnotationError>(())
/// ```
pub mod annotations {
    pub use lumis_core::annotations::{
        Annotation, AnnotationError, AnnotationRange, Position, ResolvedAnnotation,
    };
}

/// Lumis-owned overlays the built-in formatters render.
///
/// An [`Annotation`] carries data only the caller understands, so the built-in
/// formatters skip it. A [`Decoration`](decorations::Decoration) carries data
/// Lumis owns, which is how line highlighting reaches the same event stream:
/// each line arrives as a
/// [`DecorationStart`](events::HighlightEvent::DecorationStart) carrying its
/// number and whether the caller asked for it to be highlighted, and the
/// matching [`DecorationEnd`](events::HighlightEvent::DecorationEnd) closes it.
///
/// ```rust
/// use lumis::decorations::Decoration;
/// use lumis::events::HighlightEvent;
///
/// let event = HighlightEvent::<()>::DecorationStart {
///     decoration: Decoration::Line { number: 3, highlighted: true },
/// };
///
/// assert!(matches!(
///     event,
///     HighlightEvent::DecorationStart {
///         decoration: Decoration::Line { number: 3, .. }
///     }
/// ));
/// ```
pub mod decorations {
    pub use lumis_core::decorations::Decoration;
}

pub use lumis_core::events;
pub use lumis_core::highlights;

// Re-export helper modules from formatters for convenience
pub use formatters::ansi;
pub use formatters::html;

use crate::formatters::Formatter;
use lumis_core::annotations::compose_annotations;
use std::io::{self, Write};

// Re-export builders for easier access
pub use crate::formatters::{
    BBCodeScopedBuilder, HtmlInlineBuilder, HtmlLinkedBuilder, HtmlMultiThemesBuilder,
    TerminalBackground, TerminalBuilder,
};
pub use crate::highlight::HighlightOptions;
pub use lumis_core::annotations::{Annotation, AnnotationError, AnnotationRange};

/// Highlights source code and returns it as a string.
///
/// This is a convenience wrapper that calls the formatter and returns the result as a String.
/// For streaming to files or other writers, use [`write_highlight()`] instead.
///
/// # Arguments
///
/// * `source` - The source code to highlight.
/// * `formatter` - A configured formatter (e.g., from [`HtmlInlineBuilder`], [`TerminalBuilder`]).
/// # Panics
///
/// Panics if the formatter fails to format the source code or produces invalid UTF-8 output.
/// For fallible formatting, use [`write_highlight()`] instead.
///
/// # Examples
///
/// ```rust
/// use lumis::{highlight, HtmlInlineBuilder, languages::Language, themes};
///
/// let code = r#"fn main() { println!("Hello!"); }"#;
///
/// let formatter = HtmlInlineBuilder::new()
///     .language(Language::Rust)
///     .theme(Some(themes::get("onedark").unwrap()))
///     .build()
///     .unwrap();
///
/// let html = highlight(code, formatter);
/// ```
pub fn highlight<F>(source: &str, formatter: F) -> String
where
    F: Formatter<()>,
{
    highlight_with_options(source, formatter, HighlightOptions::new())
}

/// Highlights source code with per-operation options and returns it as a string.
///
/// Use this variant when supplying annotations or enabling rainbow brackets.
///
/// # Panics
///
/// Annotation ranges are checked against `source`, not at construction, so an
/// annotation reaching past the end of the source or landing inside a UTF-8
/// character panics here. Call
/// [`write_highlight_with_options`] instead to handle that as an error.
pub fn highlight_with_options<T, F>(
    source: &str,
    formatter: F,
    options: HighlightOptions<'_, T>,
) -> String
where
    F: Formatter<T>,
{
    let mut buffer = Vec::new();
    write_highlight_with_options(&mut buffer, source, formatter, options)
        .unwrap_or_else(|error| panic!("could not highlight source: {error}"));
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
/// # Examples
///
/// ```rust,no_run
/// use lumis::{write_highlight, HtmlInlineBuilder, languages::Language, themes};
/// use std::fs::File;
///
/// let code = "fn main() { }";
/// let formatter = HtmlInlineBuilder::new()
///     .language(Language::Rust)
///     .theme(Some(themes::get("onedark").unwrap()))
///     .build()
///     .unwrap();
///
/// let mut file = File::create("output.html")?;
/// write_highlight(&mut file, code, formatter)?;
/// # Ok::<(), std::io::Error>(())
/// ```
pub fn write_highlight<F>(output: &mut dyn Write, source: &str, formatter: F) -> io::Result<()>
where
    F: Formatter<()>,
{
    write_highlight_with_options(output, source, formatter, HighlightOptions::new())
}

/// Writes syntax highlighted output with per-operation options.
///
/// Use this variant when supplying annotations or enabling rainbow brackets.
pub fn write_highlight_with_options<T, F>(
    output: &mut dyn Write,
    source: &str,
    formatter: F,
    options: HighlightOptions<'_, T>,
) -> io::Result<()>
where
    F: Formatter<T>,
{
    let syntax_events = crate::highlight::highlight_events_with_options(
        source,
        formatter.language(),
        HighlightOptions::new().rainbow_brackets(options.rainbow_brackets_enabled()),
    )
    .map_err(io::Error::other)?;
    let events = compose_annotations(source, &syntax_events, options.annotation_items())
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;

    formatter.render(source, &events, output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::languages::Language;

    fn test_theme() -> themes::Theme {
        themes::from_json(
            r##"{
                "name": "test_theme",
                "appearance": "dark",
                "revision": "test",
                "highlights": {
                    "normal": {"fg": "#c6d0f6", "bg": "#303447"},
                    "character": {"fg": "#81c8bf"},
                    "comment": {"fg": "#949cbc"},
                    "comment.documentation": {"fg": "#949cbc"},
                    "constant": {"fg": "#ef9f77"},
                    "function": {"fg": "#8caaef"},
                    "function.call": {"fg": "#8caaef"},
                    "keyword": {"fg": "#ca9ee7"},
                    "keyword.function": {"fg": "#ca9ee7"},
                    "module": {"fg": "#e5c891"},
                    "number": {"fg": "#ef9f77"},
                    "operator": {"fg": "#99d1dc"},
                    "punctuation.bracket": {"fg": "#949cbc"},
                    "punctuation.delimiter": {"fg": "#949cbc"},
                    "string": {"fg": "#a6d18a"},
                    "string.special.symbol": {"fg": "#eebebf"}
                }
            }"##,
        )
        .unwrap()
    }

    #[test]
    fn test_write_highlight() {
        let code = r"const = 1";

        let expected = r#"<pre class="lumis" style="color: #c6d0f6; background-color: #303447;"><code class="language-javascript" translate="no" tabindex="0"><div class="l-line" data-line="1"><span style="color: #ca9ee7;">const</span> <span style="color: #99d1dc;">=</span> <span style="color: #ef9f77;">1</span>
</div></code></pre>"#;

        let mut buffer = Vec::new();

        let formatter = HtmlInlineBuilder::default()
            .language(Language::JavaScript)
            .theme(Some(test_theme()))
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

        let expected = r#"<pre class="lumis" style="color: #c6d0f6; background-color: #303447;"><code class="language-elixir" translate="no" tabindex="0"><div class="l-line" data-line="1"><span style="color: #ca9ee7;">defmodule</span> <span style="color: #e5c891;">Foo</span> <span style="color: #ca9ee7;">do</span>
</div><div class="l-line" data-line="2">  <span style="color: #99d1dc;"><span style="color: #949cbc;"><span style="color: #949cbc;">@</span><span style="color: #949cbc;">moduledoc</span> <span style="color: #949cbc;">&quot;&quot;&quot;</span></span></span>
</div><div class="l-line" data-line="3"><span style="color: #99d1dc;"><span style="color: #949cbc;"><span style="color: #949cbc;">  Test Module</span></span></span>
</div><div class="l-line" data-line="4"><span style="color: #99d1dc;"><span style="color: #949cbc;"><span style="color: #949cbc;">  &quot;&quot;&quot;</span></span></span>
</div><div class="l-line" data-line="5">
</div><div class="l-line" data-line="6">  <span style="color: #99d1dc;"><span style="color: #ef9f77;">@<span style="color: #8caaef;"><span style="color: #ef9f77;">projects <span style="color: #949cbc;">[</span><span style="color: #a6d18a;">&quot;Phoenix&quot;</span><span style="color: #949cbc;">,</span> <span style="color: #a6d18a;">&quot;MDEx&quot;</span><span style="color: #949cbc;">]</span></span></span></span></span>
</div><div class="l-line" data-line="7">
</div><div class="l-line" data-line="8">  <span style="color: #ca9ee7;">def</span> <span style="color: #8caaef;">projects</span><span style="color: #949cbc;">,</span> <span style="color: #eebebf;">do: </span><span style="color: #99d1dc;"><span style="color: #ef9f77;">@<span style="color: #ef9f77;">projects</span></span></span>
</div><div class="l-line" data-line="9"><span style="color: #ca9ee7;">end</span>
</div><div class="l-line" data-line="10">
</div></code></pre>"#;

        let formatter = HtmlInlineBuilder::default()
            .language(Language::Elixir)
            .theme(Some(test_theme()))
            .build()
            .unwrap();

        let result = highlight(code, formatter);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_highlight_html_inline_include_highlights() {
        let code = r"defmodule Foo do
  @lang :elixir
end
";

        let expected = r#"<pre class="lumis" style="color: #c6d0f6; background-color: #303447;"><code class="language-elixir" translate="no" tabindex="0"><div class="l-line" data-line="1"><span data-highlight="keyword.function" style="color: #ca9ee7;">defmodule</span> <span data-highlight="module" style="color: #e5c891;">Foo</span> <span data-highlight="keyword" style="color: #ca9ee7;">do</span>
</div><div class="l-line" data-line="2">  <span data-highlight="operator" style="color: #99d1dc;"><span data-highlight="constant" style="color: #ef9f77;">@<span data-highlight="function.call" style="color: #8caaef;"><span data-highlight="constant" style="color: #ef9f77;">lang <span data-highlight="string.special.symbol" style="color: #eebebf;">:elixir</span></span></span></span></span>
</div><div class="l-line" data-line="3"><span data-highlight="keyword" style="color: #ca9ee7;">end</span>
</div><div class="l-line" data-line="4">
</div></code></pre>"#;

        let formatter = HtmlInlineBuilder::default()
            .language(Language::Elixir)
            .include_highlights(true)
            .theme(Some(test_theme()))
            .build()
            .unwrap();

        let result = highlight(code, formatter);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_highlight_html_inline_preserves_curly_braces() {
        let code = "{:ok, char: '{'}";
        let expected = r#"<pre class="lumis" style="color: #c6d0f6; background-color: #303447;"><code class="language-elixir" translate="no" tabindex="0"><div class="l-line" data-line="1"><span style="color: #949cbc;">{</span><span style="color: #eebebf;">:ok</span><span style="color: #949cbc;">,</span> <span style="color: #eebebf;">char: </span><span style="color: #81c8bf;">&#39;{&#39;</span><span style="color: #949cbc;">}</span>
</div></code></pre>"#;

        let formatter = HtmlInlineBuilder::default()
            .language(Language::Elixir)
            .theme(Some(test_theme()))
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

        let expected = r#"<pre class="lumis"><code class="language-elixir" translate="no" tabindex="0"><div class="l-line" data-line="1"><span class="l-keyword-function">defmodule</span> <span class="l-module">Foo</span> <span class="l-keyword">do</span>
</div><div class="l-line" data-line="2">  <span class="l-operator"><span class="l-comment-documentation"><span class="l-comment">@</span><span class="l-comment">moduledoc</span> <span class="l-comment">&quot;&quot;&quot;</span></span></span>
</div><div class="l-line" data-line="3"><span class="l-operator"><span class="l-comment-documentation"><span class="l-comment">  Test Module</span></span></span>
</div><div class="l-line" data-line="4"><span class="l-operator"><span class="l-comment-documentation"><span class="l-comment">  &quot;&quot;&quot;</span></span></span>
</div><div class="l-line" data-line="5">
</div><div class="l-line" data-line="6">  <span class="l-operator"><span class="l-constant">@<span class="l-function-call"><span class="l-constant">projects <span class="l-punctuation-bracket">[</span><span class="l-string">&quot;Phoenix&quot;</span><span class="l-punctuation-delimiter">,</span> <span class="l-string">&quot;MDEx&quot;</span><span class="l-punctuation-bracket">]</span></span></span></span></span>
</div><div class="l-line" data-line="7">
</div><div class="l-line" data-line="8">  <span class="l-keyword-function">def</span> <span class="l-function">projects</span><span class="l-punctuation-delimiter">,</span> <span class="l-string-special-symbol">do: </span><span class="l-operator"><span class="l-constant">@<span class="l-constant">projects</span></span></span>
</div><div class="l-line" data-line="9"><span class="l-keyword">end</span>
</div><div class="l-line" data-line="10">
</div></code></pre>"#;

        let formatter = HtmlLinkedBuilder::default()
            .language(Language::Elixir)
            .build()
            .unwrap();

        let result = highlight(code, formatter);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_highlight_html_linked_preserves_curly_braces() {
        let code = "{:ok, char: '{'}";
        let expected = r#"<pre class="lumis"><code class="language-elixir" translate="no" tabindex="0"><div class="l-line" data-line="1"><span class="l-punctuation-bracket">{</span><span class="l-string-special-symbol">:ok</span><span class="l-punctuation-delimiter">,</span> <span class="l-string-special-symbol">char: </span><span class="l-character">&#39;{&#39;</span><span class="l-punctuation-bracket">}</span>
</div></code></pre>"#;

        let formatter = HtmlLinkedBuilder::default()
            .language(Language::Elixir)
            .build()
            .unwrap();

        let result = highlight(code, formatter);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_guess_language_by_file_name() {
        let code = "foo = 1";
        let formatter = HtmlInlineBuilder::default()
            .language(Language::Elixir)
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
            .language(Language::Markdown)
            .theme(themes::get("catppuccin_frappe").ok())
            .build()
            .unwrap();

        let result = highlight(code1, formatter1);
        assert!(result.contains("language-markdown"));

        let code2 = "foo = 1";
        let formatter2 = HtmlInlineBuilder::default()
            .language(Language::Elixir)
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
            .language(Language::Elixir)
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
            .language(Language::PlainText)
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
            .language(Language::Ruby)
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
            .language(Language::Rust)
            .header(Some(formatter::HtmlElement {
                open_tag: "<div class=\"code-container\">".to_string(),
                close_tag: "</div>".to_string(),
            }))
            .build()
            .unwrap();

        let inline_result = highlight(code, inline_formatter);

        assert!(inline_result.starts_with("<div class=\"code-container\">"));
        assert!(inline_result.ends_with("</div>"));
        assert!(inline_result.contains("<pre class=\"lumis\">"));

        // Test HtmlLinked with header
        let linked_formatter = HtmlLinkedBuilder::default()
            .language(Language::Rust)
            .header(Some(formatter::HtmlElement {
                open_tag: "<section class=\"code-section\">".to_string(),
                close_tag: "</section>".to_string(),
            }))
            .build()
            .unwrap();

        let linked_result = highlight(code, linked_formatter);

        assert!(linked_result.starts_with("<section class=\"code-section\">"));
        assert!(linked_result.ends_with("</section>"));
        assert!(linked_result.contains("<pre class=\"lumis\">"));
    }
}

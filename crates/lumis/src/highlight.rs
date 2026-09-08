//! Core highlighting API that abstracts away tree-sitter complexity.
//!
//! This module provides multiple levels of abstraction for accessing
//! syntax-highlighted tokens.
//!
//! # Custom Formatters
//!
//! Custom formatters should implement [`Formatter`](crate::formatters::Formatter)
//! and render the unified syntax and annotation event stream. Lumis owns
//! highlighting and event composition before calling the formatter's
//! [`render()`](crate::formatters::Formatter::render) method.
//!
//! To highlight something else from inside `render()`, or outside a formatter
//! entirely, [`highlight_iter()`] gives per-token access and
//! [`highlight_iter_with_options()`] opts into the scopes the built-in
//! formatters produce.
//!
//! See also:
//! - [`Formatter`](crate::formatters::Formatter) trait documentation
//! - [`HighlightEvent`](crate::events::HighlightEvent) event contract
//! - [`formatters::html`](crate::formatters::html) module for HTML-specific helpers
//! - [`formatters::ansi`](crate::formatters::ansi) module for terminal/ANSI-specific helpers
//!
//! # Examples
//!
//! ## Simple highlighting
//!
//! ```rust
//! use lumis::highlight::Highlighter;
//! use lumis::languages::Language;
//! use lumis::themes;
//!
//! let code = "fn main() { println!(\"Hello\"); }";
//! let theme = themes::get("dracula").unwrap();
//!
//! let highlighter = Highlighter::new(Language::Rust, Some(theme));
//! let segments = highlighter.highlight(code).unwrap();
//!
//! for (style, text) in segments {
//!     println!("Text: '{}', Color: {:?}", text, style.fg);
//! }
//! ```
//!
//! ## Using the token callback API
//!
//! ```rust
//! use lumis::highlight::highlight_iter;
//! use lumis::languages::Language;
//! use lumis::themes;
//! use std::io::Write;
//!
//! let code = "let x = 42;";
//! let theme = themes::get("github_light").unwrap();
//!
//! highlight_iter(code, Language::Rust, Some(theme), |text, language, range, scope, style| {
//!     println!(
//!         "{}..{}: '{}' (language: {}, scope: {}, color: {:?})",
//!         range.start,
//!         range.end,
//!         text,
//!         language.id_name(),
//!         scope,
//!         style.fg
//!     );
//!     Ok::<_, std::io::Error>(())
//! }).unwrap();
//! ```

use crate::languages::{bracket_query_for_language, Language, LanguageConfig};
use crate::themes::Theme;
use lumis_core::annotations::Annotation;
use lumis_core::events::HighlightEvent as CoreHighlightEvent;
use lumis_core::highlights::HIGHLIGHT_NAMES;
use lumis_wasm_runtime::tree_sitter_highlight::{HighlightEvent, Highlighter as TSHighlighter};
use smol_str::format_smolstr;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Range;
use std::sync::{Arc, LazyLock};
use streaming_iterator::StreamingIterator;
use thiserror::Error;
use tree_sitter::{Parser, Query, QueryCursor};

pub use crate::themes::{Style, TextDecoration, UnderlineStyle};

fn resolve_style(theme: Option<&Theme>, scope: &str, language: &str) -> Style {
    let specialized_scope = format_smolstr!("{}.{}", scope, language);
    theme
        .and_then(|t| t.get_style(&specialized_scope))
        .cloned()
        .unwrap_or_default()
}

static DEFAULT_STYLE: LazyLock<Arc<Style>> = LazyLock::new(|| Arc::new(Style::default()));

/// Options for one highlighting operation.
///
/// The counterpart of the `options` argument to JavaScript's `highlight()` and
/// `highlightEvents()`.
///
/// ```rust
/// use lumis::highlight::HighlightOptions;
///
/// let options = HighlightOptions::new().rainbow_brackets(true);
/// ```
#[derive(Debug, PartialEq, Eq)]
#[must_use]
pub struct HighlightOptions<'a, T = ()> {
    annotations: &'a [Annotation<T>],
    rainbow_brackets: bool,
}

impl<T> Copy for HighlightOptions<'_, T> {}

impl<T> Clone for HighlightOptions<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl Default for HighlightOptions<'static> {
    fn default() -> Self {
        Self::new()
    }
}

impl HighlightOptions<'static> {
    /// Creates options with no annotations and rainbow brackets disabled.
    pub const fn new() -> Self {
        Self {
            annotations: &[],
            rainbow_brackets: false,
        }
    }

    /// Adds caller-provided annotations to this highlighting operation.
    pub fn annotations<T>(self, annotations: &[Annotation<T>]) -> HighlightOptions<'_, T> {
        HighlightOptions {
            annotations,
            rainbow_brackets: self.rainbow_brackets,
        }
    }
}

impl<'a, T> HighlightOptions<'a, T> {
    /// Emit `punctuation.bracket.rainbow.N` scopes around bracket pairs.
    pub const fn rainbow_brackets(mut self, enabled: bool) -> Self {
        self.rainbow_brackets = enabled;
        self
    }

    pub(crate) const fn annotation_items(&self) -> &'a [Annotation<T>] {
        self.annotations
    }

    pub(crate) const fn rainbow_brackets_enabled(&self) -> bool {
        self.rainbow_brackets
    }
}

thread_local! {
    static DOCUMENT_TS_HIGHLIGHTER: RefCell<TSHighlighter> = RefCell::new(TSHighlighter::new());
    static BRACKET_QUERY_CACHE: RefCell<HashMap<&'static str, Option<BracketQueryConfig>>> = RefCell::new(HashMap::new());
    static RAINBOW_PARSER: RefCell<Parser> = RefCell::new(Parser::new());
}

/// Error type for syntax highlighting operations.
///
/// # Examples
///
/// ```rust
/// use lumis::highlight::{highlight_iter, HighlightError};
/// use lumis::languages::Language;
/// use std::io::Write;
///
/// let result = highlight_iter("fn main() {}", Language::Rust, None, |text, _language, _range, _scope, _style| {
///     print!("{}", text);
///     Ok::<_, std::io::Error>(())
/// });
///
/// match result {
///     Ok(()) => {}
///     Err(HighlightError::HighlighterInit(msg)) => {
///         eprintln!("Failed to initialize highlighter: {}", msg);
///     }
///     Err(HighlightError::EventProcessing(msg)) => {
///         eprintln!("Failed to process highlight event: {}", msg);
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum HighlightError {
    /// Failed to initialize the tree-sitter highlighter for the given language.
    #[error("failed to initialize highlighter: {0}")]
    HighlighterInit(String),

    /// Failed to process a highlight event during parsing.
    #[error("failed to process highlight event: {0}")]
    EventProcessing(String),
}

/// High-level stateful highlighter for syntax highlighting.
///
/// This is the primary API for most users. It manages tree-sitter state internally
/// and provides simple methods for highlighting code.
///
/// # Examples
///
/// ```rust
/// use lumis::highlight::Highlighter;
/// use lumis::languages::Language;
/// use lumis::themes;
///
/// let code = "fn main() {}";
/// let theme = themes::get("dracula").unwrap();
///
/// let highlighter = Highlighter::new(Language::Rust, Some(theme));
/// let segments = highlighter.highlight(code).unwrap();
/// ```
pub struct Highlighter {
    language: Language,
    theme: Option<Theme>,
    tree_sitter: RefCell<TSHighlighter>,
}

impl Highlighter {
    /// Create a new highlighter for the given language and optional theme.
    ///
    /// # Arguments
    ///
    /// * `language` - The programming language to highlight
    /// * `theme` - Optional theme for styling. If None, segments will have empty styles.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lumis::highlight::Highlighter;
    /// use lumis::languages::Language;
    /// use lumis::themes;
    ///
    /// // With theme
    /// let theme = themes::get("dracula").unwrap();
    /// let highlighter = Highlighter::new(Language::Rust, Some(theme));
    ///
    /// // Without theme (styles will be empty)
    /// let highlighter = Highlighter::new(Language::JavaScript, None);
    /// ```
    pub fn new(language: Language, theme: Option<Theme>) -> Self {
        Self {
            language,
            theme,
            tree_sitter: RefCell::new(TSHighlighter::new()),
        }
    }

    /// Highlight the entire source code and return styled segments.
    ///
    /// This is the main entry point for highlighting. It returns a vector of
    /// (Style, &str) tuples representing styled segments of the source code.
    ///
    /// # Arguments
    ///
    /// * `source` - The source code to highlight
    ///
    /// # Returns
    ///
    /// A vector of (`Arc<Style>`, `&str`) tuples where:
    /// - `Arc<Style>` contains the styling information (colors, modifiers) in a shared reference
    /// - `&str` is a slice of the original source text
    ///
    /// # Errors
    ///
    /// Returns [`HighlightError`] if tree-sitter highlighting fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lumis::highlight::Highlighter;
    /// use lumis::languages::Language;
    ///
    /// let code = "fn main() { println!(\"Hello\"); }";
    /// let highlighter = Highlighter::new(Language::Rust, None);
    ///
    /// let segments = highlighter.highlight(code).unwrap();
    /// for (style, text) in segments {
    ///     print!("{}", text);  // Print the highlighted code
    /// }
    /// ```
    pub fn highlight<'a>(
        &self,
        source: &'a str,
    ) -> Result<Vec<(Arc<Style>, &'a str)>, HighlightError> {
        let mut ts_highlighter = self.tree_sitter.borrow_mut();
        let events = ts_highlighter
            .highlight(
                self.language.config(),
                source.as_bytes(),
                None,
                |injected| Some(Language::guess(Some(injected), "").config()),
            )
            .map_err(|e| HighlightError::HighlighterInit(format!("{e:?}")))?;

        let mut result = Vec::new();
        let mut style_stack: Vec<Arc<Style>> = vec![Arc::clone(&DEFAULT_STYLE)];

        for event in events {
            let event = event.map_err(|e| HighlightError::EventProcessing(format!("{e:?}")))?;

            match event {
                HighlightEvent::HighlightStart {
                    highlight,
                    language,
                } => {
                    let scope = HIGHLIGHT_NAMES[highlight.0];
                    let specialized_scope = format_smolstr!("{}.{}", scope, language);
                    let new_style = self
                        .theme
                        .as_ref()
                        .and_then(|t| t.get_style(&specialized_scope))
                        .map_or_else(|| Arc::clone(&DEFAULT_STYLE), |s| Arc::new(s.clone()));
                    style_stack.push(new_style);
                }
                HighlightEvent::Source { start, end } => {
                    let text = &source[start..end];
                    if !text.is_empty() {
                        let current_style = style_stack.last().map(Arc::clone).unwrap_or_default();
                        result.push((current_style, text));
                    }
                }
                HighlightEvent::HighlightEnd => {
                    if style_stack.len() > 1 {
                        style_stack.pop();
                    }
                }
            }
        }

        Ok(result)
    }
}

/// Flat token iteration with a callback.
///
/// Walks the highlight events for `source` and calls `on_event_source` once per
/// text segment, with the innermost scope and its resolved style, instead of
/// returning the segments as a vector the way [`Highlighter::highlight`] does.
///
/// # Arguments
///
/// * `source` - Source code to highlight
/// * `language` - The [`Language`] to use for syntax highlighting
/// * `theme` - Optional theme for styling
/// * `on_event_source` - Callback invoked for each text segment, receives `(text, language, range, scope, style)`
///
/// # Errors
///
/// Returns [`HighlightError::HighlighterInit`] if tree-sitter initialization fails,
/// or [`HighlightError::EventProcessing`] if parsing or the callback encounters an error.
///
/// # Examples
///
/// ```rust
/// use lumis::highlight::highlight_iter;
/// use lumis::languages::Language;
/// use lumis::themes;
/// use std::io::Write;
///
/// let code = "fn main() {}";
/// let theme = themes::get("dracula").unwrap();
///
/// let mut output = Vec::new();
/// highlight_iter(code, Language::Rust, Some(theme), |text, _language, _range, _scope, style| {
///     if let Some(ref color) = style.fg {
///         write!(output, "<span style=\"color: {}\">{}</span>", color, text)
///     } else {
///         write!(output, "{}", text)
///     }
/// }).unwrap();
/// ```
pub fn highlight_iter<F, E>(
    source: &str,
    language: Language,
    theme: Option<Theme>,
    on_event_source: F,
) -> Result<(), HighlightError>
where
    F: FnMut(&str, Language, Range<usize>, &'static str, &Style) -> Result<(), E>,
    E: std::error::Error + Send + Sync + 'static,
{
    highlight_iter_with_options(
        source,
        language,
        theme,
        HighlightOptions::default(),
        on_event_source,
    )
}

/// Flat token iteration with a callback, with options.
///
/// See [`highlight_iter`]. With [`HighlightOptions::rainbow_brackets`] the
/// callback receives the `punctuation.bracket.rainbow.N` scopes the built-in
/// formatters render, instead of a flat `punctuation.bracket`.
///
/// # Errors
///
/// Same as [`highlight_iter`].
///
/// # Examples
///
/// ```rust
/// use lumis::highlight::{highlight_iter_with_options, HighlightOptions};
/// use lumis::languages::Language;
///
/// let options = HighlightOptions::new().rainbow_brackets(true);
/// let mut scopes = Vec::new();
///
/// highlight_iter_with_options(
///     "fn main() { let x = (1); }",
///     Language::Rust,
///     None,
///     options,
///     |_text, _language, _range, scope, _style| {
///         scopes.push(scope);
///         Ok::<_, std::io::Error>(())
///     },
/// )
/// .unwrap();
///
/// assert!(scopes.contains(&"punctuation.bracket.rainbow.1"));
/// ```
pub fn highlight_iter_with_options<F, E>(
    source: &str,
    language: Language,
    theme: Option<Theme>,
    options: HighlightOptions<'_>,
    mut on_event_source: F,
) -> Result<(), HighlightError>
where
    F: FnMut(&str, Language, Range<usize>, &'static str, &Style) -> Result<(), E>,
    E: std::error::Error + Send + Sync + 'static,
{
    let mut ts_highlighter = TSHighlighter::new();
    let events =
        highlight_events_with(&mut ts_highlighter, source, language, options, |injected| {
            Some(Language::guess(Some(injected), ""))
        })?;

    let mut style_stack: Vec<Style> = vec![Style::default()];
    let mut scope_stack: Vec<&'static str> = vec![""];
    let mut language_stack = vec![language];

    for event in events {
        match event {
            CoreHighlightEvent::Start {
                scope_index,
                language: lang,
            } => {
                let scope = HIGHLIGHT_NAMES.get(scope_index).copied().unwrap_or("");
                let injected_language = Language::guess(Some(&lang), "");
                let new_style = resolve_style(theme.as_ref(), scope, injected_language.id_name());
                style_stack.push(new_style);
                scope_stack.push(scope);
                language_stack.push(injected_language);
            }
            CoreHighlightEvent::Source { start, end } => {
                let text = &source[start..end];
                if !text.is_empty() {
                    let default_style = Style::default();
                    let current_style = style_stack.last().unwrap_or(&default_style);
                    let current_scope = scope_stack.last().copied().unwrap_or("");
                    let current_language = language_stack.last().copied().unwrap_or(language);
                    on_event_source(
                        text,
                        current_language,
                        start..end,
                        current_scope,
                        current_style,
                    )
                    .map_err(|e| HighlightError::EventProcessing(e.to_string()))?;
                }
            }
            CoreHighlightEvent::End => {
                if style_stack.len() > 1 {
                    style_stack.pop();
                }
                if scope_stack.len() > 1 {
                    scope_stack.pop();
                }
                if language_stack.len() > 1 {
                    language_stack.pop();
                }
            }
            // The flat token stream has no place to surface anything but a
            // scope and its text. Formatters take the composed stream instead.
            _ => {}
        }
    }

    Ok(())
}

/// Highlight `source` into nested open/close events.
///
/// The lower-level counterpart of [`highlight_iter`]: a `Start` opens a scope,
/// `Source` events carry byte ranges, and `End` closes it, with parent scopes
/// staying open across children. Custom formatters that need the nesting rather
/// than a flat token stream consume these.
///
/// [`HighlightEvent::scope`] resolves a `Start` to its scope name, which is
/// what JavaScript's `highlightEvents()` carries directly.
///
/// # Examples
///
/// ```rust
/// # #[cfg(feature = "lang-rust")] {
/// use lumis::{highlight::highlight_events, languages::Language};
/// use lumis::events::HighlightEvent;
///
/// let events = highlight_events("let x = 1;", Language::Rust).unwrap();
///
/// let first_scope = events.iter().find_map(|event| match event {
///     HighlightEvent::Start { .. } => event.scope(),
///     _ => None,
/// });
///
/// assert_eq!(first_scope, Some("keyword"));
/// # }
/// ```
pub fn highlight_events(
    source: &str,
    language: Language,
) -> Result<Vec<CoreHighlightEvent<'static>>, HighlightError> {
    highlight_events_with_options(source, language, HighlightOptions::new())
}

/// Highlight `source` into nested open/close events, with options.
///
/// See [`highlight_events`].
pub fn highlight_events_with_options<T>(
    source: &str,
    language: Language,
    options: HighlightOptions<'_, T>,
) -> Result<Vec<CoreHighlightEvent<'static>>, HighlightError> {
    DOCUMENT_TS_HIGHLIGHTER.with(|ts_highlighter| {
        let mut ts_highlighter = ts_highlighter.borrow_mut();
        highlight_events_with(&mut ts_highlighter, source, language, options, |injected| {
            Some(Language::guess(Some(injected), ""))
        })
    })
}

/// Highlight using only the supplied languages for injections.
///
/// This is used by stateful runtime bindings where loading a language controls
/// whether it may be injected into another language.
#[doc(hidden)]
pub fn highlight_events_with_languages<T>(
    source: &str,
    language: Language,
    options: HighlightOptions<'_, T>,
    languages: &std::collections::HashSet<Language>,
) -> Result<Vec<CoreHighlightEvent<'static>>, HighlightError> {
    DOCUMENT_TS_HIGHLIGHTER.with(|ts_highlighter| {
        let mut ts_highlighter = ts_highlighter.borrow_mut();
        highlight_events_with(&mut ts_highlighter, source, language, options, |injected| {
            let language = Language::guess(Some(injected), "");
            languages.contains(&language).then_some(language)
        })
    })
}

const RAINBOW_BRACKET_SCOPES: [&str; 6] = [
    "punctuation.bracket.rainbow.1",
    "punctuation.bracket.rainbow.2",
    "punctuation.bracket.rainbow.3",
    "punctuation.bracket.rainbow.4",
    "punctuation.bracket.rainbow.5",
    "punctuation.bracket.rainbow.6",
];

/// `HIGHLIGHT_NAMES` indices for the six rainbow bracket scopes, resolved once.
///
/// Each entry falls back to the generic `punctuation.bracket` scope (or `0`) if a
/// rainbow scope is missing, so lookups during highlighting stay O(1).
static RAINBOW_SCOPE_INDICES: LazyLock<[usize; 6]> = LazyLock::new(|| {
    let fallback = HIGHLIGHT_NAMES
        .iter()
        .position(|candidate| *candidate == "punctuation.bracket")
        .unwrap_or(0);
    std::array::from_fn(|i| {
        HIGHLIGHT_NAMES
            .iter()
            .position(|candidate| *candidate == RAINBOW_BRACKET_SCOPES[i])
            .unwrap_or(fallback)
    })
});

#[derive(Clone, Debug)]
struct BracketPair {
    open: Range<usize>,
    close: Range<usize>,
}

#[derive(Clone, Debug)]
struct RainbowRange {
    start: usize,
    end: usize,
    scope_index: usize,
}

struct BracketQueryConfig {
    query: Query,
    open_capture: u32,
    close_capture: u32,
    rainbow_exclude_patterns: Vec<bool>,
}

fn highlight_events_with<T, F>(
    ts_highlighter: &mut TSHighlighter,
    source: &str,
    language: Language,
    options: HighlightOptions<'_, T>,
    injected_language: F,
) -> Result<Vec<CoreHighlightEvent<'static>>, HighlightError>
where
    F: Fn(&str) -> Option<Language>,
{
    let events = ts_highlighter
        .highlight(language.config(), source.as_bytes(), None, |injected| {
            injected_language(injected).map(|language| language.config())
        })
        .map_err(|e| HighlightError::HighlighterInit(format!("{e:?}")))?;

    let core_events = events
        .map(|event| {
            event
                .map_err(|e| HighlightError::EventProcessing(format!("{e:?}")))
                .map(|event| match event {
                    HighlightEvent::HighlightStart {
                        highlight,
                        language,
                    } => CoreHighlightEvent::Start {
                        scope_index: highlight.0,
                        language,
                    },
                    HighlightEvent::Source { start, end } => {
                        CoreHighlightEvent::Source { start, end }
                    }
                    HighlightEvent::HighlightEnd => CoreHighlightEvent::End,
                })
        })
        .collect::<Result<Vec<_>, _>>()?;

    if options.rainbow_brackets_enabled() {
        Ok(apply_query_rainbow_brackets(source, core_events, language))
    } else {
        Ok(core_events)
    }
}

fn apply_query_rainbow_brackets(
    source: &str,
    events: Vec<CoreHighlightEvent<'static>>,
    language: Language,
) -> Vec<CoreHighlightEvent<'static>> {
    let ranges = query_rainbow_ranges(source, language);
    if ranges.is_empty() {
        return events;
    }

    overlay_rainbow_ranges(events, &ranges, language.id_name())
}

fn query_rainbow_ranges(source: &str, language: Language) -> Vec<RainbowRange> {
    let config = language.config();
    let tree = RAINBOW_PARSER.with(|parser| {
        let mut parser = parser.borrow_mut();
        if parser.set_language(&config.language).is_err() {
            return None;
        }
        parser.parse(source.as_bytes(), None)
    });
    let Some(tree) = tree else {
        return Vec::new();
    };

    with_bracket_query_config(config, |bracket_config| {
        let Some(bracket_config) = bracket_config else {
            return Vec::new();
        };

        let mut cursor = QueryCursor::new();
        let mut matches =
            cursor.matches(&bracket_config.query, tree.root_node(), source.as_bytes());
        let mut pairs = Vec::new();

        while let Some(query_match) = matches.next() {
            if bracket_config.rainbow_exclude_patterns[query_match.pattern_index] {
                continue;
            }

            let mut opens = Vec::new();
            let mut closes = Vec::new();
            for capture in query_match.captures {
                if capture.index == bracket_config.open_capture {
                    opens.push(capture.node.byte_range());
                } else if capture.index == bracket_config.close_capture {
                    closes.push(capture.node.byte_range());
                }
            }

            for (open, close) in opens.into_iter().zip(closes) {
                if open.start < close.end && (open.len() == 1 || close.len() == 1) {
                    pairs.push(BracketPair { open, close });
                }
            }
        }

        colorize_bracket_pairs(pairs)
    })
}

fn with_bracket_query_config<R>(
    config: &'static lumis_wasm_runtime::tree_sitter_highlight::HighlightConfiguration,
    f: impl FnOnce(Option<&BracketQueryConfig>) -> R,
) -> R {
    BRACKET_QUERY_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        let entry = cache
            .entry(config.language_name.as_str())
            .or_insert_with(|| {
                let query_source = bracket_query_for_language(config.language_name.as_str());
                if query_source.trim().is_empty() {
                    return None;
                }

                let query = Query::new(&config.language, query_source).ok()?;
                let open_capture = query
                    .capture_names()
                    .iter()
                    .position(|name| *name == "open")
                    .map(|index| index as u32)?;
                let close_capture = query
                    .capture_names()
                    .iter()
                    .position(|name| *name == "close")
                    .map(|index| index as u32)?;
                let rainbow_exclude_patterns = (0..query.pattern_count())
                    .map(|pattern_index| {
                        query
                            .property_settings(pattern_index)
                            .iter()
                            .any(|property| property.key.as_ref() == "rainbow.exclude")
                    })
                    .collect();

                Some(BracketQueryConfig {
                    query,
                    open_capture,
                    close_capture,
                    rainbow_exclude_patterns,
                })
            });

        f(entry.as_ref())
    })
}

fn colorize_bracket_pairs(pairs: Vec<BracketPair>) -> Vec<RainbowRange> {
    let mut opens: Vec<_> = pairs.iter().map(|pair| pair.open.clone()).collect();
    opens.sort_by_key(|range| (range.start, range.end));
    opens.dedup_by(|a, b| a.start == b.start && a.end == b.end);

    let mut color_pairs: Vec<_> = pairs.into_iter().collect();
    color_pairs.sort_by_key(|pair| pair.close.end);

    let mut open_stack: Vec<Range<usize>> = Vec::new();
    let mut open_index = 0usize;
    let mut ranges = Vec::new();

    for pair in color_pairs {
        while open_index < opens.len() && opens[open_index].start < pair.close.start {
            open_stack.push(opens[open_index].clone());
            open_index += 1;
        }

        if open_stack.last() == Some(&pair.open) {
            let depth = open_stack.len() - 1;
            let scope_index = rainbow_scope_index(depth);
            ranges.push(RainbowRange {
                start: pair.open.start,
                end: pair.open.end,
                scope_index,
            });
            ranges.push(RainbowRange {
                start: pair.close.start,
                end: pair.close.end,
                scope_index,
            });
            open_stack.pop();
        }
    }

    ranges.sort_by_key(|range| (range.start, range.end));
    ranges
}

fn overlay_rainbow_ranges(
    events: Vec<CoreHighlightEvent<'static>>,
    ranges: &[RainbowRange],
    language: &str,
) -> Vec<CoreHighlightEvent<'static>> {
    let mut output = Vec::with_capacity(events.len() + ranges.len() * 3);
    let mut range_index = 0usize;

    for event in events {
        let CoreHighlightEvent::Source { start, end } = event else {
            // Move start/end events through untouched.
            output.push(event);
            continue;
        };

        let mut cursor = start;

        while range_index < ranges.len() && ranges[range_index].end <= start {
            range_index += 1;
        }

        let mut next_index = range_index;
        while next_index < ranges.len() {
            let range = &ranges[next_index];
            if range.start >= end {
                break;
            }
            if range.start < start || range.end > end {
                next_index += 1;
                continue;
            }

            if cursor < range.start {
                output.push(CoreHighlightEvent::Source {
                    start: cursor,
                    end: range.start,
                });
            }

            output.push(CoreHighlightEvent::Start {
                scope_index: range.scope_index,
                language: language.to_string(),
            });
            output.push(CoreHighlightEvent::Source {
                start: range.start,
                end: range.end,
            });
            output.push(CoreHighlightEvent::End);
            cursor = range.end;
            next_index += 1;
        }

        if cursor < end {
            output.push(CoreHighlightEvent::Source { start: cursor, end });
        }
    }

    output
}

fn rainbow_scope_index(depth: usize) -> usize {
    RAINBOW_SCOPE_INDICES[depth % RAINBOW_SCOPE_INDICES.len()]
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::themes;

    #[test]
    fn options_default_to_every_switch_off() {
        let options = HighlightOptions::new().rainbow_brackets(true);

        assert!(options.rainbow_brackets_enabled());
        assert!(!HighlightOptions::default().rainbow_brackets_enabled());
        assert_eq!(HighlightOptions::default().annotation_items(), []);
    }

    #[test]
    fn test_highlighter_without_theme() {
        let code = "fn main() {}";
        let highlighter = Highlighter::new(Language::Rust, None);
        let segments = highlighter.highlight(code).unwrap();

        assert!(!segments.is_empty(), "highlighting produced no segments");
        // Segments should have text but no styling
        for (style, _text) in &segments {
            assert_eq!(style.fg, None);
            assert_eq!(style.bg, None);
        }
    }

    #[test]
    fn test_highlighter_with_theme() {
        let code = "fn main() {}";
        let theme = themes::get("dracula").unwrap();
        let highlighter = Highlighter::new(Language::Rust, Some(theme));
        let segments = highlighter.highlight(code).unwrap();

        assert!(!segments.is_empty(), "highlighting produced no segments");

        // At least some segments should have styling
        let has_styling = segments.iter().any(|(style, _text)| style.fg.is_some());
        assert!(has_styling, "Expected at least some styled segments");
    }

    #[test]
    fn test_highlight_preserves_source_text() {
        let code = "fn main() { println!(\"Hello\"); }";
        let highlighter = Highlighter::new(Language::Rust, None);
        let segments = highlighter.highlight(code).unwrap();

        // Concatenating all segments should give back original code
        let reconstructed: String = segments.iter().map(|(_, text)| *text).collect();
        assert_eq!(reconstructed, code);
    }

    #[test]
    fn test_streaming_api() {
        let code = "let x = 42;";
        let mut segments = Vec::new();

        highlight_iter(
            code,
            Language::Rust,
            None,
            |text, language, range, scope, style| {
                segments.push((text.to_string(), language, range, scope, style.clone()));
                Ok::<_, std::io::Error>(())
            },
        )
        .unwrap();

        assert!(!segments.is_empty(), "highlighting produced no segments");

        // Check that ranges are valid and scopes are present
        for (text, language, range, _scope, _style) in &segments {
            assert_eq!(&code[range.clone()], text.as_str());
            assert_eq!(*language, Language::Rust);
        }
    }

    #[test]
    fn test_streaming_with_theme() {
        let code = "let x = 42;";
        let theme = themes::get("github_light").unwrap();
        let mut has_colors = false;
        let mut count = 0;

        highlight_iter(
            code,
            Language::Rust,
            Some(theme),
            |_text, _language, _range, _scope, style| {
                count += 1;
                if style.fg.is_some() {
                    has_colors = true;
                }
                Ok::<_, std::io::Error>(())
            },
        )
        .unwrap();

        assert!(count > 0, "Expected at least some segments");
        assert!(has_colors, "Expected at least some segments with colors");
    }

    fn iter_scopes(code: &str, options: HighlightOptions<'_>) -> Vec<&'static str> {
        let mut scopes = Vec::new();
        highlight_iter_with_options(
            code,
            Language::Rust,
            None,
            options,
            |_text, _language, _range, scope, _style| {
                scopes.push(scope);
                Ok::<_, std::io::Error>(())
            },
        )
        .unwrap();
        scopes
    }

    #[test]
    fn highlight_iter_reports_rainbow_bracket_scopes_when_enabled() {
        let code = "fn main() { let x = (1); }";

        let plain = iter_scopes(code, HighlightOptions::default());
        assert!(plain.contains(&"punctuation.bracket"));
        assert!(!plain
            .iter()
            .any(|scope| scope.starts_with("punctuation.bracket.rainbow")));

        let rainbow = iter_scopes(code, HighlightOptions::new().rainbow_brackets(true));
        assert!(rainbow.contains(&"punctuation.bracket.rainbow.1"));
        assert!(rainbow.contains(&"punctuation.bracket.rainbow.2"));
    }

    #[test]
    fn highlight_iter_with_rainbow_brackets_preserves_source_text() {
        let code = "fn main() { let x = (1); }";
        let mut text = String::new();

        highlight_iter_with_options(
            code,
            Language::Rust,
            None,
            HighlightOptions::new().rainbow_brackets(true),
            |token, _language, range, _scope, _style| {
                assert_eq!(&code[range], token);
                text.push_str(token);
                Ok::<_, std::io::Error>(())
            },
        )
        .unwrap();

        assert_eq!(text, code);
    }

    #[test]
    fn test_empty_source() {
        let code = "";
        let highlighter = Highlighter::new(Language::Rust, None);
        let segments = highlighter.highlight(code).unwrap();

        assert!(segments.is_empty(), "empty source produced segments");
    }

    #[test]
    fn test_multiline_code() {
        let code = "fn main() {\n    println!(\"Hello\");\n}";
        let highlighter = Highlighter::new(Language::Rust, None);
        let segments = highlighter.highlight(code).unwrap();

        let reconstructed: String = segments.iter().map(|(_, text)| *text).collect();
        assert_eq!(reconstructed, code);
    }

    #[test]
    fn test_stateful_highlighter_supports_multiple_calls() {
        let highlighter = Highlighter::new(Language::Rust, None);

        let first = highlighter.highlight("fn first() {}\n").unwrap();
        let second = highlighter.highlight("let second = 2;\n").unwrap();

        let first_text: String = first.iter().map(|(_, text)| *text).collect();
        let second_text: String = second.iter().map(|(_, text)| *text).collect();

        assert_eq!(first_text, "fn first() {}\n");
        assert_eq!(second_text, "let second = 2;\n");
    }

    #[test]
    fn test_highlight_events_preserve_unicode_byte_ranges() {
        let code = "{\"emoji\": \"😀 café\"}";
        let events = highlight_events(code, Language::JSON).unwrap();

        let total_source_bytes = events
            .iter()
            .filter_map(|event| match event {
                CoreHighlightEvent::Source { start, end } => Some(end - start),
                _ => None,
            })
            .sum::<usize>();

        assert_eq!(total_source_bytes, code.len());
    }

    #[test]
    fn test_highlight_events_support_multiple_calls() {
        let first = highlight_events("{\"first\": 1}", Language::JSON).unwrap();
        let second = highlight_events("{\"second\": 2}", Language::JSON).unwrap();

        assert!(!first.is_empty(), "the first call produced no events");
        assert!(!second.is_empty(), "the second call produced no events");
    }

    #[test]
    fn highlight_events_cover_source_contiguously() {
        let code = "fn main() {\n    println!(\"hi\");\n}\n";
        let events = highlight_events(code, Language::Rust).unwrap();

        let mut cursor = 0;
        for event in events {
            if let CoreHighlightEvent::Source { start, end } = event {
                assert_eq!(start, cursor);
                assert!(end >= start);
                cursor = end;
            }
        }

        assert_eq!(cursor, code.len());
    }
}

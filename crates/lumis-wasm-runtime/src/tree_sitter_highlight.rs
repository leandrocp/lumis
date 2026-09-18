// Cognitive complexity is not enforced here, for the same reason `next` is in
// `.lizard-whitelist`: this file tracks upstream, and the local deltas listed
// below are the only ones it should carry.
#![allow(clippy::cognitive_complexity)]
// Vendored from tree-sitter `crates/highlight/src/highlight.rs`.
//
// Shared by Lumis runtimes. The only change Lumis fundamentally needs is
// language-aware highlight events so injected-language layers keep the grammar that
// produced each scope in the event stream itself.
//
// Current local deltas:
// - `HighlightEvent::HighlightStart` carries `language: String`
// - `String::from_utf8_lossy` is used where upstream uses newer tree-sitter helpers
// - Only exclude named children from injections like nvim (https://github.com/leandrocp/lumis/issues/429)
// - `#offset!` is applied to injection ranges and highlight capture ranges, which upstream
//   ignores entirely. nvim-treesitter queries rely on it to strip delimiters (backticks,
//   `${`/`}`, code-fence lines) before the injected grammar sees the text. Neovim applies it
//   in both places: `highlighter.lua` -> `get_range`, and `languagetree.lua` ->
//   `get_node_ranges` -> `get_range`. Note the stale TODO at `languagetree.lua:1087` claiming
//   injections do not support offsets; the code above it does.
//   A same-row offset may reach its own newline, which `(#offset! @c 0 1 0 1)` in the diff
//   injection queries needs to keep joined hunk lines apart, and no further, so the byte and
//   the point keep describing one place. Neovim clamps to neither.
// - `@injection.filename` resolves an injected language from a path, as Neovim's
//   `LanguageTree:_get_injection` does through `vim.filetype.match`. It sits beside the
//   `injection.language` capture it is an alternative to, and is the only reason this file
//   references `lumis_core`. Upstream has no equivalent, so the diff hunk injection queries
//   cannot work without it.
// - A module-level `allow` holds the workspace lint levels off this file. Upstream is not
//   written to them, and restyling it to satisfy `clippy::pedantic` would grow the diff for
//   nothing. Lumis code in here is still reviewed against those lints by hand.
//
// When touching this file, prefer minimizing the diff against upstream rather than extending it,
// and add what you did to the list above in the same change. The list is the only record of why
// this file differs, so an undocumented edit is what makes the next upstream sync guesswork.
//
// Reference:
// https://github.com/tree-sitter/tree-sitter/blob/master/crates/highlight/src/highlight.rs
#![allow(clippy::all, clippy::pedantic, dead_code, elided_lifetimes_in_paths)]

use core::slice;
use std::{
    collections::{HashMap, HashSet},
    iter,
    marker::PhantomData,
    mem::{self, MaybeUninit},
    ops, str,
    sync::{
        atomic::{AtomicUsize, Ordering},
        LazyLock,
    },
};

use streaming_iterator::StreamingIterator;
use thiserror::Error;
use tree_sitter::{
    ffi, Language, Node, ParseOptions, Parser, Point, Query, QueryCapture, QueryCaptures,
    QueryCursor, QueryError, QueryMatch, QueryPredicateArg, Range, TextProvider, Tree,
};

const CANCELLATION_CHECK_INTERVAL: usize = 100;
const BUFFER_HTML_RESERVE_CAPACITY: usize = 10 * 1024;
const BUFFER_LINES_RESERVE_CAPACITY: usize = 1000;

// Bound the number of in-progress query matches so the capture list pool stays
// within tree-sitter's 16-bit capture-list id space, which overflowed and
// corrupted memory on very large inputs before tree-sitter 0.26.9.
const MATCH_LIMIT: u32 = u16::MAX as u32;

static STANDARD_CAPTURE_NAMES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    vec![
        "attribute",
        "boolean",
        "carriage-return",
        "comment",
        "comment.documentation",
        "constant",
        "constant.builtin",
        "constructor",
        "constructor.builtin",
        "embedded",
        "error",
        "escape",
        "function",
        "function.builtin",
        "keyword",
        "markup",
        "markup.bold",
        "markup.heading",
        "markup.italic",
        "markup.link",
        "markup.link.url",
        "markup.list",
        "markup.list.checked",
        "markup.list.numbered",
        "markup.list.unchecked",
        "markup.list.unnumbered",
        "markup.quote",
        "markup.raw",
        "markup.raw.block",
        "markup.raw.inline",
        "markup.strikethrough",
        "module",
        "number",
        "operator",
        "property",
        "property.builtin",
        "punctuation",
        "punctuation.bracket",
        "punctuation.delimiter",
        "punctuation.special",
        "string",
        "string.escape",
        "string.regexp",
        "string.special",
        "string.special.symbol",
        "tag",
        "type",
        "type.builtin",
        "variable",
        "variable.builtin",
        "variable.member",
        "variable.parameter",
    ]
    .into_iter()
    .collect()
});

/// Indicates which highlight should be applied to a region of source code.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Highlight(pub usize);

/// Represents the reason why syntax highlighting failed.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("Cancelled")]
    Cancelled,
    #[error("Invalid language")]
    InvalidLanguage,
    #[error("Unknown error")]
    Unknown,
}

/// Represents a single step in rendering a syntax-highlighted document.
///
/// Modified from upstream to include language context in `HighlightStart`.
#[derive(Clone, Debug)]
pub enum HighlightEvent {
    Source {
        start: usize,
        end: usize,
    },
    HighlightStart {
        highlight: Highlight,
        /// The language name this highlight belongs to (e.g., "markdown", "elixir").
        language: String,
    },
    HighlightEnd,
}

/// Contains the data needed to highlight code written in a particular language.
///
/// This struct is immutable and can be shared between threads.
pub struct HighlightConfiguration {
    pub language: Language,
    pub language_name: String,
    pub query: Query,
    combined_injections_query: Option<Query>,
    locals_pattern_index: usize,
    highlights_pattern_index: usize,
    highlight_indices: Vec<Option<Highlight>>,
    non_local_variable_patterns: Vec<bool>,
    injection_content_capture_index: Option<u32>,
    injection_language_capture_index: Option<u32>,
    injection_filename_capture_index: Option<u32>,
    local_scope_capture_index: Option<u32>,
    local_def_capture_index: Option<u32>,
    local_def_value_capture_index: Option<u32>,
    local_ref_capture_index: Option<u32>,
    /// `(#offset! @capture start_row start_col end_row end_col)` per pattern and capture.
    offsets: HashMap<(usize, u32), [i64; 4]>,
}

/// An `@injection.content` capture together with its `#offset!`-adjusted range.
#[derive(Clone, Copy)]
struct InjectionContent<'a> {
    node: Node<'a>,
    range: Range,
}

/// Applies Neovim's `#offset!` directive to a node's range.
///
/// The four deltas are added to `(start_row, start_col, end_row, end_col)` as in
/// Neovim. Lumis keeps the original range when the adjusted endpoints cannot be
/// represented safely. Tree-sitter columns are byte offsets within a row, so a
/// same-row shift is a plain byte shift; a row shift has to walk to the target line.
fn apply_range_offset(node: Node, offset: [i64; 4], source: &[u8]) -> Range {
    offset_range(node.range(), offset, source)
}

/// The range arithmetic of [`apply_range_offset`], on a range rather than a node,
/// so `fixtures/offset-directive.json` can pin Neovim-compatible results and
/// Lumis's invalid-range safety policy across both runtimes.
fn offset_range(original: Range, offset: [i64; 4], source: &[u8]) -> Range {
    let start = shift_point(
        source,
        original.start_byte,
        original.start_point,
        offset[0],
        offset[1],
    );
    let end = shift_point(
        source,
        original.end_byte,
        original.end_point,
        offset[2],
        offset[3],
    );

    match (start, end) {
        (Some((start_byte, start_point)), Some((end_byte, end_point)))
            if start_byte <= end_byte
                && is_utf8_boundary(source, start_byte)
                && is_utf8_boundary(source, end_byte) =>
        {
            Range {
                start_byte,
                start_point,
                end_byte,
                end_point,
            }
        }
        // Event byte ranges must be ordered UTF-8 slice boundaries.
        _ => original,
    }
}

fn is_utf8_boundary(source: &[u8], byte: usize) -> bool {
    byte == source.len()
        || source
            .get(byte)
            .is_some_and(|value| value & 0b1100_0000 != 0b1000_0000)
}

/// Shifts one endpoint by `row_delta` rows and `column_delta` byte columns.
fn shift_point(
    source: &[u8],
    byte: usize,
    point: Point,
    row_delta: i64,
    column_delta: i64,
) -> Option<(usize, Point)> {
    let row = add_offset_delta(point.row, row_delta)?;
    let column = add_offset_delta(point.column, column_delta)?;

    // A column one past the end addresses that line's newline, which
    // `(#offset! @c 0 1 0 1)` in the diff injection queries depends on: the marker
    // column is dropped and the newline is kept, so joined hunk lines still parse
    // as separate lines. Stopping at the newline rather than at the document end
    // keeps the byte and the point describing one place. Neovim clamps to neither
    // and will return a point whose row no longer holds its byte.
    let same_row = row_delta == 0;
    let line_start = if same_row {
        byte.checked_sub(point.column)?
    } else {
        // Neovim adds the column delta to the endpoint's own column whatever the
        // row delta is, so the column survives the row shift.
        line_start_byte(source, byte, point, row)?
    };
    let line_length = source
        .get(line_start..)?
        .iter()
        .position(|byte| *byte == b'\n')
        .unwrap_or(source.len() - line_start);
    if column > line_length + usize::from(same_row && line_start + line_length < source.len()) {
        return None;
    }
    let byte = line_start.checked_add(column)?;
    Some((byte, Point::new(row, column)))
}

fn add_offset_delta(value: usize, delta: i64) -> Option<usize> {
    let value = i128::try_from(value).ok()?;
    usize::try_from(value + i128::from(delta)).ok()
}

/// The four numeric operands of `#offset!`.
///
/// Neovim reads exactly `pred[3]` through `pred[6]` and never looks further, so
/// an omitted operand is zero and anything after the fourth is untouched —
/// including a non-numeric one. Operands use `LuaJIT`'s numeric-string coercion;
/// values that cannot form an integral Tree-sitter point make the directive
/// unusable instead of reaching range arithmetic.
fn parse_offset_operands(deltas: &[&str]) -> Option<[i64; 4]> {
    let mut offset = [0i64; 4];
    for (slot, value) in offset.iter_mut().zip(deltas) {
        *slot = parse_offset_delta(value)?;
    }
    Some(offset)
}

fn parse_query_offset_operands(deltas: &[QueryPredicateArg]) -> Option<[i64; 4]> {
    let mut offset = [0i64; 4];
    for (slot, value) in offset.iter_mut().zip(deltas) {
        *slot = match value {
            QueryPredicateArg::String(value) => parse_offset_delta(value)?,
            QueryPredicateArg::Capture(capture) => i64::from(*capture) + 1,
        };
    }
    Some(offset)
}

const MAX_EXACT_OFFSET: f64 = 9_007_199_254_740_991.0;
const MAX_LUA_EXPONENT: &str = "1048575";

fn parse_offset_delta(value: &str) -> Option<i64> {
    let value = value.trim_ascii();
    if value.is_empty() {
        return None;
    }

    let unsigned = value.strip_prefix(['+', '-']).unwrap_or(value);
    let parsed = if unsigned.starts_with("0b") || unsigned.starts_with("0B") {
        parse_binary_integer(value)?
    } else if unsigned.starts_with("0x") || unsigned.starts_with("0X") {
        parse_hex_float(value)?
    } else {
        if !is_decimal_float(value) {
            return None;
        }
        value.parse::<f64>().ok()?
    };

    (parsed.is_finite()
        && parsed.fract() == 0.0
        && (-MAX_EXACT_OFFSET..=MAX_EXACT_OFFSET).contains(&parsed))
    .then_some(parsed as i64)
}

fn parse_binary_integer(value: &str) -> Option<f64> {
    let (sign, value) = if let Some(value) = value.strip_prefix('-') {
        (-1.0, value)
    } else {
        (1.0, value.strip_prefix('+').unwrap_or(value))
    };
    let value = value
        .strip_prefix("0b")
        .or_else(|| value.strip_prefix("0B"))?;
    if value.is_empty() {
        return None;
    }

    let mut result = 0.0;
    for byte in value.bytes() {
        result = result * 2.0
            + match byte {
                b'0' => 0.0,
                b'1' => 1.0,
                _ => return None,
            };
    }
    Some(sign * result)
}

fn is_decimal_float(value: &str) -> bool {
    let value = value.strip_prefix(['+', '-']).unwrap_or(value);
    let (mantissa, exponent) = value.find(['e', 'E']).map_or((value, None), |index| {
        (&value[..index], Some(&value[index + 1..]))
    });

    if exponent.is_some_and(|exponent| !is_lua_exponent(exponent)) || mantissa.contains(['e', 'E'])
    {
        return false;
    }

    let mut parts = mantissa.split('.');
    let before = parts.next().unwrap_or_default();
    let after = parts.next();
    if parts.next().is_some() {
        return false;
    }

    let before_is_digits = before.bytes().all(|byte| byte.is_ascii_digit());
    let after_is_digits = after.is_none_or(|part| part.bytes().all(|byte| byte.is_ascii_digit()));
    before_is_digits
        && after_is_digits
        && (!before.is_empty() || after.is_some_and(|part| !part.is_empty()))
}

fn is_signed_decimal(value: &str) -> bool {
    let value = value.strip_prefix(['+', '-']).unwrap_or(value);
    !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit())
}

fn is_lua_exponent(value: &str) -> bool {
    if !is_signed_decimal(value) {
        return false;
    }
    let value = value
        .strip_prefix(['+', '-'])
        .unwrap_or(value)
        .trim_start_matches('0');
    value.len() < MAX_LUA_EXPONENT.len()
        || (value.len() == MAX_LUA_EXPONENT.len() && value <= MAX_LUA_EXPONENT)
}

fn parse_hex_float(value: &str) -> Option<f64> {
    let (negative, value) = if let Some(value) = value.strip_prefix('-') {
        (true, value)
    } else {
        (false, value.strip_prefix('+').unwrap_or(value))
    };
    let value = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))?;
    let (mantissa, exponent) = value.find(['p', 'P']).map_or((value, None), |index| {
        (&value[..index], Some(&value[index + 1..]))
    });
    if mantissa.contains(['p', 'P']) || exponent.is_some_and(|value| !is_lua_exponent(value)) {
        return None;
    }

    let mut significand = 0u128;
    let mut binary_exponent =
        i64::from(exponent.map_or(Some(0), |value| value.parse::<i32>().ok())?);
    let mut seen_point = false;
    let mut inexact = false;
    let mut digits = 0;
    for byte in mantissa.bytes() {
        if byte == b'.' {
            if seen_point {
                return None;
            }
            seen_point = true;
            continue;
        }

        let digit = u128::from(char::from(byte).to_digit(16)?);
        digits += 1;
        if significand >> 124 == 0 {
            significand = (significand << 4) | digit;
        } else {
            binary_exponent += 4;
            inexact |= digit != 0;
        }
        if seen_point {
            binary_exponent -= 4;
        }
    }
    if digits == 0 {
        return None;
    }

    if significand == 0 {
        return Some(if negative { -0.0 } else { 0.0 });
    }
    significand |= u128::from(inexact);

    let mut bits = rounded_hex_f64_bits(significand, binary_exponent);
    if negative {
        bits |= 1 << 63;
    }
    Some(f64::from_bits(bits))
}

fn rounded_hex_f64_bits(mut significand: u128, mut exponent: i64) -> u64 {
    const SIGNIFICAND_BITS: i64 = 52;
    const MIN_SUBNORMAL_EXPONENT: i64 = -1074;
    const INFINITY_BITS: u128 = 0x7ff << SIGNIFICAND_BITS;

    let mut round_bits = i64::from(significand.ilog2()) - SIGNIFICAND_BITS;
    if exponent < MIN_SUBNORMAL_EXPONENT - round_bits {
        round_bits = MIN_SUBNORMAL_EXPONENT - exponent;
    }
    exponent += round_bits;

    if round_bits > 0 {
        if round_bits == 1 {
            significand <<= 1;
        } else if round_bits > 2 {
            significand = if round_bits - 2 < 128 {
                let shift = u32::try_from(round_bits - 2).expect("bounded significand shift");
                (significand >> shift) | u128::from(significand.trailing_zeros() < shift)
            } else {
                1
            };
        }

        let trailing = (significand & 0b111) as u32;
        significand >>= 2;
        significand += u128::from((0b1100_1000_u8 >> trailing) & 1);
    } else if round_bits < 0 {
        significand <<= u32::try_from(-round_bits).expect("bounded significand shift");
    }

    let encoded_exponent = u128::try_from(exponent - MIN_SUBNORMAL_EXPONENT)
        .expect("rounded exponent is representable")
        << SIGNIFICAND_BITS;
    significand
        .checked_add(encoded_exponent)
        .filter(|bits| *bits < INFINITY_BITS)
        .unwrap_or(INFINITY_BITS) as u64
}

/// Byte offset of the first character of `target_row`, walking from a known anchor.
fn line_start_byte(source: &[u8], byte: usize, point: Point, target_row: usize) -> Option<usize> {
    let mut cursor = byte.checked_sub(point.column)?;
    let mut row = point.row;

    while row < target_row {
        let newline = source.get(cursor..)?.iter().position(|b| *b == b'\n')?;
        cursor += newline + 1;
        row += 1;
    }
    while row > target_row {
        // `cursor` sits on a line start, so step back over that newline and find the
        // start of the line before it.
        let previous = cursor.checked_sub(1)?;
        cursor = source
            .get(..previous)?
            .iter()
            .rposition(|b| *b == b'\n')
            .map_or(0, |index| index + 1);
        row -= 1;
    }
    Some(cursor)
}

/// Performs syntax highlighting, recognizing a given list of highlight names.
///
/// For the best performance `Highlighter` values should be reused between
/// syntax highlighting calls. A separate highlighter is needed for each thread that
/// is performing highlighting.
pub struct Highlighter {
    pub parser: Parser,
    cursors: Vec<QueryCursor>,
    record_parsed_layers: bool,
    parsed_layers: Vec<ParsedLayer>,
}

/// A syntax tree produced while highlighting a host or injected language.
pub struct ParsedLayer {
    pub tree: Tree,
    pub language: String,
    pub ranges: Vec<Range>,
    pub depth: usize,
}

/// Converts a general-purpose syntax highlighting iterator into a sequence of lines of HTML.
///
/// Modified from upstream to track language with highlights and pass to callback.
pub struct HtmlRenderer {
    pub html: Vec<u8>,
    pub line_offsets: Vec<u32>,
    carriage_return_highlight: Option<Highlight>,
    // The offset in `self.html` of the last carriage return.
    last_carriage_return: Option<usize>,
}

#[derive(Debug)]
struct LocalDef<'a> {
    name: &'a str,
    value_range: ops::Range<usize>,
    highlight: Option<Highlight>,
}

#[derive(Debug)]
struct LocalScope<'a> {
    inherits: bool,
    range: ops::Range<usize>,
    local_defs: Vec<LocalDef<'a>>,
}

struct HighlightIter<'a, F>
where
    F: FnMut(&str) -> Option<&'a HighlightConfiguration> + 'a,
{
    source: &'a [u8],
    language_name: &'a str,
    byte_offset: usize,
    highlighter: &'a mut Highlighter,
    injection_callback: F,
    cancellation_flag: Option<&'a AtomicUsize>,
    layers: Vec<HighlightIterLayer<'a>>,
    iter_count: usize,
    next_event: Option<HighlightEvent>,
    last_highlight_range: Option<(usize, usize, usize)>,
}

struct HighlightIterLayer<'a> {
    _tree: Tree,
    cursor: QueryCursor,
    captures: iter::Peekable<_QueryCaptures<'a, 'a, &'a [u8], &'a [u8]>>,
    config: &'a HighlightConfiguration,
    highlight_end_stack: Vec<usize>,
    scope_stack: Vec<LocalScope<'a>>,
    ranges: Vec<Range>,
    depth: usize,
}

pub struct _QueryCaptures<'query, 'tree, T: TextProvider<I>, I: AsRef<[u8]>> {
    ptr: *mut ffi::TSQueryCursor,
    query: &'query Query,
    text_provider: T,
    buffer1: Vec<u8>,
    buffer2: Vec<u8>,
    _current_match: Option<(QueryMatch<'query, 'tree>, usize)>,
    _options: Option<*mut ffi::TSQueryCursorOptions>,
    _phantom: PhantomData<(&'tree (), I)>,
}

struct _QueryMatch<'cursor, 'tree> {
    pub _pattern_index: usize,
    pub _captures: &'cursor [QueryCapture<'tree>],
    _id: u32,
    _cursor: *mut ffi::TSQueryCursor,
}

impl<'tree> _QueryMatch<'_, 'tree> {
    fn new(m: &ffi::TSQueryMatch, cursor: *mut ffi::TSQueryCursor) -> Self {
        _QueryMatch {
            _cursor: cursor,
            _id: m.id,
            _pattern_index: m.pattern_index as usize,
            _captures: (m.capture_count > 0)
                .then(|| unsafe {
                    slice::from_raw_parts(
                        m.captures.cast::<QueryCapture<'tree>>(),
                        m.capture_count as usize,
                    )
                })
                .unwrap_or_default(),
        }
    }
}

impl<'query, 'tree: 'query, T: TextProvider<I>, I: AsRef<[u8]>> Iterator
    for _QueryCaptures<'query, 'tree, T, I>
{
    type Item = (QueryMatch<'query, 'tree>, usize);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            loop {
                let mut capture_index = 0u32;
                let mut m = MaybeUninit::<ffi::TSQueryMatch>::uninit();
                if ffi::ts_query_cursor_next_capture(
                    self.ptr,
                    m.as_mut_ptr(),
                    core::ptr::addr_of_mut!(capture_index),
                ) {
                    let result = std::mem::transmute::<_QueryMatch, QueryMatch>(_QueryMatch::new(
                        &m.assume_init(),
                        self.ptr,
                    ));
                    if result.satisfies_text_predicates(
                        self.query,
                        &mut self.buffer1,
                        &mut self.buffer2,
                        &mut self.text_provider,
                    ) {
                        return Some((result, capture_index as usize));
                    }
                    result.remove();
                } else {
                    return None;
                }
            }
        }
    }
}

impl Default for Highlighter {
    fn default() -> Self {
        Self::new()
    }
}

impl Highlighter {
    #[must_use]
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
            cursors: Vec::new(),
            record_parsed_layers: false,
            parsed_layers: Vec::new(),
        }
    }

    pub fn parser(&mut self) -> &mut Parser {
        &mut self.parser
    }

    pub fn record_parsed_layers(&mut self, record: bool) {
        self.record_parsed_layers = record;
    }

    pub fn take_parsed_layers(&mut self) -> Vec<ParsedLayer> {
        mem::take(&mut self.parsed_layers)
    }

    /// Iterate over the highlighted regions for a given slice of source code.
    pub fn highlight<'a>(
        &'a mut self,
        config: &'a HighlightConfiguration,
        source: &'a [u8],
        cancellation_flag: Option<&'a AtomicUsize>,
        mut injection_callback: impl FnMut(&str) -> Option<&'a HighlightConfiguration> + 'a,
    ) -> Result<impl Iterator<Item = Result<HighlightEvent, Error>> + 'a, Error> {
        self.parsed_layers.clear();
        let layers = HighlightIterLayer::new(
            source,
            None,
            self,
            cancellation_flag,
            &mut injection_callback,
            config,
            0,
            vec![Range {
                start_byte: 0,
                end_byte: usize::MAX,
                start_point: Point::new(0, 0),
                end_point: Point::new(usize::MAX, usize::MAX),
            }],
        )?;
        assert_ne!(layers.len(), 0);
        let mut result = HighlightIter {
            source,
            language_name: &config.language_name,
            byte_offset: 0,
            injection_callback,
            cancellation_flag,
            highlighter: self,
            iter_count: 0,
            layers,
            next_event: None,
            last_highlight_range: None,
        };
        result.sort_layers();
        Ok(result)
    }
}

impl HighlightConfiguration {
    /// Creates a `HighlightConfiguration` for a given `Language` and set of highlighting
    /// queries.
    ///
    /// # Parameters
    ///
    /// * `language`  - The Tree-sitter `Language` that should be used for parsing.
    /// * `highlights_query` - A string containing tree patterns for syntax highlighting. This
    ///   should be non-empty, otherwise no syntax highlights will be added.
    /// * `injections_query` -  A string containing tree patterns for injecting other languages into
    ///   the document. This can be empty if no injections are desired.
    /// * `locals_query` - A string containing tree patterns for tracking local variable definitions
    ///   and references. This can be empty if local variable tracking is not needed.
    ///
    /// Returns a `HighlightConfiguration` that can then be used with the `highlight` method.
    pub fn new(
        language: Language,
        name: impl Into<String>,
        highlights_query: &str,
        injection_query: &str,
        locals_query: &str,
    ) -> Result<Self, QueryError> {
        // Concatenate the query strings, keeping track of the start offset of each section.
        let mut query_source = String::with_capacity(
            injection_query.len() + locals_query.len() + highlights_query.len(),
        );
        query_source.push_str(injection_query);
        let locals_query_offset = injection_query.len();
        query_source.push_str(locals_query);
        let highlights_query_offset = injection_query.len() + locals_query.len();
        query_source.push_str(highlights_query);

        // Construct a single query by concatenating the three query strings, but record the
        // range of pattern indices that belong to each individual string.
        let mut query = Query::new(&language, &query_source)?;
        let mut locals_pattern_index = 0;
        let mut highlights_pattern_index = 0;
        for i in 0..(query.pattern_count()) {
            let pattern_offset = query.start_byte_for_pattern(i);
            if pattern_offset < highlights_query_offset {
                if pattern_offset < highlights_query_offset {
                    highlights_pattern_index += 1;
                }
                if pattern_offset < locals_query_offset {
                    locals_pattern_index += 1;
                }
            }
        }

        // Construct a separate query only when the injections actually contain
        // `injection.combined` patterns. Compiling large injection queries twice
        // is otherwise pure initialization overhead.
        let has_combined_queries = (0..locals_pattern_index).any(|pattern_index| {
            query
                .property_settings(pattern_index)
                .iter()
                .any(|setting| &*setting.key == "injection.combined")
        });
        let combined_injections_query = if has_combined_queries {
            let mut combined_injections_query = Query::new(&language, injection_query)?;
            for pattern_index in 0..locals_pattern_index {
                let combined = query
                    .property_settings(pattern_index)
                    .iter()
                    .any(|setting| &*setting.key == "injection.combined");
                if combined {
                    query.disable_pattern(pattern_index);
                } else {
                    combined_injections_query.disable_pattern(pattern_index);
                }
            }
            Some(combined_injections_query)
        } else {
            None
        };

        // Find all of the highlighting patterns that are disabled for nodes that
        // have been identified as local variables.
        let non_local_variable_patterns = (0..query.pattern_count())
            .map(|i| {
                query
                    .property_predicates(i)
                    .iter()
                    .any(|(prop, positive)| !*positive && prop.key.as_ref() == "local")
            })
            .collect();

        // Store the numeric ids for all of the special captures.
        let mut injection_content_capture_index = None;
        let mut injection_language_capture_index = None;
        let mut injection_filename_capture_index = None;
        let mut local_def_capture_index = None;
        let mut local_def_value_capture_index = None;
        let mut local_ref_capture_index = None;
        let mut local_scope_capture_index = None;
        for (i, name) in query.capture_names().iter().enumerate() {
            let i = Some(i as u32);
            match *name {
                "injection.content" => injection_content_capture_index = i,
                "injection.language" => injection_language_capture_index = i,
                "injection.filename" => injection_filename_capture_index = i,
                "local.definition" => local_def_capture_index = i,
                "local.definition-value" => local_def_value_capture_index = i,
                "local.reference" => local_ref_capture_index = i,
                "local.scope" => local_scope_capture_index = i,
                _ => {}
            }
        }

        // `#offset!` is a directive, so tree-sitter leaves it in `general_predicates`
        // rather than parsing it into `property_settings` like `#set!`.
        let mut offsets = HashMap::new();
        for pattern_index in 0..query.pattern_count() {
            for predicate in query.general_predicates(pattern_index) {
                if predicate.operator.as_ref() != "offset!" {
                    continue;
                }
                let [QueryPredicateArg::Capture(capture), deltas @ ..] = &*predicate.args else {
                    continue;
                };
                if let Some(offset) = parse_query_offset_operands(deltas) {
                    offsets.insert((pattern_index, *capture), offset);
                }
            }
        }

        let highlight_indices = vec![None; query.capture_names().len()];
        Ok(Self {
            offsets,
            language,
            language_name: name.into(),
            query,
            combined_injections_query,
            locals_pattern_index,
            highlights_pattern_index,
            highlight_indices,
            non_local_variable_patterns,
            injection_content_capture_index,
            injection_language_capture_index,
            injection_filename_capture_index,
            local_def_capture_index,
            local_def_value_capture_index,
            local_ref_capture_index,
            local_scope_capture_index,
        })
    }

    /// Get a slice containing all of the highlight names used in the configuration.
    #[must_use]
    pub const fn names(&self) -> &[&str] {
        self.query.capture_names()
    }

    /// Set the list of recognized highlight names.
    ///
    /// Tree-sitter syntax-highlighting queries specify highlights in the form of dot-separated
    /// highlight names like `punctuation.bracket` and `function.method.builtin`. Consumers of
    /// these queries can choose to recognize highlights with different levels of specificity.
    /// For example, the string `function.builtin` will match against `function.method.builtin`
    /// and `function.builtin.constructor`, but will not match `function.method`.
    ///
    /// When highlighting, results are returned as `Highlight` values, which contain the index
    /// of the matched highlight this list of highlight names.
    pub fn configure(&mut self, recognized_names: &[impl AsRef<str>]) {
        let mut capture_parts = Vec::new();
        self.highlight_indices.clear();
        self.highlight_indices
            .extend(self.query.capture_names().iter().map(move |capture_name| {
                capture_parts.clear();
                capture_parts.extend(capture_name.split('.'));

                let mut best_index = None;
                let mut best_match_len = 0;
                for (i, recognized_name) in recognized_names.iter().enumerate() {
                    let mut len = 0;
                    let mut matches = true;
                    for part in recognized_name.as_ref().split('.') {
                        len += 1;
                        if !capture_parts.contains(&part) {
                            matches = false;
                            break;
                        }
                    }
                    if matches && len > best_match_len {
                        best_index = Some(i);
                        best_match_len = len;
                    }
                }
                best_index.map(Highlight)
            }));
    }

    // Return the list of this configuration's capture names that are neither present in the
    // list of predefined 'canonical' names nor start with an underscore (denoting 'private'
    // captures used as part of capture internals).
    #[must_use]
    pub fn nonconformant_capture_names(&self, capture_names: &HashSet<&str>) -> Vec<&str> {
        let capture_names = if capture_names.is_empty() {
            &*STANDARD_CAPTURE_NAMES
        } else {
            capture_names
        };
        self.names()
            .iter()
            .filter(|&n| !(n.starts_with('_') || capture_names.contains(n)))
            .copied()
            .collect()
    }
}

impl<'a> HighlightIterLayer<'a> {
    /// Create a new 'layer' of highlighting for this document.
    ///
    /// In the event that the new layer contains "combined injections" (injections where multiple
    /// disjoint ranges are parsed as one syntax tree), these will be eagerly processed and
    /// added to the returned vector.
    #[allow(clippy::too_many_arguments)]
    fn new<F: FnMut(&str) -> Option<&'a HighlightConfiguration> + 'a>(
        source: &'a [u8],
        parent_name: Option<&str>,
        highlighter: &mut Highlighter,
        cancellation_flag: Option<&'a AtomicUsize>,
        injection_callback: &mut F,
        mut config: &'a HighlightConfiguration,
        mut depth: usize,
        mut ranges: Vec<Range>,
    ) -> Result<Vec<Self>, Error> {
        let mut result = Vec::with_capacity(1);
        let mut queue = Vec::new();
        loop {
            if highlighter.parser.set_included_ranges(&ranges).is_ok() {
                highlighter
                    .parser
                    .set_language(&config.language)
                    .map_err(|_| Error::InvalidLanguage)?;

                // tree-sitter 0.26 uses ControlFlow for cancellation checks.
                let tree = highlighter
                    .parser
                    .parse_with_options(
                        &mut |i, _| {
                            if i < source.len() {
                                &source[i..]
                            } else {
                                &[]
                            }
                        },
                        None,
                        Some(ParseOptions::new().progress_callback(&mut |_| {
                            if let Some(cancellation_flag) = cancellation_flag {
                                if cancellation_flag.load(Ordering::SeqCst) != 0 {
                                    ops::ControlFlow::Break(())
                                } else {
                                    ops::ControlFlow::Continue(())
                                }
                            } else {
                                ops::ControlFlow::Continue(())
                            }
                        })),
                    )
                    .ok_or(Error::Cancelled)?;
                let mut cursor = highlighter.cursors.pop().unwrap_or_default();
                cursor.set_match_limit(MATCH_LIMIT);

                // Process combined injections.
                if let Some(combined_injections_query) = &config.combined_injections_query {
                    let mut injections_by_pattern_index =
                        vec![
                            (None, Vec::<InjectionContent>::new(), false);
                            combined_injections_query.pattern_count()
                        ];
                    let mut matches =
                        cursor.matches(combined_injections_query, tree.root_node(), source);
                    while let Some(mat) = matches.next() {
                        let entry = &mut injections_by_pattern_index[mat.pattern_index];
                        let (language_name, content_node, include_children) = injection_for_match(
                            config,
                            parent_name,
                            combined_injections_query,
                            mat,
                            source,
                        );
                        if language_name.is_some() {
                            entry.0 = language_name;
                        }
                        if let Some(content_node) = content_node {
                            entry.1.push(content_node);
                        }
                        entry.2 = include_children;
                    }
                    for (lang_name, content_nodes, includes_children) in injections_by_pattern_index
                    {
                        if let (Some(lang_name), false) = (lang_name, content_nodes.is_empty()) {
                            if let Some(next_config) = (injection_callback)(lang_name) {
                                let ranges = Self::intersect_ranges(
                                    &ranges,
                                    &content_nodes,
                                    includes_children,
                                );
                                if !ranges.is_empty() {
                                    queue.push((next_config, depth + 1, ranges));
                                }
                            }
                        }
                    }
                }

                // The `captures` iterator borrows the `Tree` and the `QueryCursor`, which
                // prevents them from being moved. But both of these values are really just
                // pointers, so it's actually ok to move them.
                let tree_ref = unsafe { mem::transmute::<&Tree, &'static Tree>(&tree) };
                let cursor_ref = unsafe {
                    mem::transmute::<&mut QueryCursor, &'static mut QueryCursor>(&mut cursor)
                };
                let captures = unsafe {
                    std::mem::transmute::<QueryCaptures<_, _>, _QueryCaptures<_, _>>(
                        cursor_ref.captures(&config.query, tree_ref.root_node(), source),
                    )
                }
                .peekable();

                if highlighter.record_parsed_layers {
                    highlighter.parsed_layers.push(ParsedLayer {
                        tree: tree.clone(),
                        language: config.language_name.clone(),
                        ranges: ranges.clone(),
                        depth,
                    });
                }

                result.push(HighlightIterLayer {
                    highlight_end_stack: Vec::new(),
                    scope_stack: vec![LocalScope {
                        inherits: false,
                        range: 0..usize::MAX,
                        local_defs: Vec::new(),
                    }],
                    cursor,
                    depth,
                    _tree: tree,
                    captures,
                    config,
                    ranges,
                });
            }

            if queue.is_empty() {
                break;
            }

            let (next_config, next_depth, next_ranges) = queue.remove(0);
            config = next_config;
            depth = next_depth;
            ranges = next_ranges;
        }

        Ok(result)
    }

    // Compute the ranges that should be included when parsing an injection.
    // This takes into account three things:
    // * `parent_ranges` - The ranges must all fall within the *current* layer's ranges.
    // * `nodes` - Every injection takes place within a set of nodes. The injection ranges are the
    //   ranges of those nodes.
    // * `includes_children` - For some injections, the content nodes' children should be excluded
    //   from the nested document, so that only the content nodes' *own* content is reparsed. For
    //   other injections, the content nodes' entire ranges should be reparsed, including the ranges
    //   of their children.
    fn intersect_ranges(
        parent_ranges: &[Range],
        contents: &[InjectionContent],
        includes_children: bool,
    ) -> Vec<Range> {
        let mut cursor = contents[0].node.walk();
        let mut result = Vec::new();
        let mut parent_range_iter = parent_ranges.iter();
        let mut parent_range = parent_range_iter
            .next()
            .expect("Layers should only be constructed with non-empty ranges vectors");
        for content in contents {
            let node = content.node;
            // The outer bounds come from the `#offset!`-adjusted range; children are still
            // masked out from the node itself, as in Neovim's `get_node_ranges`.
            let mut preceding_range = Range {
                start_byte: 0,
                start_point: Point::new(0, 0),
                end_byte: content.range.start_byte,
                end_point: content.range.start_point,
            };
            let following_range = Range {
                start_byte: content.range.end_byte,
                start_point: content.range.end_point,
                end_byte: usize::MAX,
                end_point: Point::new(usize::MAX, usize::MAX),
            };

            for excluded_range in node
                .children(&mut cursor)
                .filter_map(|child| {
                    if includes_children || !child.is_named() {
                        None
                    } else {
                        Some(child.range())
                    }
                })
                .chain(std::iter::once(following_range))
            {
                let mut range = Range {
                    start_byte: preceding_range.end_byte,
                    start_point: preceding_range.end_point,
                    end_byte: excluded_range.start_byte,
                    end_point: excluded_range.start_point,
                };
                preceding_range = excluded_range;

                if range.end_byte < parent_range.start_byte {
                    continue;
                }

                while parent_range.start_byte <= range.end_byte {
                    if parent_range.end_byte > range.start_byte {
                        if range.start_byte < parent_range.start_byte {
                            range.start_byte = parent_range.start_byte;
                            range.start_point = parent_range.start_point;
                        }

                        if parent_range.end_byte < range.end_byte {
                            if range.start_byte < parent_range.end_byte {
                                result.push(Range {
                                    start_byte: range.start_byte,
                                    start_point: range.start_point,
                                    end_byte: parent_range.end_byte,
                                    end_point: parent_range.end_point,
                                });
                            }
                            range.start_byte = parent_range.end_byte;
                            range.start_point = parent_range.end_point;
                        } else {
                            if range.start_byte < range.end_byte {
                                result.push(range);
                            }
                            break;
                        }
                    }

                    if let Some(next_range) = parent_range_iter.next() {
                        parent_range = next_range;
                    } else {
                        return result;
                    }
                }
            }
        }
        result
    }

    // First, sort scope boundaries by their byte offset in the document. At a
    // given position, emit scope endings before scope beginnings. Finally, emit
    // scope boundaries from deeper layers first.
    fn sort_key(&mut self) -> Option<(usize, bool, isize)> {
        let depth = -(self.depth as isize);
        let next_start = self
            .captures
            .peek()
            .map(|(m, i)| m.captures[*i].node.start_byte());
        let next_end = self.highlight_end_stack.last().copied();
        match (next_start, next_end) {
            (Some(start), Some(end)) => {
                if start < end {
                    Some((start, true, depth))
                } else {
                    Some((end, false, depth))
                }
            }
            (Some(i), None) => Some((i, true, depth)),
            (None, Some(j)) => Some((j, false, depth)),
            _ => None,
        }
    }
}

impl<'a, F> HighlightIter<'a, F>
where
    F: FnMut(&str) -> Option<&'a HighlightConfiguration> + 'a,
{
    fn emit_event(
        &mut self,
        offset: usize,
        event: Option<HighlightEvent>,
    ) -> Option<Result<HighlightEvent, Error>> {
        let result;
        if self.byte_offset < offset {
            result = Some(Ok(HighlightEvent::Source {
                start: self.byte_offset,
                end: offset,
            }));
            self.byte_offset = offset;
            self.next_event = event;
        } else {
            result = event.map(Ok);
        }
        self.sort_layers();
        result
    }

    fn sort_layers(&mut self) {
        while !self.layers.is_empty() {
            if let Some(sort_key) = self.layers[0].sort_key() {
                let mut i = 0;
                while i + 1 < self.layers.len() {
                    if let Some(next_offset) = self.layers[i + 1].sort_key() {
                        if next_offset < sort_key {
                            i += 1;
                            continue;
                        }
                    }
                    break;
                }
                if i > 0 {
                    self.layers[0..=i].rotate_left(1);
                }
                break;
            }
            let layer = self.layers.remove(0);
            self.highlighter.cursors.push(layer.cursor);
        }
    }

    fn insert_layer(&mut self, mut layer: HighlightIterLayer<'a>) {
        if let Some(sort_key) = layer.sort_key() {
            let mut i = 1;
            while i < self.layers.len() {
                if let Some(sort_key_i) = self.layers[i].sort_key() {
                    if sort_key_i > sort_key {
                        self.layers.insert(i, layer);
                        return;
                    }
                    i += 1;
                } else {
                    self.layers.remove(i);
                }
            }
            self.layers.push(layer);
        }
    }
}

impl<'a, F> Iterator for HighlightIter<'a, F>
where
    F: FnMut(&str) -> Option<&'a HighlightConfiguration> + 'a,
{
    type Item = Result<HighlightEvent, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        'main: loop {
            // If we've already determined the next highlight boundary, just return it.
            if let Some(e) = self.next_event.take() {
                return Some(Ok(e));
            }

            // Periodically check for cancellation, returning `Cancelled` error if the
            // cancellation flag was flipped.
            if let Some(cancellation_flag) = self.cancellation_flag {
                self.iter_count += 1;
                if self.iter_count >= CANCELLATION_CHECK_INTERVAL {
                    self.iter_count = 0;
                    if cancellation_flag.load(Ordering::Relaxed) != 0 {
                        return Some(Err(Error::Cancelled));
                    }
                }
            }

            // If none of the layers have any more highlight boundaries, terminate.
            if self.layers.is_empty() {
                return if self.byte_offset < self.source.len() {
                    let result = Some(Ok(HighlightEvent::Source {
                        start: self.byte_offset,
                        end: self.source.len(),
                    }));
                    self.byte_offset = self.source.len();
                    result
                } else {
                    None
                };
            }

            // Get the next capture from whichever layer has the earliest highlight boundary.
            let range;
            let layer = &mut self.layers[0];
            if let Some((next_match, capture_index)) = layer.captures.peek() {
                let next_capture = next_match.captures[*capture_index];
                // Neovim's highlighter resolves a capture's range through `get_range`, so
                // `#offset!` narrows the highlighted span too, not just injections.
                range = layer
                    .config
                    .offsets
                    .get(&(next_match.pattern_index, next_capture.index))
                    .map_or_else(
                        || next_capture.node.byte_range(),
                        |offset| {
                            let adjusted =
                                apply_range_offset(next_capture.node, *offset, self.source);
                            adjusted.start_byte..adjusted.end_byte
                        },
                    );

                // If any previous highlight ends before this node starts, then before
                // processing this capture, emit the source code up until the end of the
                // previous highlight, and an end event for that highlight.
                if let Some(end_byte) = layer.highlight_end_stack.last().copied() {
                    if end_byte <= range.start {
                        layer.highlight_end_stack.pop();
                        return self.emit_event(end_byte, Some(HighlightEvent::HighlightEnd));
                    }
                }
            }
            // If there are no more captures, then emit any remaining highlight end events.
            // And if there are none of those, then just advance to the end of the document.
            else {
                if let Some(end_byte) = layer.highlight_end_stack.last().copied() {
                    layer.highlight_end_stack.pop();
                    return self.emit_event(end_byte, Some(HighlightEvent::HighlightEnd));
                }
                return self.emit_event(self.source.len(), None);
            }

            let (mut match_, capture_index) = layer.captures.next().unwrap();
            let mut capture = match_.captures[capture_index];

            // If this capture represents an injection, then process the injection.
            if match_.pattern_index < layer.config.locals_pattern_index {
                let (language_name, content_node, include_children) = injection_for_match(
                    layer.config,
                    Some(self.language_name),
                    &layer.config.query,
                    &match_,
                    self.source,
                );

                // `captures()` yields a match as soon as its first capture is found,
                // so the content capture may still be ahead of us. Removing the match
                // now would take that capture with it and lose the injection: Rust's
                // macro rule captures `@_macro_name` before `(token_tree)
                // @injection.content`, and its `#not-any-of?` can be decided from the
                // name alone, so the match arrives holding only the name. Leave it in
                // the stream and act on it when the content capture shows up.
                if content_node.is_none() {
                    self.sort_layers();
                    continue 'main;
                }

                // Explicitly remove this match so that none of its other captures will remain
                // in the stream of captures.
                match_.remove();

                // If a language is found with the given name, then add a new language layer
                // to the highlighted document.
                if let (Some(language_name), Some(content_node)) = (language_name, content_node) {
                    if let Some(config) = (self.injection_callback)(language_name) {
                        let ranges = HighlightIterLayer::intersect_ranges(
                            &self.layers[0].ranges,
                            &[content_node],
                            include_children,
                        );
                        if !ranges.is_empty() {
                            match HighlightIterLayer::new(
                                self.source,
                                Some(self.language_name),
                                self.highlighter,
                                self.cancellation_flag,
                                &mut self.injection_callback,
                                config,
                                self.layers[0].depth + 1,
                                ranges,
                            ) {
                                Ok(layers) => {
                                    for layer in layers {
                                        self.insert_layer(layer);
                                    }
                                }
                                Err(e) => return Some(Err(e)),
                            }
                        }
                    }
                }

                self.sort_layers();
                continue 'main;
            }

            // Remove from the local scope stack any local scopes that have already ended.
            while range.start > layer.scope_stack.last().unwrap().range.end {
                layer.scope_stack.pop();
            }

            // If this capture is for tracking local variables, then process the
            // local variable info.
            let mut reference_highlight = None;
            let mut definition_highlight = None;
            while match_.pattern_index < layer.config.highlights_pattern_index {
                // If the node represents a local scope, push a new local scope onto
                // the scope stack.
                if Some(capture.index) == layer.config.local_scope_capture_index {
                    definition_highlight = None;
                    let mut scope = LocalScope {
                        inherits: true,
                        range: range.clone(),
                        local_defs: Vec::new(),
                    };
                    for prop in layer.config.query.property_settings(match_.pattern_index) {
                        if prop.key.as_ref() == "local.scope-inherits" {
                            scope.inherits =
                                prop.value.as_ref().is_none_or(|r| r.as_ref() == "true");
                        }
                    }
                    layer.scope_stack.push(scope);
                }
                // If the node represents a definition, add a new definition to the
                // local scope at the top of the scope stack.
                else if Some(capture.index) == layer.config.local_def_capture_index {
                    reference_highlight = None;
                    definition_highlight = None;
                    let scope = layer.scope_stack.last_mut().unwrap();

                    let mut value_range = 0..0;
                    for capture in match_.captures {
                        if Some(capture.index) == layer.config.local_def_value_capture_index {
                            value_range = capture.node.byte_range();
                        }
                    }

                    if let Ok(name) = str::from_utf8(&self.source[range.clone()]) {
                        scope.local_defs.push(LocalDef {
                            name,
                            value_range,
                            highlight: None,
                        });
                        definition_highlight =
                            scope.local_defs.last_mut().map(|s| &mut s.highlight);
                    }
                }
                // If the node represents a reference, then try to find the corresponding
                // definition in the scope stack.
                else if Some(capture.index) == layer.config.local_ref_capture_index
                    && definition_highlight.is_none()
                {
                    definition_highlight = None;
                    if let Ok(name) = str::from_utf8(&self.source[range.clone()]) {
                        for scope in layer.scope_stack.iter().rev() {
                            if let Some(highlight) = scope.local_defs.iter().rev().find_map(|def| {
                                if def.name == name && range.start >= def.value_range.end {
                                    Some(def.highlight)
                                } else {
                                    None
                                }
                            }) {
                                reference_highlight = highlight;
                                break;
                            }
                            if !scope.inherits {
                                break;
                            }
                        }
                    }
                }

                // Continue processing any additional matches for the same node.
                if let Some((next_match, next_capture_index)) = layer.captures.peek() {
                    let next_capture = next_match.captures[*next_capture_index];
                    if next_capture.node == capture.node {
                        capture = next_capture;
                        match_ = layer.captures.next().unwrap().0;
                        continue;
                    }
                }

                self.sort_layers();
                continue 'main;
            }

            // Otherwise, this capture must represent a highlight.
            // If this exact range has already been highlighted by an earlier pattern, or by
            // a different layer, then skip over this one.
            if let Some((last_start, last_end, last_depth)) = self.last_highlight_range {
                if range.start == last_start && range.end == last_end && layer.depth < last_depth {
                    self.sort_layers();
                    continue 'main;
                }
            }

            // Once a highlighting pattern is found for the current node, keep iterating over
            // any later highlighting patterns that also match this node and set the match to it.
            // Captures for a given node are ordered by pattern index, so these subsequent
            // captures are guaranteed to be for highlighting, not injections or
            // local variables.
            while let Some((next_match, next_capture_index)) = layer.captures.peek() {
                let next_capture = next_match.captures[*next_capture_index];
                if next_capture.node == capture.node {
                    let following_match = layer.captures.next().unwrap().0;
                    // If the current node was found to be a local variable, then ignore
                    // the following match if it's a highlighting pattern that is disabled
                    // for local variables.
                    if (definition_highlight.is_some() || reference_highlight.is_some())
                        && layer.config.non_local_variable_patterns[following_match.pattern_index]
                    {
                        continue;
                    }
                    // MODIFICATION: a capture with no recognized highlight does not
                    // win the node. nvim-treesitter marks helper captures `@_name`,
                    // and Neovim skips them because they resolve to no highlight
                    // group. Letting one win here would blank the node and, through
                    // `remove()`, discard its match's other captures as well.
                    if layer.config.highlight_indices[next_capture.index as usize].is_none() {
                        continue;
                    }
                    match_.remove();
                    capture = next_capture;
                    match_ = following_match;
                } else {
                    break;
                }
            }

            // A MISSING node is synthesised by error recovery, spans no bytes, and so
            // can only ever produce an empty span. Skip it: this runtime and
            // `web-tree-sitter` do not recover identically — given `write!(x, "y")`,
            // the injected macro body parses to an `ERROR` here and to a
            // `MISSING ";"` there — and highlighting the artefact would leak that
            // disagreement into output the two are required to render identically.
            if capture.node.is_missing() {
                self.sort_layers();
                continue 'main;
            }

            let current_highlight = layer.config.highlight_indices[capture.index as usize];

            // If this node represents a local definition, then store the current
            // highlight value on the local scope entry representing this node.
            if let Some(definition_highlight) = definition_highlight {
                *definition_highlight = current_highlight;
            }

            // Emit a scope start event and push the node's end position to the stack.
            // MODIFICATION: Include the layer's language name in the event.
            if let Some(highlight) = reference_highlight.or(current_highlight) {
                self.last_highlight_range = Some((range.start, range.end, layer.depth));
                let language = layer.config.language_name.clone();
                layer.highlight_end_stack.push(range.end);
                return self.emit_event(
                    range.start,
                    Some(HighlightEvent::HighlightStart {
                        highlight,
                        language,
                    }),
                );
            }

            self.sort_layers();
        }
    }
}

impl Default for HtmlRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl HtmlRenderer {
    #[must_use]
    pub fn new() -> Self {
        let mut result = Self {
            html: Vec::with_capacity(BUFFER_HTML_RESERVE_CAPACITY),
            line_offsets: Vec::with_capacity(BUFFER_LINES_RESERVE_CAPACITY),
            carriage_return_highlight: None,
            last_carriage_return: None,
        };
        result.line_offsets.push(0);
        result
    }

    pub fn set_carriage_return_highlight(&mut self, highlight: Option<Highlight>) {
        self.carriage_return_highlight = highlight;
    }

    pub fn reset(&mut self) {
        shrink_and_clear(&mut self.html, BUFFER_HTML_RESERVE_CAPACITY);
        shrink_and_clear(&mut self.line_offsets, BUFFER_LINES_RESERVE_CAPACITY);
        self.line_offsets.push(0);
    }

    pub fn render<F>(
        &mut self,
        highlighter: impl Iterator<Item = Result<HighlightEvent, Error>>,
        source: &[u8],
        attribute_callback: &F,
    ) -> Result<(), Error>
    where
        F: Fn(Highlight, &mut Vec<u8>),
    {
        let mut highlights = Vec::new();
        for event in highlighter {
            match event {
                Ok(HighlightEvent::HighlightStart { highlight, .. }) => {
                    highlights.push(highlight);
                    self.start_highlight(highlight, &attribute_callback);
                }
                Ok(HighlightEvent::HighlightEnd) => {
                    highlights.pop();
                    self.end_highlight();
                }
                Ok(HighlightEvent::Source { start, end }) => {
                    self.add_text(&source[start..end], &highlights, &attribute_callback);
                }
                Err(a) => return Err(a),
            }
        }
        if let Some(offset) = self.last_carriage_return.take() {
            self.add_carriage_return(offset, attribute_callback);
        }
        if self.html.last() != Some(&b'\n') {
            self.html.push(b'\n');
        }
        if self.line_offsets.last() == Some(&(self.html.len() as u32)) {
            self.line_offsets.pop();
        }
        Ok(())
    }

    pub fn lines(&self) -> impl Iterator<Item = &str> {
        self.line_offsets
            .iter()
            .enumerate()
            .map(move |(i, line_start)| {
                let line_start = *line_start as usize;
                let line_end = if i + 1 == self.line_offsets.len() {
                    self.html.len()
                } else {
                    self.line_offsets[i + 1] as usize
                };
                str::from_utf8(&self.html[line_start..line_end]).unwrap()
            })
    }

    fn add_carriage_return<F>(&mut self, offset: usize, attribute_callback: &F)
    where
        F: Fn(Highlight, &mut Vec<u8>),
    {
        if let Some(highlight) = self.carriage_return_highlight {
            // If a CR is the last character in a `HighlightEvent::Source`
            // region, then we don't know until the next `Source` event or EOF
            // whether it is part of CRLF or on its own. To avoid unbounded
            // lookahead, save the offset of the CR and insert there now that we
            // know.
            let rest = self.html.split_off(offset);
            self.html.extend(b"<span ");
            (attribute_callback)(highlight, &mut self.html);
            self.html.extend(b"></span>");
            self.html.extend(rest);
        }
    }

    fn start_highlight<F>(&mut self, h: Highlight, attribute_callback: &F)
    where
        F: Fn(Highlight, &mut Vec<u8>),
    {
        self.html.extend(b"<span ");
        (attribute_callback)(h, &mut self.html);
        self.html.extend(b">");
    }

    fn end_highlight(&mut self) {
        self.html.extend(b"</span>");
    }

    fn add_text<F>(&mut self, src: &[u8], highlights: &[Highlight], attribute_callback: &F)
    where
        F: Fn(Highlight, &mut Vec<u8>),
    {
        pub(crate) const fn html_escape(c: u8) -> Option<&'static [u8]> {
            match c as char {
                '>' => Some(b"&gt;"),
                '<' => Some(b"&lt;"),
                '&' => Some(b"&amp;"),
                '\'' => Some(b"&#39;"),
                '"' => Some(b"&quot;"),
                _ => None,
            }
        }

        // Note: Using String::from_utf8_lossy instead of LossyUtf8 (not exported by tree-sitter)
        for c in String::from_utf8_lossy(src).chars() {
            // Don't render carriage return characters, but allow lone carriage returns (not
            // followed by line feeds) to be styled via the attribute callback.
            if c == '\r' {
                self.last_carriage_return = Some(self.html.len());
                continue;
            }
            if let Some(offset) = self.last_carriage_return.take() {
                if c != '\n' {
                    self.add_carriage_return(offset, attribute_callback);
                }
            }

            // At line boundaries, close and re-open all of the open tags.
            if c == '\n' {
                for _ in highlights {
                    self.end_highlight();
                }
                self.html.push(c as u8);
                self.line_offsets.push(self.html.len() as u32);
                for scope in highlights {
                    self.start_highlight(*scope, attribute_callback);
                }
            } else if let Some(escape) = html_escape(c as u8) {
                self.html.extend_from_slice(escape);
            } else {
                let mut buf = [0u8; 4];
                self.html
                    .extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
            }
        }
    }
}

fn injection_for_match<'a>(
    config: &'a HighlightConfiguration,
    parent_name: Option<&'a str>,
    query: &'a Query,
    query_match: &QueryMatch<'a, 'a>,
    source: &'a [u8],
) -> (Option<&'a str>, Option<InjectionContent<'a>>, bool) {
    let content_capture_index = config.injection_content_capture_index;
    let language_capture_index = config.injection_language_capture_index;
    let filename_capture_index = config.injection_filename_capture_index;

    let mut language_name: Option<&str> = None;
    let mut filename_language: Option<&'static str> = None;
    let mut content_node = None;

    for capture in query_match.captures {
        let index = Some(capture.index);
        if index == language_capture_index {
            language_name = capture.node.utf8_text(source).ok();
        } else if index == filename_capture_index {
            // Neovim resolves this capture through `vim.filetype.match`, which
            // reads the text as a path rather than a language name.
            let range = config
                .offsets
                .get(&(query_match.pattern_index, capture.index))
                .map_or_else(
                    || capture.node.range(),
                    |offset| apply_range_offset(capture.node, *offset, source),
                );
            filename_language = source
                .get(range.start_byte..range.end_byte)
                .and_then(|bytes| std::str::from_utf8(bytes).ok())
                .and_then(lumis_core::languages::language_id_for_filename);
        } else if index == content_capture_index {
            // Neovim narrows the injected range with `#offset!` before parsing it, so
            // delimiters such as backticks or `${`/`}` never reach the injected grammar.
            let range = config
                .offsets
                .get(&(query_match.pattern_index, capture.index))
                .map_or_else(
                    || capture.node.range(),
                    |offset| apply_range_offset(capture.node, *offset, source),
                );
            content_node = Some(InjectionContent {
                node: capture.node,
                range,
            });
        }
    }

    // An explicit `@injection.language` capture outranks a filename, which is
    // only ever an inference about one.
    let mut language_name = language_name.or(filename_language);

    let mut include_children = false;
    for prop in query.property_settings(query_match.pattern_index) {
        match prop.key.as_ref() {
            // In addition to specifying the language name via the text of a
            // captured node, it can also be hard-coded via a `#set!` predicate
            // that sets the injection.language key.
            "injection.language" => {
                if language_name.is_none() {
                    language_name = prop.value.as_ref().map(std::convert::AsRef::as_ref);
                }
            }

            // Setting the `injection.self` key can be used to specify that the
            // language name should be the same as the language of the current
            // layer.
            "injection.self" => {
                if language_name.is_none() {
                    language_name = Some(config.language_name.as_str());
                }
            }

            // Setting the `injection.parent` key can be used to specify that
            // the language name should be the same as the language of the
            // parent layer
            "injection.parent" => {
                if language_name.is_none() {
                    language_name = parent_name;
                }
            }

            // By default, injections do not include the *children* of an
            // `injection.content` node - only the ranges that belong to the
            // node itself. This can be changed using a `#set!` predicate that
            // sets the `injection.include-children` key.
            "injection.include-children" => include_children = true,
            _ => {}
        }
    }

    (language_name, content_node, include_children)
}

fn shrink_and_clear<T>(vec: &mut Vec<T>, capacity: usize) {
    if vec.len() > capacity {
        vec.truncate(capacity);
        vec.shrink_to_fit();
    }
    vec.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_event_has_language() {
        let event = HighlightEvent::HighlightStart {
            highlight: Highlight(0),
            language: "rust".to_string(),
        };

        if let HighlightEvent::HighlightStart { language, .. } = event {
            assert_eq!(language, "rust");
        } else {
            panic!("Expected HighlightStart");
        }
    }

    /// Representable cases in `fixtures/offset-directive.json` came from Neovim;
    /// invalid-range cases pin Lumis's documented safety fallback.
    /// `packages/javascript/lumis/test/offset-directive.test.ts` reads the same file.
    #[test]
    fn offset_arithmetic_matches_neovim() {
        #[derive(serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Endpoint {
            start_row: usize,
            start_column: usize,
            start_byte: usize,
            end_row: usize,
            end_column: usize,
            end_byte: usize,
        }

        #[derive(serde::Deserialize)]
        struct Case {
            name: String,
            source: String,
            query: String,
            offset: Vec<String>,
            original: Endpoint,
            expected: Endpoint,
        }

        #[derive(serde::Deserialize)]
        struct Fixture {
            cases: Vec<Case>,
        }

        let raw = include_str!("../../../fixtures/offset-directive.json");
        let fixture: Fixture = serde_json::from_str(raw).expect("offset fixture is valid JSON");
        assert!(
            fixture.cases.len() >= 18,
            "the fixture must not silently shrink: {} cases",
            fixture.cases.len()
        );

        let language = tree_sitter::Language::new(tree_sitter_json::LANGUAGE);

        for case in &fixture.cases {
            // Compiled through the real query path, so a change to how operands
            // are extracted from a predicate fails here too.
            let config =
                HighlightConfiguration::new(language.clone(), "json", &case.query, "", "").unwrap();
            let offset = *config
                .offsets
                .values()
                .next()
                .unwrap_or_else(|| panic!("{}: the directive was dropped", case.name));

            let operands: Vec<&str> = case.offset.iter().map(String::as_str).collect();
            assert_eq!(
                parse_offset_operands(&operands),
                Some(offset),
                "{}: the compiled query and the operand parser disagree",
                case.name
            );

            let original = Range {
                start_byte: case.original.start_byte,
                start_point: Point::new(case.original.start_row, case.original.start_column),
                end_byte: case.original.end_byte,
                end_point: Point::new(case.original.end_row, case.original.end_column),
            };
            let actual = offset_range(original, offset, case.source.as_bytes());

            assert_eq!(
                (
                    actual.start_point.row,
                    actual.start_point.column,
                    actual.start_byte,
                    actual.end_point.row,
                    actual.end_point.column,
                    actual.end_byte,
                ),
                (
                    case.expected.start_row,
                    case.expected.start_column,
                    case.expected.start_byte,
                    case.expected.end_row,
                    case.expected.end_column,
                    case.expected.end_byte,
                ),
                "{}",
                case.name
            );
        }
    }

    // `#offset!` arithmetic, pinned to Neovim where the result is representable
    // and to Lumis's shared safety policy otherwise. The conformance fixtures
    // cover the common same-row case end to end; these cover row shifts and
    // degenerate cases they do not reach.

    #[test]
    fn same_row_offset_is_a_byte_shift() {
        let source = b"html`<div>`";
        // Start of the template string, shifted right one byte past the backtick.
        let shifted = shift_point(source, 4, Point::new(0, 4), 0, 1).unwrap();
        assert_eq!(shifted, (5, Point::new(0, 5)));
        // End shifted left one byte, off the closing backtick.
        let shifted = shift_point(source, 11, Point::new(0, 11), 0, -1).unwrap();
        assert_eq!(shifted, (10, Point::new(0, 10)));
    }

    #[test]
    fn row_offset_walks_to_the_target_line() {
        // The `(#offset! @injection.content 1 0 -1 0)` shape used to strip fence lines.
        let source = b"```lua\nprint(1)\n```\n";
        // Start on row 0 moves to the beginning of row 1.
        assert_eq!(
            shift_point(source, 0, Point::new(0, 0), 1, 0).unwrap(),
            (7, Point::new(1, 0))
        );
        // End on row 2 moves back to the beginning of row 1.
        assert_eq!(
            shift_point(source, 16, Point::new(2, 0), -1, 0).unwrap(),
            (7, Point::new(1, 0))
        );
    }

    /// Neovim's `apply_range_offset` does `range[2] = range[2] + start_col_offset`
    /// whatever the row delta is, so a row shift keeps the capture's own column
    /// instead of resetting it to the start of the target line. Every row-shifting
    /// pattern in the shipped corpus captures frontmatter, which always begins at
    /// column zero, so only a custom query reaches this.
    #[test]
    fn a_row_offset_keeps_the_original_column() {
        //                    row 0        row 1        row 2
        let source = b"  {\n    \"a\": 1,\n  }\n";

        // A capture at (0, 2) shifted down one row lands at (1, 2), not (1, 0).
        assert_eq!(
            shift_point(source, 2, Point::new(0, 2), 1, 0).unwrap(),
            (6, Point::new(1, 2))
        );
        // And the column delta still applies on top of the original column.
        assert_eq!(
            shift_point(source, 2, Point::new(0, 2), 1, 2).unwrap(),
            (8, Point::new(1, 4))
        );
        // Including a negative one, which the row branch used to reject outright.
        assert_eq!(
            shift_point(source, 6, Point::new(1, 2), 1, -1).unwrap(),
            (17, Point::new(2, 1))
        );
    }

    /// Neovim reads `pred[3]` through `pred[6]` and stops, so a fifth operand of
    /// any kind is untouched. Inspecting every argument instead made a capture
    /// there void the whole directive.
    #[test]
    fn a_fifth_operand_never_voids_the_directive() {
        let language = tree_sitter::Language::new(tree_sitter_json::LANGUAGE);
        let compile = |query: &str| {
            HighlightConfiguration::new(language.clone(), "json", query, "", "")
                .unwrap()
                .offsets
                .values()
                .next()
                .copied()
        };

        for query in [
            "((string) @cap (#offset! @cap 0 1 0 -1))",
            "((string) @cap (#offset! @cap 0 1 0 -1 99))",
            "((string) @cap (#offset! @cap 0 1 0 -1 nope))",
            "((string) @cap (#offset! @cap 0 1 0 -1 @cap))",
        ] {
            assert_eq!(compile(query), Some([0, 1, 0, -1]), "{query}");
        }

        // A literal in one of the four slots still has to be numeric. Capture
        // operands arrive at Neovim's Lua directive as one-based capture IDs.
        assert_eq!(compile("((string) @cap (#offset! @cap 0 nope 0 -1))"), None);
        assert_eq!(
            compile("((string) @cap (#offset! @cap 0 @cap 0 -1))"),
            Some([0, 1, 0, -1])
        );
        assert_eq!(
            compile(
                "((array (string) @first (string) @second) @cap \
                 (#offset! @cap 0 @second 0 -1))"
            ),
            Some([0, 2, 0, -1])
        );
    }

    /// `LuaJIT`'s numeric-string coercion is broader than signed decimal, but a
    /// fractional or non-finite result cannot form a Tree-sitter point. The
    /// browser reads the same cases from the fixture.
    #[test]
    fn offset_operand_coercion_matches_neovim() {
        #[derive(serde::Deserialize)]
        struct OperandCoercion {
            name: String,
            operand: String,
            expected: Option<i64>,
        }

        #[derive(serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Fixture {
            operand_coercions: Vec<OperandCoercion>,
        }

        let raw = include_str!("../../../fixtures/offset-directive.json");
        let fixture: Fixture = serde_json::from_str(raw).expect("offset fixture is valid JSON");
        assert!(
            fixture.operand_coercions.len() >= 24,
            "the operand fixture must not silently shrink"
        );
        let language = tree_sitter::Language::new(tree_sitter_json::LANGUAGE);

        for case in fixture.operand_coercions {
            let parsed =
                parse_offset_operands(&["0", &case.operand, "0", "-1"]).map(|offset| offset[1]);
            assert_eq!(parsed, case.expected, "{} parser", case.name);

            let operand = serde_json::to_string(&case.operand).unwrap();
            let query = format!("((string) @cap (#offset! @cap 0 {operand} 0 -1))");
            let compiled = HighlightConfiguration::new(language.clone(), "json", &query, "", "")
                .unwrap()
                .offsets
                .values()
                .next()
                .map(|offset| offset[1]);
            assert_eq!(compiled, case.expected, "{} compiled query", case.name);
        }
    }

    /// Neovim defaults an omitted numeric operand to zero (`pred[3] or 0`).
    #[test]
    fn omitted_offset_operands_default_to_zero() {
        assert_eq!(parse_offset_operands(&[]), Some([0, 0, 0, 0]));
        assert_eq!(parse_offset_operands(&["1"]), Some([1, 0, 0, 0]));
        assert_eq!(parse_offset_operands(&["0", "1", "0"]), Some([0, 1, 0, 0]));
        assert_eq!(
            parse_offset_operands(&["0", "1", "0", "-1"]),
            Some([0, 1, 0, -1])
        );
        // Neovim reads exactly four operands and never looks at a fifth.
        assert_eq!(
            parse_offset_operands(&["0", "1", "0", "-1", "9"]),
            Some([0, 1, 0, -1])
        );
        assert_eq!(parse_offset_operands(&["not-a-number"]), None);
    }

    #[test]
    fn multibyte_rows_are_walked_by_byte_not_character() {
        // "é" is two bytes, so row 1 starts at byte 3, not code-unit offset 2.
        let source = "é\nx\n".as_bytes();
        assert_eq!(
            shift_point(source, 0, Point::new(0, 0), 1, 0).unwrap(),
            (3, Point::new(1, 0))
        );
        assert_eq!(line_start_byte(source, 3, Point::new(1, 0), 1), Some(3));
    }

    #[test]
    fn same_row_offsets_may_pass_the_end_of_their_line() {
        // Neovim adds the delta to the byte without clamping, so one column past
        // the end addresses the newline. The diff injection queries need it to
        // keep hunk lines apart once they are joined into one injected document.
        let source = b"ab\ncd\n";
        assert_eq!(
            shift_point(source, 2, Point::new(0, 2), 0, 1).unwrap(),
            (3, Point::new(0, 3))
        );
    }

    #[test]
    fn offsets_that_run_off_the_document_are_rejected() {
        let source = b"ab";
        assert_eq!(shift_point(source, 0, Point::new(0, 0), 0, -1), None);
        assert_eq!(shift_point(source, 2, Point::new(0, 2), 0, 5), None);
        assert_eq!(
            shift_point(source, 0, Point::new(0, 0), 0, 99_999_999_999),
            None
        );
        // No such row to walk to.
        assert_eq!(shift_point(source, 0, Point::new(0, 0), 3, 0), None);

        let original = Range {
            start_byte: 0,
            start_point: Point::new(0, 0),
            end_byte: 2,
            end_point: Point::new(0, 2),
        };
        assert_eq!(
            offset_range(original, [0, 99_999_999_999, 0, 0], source),
            original
        );
        assert_eq!(
            offset_range(original, [0, 0, 0, 99_999_999_999], source),
            original
        );
    }

    #[test]
    fn a_same_row_offset_reaches_its_own_newline() {
        let source = b"a\nb";
        let original = Range {
            start_byte: 0,
            start_point: Point::new(0, 0),
            end_byte: 1,
            end_point: Point::new(0, 1),
        };

        assert_eq!(
            offset_range(original, [0, 0, 0, 1], source),
            Range {
                start_byte: 0,
                start_point: Point::new(0, 0),
                end_byte: 2,
                end_point: Point::new(0, 2),
            }
        );
    }
}

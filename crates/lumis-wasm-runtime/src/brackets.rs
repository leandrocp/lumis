//! Rainbow-bracket resolution shared by the CLI and the pooled WASM `Runtime`.
//!
//! Callers own the cache their compiled `Query` lives in; the compile rule
//! itself is [`compile`].

use std::ops::Range;
use std::sync::LazyLock;

use lumis_core::highlights::HIGHLIGHT_NAMES;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Node, Query, QueryCursor};

/// Scope names cycled through by nesting depth.
pub const RAINBOW_BRACKET_SCOPES: [&str; 6] = [
    "punctuation.bracket.rainbow.1",
    "punctuation.bracket.rainbow.2",
    "punctuation.bracket.rainbow.3",
    "punctuation.bracket.rainbow.4",
    "punctuation.bracket.rainbow.5",
    "punctuation.bracket.rainbow.6",
];

/// Resolved once, falling back to `punctuation.bracket` for themes without the
/// rainbow scopes.
pub static RAINBOW_SCOPE_INDICES: LazyLock<[usize; RAINBOW_BRACKET_SCOPES.len()]> =
    LazyLock::new(|| {
        let fallback = HIGHLIGHT_NAMES
            .iter()
            .position(|candidate| *candidate == "punctuation.bracket")
            .unwrap_or(0);
        std::array::from_fn(|index| {
            HIGHLIGHT_NAMES
                .iter()
                .position(|candidate| *candidate == RAINBOW_BRACKET_SCOPES[index])
                .unwrap_or(fallback)
        })
    });

/// A bracket and the highlight index its nesting depth earned it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RainbowRange {
    pub start: usize,
    pub end: usize,
    pub scope_index: usize,
}

/// A matched open/close bracket pair.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BracketPair {
    pub open: Range<usize>,
    pub close: Range<usize>,
}

/// Compile a bracket query, or decide the language has no rainbow brackets.
///
/// A compile failure is not an error: the shared default query deliberately
/// names tokens that some grammars lack, such as `(` in HTML.
#[must_use]
pub fn compile(grammar: &Language, source: &str) -> Option<Query> {
    if source.trim().is_empty() {
        return None;
    }
    Query::new(grammar, source).ok()
}

/// The `@open` and `@close` capture indices, when the query defines both.
#[must_use]
pub fn capture_indices(query: &Query) -> Option<(u32, u32)> {
    let index = |wanted: &str| {
        query
            .capture_names()
            .iter()
            .position(|name| *name == wanted)
            .map(|position| position as u32)
    };
    Some((index("open")?, index("close")?))
}

/// Collect bracket pairs, skipping patterns that carry `(#set! rainbow.exclude)`.
#[must_use]
pub fn bracket_pairs(query: &Query, root: Node<'_>, source: &[u8]) -> Vec<BracketPair> {
    let Some((open_capture, close_capture)) = capture_indices(query) else {
        return Vec::new();
    };

    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(query, root, source);
    let mut pairs = Vec::new();

    while let Some(query_match) = matches.next() {
        if query
            .property_settings(query_match.pattern_index)
            .iter()
            .any(|property| property.key.as_ref() == "rainbow.exclude")
        {
            continue;
        }

        let mut opens = Vec::new();
        let mut closes = Vec::new();
        for capture in query_match.captures {
            if capture.index == open_capture {
                opens.push(capture.node.byte_range());
            } else if capture.index == close_capture {
                closes.push(capture.node.byte_range());
            }
        }

        for (open, close) in opens.into_iter().zip(closes) {
            if open.start < close.end && (open.len() == 1 || close.len() == 1) {
                pairs.push(BracketPair { open, close });
            }
        }
    }

    pairs
}

/// Assign a depth-derived scope to each pair, walking them in closing order.
#[must_use]
pub fn colorize_bracket_pairs(pairs: Vec<BracketPair>) -> Vec<RainbowRange> {
    let mut opens: Vec<_> = pairs.iter().map(|pair| pair.open.clone()).collect();
    opens.sort_by_key(|range| (range.start, range.end));
    opens.dedup_by(|left, right| left.start == right.start && left.end == right.end);

    let mut color_pairs = pairs;
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
            let scope_index =
                RAINBOW_SCOPE_INDICES[(open_stack.len() - 1) % RAINBOW_SCOPE_INDICES.len()];
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

#[cfg(test)]
mod tests {
    use super::*;

    fn pair(open: Range<usize>, close: Range<usize>) -> BracketPair {
        BracketPair { open, close }
    }

    #[test]
    fn nesting_depth_selects_the_scope() {
        // ( [ ] )  -> outer depth 0, inner depth 1
        let ranges = colorize_bracket_pairs(vec![pair(0..1, 5..6), pair(2..3, 3..4)]);
        assert_eq!(ranges.len(), 4);
        assert_eq!(ranges[0].scope_index, RAINBOW_SCOPE_INDICES[0]);
        assert_eq!(ranges[1].scope_index, RAINBOW_SCOPE_INDICES[1]);
        assert_eq!(ranges[3].scope_index, RAINBOW_SCOPE_INDICES[0]);
    }

    #[test]
    fn depth_wraps_around_the_scope_list() {
        // Seven nested pairs: the outermost and the seventh share a scope.
        let pairs: Vec<_> = (0..7).map(|i| pair(i..i + 1, 20 - i..21 - i)).collect();
        let ranges = colorize_bracket_pairs(pairs);
        let opens: Vec<_> = ranges.iter().filter(|r| r.start < 7).collect();
        assert_eq!(opens[0].scope_index, opens[6].scope_index);
    }

    #[test]
    fn an_empty_query_means_no_rainbow_brackets() {
        let grammar: Language = tree_sitter_json::LANGUAGE.into();
        assert!(compile(&grammar, "").is_none());
        assert!(compile(&grammar, "   \n  ").is_none());
    }

    #[test]
    fn a_query_naming_tokens_the_grammar_lacks_means_no_rainbow_brackets() {
        let grammar: Language = tree_sitter_json::LANGUAGE.into();
        assert!(compile(&grammar, "(\"nonexistent_token_xyz\" @open)").is_none());
    }

    #[test]
    fn a_valid_query_compiles() {
        let grammar: Language = tree_sitter_json::LANGUAGE.into();
        assert!(compile(&grammar, "(\"[\" @open \"]\" @close)").is_some());
    }

    #[test]
    fn unmatched_pairs_are_dropped() {
        assert!(
            colorize_bracket_pairs(Vec::new()).is_empty(),
            "no pairs in, so nothing should come out"
        );
    }
}

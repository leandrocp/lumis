//! Rainbow-bracket resolution shared by the CLI and the pooled WASM `Runtime`.
//!
//! Callers own the cache their compiled `Query` lives in; the compile rule
//! itself is [`compile`].

pub use lumis_core::decorations::{RainbowRange, RAINBOW_BRACKET_SCOPES, RAINBOW_SCOPE_INDICES};
use std::ops::{ControlFlow, Range};
use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Node, Query, QueryCursor, QueryCursorOptions};

use crate::tree_sitter_highlight::Interrupt;

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
pub fn bracket_pairs(
    query: &Query,
    root: Node<'_>,
    source: &[u8],
    match_limit: u32,
) -> Vec<BracketPair> {
    bracket_pairs_within(query, root, source, match_limit, Interrupt::none()).pairs
}

/// What one bracket query produced, and whether either limit bound it.
pub struct BracketPairs {
    pub pairs: Vec<BracketPair>,
    /// The cursor dropped matches at the limit, so pairs may be missing.
    pub exceeded_match_limit: bool,
}

/// [`bracket_pairs`], bounded by `interrupt`.
///
/// Rainbow brackets are a second query over the same document, so a render that
/// bounds its highlight and leaves this unbounded is not bounded at all. The
/// same goes for the match limit: this cursor drops matches like any other, and
/// a caller that reports one query's exhaustion and not the other's reports a
/// complete render for a document that lost brackets here.
///
/// The pairs found before the interrupt come back, and every caller here throws
/// them away — a render that ran out has no highlight left to decorate.
#[must_use]
pub fn bracket_pairs_within(
    query: &Query,
    root: Node<'_>,
    source: &[u8],
    match_limit: u32,
    interrupt: Interrupt<'_>,
) -> BracketPairs {
    let Some((open_capture, close_capture)) = capture_indices(query) else {
        return BracketPairs {
            pairs: Vec::new(),
            exceeded_match_limit: false,
        };
    };

    let mut cursor = QueryCursor::new();
    cursor.set_match_limit(match_limit);
    let mut stopped = |_: &tree_sitter::QueryCursorState| {
        if interrupt.stopped().is_some() {
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    };
    let mut matches = cursor.matches_with_options(
        query,
        root,
        source,
        QueryCursorOptions::new().progress_callback(&mut stopped),
    );
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

    drop(matches);
    BracketPairs {
        exceeded_match_limit: cursor.did_exceed_match_limit(),
        pairs,
    }
}

/// Assign the real nesting depth to each pair, walking them in closing order.
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
            let depth = open_stack.len() - 1;
            ranges.push(RainbowRange {
                start: pair.open.start,
                end: pair.open.end,
                depth,
            });
            ranges.push(RainbowRange {
                start: pair.close.start,
                end: pair.close.end,
                depth,
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
    fn nesting_depth_is_preserved() {
        // ( [ ] )  -> outer depth 0, inner depth 1
        let ranges = colorize_bracket_pairs(vec![pair(0..1, 5..6), pair(2..3, 3..4)]);
        assert_eq!(ranges.len(), 4);
        assert_eq!(ranges[0].depth, 0);
        assert_eq!(ranges[1].depth, 1);
        assert_eq!(ranges[3].depth, 0);
    }

    #[test]
    fn depth_does_not_wrap() {
        // Seven nested pairs keep all seven depths; formatters alone cycle
        // those values through the six compatibility scopes.
        let pairs: Vec<_> = (0..7).map(|i| pair(i..i + 1, 20 - i..21 - i)).collect();
        let ranges = colorize_bracket_pairs(pairs);
        let opens: Vec<_> = ranges.iter().filter(|r| r.start < 7).collect();
        assert_eq!(opens[0].depth, 0);
        assert_eq!(opens[6].depth, 6);
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

    /// The bracket cursor drops matches like any other, and says so.
    ///
    /// Without this a rainbow render could lose brackets to the match limit and
    /// still report a complete document, because the caller only ever saw the
    /// highlight query's flag.
    #[test]
    fn a_spent_match_limit_is_reported() {
        let grammar: Language = tree_sitter_json::LANGUAGE.into();
        let query = compile(&grammar, "(\"[\" @open \"]\" @close)").unwrap();
        let source = "[[[[[[[[1]]]]]]]]";
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&grammar).unwrap();
        let tree = parser.parse(source, None).unwrap();

        let generous = bracket_pairs_within(
            &query,
            tree.root_node(),
            source.as_bytes(),
            4096,
            Interrupt::none(),
        );
        let starved = bracket_pairs_within(
            &query,
            tree.root_node(),
            source.as_bytes(),
            1,
            Interrupt::none(),
        );

        assert!(
            !generous.exceeded_match_limit,
            "a limit nothing reaches is not exhausted"
        );
        assert!(
            starved.exceeded_match_limit,
            "a limit of one in-progress match is exhausted by eight nested pairs"
        );
        assert!(
            starved.pairs.len() < generous.pairs.len(),
            "the starved cursor lost pairs: {} vs {}",
            starved.pairs.len(),
            generous.pairs.len()
        );
    }

    #[test]
    fn unmatched_pairs_are_dropped() {
        assert!(
            colorize_bracket_pairs(Vec::new()).is_empty(),
            "no pairs in, so nothing should come out"
        );
    }
}

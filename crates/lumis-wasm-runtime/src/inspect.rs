//! What a highlight saw, for tools that show it: every named syntax node of every
//! layer, injected layers nested under the node that owns them, and every
//! highlight capture attached to the smallest node that contains it.
//!
//! `lumis dump tree` prints this, and the inspector draws it, so both report the
//! same tree and the same captures.

use std::collections::HashMap;
use std::ops::Range;

use lumis_core::events::HighlightEvent;
use lumis_core::highlights::HIGHLIGHT_NAMES;
use thiserror::Error;
use tree_sitter::{Node, Point};

use crate::tree_sitter_highlight::ParsedLayer;

/// A highlight's tree and captures, flattened in document order.
#[derive(Clone, Debug, Default)]
pub struct Inspection {
    /// Named nodes in pre-order. A layer injected under a node follows that
    /// node, before the node's own children.
    pub nodes: Vec<SyntaxNode>,
    /// Captures in the order the highlight opened them.
    pub captures: Vec<Capture>,
}

/// One named node.
#[derive(Clone, Debug)]
pub struct SyntaxNode {
    pub kind: String,
    /// The field the parent holds this node in, if any.
    pub field: Option<String>,
    /// The language of the layer the node belongs to.
    pub language: String,
    /// Index into [`ParsedLayer`]s the inspection was built from.
    pub layer: usize,
    /// Nesting depth, counting injected layers as one level each.
    pub depth: usize,
    pub parent: Option<usize>,
    pub byte_range: Range<usize>,
    pub start: Point,
    pub end: Point,
    pub is_missing: bool,
    pub is_error: bool,
    /// Indices into [`Inspection::captures`], by start byte.
    pub captures: Vec<usize>,
}

/// One highlight capture.
#[derive(Clone, Debug)]
pub struct Capture {
    /// Index into [`HIGHLIGHT_NAMES`].
    pub scope_index: usize,
    /// The scope, e.g. `keyword.function`.
    pub scope: &'static str,
    pub language: String,
    pub byte_range: Range<usize>,
    pub start: Point,
    pub end: Point,
    /// The smallest node of the capture's language that contains it.
    pub node: usize,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum InspectError {
    #[error("highlight event stream contains an unmatched end event")]
    UnmatchedEnd,
    #[error("highlight event stream contains unclosed start events")]
    Unclosed,
    #[error("no {language} syntax node contains highlight capture {start}..{end}")]
    Unowned {
        language: String,
        start: usize,
        end: usize,
    },
}

impl Inspection {
    /// Build from the layers a highlight kept (`HighlightOptions::layers`) and
    /// its events. Pass no events for the tree alone.
    pub fn new(
        source: &str,
        layers: &[ParsedLayer],
        events: &[HighlightEvent<'_>],
    ) -> Result<Self, InspectError> {
        let injections = injections(layers);
        let mut nodes = Vec::new();
        let mut ids = HashMap::new();
        for (index, layer) in layers.iter().enumerate().filter(|(_, l)| l.depth == 0) {
            let mut walk = Walk {
                layers,
                injections: &injections,
                nodes: &mut nodes,
                ids: &mut ids,
            };
            walk.collect(index, layer.tree.root_node(), 0, None, None);
        }
        let mut by_language = HashMap::<&str, Vec<usize>>::new();
        for (index, layer) in layers.iter().enumerate() {
            by_language.entry(&layer.language).or_default().push(index);
        }
        let lines = line_starts(source);
        let mut captures = Vec::new();
        for (scope_index, language, byte_range) in spans(events)? {
            let candidates = by_language
                .get(language.as_str())
                .map_or(&[][..], Vec::as_slice);
            let node = owner(layers, candidates, &nodes, &ids, &byte_range).ok_or_else(|| {
                InspectError::Unowned {
                    language: language.clone(),
                    start: byte_range.start,
                    end: byte_range.end,
                }
            })?;
            nodes[node].captures.push(captures.len());
            captures.push(Capture {
                scope_index,
                scope: HIGHLIGHT_NAMES[scope_index],
                language,
                start: point(&lines, byte_range.start),
                end: point(&lines, byte_range.end),
                byte_range,
                node,
            });
        }
        for node in &mut nodes {
            node.captures.sort_by_key(|&c| captures[c].byte_range.start);
        }
        Ok(Self { nodes, captures })
    }

    /// The deepest node containing `byte`, preferring the innermost layer.
    #[must_use]
    pub fn node_at(&self, byte: usize) -> Option<usize> {
        self.nodes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.byte_range.start <= byte && byte < n.byte_range.end)
            .max_by_key(|(_, n)| n.depth)
            .map(|(i, _)| i)
    }

    /// Captures containing `byte`, outermost first: the last one is the one a
    /// renderer paints with.
    #[must_use]
    pub fn captures_at(&self, byte: usize) -> Vec<usize> {
        let mut out: Vec<usize> = (0..self.captures.len())
            .filter(|&i| {
                let r = &self.captures[i].byte_range;
                r.start <= byte && byte < r.end
            })
            .collect();
        out.sort_by_key(|&i| self.nodes[self.captures[i].node].depth);
        out
    }

    /// `node` and its ancestors, root first.
    #[must_use]
    pub fn ancestors(&self, node: usize) -> Vec<usize> {
        let mut out = vec![node];
        while let Some(parent) = self.nodes[*out.last().unwrap_or(&node)].parent {
            out.push(parent);
        }
        out.reverse();
        out
    }

    /// The index one past `node`'s last descendant.
    #[must_use]
    pub fn subtree_end(&self, node: usize) -> usize {
        let depth = self.nodes[node].depth;
        self.nodes[node + 1..]
            .iter()
            .position(|n| n.depth <= depth)
            .map_or(self.nodes.len(), |offset| node + 1 + offset)
    }
}

/// For each node that has injected layers, those layers, largest first.
fn injections(layers: &[ParsedLayer]) -> HashMap<(usize, usize), Vec<usize>> {
    let roots: Vec<Node<'_>> = layers.iter().map(|l| l.tree.root_node()).collect();
    let mut by_depth = HashMap::<usize, Vec<usize>>::new();
    for (index, layer) in layers.iter().enumerate() {
        by_depth.entry(layer.depth).or_default().push(index);
    }
    let mut out = HashMap::<(usize, usize), Vec<usize>>::new();
    for (child_index, child) in layers.iter().enumerate() {
        let child_root = roots[child_index];
        let Some(candidates) = child.depth.checked_sub(1).and_then(|d| by_depth.get(&d)) else {
            continue;
        };
        let parent = candidates
            .iter()
            .filter(|&&p| {
                roots[p].start_byte() <= child_root.start_byte()
                    && roots[p].end_byte() >= child_root.end_byte()
            })
            .min_by_key(|&&p| roots[p].byte_range().len());
        if let Some(&parent_index) = parent {
            let parent_root = roots[parent_index];
            let owner = parent_root
                .named_descendant_for_byte_range(child_root.start_byte(), child_root.end_byte())
                .unwrap_or(parent_root);
            out.entry((parent_index, owner.id()))
                .or_default()
                .push(child_index);
        }
    }
    for children in out.values_mut() {
        children.sort_by_key(|&index| std::cmp::Reverse(roots[index].byte_range().len()));
    }
    out
}

struct Walk<'a> {
    layers: &'a [ParsedLayer],
    injections: &'a HashMap<(usize, usize), Vec<usize>>,
    nodes: &'a mut Vec<SyntaxNode>,
    /// `(layer, tree-sitter node id)` to index in `nodes`.
    ids: &'a mut HashMap<(usize, usize), usize>,
}

impl Walk<'_> {
    fn collect(
        &mut self,
        layer: usize,
        node: Node<'_>,
        depth: usize,
        field: Option<String>,
        parent: Option<usize>,
    ) {
        let index = self.nodes.len();
        self.ids.insert((layer, node.id()), index);
        self.nodes.push(SyntaxNode {
            kind: node.kind().to_string(),
            field,
            language: self.layers[layer].language.clone(),
            layer,
            depth,
            parent,
            byte_range: node.byte_range(),
            start: node.start_position(),
            end: node.end_position(),
            is_missing: node.is_missing(),
            is_error: node.is_error(),
            captures: Vec::new(),
        });
        if let Some(children) = self.injections.get(&(layer, node.id())) {
            for &child in children {
                let root = self.layers[child].tree.root_node();
                self.collect(child, root, depth + 1, None, Some(index));
            }
        }
        let mut cursor = node.walk();
        if !cursor.goto_first_child() {
            return;
        }
        loop {
            let child = cursor.node();
            if child.is_named() {
                let field = cursor.field_name().map(str::to_string);
                self.collect(layer, child, depth + 1, field, Some(index));
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

/// Each capture's scope, language and the bytes from its first source to its
/// last, in the order the highlight opened them, without duplicates.
fn spans(
    events: &[HighlightEvent<'_>],
) -> Result<Vec<(usize, String, Range<usize>)>, InspectError> {
    struct Open {
        scope_index: usize,
        language: String,
        range: Option<Range<usize>>,
        order: usize,
    }
    let mut open: Vec<Open> = Vec::new();
    let mut done: Vec<(usize, usize, String, Range<usize>)> = Vec::new();
    let mut order = 0;
    for event in events {
        match event {
            HighlightEvent::Start {
                scope_index,
                language,
            } => {
                open.push(Open {
                    scope_index: *scope_index,
                    language: language.clone(),
                    range: None,
                    order,
                });
                order += 1;
            }
            HighlightEvent::Source { start, end } => {
                for o in &mut open {
                    let from = o.range.as_ref().map_or(*start, |r| r.start);
                    o.range = Some(from..*end);
                }
            }
            HighlightEvent::End => {
                let o = open.pop().ok_or(InspectError::UnmatchedEnd)?;
                if let Some(range) = o.range {
                    done.push((o.order, o.scope_index, o.language, range));
                }
            }
            _ => {}
        }
    }
    if !open.is_empty() {
        return Err(InspectError::Unclosed);
    }
    done.sort_by_key(|d| d.0);
    done.dedup_by(|a, b| a.1 == b.1 && a.2 == b.2 && a.3 == b.3);
    Ok(done.into_iter().map(|(_, s, l, r)| (s, l, r)).collect())
}

/// The smallest node containing `range` among the `candidates` layers, the
/// deepest on a tie. A layer whose root does not contain `range` is skipped
/// without descending, so a capture costs one comparison per layer of its
/// language and one descent per layer that encloses it.
fn owner(
    layers: &[ParsedLayer],
    candidates: &[usize],
    nodes: &[SyntaxNode],
    ids: &HashMap<(usize, usize), usize>,
    range: &Range<usize>,
) -> Option<usize> {
    candidates
        .iter()
        .filter_map(|&index| {
            let root = layers[index].tree.root_node();
            if root.start_byte() > range.start || root.end_byte() < range.end {
                return None;
            }
            let node = root.named_descendant_for_byte_range(range.start, range.end)?;
            let contains = node.start_byte() <= range.start && node.end_byte() >= range.end;
            contains
                .then(|| ids.get(&(index, node.id())).copied())
                .flatten()
        })
        .min_by_key(|&n| (nodes[n].byte_range.len(), std::cmp::Reverse(nodes[n].depth)))
}

fn line_starts(source: &str) -> Vec<usize> {
    std::iter::once(0)
        .chain(
            source
                .bytes()
                .enumerate()
                .filter_map(|(i, b)| (b == b'\n').then_some(i + 1)),
        )
        .collect()
}

fn point(lines: &[usize], byte: usize) -> Point {
    let row = lines.partition_point(|&s| s <= byte) - 1;
    Point::new(row, byte - lines[row])
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::Parser;

    fn layer(source: &str, ranges: Option<tree_sitter::Range>, depth: usize) -> ParsedLayer {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_json::LANGUAGE.into())
            .unwrap();
        if let Some(range) = ranges {
            parser.set_included_ranges(&[range]).unwrap();
        }
        let tree = parser.parse(source, None).unwrap();
        ParsedLayer {
            ranges: vec![tree.root_node().range()],
            tree,
            language: if depth == 0 { "json" } else { "inner" }.to_string(),
            depth,
        }
    }

    fn scope(name: &str) -> usize {
        HIGHLIGHT_NAMES.iter().position(|s| *s == name).unwrap()
    }

    #[test]
    fn nests_injected_layers_under_their_owner_and_attaches_captures_to_the_smallest_node() {
        // The string's content, `[1, 2]`, is parsed again as an injected layer.
        let source = r#"{"k": "[1, 2]"}"#;
        let inner_start = source.find('[').unwrap();
        let inner_end = source.find(']').unwrap() + 1;
        let root = layer(source, None, 0);
        let inner_range = tree_sitter::Range {
            start_byte: inner_start,
            end_byte: inner_end,
            start_point: Point::new(0, inner_start),
            end_point: Point::new(0, inner_end),
        };
        let inner = layer(source, Some(inner_range), 1);
        let string = scope("string");
        let number = scope("number");
        let events = [
            HighlightEvent::Start {
                scope_index: string,
                language: "json".into(),
            },
            HighlightEvent::Source {
                start: inner_start - 1,
                end: inner_end + 1,
            },
            HighlightEvent::Start {
                scope_index: number,
                language: "inner".into(),
            },
            HighlightEvent::Source {
                start: inner_start + 1,
                end: inner_start + 2,
            },
            HighlightEvent::End,
            HighlightEvent::End,
        ];

        let inspection = Inspection::new(source, &[root, inner], &events).unwrap();

        let kinds: Vec<(&str, usize, &str)> = inspection
            .nodes
            .iter()
            .map(|n| (n.kind.as_str(), n.depth, n.language.as_str()))
            .collect();
        let owner = kinds
            .iter()
            .rposition(|k| *k == ("string_content", 4, "json"))
            .unwrap();
        assert_eq!(
            kinds[owner + 1],
            ("document", 5, "inner"),
            "the injected layer follows the node that owns it, before its children: {kinds:?}"
        );

        let one = inspection
            .captures
            .iter()
            .find(|c| c.scope_index == number)
            .unwrap();
        assert_eq!(inspection.nodes[one.node].kind, "number");
        assert_eq!(inspection.nodes[one.node].language, "inner");
        assert_eq!(one.start, Point::new(0, inner_start + 1));

        let at = inspection.captures_at(inner_start + 1);
        let scopes: Vec<&str> = at.iter().map(|&c| inspection.captures[c].scope).collect();
        assert_eq!(scopes, ["string", "number"], "outermost first");
        let deepest = inspection.node_at(inner_start + 1).unwrap();
        assert_eq!(inspection.nodes[deepest].kind, "number");
        assert_eq!(
            inspection
                .ancestors(deepest)
                .iter()
                .map(|&n| inspection.nodes[n].kind.as_str())
                .collect::<Vec<_>>(),
            [
                "document",
                "object",
                "pair",
                "string",
                "string_content",
                "document",
                "array",
                "number"
            ]
        );
    }

    #[test]
    fn lookups_treat_ranges_as_half_open() {
        let source = "[1, [22], 3]";
        let two = source.find("22").unwrap();
        let events = [
            HighlightEvent::Start {
                scope_index: scope("number"),
                language: "json".into(),
            },
            HighlightEvent::Source {
                start: two,
                end: two + 2,
            },
            HighlightEvent::End,
        ];
        let inspection = Inspection::new(source, &[layer(source, None, 0)], &events).unwrap();
        let around = [two - 1, two, two + 1, two + 2, source.len()];
        let nodes: Vec<Option<&str>> = around
            .iter()
            .map(|&byte| {
                inspection
                    .node_at(byte)
                    .map(|n| inspection.nodes[n].kind.as_str())
            })
            .collect();
        let captures: Vec<usize> = around
            .iter()
            .map(|&byte| inspection.captures_at(byte).len())
            .collect();
        assert_eq!(
            nodes,
            [
                Some("array"),
                Some("number"),
                Some("number"),
                Some("array"),
                None
            ],
            "a range holds its start byte and not its end byte"
        );
        assert_eq!(captures, [0, 1, 1, 0, 0]);

        let kinds: Vec<&str> = inspection.nodes.iter().map(|n| n.kind.as_str()).collect();
        assert_eq!(
            kinds,
            ["document", "array", "number", "array", "number", "number"]
        );
        let ends: Vec<usize> = (0..kinds.len())
            .map(|n| inspection.subtree_end(n))
            .collect();
        assert_eq!(ends, [6, 6, 3, 5, 5, 6]);
    }

    #[test]
    fn rejects_a_capture_no_node_of_its_language_contains() {
        let source = "[1]";
        let events = [
            HighlightEvent::Start {
                scope_index: scope("number"),
                language: "other".into(),
            },
            HighlightEvent::Source { start: 1, end: 2 },
            HighlightEvent::End,
        ];
        let error = Inspection::new(source, &[layer(source, None, 0)], &events).unwrap_err();
        assert_eq!(
            error,
            InspectError::Unowned {
                language: "other".into(),
                start: 1,
                end: 2
            }
        );
    }
}

//! The editable source, painted the way Lumis paints it, with a status bar under it.

use rune::text::TextHighlight;
use rune::{Align, Color, El, Span, col, rich, row, text};
use rune_kit::{Kit, Spinner};

use crate::{Inspector, style};

pub(crate) const FONT_SIZE: f32 = 13.0;
const LINE_HEIGHT: f32 = 20.0;
/// IBM Plex Mono advances 600 units per 1000 em.
const ADVANCE: f32 = 0.6 * FONT_SIZE;

/// Spans of the last analysis under the last theme, which only change when either does.
#[derive(Default)]
pub(crate) struct Paint {
    key: Option<(u64, usize)>,
    spans: Vec<Span>,
}

fn spans(s: &Inspector) -> Vec<Span> {
    let Some(analysis) = s.current() else {
        return Vec::new();
    };
    let key = (analysis.id, s.theme);
    let mut paint = s.paint.borrow_mut();
    if paint.key != Some(key) {
        let theme = s.theme();
        paint.spans = analysis
            .runs
            .iter()
            .filter(|run| !run.scopes.is_empty())
            .map(|run| style::span(theme, run.bytes.clone(), &run.scopes))
            .collect();
        paint.key = Some(key);
    }
    let text = &s.edit.text;
    paint
        .spans
        .iter()
        .filter(|span| {
            let (start, end) = (span.start as usize, span.end as usize);
            end <= text.len() && text.is_char_boundary(start) && text.is_char_boundary(end)
        })
        .cloned()
        .collect()
}

pub(crate) fn mono(k: &Kit) -> rune::TextStyle {
    let mut mono = k.ty.mono;
    mono.size = FONT_SIZE;
    mono.line_height = LINE_HEIGHT;
    mono
}

/// A count as a length in points, saturating far past any window.
pub(crate) fn points(count: usize) -> f32 {
    f32::from(u16::try_from(count).unwrap_or(u16::MAX))
}

/// Line and column of `byte`, both from 1, the column in characters.
pub(crate) fn position(text: &str, byte: usize) -> (usize, usize) {
    let before = &text[..byte.min(text.len())];
    let line_start = before.rfind('\n').map_or(0, |i| i + 1);
    (
        before.matches('\n').count() + 1,
        before[line_start..].chars().count() + 1,
    )
}

fn gutter(s: &Inspector, k: &Kit, lines: usize, caret_line: usize) -> El {
    let theme = s.theme();
    let number = theme.get_style("line_number");
    let current = theme.get_style("line_number.highlighted");
    let muted = style::color(number.and_then(|n| n.fg.as_deref())).unwrap_or(k.c.text_faint);
    let bright = style::color(current.and_then(|n| n.fg.as_deref())).unwrap_or(k.c.text_base);
    let numbers = (1..=lines)
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join("\n");
    let start: usize = (1..caret_line).map(|n| n.to_string().len() + 1).sum();
    let width = points(caret_line.max(lines).to_string().len()) * ADVANCE;
    rich(
        numbers,
        vec![Span::new(start, start + caret_line.to_string().len()).color(bright)],
    )
    .font(mono(k))
    .color(muted)
    .align(Align::End)
    .w(width)
    .pr(16.0)
}

fn field(s: &Inspector, k: &Kit, fg: Color) -> El {
    let longest = s
        .edit
        .text
        .lines()
        .map(|l| l.chars().count())
        .max()
        .unwrap_or(0);
    let mut field = rune::text_input(&s.edit, mono(k))
        .on_edit(|s: &mut Inspector| &mut s.edit)
        .color(fg)
        .caret(k.c.text_base, k.c.text_accent.alpha(0.3))
        .spans(spans(s))
        .min_w(points(longest) * ADVANCE + 32.0)
        .grow(1.0)
        .key("source");
    if let Some(analysis) = s.current() {
        let nodes = &analysis.inspection.nodes;
        let len = s.edit.text.len();
        let clamp = |r: &std::ops::Range<usize>| r.start.min(len)..r.end.min(len);
        let caret = s.caret();
        if let Some(node) = s.node() {
            field = field.text_highlight(TextHighlight::new(
                clamp(&nodes[node].byte_range),
                k.c.text_accent.alpha(0.08),
            ));
        }
        if let Some(&capture) = analysis.inspection.captures_at(caret).last() {
            field = field.text_highlight(TextHighlight::new(
                clamp(&analysis.inspection.captures[capture].byte_range),
                k.c.text_accent.alpha(0.24),
            ));
        }
        if let Some(node) = s.hovered.filter(|&n| n < nodes.len()) {
            field = field.text_highlight(TextHighlight::new(
                clamp(&nodes[node].byte_range),
                k.c.text_accent.alpha(0.32),
            ));
        }
    }
    field
}

fn status(s: &Inspector, k: &Kit, caret: (usize, usize)) -> El {
    let item = |t: String| text(t).font(k.ty.small).color(k.c.text_muted);
    let mut bar = row()
        .key("status")
        .h(30.0)
        .px(14.0)
        .gap(16.0)
        .items_center()
        .fill(k.c.layer_01)
        .child(
            text(s.language().to_string())
                .font(k.ty.small_medium)
                .color(k.c.text_base),
        )
        .child(item(format!("Ln {}, Col {}", caret.0, caret.1)))
        .child(item(format!("byte {}", s.caret())));
    if let Some(analysis) = s.current() {
        let inspection = &analysis.inspection;
        let errors = inspection
            .nodes
            .iter()
            .filter(|n| n.is_error || n.is_missing)
            .count();
        bar = bar
            .child(item(format!("{} nodes", inspection.nodes.len())))
            .child(item(format!("{} captures", inspection.captures.len())))
            .child(item(format!(
                "highlighted in {:.1} ms",
                analysis.elapsed.as_secs_f64() * 1000.0
            )));
        if errors > 0 {
            bar = bar.child(
                text(format!("{errors} syntax errors"))
                    .font(k.ty.small_medium)
                    .color(k.c.danger.fg),
            );
        }
        if !analysis.unresolved.is_empty() {
            bar = bar.child(
                text(format!("not loaded: {}", analysis.unresolved.join(", ")))
                    .font(k.ty.small_medium)
                    .color(k.c.warning.fg),
            );
        }
    }
    if s.pending() {
        bar = bar.child(
            row()
                .gap(6.0)
                .items_center()
                .child(Spinner::new().build(k))
                .child(item("Highlighting…".into())),
        );
    }
    if let Some(error) = &s.error {
        bar = bar.child(
            text(error.clone())
                .font(k.ty.small_medium)
                .color(k.c.danger.fg)
                .truncate(),
        );
    }
    bar
}

pub(crate) fn build(s: &Inspector, k: &Kit) -> El {
    let theme = s.theme();
    let bg = style::color(theme.bg()).unwrap_or(k.c.bg_base);
    let fg = style::color(theme.fg()).unwrap_or(k.c.text_base);
    let caret = position(&s.edit.text, s.caret());
    let lines = s.edit.text.lines().count().max(1);
    let code = row()
        .items_start()
        .padding(20.0)
        .child(gutter(s, k, lines, caret.0))
        .child(field(s, k, fg));
    col()
        .key("editor")
        .w_full()
        .h_full()
        .fill(bg)
        .child(col().key("code").grow(1.0).scroll_xy().child(code))
        .child(status(s, k, caret))
}

//! The inspector pane: what is at the caret, then the whole tree.

use std::borrow::Cow;

use lumis_core::themes::{Style, contrast_ratio};
use lumis_wasm_runtime::inspect::{Inspection, SyntaxNode};
use rune::{Color, El, GrowthSpec, ScrollSpec, col, el, row, text};
use rune_kit::Kit;

use crate::Inspector;
use crate::analysis::Analysis;
use crate::editor::{points, position};
use crate::style::{self, Effective};

pub(crate) fn row_key(analysis: u64, node: usize) -> String {
    format!("node-{analysis}-{node}")
}

pub(crate) fn build(s: &Inspector, k: &'static Kit) -> El {
    let Some(analysis) = s.current() else {
        return col()
            .key("inspector")
            .h_full()
            .padding(20.0)
            .child(muted(k, empty_message(s)));
    };
    col()
        .key("inspector")
        .h_full()
        .child(
            col()
                .key("details")
                .grow(3.0)
                .min_h(0.0)
                .scroll(ScrollSpec {
                    stick_bottom: false,
                })
                .scroll_edge_fade(24.0)
                .child(details(s, k, analysis)),
        )
        .child(el().h(1.0).w_full().fill(k.c.border_muted))
        .child(tree(analysis, k))
}

fn empty_message(s: &Inspector) -> String {
    if let Some(error) = &s.error {
        format!("Nothing to inspect: {error}")
    } else if s.pending() {
        "Highlighting…".to_string()
    } else {
        "Nothing to inspect".to_string()
    }
}

fn muted(k: &Kit, t: impl Into<String>) -> El {
    text(t.into()).font(k.ty.small).color(k.c.text_muted).wrap()
}

fn heading(k: &Kit, title: &str) -> El {
    text(title.to_string())
        .font(k.ty.small_medium)
        .color(k.c.text_muted)
        .pt(14.0)
        .pb(6.0)
}

fn mono(k: &Kit, t: impl Into<String>, color: Color) -> El {
    let mut style = k.ty.mono;
    style.size = 12.0;
    text(t.into()).font(style).color(color)
}

fn fact(k: &Kit, label: &str, value: El) -> El {
    row()
        .gap(12.0)
        .items_center()
        .min_h(22.0)
        .child(
            text(label.to_string())
                .font(k.ty.small)
                .color(k.c.text_faint)
                .w(84.0),
        )
        .child(value)
}

fn details(s: &Inspector, k: &'static Kit, analysis: &Analysis) -> El {
    let caret = s.caret();
    let inspection = &analysis.inspection;
    let captures = inspection.captures_at(caret);
    let scopes = analysis
        .runs
        .iter()
        .find(|run| run.bytes.contains(&caret))
        .map_or(&[][..], |run| run.scopes.as_slice());
    let mut out = col()
        .px(16.0)
        .pb(16.0)
        .child(heading(k, "Cursor"))
        .child(cursor(s, k, caret))
        .child(heading(k, &format!("Captures ({})", captures.len())));
    if captures.is_empty() {
        out = out.child(muted(k, "No capture covers this byte."));
    }
    for &capture in captures.iter().rev() {
        out = out.child(capture_card(s, k, inspection, capture));
    }
    out = out
        .child(heading(k, "Painted style"))
        .child(effective_style(s, k, scopes));
    if let Some(node) = s.node() {
        out = out
            .child(heading(k, "Node"))
            .child(breadcrumb(inspection, k, node))
            .child(node_facts(s, k, &inspection.nodes[node]));
    }
    out
}

fn cursor(s: &Inspector, k: &Kit, caret: usize) -> El {
    let (line, column) = position(&s.edit.text, caret);
    let character = s.edit.text[caret..].chars().next().map_or_else(
        || "end of file".to_string(),
        |c| match c {
            '\n' => "newline".to_string(),
            '\t' => "tab".to_string(),
            ' ' => "space".to_string(),
            c => format!("'{c}' U+{:04X}", c as u32),
        },
    );
    row()
        .gap(16.0)
        .child(mono(k, format!("{line}:{column}"), k.c.text_base))
        .child(mono(k, format!("byte {caret}"), k.c.text_muted))
        .child(mono(k, character, k.c.text_muted))
}

fn breadcrumb(inspection: &Inspection, k: &Kit, node: usize) -> El {
    let ancestors = inspection.ancestors(node);
    let mut crumbs = row().flex_wrap().gap(4.0).items_center().pb(8.0);
    for (n, &index) in ancestors.iter().enumerate() {
        let current = &inspection.nodes[index];
        let injected = current
            .parent
            .is_some_and(|p| inspection.nodes[p].language != current.language);
        if n > 0 {
            crumbs = crumbs.child(text("›").font(k.ty.small).color(k.c.text_faint));
        }
        let color = if index == node {
            k.c.text_base
        } else {
            k.c.text_muted
        };
        let mut label = current.kind.clone();
        if injected {
            label = format!("{} ({})", current.kind, current.language);
        }
        crumbs = crumbs.child(
            mono(
                k,
                label,
                if current.is_error {
                    k.c.danger.fg
                } else {
                    color
                },
            )
            .px(5.0)
            .py(1.0)
            .radius(4.0)
            .fill(if index == node {
                k.c.layer_02
            } else {
                Color::TRANSPARENT
            })
            .on_hover_change(move |s: &mut Inspector, on, _| hover(s, index, on))
            .on_press(move |s: &mut Inspector| s.pick(index)),
        );
    }
    crumbs
}

fn hover(s: &mut Inspector, node: usize, on: bool) {
    if on {
        s.hovered = Some(node);
    } else if s.hovered == Some(node) {
        s.hovered = None;
    }
}

fn node_facts(s: &Inspector, k: &Kit, node: &SyntaxNode) -> El {
    let base = k.c.text_base;
    let range = format!(
        "{}:{} – {}:{}",
        node.start.row + 1,
        node.start.column + 1,
        node.end.row + 1,
        node.end.column + 1
    );
    let bytes = format!(
        "{}..{}  ({} bytes)",
        node.byte_range.start,
        node.byte_range.end,
        node.byte_range.len()
    );
    let mut facts = col()
        .child(fact(k, "Kind", mono(k, node.kind.clone(), base)))
        .child(fact(
            k,
            "Field",
            mono(k, node.field.clone().unwrap_or_else(|| "—".into()), base),
        ))
        .child(fact(k, "Language", mono(k, node.language.clone(), base)))
        .child(fact(k, "Range", mono(k, range, base)))
        .child(fact(k, "Bytes", mono(k, bytes, base)));
    if node.is_error || node.is_missing {
        let flag = if node.is_error { "ERROR" } else { "MISSING" };
        facts = facts.child(fact(k, "Parse", mono(k, flag, k.c.danger.fg)));
    }
    let source = s.edit.text.get(node.byte_range.clone()).unwrap_or_default();
    facts.child(fact(
        k,
        "Text",
        mono(k, preview(source), k.c.text_muted).truncate(),
    ))
}

fn preview(source: &str) -> String {
    let flat: String = source
        .chars()
        .take(80)
        .map(|c| if c == '\n' { '⏎' } else { c })
        .collect();
    if source.chars().count() > 80 {
        format!("{flat}…")
    } else {
        flat
    }
}

fn swatch(k: &Kit, hex: Option<&str>) -> El {
    let fill = style::color(hex).unwrap_or(Color::TRANSPARENT);
    el().size(12.0, 12.0)
        .radius(3.0)
        .fill(fill)
        .border(1.0, k.c.border_base)
}

fn color_value(k: &Kit, hex: Option<&str>, from: Option<String>) -> El {
    let mut value = row()
        .gap(8.0)
        .items_center()
        .child(swatch(k, hex))
        .child(mono(k, hex.unwrap_or("—").to_string(), k.c.text_base));
    if let Some(from) = from {
        value = value.child(mono(k, format!("from {from}"), k.c.text_faint));
    }
    value
}

fn flag_value(k: &Kit, set: bool, from: Option<String>) -> El {
    let label = match (set, from) {
        (true, Some(from)) => format!("yes  from {from}"),
        (true, None) => "yes".to_string(),
        (false, _) => "no".to_string(),
    };
    mono(k, label, if set { k.c.text_base } else { k.c.text_faint })
}

fn effective_style(s: &Inspector, k: &Kit, scopes: &[(usize, String)]) -> El {
    let theme = s.theme();
    let Effective {
        style:
            Style {
                fg,
                bg,
                bold,
                italic,
                text_decoration,
            },
        fg: fg_from,
        bg: bg_from,
        bold: bold_from,
        italic: italic_from,
        underline: underline_from,
        strikethrough: strike_from,
    } = style::effective(theme, scopes);
    let name = |index: Option<usize>| {
        index.map(|i| format!("@{}", lumis_core::highlights::HIGHLIGHT_NAMES[scopes[i].0]))
    };
    let normal =
        |hex: Option<String>, normal: Option<&str>| hex.or_else(|| normal.map(str::to_string));
    let (fg_shown, fg_origin) = match &fg {
        Some(_) => (fg.clone(), name(fg_from)),
        None => (normal(None, theme.fg()), Some("normal".to_string())),
    };
    let (bg_shown, bg_origin) = match &bg {
        Some(_) => (bg.clone(), name(bg_from)),
        None => (normal(None, theme.bg()), Some("normal".to_string())),
    };
    let underline = text_decoration.underline != lumis_core::themes::UnderlineStyle::None;
    col()
        .child(fact(
            k,
            "Foreground",
            color_value(k, fg_shown.as_deref(), fg_origin),
        ))
        .child(fact(
            k,
            "Background",
            color_value(k, bg_shown.as_deref(), bg_origin),
        ))
        .child(fact(k, "Bold", flag_value(k, bold, name(bold_from))))
        .child(fact(k, "Italic", flag_value(k, italic, name(italic_from))))
        .child(fact(
            k,
            "Underline",
            flag_value(k, underline, name(underline_from)),
        ))
        .child(fact(
            k,
            "Strike",
            flag_value(k, text_decoration.strikethrough, name(strike_from)),
        ))
        .child(fact(
            k,
            "Contrast",
            contrast(k, fg_shown.as_deref(), bg_shown.as_deref()),
        ))
}

fn contrast(k: &Kit, fg: Option<&str>, bg: Option<&str>) -> El {
    let Some(ratio) = fg.zip(bg).and_then(|(fg, bg)| contrast_ratio(fg, bg)) else {
        return mono(k, "—", k.c.text_faint);
    };
    let (grade, color) = match ratio {
        r if r >= 7.0 => ("AAA", k.c.success.fg),
        r if r >= 4.5 => ("AA", k.c.success.fg),
        r if r >= 3.0 => ("AA large", k.c.warning.fg),
        _ => ("fails", k.c.danger.fg),
    };
    row()
        .gap(8.0)
        .items_center()
        .child(mono(k, format!("{ratio:.2}:1"), k.c.text_base))
        .child(
            text(grade.to_string())
                .font(k.ty.text_xsmall_medium)
                .color(color)
                .px(6.0)
                .py(1.0)
                .radius(4.0)
                .fill(color.alpha(0.14)),
        )
}

fn capture_card(s: &Inspector, k: &Kit, inspection: &Inspection, capture: usize) -> El {
    let theme = s.theme();
    let c = &inspection.captures[capture];
    let own = style::own(theme, c.scope_index, &c.language);
    let range = format!(
        "{}:{} – {}:{}",
        c.start.row + 1,
        c.start.column + 1,
        c.end.row + 1,
        c.end.column + 1
    );
    let scope_color = own
        .style
        .as_ref()
        .and_then(|st| style::color(st.fg.as_deref()))
        .unwrap_or(k.c.text_base);
    let editor_bg = style::color(theme.bg()).unwrap_or(k.c.bg_base);
    let title = row()
        .gap(8.0)
        .items_center()
        .child(
            mono(k, format!("@{}", c.scope), scope_color)
                .px(6.0)
                .py(2.0)
                .radius(4.0)
                .fill(editor_bg),
        )
        .child(mono(k, c.language.clone(), k.c.text_faint))
        .child(rune::spacer())
        .child(mono(k, range, k.c.text_faint));
    let theme_key = match &own.matched {
        Some(matched) => mono(k, matched.clone(), k.c.text_base),
        None => mono(k, "unstyled", k.c.text_faint),
    };
    let mut card = col()
        .gap(2.0)
        .padding(10.0)
        .mb(8.0)
        .radius(8.0)
        .fill(k.c.layer_01)
        .border(1.0, k.c.border_muted)
        .child(title.pb(4.0))
        .child(fact(k, "Theme key", theme_key))
        .child(fact(
            k,
            "Looked up",
            mono(k, own.query.clone(), k.c.text_faint),
        ));
    if let Some(st) = &own.style {
        card = card
            .child(fact(k, "fg", color_value(k, st.fg.as_deref(), None)))
            .child(fact(k, "bg", color_value(k, st.bg.as_deref(), None)));
        let flags: Vec<&str> = [
            (st.bold, "bold"),
            (st.italic, "italic"),
            (
                st.text_decoration.underline != lumis_core::themes::UnderlineStyle::None,
                "underline",
            ),
            (st.text_decoration.strikethrough, "strikethrough"),
        ]
        .into_iter()
        .filter_map(|(on, name)| on.then_some(name))
        .collect();
        if !flags.is_empty() {
            card = card.child(fact(k, "Font", mono(k, flags.join(", "), k.c.text_base)));
        }
    }
    card
}

fn tree(analysis: &Analysis, k: &'static Kit) -> El {
    let id = analysis.id;
    let count = analysis.inspection.nodes.len();
    col()
        .key("tree")
        .grow(2.0)
        .min_h(0.0)
        .child(
            row()
                .px(16.0)
                .pt(12.0)
                .pb(6.0)
                .child(text("Tree").font(k.ty.small_medium).color(k.c.text_muted))
                .child(rune::spacer())
                .child(
                    text(format!("{count} nodes"))
                        .font(k.ty.small)
                        .color(k.c.text_faint),
                ),
        )
        .child(
            col()
                .key("tree-scroll")
                .grow(1.0)
                .scroll(ScrollSpec {
                    stick_bottom: false,
                })
                .scroll_edge_fade(24.0)
                .child(
                    col()
                        .key(format!("tree-rows-{id}"))
                        .growth(GrowthSpec::default())
                        .virtual_rows(
                            count,
                            22.0,
                            move |_: &Inspector, i| Cow::Owned(row_key(id, i)),
                            move |s: &Inspector, i| tree_row(s, k, i),
                        ),
                ),
        )
}

fn tree_row(s: &Inspector, k: &Kit, index: usize) -> El {
    let Some(analysis) = s.current() else {
        return el();
    };
    let inspection = &analysis.inspection;
    let Some(node) = inspection.nodes.get(index) else {
        return el();
    };
    let theme = s.theme();
    let editor_bg = style::color(theme.bg()).unwrap_or(k.c.bg_base);
    let injected = node
        .parent
        .is_some_and(|p| inspection.nodes[p].language != node.language);
    let at_caret = s.node() == Some(index);
    let kind_color = if node.is_error || node.is_missing {
        k.c.danger.fg
    } else if injected {
        k.c.text_accent
    } else {
        k.c.text_base
    };
    let mut line = row()
        .h(22.0)
        .pl(16.0 + points(node.depth) * 14.0)
        .pr(12.0)
        .gap(6.0)
        .items_center();
    if let Some(field) = &node.field {
        line = line.child(mono(k, format!("{field}:"), k.c.text_faint));
    }
    let mut kind = node.kind.clone();
    if node.is_missing {
        kind = format!("MISSING {kind}");
    }
    if injected {
        kind = format!("{kind}  ({})", node.language);
    }
    line = line.child(mono(k, kind, kind_color)).child(mono(
        k,
        format!("{}:{}", node.start.row + 1, node.start.column + 1),
        k.c.text_faint,
    ));
    for &capture in node.captures.iter().take(3) {
        let c = &inspection.captures[capture];
        let fg = style::own(theme, c.scope_index, &c.language)
            .style
            .and_then(|st| style::color(st.fg.as_deref()))
            .unwrap_or(k.c.text_muted);
        line = line.child(
            mono(k, format!("@{}", c.scope), fg)
                .px(4.0)
                .radius(3.0)
                .fill(editor_bg),
        );
    }
    if node.captures.len() > 3 {
        line = line.child(mono(
            k,
            format!("+{}", node.captures.len() - 3),
            k.c.text_faint,
        ));
    }
    let line = line
        .on_hover_change(move |s: &mut Inspector, on, _| hover(s, index, on))
        .on_press(move |s: &mut Inspector| s.pick(index));
    let line = if at_caret {
        line.fill(k.c.text_accent.alpha(0.16))
    } else {
        k.interactive(line)
    };
    line.key(row_key(analysis.id, index))
}

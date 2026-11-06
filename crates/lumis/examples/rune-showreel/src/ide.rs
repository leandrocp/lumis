//! An editor window, full-bleed, painted from one Lumis theme: its chrome takes the theme's
//! background and text colours, and its file icons the theme's token colours. Code sits on a
//! fixed monospace grid, so a scene can put an effect exactly over any column and line.

use std::ops::Range;

use lumis::themes::Theme;
use rune::color::Color;
use rune::{El, Span, col, el, rich, row, spacer};

use crate::data::{Run, bg, dark, fg, palette};
use crate::kit::{H, W, float, mono, t};

pub(crate) const TITLE: f32 = 30.0;
pub(crate) const SIDE: f32 = 176.0;
pub(crate) const TABS: f32 = 32.0;
pub(crate) const STATUS: f32 = 22.0;
pub(crate) const GUTTER: f32 = 48.0;
pub(crate) const LH: f32 = 21.0;
pub(crate) const FS: f32 = 13.5;
/// IBM Plex Mono advances 600 units per 1000-unit em.
pub(crate) const CW: f32 = FS * 0.6;
pub(crate) const CODE_X: f32 = SIDE + GUTTER + 12.0;
pub(crate) const CODE_Y: f32 = TITLE + TABS + 10.0;

pub(crate) fn x_of(column: usize) -> f32 {
    CODE_X + float(column) * CW
}

pub(crate) fn y_of(line: usize) -> f32 {
    CODE_Y + float(line) * LH
}

/// The theme's panel colour: its background, a shade further from its text.
pub(crate) fn panel(th: &Theme) -> Color {
    bg(th).mix(
        if dark(th) { Color::BLACK } else { fg(th) },
        if dark(th) { 0.28 } else { 0.05 },
    )
}

pub(crate) fn hair(th: &Theme) -> Color {
    fg(th).alpha(0.1)
}

/// `0xRRGGBB` as a colour.
pub(crate) fn hex(c: u32) -> Color {
    rune::color::hex(&format!("#{c:06x}")).unwrap_or(Color::WHITE)
}

fn icon(color: u32) -> El {
    el().size(9.0, 9.0).radius(2.5).fill(hex(color))
}

fn sidebar(th: &Theme, files: &[&str], active: usize) -> El {
    let p = palette(th);
    let mut side = col()
        .absolute(0.0, TITLE)
        .size(SIDE, H - TITLE - STATUS)
        .px(10.0)
        .pt(12.0)
        .gap(2.0)
        .fill(panel(th))
        .border(1.0, hair(th))
        .child(
            t("EXPLORER", 10.5, 600.0, fg(th).alpha(0.45))
                .px(8.0)
                .pb(8.0),
        );
    for (i, f) in files.iter().enumerate() {
        let on = i == active;
        let depth = f.chars().take_while(|c| *c == ' ').count();
        side = side.child(
            row()
                .h(24.0)
                .pl(8.0 + 6.0 * float(depth))
                .radius(6.0)
                .gap(8.0)
                .center_y()
                .fill(if on {
                    fg(th).alpha(0.09)
                } else {
                    Color::TRANSPARENT
                })
                .child(icon(p[i % p.len()]))
                .child(t(
                    f.trim_start(),
                    12.5,
                    if on { 600.0 } else { 450.0 },
                    fg(th).alpha(if on { 0.95 } else { 0.6 }),
                )),
        );
    }
    side
}

fn titlebar(th: &Theme, title: &str) -> El {
    row()
        .absolute(0.0, 0.0)
        .size(W, TITLE)
        .px(12.0)
        .gap(8.0)
        .center_y()
        .fill(panel(th))
        .border(1.0, hair(th))
        .child(el().size(11.0, 11.0).radius(6.0).fill(hex(0x00ff_5f57)))
        .child(el().size(11.0, 11.0).radius(6.0).fill(hex(0x00fe_bc2e)))
        .child(el().size(11.0, 11.0).radius(6.0).fill(hex(0x0028_c840)))
        .child(spacer())
        .child(t(title, 12.0, 500.0, fg(th).alpha(0.55)))
        .child(spacer())
        .child(el().w(51.0))
}

fn tabs(th: &Theme, names: &[&str]) -> El {
    let mut r = row()
        .absolute(SIDE, TITLE)
        .size(W - SIDE, TABS)
        .fill(panel(th))
        .border(1.0, hair(th));
    for (i, n) in names.iter().enumerate() {
        let on = i == 0;
        r = r.child(
            row()
                .h(TABS)
                .px(14.0)
                .center_y()
                .fill(if on { bg(th) } else { Color::TRANSPARENT })
                .child(t(
                    *n,
                    12.0,
                    500.0,
                    fg(th).alpha(if on { 0.9 } else { 0.45 }),
                )),
        );
    }
    r
}

fn status(th: &Theme, lang: &str) -> El {
    let accent = hex(palette(th)[0]);
    row()
        .absolute(0.0, H - STATUS)
        .size(W, STATUS)
        .px(12.0)
        .gap(16.0)
        .center_y()
        .fill(panel(th))
        .border(1.0, hair(th))
        .child(t("main", 11.0, 500.0, fg(th).alpha(0.6)))
        .child(spacer())
        .child(t(lang, 11.0, 500.0, fg(th).alpha(0.6)))
        .child(t(th.name.as_str(), 11.0, 500.0, fg(th).alpha(0.6)))
        .child(t("Lumis", 11.0, 700.0, accent))
}

/// What the window shows around the code.
pub(crate) struct Chrome<'a> {
    pub(crate) title: &'a str,
    pub(crate) files: &'a [&'a str],
    pub(crate) active: usize,
    pub(crate) tabs: &'a [&'a str],
    pub(crate) lang: &'a str,
    pub(crate) lines: usize,
}

/// The window around `code`, which is drawn at [`CODE_X`], [`CODE_Y`].
pub(crate) fn window(th: &Theme, c: &Chrome<'_>, code: El) -> El {
    let mut gutter = col().absolute(SIDE, CODE_Y).w(GUTTER);
    for n in 0..c.lines {
        gutter =
            gutter.child(
                row().h(LH).w(GUTTER).center_y().child(spacer()).child(
                    t(format!("{}", n + 1), 12.0, 400.0, fg(th).alpha(0.28)).font(mono(12.0)),
                ),
            );
    }
    el().w(W)
        .h(H)
        .fill(bg(th))
        .child(gutter)
        .child(code.absolute(CODE_X, CODE_Y))
        .child(sidebar(th, c.files, c.active))
        .child(tabs(th, c.tabs))
        .child(titlebar(th, c.title))
        .child(status(th, c.lang))
}

/// A run's span on its line: Lumis's colour, weight and slant, before the scene restyles it.
pub(crate) fn span<M>(run: &Run<M>, on_line: Range<usize>) -> Span {
    let mut s = Span::new(on_line.start, on_line.end);
    s.color = Some(run.color);
    s.weight = run.bold.then_some(600.0);
    s.italic = run.italic;
    s
}

/// `code` up to byte `upto`, one fixed-height row per line. `paint` restyles each run's span; it
/// gets the run's index, so a scene can blend it with the same run in another theme.
pub(crate) fn lines<M>(
    code: &str,
    runs: &[Run<M>],
    upto: usize,
    base: Color,
    mut paint: impl FnMut(usize, &Run<M>, &mut Span),
) -> El {
    let mut out = col();
    let mut start = 0;
    for text in code[..upto].split('\n') {
        let end = start + text.len();
        let mut spans = Vec::new();
        for (i, run) in runs.iter().enumerate() {
            let (a, b) = (run.range.start.max(start), run.range.end.min(end));
            if a < b {
                let mut s = span(run, a - start..b - start);
                paint(i, run, &mut s);
                spans.push(s);
            }
        }
        out = out.child(rich(text, spans).font(mono(FS)).color(base).h(LH));
        start = end + 1;
    }
    out
}

/// The line and column of byte `at`.
pub(crate) fn place(code: &str, at: usize) -> (usize, usize) {
    let before = &code[..at];
    let line = before.matches('\n').count();
    let column = before.len() - before.rfind('\n').map_or(0, |i| i + 1);
    (line, column)
}

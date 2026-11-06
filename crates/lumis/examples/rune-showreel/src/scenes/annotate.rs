//! Three things that have nothing to do with syntax, drawn over the same code: search hits,
//! compiler diagnostics (one with a help note nested inside it) and a profile with the hottest
//! line on fire. Each is an `Annotation<Mark>`; Lumis cuts the source at their edges and reports
//! which are open over every piece, and this scene decides what each one looks like.

use std::sync::OnceLock;

use lumis::annotations::{Annotation, Position};
use lumis::languages::Language;
use rune::color::Color;
use rune::{El, Span, col, el, rich, row, spacer};

use crate::data::{Run, annotated, bg, color, fg, theme};
use crate::gfx::{FIRE, GRAIN};
use crate::kit::{H, W, ease, float, fx, mono, sp, t, track};

pub(crate) const LEN: f64 = 11.0;
const THEME: &str = "tokyonight_night";
const CODE: &str = "fn total_price(cart: &Cart, rates: &Rates) -> f64 {
    let mut total = 0.0;
    for item in &cart.items {
        let rate = rates.lookup(&item.region);
        for _ in 0..item.qty {
            total += item.price * rate.factor();
        }
    }
    let discount = cart.coupon.map(|c| c.percent);
    total
}";

const LH: f32 = 26.0;
const CARD: (f32, f32, f32) = (48.0, 112.0, 590.0);
const HEADER: f32 = 44.0;
const SEARCH: (f64, f64) = (0.7, 3.6);
const DIAG: (f64, f64) = (3.9, 7.6);
const HEAT: f64 = 7.9;
/// The line the profile spends most of its time on.
const HOT: usize = 5;

/// What an annotation means. Lumis carries it without looking at it.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Mark {
    /// The `n`th match of the search.
    Hit(usize),
    Error(&'static str),
    Warning(&'static str),
    Help(&'static str),
    /// A line's share of the profile, 0 to 1.
    Heat(f32),
}

fn at(line: usize, from: usize, to: usize) -> std::ops::Range<Position> {
    Position::new(line, from)..Position::new(line, to)
}

fn annotations() -> &'static [Annotation<Mark>] {
    static ALL: OnceLock<Vec<Annotation<Mark>>> = OnceLock::new();
    ALL.get_or_init(|| {
        let ok = |a: Result<Annotation<Mark>, _>| a.expect("valid range");
        let mut all: Vec<Annotation<Mark>> = CODE
            .match_indices("total")
            .enumerate()
            .map(|(n, (i, m))| ok(Annotation::new(i..i + m.len(), Mark::Hit(n))))
            .collect();
        all.push(ok(Annotation::new(
            at(5, 21, 31),
            Mark::Error("no field `price` on `&Item`"),
        )));
        all.push(ok(Annotation::new(
            at(5, 26, 31),
            Mark::Help("a field with a similar name exists: `unit_price`"),
        )));
        all.push(ok(Annotation::new(
            at(8, 8, 16),
            Mark::Warning("unused variable: `discount`"),
        )));
        for (line, share) in [(2, 0.02), (3, 0.11), (4, 0.05), (HOT, 0.82)] {
            all.push(ok(Annotation::new(
                Position::new(line, 0)..Position::new(line + 1, 0),
                Mark::Heat(share),
            )));
        }
        all
    })
}

fn runs() -> &'static [Run<Mark>] {
    static RUNS: OnceLock<Vec<Run<Mark>>> = OnceLock::new();
    RUNS.get_or_init(|| annotated(CODE, Language::Rust, theme(THEME), annotations()))
}

/// How far each part of the scene is in, 0 to 1.
struct Phase {
    hits: f32,
    diags: f32,
    heat: f32,
    t: f64,
}

impl Phase {
    fn at(t: f64) -> Phase {
        Phase {
            hits: ease(t, SEARCH.0, 0.3) * (1.0 - ease(t, SEARCH.1, 0.3)),
            diags: ease(t, DIAG.0, 0.3) * (1.0 - ease(t, DIAG.1, 0.3)),
            heat: ease(t, HEAT, 1.2),
            t,
        }
    }

    /// A search hit pops in after the one before it.
    fn hit(&self, n: usize) -> f32 {
        let start = SEARCH.0 + 0.18 * f64::from(float(n));
        track(self.t, 0.0, &[(start, 1.0, sp(0.45, 0.4))]) * self.hits.min(1.0)
    }
}

const HIT: &str = "comment.warning";
const ERROR: &str = "comment.error";
const WARNING: &str = "comment.warning";
const HELP: &str = "comment.hint";
const COLD: &str = "comment.todo";

/// The theme's colour for `scope`, so the marks change with the theme like the code does.
fn accent(scope: &str) -> Color {
    theme(THEME)
        .get_style(scope)
        .and_then(|s| s.fg.as_deref())
        .map_or_else(|| fg(theme(THEME)), color)
}

/// The span for one run, styled by its syntax and then by the annotations open over it.
fn span(run: &Run<Mark>, range: std::ops::Range<usize>, p: &Phase) -> Span {
    let mut s = Span::new(range.start, range.end);
    s.color = Some(run.color);
    s.weight = run.bold.then_some(600.0);
    s.italic = run.italic;
    for mark in &run.marks {
        match **mark {
            Mark::Hit(n) => {
                let k = p.hit(n);
                s.bg = Some(accent(HIT).alpha(0.34 * k.clamp(0.0, 1.2)));
                s.bg_pad = 2.0 * k.clamp(0.0, 1.0);
            }
            Mark::Error(_) => s.underline = Some(accent(ERROR).alpha(p.diags)),
            Mark::Warning(_) => s.underline = Some(accent(WARNING).alpha(p.diags)),
            Mark::Help(_) => {
                s.underline = Some(accent(HELP).alpha(p.diags));
                s.bg = Some(accent(HELP).alpha(0.18 * p.diags));
            }
            Mark::Heat(_) => {}
        }
    }
    s
}

/// One line's spans, relative to where the line starts, and its share of the profile.
fn line_spans(start: usize, end: usize, p: &Phase) -> (Vec<Span>, f32) {
    let mut spans = Vec::new();
    let mut heat = 0.0f32;
    for run in runs() {
        let (a, b) = (run.range.start.max(start), run.range.end.min(end));
        if a >= b {
            continue;
        }
        for mark in &run.marks {
            if let Mark::Heat(share) = **mark {
                heat = heat.max(share);
            }
        }
        spans.push(span(run, a - start..b - start, p));
    }
    (spans, heat)
}

fn heat_color(share: f32) -> Color {
    accent(COLD).mix(accent(ERROR), (share / 0.82).clamp(0.0, 1.0))
}

fn line(n: usize, text: &str, start: usize, p: &Phase) -> El {
    let th = theme(THEME);
    let (spans, heat) = line_spans(start, start + text.len(), p);
    let glow = p.heat * (0.1 + 0.5 * heat.sqrt());
    let share = if heat > 0.0 && p.heat > 0.0 {
        format!("{:>3.0}%", heat * 100.0 * p.heat)
    } else {
        String::new()
    };
    row()
        .w(CARD.2 - 32.0)
        .h(LH)
        .radius(6.0)
        .center_y()
        .fill(heat_color(heat).alpha(if heat > 0.0 { glow } else { 0.0 }))
        .child(
            t(format!("{:>2}", n + 1), 13.0, 400.0, fg(th).alpha(0.3))
                .font(mono(13.0))
                .w(36.0),
        )
        .child(rich(text, spans).font(mono(14.5)).color(fg(th)).grow(1.0))
        .child(
            t(share, 12.0, 600.0, heat_color(heat))
                .font(mono(12.0))
                .w(48.0),
        )
}

fn code_card(p: &Phase) -> El {
    let th = theme(THEME);
    let mut lines = col().gap(0.0);
    if p.heat > 0.0 {
        lines = lines.child(
            fx(FIRE, [10.0 + 12.0 * p.heat, 1.0 * p.heat, 0.7, 0.2])
                .absolute(52.0, float(HOT + 1) * LH - 4.0)
                .size(CARD.2 - 140.0, 8.0)
                .radius(6.0)
                .effect_bleed(90.0),
        );
    }
    let mut start = 0;
    for (n, text) in CODE.split('\n').enumerate() {
        lines = lines.child(line(n, text, start, p));
        start += text.len() + 1;
    }
    let (label, count) = if p.heat > 0.0 {
        ("profile", "82% of time in one line".to_string())
    } else if p.t >= DIAG.0 - 0.15 {
        ("diagnostics", "1 error · 1 warning · 1 help".to_string())
    } else {
        (
            "search: total",
            format!("{} matches", CODE.matches("total").count()),
        )
    };
    col()
        .absolute(CARD.0, CARD.1)
        .w(CARD.2)
        .padding(16.0)
        .radius(16.0)
        .fill(bg(th).mix(Color::WHITE, 0.03))
        .border(1.0, fg(th).alpha(0.1))
        .shadow(16.0, 44.0, Color::BLACK.alpha(0.45))
        .child(
            row()
                .h(HEADER - 16.0)
                .gap(10.0)
                .center_y()
                .child(t("pricing.rs", 13.0, 500.0, fg(th).alpha(0.6)))
                .child(spacer())
                .child(
                    el().px(8.0)
                        .py(3.0)
                        .radius(6.0)
                        .fill(fg(th).alpha(0.08))
                        .child(t(label, 12.0, 600.0, fg(th).alpha(0.8)).font(mono(12.0))),
                )
                .child(t(count, 12.0, 500.0, fg(th).alpha(0.5))),
        )
        .child(lines)
}

/// A note beside line `line` for a diagnostic, springing out after the one before it.
fn note(i: usize, line: usize, scope: &str, kind: &str, msg: &str, p: &Phase) -> El {
    const WIDTH: f32 = W - CARD.0 - CARD.2 - 42.0;
    let th = theme(THEME);
    let k = track(
        p.t,
        0.0,
        &[(DIAG.0 + 0.35 * f64::from(float(i)), 1.0, sp(0.5, 0.3))],
    );
    row()
        .w(WIDTH)
        .gap(8.0)
        .center_y()
        .opacity(k.clamp(0.0, 1.0) * p.diags)
        .offset(28.0 * (1.0 - k), 0.0)
        .child(el().size(8.0, 8.0).radius(4.0).fill(accent(scope)))
        .child(
            col()
                .w(WIDTH - 16.0)
                .px(10.0)
                .py(7.0)
                .gap(3.0)
                .radius(8.0)
                .fill(bg(th).mix(Color::WHITE, 0.06))
                .border(1.0, accent(scope).alpha(0.4))
                .child(
                    t(
                        format!("{kind} · line {}", line + 1),
                        11.0,
                        700.0,
                        accent(scope),
                    )
                    .font(mono(11.0)),
                )
                .child(
                    t(msg, 12.0, 500.0, fg(th).alpha(0.85))
                        .font(mono(12.0))
                        .wrap(),
                ),
        )
}

fn notes(p: &Phase) -> El {
    let mut out = Vec::new();
    for a in annotations() {
        let (scope, kind, msg, line) = match *a.data() {
            Mark::Error(m) => (ERROR, "error", m, 5),
            Mark::Help(m) => (HELP, "help", m, 5),
            Mark::Warning(m) => (WARNING, "warning", m, 8),
            _ => continue,
        };
        out.push(note(out.len(), line, scope, kind, msg, p));
    }
    col()
        .absolute(CARD.0 + CARD.2 + 18.0, CARD.1 + HEADER + 3.0 * LH)
        .gap(10.0)
        .children(out)
}

fn caption(p: &Phase) -> El {
    let th = theme(THEME);
    let code = if p.heat > 0.0 {
        "Annotation::new(Position::new(5, 0)..Position::new(6, 0), Mark::Heat(0.82))"
    } else if p.t >= DIAG.0 - 0.15 {
        "Annotation::new(Position::new(5, 21)..Position::new(5, 31), Mark::Error(msg))"
    } else {
        "Annotation::new(start..end, Mark::Hit(n))"
    };
    col()
        .absolute(0.0, H - 78.0)
        .w(W)
        .gap(6.0)
        .items_center()
        .child(t(code, 13.0, 500.0, fg(th).alpha(0.75)).font(mono(13.0)))
        .child(t(
            "Your ranges, your meaning. Lumis fits them into the syntax.",
            13.0,
            400.0,
            fg(th).alpha(0.45),
        ))
}

pub(crate) fn view(t_: f64) -> El {
    let th = theme(THEME);
    let p = Phase::at(t_);
    let enter = ease(t_, 0.0, 0.4);
    el().w(W)
        .h(H)
        .fill(bg(th))
        .opacity(enter)
        .child(
            col()
                .absolute(0.0, 34.0)
                .w(W)
                .items_center()
                .gap(6.0)
                .child(t("Annotations", 30.0, 600.0, fg(th)))
                .child(t(
                    "Search, diagnostics, profiles: any range, any data",
                    14.0,
                    400.0,
                    fg(th).alpha(0.55),
                )),
        )
        .child(code_card(&p))
        .child(notes(&p))
        .child(caption(&p))
        .child(
            fx(GRAIN, [0.05, 24.0, 0.0, 0.0])
                .absolute(0.0, 0.0)
                .size(W, H),
        )
}

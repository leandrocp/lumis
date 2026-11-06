//! The same function as the compiler and a profiler see it, in another theme. Both arrive as
//! annotations: an error and a warning under the exact bytes they name, and each line's share of
//! the run time. Lumis cuts them into the tokens they cover; the line that takes most of the
//! time catches fire.

use std::ops::Range;
use std::sync::OnceLock;

use lumis::annotations::Annotation;
use lumis::languages::Language;
use rune::color::Color;
use rune::{El, el, row};

use crate::data::{Run, annotated, bg, danger, fg, first, theme};
use crate::gfx::{FIRE, GRAIN, HAZE, SHOCK};
use crate::ide::{CODE_X, Chrome, LH, lines, place, window, x_of, y_of};
use crate::kit::{H, W, ease, fx, mono, sp, t, track};
use crate::scenes::{FILES, function};

pub(crate) const LEN: f64 = 4.2;
const THEME: &str = "kanagawa_wave";
const DIAG: f64 = 0.3;
const HEAT: f64 = 1.4;
const IGNITE: f64 = 2.2;

#[derive(Clone, Copy, Debug)]
enum Mark {
    Error(&'static str),
    Warning(&'static str),
    /// The line's share of the profile, 0 to 1.
    Heat(f32),
}

fn bytes(needle: &str) -> Range<usize> {
    let at = function().find(needle).expect("in the code");
    at..at + needle.len()
}

fn line_with(needle: &str) -> Range<usize> {
    let code = function();
    let at = bytes(needle).start;
    let start = code[..at].rfind('\n').map_or(0, |i| i + 1);
    let end = code[at..].find('\n').map_or(code.len(), |i| at + i);
    start..end
}

/// What the compiler and the profiler report, by byte range.
fn marks() -> &'static [(Range<usize>, Mark)] {
    static MARKS: OnceLock<Vec<(Range<usize>, Mark)>> = OnceLock::new();
    MARKS.get_or_init(|| {
        vec![
            (line_with("let rate"), Mark::Heat(0.11)),
            (line_with("for _ in"), Mark::Heat(0.05)),
            (line_with("total +="), Mark::Heat(0.82)),
            (
                bytes("item.price"),
                Mark::Error("no field `price` on `&Item`"),
            ),
            (
                bytes("discount"),
                Mark::Warning("unused variable: `discount`"),
            ),
        ]
    })
}

fn annotations() -> &'static [Annotation<Mark>] {
    static ALL: OnceLock<Vec<Annotation<Mark>>> = OnceLock::new();
    ALL.get_or_init(|| {
        marks()
            .iter()
            .map(|(range, mark)| Annotation::new(range.clone(), *mark).expect("valid range"))
            .collect()
    })
}

fn runs() -> &'static [Run<Mark>] {
    static RUNS: OnceLock<Vec<Run<Mark>>> = OnceLock::new();
    RUNS.get_or_init(|| annotated(function(), Language::Rust, theme(THEME), annotations()))
}

fn warning(th: &lumis::themes::Theme) -> Color {
    first(th, &["diff.delta", "comment.warning", "warning"])
}

/// Each line's heat bar and share, behind the code.
fn heat(th: &lumis::themes::Theme, k: f32) -> El {
    let cold = first(th, &["comment.todo", "function"]);
    let mut out = el().absolute(0.0, 0.0).size(W - CODE_X, H);
    for (range, mark) in marks() {
        let Mark::Heat(share) = *mark else {
            continue;
        };
        let (line, _) = place(function(), range.start);
        let tint = cold.mix(danger(th), share / 0.82);
        let y = y_of(line) - y_of(0);
        out = out
            .child(
                el().absolute(-8.0, y)
                    .size(W - CODE_X - 24.0, LH)
                    .radius(5.0)
                    .fill(tint.alpha(k * (0.12 + 0.45 * share.sqrt()))),
            )
            .child(
                t(format!("{:>3.0}%", share * 100.0 * k), 12.0, 700.0, tint)
                    .font(mono(12.0))
                    .absolute(W - CODE_X - 76.0, y + 3.0)
                    .opacity(k),
            );
    }
    out
}

/// The diagnostics' messages at the end of their lines, sliding in one after the other.
fn lens(th: &lumis::themes::Theme, t_: f64) -> El {
    let code = function();
    let mut out = el().absolute(0.0, 0.0).size(W - CODE_X, H);
    let mut i = 0.0;
    for (range, mark) in marks() {
        let (color, msg) = match *mark {
            Mark::Error(m) => (danger(th), m),
            Mark::Warning(m) => (warning(th), m),
            Mark::Heat(_) => continue,
        };
        let (line, _) = place(code, range.start);
        let len = code.lines().nth(line).map_or(0, str::len);
        let k = track(t_, 0.0, &[(DIAG + 0.25 * i, 1.0, sp(0.45, 0.3))]);
        out = out.child(
            row()
                .absolute(x_of(len + 3) - CODE_X, y_of(line) - y_of(0))
                .h(LH)
                .px(8.0)
                .gap(6.0)
                .center_y()
                .radius(5.0)
                .fill(color.alpha(0.14))
                .opacity(k.clamp(0.0, 1.0))
                .offset(24.0 * (1.0 - k), 0.0)
                .child(t(format!("● {msg}"), 12.5, 500.0, color).font(mono(12.5))),
        );
        i += 1.0;
    }
    out
}

fn pill(th: &lumis::themes::Theme, t_: f64) -> El {
    let call = if t_ < HEAT {
        "Annotation::new(span, Mark::Error(msg))"
    } else {
        "Annotation::new(line, Mark::Heat(0.82))"
    };
    row()
        .absolute(W - 400.0, H - 66.0)
        .px(12.0)
        .py(8.0)
        .radius(8.0)
        .fill(bg(th).mix(fg(th), 0.08))
        .border(1.0, fg(th).alpha(0.15))
        .shadow(10.0, 30.0, Color::BLACK.alpha(0.4))
        .child(t(call, 12.5, 500.0, fg(th).alpha(0.85)).font(mono(12.5)))
}

pub(crate) fn view(t_: f64) -> El {
    let th = theme(THEME);
    let diag = ease(t_, DIAG, 0.4);
    let code = lines(function(), runs(), function().len(), fg(th), |_, run, s| {
        for mark in &run.marks {
            match **mark {
                Mark::Error(_) => s.underline = Some(danger(th).alpha(diag)),
                Mark::Warning(_) => s.underline = Some(warning(th).alpha(diag)),
                Mark::Heat(_) => {}
            }
        }
    });
    let chrome = Chrome {
        title: "lumis — pricing.rs",
        files: FILES,
        active: 2,
        tabs: &["pricing.rs", "cart.rs", "rates.rs"],
        lang: "Rust",
        lines: function().lines().count(),
    };
    let (hot, _) = place(function(), bytes("total +=").start);
    let blaze = ease(t_, IGNITE, 0.6);
    let age = (t_ - IGNITE) as f32;
    let zoom = 1.0 + 0.22 * ease(t_, HEAT, 1.4);
    let shake = if age > 0.0 {
        7.0 * (1.0 - ease(t_, IGNITE, 0.6)) * (age * 70.0).sin()
    } else {
        0.0
    };
    let fire_y = y_of(hot) + LH - 6.0;
    let mut scene = window(
        th,
        &chrome,
        el().size(W - CODE_X, H - y_of(0))
            .child(heat(th, ease(t_, HEAT, 0.5)))
            .child(code)
            .child(lens(th, t_)),
    );
    if age > -0.05 {
        scene = scene
            .child(
                fx(HAZE, [150.0, 1.6 * blaze, 0.0, 0.0])
                    .effect_backdrop(true)
                    .absolute(CODE_X, fire_y - 170.0)
                    .size(560.0, 176.0),
            )
            .child(
                fx(FIRE, [20.0 + 70.0 * blaze, 0.3 + 1.1 * blaze, 0.5, 0.9])
                    .absolute(CODE_X - 8.0, fire_y)
                    .size(W - CODE_X - 40.0, 8.0)
                    .radius(4.0)
                    .effect_bleed(220.0),
            );
    }
    el().w(W)
        .h(H)
        .child(
            scene
                .child(
                    fx(
                        SHOCK,
                        [
                            CODE_X + 220.0,
                            fire_y,
                            age,
                            if age >= 0.0 { 14.0 } else { 0.0 },
                        ],
                    )
                    .effect_backdrop(true)
                    .absolute(0.0, 0.0)
                    .size(W, H),
                )
                .scale(zoom)
                .origin(x_of(20) / W, y_of(hot) / H)
                .offset(shake, 0.6 * shake),
        )
        .child(pill(th, t_))
        .child(
            fx(GRAIN, [0.04, 24.0, 0.0, 0.0])
                .absolute(0.0, 0.0)
                .size(W, H),
        )
        .fill(Color::BLACK)
        .opacity(ease(t_, 0.0, 0.12))
}

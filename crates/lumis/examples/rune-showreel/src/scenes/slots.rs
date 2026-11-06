//! ⌘K, "Color Theme", and the palette spins through every bundled theme like a slot machine:
//! slow, then a blur of a hundred themes a second, then slow again. The whole editor repaints
//! on every step, and the landing hits with a shockwave.

use std::sync::OnceLock;

use lumis::languages::Language;
use rune::color::Color;
use rune::{El, col, el, row, spacer};

use crate::data::{bg, fg, packed, styled, themes};
use crate::gfx::{GRAIN, SHOCK, SPARKLES};
use crate::ide::{Chrome, hex, lines, window};
use crate::kit::{H, W, float, fx, index, mono, sp, t, track};
use crate::scenes::{FILES, PRICING};

pub(crate) const LEN: f64 = 3.2;
const START: f64 = 0.25;
const RUN: f64 = 2.3;
const SLOW_DWELL: f64 = 0.3;
const FIRST: &str = "tokyonight_night";
const LAST: &str = "catppuccin_mocha";
const ROW: f32 = 26.0;
const PAL: (f32, f32) = (70.0, 380.0);
/// The middle of the palette's selected row.
const SELECTED: f32 = PAL.0 + 8.0 + 34.0 + 6.0 + 2.5 * ROW;

/// Where the spin is at `t_`: a fractional index into the order. Its speed is
/// `slow + (peak - slow) * sin^4(pi * p)`, integrated in closed form so the last theme lands
/// exactly at `RUN`.
fn spin(t_: f64, n: usize) -> f64 {
    use std::f64::consts::PI;
    let p = ((t_ - START) / RUN).clamp(0.0, 1.0);
    let span = f64::from(float(n - 1));
    let slow = RUN / SLOW_DWELL;
    let rush = (span - slow) / 0.375;
    let sin4 =
        |p: f64| 0.375 * p - (2.0 * PI * p).sin() / (4.0 * PI) + (4.0 * PI * p).sin() / (32.0 * PI);
    (slow * p + rush * sin4(p)).min(span)
}

/// Every theme, shuffled so neighbours differ, from the opener's theme to the landing one.
fn order() -> &'static [usize] {
    static ORDER: OnceLock<Vec<usize>> = OnceLock::new();
    ORDER.get_or_init(|| {
        let all = themes();
        let n = all.len();
        let pos = |name: &str| all.iter().position(|t| t.name == name).unwrap_or(0);
        let (first, last) = (pos(FIRST), pos(LAST));
        let mut out: Vec<usize> = (0..n)
            .map(|i| (i * 89 + 13) % n)
            .filter(|&i| i != first && i != last)
            .collect();
        out.insert(0, first);
        out.push(last);
        out
    })
}

fn palette_list(x: f64, th: &'static lumis::themes::Theme, landed: f32) -> El {
    let all = themes();
    let ord = order();
    let at = index(x);
    let frac = (x - f64::from(float(at))) as f32;
    let mut list = col().absolute(0.0, -(2.0 + frac) * ROW);
    for k in 0..9usize {
        let Some(&i) = (at + k).checked_sub(4).and_then(|j| ord.get(j)) else {
            list = list.child(el().h(ROW));
            continue;
        };
        let near = (float(k) - 4.0 - frac).abs();
        list = list.child(
            row().h(ROW).w(PAL.1 - 16.0).px(10.0).center_y().child(
                t(
                    all[i].name.as_str(),
                    13.0,
                    500.0,
                    fg(th).alpha(1.0 - 0.22 * near),
                )
                .font(mono(13.0)),
            ),
        );
    }
    let accent = hex(crate::data::palette(th)[0]);
    col()
        .absolute((W - PAL.1) / 2.0, PAL.0)
        .w(PAL.1)
        .padding(8.0)
        .gap(6.0)
        .radius(12.0)
        .fill(bg(th).mix(fg(th), 0.06))
        .border(1.0, fg(th).alpha(0.14))
        .shadow(20.0, 50.0, Color::BLACK.alpha(0.5))
        .child(
            row()
                .h(34.0)
                .px(10.0)
                .gap(8.0)
                .center_y()
                .radius(8.0)
                .fill(fg(th).alpha(0.05))
                .child(t("›", 15.0, 600.0, accent))
                .child(t("Color Theme", 14.0, 500.0, fg(th).alpha(0.85)))
                .child(spacer())
                .child(t(
                    format!("{} / {}", at + 1, all.len()),
                    12.0,
                    500.0,
                    fg(th).alpha(0.4),
                )),
        )
        .child(
            el().h(ROW * 5.0)
                .w_full()
                .clip_rounded(8.0)
                .child(
                    el().absolute(0.0, 2.0 * ROW)
                        .w_full()
                        .h(ROW)
                        .radius(7.0)
                        .fill(accent.alpha(0.18 + 0.3 * landed))
                        .border(1.0, accent.alpha(0.4 + 0.6 * landed)),
                )
                .child(list),
        )
}

pub(crate) fn view(t_: f64) -> El {
    let all = themes();
    let ord = order();
    let x = spin(t_, ord.len());
    let th = all[ord[index(x + 0.5).min(ord.len() - 1)]];
    let runs = styled(PRICING, Language::Rust, th);
    let code = lines(PRICING, &runs, PRICING.len(), fg(th), |_, _, _| {});
    let chrome = Chrome {
        title: "lumis — pricing.rs",
        files: FILES,
        active: 2,
        tabs: &["pricing.rs", "cart.rs", "rates.rs"],
        lang: "Rust",
        lines: PRICING.lines().count(),
    };
    let land = START + RUN;
    let landed = track(t_, 0.0, &[(land, 1.0, sp(0.4, 0.5))]);
    let open = track(
        t_,
        0.0,
        &[(0.0, 1.0, sp(0.3, 0.3)), (LEN - 0.35, 0.0, sp(0.25, 0.0))],
    );
    let accent = hex(crate::data::palette(th)[0]);
    let age = (t_ - land) as f32;
    window(th, &chrome, code)
        .child(
            el().absolute(0.0, 0.0)
                .size(W, H)
                .fill(Color::BLACK.alpha(0.25 * open)),
        )
        .child(
            palette_list(x, th, landed)
                .scale(0.94 + 0.06 * open + 0.04 * (landed - landed.min(1.0)))
                .opacity(open.clamp(0.0, 1.0)),
        )
        .child(
            fx(
                SPARKLES,
                [
                    if age >= 0.0 { age } else { -1.0 },
                    40.0,
                    5.0,
                    packed(accent),
                ],
            )
            .absolute((W - PAL.1) / 2.0, SELECTED - ROW / 2.0)
            .size(PAL.1, ROW)
            .effect_bleed(160.0),
        )
        .child(
            fx(
                SHOCK,
                [W / 2.0, SELECTED, age, if age >= 0.0 { 18.0 } else { 0.0 }],
            )
            .effect_backdrop(true)
            .absolute(0.0, 0.0)
            .size(W, H),
        )
        .child(
            fx(GRAIN, [0.04, 24.0, 0.0, 0.0])
                .absolute(0.0, 0.0)
                .size(W, H),
        )
}

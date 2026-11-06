//! One snippet through every bundled theme, slow at first and then faster, each token
//! crossfading from one theme's colour to the next. The lava follows the palette.

use rune::{El, col, el, row, spacer};

use std::sync::OnceLock;

use crate::data::{Snippet, bg, code, custom, dark, fg, themes};
use crate::gfx::{GRAIN, LAVA, SPARKLES};
use crate::kit::{H, W, ease, float, fx, index, mono, sp, t, track};

pub(crate) const LEN: f64 = 8.0;
const RUN: f64 = 6.6;
/// Seconds each theme holds at the slow ends of the carousel.
const SLOW_DWELL: f64 = 0.6;
const FADE: f64 = 0.22;
const FIRST: &str = "catppuccin_mocha";
const LAST: &str = "tokyonight_night";
const CODE: &str = r#"defmodule Lumis.Showcase do
  @moduledoc "Every theme, one highlighter."
  @themes ~w(dracula nord tokyonight)a

  def render(code, opts \\ []) do
    theme = Keyword.get(opts, :theme, "catppuccin_mocha")

    code
    |> Lumis.highlight!(formatter: {:html_inline, theme: theme})
    |> then(&{:ok, &1, length(@themes)})
  end
end"#;

fn elixir() -> &'static Snippet {
    static SNIPPET: OnceLock<Snippet> = OnceLock::new();
    SNIPPET.get_or_init(|| custom("elixir", CODE))
}

/// Where the carousel is at `t_` (a fractional index into the order) and how long the theme
/// there is on screen. Its speed is `SLOW + (peak - SLOW) * sin^4(pi * p)`: a slow start, a
/// rush through the middle and a slow landing, integrated in closed form so the last theme
/// lands exactly at `RUN`.
fn carousel(t_: f64, n: usize) -> (f64, f64) {
    use std::f64::consts::PI;
    let p = (t_ / RUN).clamp(0.0, 1.0);
    let span = f64::from(float(n - 1));
    let slow = RUN / SLOW_DWELL;
    let rush = (span - slow) / 0.375;
    let sin4 =
        |p: f64| 0.375 * p - (2.0 * PI * p).sin() / (4.0 * PI) + (4.0 * PI * p).sin() / (32.0 * PI);
    let x = (slow * p + rush * sin4(p)).min(span);
    let speed = slow + rush * (PI * p).sin().powi(4);
    (x, RUN / speed)
}

/// Every theme, shuffled so neighbours differ, starting and ending on named ones.
fn order() -> Vec<usize> {
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
}

fn card(s: &'static Snippet, a: usize, b: usize, k: f32) -> El {
    let all = themes();
    let (ta, tb) = (all[a], all[b]);
    let fill = bg(ta).mix(bg(tb), k);
    let ink = fg(ta).mix(fg(tb), k);
    col()
        .w(640.0)
        .h(300.0)
        .padding(22.0)
        .gap(14.0)
        .radius(16.0)
        .fill(fill.alpha(0.9))
        .border(1.0, ink.alpha(0.12))
        .shadow(16.0, 44.0, rune::Color::BLACK.alpha(0.4))
        .clip_rounded(16.0)
        .child(
            row()
                .child(t("lib.ex", 13.0, 500.0, ink.alpha(0.55)))
                .child(spacer())
                .child(t(s.name, 12.0, 500.0, ink.alpha(0.4))),
        )
        .child(code(s, ta, tb, k, usize::MAX, mono(15.0)))
}

pub(crate) fn view(t_: f64) -> El {
    let all = themes();
    let order = order();
    let total = order.len();
    let (pos, dwell) = carousel(t_, total);
    let i = index(pos).min(total - 1);
    // Each theme holds, then fades into the next over its last 0.22 s (at once when faster).
    let frac = (1.0 - (1.0 - (pos - pos.floor())) * dwell / FADE).clamp(0.0, 1.0) as f32;
    let k = frac * frac * (3.0 - 2.0 * frac);
    let (a, b) = (order[i], order[(i + 1).min(total - 1)]);
    let shown = if k < 0.5 { a } else { b };
    let (ta, tb) = (all[a], all[b]);
    let ink = fg(ta).mix(fg(tb), k);

    let finale = ease(t_, RUN + 0.1, 0.5);
    let pop = track(t_, 0.0, &[(RUN + 0.1, 1.0, sp(0.5, 0.35))]);
    let enter = ease(t_, 0.0, 0.35);

    let label = col()
        .gap(6.0)
        .items_center()
        .child(t(all[shown].name.clone(), 26.0, 600.0, ink).font(mono(26.0)))
        .child(
            row()
                .gap(10.0)
                .center_y()
                .child(
                    t(
                        format!(
                            "{:>3} / {}",
                            (i + usize::from(k >= 0.5) + 1).min(total),
                            total
                        ),
                        13.0,
                        500.0,
                        ink.alpha(0.55),
                    )
                    .font(mono(13.0)),
                )
                .child(
                    el().px(8.0)
                        .py(2.0)
                        .radius(6.0)
                        .fill(ink.alpha(0.1))
                        .child(t(
                            if dark(all[shown]) { "dark" } else { "light" },
                            11.0,
                            500.0,
                            ink.alpha(0.7),
                        )),
                ),
        );

    let count = t(format!("{total} themes"), 64.0, 600.0, ink)
        .scale(0.7 + 0.3 * pop)
        .opacity(finale);

    el().w(W)
        .h(H)
        .fill(bg(ta).mix(bg(tb), k))
        .opacity(enter)
        .child(
            fx(LAVA, [float(a), float(b), k, 0.45])
                .absolute(0.0, 0.0)
                .size(W, H),
        )
        .child(
            col()
                .absolute(0.0, 0.0)
                .size(W, H)
                .center()
                .gap(24.0)
                .opacity(1.0 - 0.9 * finale)
                .scale(1.0 - 0.06 * finale)
                .blur(10.0 * finale)
                .child(label)
                .child(card(elixir(), a, b, k)),
        )
        .child(col().absolute(0.0, 0.0).size(W, H).center().child(count))
        .child(
            fx(SPARKLES, [(t_ - RUN - 0.1) as f32, 26.0, 9.0, float(b)])
                .absolute(W / 2.0 - 200.0, H / 2.0 - 50.0)
                .size(400.0, 100.0)
                .effect_bleed(60.0),
        )
        .child(
            fx(GRAIN, [0.05, 24.0, 0.0, 0.0])
                .absolute(0.0, 0.0)
                .size(W, H),
        )
}

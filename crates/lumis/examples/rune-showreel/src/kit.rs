//! Shared look and timing helpers.

use rune::color::Color;
use rune::{El, TextStyle, el, text};
use rune_motion::Spring;

pub(crate) const W: f32 = 960.0;
pub(crate) const H: f32 = 540.0;

pub(crate) fn t(s: impl Into<String>, size: f32, weight: f32, color: Color) -> El {
    text(s).font(TextStyle::new(size, weight)).color(color)
}

pub(crate) fn mono(size: f32) -> TextStyle {
    TextStyle::new(size, 400.0).mono()
}

/// An effect slot: `kind` is a `gfx` constant, `data` its numbers.
pub(crate) fn fx(kind: u32, data: [f32; 4]) -> El {
    rune::effect(kind).effect_data(data)
}

/// 0 → 1 between `t0` and `t0 + dur`, smoothstep.
pub(crate) fn ease(t: f64, t0: f64, dur: f64) -> f32 {
    let x = ((t - t0) / dur).clamp(0.0, 1.0) as f32;
    x * x * (3.0 - 2.0 * x)
}

/// A value that starts at `from` and springs to each key's target at its time, carrying velocity
/// across retargets.
pub(crate) fn track(t: f64, from: f32, keys: &[(f64, f32, Spring)]) -> f32 {
    let (mut x, mut v, mut target, mut t0, mut spring) =
        (from, 0.0f32, from, 0.0f64, Spring::new(0.3, 0.0));
    for &(tk, to, sp) in keys {
        if t < tk {
            break;
        }
        let (nx, nv) = spring.sample(x, v, target, (tk - t0) as f32);
        x = nx;
        v = nv;
        target = to;
        t0 = tk;
        spring = sp;
    }
    spring.sample(x, v, target, (t - t0) as f32).0
}

/// A count or index as a float. Those here count cards and themes, a few hundred at most.
#[allow(clippy::cast_precision_loss)]
pub(crate) fn float(i: usize) -> f32 {
    i as f32
}

/// A position along a sequence, floored to the index it is at.
#[allow(clippy::cast_sign_loss)]
pub(crate) fn index(x: f64) -> usize {
    x.max(0.0) as usize
}

pub(crate) const fn sp(duration: f32, bounce: f32) -> Spring {
    Spring::new(duration, bounce)
}

/// Press depth (0 → 1 → 0) for a tap at `tp`: down fast, back on a bouncy spring.
pub(crate) fn pressed(t: f64, tp: f64) -> f32 {
    track(
        t,
        0.0,
        &[(tp, 1.0, sp(0.12, 0.0)), (tp + 0.14, 0.0, sp(0.35, 0.45))],
    )
}

/// A round pointer that follows `path` (time, x, y) on springs and squeezes at each tap.
pub(crate) fn pointer(now: f64, start: [f32; 2], path: &[(f64, f32, f32)], taps: &[f64]) -> El {
    let glide = sp(0.55, 0.0);
    let kx: Vec<_> = path.iter().map(|&(tk, px, _)| (tk, px, glide)).collect();
    let ky: Vec<_> = path.iter().map(|&(tk, _, py)| (tk, py, glide)).collect();
    let (px, py) = (track(now, start[0], &kx), track(now, start[1], &ky));
    let press = taps
        .iter()
        .map(|&tp| pressed(now, tp))
        .fold(0.0f32, f32::max);
    let radius = 9.0 * (1.0 - 0.22 * press);
    el().absolute(px - radius, py - radius)
        .size(2.0 * radius, 2.0 * radius)
        .radius(radius)
        .fill(Color::WHITE.alpha(0.92))
        .border(1.0, Color::BLACK.alpha(0.35))
        .shadow(2.0, 8.0, Color::BLACK.alpha(0.45))
}

//! Shared look and timing helpers.

use rune::color::Color;
use rune::{El, TextStyle, text};
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

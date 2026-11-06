//! The compositor: every `El::effect` slot becomes one rune-fx layer. The slot's mark says which
//! effect and its `effect_data` the effect's live numbers. Colours arrive as `0xRRGGBB` packed in
//! an `f32` (exact below 2^24), so they come from Lumis themes like the code does. One-shot
//! effects take their age rather than a start time, so a scene keeps its own clock while effects
//! run on the reel's, `start + t`, and a recording that starts partway draws the same frames.

use rune::compose::{Compositor, Frame};
use rune::render::{EffectPhase, EffectSite, KIND_EFFECT};
use rune_fx::{
    DistortParams, ElectricParams, FireParams, FireStyle, Fx, FxFrame, FxLayer, FxOptions,
    FxRenderer, FxSlot, GlowParams, GlowStyle, GrainParams, LensParams, PlasticParams, PulseParams,
    PulseStyle, Rgb, Shape, ShimmerParams, SparkleParams, rgb,
};

use crate::State;

/// `[height, intensity, smoke, opacity]`
pub(crate) const FIRE: u32 = 1;
/// `[intensity, amplitude, charged (0 or 1), colour]`
pub(crate) const ELECTRIC: u32 = 2;
/// `[intensity, radius, aura (0 or 1), colour]`
pub(crate) const GLOW: u32 = 3;
/// `[age (no burst below 0), count, size, colour]`
pub(crate) const SPARKLES: u32 = 4;
/// `[period, reach, intensity, colour]`
pub(crate) const PULSE: u32 = 5;
/// `[intensity, period, band, colour]`
pub(crate) const SHIMMER: u32 = 6;
/// `[presence, magnification, frost, unused]`
pub(crate) const LENS: u32 = 7;
/// `[x, y, age, amount]`
pub(crate) const SHOCK: u32 = 8;
/// `[height, amount, unused, unused]`
pub(crate) const HAZE: u32 = 9;
/// `[colour, roughness, dome, bevel]`
pub(crate) const PLASTIC: u32 = 10;
/// `[amount, fps, unused, unused]`
pub(crate) const GRAIN: u32 = 11;

#[allow(clippy::cast_sign_loss)]
fn hue(x: f32) -> Rgb {
    rgb(x.max(0.0) as u32)
}

#[allow(clippy::cast_sign_loss)]
fn count(x: f32) -> u32 {
    x.max(0.0) as u32
}

fn layer(kind: u32, d: [f32; 4], shape: Shape, t: f32) -> Option<FxLayer> {
    let fx = match kind {
        FIRE => {
            let p = FireParams {
                style: FireStyle::Top,
                height: d[0],
                intensity: d[1],
                smoke: d[2],
                embers: 24,
                scale: 26.0,
                ..FireParams::default()
            };
            return Some(Fx::fire(p).on(shape).at(t).opacity(d[3]));
        }
        ELECTRIC => {
            let base = if d[2] > 0.5 {
                ElectricParams::charged()
            } else {
                ElectricParams::default()
            };
            Fx::electric(ElectricParams {
                intensity: d[0],
                amplitude: d[1],
                color: hue(d[3]),
                ..base
            })
        }
        GLOW => Fx::glow(GlowParams {
            style: if d[2] > 0.5 {
                GlowStyle::Aura
            } else {
                GlowStyle::Soft
            },
            color: hue(d[3]),
            intensity: d[0],
            radius: d[1],
            aura_speed: 0.35,
            ..Default::default()
        }),
        SPARKLES if d[0] < 0.0 => return None,
        SPARKLES => Fx::sparkles(SparkleParams {
            burst: Some(t - d[0]),
            count: count(d[1]),
            size: d[2],
            color: hue(d[3]),
            margin: 30.0,
            ..Default::default()
        }),
        PULSE => Fx::pulse(PulseParams {
            style: PulseStyle::Radar,
            color: hue(d[3]),
            period: d[0],
            reach: d[1],
            intensity: d[2],
            ..Default::default()
        }),
        SHIMMER => Fx::shimmer(ShimmerParams {
            intensity: d[0],
            period: d[1],
            band: d[2],
            color: hue(d[3]),
            ..Default::default()
        }),
        LENS => Fx::lens(LensParams {
            presence: d[0],
            magnification: d[1],
            frost: d[2],
            bevel: 18.0,
            depth: 22.0,
            dispersion: 0.2,
            ..Default::default()
        }),
        SHOCK => Fx::distort(DistortParams::Shockwave {
            at: [d[0], d[1]],
            reach: 1100.0,
            width: 90.0,
            amount: d[3],
            chroma: 0.6,
            t0: t - d[2],
            duration: 1.1,
        }),
        HAZE => Fx::distort(DistortParams::HeatHaze {
            height: d[0],
            scale: 14.0,
            speed: 1.4,
            amount: d[1],
            whole_shape: false,
        }),
        PLASTIC => Fx::fill(PlasticParams {
            color: hue(d[0]),
            roughness: d[1],
            dome: d[2],
            bevel: d[3],
            angle: -0.6,
        }),
        GRAIN => Fx::grain(GrainParams {
            amount: d[0],
            fps: d[1],
            ..Default::default()
        }),
        _ => return None,
    };
    Some(fx.on(shape).at(t))
}

#[derive(Default)]
pub(crate) struct Gfx {
    fx: Option<FxRenderer>,
}

impl Compositor<State> for Gfx {
    fn animating(&self) -> bool {
        true
    }

    fn render(&mut self, frame: &mut Frame<'_, State>) {
        let (device, queue, size) = (frame.device, frame.queue, frame.size);
        let time = (frame.rt.state.start + frame.rt.state.t) as f32;
        let marks = &frame.rt.marks;
        let mut at: Vec<u32> = Vec::new();
        let mut layers: Vec<FxLayer> = Vec::new();
        let mut placements: Vec<FxSlot> = Vec::new();
        let mut fire = false;
        for (slot, inst) in frame.rt.instances.iter().enumerate() {
            if inst.params[1] != KIND_EFFECT {
                continue;
            }
            let kind = marks.get(slot).copied().unwrap_or(0) & 0xff;
            let scale = inst.params[3];
            let [x, y, w, h] = inst.rect.map(|v| v / scale);
            let radius = inst.radii.iter().fold(0.0f32, |m, v| m.max(v.abs())) / scale;
            let Some(fx_layer) = layer(kind, inst.uv, Shape::rounded(x, y, w, h, radius), time)
            else {
                continue;
            };
            fire |= kind == FIRE;
            at.push(slot as u32);
            layers.push(fx_layer);
            placements.push(FxSlot::default().scale(scale).clip(inst.clip, inst.fade));
        }
        let fx = self.fx.get_or_insert_with(|| {
            let mut fx = FxRenderer::new(device, frame.format, FxOptions { dither: true });
            fx.set_fire_sim(device, true);
            fx
        });
        fx.set_srgb_blend(frame.srgb_blend());
        fx.add_format(device, wgpu::TextureFormat::Rgba16Float);
        fx.prepare_with_slots(
            device,
            queue,
            &FxFrame {
                viewport: [size.0, size.1],
                scale: 1.0,
                layers: &layers,
                backdrop: None,
                sources: &[],
            },
            &placements,
        );
        if fire {
            fx.simulate(queue, frame.encoder, None);
        }
        let clear = frame.rt.clear_color();
        frame.ui.prepare(
            device,
            queue,
            &mut frame.rt.text.atlas,
            &frame.rt.instances,
            size,
        );
        frame
            .ui
            .draw_with_backdrop(frame.encoder, frame.target, clear, &mut |phase,
                                                                          s: &EffectSite<
                '_,
            >| {
                let Some(k) = at.iter().position(|&slot| slot == s.index) else {
                    return;
                };
                match phase {
                    EffectPhase::Backdrop(enc) => {
                        if let Some(bd) = s.backdrop {
                            fx.blur_backdrop(device, queue, enc, bd, k);
                        }
                    }
                    EffectPhase::Draw(pass) => fx.draw_layer_in(pass, s.format, s.origin, k),
                }
            });
    }
}

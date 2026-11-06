//! Lumis in Rune: every bundled language and theme, animated. Each scene runs off the (virtual)
//! clock, so a headless `--video` run records it frame-exact:
//!
//!     lumis-rune-showreel --scene reel --video reel.mp4 --size 960x540 --scale 2 --duration 33
//!
//! `reel@N` starts the reel N seconds in, on a fresh clock, so a long recording can be made in
//! segments without simulating the frames before each one.

mod data;
mod gfx;
mod kit;
mod scenes;

use rune::{AppSpec, Cx, El};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(crate) enum Scene {
    #[default]
    Reel,
    Type,
    Themes,
    Langs,
    Annotate,
}

#[derive(Default)]
pub(crate) struct State {
    pub(crate) scene: Scene,
    /// Seconds since the scene started (Rune's virtual clock when headless).
    pub(crate) t: f64,
    /// Seconds into the scene the clock starts at.
    pub(crate) start: f64,
}

fn view(s: &State, _: &mut Cx<State>) -> El {
    let t = s.start + s.t;
    let root = match s.scene {
        Scene::Reel => scenes::reel(t),
        Scene::Type => scenes::type_::view(t),
        Scene::Themes => scenes::themes::view(t),
        Scene::Langs => scenes::langs::view(t),
        Scene::Annotate => scenes::annotate::view(t),
    };
    root.instant_changes()
}

fn main() {
    // Tokenise every sample before the first frame rather than during it.
    let _ = data::snippets();
    let app = rune::App::new(AppSpec::<State> {
        scenes: &[
            ("reel", |s| s.scene = Scene::Reel),
            ("type", |s| s.scene = Scene::Type),
            ("themes", |s| s.scene = Scene::Themes),
            ("langs", |s| s.scene = Scene::Langs),
            ("annotate", |s| s.scene = Scene::Annotate),
            ("reel@2", |s| s.start = 2.0),
            ("reel@4", |s| s.start = 4.0),
            ("reel@6", |s| s.start = 6.0),
            ("reel@8", |s| s.start = 8.0),
            ("reel@10", |s| s.start = 10.0),
            ("reel@12", |s| s.start = 12.0),
            ("reel@14", |s| s.start = 14.0),
            ("reel@16", |s| s.start = 16.0),
            ("reel@18", |s| s.start = 18.0),
            ("reel@20", |s| s.start = 20.0),
            ("reel@22", |s| s.start = 22.0),
            ("reel@24", |s| s.start = 24.0),
            ("reel@26", |s| s.start = 26.0),
            ("reel@28", |s| s.start = 28.0),
            ("reel@30", |s| s.start = 30.0),
            ("reel@32", |s| s.start = 32.0),
        ],
        fonts: &[
            include_bytes!("../assets/Inter-Regular.ttf"),
            include_bytes!("../assets/Inter-Medium.ttf"),
            include_bytes!("../assets/Inter-SemiBold.ttf"),
            include_bytes!("../assets/IBMPlexMono-Regular.ttf"),
            include_bytes!("../assets/IBMPlexMono-Medium.ttf"),
        ],
        families: ("Inter", "IBM Plex Mono"),
        // Every frame is drawn from the clock, so colours land at once rather than springing:
        // a theme that changes every few frames would otherwise average out to grey.
        poll: Some(|s, t, tx| {
            s.t = t;
            tx.instant();
            true
        }),
        ..AppSpec::new("Lumis × Rune", (kit::W, kit::H), State::default, view)
    })
    .compositor(gfx::Gfx::default());
    app.run();
}

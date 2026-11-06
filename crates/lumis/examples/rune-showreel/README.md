# Lumis × Rune showreel

A 26-second reel of an editor built in [Rune](https://github.com/anomalyco/rune), a GPU-rendered
Rust UI framework, whose code Lumis colours. Effects are
[rune-fx](https://github.com/anomalyco/rune/tree/dev/crates/rune-fx) shaders. Six shots:

- **ignite**: grey code lights up token by token in its Lumis colours, then a test types itself.
- **slots**: the theme picker spins through every bundled theme, repainting the whole editor, and
  lands with a shockwave.
- **review**: a pull request whose changed words come from Lumis's word diff, laid over the Rust
  highlighting as annotations. Approving it sparkles.
- **burn**: compiler diagnostics and a profile arrive as annotations on exact byte ranges; the
  hottest line catches fire.
- **blitz**: a cut every beat, a different language and theme each time.
- **langs**: one card per language in `samples/`, zooming out to the full count.

From this directory, run it in a window:

```sh
cargo run --release
```

Render it headless. Scenes run off Rune's virtual clock, so the recording is frame-exact:

```sh
cargo run --release -- --video reel.mp4 --size 960x540 --scale 2 --duration 27
cargo run --release -- --scene reel@12 --at 1 --snap burn.png
```

On Linux without a GPU, use Mesa's software Vulkan driver:

```sh
VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.x86_64.json cargo run --release -- --scene reel --video reel.mp4
```

Under that driver the headless recorder holds about 20 MB per frame, so a whole reel does not
fit in 8 GB. `reel@N` starts the reel N seconds in on a fresh clock (N = 2, 4 … 30); record 2 s
segments and join them with `ffmpeg -f concat -c copy`.

To colour Markdown code fences in a Rune app with Lumis, use `rune_kit::syntax::Highlighter`; see
https://docs.lumis.sh/integrations/rune. This reel draws code with `rune::rich` instead, one span
per token, so it keeps each theme's bold and italic.

Fonts in `assets/` are Inter and IBM Plex Mono, under the SIL Open Font License.

//! The Lumis inspector: pick a sample and a theme, put the caret anywhere, and see the
//! syntax node, the captures and the theme styles behind that byte.
//!
//! ```sh
//! mise run inspect
//! lumis-inspect --scene caret --snap inspect.png --size 1440x900
//! ```

mod analysis;
mod editor;
mod panel;
mod samples;
mod style;

use std::cell::RefCell;

use analysis::{Analysis, Outcome, Request, Worker};
use lumis_core::themes::{self, Appearance, Theme};
use rune::{AppSpec, Cx, El, TextEdit, Tx, Waker};
use rune_kit::{Combobox, ComboboxOption, ComboboxState, Kit, PaneState, SplitView, Toolbar};

pub(crate) struct Inspector {
    pub(crate) samples: Vec<samples::Sample>,
    pub(crate) sample: usize,
    pub(crate) themes: Vec<&'static Theme>,
    pub(crate) theme: usize,
    pub(crate) edit: TextEdit,
    pub(crate) sample_picker: ComboboxState,
    pub(crate) theme_picker: ComboboxState,
    pub(crate) pane: PaneState,
    pub(crate) worker: Option<Worker>,
    pub(crate) analysis: Option<Analysis>,
    pub(crate) error: Option<String>,
    /// The id, language and text of the last highlight asked for.
    pub(crate) sent: Option<(u64, String, String)>,
    /// A node of the tree under the pointer, outlined in the editor.
    pub(crate) hovered: Option<usize>,
    caret_node: Option<(u64, usize)>,
    /// A node picked in the tree or the breadcrumb: `(analysis, node, caret)`. It stays the
    /// one described until the caret moves.
    picked: Option<(u64, usize, usize)>,
    pub(crate) paint: RefCell<editor::Paint>,
}

impl Inspector {
    fn new() -> Self {
        let samples = samples::all();
        let themes: Vec<_> = themes::available().collect();
        let sample = samples
            .iter()
            .position(|s| s.file == "rust.rs")
            .unwrap_or(0);
        let theme = themes
            .iter()
            .position(|t| t.name == "github_dark")
            .unwrap_or(0);
        let mut inspector = Self {
            samples,
            sample,
            themes,
            theme,
            edit: TextEdit::default(),
            sample_picker: ComboboxState::default(),
            theme_picker: ComboboxState::default(),
            pane: PaneState::new(440.0).range(320.0, 720.0),
            worker: None,
            analysis: None,
            error: None,
            sent: None,
            hovered: None,
            caret_node: None,
            picked: None,
            paint: RefCell::default(),
        };
        inspector.open(sample);
        inspector
    }

    fn open(&mut self, sample: usize) {
        self.sample = sample;
        let text = self
            .samples
            .get(sample)
            .map(samples::read)
            .unwrap_or_default();
        self.edit = TextEdit::new(text);
        self.edit.set_caret(0);
        self.edit.focused = true;
        self.hovered = None;
    }

    pub(crate) fn theme(&self) -> &'static Theme {
        self.themes[self.theme]
    }

    pub(crate) fn language(&self) -> &str {
        self.samples
            .get(self.sample)
            .map_or("plaintext", |s| s.language.as_str())
    }

    /// The analysis of the text on screen, or of the text just before the last edit while
    /// its highlight is still running.
    pub(crate) fn current(&self) -> Option<&Analysis> {
        self.analysis
            .as_ref()
            .filter(|a| a.language == self.language())
    }

    pub(crate) fn pending(&self) -> bool {
        self.sent
            .as_ref()
            .is_some_and(|(id, ..)| self.analysis.as_ref().is_none_or(|a| a.id != *id))
            && self.error.is_none()
    }

    /// The byte the panel describes: the caret, or the start of the selection.
    pub(crate) fn caret(&self) -> usize {
        self.edit
            .anchor
            .min(self.edit.focus)
            .min(self.edit.text.len())
    }

    pub(crate) fn place_caret(&mut self, byte: usize) {
        self.edit.set_caret(byte);
        self.edit.focused = true;
    }

    /// The node the pane describes: the one picked, or the deepest at the caret.
    pub(crate) fn node(&self) -> Option<usize> {
        let analysis = self.current()?;
        match self.picked {
            Some((id, node, caret)) if id == analysis.id && caret == self.caret() => Some(node),
            _ => analysis.inspection.node_at(self.caret()),
        }
    }

    /// Describe `node` and put the caret at its start, without scrolling the tree it was
    /// picked from.
    /// `analysis` is the one the row or crumb was drawn from: a press that lands after a newer
    /// highlight replaced it picks nothing.
    pub(crate) fn pick(&mut self, analysis: u64, node: usize) {
        let Some(start) = self
            .current()
            .filter(|a| a.id == analysis)
            .and_then(|a| a.inspection.nodes.get(node))
            .map(|n| n.byte_range.start)
        else {
            return;
        };
        let id = analysis;
        self.place_caret(start);
        self.picked = Some((id, node, self.caret()));
        self.caret_node = Some((id, node));
    }

    fn request(&mut self) -> bool {
        let unchanged = self.sent.as_ref().is_some_and(|(_, language, text)| {
            language == self.language() && *text == self.edit.text
        });
        let Some(worker) = &self.worker else {
            return false;
        };
        if unchanged {
            return false;
        }
        let id = self.sent.as_ref().map_or(1, |(id, ..)| id + 1);
        let request = Request {
            id,
            language: self.language().to_string(),
            text: self.edit.text.clone(),
        };
        worker.send(request);
        self.sent = Some((id, self.language().to_string(), self.edit.text.clone()));
        self.error = None;
        true
    }

    fn accept(&mut self, outcome: Outcome) -> bool {
        if self.sent.as_ref().is_none_or(|(id, ..)| *id != outcome.id) {
            return false;
        }
        match outcome.result {
            Ok(analysis) => {
                self.analysis = Some(analysis);
                self.error = None;
            }
            Err(error) => {
                self.analysis = None;
                self.error = Some(error);
            }
        }
        self.hovered = None;
        true
    }

    /// Keep the caret's node in view in the tree as the caret moves.
    fn follow_caret(&mut self, tx: &mut Tx) {
        let Some(at) = self.current().map(|a| a.id).zip(self.node()) else {
            return;
        };
        if self.caret_node != Some(at) {
            self.caret_node = Some(at);
            tx.scroll_center(panel::row_key(at.0, at.1));
        }
    }
}

fn setup(s: &mut Inspector, wake: Waker) {
    s.worker = Some(Worker::spawn(wake));
}

fn poll(s: &mut Inspector, _: f64, tx: &mut Tx) -> bool {
    let mut changed = s.request();
    let wait = s.pending() && rune::is_headless();
    let outcome = s.worker.as_ref().and_then(|w| w.receive(wait));
    if let Some(outcome) = outcome {
        changed |= s.accept(outcome);
    }
    s.follow_caret(tx);
    changed
}

fn header(s: &Inspector, k: &Kit, overlays: &mut Vec<El>) -> Toolbar {
    let samples: Vec<_> = s
        .samples
        .iter()
        .enumerate()
        .map(|(i, sample)| {
            let mut option = ComboboxOption::new(i.to_string(), &sample.file);
            option.detail.clone_from(&sample.language);
            option
        })
        .collect();
    let themes: Vec<_> = s
        .themes
        .iter()
        .enumerate()
        .map(|(i, theme)| {
            let mut option = ComboboxOption::new(i.to_string(), &theme.name);
            option.detail = appearance(theme).to_string();
            option
        })
        .collect();
    let sample = Combobox::new("sample", &s.sample_picker, &samples, |s: &mut Inspector| {
        &mut s.sample_picker
    })
    .label("Sample")
    .placeholder("Choose a sample")
    .search_placeholder("Search samples")
    .current(Some(&s.sample.to_string()))
    .width(240.0)
    .max_height(520.0)
    .on_choose(|s, id, _| {
        if let Ok(index) = id.parse() {
            s.open(index);
        }
    })
    .build(k, overlays);
    let theme = Combobox::new("theme", &s.theme_picker, &themes, |s: &mut Inspector| {
        &mut s.theme_picker
    })
    .label("Theme")
    .placeholder("Choose a theme")
    .search_placeholder("Search themes")
    .current(Some(&s.theme.to_string()))
    .width(260.0)
    .max_height(520.0)
    .on_choose(|s, id, _| {
        if let Ok(index) = id.parse() {
            s.theme = index;
        }
    })
    .build(k, overlays);
    Toolbar::new().leading(
        rune::row()
            .gap(8.0)
            .items_center()
            .child(sample)
            .child(theme),
    )
}

pub(crate) fn appearance(theme: &Theme) -> &'static str {
    match theme.appearance {
        Appearance::Light => "light",
        Appearance::Dark => "dark",
    }
}

fn view(s: &Inspector, cx: &mut Cx<Inspector>) -> El {
    let dark = s.theme().appearance == Appearance::Dark;
    let k = rune_theme_opencode::kit(dark);
    let mut overlays = Vec::new();
    let toolbar = header(s, k, &mut overlays);
    let shell = SplitView::new(editor::build(s, k))
        .toolbar(toolbar)
        .inspector(&s.pane, |s: &mut Inspector| &mut s.pane, panel::build(s, k))
        .hide_sidebar_toggle()
        .titlebar_inset(cx.titlebar_inset())
        .build(k);
    rune_kit::root(shell, overlays)
}

fn main() {
    rune::run(AppSpec {
        scenes: &[
            ("default", |_: &mut Inspector| {}),
            ("caret", |s: &mut Inspector| {
                s.place_caret(caret_scene_byte(s));
            }),
            ("light", |s: &mut Inspector| {
                s.theme = s
                    .themes
                    .iter()
                    .position(|t| t.name == "github_light")
                    .unwrap_or(s.theme);
                s.place_caret(caret_scene_byte(s));
            }),
            ("injection", |s: &mut Inspector| {
                if let Some(i) = s.samples.iter().position(|x| x.file == "html.html") {
                    s.open(i);
                    let byte = s.edit.text.find("addEventListener").map_or(0, |b| b + 3);
                    s.place_caret(byte);
                }
            }),
        ],
        fonts: rune_theme_opencode::FONTS,
        families: rune_theme_opencode::FAMILIES,
        setup: Some(setup),
        poll: Some(poll),
        ..AppSpec::new("Lumis Inspector", (1440.0, 900.0), Inspector::new, view)
    });
}

/// Inside the first `fn` of the sample, so the scenes show a keyword's captures.
fn caret_scene_byte(s: &Inspector) -> usize {
    s.edit.text.find("fn ").map_or(0, |b| b + 1)
}

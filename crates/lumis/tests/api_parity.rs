//! Rust's half of the cross-runtime top-level API check.
//!
//! `fixtures/api.json` lists the entry points every runtime must offer a caller.
//! Rust cannot reflect on a module at run time, so the check has four parts:
//!
//! - `every_manifest_capability_is_callable` calls every capability by name. A
//!   capability added to the manifest that Rust lacks fails to **compile**.
//! - `manifest_matches_the_entry_points_exercised_here` reads the manifest and
//!   requires it to name exactly the functions exercised above.
//! - `every_public_entry_point_is_accounted_for` parses `lib.rs` and
//!   `highlight.rs` and fails on a module-level `pub fn` that is in neither the
//!   canonical set nor `runtime_only`. This is the one that catches drift: an
//!   entry point added to Rust and nowhere else has to be classified before it
//!   can ship.
//! - `every_runtime_only_entry_point_still_exists` fails on a `runtime_only`
//!   name Rust no longer exports, so the list cannot outlive its reasons.
//!
//! A capability whose home is outside those two files carries a `spelling` with
//! `::` in it and is proven by the call alone, because the scan cannot see it.
//!
//! Gated on `lang-rust` because every capability here takes a `Language`, and the
//! catalog is feature-gated.
#![cfg(feature = "lang-rust")]

use lumis::highlight::{
    highlight_events, highlight_events_with_options, highlight_iter, highlight_iter_with_options,
};
use lumis::languages::Language;
use lumis::{
    highlight, highlight_with_options, languages, themes, HighlightOptions, HtmlInlineBuilder,
};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr as _;
use syn::{Item, Meta};

const RUNTIME: &str = "rust";

#[derive(Debug, Deserialize)]
struct Manifest {
    capabilities: Vec<Capability>,
    runtime_only: BTreeMap<String, serde_json::Value>,
    waived: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct Capability {
    name: String,
    #[serde(default)]
    spelling: BTreeMap<String, Spelling>,
}

/// A runtime spells one capability with one name, or splits it across several.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Spelling {
    One(String),
    Many(Vec<String>),
}

impl Spelling {
    fn names(&self) -> Vec<&str> {
        match self {
            Spelling::One(name) => vec![name.as_str()],
            Spelling::Many(names) => names.iter().map(String::as_str).collect(),
        }
    }
}

impl Capability {
    /// The names Rust spells this capability, which is the canonical one unless
    /// the manifest overrides it.
    fn rust_names(&self) -> Vec<&str> {
        self.spelling
            .get(RUNTIME)
            .map_or_else(|| vec![self.name.as_str()], Spelling::names)
    }
}

fn manifest() -> Manifest {
    let path = fixtures_dir().join("api.json");
    serde_json::from_str(&fs::read_to_string(&path).expect("read api.json"))
        .expect("parse api.json")
}

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .canonicalize()
        .expect("fixtures/ is reachable")
}

/// Every capability in the manifest, called once. Adding one to the manifest
/// without adding it here fails `manifest_matches_the_entry_points_exercised_here`;
/// adding it here without a Rust function fails to compile.
///
/// The values are the Rust spellings, so a spelling naming a home outside the
/// scanned files is accounted for by this call and not by the scan.
fn exercised_entry_points() -> BTreeSet<&'static str> {
    let source = "fn main() { let x = 1; }\n";
    let language = Language::Rust;
    let theme = themes::get("dracula").expect("dracula is a built-in theme");
    let formatter = HtmlInlineBuilder::new()
        .language(language)
        .theme(Some(theme.clone()))
        .build()
        .expect("html_inline builds");
    let with_options_formatter = HtmlInlineBuilder::new()
        .language(language)
        .theme(Some(theme.clone()))
        .build()
        .expect("html_inline builds");

    // Each `_with_options` sibling gets the default options, so it has to agree
    // with the plain one; that is a real property and not just "did not panic".
    let html = highlight(source, formatter);
    assert!(
        html.contains("<span"),
        "highlight rendered no spans: {html}"
    );
    assert_eq!(
        html,
        highlight_with_options(source, with_options_formatter, HighlightOptions::new()),
        "default options changed the rendered output"
    );

    let events = highlight_events(source, language).expect("highlight_events succeeds");
    assert!(
        events.iter().any(|event| event.scope() == Some("keyword")),
        "highlight_events found no keyword in {source:?}"
    );
    assert_eq!(
        events,
        highlight_events_with_options(source, language, HighlightOptions::new())
            .expect("highlight_events_with_options succeeds"),
        "default options changed the event stream"
    );

    let mut tokens = 0usize;
    highlight_iter(source, language, Some(theme.clone()), |_, _, _, _, _| {
        tokens += 1;
        Ok::<_, std::io::Error>(())
    })
    .expect("highlight_iter succeeds");
    highlight_iter_with_options(
        source,
        language,
        Some(theme),
        HighlightOptions::new(),
        |_, _, _, _, _| {
            tokens += 1;
            Ok::<_, std::io::Error>(())
        },
    )
    .expect("highlight_iter_with_options succeeds");
    assert!(tokens > 0, "highlight_iter yielded no tokens");

    assert_eq!(Language::guess(Some("main.rs"), "").id_name(), "rust");
    assert!(
        languages::available_languages()
            .iter()
            .any(|info| info.id == "rust"),
        "available_languages does not list Rust"
    );
    assert!(
        themes::available_themes().any(|theme| theme.name == "dracula"),
        "available_themes does not list dracula"
    );
    assert_eq!(
        Language::from_str("rust")
            .expect("`rust` is in the catalog")
            .info()
            .id,
        "rust"
    );

    [
        "highlight",
        "highlight_with_options",
        "highlight_events",
        "highlight_events_with_options",
        "highlight_iter",
        "highlight_iter_with_options",
        "languages::Language::guess",
        "languages::available_languages",
        "themes::available_themes",
        "languages::Language::from_str",
        "languages::Language::info",
    ]
    .into()
}

/// Every module-level `pub fn` a caller reaches first, and whether it carries
/// `#[deprecated]`. `#[doc(hidden)]` is not public API, so it is skipped, which
/// is what keeps `highlight_events_for_render` out of the manifest.
fn public_entry_points() -> BTreeMap<String, bool> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let sources = [
        manifest_dir.join("src/lib.rs"),
        manifest_dir.join("src/highlight.rs"),
    ];

    let mut entry_points = BTreeMap::new();
    for path in &sources {
        collect_public_fns(path, &mut entry_points);
    }

    assert!(
        entry_points.len() > 4,
        "source scan found almost nothing: {} entry points",
        entry_points.len()
    );

    entry_points
}

fn collect_public_fns(path: &Path, entry_points: &mut BTreeMap<String, bool>) {
    let source =
        fs::read_to_string(path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    let syntax = syn::parse_file(&source)
        .unwrap_or_else(|error| panic!("parse {}: {error}", path.display()));

    for item in &syntax.items {
        let Item::Fn(item) = item else { continue };

        if !matches!(item.vis, syn::Visibility::Public(_)) {
            continue;
        }

        if item.attrs.iter().any(is_doc_hidden) {
            continue;
        }

        let deprecated = item.attrs.iter().any(|attr| match &attr.meta {
            Meta::Path(path) | Meta::List(syn::MetaList { path, .. }) => {
                path.is_ident("deprecated")
            }
            Meta::NameValue(_) => false,
        });

        entry_points.insert(item.sig.ident.to_string(), deprecated);
    }
}

fn is_doc_hidden(attr: &syn::Attribute) -> bool {
    let Meta::List(list) = &attr.meta else {
        return false;
    };

    list.path.is_ident("doc") && list.tokens.to_string().trim() == "hidden"
}

fn names_under(section: &BTreeMap<String, serde_json::Value>) -> BTreeSet<String> {
    section
        .get(RUNTIME)
        .and_then(serde_json::Value::as_object)
        .map(|names| {
            names
                .keys()
                .filter(|name| !name.starts_with('$'))
                .cloned()
                .collect()
        })
        .unwrap_or_default()
}

/// Every name Rust is required to offer: the canonical set minus what `waived`
/// exempts it from.
fn required_names(manifest: &Manifest) -> BTreeSet<&str> {
    let waived = names_under(&manifest.waived);

    manifest
        .capabilities
        .iter()
        .filter(|capability| !waived.contains(&capability.name))
        .flat_map(Capability::rust_names)
        .collect()
}

/// The required names the scan can see: a spelling naming a home outside
/// `lib.rs` and `highlight.rs` is proven by the call instead.
fn required_in_scope(manifest: &Manifest) -> BTreeSet<&str> {
    required_names(manifest)
        .into_iter()
        .filter(|name| !name.contains("::"))
        .collect()
}

#[test]
fn every_manifest_capability_is_callable() {
    // The capabilities run here, so reaching this line is the proof. Whether the
    // set matches the manifest is the next test's job.
    assert_ne!(exercised_entry_points(), BTreeSet::new());
}

#[test]
fn manifest_matches_the_entry_points_exercised_here() {
    let manifest = manifest();

    assert_eq!(
        required_names(&manifest),
        exercised_entry_points(),
        "the manifest and the Rust functions exercised here disagree"
    );
}

#[test]
fn every_public_entry_point_is_accounted_for() {
    let manifest = manifest();
    let required = required_in_scope(&manifest);
    let runtime_only = names_under(&manifest.runtime_only);
    // A waived name is accounted for, so a runtime that grows one fails
    // `no_waiver_outlives_its_reason` alone rather than here as well.
    let waived = names_under(&manifest.waived);

    let public = public_entry_points();
    let unaccounted: Vec<&String> = public
        .keys()
        .filter(|name| {
            !required.contains(name.as_str())
                && !runtime_only.contains(*name)
                && !waived.contains(*name)
        })
        .collect();

    assert!(
        unaccounted.is_empty(),
        "public entry points in neither the manifest's capability set nor runtime_only: \
         {unaccounted:?}. Add them to fixtures/api.json and to the other runtimes, or \
         classify them."
    );
}

#[test]
fn every_runtime_only_entry_point_still_exists() {
    let public = public_entry_points();

    for name in names_under(&manifest().runtime_only) {
        assert!(
            public.contains_key(&name),
            "runtime_only lists {name}, which Rust no longer exports; drop the entry"
        );
    }
}

#[test]
fn no_waiver_outlives_its_reason() {
    let public = public_entry_points();

    for name in names_under(&manifest().waived) {
        assert!(
            !public.contains_key(&name),
            "waived lists {name}, which Rust offers; drop the waiver"
        );
    }
}

#[test]
fn every_waiver_names_a_capability() {
    let manifest = manifest();
    let canonical: BTreeSet<&str> = manifest
        .capabilities
        .iter()
        .map(|capability| capability.name.as_str())
        .collect();

    for (runtime, waived) in &manifest.waived {
        if runtime.starts_with('$') {
            continue;
        }

        let names = waived
            .as_object()
            .unwrap_or_else(|| panic!("waived.{runtime} is a table of names"));

        for name in names.keys().filter(|name| !name.starts_with('$')) {
            assert!(
                canonical.contains(name.as_str()),
                "waived.{runtime} lists {name}, which is not a capability in this manifest; \
                 a waiver only exempts a runtime from the canonical set"
            );
        }
    }
}

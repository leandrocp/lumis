//! The repository's `samples/` files, one per language, named by language id.

use std::path::{Path, PathBuf};

use lumis_core::languages::Language;
use lumis_wasm_runtime::catalog;

pub(crate) struct Sample {
    pub(crate) file: String,
    pub(crate) path: PathBuf,
    pub(crate) language: String,
}

pub(crate) fn dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../samples")
}

pub(crate) fn all() -> Vec<Sample> {
    let Ok(entries) = std::fs::read_dir(dir()) else {
        return Vec::new();
    };
    let mut samples: Vec<Sample> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|file| file != "README.md" && file != "LICENSE.md")
        .map(|file| Sample {
            language: language(&file),
            path: dir().join(&file),
            file,
        })
        .collect();
    samples.sort_by(|a, b| a.file.cmp(&b.file));
    samples
}

/// The filename before the first dot is the language id, as the website and the query
/// tests read it. A few files predate their id and fall back to detection.
fn language(file: &str) -> String {
    let stem = file.split('.').next().unwrap_or(file);
    catalog::find(stem).map_or_else(
        || Language::guess(Some(file), "").id_name().to_string(),
        |entry| entry.id.to_string(),
    )
}

pub(crate) fn read(sample: &Sample) -> String {
    std::fs::read_to_string(&sample.path).unwrap_or_default()
}

//! Every patch `languages.toml` declares is present in the vendored sources.
//!
//! A patch is applied to a fresh checkout by
//! `mise run langs-fetch-vendored-parsers`, so a parser refetched without it —
//! by a revision bump, or by running the underlying `fetch-parsers` command
//! against an older manifest — comes back carrying upstream's behaviour and
//! nothing says so. The parsers still build and still highlight; they are only
//! slow again, on inputs no unit test has.
//!
//! This reads what the build actually compiles, which is the compressed source
//! under `vendored_parsers/`, rather than the patch file beside it.

use std::{fs, io::Read, path::PathBuf};

/// A marker that only the patch puts in a vendored source, and the source it
/// has to appear in.
const PATCHED_SOURCES: &[(&str, &str, &str)] = &[(
    "gleam",
    "tree-sitter-gleam/src/scanner.c.xz",
    "valid_symbols[ERROR_SENTINEL]",
)];

fn vendored_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("vendored_parsers")
}

fn read_source(relative_path: &str) -> String {
    let path = vendored_root().join(relative_path);
    let bytes = fs::read(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));

    if path.extension().is_none_or(|extension| extension != "xz") {
        return String::from_utf8_lossy(&bytes).into_owned();
    }

    let mut source = String::new();
    xz2::read::XzDecoder::new(&bytes[..])
        .read_to_string(&mut source)
        .unwrap_or_else(|error| panic!("failed to decompress {}: {error}", path.display()));
    source
}

#[test]
fn vendored_parsers_carry_the_patches_lumis_declares() {
    for (parser, relative_path, marker) in PATCHED_SOURCES {
        let source = read_source(relative_path);

        assert!(
            source.contains(marker),
            "vendored {parser} parser is missing `{marker}`, which its patch in `patches/` \
             adds. Run: mise run langs-fetch-vendored-parsers {parser}"
        );
    }
}

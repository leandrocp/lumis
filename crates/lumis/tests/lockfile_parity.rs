//! Every lockfile in this repository resolves the same tree-sitter.
//!
//! The workspace is not the only thing here with a `Cargo.lock`. `crates/dev`
//! and `benchmarks/rust` declare their own `[workspace]`, so they neither
//! inherit the root's resolution nor appear in its lockfile, and the packaged
//! NIF ships a lockfile of its own because `rustler_precompiled` builds it from
//! the published tarball, where this workspace does not exist.
//!
//! Nothing failed when they drifted, which is why they did: `crates/dev` and
//! `benchmarks/rust` sat on tree-sitter 0.26.11 while the library shipped
//! 0.26.13. That crate is not a detail those tools can differ on. `crates/dev`
//! generates `fixtures/conformance` and runs `stress-test/`, so it was
//! generating the expected output and measuring the corpus against a parser the
//! library does not use, and on one stress case the two versions differ by 15x
//! in wall time (<https://github.com/leandrocp/lumis/issues/1447>).
//!
//! Adding a workspace means adding it here. Read `LOCKFILES`, not this
//! paragraph, for the current list.

use std::{fs, path::PathBuf};

/// Every tracked `Cargo.lock`, relative to the repository root.
const LOCKFILES: &[&str] = &[
    "Cargo.lock",
    "crates/dev/Cargo.lock",
    "benchmarks/rust/Cargo.lock",
    "packages/elixir/lumis/native/lumis_nif/Cargo.lock",
];

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// The version a lockfile resolves `name` to, if it holds that package at all.
fn locked_version(lockfile: &str, name: &str) -> Option<String> {
    let path = repository_root().join(lockfile);
    let contents = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));

    let package = format!("name = \"{name}\"\n");
    let start = contents.find(&package)? + package.len();
    let version = contents[start..].strip_prefix("version = \"")?;

    Some(version[..version.find('"')?].to_string())
}

#[test]
fn every_lockfile_resolves_the_same_tree_sitter() {
    let expected = locked_version("Cargo.lock", "tree-sitter")
        .expect("the workspace lockfile must resolve tree-sitter");

    for lockfile in LOCKFILES {
        let Some(version) = locked_version(lockfile, "tree-sitter") else {
            continue;
        };

        assert_eq!(
            version, expected,
            "{lockfile} resolves tree-sitter {version}, the workspace resolves {expected}. \
             Run: cargo update --manifest-path <that workspace> -p tree-sitter --precise {expected}"
        );
    }
}

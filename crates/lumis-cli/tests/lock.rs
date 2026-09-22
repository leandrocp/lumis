//! `lumis languages add|remove|update|install` against a staged store.
//!
//! These run against the staged fixture store rather than a CDN, which is also
//! the deployment shape the lock exists to make reproducible. The one exception
//! is noted where it happens: refusing a version the store holds necessarily
//! falls through to a fetch, and that fetch is expected to fail.

use assert_cmd::cargo::cargo_bin_cmd;
use lumis_wasm_runtime::sha256_hex;
use predicates::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

mod common;

/// A git repository with a nested directory, so lock discovery has something to
/// walk up from.
struct Project {
    _dir: tempfile::TempDir,
    root: PathBuf,
    nested: PathBuf,
}

fn project() -> Project {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().to_path_buf();
    fs::create_dir_all(root.join(".git")).unwrap();
    let nested = root.join("apps").join("web");
    fs::create_dir_all(&nested).unwrap();
    Project {
        _dir: dir,
        root,
        nested,
    }
}

fn cmd(cwd: &Path) -> assert_cmd::Command {
    let mut command = cargo_bin_cmd!("lumis");
    command.env(
        "LUMIS_CONFIG",
        common::source_fixtures_dir().join("missing-config.toml"),
    );
    command.env("LUMIS_DATA_DIR", common::data_dir());
    command.current_dir(cwd);
    command
}

fn lock_path(project: &Project) -> PathBuf {
    project.root.join("lumis-lock.toml")
}

/// A lock naming the staged fixture for `language`, with the digests that
/// fixture actually has, so `install` can be satisfied without a network.
fn write_lock_for(project: &Project, language: &str) {
    let location = lumis_wasm_runtime::catalog::find(language).unwrap();
    let suffix = lumis_wasm_runtime::package_suffix(location.package_name).unwrap();
    let manifest = common::data_dir()
        .join("parsers")
        .join(format!("{suffix}.lumis.json"));
    let bytes = fs::read(&manifest)
        .unwrap_or_else(|error| panic!("staged manifest {}: {error}", manifest.display()));
    let package: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

    fs::write(
        lock_path(project),
        format!(
            "version = 1\nrange = \"{range}\"\n\n\
             [[package]]\n\
             name = \"{name}\"\n\
             version = \"{version}\"\n\
             languages = [\"{language}\"]\n\
             manifest_sha256 = \"{manifest_sha256}\"\n\
             parser_sha256 = \"{parser_sha256}\"\n",
            range = lumis_wasm_runtime::catalog::LANGUAGE_PACKAGE_VERSION_RANGE,
            name = location.package_name,
            version = package["version"].as_str().unwrap(),
            manifest_sha256 = sha256_hex(&bytes),
            parser_sha256 = package["parser"]["sha256"].as_str().unwrap(),
        ),
    )
    .unwrap();
}

#[test]
fn install_is_satisfied_from_the_staged_store_without_a_network() {
    let project = project();
    write_lock_for(&project, "json");

    cmd(&project.nested)
        .args(["languages", "install"])
        .assert()
        .success();
}

/// The lock is discovered by walking up, so a command run inside a nested
/// package uses the repository's lock rather than looking only beside itself.
#[test]
fn install_finds_the_lock_from_a_nested_directory() {
    let project = project();
    write_lock_for(&project, "json");

    cmd(&project.nested)
        .args(["languages", "install"])
        .assert()
        .success();

    assert!(
        !project.nested.join("lumis-lock.toml").exists(),
        "install must not create a second lock beside itself"
    );
}

/// The enforcement the whole feature is for: the staged store holds a
/// range-compatible manifest, but it is not the version the lock names, so it
/// must not satisfy the lock.
///
/// Refusing the local file necessarily falls through to a fetch for the pinned
/// version, which is the one place here that touches the network. The assertion
/// is on the store having been bypassed rather than on the fetch's own failure,
/// so it reads the same whether the CDN answers 404 or is unreachable — and it
/// still discriminates, because accepting the stale manifest would *succeed*.
#[test]
fn install_refuses_a_version_the_store_has_but_the_lock_does_not_name() {
    let project = project();
    write_lock_for(&project, "json");
    let contents = fs::read_to_string(lock_path(&project)).unwrap();
    let staged_version = lumis_wasm_runtime::lowest_compatible_package_version();
    fs::write(
        lock_path(&project),
        contents.replace(
            &format!("version = \"{staged_version}\""),
            "version = \"0.26.999\"",
        ),
    )
    .unwrap();

    cmd(&project.nested)
        .args(["languages", "install"])
        .assert()
        .failure()
        // Naming the pinned version proves the stale local manifest was refused
        // and the pinned one was sought, rather than the run failing earlier.
        .stderr(predicate::str::contains("0.26.999"));
}

#[test]
fn install_without_a_lock_says_how_to_make_one() {
    let project = project();

    cmd(&project.nested)
        .args(["languages", "install"])
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("lumis-lock.toml")
                .and(predicate::str::contains("languages add")),
        );
}

/// `npm install` both adds and materializes, which is why `npm ci` had to be
/// invented. Keeping them apart is worth an explicit error.
#[test]
fn install_rejects_language_names_and_points_at_add() {
    let project = project();
    write_lock_for(&project, "json");

    cmd(&project.nested)
        .args(["languages", "install", "json"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("lumis languages add json"));
}

#[test]
fn update_refuses_a_language_that_is_not_locked() {
    let project = project();
    write_lock_for(&project, "json");

    cmd(&project.nested)
        .args(["languages", "update", "haskell"])
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("haskell is not in lumis-lock.toml")
                .and(predicate::str::contains("lumis languages add haskell")),
        );
}

#[test]
fn update_requires_a_name_or_all() {
    let project = project();
    write_lock_for(&project, "json");

    cmd(&project.nested)
        .args(["languages", "update"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--all"));
}

#[test]
fn remove_drops_the_language_and_its_package() {
    let project = project();
    write_lock_for(&project, "json");

    cmd(&project.nested)
        .args(["languages", "remove", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("removed json"));

    let contents = fs::read_to_string(lock_path(&project)).unwrap();
    assert!(
        !contents.contains("wasm-json"),
        "the package goes once nothing needs it:\n{contents}"
    );
    assert!(
        contents.contains("range ="),
        "the lock itself stays:\n{contents}"
    );
}

#[test]
fn remove_reports_a_language_that_was_never_locked() {
    let project = project();
    write_lock_for(&project, "json");

    cmd(&project.nested)
        .args(["languages", "remove", "haskell"])
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "haskell is not in lumis-lock.toml",
        ));
}

/// Upgrading across Tree-sitter series invalidates every entry at once, and the
/// message has to name the command rather than leave the reader to find it.
#[test]
fn a_lock_from_another_tree_sitter_series_names_the_fix() {
    let project = project();
    fs::write(lock_path(&project), "version = 1\nrange = \"0.1\"\n").unwrap();

    cmd(&project.nested)
        .args(["languages", "install"])
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("resolved against tree-sitter 0.1")
                .and(predicate::str::contains("update --all")),
        );
}

/// Parsing a newer format rather than rejecting it is deliberate; saying so is
/// what keeps the entries this build ignored from being invisible.
#[test]
fn a_newer_lock_format_warns_instead_of_failing() {
    let project = project();
    write_lock_for(&project, "json");
    let contents = fs::read_to_string(lock_path(&project)).unwrap();
    fs::write(
        lock_path(&project),
        contents.replace("version = 1\n", "version = 99\nfuture_key = \"ignored\"\n"),
    )
    .unwrap();

    let output = cmd(&project.nested)
        .args(["languages", "install"])
        .assert()
        .success()
        .get_output()
        .clone();

    let stderr = String::from_utf8(output.stderr).unwrap();
    assert_eq!(
        stderr.matches("written by a newer Lumis").count(),
        1,
        "the lock is loaded once, so the warning is said once:\n{stderr}"
    );
}

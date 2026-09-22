//! `lumis languages add|remove|update|install`: the only writer of the lock.
//!
//! Concentrating writes here is what lets every other runtime read the file
//! without reimplementing resolution, so a Mix task and an npx wrapper stay
//! wrappers over one binary rather than second implementations.

use anyhow::{bail, Context, Result};
use lumis_wasm_runtime::catalog;
use lumis_wasm_runtime::lock::{self, Lock, LockedPackage, LOCK_FILE_NAME};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::registry::Registry;

/// The lock for the current directory, and where it lives.
pub(crate) struct Located {
    pub(crate) path: PathBuf,
    pub(crate) lock: Lock,
    /// Whether the file existed; `add` is the only command that may create one.
    pub(crate) existed: bool,
}

/// Find the lock governing `cwd` without creating anything.
pub(crate) fn load(cwd: &Path) -> Result<Option<Located>> {
    let Some(path) = lock::find(cwd) else {
        return Ok(None);
    };
    let lock = Lock::read(&path)?.unwrap_or_else(|| Lock::new(range()));
    warn_if_newer(&lock, &path);
    lock.require_range(range(), &path)?;
    Ok(Some(Located {
        path,
        lock,
        existed: true,
    }))
}

/// Find the lock governing `cwd`, or where `add` would create one.
fn load_or_create(cwd: &Path) -> Result<Located> {
    if let Some(found) = load(cwd)? {
        return Ok(found);
    }
    let path = lock::default_path(cwd).with_context(|| {
        format!(
            "no {LOCK_FILE_NAME} found above {} and no repository root to create one in\n  \
             run this inside a git repository, or create {LOCK_FILE_NAME} yourself",
            cwd.display()
        )
    })?;
    Ok(Located {
        path,
        lock: Lock::new(range()),
        existed: false,
    })
}

fn range() -> &'static str {
    catalog::LANGUAGE_PACKAGE_VERSION_RANGE
}

/// A newer format parses here by design; saying so is better than silence,
/// because the entries this build ignored are invisible otherwise.
fn warn_if_newer(lock: &Lock, path: &Path) {
    if lock.is_newer_format() {
        eprintln!(
            "warning: {} was written by a newer Lumis (format {}, this build knows {})",
            path.display(),
            lock.version,
            lumis_wasm_runtime::LOCK_FORMAT_VERSION
        );
    }
}

/// Language names, with bundles expanded and plaintext dropped.
///
/// A bundle is shorthand at this point and nothing records its name: re-running
/// `add bundle-web` is what picks up a member the catalog gained, so widening
/// the lock stays a visible diff rather than something `update` does silently.
fn expand(languages: &[String]) -> Result<Vec<String>> {
    if languages.is_empty() {
        bail!("specify language names, or a bundle such as bundle-web");
    }
    let expanded = catalog::expand_bundles(languages.iter().map(String::as_str))?;
    let mut seen = std::collections::HashSet::new();
    Ok(expanded
        .into_iter()
        .map(|name| crate::resolve_language_id(&name).to_string())
        .filter(|id| id != "plaintext" && seen.insert(id.clone()))
        .collect())
}

/// Group languages by the package that provides them.
///
/// `ejs` and `erb` are both `@lumis-sh/wasm-embedded-template`, so adding both
/// has to resolve once and write one row.
fn by_package(languages: &[String]) -> Result<BTreeMap<&'static str, Vec<String>>> {
    let mut grouped: BTreeMap<&'static str, Vec<String>> = BTreeMap::new();
    for language in languages {
        let location =
            catalog::find(language).with_context(|| format!("unknown language '{language}'"))?;
        grouped
            .entry(location.package_name)
            .or_default()
            .push(language.clone());
    }
    Ok(grouped)
}

pub(crate) fn add(reg: &Registry, cwd: &Path, languages: &[String]) -> Result<()> {
    let names = expand(languages)?;
    let mut located = load_or_create(cwd)?;

    for (package_name, languages) in by_package(&names)? {
        let (package, manifest_sha256) = reg.resolve_for_lock(package_name)?;
        located.lock.insert(LockedPackage {
            name: package.package_name.clone(),
            version: package.version.clone(),
            languages,
            manifest_sha256,
            parser_sha256: package.parser.sha256.clone(),
            definition_hash: Some(package.definition_hash.clone()),
        });
        println!("added {} {}", package.package_name, package.version);
    }

    if !located.existed {
        println!("created {}", located.path.display());
    }
    located.lock.write(&located.path)?;

    // `add` fetches as well as records, so the parser is on disk before anything
    // asks for it. Materializing here also proves the entry just written can be
    // satisfied, rather than leaving that to the next run.
    materialize(reg, &names)
}

pub(crate) fn remove(cwd: &Path, languages: &[String]) -> Result<()> {
    let names = expand(languages)?;
    let mut located =
        load(cwd)?.with_context(|| format!("no {LOCK_FILE_NAME} found above {}", cwd.display()))?;

    let mut removed = Vec::new();
    for name in &names {
        if located.lock.remove_language(name) {
            removed.push(name.clone());
        } else {
            eprintln!("warning: {name} is not in {LOCK_FILE_NAME}");
        }
    }

    if removed.is_empty() {
        return Ok(());
    }
    located.lock.write(&located.path)?;
    for name in removed {
        println!("removed {name}");
    }
    Ok(())
}

/// The rows `update` will re-resolve: every one under `--all`, otherwise the
/// rows pinning the named languages.
fn update_targets(
    located: &Located,
    languages: &[String],
    all: bool,
) -> Result<Vec<LockedPackage>> {
    if all {
        return Ok(located.lock.packages.clone());
    }
    expand(languages)?
        .iter()
        .map(|name| {
            located.lock.language(name).cloned().with_context(|| {
                format!("{name} is not in {LOCK_FILE_NAME}\n  lumis languages add {name}")
            })
        })
        .collect()
}

/// Resolve every target before anything is written.
///
/// A partial update leaves a lock nobody chose, so a registry that is down for
/// one package changes nothing rather than moving the rest. Every failure is
/// reported, not just the first.
type Resolved = (LockedPackage, lumis_wasm_runtime::LanguagePackage, String);

fn resolve_all(reg: &Registry, targets: Vec<LockedPackage>) -> Result<Vec<Resolved>> {
    let mut resolved = Vec::new();
    let mut failures = Vec::new();
    for target in targets {
        match reg.resolve_for_lock(&target.name) {
            Ok((package, manifest_sha256)) => resolved.push((target, package, manifest_sha256)),
            Err(error) => failures.push(format!("{}: {error}", target.name)),
        }
    }
    if !failures.is_empty() {
        for failure in &failures {
            eprintln!("{failure}");
        }
        bail!(
            "failed to resolve {} package(s); {LOCK_FILE_NAME} is unchanged",
            failures.len()
        );
    }
    Ok(resolved)
}

pub(crate) fn update(reg: &Registry, cwd: &Path, languages: &[String], all: bool) -> Result<()> {
    if all && !languages.is_empty() {
        bail!("name a language or pass --all, not both");
    }
    if !all && languages.is_empty() {
        bail!("name a language to update, or pass --all");
    }
    let mut located =
        load(cwd)?.with_context(|| format!("no {LOCK_FILE_NAME} found above {}", cwd.display()))?;

    let targets = update_targets(&located, languages, all)?;
    let resolved = resolve_all(reg, targets)?;

    let mut moved = Vec::new();
    for (previous, package, manifest_sha256) in resolved {
        if previous.version != package.version {
            moved.push(format!(
                "{} {} -> {}",
                package.package_name, previous.version, package.version
            ));
        }
        located.lock.insert(LockedPackage {
            name: package.package_name.clone(),
            version: package.version,
            languages: previous.languages,
            manifest_sha256,
            parser_sha256: package.parser.sha256,
            definition_hash: Some(package.definition_hash),
        });
    }

    located.lock.write(&located.path)?;
    if moved.is_empty() {
        println!("already up to date");
    } else {
        for line in moved {
            println!("{line}");
        }
    }
    Ok(())
}

/// Materialize `located`, which the caller has already loaded so the store can
/// be built from it. Loading here as well would repeat every warning.
pub(crate) fn install(reg: &Registry, located: &Located) -> Result<()> {
    if located.lock.is_empty() {
        println!("{} names no languages", located.path.display());
        return Ok(());
    }

    let names: Vec<String> = located
        .lock
        .packages
        .iter()
        .flat_map(|package| package.languages.clone())
        .collect();
    materialize(reg, &names)
}

/// Download, verify and compile `names`, reporting every failure rather than
/// stopping at the first.
fn materialize(reg: &Registry, names: &[String]) -> Result<()> {
    let mut errors = Vec::new();
    for (name, result) in names.iter().zip(reg.cache_parsers_detailed(names, false)) {
        if let Err(error) = result {
            eprintln!("{name}: {error}");
            errors.push(name);
        }
    }
    if !errors.is_empty() {
        bail!("failed to prepare {} language(s)", errors.len());
    }

    for (name, result) in names.iter().zip(reg.precompile_parsers_detailed(names)) {
        if let Err(error) = result {
            eprintln!("{name}: failed to compile: {error}");
            errors.push(name);
        }
    }
    if !errors.is_empty() {
        bail!("failed to compile {} language(s)", errors.len());
    }
    Ok(())
}

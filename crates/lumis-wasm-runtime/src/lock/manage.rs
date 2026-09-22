//! Editing a lock: what `mix lumis.add` and its counterparts call.
//!
//! These take a `&mut Lock` and leave file I/O, argument parsing and output to
//! the caller, so every runtime spells the command its own way over one
//! implementation of what the command *means*. Bundle expansion, grouping
//! languages by the package that provides them, and resolve-before-write are the
//! parts that would otherwise be reimplemented per binding and drift.
//!
//! The CLI has no lock of its own — it is a viewer and a store filler, so
//! `lumis languages download` is its whole store surface. A lock belongs to a
//! project, and the runtime that owns the project is what manages it.

use std::collections::{BTreeMap, HashSet};

use super::{Lock, LockedPackage};
use crate::catalog;
use crate::package::LanguagePackage;
use crate::store::LanguageStore;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ManageError {
    #[error("specify language names, or a bundle such as bundle-web")]
    NoLanguages,
    #[error(transparent)]
    UnknownBundle(#[from] crate::UnknownBundle),
    #[error(transparent)]
    Package(#[from] crate::package::LanguagePackageError),
    #[error("unknown language '{0}'")]
    UnknownLanguage(String),
    #[error(
        "{language} is not in {lock}\n  add it first",
        lock = super::LOCK_FILE_NAME
    )]
    NotLocked { language: String },
    #[error(
        "failed to resolve {} package(s); {lock} is unchanged:\n{}",
        failures.len(),
        failures.join("\n"),
        lock = super::LOCK_FILE_NAME
    )]
    Unresolved { failures: Vec<String> },
}

/// One package an operation touched, for the caller to report.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Change {
    pub package_name: String,
    /// The version before the change, when there was one.
    pub previous_version: Option<String>,
    pub version: String,
}

impl Change {
    /// Whether this moved a package that was already pinned.
    #[must_use]
    pub fn moved(&self) -> bool {
        self.previous_version
            .as_ref()
            .is_some_and(|previous| previous != &self.version)
    }
}

/// Language names with bundles expanded, plaintext dropped and repeats removed.
///
/// A bundle is shorthand here and nothing records its name: re-running
/// `add bundle-web` is what picks up a member the catalog gained, so widening a
/// project stays a visible diff rather than something `update` does silently.
///
/// # Errors
/// Fails when nothing was named, or a `bundle-*` name matches no bundle.
pub fn expand(languages: &[String]) -> Result<Vec<String>, ManageError> {
    if languages.is_empty() {
        return Err(ManageError::NoLanguages);
    }
    let expanded = catalog::expand_bundles(languages.iter().map(String::as_str))?;
    let mut seen = HashSet::new();
    Ok(expanded
        .into_iter()
        .filter(|name| name != "plaintext" && seen.insert(name.clone()))
        .collect())
}

/// Group languages by the package that provides them.
///
/// `ejs` and `erb` are both `@lumis-sh/wasm-embedded-template`, so adding both
/// resolves once and writes one row.
fn by_package(languages: &[String]) -> Result<BTreeMap<&'static str, Vec<String>>, ManageError> {
    let mut grouped: BTreeMap<&'static str, Vec<String>> = BTreeMap::new();
    for language in languages {
        let location = catalog::find(language)
            .ok_or_else(|| ManageError::UnknownLanguage(language.clone()))?;
        grouped
            .entry(location.package_name)
            .or_default()
            .push(location.id.to_string());
    }
    Ok(grouped)
}

/// The row to record, once the resolved package is known to provide every
/// language being pinned.
///
/// The catalog maps a language to a package name and ships with the *runtime*,
/// while the package is whatever the registry serves now. The two can disagree —
/// a package that drops or moves a language leaves the catalog pointing at it —
/// and without this check the lock would record membership the package does not
/// have, turning a clear failure here into a confusing one at materialization.
fn entry(
    package: &LanguagePackage,
    manifest_sha256: String,
    languages: Vec<String>,
) -> Result<LockedPackage, ManageError> {
    for language in &languages {
        package.require_language(language)?;
    }
    Ok(LockedPackage {
        name: package.package_name.clone(),
        version: package.version.clone(),
        languages,
        manifest_sha256,
        parser_sha256: package.parser.sha256.clone(),
        definition_hash: Some(package.definition_hash.clone()),
    })
}

/// Record `languages` in `lock`, resolving each package's current version.
///
/// Returns what changed; the caller writes the lock and reports.
///
/// # Errors
/// Fails when a name is unknown or a package cannot be resolved.
pub fn add(
    store: &LanguageStore,
    lock: &mut Lock,
    languages: &[String],
) -> Result<Vec<Change>, ManageError> {
    let names = expand(languages)?;
    let mut changes = Vec::new();
    for (package_name, languages) in by_package(&names)? {
        let previous_version = lock.package(package_name).map(|row| row.version.clone());
        let (package, manifest_sha256) =
            store
                .resolve_for_lock(package_name)
                .map_err(|error| ManageError::Unresolved {
                    failures: vec![format!("{package_name}: {error}")],
                })?;
        changes.push(Change {
            package_name: package.package_name.clone(),
            previous_version,
            version: package.version.clone(),
        });
        lock.insert(entry(&package, manifest_sha256, languages)?);
    }
    Ok(changes)
}

/// Stop pinning `languages`, dropping a package once nothing needs it.
///
/// Returns the languages that were actually pinned; anything else is reported by
/// the caller as a no-op rather than an error, because removing something that
/// is already absent is what the user asked for.
///
/// # Errors
/// Fails when nothing was named or a `bundle-*` name matches no bundle.
pub fn remove(lock: &mut Lock, languages: &[String]) -> Result<Vec<String>, ManageError> {
    let names = expand(languages)?;
    Ok(names
        .into_iter()
        .filter(|name| lock.remove_language(name))
        .collect())
}

/// Re-resolve pinned packages and move `lock` to what the registry now serves.
///
/// Everything is resolved before anything is written, so a registry that is down
/// for one package leaves the lock untouched rather than moving the rest, and
/// every failure is reported instead of just the first.
///
/// # Errors
/// Fails when a named language is not pinned, or any package cannot be resolved.
pub fn update(
    store: &LanguageStore,
    lock: &mut Lock,
    languages: &[String],
    all: bool,
) -> Result<Vec<Change>, ManageError> {
    let targets: Vec<LockedPackage> = if all {
        lock.packages.clone()
    } else {
        // `ejs` and `erb` share a package, so naming both would otherwise
        // resolve it twice and report it twice.
        let mut seen = HashSet::new();
        expand(languages)?
            .into_iter()
            .map(|name| {
                lock.language(&name)
                    .cloned()
                    .ok_or(ManageError::NotLocked { language: name })
            })
            .filter(|row| match row {
                Ok(row) => seen.insert(row.name.clone()),
                Err(_) => true,
            })
            .collect::<Result<_, _>>()?
    };

    let mut resolved = Vec::new();
    let mut failures = Vec::new();
    for target in targets {
        match store.resolve_for_lock(&target.name) {
            Ok((package, manifest_sha256)) => resolved.push((target, package, manifest_sha256)),
            Err(error) => failures.push(format!("{}: {error}", target.name)),
        }
    }
    if !failures.is_empty() {
        return Err(ManageError::Unresolved { failures });
    }

    let mut changes = Vec::new();
    for (previous, package, manifest_sha256) in resolved {
        changes.push(Change {
            package_name: package.package_name.clone(),
            previous_version: Some(previous.version),
            version: package.version.clone(),
        });
        lock.insert(entry(&package, manifest_sha256, previous.languages)?);
    }
    Ok(changes)
}

/// Every language `lock` pins, for a caller about to materialize it.
#[must_use]
pub fn locked_languages(lock: &Lock) -> Vec<String> {
    lock.packages
        .iter()
        .flat_map(|package| package.languages.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expanding_nothing_is_an_error_rather_than_a_silent_no_op() {
        assert!(matches!(expand(&[]), Err(ManageError::NoLanguages)));
    }

    #[test]
    fn a_bundle_expands_and_its_name_is_not_kept() {
        let expanded = expand(&["bundle-web".to_string()]).unwrap();
        assert!(expanded.contains(&"css".to_string()));
        assert!(!expanded.iter().any(|name| name.starts_with("bundle-")));
    }

    #[test]
    fn plaintext_is_dropped_because_it_has_no_package() {
        let expanded = expand(&["plaintext".to_string(), "rust".to_string()]).unwrap();
        assert_eq!(expanded, ["rust"]);
    }

    /// `ejs` and `erb` share `@lumis-sh/wasm-embedded-template`, so both have to
    /// land under one key or the caller resolves the same package twice and
    /// writes two rows that can disagree.
    #[test]
    fn languages_sharing_a_package_are_grouped() {
        let grouped = by_package(&["ejs".to_string(), "erb".to_string()]).unwrap();
        assert_eq!(grouped.len(), 1);
        assert_eq!(
            grouped["@lumis-sh/wasm-embedded-template"],
            ["ejs", "erb"],
            "one resolve, one row"
        );
    }

    #[test]
    fn an_alias_is_recorded_under_its_stable_id() {
        let grouped = by_package(&["js".to_string()]).unwrap();
        let languages = grouped.values().next().unwrap();
        assert_eq!(
            languages,
            &["javascript"],
            "a lock keyed on aliases could not say whether javascript was pinned"
        );
    }

    #[test]
    fn an_unknown_language_is_named() {
        let error = by_package(&["nope".to_string()]).unwrap_err();
        assert!(error.to_string().contains("nope"), "{error}");
    }

    #[test]
    fn removing_reports_only_what_was_pinned() {
        let mut lock = Lock::new("0.26");
        lock.insert(LockedPackage {
            name: "@lumis-sh/wasm-rust".into(),
            version: "0.26.1".into(),
            languages: vec!["rust".into()],
            manifest_sha256: "a".repeat(64),
            parser_sha256: "b".repeat(64),
            definition_hash: None,
        });

        let removed = remove(&mut lock, &["rust".to_string(), "haskell".to_string()]).unwrap();
        assert_eq!(removed, ["rust"]);
        assert!(lock.is_empty());
    }

    /// The catalog ships with the runtime and the package comes from the
    /// registry, so they can disagree. Recording membership a package does not
    /// have turns a clear failure here into a confusing one at materialization.
    #[test]
    fn a_package_that_does_not_provide_the_language_is_refused() {
        use crate::package::{LanguagePackage, PackagedLanguage, ParserMetadata};
        use std::collections::BTreeMap;

        let package = LanguagePackage {
            package_name: "@lumis-sh/wasm-rust".into(),
            version: "0.26.1".into(),
            definition_hash: "hash".into(),
            parser: ParserMetadata {
                name: "tree-sitter-rust".into(),
                grammar_name: "rust".into(),
                upstream_version: None,
                revision: None,
                sha256: "a".repeat(64),
                size: 1,
            },
            // The package no longer carries the language the catalog maps here.
            languages: BTreeMap::from([("ron".into(), PackagedLanguage::default())]),
        };

        let error = entry(&package, "b".repeat(64), vec!["rust".into()]).unwrap_err();
        assert!(error.to_string().contains("rust"), "{error}");
    }

    #[test]
    fn a_package_providing_the_language_is_recorded() {
        use crate::package::{LanguagePackage, PackagedLanguage, ParserMetadata};
        use std::collections::BTreeMap;

        let package = LanguagePackage {
            package_name: "@lumis-sh/wasm-rust".into(),
            version: "0.26.1".into(),
            definition_hash: "hash".into(),
            parser: ParserMetadata {
                name: "tree-sitter-rust".into(),
                grammar_name: "rust".into(),
                upstream_version: None,
                revision: None,
                sha256: "a".repeat(64),
                size: 1,
            },
            languages: BTreeMap::from([("rust".into(), PackagedLanguage::default())]),
        };

        let row = entry(&package, "b".repeat(64), vec!["rust".into()]).unwrap();
        assert_eq!(row.languages, ["rust"]);
        assert_eq!(row.version, "0.26.1");
    }

    #[test]
    fn a_change_reports_whether_it_moved() {
        let fresh = Change {
            package_name: "@lumis-sh/wasm-rust".into(),
            previous_version: None,
            version: "0.26.1".into(),
        };
        assert!(!fresh.moved(), "a first pin has not moved");

        let same = Change {
            previous_version: Some("0.26.1".into()),
            ..fresh.clone()
        };
        assert!(!same.moved());

        let moved = Change {
            previous_version: Some("0.26.0".into()),
            ..fresh
        };
        assert!(moved.moved());
    }
}

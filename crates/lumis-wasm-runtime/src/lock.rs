//! The project lock: which language packages a project pins, and their digests.
//!
//! Only the CLI writes this file. Every other runtime reads it, so the parser
//! a document resolves is the one a human reviewed rather than whichever
//! version the machine's store happened to download first.
//!
//! The format is **additive-only**, like [`crate::package::LanguagePackage`].
//! Unknown fields and unknown tables are ignored rather than rejected, so a lock
//! written by a newer Lumis still parses here. [`Lock::is_newer_format`] reports
//! the case a caller may want to warn about. Erroring instead is what forces the
//! whole ecosystem to upgrade in lockstep, which is the failure pnpm and cargo
//! both ship.

use std::collections::BTreeSet;
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// The file a project checks in.
pub const LOCK_FILE_NAME: &str = "lumis-lock.toml";

/// The format this Lumis writes. A lock declaring more than this still parses.
pub const LOCK_FORMAT_VERSION: u32 = 1;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum LockError {
    #[error("{context}: {source}")]
    Io {
        context: String,
        #[source]
        source: std::io::Error,
    },
    #[error("{path} is not valid TOML: {source}", path = path.display())]
    Parse {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },
    #[error("could not serialize the lock: {0}")]
    Serialize(#[from] toml::ser::Error),
    #[error(
        "{path} was resolved against tree-sitter {locked}, this build requires {required}\n  \
         every entry needs re-resolving: lumis languages update --all",
        path = path.display()
    )]
    RangeMoved {
        path: PathBuf,
        locked: String,
        required: &'static str,
    },
}

impl LockError {
    fn io(context: impl Into<String>, source: std::io::Error) -> Self {
        Self::Io {
            context: context.into(),
            source,
        }
    }
}

/// One pinned language package.
///
/// Keyed on the package rather than the language because the package is the unit
/// that is published and versioned, and one package can back several languages:
/// `ejs` and `erb` are both `@lumis-sh/wasm-embedded-template`. Keying on the
/// language would duplicate `version` and both digests across rows that can then
/// disagree.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LockedPackage {
    pub name: String,
    pub version: String,
    /// The languages that were **added**, not everything the package provides.
    ///
    /// That is what lets a lock say "ejs but not erb", and it means the field
    /// changes when someone runs `add` or `remove` rather than when a package
    /// gains a language upstream.
    #[serde(default)]
    pub languages: Vec<String>,
    /// SHA-256 of the `lumis.json` bytes.
    ///
    /// The anchor the format was missing. `parser_sha256` is only trustworthy
    /// because the manifest declared it, and the manifest arrives over the
    /// network; without this, a wrong response on first fetch is trusted
    /// permanently.
    pub manifest_sha256: String,
    pub parser_sha256: String,
    /// Change detection, not integrity: a reader can compare it but cannot
    /// recompute it, since it is derived from `languages.toml` and the queries
    /// at publish time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition_hash: Option<String>,
}

impl LockedPackage {
    /// Whether this row pins `language`, matching the stable package language id.
    pub fn provides(&self, language: &str) -> bool {
        self.languages
            .iter()
            .any(|name| name.eq_ignore_ascii_case(language))
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct Lock {
    pub version: u32,
    /// The Tree-sitter series this was resolved against, so a runtime whose ABI
    /// range has moved can say so instead of silently ignoring every entry.
    pub range: String,
    #[serde(default, rename = "package")]
    pub packages: Vec<LockedPackage>,
}

impl Lock {
    /// An empty lock for `range`, as `add` creates before its first entry.
    pub fn new(range: impl Into<String>) -> Self {
        Self {
            version: LOCK_FORMAT_VERSION,
            range: range.into(),
            packages: Vec::new(),
        }
    }

    /// Whether this lock was written by a Lumis that knows a later format.
    ///
    /// Parsing succeeds regardless; this exists so a caller can warn.
    pub fn is_newer_format(&self) -> bool {
        self.version > LOCK_FORMAT_VERSION
    }

    /// The row pinning `package_name`, if any.
    pub fn package(&self, package_name: &str) -> Option<&LockedPackage> {
        self.packages
            .iter()
            .find(|locked| locked.name == package_name)
    }

    /// The row that pins `language`, if any.
    pub fn language(&self, language: &str) -> Option<&LockedPackage> {
        self.packages
            .iter()
            .find(|locked| locked.provides(language))
    }

    /// Whether anything is pinned. A lock with no rows still governs: it means
    /// nothing is allowed, not that everything is.
    pub fn is_empty(&self) -> bool {
        self.packages.is_empty()
    }

    /// Fail when `required` is not the series this lock was resolved against.
    ///
    /// Upgrading Lumis across Tree-sitter series invalidates every entry at
    /// once, and the message has to name the command rather than leave the
    /// reader to find it.
    pub fn require_range(&self, required: &'static str, path: &Path) -> Result<(), LockError> {
        if self.range == required {
            return Ok(());
        }
        Err(LockError::RangeMoved {
            path: path.to_path_buf(),
            locked: self.range.clone(),
            required,
        })
    }

    /// Record `package`, replacing any existing row for the same name and
    /// merging `languages` into it.
    ///
    /// Merging is what makes `add ejs` then `add erb` one row rather than two,
    /// and what keeps `remove erb` from dropping a package `ejs` still needs.
    pub fn insert(&mut self, package: LockedPackage) {
        match self
            .packages
            .iter_mut()
            .find(|existing| existing.name == package.name)
        {
            Some(existing) => {
                let mut languages: BTreeSet<String> = existing.languages.drain(..).collect();
                languages.extend(package.languages);
                existing.languages = languages.into_iter().collect();
                existing.version = package.version;
                existing.manifest_sha256 = package.manifest_sha256;
                existing.parser_sha256 = package.parser_sha256;
                existing.definition_hash = package.definition_hash;
            }
            None => self.packages.push(package),
        }
        self.sort();
    }

    /// Stop pinning `language`, dropping its package once nothing needs it.
    ///
    /// Returns whether anything changed.
    pub fn remove_language(&mut self, language: &str) -> bool {
        let Some(row) = self
            .packages
            .iter_mut()
            .find(|locked| locked.provides(language))
        else {
            return false;
        };
        row.languages
            .retain(|name| !name.eq_ignore_ascii_case(language));
        self.packages.retain(|locked| !locked.languages.is_empty());
        true
    }

    /// Sorted by package name, then by language, so a diff stays small and two
    /// people adding different languages do not reorder each other's rows.
    fn sort(&mut self) {
        for package in &mut self.packages {
            package.languages.sort();
            package.languages.dedup();
        }
        self.packages.sort_by(|a, b| a.name.cmp(&b.name));
    }

    /// Parse `contents`, ignoring anything this format version does not know.
    pub fn parse(contents: &str, path: &Path) -> Result<Self, LockError> {
        let mut lock: Self = toml::from_str(contents).map_err(|source| LockError::Parse {
            path: path.to_path_buf(),
            source,
        })?;
        lock.sort();
        Ok(lock)
    }

    /// Read the lock at `path`, or `None` when there is no file there.
    pub fn read(path: &Path) -> Result<Option<Self>, LockError> {
        let contents = match std::fs::read_to_string(path) {
            Ok(contents) => contents,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(error) => {
                return Err(LockError::io(
                    format!("could not read {}", path.display()),
                    error,
                ))
            }
        };
        Self::parse(&contents, path).map(Some)
    }

    pub fn to_toml(&self) -> Result<String, LockError> {
        let mut sorted = self.clone();
        sorted.sort();
        Ok(toml::to_string_pretty(&sorted)?)
    }

    /// Write the lock to `path` under an advisory lock.
    ///
    /// The parser store deliberately has no file lock: writes rename a uniquely
    /// named temporary into place and content is verified first, so concurrent
    /// writers converge. A lock file cannot use that trick — it is one mutable
    /// document with many keys, so two `add` runs would each write the entry
    /// they knew about and the later rename would win outright.
    pub fn write(&self, path: &Path) -> Result<(), LockError> {
        let contents = self.to_toml()?;
        let guard = LockGuard::acquire(path)?;
        crate::store::write_atomic(path, contents.as_bytes()).map_err(|error| {
            LockError::io(
                format!("could not write {}", path.display()),
                std::io::Error::other(error),
            )
        })?;
        drop(guard);
        Ok(())
    }
}

/// Where the lock for `start` lives, searching upward.
///
/// An existing lock wins wherever it sits, so a nested package in a monorepo
/// extends the repository's lock rather than starting a second one.
pub fn find(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .map(|dir| dir.join(LOCK_FILE_NAME))
        .find(|candidate| candidate.is_file())
}

/// Where `add` creates a lock when [`find`] came up empty: the repository root.
///
/// Deliberately not the working directory, and deliberately not derived from
/// `mix.exs`, `package.json` or `Cargo.toml` — a monorepo has many of those and
/// nothing says which one is right, which is the same reason there is one lock
/// rather than one per project.
pub fn default_path(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|dir| dir.join(".git").exists())
        .map(|root| root.join(LOCK_FILE_NAME))
}

/// The lock to use for `start`: an existing one, else where one would go.
pub fn resolve_path(start: &Path) -> Option<PathBuf> {
    find(start).or_else(|| default_path(start))
}

/// Held for the duration of a lock-file write.
struct LockGuard {
    file: std::fs::File,
    path: PathBuf,
}

impl LockGuard {
    fn acquire(target: &Path) -> Result<Self, LockError> {
        let path = target.with_extension("toml.lock");
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|error| {
                LockError::io(format!("could not create {}", parent.display()), error)
            })?;
        }
        let file = std::fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(&path)
            .map_err(|error| LockError::io(format!("could not open {}", path.display()), error))?;
        file.lock()
            .map_err(|error| LockError::io(format!("could not lock {}", path.display()), error))?;
        Ok(Self { file, path })
    }
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        let _ = self.file.flush();
        let _ = self.file.unlock();
        let _ = std::fs::remove_file(&self.path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn package(name: &str, version: &str, languages: &[&str]) -> LockedPackage {
        LockedPackage {
            name: name.to_string(),
            version: version.to_string(),
            languages: languages.iter().map(|s| (*s).to_string()).collect(),
            manifest_sha256: "a".repeat(64),
            parser_sha256: "b".repeat(64),
            definition_hash: Some("hash".into()),
        }
    }

    #[test]
    fn a_lock_round_trips() {
        let mut lock = Lock::new("0.26");
        lock.insert(package("@lumis-sh/wasm-rust", "0.26.4", &["rust"]));
        let toml = lock.to_toml().unwrap();
        assert_eq!(
            Lock::parse(&toml, Path::new("lumis-lock.toml")).unwrap(),
            lock
        );
    }

    /// The whole point of the additive-only contract: a lock from a newer Lumis
    /// parses, and the caller decides whether to say anything about it.
    #[test]
    fn an_unknown_field_is_ignored_rather_than_rejected() {
        let toml = r#"
version = 2
range = "0.26"
future_top_level = "whatever"

[[package]]
name = "@lumis-sh/wasm-rust"
version = "0.26.4"
languages = ["rust"]
manifest_sha256 = "aa"
parser_sha256 = "bb"
future_row_field = 7
"#;
        let lock = Lock::parse(toml, Path::new("lumis-lock.toml")).unwrap();
        assert!(lock.is_newer_format());
        assert_eq!(lock.packages.len(), 1);
        assert_eq!(lock.packages[0].version, "0.26.4");
    }

    #[test]
    fn rows_are_sorted_by_package_name() {
        let mut lock = Lock::new("0.26");
        lock.insert(package("@lumis-sh/wasm-rust", "0.26.4", &["rust"]));
        lock.insert(package("@lumis-sh/wasm-elixir", "0.26.2", &["elixir"]));
        let names: Vec<&str> = lock.packages.iter().map(|p| p.name.as_str()).collect();
        assert_eq!(
            names,
            ["@lumis-sh/wasm-elixir", "@lumis-sh/wasm-rust"],
            "an unsorted lock makes every add a merge conflict"
        );
    }

    /// `ejs` and `erb` share `@lumis-sh/wasm-embedded-template`, so adding both
    /// has to produce one row rather than two that can disagree about a digest.
    #[test]
    fn two_languages_from_one_package_share_a_row() {
        let mut lock = Lock::new("0.26");
        lock.insert(package(
            "@lumis-sh/wasm-embedded-template",
            "0.26.1",
            &["ejs"],
        ));
        lock.insert(package(
            "@lumis-sh/wasm-embedded-template",
            "0.26.1",
            &["erb"],
        ));

        assert_eq!(lock.packages.len(), 1);
        assert_eq!(lock.packages[0].languages, ["ejs", "erb"]);
    }

    #[test]
    fn removing_one_language_keeps_a_package_the_other_still_needs() {
        let mut lock = Lock::new("0.26");
        lock.insert(package(
            "@lumis-sh/wasm-embedded-template",
            "0.26.1",
            &["ejs", "erb"],
        ));

        assert!(lock.remove_language("erb"));
        assert_eq!(lock.packages.len(), 1);
        assert_eq!(lock.packages[0].languages, ["ejs"]);

        assert!(lock.remove_language("ejs"));
        assert!(lock.is_empty(), "the last language drops the package");
    }

    #[test]
    fn removing_something_unlocked_changes_nothing() {
        let mut lock = Lock::new("0.26");
        lock.insert(package("@lumis-sh/wasm-rust", "0.26.4", &["rust"]));
        assert!(!lock.remove_language("haskell"));
        assert_eq!(lock.packages.len(), 1);
    }

    #[test]
    fn a_lock_pins_a_language_not_merely_its_package() {
        let mut lock = Lock::new("0.26");
        lock.insert(package(
            "@lumis-sh/wasm-embedded-template",
            "0.26.1",
            &["ejs"],
        ));

        assert!(lock.language("ejs").is_some());
        assert!(
            lock.language("erb").is_none(),
            "erb shares the package but was never added"
        );
    }

    #[test]
    fn a_moved_range_names_the_command_that_fixes_it() {
        let lock = Lock::new("0.26");
        let error = lock
            .require_range("0.27", Path::new("/app/lumis-lock.toml"))
            .unwrap_err();
        let message = error.to_string();
        assert!(message.contains("0.26"), "{message}");
        assert!(message.contains("0.27"), "{message}");
        assert!(message.contains("update --all"), "{message}");
    }

    #[test]
    fn reading_a_missing_lock_is_not_an_error() {
        let dir = tempfile::tempdir().unwrap();
        assert!(Lock::read(&dir.path().join(LOCK_FILE_NAME))
            .unwrap()
            .is_none());
    }

    #[test]
    fn a_written_lock_reads_back() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(LOCK_FILE_NAME);
        let mut lock = Lock::new("0.26");
        lock.insert(package("@lumis-sh/wasm-rust", "0.26.4", &["rust"]));

        lock.write(&path).unwrap();
        assert_eq!(Lock::read(&path).unwrap().unwrap(), lock);
        assert!(
            !path.with_extension("toml.lock").exists(),
            "the advisory lock file is cleaned up"
        );
    }

    #[test]
    fn an_existing_lock_wins_over_the_repository_root() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::create_dir_all(root.join(".git")).unwrap();
        let nested = root.join("apps").join("web");
        std::fs::create_dir_all(&nested).unwrap();
        std::fs::write(
            nested.join(LOCK_FILE_NAME),
            "version = 1\nrange = \"0.26\"\n",
        )
        .unwrap();

        assert_eq!(resolve_path(&nested).unwrap(), nested.join(LOCK_FILE_NAME));
    }

    #[test]
    fn a_new_lock_goes_to_the_repository_root_not_the_working_directory() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::create_dir_all(root.join(".git")).unwrap();
        let nested = root.join("apps").join("web");
        std::fs::create_dir_all(&nested).unwrap();

        assert_eq!(resolve_path(&nested).unwrap(), root.join(LOCK_FILE_NAME));
    }

    #[test]
    fn an_empty_lock_allows_nothing_rather_than_everything() {
        let lock = Lock::new("0.26");
        assert!(lock.is_empty());
        assert!(lock.language("rust").is_none());
    }
}

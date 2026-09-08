//! Tree-sitter highlighting shared by Lumis runtimes.

use std::sync::Mutex;

macro_rules! define_catalog {
    (
        package_version_range: $package_version_range:literal,
        languages: {
            $(
                $id:literal => {
                    aliases: [$($alias:literal),* $(,)?],
                    package_name: $package_name:literal
                }
            ),* $(,)?
        },
        bundles: {
            $($bundle:literal => [$($member:literal),* $(,)?]),* $(,)?
        } $(,)?
    ) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct LanguagePackageRef {
            pub id: &'static str,
            pub aliases: &'static [&'static str],
            pub package_name: &'static str,
        }

        /// npm range accepted by this runtime's Tree-sitter ABI.
        pub const LANGUAGE_PACKAGE_VERSION_RANGE: &str = $package_version_range;

        pub static LANGUAGES: &[LanguagePackageRef] = &[
            $(
                LanguagePackageRef {
                    id: $id,
                    aliases: &[$($alias),*],
                    package_name: $package_name,
                },
            )*
        ];

        /// The language sets the `@lumis-sh/wasm-bundle-*` packages publish, so
        /// every runtime can name the same group instead of listing members.
        pub static BUNDLES: &[(&str, &[&str])] = &[
            $(($bundle, &[$($member),*]),)*
        ];

        pub fn find(name: &str) -> Option<&'static LanguagePackageRef> {
            LANGUAGES.iter().find(|entry| {
                entry.id.eq_ignore_ascii_case(name)
                    || entry
                        .aliases
                        .iter()
                        .any(|alias| alias.eq_ignore_ascii_case(name))
            })
        }

        /// `-` and `_` are interchangeable and case is ignored, so Elixir's
        /// `:bundle_web_extra` and the CLI's `bundle-web-extra` are one name.
        fn normalize_bundle(name: &str) -> String {
            name.to_ascii_lowercase().replace('_', "-")
        }

        /// The `bundle-` prefix, however the caller spelled it.
        fn strip_bundle_prefix(name: &str) -> Option<&str> {
            let (prefix, suffix) = name.split_at_checked("bundle-".len())?;
            matches!(normalize_bundle(prefix).as_str(), "bundle-").then_some(suffix)
        }

        /// Members of `bundle-<name>`, or `None` when the name is not a bundle.
        ///
        /// ```
        /// use lumis_wasm_runtime::catalog;
        ///
        /// assert!(catalog::bundle_members("bundle-web").is_some());
        /// assert!(catalog::bundle_members("bundle_web").is_some());
        /// assert!(catalog::bundle_members("rust").is_none());
        /// ```
        pub fn bundle_members(name: &str) -> Option<&'static [&'static str]> {
            let wanted = normalize_bundle(strip_bundle_prefix(name)?);

            BUNDLES
                .iter()
                .find_map(|(bundle, members)| (normalize_bundle(bundle) == wanted).then_some(*members))
        }

        /// Expand every `bundle-<name>` token into its members, leaving other
        /// names alone and dropping repeats.
        ///
        /// ```
        /// use lumis_wasm_runtime::catalog;
        ///
        /// let expanded = catalog::expand_bundles(["bundle-web", "css"]).unwrap();
        /// assert!(expanded.contains(&"css".to_string()));
        /// assert_eq!(expanded.iter().filter(|name| *name == "css").count(), 1);
        ///
        /// assert!(catalog::expand_bundles(["bundle-nope"]).is_err());
        /// ```
        pub fn expand_bundles<'a>(
            names: impl IntoIterator<Item = &'a str>,
        ) -> Result<Vec<String>, crate::UnknownBundle> {
            let mut expanded = Vec::new();
            let mut seen = std::collections::HashSet::new();

            for name in names {
                match bundle_members(name) {
                    Some(members) => expanded.extend(members.iter().map(|m| (*m).to_string())),
                    None if strip_bundle_prefix(name).is_some() => {
                        return Err(crate::UnknownBundle(name.to_string()))
                    }
                    None => expanded.push(name.to_string()),
                }
            }

            // `Vec::dedup` only collapses adjacent equals, so a bundle followed
            // by one of its own members would cache that member twice.
            expanded.retain(|name| seen.insert(name.clone()));
            Ok(expanded)
        }
    };
}

/// A name spelled like `bundle-<name>` that names no bundle.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnknownBundle(String);

impl UnknownBundle {
    /// The name as the caller spelled it.
    pub fn name(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for UnknownBundle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown bundle '{}'", self.0)
    }
}

impl std::error::Error for UnknownBundle {}

/// Run `work` over every item on up to `concurrency` threads, in input order.
///
/// Preparing a bundle is a hundred-odd independent downloads and compiles, and
/// doing them one at a time is what makes `bundle-full` take minutes. Threads
/// pull from a shared cursor rather than taking a fixed slice each, so one
/// 22 MB parser cannot leave a worker holding the whole run.
///
/// Results come back positionally, so a caller can zip them against the names it
/// passed without threading identity through the closure.
pub(crate) fn parallel_map<Item, Output, Work>(
    items: &[Item],
    concurrency: usize,
    work: Work,
) -> Vec<Output>
where
    Item: Sync,
    Output: Send,
    Work: Fn(&Item) -> Output + Sync,
{
    use std::sync::atomic::{AtomicUsize, Ordering};

    let threads = concurrency.clamp(1, items.len().max(1));
    if threads == 1 {
        return items.iter().map(work).collect();
    }

    // `Option` because a slot is filled exactly once, by whichever thread claims
    // that index, and there is no meaningful value to seed it with. The lock is
    // taken to store a finished result, never across `work`.
    let results: Mutex<Vec<Option<Output>>> = Mutex::new((0..items.len()).map(|_| None).collect());
    let cursor = AtomicUsize::new(0);

    let drain = || loop {
        let index = cursor.fetch_add(1, Ordering::Relaxed);
        let Some(item) = items.get(index) else { return };
        let output = work(item);
        results.lock().expect("parallel_map lock poisoned")[index] = Some(output);
    };

    std::thread::scope(|scope| {
        // Downloading resolves TLS, which wants far more stack than a BEAM dirty
        // scheduler carries, and the NIF reaches this directly rather than
        // through its own 8 MiB executor threads.
        for index in 1..threads {
            let _ = std::thread::Builder::new()
                .name(format!("lumis-batch-{index}"))
                .stack_size(8 * 1024 * 1024)
                .spawn_scoped(scope, drain);
        }
        // The calling thread is a worker too, so a refused spawn costs
        // parallelism rather than leaving an item unclaimed.
        drain();
    });

    results
        .into_inner()
        .expect("parallel_map lock poisoned")
        .into_iter()
        .map(|slot| slot.expect("every index is claimed exactly once"))
        .collect()
}

/// Threads to spread bundle downloads across.
///
/// Downloads are latency-bound rather than CPU-bound — a CI container with two
/// cores still waits on the CDN — so this deliberately does not follow
/// [`std::thread::available_parallelism`]. Held well under what a public CDN
/// would consider abusive.
pub const DOWNLOAD_CONCURRENCY: usize = 8;

/// Threads to spread parser compilation across.
///
/// Cranelift saturates a core per module but also uses enough memory that a
/// full catalog can exhaust a memory-constrained builder. Four keeps the work
/// parallel without letting a high core count multiply peak memory unchecked.
#[must_use]
pub fn compile_concurrency() -> usize {
    std::thread::available_parallelism()
        .map_or(1, usize::from)
        .min(4)
}

#[cfg(feature = "wasm")]
pub mod brackets;
pub mod catalog;
pub mod package;
pub mod store;
pub mod tree_sitter_highlight;

#[cfg(feature = "wasm")]
pub mod executor;
#[cfg(feature = "wasm")]
mod runtime;

#[cfg(feature = "wasm")]
pub use brackets::{
    bracket_pairs, capture_indices, colorize_bracket_pairs, BracketPair, RainbowRange,
    RAINBOW_BRACKET_SCOPES, RAINBOW_SCOPE_INDICES,
};
#[cfg(feature = "wasm")]
pub use executor::{runtime_with_catalog, Executor, ExecutorError, LoadFailure};
pub use package::{
    grammar_name, sha256_hex, LanguagePackage, LanguagePackageError, PackagedLanguage,
    ParserMetadata,
};
#[cfg(feature = "wasm")]
pub use runtime::{
    set_compile_cache_dir, HighlightOptions, HighlightOutput, InjectionResolution, LanguageSpec,
    Runtime, RuntimeError,
};
#[cfg(feature = "wasm")]
pub use store::HttpFetcher;
pub use store::{
    lowest_compatible_package_version, package_suffix, parser_filename, write_atomic,
    CacheLanguageOutcome, Fetcher, LanguageStore, NoNetwork, StoreConfig, StoreError,
};

#[cfg(test)]
mod parallel_map_tests {
    #[test]
    fn returns_results_in_input_order() {
        let items: Vec<usize> = (0..64).collect();
        // Reverse the durations so completion order cannot match input order.
        let squares = super::parallel_map(&items, 8, |item| {
            std::thread::sleep(std::time::Duration::from_micros((64 - *item as u64) * 50));
            item * item
        });
        assert_eq!(
            squares,
            items.iter().map(|item| item * item).collect::<Vec<_>>()
        );
    }

    #[test]
    fn runs_every_item_exactly_once() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        let items: Vec<usize> = (0..100).collect();
        let calls = AtomicUsize::new(0);
        let seen = super::parallel_map(&items, 16, |item| {
            calls.fetch_add(1, Ordering::Relaxed);
            *item
        });
        assert_eq!(calls.into_inner(), 100);
        assert_eq!(seen, items);
    }

    #[test]
    fn handles_empty_input_and_excess_concurrency() {
        let empty: Vec<usize> = Vec::new();
        assert!(
            super::parallel_map(&empty, 8, |item| *item).is_empty(),
            "no items in, so nothing should come out"
        );
        assert_eq!(super::parallel_map(&[7usize], 64, |item| *item), vec![7]);
    }

    #[test]
    fn bounds_compile_concurrency() {
        assert!((1..=4).contains(&super::compile_concurrency()));
    }
}

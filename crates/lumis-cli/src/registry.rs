#[cfg(test)]
use anyhow::Context;
use anyhow::Result;
use lumis_wasm_runtime::catalog;
#[cfg(test)]
use lumis_wasm_runtime::LanguagePackage;
use lumis_wasm_runtime::{
    HighlightOptions, HighlightOutput, HttpFetcher, LanguageStore, Runtime, StoreConfig,
};
use std::path::{Path, PathBuf};
#[cfg(test)]
use std::sync::Arc;
use tree_sitter::Tree;

/// The CLI's view of [`Runtime`]: the same one-pass highlighting Elixir and Node
/// use, plus the paths and cache reporting `lumis languages cache` prints.
pub(crate) struct Registry {
    data_dir: PathBuf,
    runtime: Runtime,
}

impl Registry {
    pub(crate) fn new(data_dir: PathBuf) -> Result<Self> {
        std::fs::create_dir_all(data_dir.join("parsers"))?;
        std::fs::create_dir_all(data_dir.join("themes"))?;
        let store = LanguageStore::new(
            StoreConfig {
                cache_dir: data_dir.clone(),
            },
            Box::new(HttpFetcher),
        );

        let runtime = lumis_wasm_runtime::runtime_with_catalog(store, 1)?;

        Ok(Self { data_dir, runtime })
    }

    pub(crate) fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    pub(crate) fn parse_tree(&self, language: &str, source: &str) -> Result<Tree> {
        Ok(self.runtime.parse_tree(source, language)?)
    }

    pub(crate) fn highlight(
        &self,
        source: &str,
        language: &str,
        options: &HighlightOptions,
    ) -> Result<HighlightOutput> {
        Ok(self.runtime.highlight_with(source, language, options)?)
    }

    /// Download and cache `languages` concurrently, one result per name.
    ///
    /// Caching a bundle one language at a time is a hundred sequential round
    /// trips to the CDN, which is most of the wall clock.
    pub(crate) fn cache_parsers_detailed(
        &self,
        languages: &[String],
        force: bool,
    ) -> Vec<Result<lumis_wasm_runtime::CacheLanguageOutcome>> {
        self.store()
            .cache_languages_detailed(languages, force, lumis_wasm_runtime::DOWNLOAD_CONCURRENCY)
            .into_iter()
            .map(|result| result.map_err(Into::into))
            .collect()
    }

    /// Compile and validate `languages` without retaining them.
    ///
    /// Each parser uses a disposable Tree-sitter store, so a whole catalog never
    /// shares one address space. The load validates the parser and writes its
    /// compiled form into the image; query configuration is validated too.
    pub(crate) fn precompile_parsers_detailed(
        &self,
        languages: &[String],
    ) -> Vec<Result<std::time::Duration>> {
        self.runtime
            .precompile_languages_detailed(languages, lumis_wasm_runtime::compile_concurrency())
            .into_iter()
            .map(|result| result.map_err(Into::into))
            .collect()
    }

    #[cfg(test)]
    pub(crate) fn is_cached(&self, language: &str) -> bool {
        self.local_package(language)
            .is_some_and(|package| self.store().cached_parser(&package).is_some())
    }

    #[cfg(test)]
    pub(crate) fn parser_path(&self, language: &str) -> Result<PathBuf> {
        Ok(self.store().parser_path(self.package(language)?.as_ref())?)
    }

    #[cfg(test)]
    pub(crate) fn parser_download_url(&self, language: &str) -> Result<String> {
        Ok(LanguageStore::parser_url(self.package(language)?.as_ref())?)
    }

    fn store(&self) -> &LanguageStore {
        self.runtime.store().expect("the CLI always has a store")
    }

    #[cfg(test)]
    fn package(&self, language: &str) -> Result<Arc<LanguagePackage>> {
        let location =
            catalog::find(language).with_context(|| format!("unknown language '{language}'"))?;
        Ok(self.store().package(location.package_name)?)
    }

    #[cfg(test)]
    fn local_package(&self, language: &str) -> Option<Arc<LanguagePackage>> {
        let location = catalog::find(language)?;
        self.store().local_package(location.package_name)
    }

    #[cfg(test)]
    pub(crate) fn cache_test_language(
        &self,
        id: &str,
        grammar_name: &str,
        wasm: &[u8],
        highlights: &str,
        injections: &str,
        locals: &str,
    ) {
        use lumis_wasm_runtime::{sha256_hex, write_atomic, PackagedLanguage, ParserMetadata};

        let location = catalog::find(id).unwrap();
        let package = LanguagePackage {
            package_name: location.package_name.into(),
            version: lumis_wasm_runtime::lowest_compatible_package_version(),
            definition_hash: "test".into(),
            parser: ParserMetadata {
                name: format!("tree-sitter-{grammar_name}"),
                grammar_name: grammar_name.into(),
                upstream_version: None,
                revision: None,
                sha256: sha256_hex(wasm),
                size: u64::try_from(wasm.len()).expect("parser size fits in u64"),
            },
            languages: std::collections::BTreeMap::from([(
                id.into(),
                PackagedLanguage {
                    highlights: highlights.into(),
                    injections: injections.into(),
                    locals: locals.into(),
                    ..PackagedLanguage::default()
                },
            )]),
        };
        write_atomic(
            &self.store().package_path(location.package_name).unwrap(),
            serde_json::to_string(&package).unwrap().as_bytes(),
        )
        .unwrap();
        write_atomic(&self.store().parser_path(&package).unwrap(), wasm).unwrap();
    }
}

pub(crate) fn all_language_ids() -> impl Iterator<Item = &'static str> {
    catalog::LANGUAGES.iter().map(|language| language.id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    const RUST_WASM: &[u8] = include_bytes!("../../../fixtures/test-parsers/tree-sitter-rust.wasm");

    fn cached_rust_registry() -> (tempfile::TempDir, Registry) {
        let dir = tempdir().unwrap();
        let registry = Registry::new(dir.path().to_path_buf()).unwrap();
        registry.cache_test_language(
            "rust",
            "rust",
            RUST_WASM,
            "(function_item \"fn\" @keyword.function)",
            "",
            "",
        );
        (dir, registry)
    }

    #[test]
    fn local_package_parses_and_highlights() {
        let (_dir, registry) = cached_rust_registry();
        let tree = registry.parse_tree("rust", "fn main() {}").unwrap();
        assert_eq!(tree.root_node().kind(), "source_file");

        let output = registry
            .highlight("fn main() {}", "rust", &HighlightOptions::default())
            .unwrap();
        assert!(
            !output.events.is_empty(),
            "highlighting Rust produced no events"
        );
    }

    #[test]
    fn parser_cache_is_content_addressed_and_verified() {
        let (_dir, registry) = cached_rust_registry();
        let path = registry.parser_path("rust").unwrap();
        assert!(path.to_string_lossy().contains(&format!(
            "tree-sitter-rust-{}-",
            lumis_wasm_runtime::lowest_compatible_package_version()
        )));
        std::fs::write(&path, b"corrupt").unwrap();
        assert!(!registry.is_cached("rust"));
        assert!(!path.exists());
    }

    #[test]
    fn package_url_is_exact_after_resolution() {
        let (_dir, registry) = cached_rust_registry();
        let url = registry.parser_download_url("rust").unwrap();
        assert!(url.contains(&format!(
            "@lumis-sh/wasm-rust@{}/",
            lumis_wasm_runtime::lowest_compatible_package_version()
        )));
        assert!(!url.contains("@latest"));
    }
}

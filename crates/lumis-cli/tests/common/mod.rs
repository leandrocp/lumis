use lumis_wasm_runtime::catalog;
use lumis_wasm_runtime::{sha256_hex, LanguagePackage, PackagedLanguage, ParserMetadata};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

pub(crate) fn source_fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
}

pub(crate) fn language_fixtures_dir() -> PathBuf {
    static FIXTURES: OnceLock<PathBuf> = OnceLock::new();
    FIXTURES.get_or_init(build_language_fixtures).clone()
}

pub(crate) fn data_dir() -> PathBuf {
    static DATA: OnceLock<PathBuf> = OnceLock::new();
    DATA.get_or_init(|| {
        let path = std::env::temp_dir().join(format!("lumis-cli-data-{}", std::process::id()));
        let parsers = path.join("parsers");
        let themes = path.join("themes");
        fs::create_dir_all(&parsers).unwrap();
        fs::create_dir_all(&themes).unwrap();
        for entry in fs::read_dir(language_fixtures_dir().join("parsers")).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() {
                fs::copy(entry.path(), parsers.join(entry.file_name())).unwrap();
            }
        }
        let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        for entry in fs::read_dir(repository.join("fixtures/conformance-themes")).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() {
                fs::copy(entry.path(), themes.join(entry.file_name())).unwrap();
            }
        }
        path
    })
    .clone()
}

fn build_language_fixtures() -> PathBuf {
    let destination =
        std::env::temp_dir().join(format!("lumis-cli-fixtures-{}", std::process::id()));
    let parsers = destination.join("parsers");
    fs::create_dir_all(&parsers).unwrap();

    let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let queries = repository.join("queries/processed");
    let definitions: toml::Value =
        toml::from_str(&fs::read_to_string(repository.join("languages.toml")).unwrap()).unwrap();
    let default_brackets = fs::read_to_string(queries.join("default/brackets.scm")).unwrap();

    for entry in fs::read_dir(repository.join("fixtures/test-parsers")).unwrap() {
        let wasm_path = entry.unwrap().path();
        let Some(stem) = wasm_path.file_stem().and_then(|name| name.to_str()) else {
            continue;
        };
        let Some(parser_id) = stem.strip_prefix("tree-sitter-") else {
            continue;
        };
        let location = catalog::find(parser_id).unwrap();
        let wasm = fs::read(&wasm_path).unwrap();
        let sha256 = sha256_hex(&wasm);
        let package_languages = catalog::LANGUAGES
            .iter()
            .filter(|language| language.package_name == location.package_name)
            .map(|language| {
                let query = |kind: &str| {
                    let query_name = definitions["parsers"][language.id]
                        .get("query_name")
                        .and_then(toml::Value::as_str)
                        .unwrap_or(language.id);
                    fs::read_to_string(queries.join(query_name).join(format!("{kind}.scm")))
                        .unwrap_or_default()
                };
                let brackets = {
                    let language_query = query("brackets");
                    if language_query.is_empty() {
                        default_brackets.clone()
                    } else {
                        language_query
                    }
                };
                (
                    language.id.into(),
                    PackagedLanguage {
                        aliases: language
                            .aliases
                            .iter()
                            .map(|alias| (*alias).into())
                            .collect(),
                        highlights: query("highlights"),
                        injections: query("injections"),
                        locals: query("locals"),
                        brackets,
                    },
                )
            })
            .collect::<BTreeMap<_, _>>();
        let package = LanguagePackage {
            package_name: location.package_name.into(),
            version: lumis_wasm_runtime::lowest_compatible_package_version(),
            parser: ParserMetadata {
                name: stem.into(),
                grammar_name: parser_id.into(),
                upstream_version: None,
                revision: None,
                sha256: sha256.clone(),
            },
            languages: package_languages,
        };
        let suffix = location
            .package_name
            .strip_prefix("@lumis-sh/wasm-")
            .unwrap();
        fs::write(
            parsers.join(format!("{suffix}.lumis.json")),
            serde_json::to_vec(&package).unwrap(),
        )
        .unwrap();
        // The store derives this name from the package, so it has to be built
        // the same way rather than assuming a version.
        fs::write(
            parsers.join(lumis_wasm_runtime::parser_filename(&package)),
            wasm,
        )
        .unwrap();
    }

    destination
}

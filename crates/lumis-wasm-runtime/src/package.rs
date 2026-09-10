//! Self-contained language package metadata shared by every native runtime.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use thiserror::Error;

/// A language package as published inside `@lumis-sh/wasm-*`.
///
/// There is deliberately **no `formatVersion` gate here**. Runtimes resolve this
/// document from a compatible range, so a hard format equality check would break
/// already-deployed clients when an additive package was published. Compatibility
/// is decided by the document's shape instead: neither
/// runtime rejects unknown fields, so additive changes are already safe, and a
/// change that removes or renames a required field fails `validate` with a message
/// naming the field.
///
/// That makes the format additive-only by contract. A change that alters the
/// *meaning* of an existing field without changing its shape cannot be detected
/// here and must be shipped as a new field instead.
///
/// The published npm `package.json` still carries `lumis.formatVersion`. That one is
/// release tooling (`dev wasm-needed`) deciding whether an artifact needs
/// republishing; no client reads it.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguagePackage {
    pub package_name: String,
    pub version: String,
    pub definition_hash: String,
    pub parser: ParserMetadata,
    pub languages: BTreeMap<String, PackagedLanguage>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParserMetadata {
    pub name: String,
    pub grammar_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    pub sha256: String,
    #[serde(deserialize_with = "deserialize_parser_size")]
    pub size: u64,
}

const JAVASCRIPT_MAX_SAFE_INTEGER: u64 = (1_u64 << 53) - 1;

fn deserialize_parser_size<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct ParserSizeVisitor;

    impl serde::de::Visitor<'_> for ParserSizeVisitor {
        type Value = u64;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                formatter,
                "an integral parser size no greater than {JAVASCRIPT_MAX_SAFE_INTEGER}"
            )
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            parser_size_from_u64(value)
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let value = u64::try_from(value).map_err(E::custom)?;
            parser_size_from_u64(value)
        }

        // The guards below leave `value` a non-negative integer no larger than
        // 2^53 - 1, which is exact in both `f64` and `u64`.
        #[allow(clippy::cast_precision_loss, clippy::cast_sign_loss)]
        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if !value.is_finite()
                || value < 0.0
                || value.fract() != 0.0
                || value > JAVASCRIPT_MAX_SAFE_INTEGER as f64
            {
                return Err(E::custom(format!(
                    "parser size must be an integer from 0 through {JAVASCRIPT_MAX_SAFE_INTEGER}"
                )));
            }
            parser_size_from_u64(value as u64)
        }
    }

    deserializer.deserialize_any(ParserSizeVisitor)
}

fn parser_size_from_u64<E>(value: u64) -> Result<u64, E>
where
    E: serde::de::Error,
{
    if value > JAVASCRIPT_MAX_SAFE_INTEGER {
        return Err(E::custom(format!(
            "parser size exceeds {JAVASCRIPT_MAX_SAFE_INTEGER}"
        )));
    }
    Ok(value)
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PackagedLanguage {
    // Deliberately not `#[serde(default)]`: the JavaScript validator requires
    // `aliases`, so tolerating its absence would accept packages Node rejects.
    pub aliases: Vec<String>,
    pub highlights: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub injections: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub locals: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub brackets: String,
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum LanguagePackageError {
    #[error("invalid language package JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("language package is missing {0}")]
    Missing(&'static str),
    #[error("language package has invalid {0}")]
    Invalid(&'static str),
    #[error("language '{language}' is not provided by {package}")]
    LanguageNotFound { language: String, package: String },
    #[error("invalid parser WASM size for '{parser}': expected {expected}, got {actual}")]
    InvalidSize {
        parser: String,
        expected: u64,
        actual: usize,
    },
    #[error(
        "invalid parser WASM integrity for '{parser}': expected sha256-{expected}, got sha256-{actual}"
    )]
    InvalidIntegrity {
        parser: String,
        expected: String,
        actual: String,
    },
    #[error("invalid parser grammar for '{parser}': expected '{expected}', got '{actual}'")]
    InvalidGrammar {
        parser: String,
        expected: String,
        actual: String,
    },
}

impl LanguagePackage {
    pub fn from_json(json: &str) -> Result<Self, LanguagePackageError> {
        // Typed deserialization skips unknown values. Parsing through Value
        // validates every raw token and gives duplicate members last-wins
        // behavior, matching JSON.parse and serde_json::Value maps.
        let value: serde_json::Value = serde_json::from_str(json)?;
        let package: Self = serde_json::from_value(value)?;
        package.validate()?;
        Ok(package)
    }

    pub fn validate(&self) -> Result<(), LanguagePackageError> {
        for (value, field) in [
            (&self.package_name, "packageName"),
            (&self.version, "version"),
            (&self.definition_hash, "definitionHash"),
            (&self.parser.name, "parser.name"),
            (&self.parser.grammar_name, "parser.grammarName"),
            (&self.parser.sha256, "parser.sha256"),
        ] {
            if value.is_empty() {
                return Err(LanguagePackageError::Missing(field));
            }
        }
        if self.languages.is_empty() {
            return Err(LanguagePackageError::Missing("languages"));
        }
        if has_ambiguous_language_names(&self.languages) {
            return Err(LanguagePackageError::Invalid("languages"));
        }
        if !is_valid_package_name(&self.package_name) {
            return Err(LanguagePackageError::Invalid("packageName"));
        }
        if !is_safe_path_segment(&self.version) {
            return Err(LanguagePackageError::Invalid("version"));
        }
        if !is_safe_path_segment(&self.parser.name) {
            return Err(LanguagePackageError::Invalid("parser.name"));
        }
        if self.parser.sha256.len() != 64
            || !self
                .parser
                .sha256
                .bytes()
                .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
        {
            return Err(LanguagePackageError::Invalid("parser.sha256"));
        }
        // Otherwise this surfaces much later as a confusing `InvalidSize` from
        // `verify_wasm`, and only for runtimes that reach that point.
        if self.parser.size == 0 || self.parser.size > JAVASCRIPT_MAX_SAFE_INTEGER {
            return Err(LanguagePackageError::Invalid("parser.size"));
        }
        Ok(())
    }

    pub fn language(&self, name: &str) -> Option<(&str, &PackagedLanguage)> {
        self.languages
            .iter()
            .find(|(id, language)| {
                id.eq_ignore_ascii_case(name)
                    || language
                        .aliases
                        .iter()
                        .any(|alias| alias.eq_ignore_ascii_case(name))
            })
            .map(|(id, language)| (id.as_str(), language))
    }

    pub fn require_language(
        &self,
        name: &str,
    ) -> Result<(&str, &PackagedLanguage), LanguagePackageError> {
        self.language(name)
            .ok_or_else(|| LanguagePackageError::LanguageNotFound {
                language: name.to_string(),
                package: self.package_name.clone(),
            })
    }

    pub fn verify_wasm(&self, bytes: &[u8]) -> Result<(), LanguagePackageError> {
        if u64::try_from(bytes.len()).ok() != Some(self.parser.size) {
            return Err(LanguagePackageError::InvalidSize {
                parser: self.parser.name.clone(),
                expected: self.parser.size,
                actual: bytes.len(),
            });
        }

        let actual = sha256_hex(bytes);
        if actual != self.parser.sha256 {
            return Err(LanguagePackageError::InvalidIntegrity {
                parser: self.parser.name.clone(),
                expected: self.parser.sha256.clone(),
                actual,
            });
        }

        let actual = grammar_name(bytes)?;
        if actual != self.parser.grammar_name {
            return Err(LanguagePackageError::InvalidGrammar {
                parser: self.parser.name.clone(),
                expected: self.parser.grammar_name.clone(),
                actual,
            });
        }
        Ok(())
    }

    #[cfg(feature = "wasm")]
    pub fn language_spec(
        &self,
        name: &str,
        wasm: Vec<u8>,
    ) -> Result<crate::LanguageSpec, LanguagePackageError> {
        self.verify_wasm(&wasm)?;
        let (id, language) = self.require_language(name)?;
        Ok(crate::LanguageSpec {
            id: id.to_string(),
            aliases: language.aliases.clone(),
            grammar_name: self.parser.grammar_name.clone(),
            wasm,
            highlights: language.highlights.clone(),
            injections: language.injections.clone(),
            locals: language.locals.clone(),
            brackets: language.brackets.clone(),
        })
    }
}

/// Returns the lowercase hexadecimal SHA-256 digest used by language packages.
pub fn sha256_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    let digest = Sha256::digest(bytes);
    let mut output = String::with_capacity(digest.len() * 2);
    for byte in digest {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

/// The grammar a parser WASM module provides, read from its exports.
///
/// A Tree-sitter parser exports exactly one `tree_sitter_<grammar>` symbol, and
/// loading it requires that name. Callers that build or vendor a parser have the
/// bytes but not the name, so this reads it rather than making them declare it.
///
/// # Errors
/// Fails when the module is not valid WASM, exports no such symbol, or exports
/// more than one.
pub fn grammar_name(wasm: &[u8]) -> Result<String, LanguagePackageError> {
    use wasmparser::{ExternalKind, Parser, Payload};

    let mut names = Vec::new();
    for payload in Parser::new(0).parse_all(wasm) {
        let payload =
            payload.map_err(|_| LanguagePackageError::Invalid("parser grammar export"))?;
        let Payload::ExportSection(exports) = payload else {
            continue;
        };
        for export in exports {
            let export =
                export.map_err(|_| LanguagePackageError::Invalid("parser grammar export"))?;
            if let Some(grammar) = export.name.strip_prefix("tree_sitter_") {
                if export.kind != ExternalKind::Func {
                    continue;
                }
                if !grammar.starts_with("external_scanner_") {
                    names.push(grammar.to_string());
                }
            }
        }
    }

    match names.as_slice() {
        [name] => Ok(name.clone()),
        _ => Err(LanguagePackageError::Invalid("parser grammar export")),
    }
}

fn has_ambiguous_language_names(languages: &BTreeMap<String, PackagedLanguage>) -> bool {
    let mut owners = BTreeMap::<String, String>::new();
    for (id, language) in languages {
        let owner = id.to_ascii_lowercase();
        if owners.insert(owner.clone(), owner.clone()).is_some() {
            return true;
        }
        for alias in &language.aliases {
            let alias = alias.to_ascii_lowercase();
            match owners.get(&alias) {
                Some(existing) if existing != &owner => return true,
                Some(_) => {}
                None => {
                    owners.insert(alias, owner.clone());
                }
            }
        }
    }
    false
}

pub(crate) fn is_safe_path_segment(value: &str) -> bool {
    !matches!(value, "" | "." | "..")
        && !value.ends_with([' ', '.'])
        && !value.chars().any(|character| {
            matches!(
                character,
                '\0'..='\u{1f}' | '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*'
            )
        })
        && !is_windows_device_name(value)
}

/// npm package-name grammar shared by package validation and cache-path derivation.
pub(crate) fn is_valid_package_name(package_name: &str) -> bool {
    if package_name.is_empty() || package_name.len() > 214 {
        return false;
    }

    let segments = match package_name.strip_prefix('@') {
        Some(scoped) => {
            let Some((scope, unscoped)) = scoped.split_once('/') else {
                return false;
            };
            [scope, unscoped]
        }
        None => [package_name, ""],
    };

    let valid_segment = |segment: &str| {
        segment.starts_with(|first: char| first.is_ascii_lowercase() || first.is_ascii_digit())
            && segment.bytes().all(|byte| {
                byte.is_ascii_lowercase() || byte.is_ascii_digit() || b"-._".contains(&byte)
            })
    };
    valid_segment(segments[0]) && (!package_name.starts_with('@') || valid_segment(segments[1]))
}

fn is_windows_device_name(value: &str) -> bool {
    let stem = value
        .split('.')
        .next()
        .unwrap_or(value)
        .trim_end_matches([' ', '.']);
    if ["con", "prn", "aux", "nul", "clock$", "conin$", "conout$"]
        .iter()
        .any(|name| stem.eq_ignore_ascii_case(name))
    {
        return true;
    }

    let (Some(prefix), Some(number)) = (stem.get(..3), stem.get(3..)) else {
        return false;
    };
    (prefix.eq_ignore_ascii_case("com") || prefix.eq_ignore_ascii_case("lpt"))
        && matches!(
            number,
            "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "¹" | "²" | "³"
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    const JSON_WASM: &[u8] = include_bytes!("../../../fixtures/test-parsers/tree-sitter-json.wasm");

    fn package() -> LanguagePackage {
        LanguagePackage {
            package_name: "@lumis-sh/wasm-json".into(),
            version: "0.26.3".into(),
            definition_hash: "definition".into(),
            parser: ParserMetadata {
                name: "tree-sitter-json".into(),
                grammar_name: "json".into(),
                upstream_version: None,
                revision: None,
                sha256: sha256_hex(JSON_WASM),
                size: u64::try_from(JSON_WASM.len()).expect("parser size fits in u64"),
            },
            languages: BTreeMap::from([(
                "json".into(),
                PackagedLanguage {
                    aliases: vec!["jsonc".into()],
                    highlights: "(string) @string".into(),
                    ..PackagedLanguage::default()
                },
            )]),
        }
    }

    #[test]
    fn resolves_ids_and_aliases() {
        let package = package();
        assert_eq!(package.require_language("json").unwrap().0, "json");
        assert_eq!(package.require_language("JSONC").unwrap().0, "json");
    }

    #[test]
    fn sha256_uses_lowercase_hex() {
        assert_eq!(
            sha256_hex(b"wasm"),
            "336154bf67f765f8f75d16a0accee61b5ee5f6a75b2a2905703df913bd550f3e"
        );
    }

    #[test]
    fn parser_provenance_is_optional_for_existing_packages() {
        let json = serde_json::to_string(&package()).unwrap();
        let parsed = LanguagePackage::from_json(&json).unwrap();

        assert_eq!(parsed.parser.upstream_version, None);
        assert_eq!(parsed.parser.revision, None);
    }

    #[test]
    fn rejects_wrong_parser_bytes() {
        let package = package();
        assert!(matches!(
            package.verify_wasm(b"bad"),
            Err(LanguagePackageError::InvalidSize { .. })
        ));
        let mut wrong = JSON_WASM.to_vec();
        let last = wrong.len() - 1;
        wrong[last] ^= 1;
        assert!(matches!(
            package.verify_wasm(&wrong),
            Err(LanguagePackageError::InvalidIntegrity { .. })
        ));
        assert!(package.verify_wasm(JSON_WASM).is_ok());
    }

    #[test]
    fn rejects_parser_bytes_whose_grammar_disagrees_with_metadata() {
        let mut package = package();
        package.parser.grammar_name = "not_json".into();

        let error = package.verify_wasm(JSON_WASM).unwrap_err();
        assert!(matches!(
            &error,
            LanguagePackageError::InvalidGrammar {
                parser,
                expected,
                actual,
            } if parser == "tree-sitter-json" && expected == "not_json" && actual == "json"
        ));
        assert_eq!(
            error.to_string(),
            "invalid parser grammar for 'tree-sitter-json': expected 'not_json', got 'json'"
        );
    }

    #[test]
    fn rejects_metadata_that_could_escape_the_cache_directory() {
        for (field, value) in [
            ("version", "../version"),
            ("parser.name", "../tree-sitter-json"),
        ] {
            let mut package = package();
            match field {
                "version" => package.version = value.into(),
                "parser.name" => package.parser.name = value.into(),
                _ => unreachable!(),
            }
            assert!(matches!(
                package.validate(),
                Err(LanguagePackageError::Invalid(actual)) if actual == field
            ));
        }
    }

    #[test]
    fn rejects_metadata_that_cannot_name_a_portable_file() {
        for value in [
            "C:",
            "C:parser",
            "tree<sitter",
            "tree>sitter",
            "tree\"sitter",
            "tree|sitter",
            "tree?sitter",
            "tree*sitter",
            "tree\u{1f}sitter",
            "tree-sitter-json ",
            "tree-sitter-json.",
            "CON",
            "nul.json",
            "Com1",
            "COM¹",
            "LPT9.log",
            "lpt³.log",
        ] {
            for field in ["version", "parser.name"] {
                let mut package = package();
                match field {
                    "version" => package.version = value.into(),
                    "parser.name" => package.parser.name = value.into(),
                    _ => unreachable!(),
                }
                assert!(
                    matches!(
                        package.validate(),
                        Err(LanguagePackageError::Invalid(actual)) if actual == field
                    ),
                    "{field} accepted {value:?}"
                );
            }
        }
    }
}

use lumis_core::themes;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

const SCHEMA_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../website/public/schemas/theme.json"
);

fn schema() -> Value {
    serde_json::from_str(&fs::read_to_string(SCHEMA_PATH).unwrap()).unwrap()
}

fn validator() -> jsonschema::Validator {
    jsonschema::draft202012::new(&schema()).expect("theme schema compiles")
}

#[test]
fn schema_is_a_valid_draft_2020_12_schema() {
    jsonschema::draft202012::meta::validate(&schema()).unwrap();
}

#[test]
fn every_bundled_theme_validates_and_points_at_the_schema() {
    let schema_id = schema()["$id"].clone();
    let validator = validator();
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("themes");
    let mut count = 0;

    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_none_or(|ext| ext != "json") {
            continue;
        }
        let theme: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        let errors: Vec<String> = validator
            .iter_errors(&theme)
            .map(|error| format!("{} at {}", error, error.instance_path()))
            .collect();
        assert!(errors.is_empty(), "{}: {errors:#?}", path.display());
        assert_eq!(theme["$schema"], schema_id, "{}", path.display());
        count += 1;
    }

    assert!(count > 200, "found only {count} bundled themes");
}

fn theme_with(highlights: Value) -> Value {
    json!({
        "$schema": "https://lumis.sh/schemas/theme.json",
        "name": "custom",
        "appearance": "light",
        "revision": "v1",
        "highlights": highlights,
    })
}

/// The schema accepts a document only if `from_json` loads it. It is stricter
/// in two ways, both for authors' sake: unknown keys, which Lumis ignores, are
/// usually typos, and colors other than `#rrggbb` do not render in every formatter.
#[test]
fn schema_agrees_with_from_json() {
    let mut missing_name = theme_with(json!({}));
    missing_name.as_object_mut().unwrap().remove("name");
    let mut without_license_or_schema = theme_with(json!({}));
    without_license_or_schema
        .as_object_mut()
        .unwrap()
        .remove("$schema");

    // (case, document, schema accepts, from_json accepts)
    let cases = [
        (
            "minimal",
            theme_with(json!({ "keyword": { "fg": "#ff79c6", "bold": true } })),
            true,
            true,
        ),
        (
            "no $schema or license",
            without_license_or_schema,
            true,
            true,
        ),
        (
            "every style field",
            theme_with(json!({ "comment": {
                "fg": "#000000", "bg": "#FFFFFF", "bold": true, "italic": true,
                "underline": "dashed", "strikethrough": true,
            }})),
            true,
            true,
        ),
        (
            "underline as boolean",
            theme_with(json!({ "a": { "underline": true } })),
            true,
            true,
        ),
        (
            "legacy undercurl",
            theme_with(json!({ "a": { "undercurl": true } })),
            true,
            true,
        ),
        ("missing name", missing_name, false, false),
        (
            "empty revision",
            json!({ "name": "x", "appearance": "dark", "revision": "", "highlights": {} }),
            false,
            false,
        ),
        (
            "unknown appearance",
            json!({ "name": "x", "appearance": "dim", "revision": "r", "highlights": {} }),
            false,
            false,
        ),
        (
            "unknown underline style",
            theme_with(json!({ "a": { "underline": "squiggly" } })),
            false,
            false,
        ),
        (
            "non-boolean bold",
            theme_with(json!({ "a": { "bold": "yes" } })),
            false,
            false,
        ),
        (
            "misspelled style field",
            theme_with(json!({ "a": { "forground": "#ffffff" } })),
            false,
            true,
        ),
        (
            "unknown top-level field",
            json!({ "name": "x", "appearance": "dark", "revision": "r", "highlight": {}, "highlights": {} }),
            false,
            true,
        ),
        (
            "short hex color",
            theme_with(json!({ "a": { "fg": "#fff" } })),
            false,
            true,
        ),
    ];

    let validator = validator();
    for (case, document, schema_accepts, from_json_accepts) in cases {
        assert_eq!(
            validator.is_valid(&document),
            schema_accepts,
            "schema on {case}"
        );
        assert_eq!(
            themes::from_json(&document.to_string()).is_ok(),
            from_json_accepts,
            "from_json on {case}"
        );
    }
}

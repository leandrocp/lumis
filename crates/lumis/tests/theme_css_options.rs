use lumis::themes::{self, CssBuilder};
use serde::Deserialize;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Deserialize)]
struct Manifest {
    options: Vec<OptionEntry>,
    theme: Value,
    cases: Vec<Case>,
}

#[derive(Deserialize)]
struct OptionEntry {
    name: String,
    default: Value,
}

#[derive(Deserialize)]
struct Case {
    name: String,
    options: BTreeMap<String, Value>,
    css: String,
}

fn manifest() -> Manifest {
    serde_json::from_str(include_str!("../../../fixtures/theme-css-options.json")).unwrap()
}

fn build(theme: &themes::Theme, options: &BTreeMap<String, Value>) -> String {
    let mut builder = CssBuilder::new(theme);
    for (name, value) in options {
        match name.as_str() {
            "layout" => {
                builder.layout(value.as_bool().unwrap());
            }
            "enable_italic" => {
                builder.enable_italic(value.as_bool().unwrap());
            }
            "scope" => {
                builder.scope(value.as_str().unwrap());
            }
            "container_selector" => {
                builder.container_selector(value.as_str().unwrap());
            }
            "container_style" => {
                let pairs: Vec<(String, String)> = serde_json::from_value(value.clone()).unwrap();
                builder.container_style(pairs);
            }
            _ => panic!("unhandled CSS option: {name}"),
        }
    }
    builder.build()
}

#[test]
fn manifest_matches_builder_fields() {
    let source = syn::parse_file(include_str!("../../lumis-core/src/themes.rs")).unwrap();
    let fields: BTreeSet<_> = source
        .items
        .into_iter()
        .find_map(|item| match item {
            syn::Item::Struct(item) if item.ident == "Css" => Some(
                item.fields
                    .into_iter()
                    .map(|field| field.ident.unwrap().to_string())
                    .filter(|name| name != "theme")
                    .collect(),
            ),
            _ => None,
        })
        .expect("Css struct exists");
    let expected = manifest()
        .options
        .into_iter()
        .map(|option| option.name)
        .collect();
    assert_eq!(fields, expected);
}

#[test]
fn shared_css_cases_and_defaults() {
    let manifest = manifest();
    let theme = themes::from_json(&manifest.theme.to_string()).unwrap();
    let defaults = manifest
        .options
        .into_iter()
        .map(|option| (option.name, option.default))
        .collect();
    assert_eq!(CssBuilder::new(&theme).build(), build(&theme, &defaults));
    assert!(manifest.cases.len() >= 6);
    for case in manifest.cases {
        assert_eq!(build(&theme, &case.options), case.css, "{}", case.name);
    }
}

#[test]
fn prebuilt_stylesheets_keep_layout() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let paths: Vec<_> = std::fs::read_dir(root.join("css"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "css"))
        .collect();
    assert!(paths.len() > 200);
    for path in paths {
        let name = path.file_stem().unwrap().to_str().unwrap();
        let theme = themes::get(name).unwrap();
        let bundled = std::fs::read_to_string(&path).unwrap();
        assert_eq!(CssBuilder::new(&theme).build(), bundled, "{name}");
        assert_eq!(
            CssBuilder::new(&theme).layout(false).build(),
            bundled.split_once("@layer lumis").unwrap().0,
            "{name}"
        );
        assert_eq!(
            std::fs::read_to_string(
                root.join("packages/elixir/lumis/priv/static/css")
                    .join(path.file_name().unwrap())
            )
            .unwrap(),
            bundled
        );
    }
}

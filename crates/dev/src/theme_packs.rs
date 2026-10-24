use crate::theme_licenses::{self, ResolvedTheme, ThemeLicenses};
use crate::ThemeDefinition;
use anyhow::{bail, ensure, Context, Result};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
struct Policy {
    schema_version: u32,
    packs: Vec<String>,
    default_licenses: Vec<String>,
    copyleft_licenses: Vec<String>,
    unavailable_licenses: Vec<String>,
    unavailable_themes: Vec<String>,
}

impl Policy {
    fn pack(&self, license: &str) -> Result<&'static str> {
        if self.default_licenses.iter().any(|item| item == license) {
            Ok("default")
        } else if self.copyleft_licenses.iter().any(|item| item == license) {
            Ok("copyleft")
        } else if self.unavailable_licenses.iter().any(|item| item == license) {
            Ok("unavailable")
        } else {
            bail!("unreviewed theme license {license}; update fixtures/theme-packs.json")
        }
    }
}

pub(crate) fn pack(license: &str) -> Result<&'static str> {
    policy()?.pack(license)
}

fn policy() -> Result<Policy> {
    Ok(serde_json::from_str(&fs::read_to_string(
        "fixtures/theme-packs.json",
    )?)?)
}

pub(crate) fn generate(
    themes: &[ResolvedTheme<'_>],
    definitions: &[ThemeDefinition],
    licenses: &ThemeLicenses,
    check: bool,
) -> Result<()> {
    let policy = policy()?;
    ensure!(policy.schema_version == 1, "unsupported theme pack policy");
    ensure!(
        policy.packs == ["default", "copyleft"],
        "unexpected theme packs"
    );
    let mut catalog = Vec::new();
    let mut unavailable = Vec::new();
    for theme in themes {
        let value: Value =
            serde_json::from_str(&fs::read_to_string(format!("themes/{}.json", theme.name))?)?;
        let pack = policy.pack(&theme.license.license)?;
        if pack == "unavailable" {
            unavailable.push(theme.name.to_owned());
            let metadata = json!({"name": theme.name, "appearance": value["appearance"], "revision": theme.revision});
            write_json(&format!("themes/{}.json", theme.name), &metadata, check)?;
            let css = Path::new("css").join(format!("{}.css", theme.name));
            if check {
                ensure!(
                    !css.exists(),
                    "unavailable theme CSS must not be distributed: {}",
                    css.display()
                );
            } else if css.exists() {
                fs::remove_file(css)?;
            }
        }
        catalog.push(json!({
            "name": theme.name,
            "appearance": value["appearance"],
            "pack": pack,
            "license": theme.license.license,
            "url": theme.url,
            "revision": theme.revision,
        }));
    }
    ensure!(
        unavailable == policy.unavailable_themes,
        "unavailable theme list changed; review fixtures/theme-packs.json"
    );
    write_json(
        "themes/catalog.json",
        &json!({"schema_version": 1, "themes": catalog}),
        check,
    )?;
    for pack in &policy.packs {
        generate_pack(pack, themes, definitions, licenses, &policy, check)?;
    }
    Ok(())
}

fn generate_pack(
    pack: &str,
    themes: &[ResolvedTheme<'_>],
    definitions: &[ThemeDefinition],
    licenses: &ThemeLicenses,
    policy: &Policy,
    check: bool,
) -> Result<()> {
    let selected = themes
        .iter()
        .filter_map(|theme| match policy.pack(&theme.license.license) {
            Ok(name) if name == pack => Some(Ok(theme.clone())),
            Ok(_) => None,
            Err(error) => Some(Err(error)),
        })
        .collect::<Result<Vec<_>>>()?;
    let definitions: Vec<_> = definitions
        .iter()
        .filter(|definition| selected.iter().any(|theme| theme.name == definition.name))
        .cloned()
        .collect();
    let data = selected
        .iter()
        .map(|theme| {
            let source = fs::read_to_string(format!("themes/{}.json", theme.name))?;
            let theme: lumis::themes::Theme = serde_json::from_str(&source)?;
            Ok(serde_json::to_value(theme)?)
        })
        .collect::<Result<Vec<Value>>>()?;
    let manifest = json!({"schema_version": 1, "pack": pack, "themes": data});
    write_json(&format!("themes/packs/{pack}.json"), &manifest, check)?;
    let notice = theme_licenses::render_notice(&selected, &definitions, licenses);
    let assets = license_assets(&selected, &definitions, licenses)?;
    for directory in pack_directories(pack) {
        write_json(&format!("{directory}/lumis-themes.json"), &manifest, check)?;
        write_file(
            Path::new(&directory).join("NOTICE.md"),
            notice.as_bytes(),
            check,
        )?;
        let license_directory = Path::new(&directory).join("licenses");
        if check {
            ensure!(
                file_count(&license_directory)? == assets.len(),
                "unexpected license files in {}",
                license_directory.display()
            );
        } else if license_directory.exists() {
            fs::remove_dir_all(&license_directory)?;
        }
        for (relative, bytes) in &assets {
            write_file(
                Path::new(&directory).join("licenses").join(relative),
                bytes,
                check,
            )?;
        }
    }
    Ok(())
}

fn file_count(directory: &Path) -> Result<usize> {
    let mut count = 0;
    for entry in fs::read_dir(directory)? {
        let path = entry?.path();
        count += if path.is_dir() { file_count(&path)? } else { 1 };
    }
    Ok(count)
}

fn pack_directories(pack: &str) -> Vec<String> {
    let suffix = if pack == "default" { "" } else { "-copyleft" };
    let rust = if pack == "default" {
        "lumis-themes"
    } else {
        "lumis-themes-copyleft-data"
    };
    vec![
        format!("crates/{rust}/data"),
        format!("packages/javascript/themes{suffix}/pack"),
        format!("packages/elixir/themes{suffix}/priv"),
    ]
}

fn license_assets(
    themes: &[ResolvedTheme<'_>],
    definitions: &[ThemeDefinition],
    licenses: &ThemeLicenses,
) -> Result<BTreeMap<PathBuf, Vec<u8>>> {
    let mut sources: Vec<_> = themes
        .iter()
        .map(|theme| (theme.url, theme.revision.as_deref()))
        .collect();
    for url in theme_licenses::dependency_users(definitions).keys() {
        sources.push((url, licenses.sources[*url].license_revision.as_deref()));
    }
    let mut assets = BTreeMap::new();
    for (url, revision) in sources {
        let license = &licenses.sources[url];
        let file = license
            .license_file
            .as_deref()
            .context("licensed pack source has no license file")?;
        let revision = revision.context("licensed pack source has no revision")?;
        let repository = url
            .strip_prefix("https://github.com/")
            .context("expected GitHub theme source")?;
        let relative = Path::new(repository).join(revision).join(file);
        let bytes = fs::read(Path::new("themes/licenses").join(&relative))?;
        assets.insert(relative, bytes);
    }
    Ok(assets)
}

fn write_json(path: &str, value: &Value, check: bool) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(value)?;
    write_file(Path::new(path), &bytes, check)
}

fn write_file(path: impl AsRef<Path>, bytes: &[u8], check: bool) -> Result<()> {
    let path = path.as_ref();
    if check {
        ensure!(
            fs::read(path).ok().as_deref() == Some(bytes),
            "missing or stale {}; run mise run docs-gen-themes-md",
            path.display()
        );
    } else {
        fs::create_dir_all(path.parent().context("pack file has no parent")?)?;
        fs::write(path, bytes)?;
    }
    Ok(())
}

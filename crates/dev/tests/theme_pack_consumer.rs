use flate2::read::GzDecoder;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::process::Command;

fn cargo(root: &Path, args: &[&str]) {
    let output = Command::new("cargo")
        .args(args)
        .current_dir(root)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "cargo {args:?}: {}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn packaged_dependency_alone_activates_and_removes_optional_themes() {
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap();
    cargo(
        &repository,
        &[
            "package",
            "-p",
            "lumis-themes-copyleft-data",
            "-p",
            "lumis-themes",
            "-p",
            "lumis-themes-copyleft",
            "-p",
            "lumis-core",
            "--allow-dirty",
            "--no-verify",
        ],
    );
    let root = std::env::temp_dir().join(format!("lumis-theme-consumer-{}", std::process::id()));
    fs::create_dir_all(root.join("src")).unwrap();
    let crates = [
        "lumis-core",
        "lumis-themes",
        "lumis-themes-copyleft",
        "lumis-themes-copyleft-data",
    ];
    let mut patches = String::from("\n[patch.crates-io]\n");
    let mut versions = std::collections::BTreeMap::new();
    for name in crates {
        let manifest: toml::Value = toml::from_str(
            &fs::read_to_string(repository.join(format!("crates/{name}/Cargo.toml"))).unwrap(),
        )
        .unwrap();
        let version = manifest["package"]["version"].as_str().unwrap();
        let archive =
            fs::File::open(repository.join(format!("target/package/{name}-{version}.crate")))
                .expect("run mise run test-theme-packs to package the consumer inputs");
        tar::Archive::new(GzDecoder::new(archive))
            .unpack(&root)
            .unwrap();
        writeln!(
            patches,
            "{name} = {{ path = {:?} }}",
            root.join(format!("{name}-{version}"))
        )
        .unwrap();
        versions.insert(name, version.to_owned());
    }
    let default: serde_json::Value = serde_json::from_slice(
        &fs::read(root.join(format!(
            "lumis-themes-{}/data/lumis-themes.json",
            versions["lumis-themes"]
        )))
        .unwrap(),
    )
    .unwrap();
    assert!(default["themes"].as_array().unwrap().len() > 200);
    assert!(default["themes"].as_array().unwrap().iter().all(|theme| ![
        "modus_vivendi",
        "darkplus",
        "no_clown_fiesta",
        "poimandres",
        "yorumi"
    ]
    .contains(&theme["name"].as_str().unwrap())));
    fs::write(root.join("src/main.rs"), r##"
use lumis_core::themes;
fn main() {
    let expected = std::env::args().nth(1).unwrap() == "optional";
    assert_eq!(themes::get("modus_vivendi").is_ok(), expected);
    assert_eq!(themes::get("github_light").unwrap().get_style("keyword").unwrap().fg.as_deref(), Some("#cf222e"));
    if expected {
        let theme = themes::get("modus_vivendi").unwrap();
        assert_eq!(theme.get_style("keyword").unwrap().fg.as_deref(), Some("#b6a0ff"));
        assert!(themes::CssBuilder::new(&theme).build().contains("#b6a0ff"));
    }
    for name in ["darkplus", "no_clown_fiesta", "poimandres", "yorumi"] { assert!(themes::get(name).is_err()); }
    println!("dependency-only catalog and exact colors verified");
}
"##).unwrap();
    let core_version = &versions["lumis-core"];
    let optional_version = &versions["lumis-themes-copyleft"];
    for resolver in ["2", "3"] {
        for optional in [false, true, false] {
            let dependency = if optional {
                format!("lumis-themes-copyleft = \"{optional_version}\"\n")
            } else {
                String::new()
            };
            let manifest = format!("[package]\nname=\"theme-consumer\"\nversion=\"0.1.0\"\nedition=\"2021\"\n[workspace]\nresolver=\"{resolver}\"\n[profile.release]\nlto=true\ncodegen-units=1\n[dependencies]\nlumis-core={{version=\"{core_version}\",default-features=false}}\n{dependency}{patches}");
            fs::write(root.join("Cargo.toml"), manifest).unwrap();
            cargo(
                &root,
                &[
                    "run",
                    "--offline",
                    "--release",
                    "--quiet",
                    "--",
                    if optional { "optional" } else { "default" },
                ],
            );
        }
    }
    fs::remove_dir_all(root).unwrap();
}

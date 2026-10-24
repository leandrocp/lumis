use std::fs;
use std::process::Command;

#[test]
fn notices_ship_revision_specific_licenses_and_check_them_offline() {
    let root = std::env::temp_dir().join(format!("lumis-theme-licenses-{}", std::process::id()));
    fs::create_dir_all(root.join("themes")).unwrap();
    fs::create_dir_all(root.join("docs/content/reference")).unwrap();
    fs::create_dir_all(root.join("fixtures")).unwrap();
    let mut policy: serde_json::Value =
        serde_json::from_str(include_str!("../../../fixtures/theme-packs.json")).unwrap();
    policy["unavailable_themes"] = serde_json::json!(["unlicensed"]);
    fs::write(
        root.join("fixtures/theme-packs.json"),
        serde_json::to_vec(&policy).unwrap(),
    )
    .unwrap();
    fs::write(
        root.join("themes/themes.lua"),
        r#"return {
            { name = "first", url = "https://github.com/a/theme", dependencies = { "https://github.com/b/dependency" } },
            { name = "second", url = "https://github.com/a/theme" },
            { name = "optional", url = "https://github.com/d/copyleft" },
            { name = "unlicensed", url = "https://github.com/c/none" },
        }"#,
    )
    .unwrap();
    fs::write(
        root.join("themes/licenses.toml"),
        r#"
        [sources."https://github.com/a/theme"]
        license = "MIT"
        license_file = "LICENSE.md"
        [sources."https://github.com/b/dependency"]
        license = "MIT"
        license_file = "LICENSE"
        license_revision = "cccccccccccccccccccccccccccccccccccccccc"
        [sources."https://github.com/c/none"]
        license = "NONE"
        [sources."https://github.com/d/copyleft"]
        license = "GPL-3.0"
        license_file = "LICENSE"
        "#,
    )
    .unwrap();
    for (name, revision) in [
        ("first", "a"),
        ("second", "b"),
        ("optional", "e"),
        ("unlicensed", "d"),
    ] {
        fs::write(
            root.join(format!("themes/{name}.json")),
            serde_json::to_vec(&serde_json::json!({"name": name, "appearance":"dark", "revision":revision.repeat(40), "highlights":{"normal":{"fg":"#abcdef"}}})).unwrap(),
        )
        .unwrap();
    }
    let files = [
        (
            format!("a/theme/{}/LICENSE.md", "a".repeat(40)),
            b"first\r\n".as_slice(),
        ),
        (
            format!("a/theme/{}/LICENSE.md", "b".repeat(40)),
            b"second\n".as_slice(),
        ),
        (
            format!("b/dependency/{}/LICENSE", "c".repeat(40)),
            b"dependency\n".as_slice(),
        ),
        (
            format!("d/copyleft/{}/LICENSE", "e".repeat(40)),
            b"optional license\n".as_slice(),
        ),
    ];
    for (file, bytes) in &files {
        let path = root.join("themes/licenses").join(file);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, bytes).unwrap();
    }
    let run = |check| {
        let mut command = Command::new(env!("CARGO_BIN_EXE_dev"));
        command.current_dir(&root).arg("gen-themes-md");
        if check {
            command.arg("--check");
        }
        command.output().unwrap()
    };
    let generated = run(false);
    assert!(generated.status.success(), "{generated:?}");
    for (file, bytes) in &files {
        let package = if file.starts_with("d/") {
            "lumis-themes-copyleft-data"
        } else {
            "lumis-themes"
        };
        assert_eq!(
            fs::read(
                root.join(format!("crates/{package}/data/licenses"))
                    .join(file)
            )
            .unwrap(),
            *bytes
        );
    }
    assert!(run(true).status.success());
    let notice = fs::read_to_string(root.join("themes/NOTICE.md")).unwrap();
    for (file, _) in &files {
        assert!(notice.contains(&format!("(licenses/{file})")));
    }
    assert!(notice.contains("None found"));

    let original = root.join("themes/licenses").join(&files[0].0);
    fs::remove_file(&original).unwrap();
    let missing = run(true);
    assert!(!missing.status.success());
    assert!(String::from_utf8_lossy(&missing.stderr).contains("themes-fetch-licenses"));
    fs::write(&original, []).unwrap();
    let empty = run(true);
    assert!(!empty.status.success());
    assert!(String::from_utf8_lossy(&empty.stderr).contains("empty license"));
    fs::write(&original, files[0].1).unwrap();

    let packaged = root
        .join("crates/lumis-themes/data/licenses")
        .join(&files[1].0);
    fs::write(&packaged, b"outdated").unwrap();
    let stale = run(true);
    assert!(!stale.status.success());
    assert!(String::from_utf8_lossy(&stale.stderr).contains("missing or stale"));
    fs::remove_file(packaged).unwrap();
    assert!(!run(true).status.success());
    assert!(run(false).status.success());
    assert!(run(true).status.success());
    fs::remove_dir_all(root).unwrap();
}

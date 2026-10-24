use lumis_core::themes;

#[test]
fn host_pack_installation_is_atomic_and_concurrently_idempotent() {
    let mut packs: Vec<String> = lumis_themes::packs().map(str::to_owned).collect();
    let mut upgraded: serde_json::Value = serde_json::from_str(&packs[0]).unwrap();
    let upgraded_name = upgraded["themes"][0]["name"].as_str().unwrap().to_owned();
    upgraded["themes"][0]["revision"] = serde_json::json!("newer-compatible-data-package");
    packs[0] = serde_json::to_string(&upgraded).unwrap();
    let packs: Vec<_> = packs.iter().map(String::as_str).collect();
    let before: Vec<_> = themes::available()
        .map(|theme| theme.name.clone())
        .collect();
    std::thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(|| themes::install_packs(&packs).unwrap());
        }
    });
    assert_eq!(
        themes::get(&upgraded_name).unwrap().revision,
        "newer-compatible-data-package"
    );
    let mut conflicting: serde_json::Value = serde_json::from_str(packs[0]).unwrap();
    conflicting["themes"][0]["name"] = serde_json::json!("must_not_be_installed");
    let conflicting = serde_json::to_string(&conflicting).unwrap();
    assert!(themes::install_packs(&[packs[0], &conflicting]).is_err());
    assert!(themes::install_packs(&[
        packs[0],
        r#"{"schema_version":2,"pack":"copyleft","themes":[]}"#
    ])
    .is_err());
    assert!(themes::get("must_not_be_installed").is_err());
    assert_eq!(
        themes::available()
            .map(|theme| theme.name.clone())
            .collect::<Vec<_>>(),
        before
    );
}

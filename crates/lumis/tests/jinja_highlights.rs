#![cfg(all(feature = "lang-jinja", feature = "lang-jinja-inline"))]

use lumis::{highlight::highlight_iter, languages::Language};

#[test]
fn jinja_member_access_and_filters_highlight() {
    for (language, source) in [
        (
            Language::Jinja,
            "{{ user.profile.name | escape }} {{ account.age + offset }}",
        ),
        (
            Language::JinjaInline,
            "# user.profile.name | escape\n# account.age + offset\n",
        ),
        (
            Language::Jinja,
            "# set label = user.profile.name | escape\n# set age = account.age + offset\n",
        ),
    ] {
        let mut tokens = Vec::new();
        highlight_iter(source, language, None, |text, _, _, scope, _| {
            tokens.push((text.to_string(), scope));
            Ok::<_, std::convert::Infallible>(())
        })
        .unwrap();

        for (text, scope) in [
            ("user", "variable"),
            ("account", "variable"),
            ("profile", "variable.member"),
            ("name", "variable.member"),
            ("age", "variable.member"),
            ("escape", "function.call"),
            ("offset", "variable"),
        ] {
            assert!(
                tokens
                    .iter()
                    .any(|token| token.0 == text && token.1 == scope),
                "missing {scope} capture for {text} in {language:?}: {tokens:?}"
            );
        }
    }
}

#![cfg(feature = "lang-sql")]

use lumis::{highlight::highlight_iter, languages::Language};

#[test]
fn upstream_sql_captures_preserve_local_highlighting_fixes() {
    let source = "SHOW ALL; SELECT OBJECT_ID('users'); SELECT id, 123, 45.67 FROM users; CREATE TABLE users (account_id INT); SELECT id FROM users QUALIFY ROW_NUMBER() OVER (PARTITION BY id) = 1;";
    let mut tokens = Vec::new();
    highlight_iter(source, Language::SQL, None, |text, _, _, scope, _| {
        tokens.push((text.to_string(), scope));
        Ok::<_, std::convert::Infallible>(())
    })
    .unwrap();

    for (text, scope) in [
        ("SHOW", "keyword"),
        ("OBJECT_ID", "function.call"),
        ("id", "variable.member"),
        ("123", "number"),
        ("45.67", "number.float"),
        ("account_id", "variable.member"),
        ("QUALIFY", "keyword"),
    ] {
        assert!(
            tokens
                .iter()
                .any(|token| token.0 == text && token.1 == scope),
            "missing {scope} capture for {text}: {tokens:?}"
        );
    }
}

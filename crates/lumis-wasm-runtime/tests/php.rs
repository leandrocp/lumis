#![cfg(feature = "wasm")]

use lumis_core::events::HighlightEvent;
use lumis_wasm_runtime::{LanguageSpec, Runtime, RuntimeError};

const SOURCE: &str = include_str!("../../../fixtures/conformance/php-heredoc-nowdoc/source.txt");
const FIXED: &[u8] = include_bytes!("../../../fixtures/test-parsers/tree-sitter-php.wasm");
const BROKEN: &[u8] = include_bytes!("../../../fixtures/failing-parsers/php-0.26.4.wasm");
const SIMPLE: &str = "<?php $a = \"x $b\";";

fn runtime(wasm: &[u8]) -> Runtime {
    let runtime = Runtime::with_worker_limit(1).unwrap();
    runtime
        .load_language(LanguageSpec {
            id: "php".into(),
            aliases: vec![],
            grammar_name: "php".into(),
            wasm: wasm.to_vec(),
            highlights: include_str!("../../../queries/processed/php_only/highlights.scm").into(),
            injections: String::new(),
            locals: String::new(),
            brackets: String::new(),
        })
        .unwrap();
    runtime
}

#[test]
fn php_heredocs_and_nowdocs_highlight_in_wasmtime() {
    let runtime = runtime(FIXED);
    for source in [SOURCE, SIMPLE] {
        let tree = runtime.parse_tree(source, "php").unwrap();
        assert!(!tree.root_node().has_error());
        let events = runtime.highlight(source, "php", false).unwrap();
        assert!(events
            .iter()
            .any(|event| matches!(event, HighlightEvent::Start { .. })));
        let recovered = events
            .iter()
            .filter_map(|event| match event {
                HighlightEvent::Source { start, end } => Some(&source[*start..*end]),
                _ => None,
            })
            .collect::<String>();
        assert_eq!(recovered, source);
    }
    runtime
        .highlight(include_str!("../../../samples/php.php"), "php", false)
        .unwrap();
}

#[test]
fn scanner_traps_are_parse_failures_and_the_worker_recovers() {
    let runtime = runtime(BROKEN);
    let expected = runtime.highlight(SIMPLE, "php", false).unwrap();
    for source in [
        "<?php\n$a = <<<END\nEND;\n",
        "<?php\n$a = <<<'END'\nx\nEND;\n",
    ] {
        for error in [
            runtime.highlight(source, "php", false).unwrap_err(),
            runtime.parse_tree(source, "php").unwrap_err(),
        ] {
            assert!(
                matches!(&error, RuntimeError::ParseFailed(language) if language == "php"),
                "{error:?}"
            );
            assert_eq!(
                error.to_string(),
                "parser returned no tree for language 'php'"
            );
        }
        assert_eq!(runtime.highlight(SIMPLE, "php", false).unwrap(), expected);
    }
}

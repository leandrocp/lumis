#![cfg(feature = "wasm")]

use lumis_core::events::HighlightEvent;
use lumis_wasm_runtime::{LanguageSpec, Runtime};

const SOURCE: &str = include_str!("../../../fixtures/conformance/php-heredoc-nowdoc/source.txt");
const FIXED: &[u8] = include_bytes!("../../../fixtures/test-parsers/tree-sitter-php.wasm");
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

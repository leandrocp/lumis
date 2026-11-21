use std::{fs, path::PathBuf};

const FIXED_ARRAY_HEADERS: &[&str] = &[
    "tree-sitter-bash/src/tree_sitter/array.h",
    "tree-sitter-html/src/tree_sitter/array.h",
    "tree-sitter-python/src/tree_sitter/array.h",
    "tree-sitter-ruby/src/tree_sitter/array.h",
    "tree-sitter-svelte/src/tree_sitter/array.h",
    "tree-sitter-xml/xml/src/tree_sitter/array.h",
    "tree-sitter-yaml/src/tree_sitter/array.h",
];

#[test]
fn affected_scanners_use_strict_aliasing_safe_array_headers() {
    let vendored_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("vendored_parsers");

    for relative_path in FIXED_ARRAY_HEADERS {
        let path = vendored_root.join(relative_path);
        let header = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));

        assert!(
            !header.contains("_array__grow((Array *)(self)"),
            "{} contains the strict-aliasing-unsafe array helper",
            path.display()
        );
        assert!(
            header.contains("#define _array__cast(self, expr)"),
            "{} does not use the corrected Tree-sitter array template",
            path.display()
        );
    }
}

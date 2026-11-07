; This file is auto-generated. Do not edit.
([
  (block_comment)
  (line_comment)
] @injection.content
  (#set! injection.language "comment")
  (#match? @injection.content "TODO|WIP|NOTE|XXX|INFO|DOCS|PERF|TEST|HACK|WARN|FIX|BUG|ERROR|#[0-9]+|https?://"))

((block_comment) @injection.content
  (#match? @injection.content "/[*][*][\\t-\\r ]")
  (#set! injection.language "javadoc"))

; markdown-style javadocs https://openjdk.org/jeps/467
((line_comment) @injection.content
  (#match? @injection.content "^///[\\t-\\r ]")
  (#set! injection.language "javadoc"))

((method_invocation
  name: (identifier) @_method
  arguments: (argument_list
    .
    (string_literal
      .
      (_) @injection.content)))
  (#any-of? @_method "format" "printf")
  (#set! injection.language "printf"))

((method_invocation
  object: (string_literal
    (string_fragment) @injection.content)
  name: (identifier) @_method)
  (#eq? @_method "formatted")
  (#set! injection.language "printf"))
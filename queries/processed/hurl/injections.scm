; This file is auto-generated. Do not edit.
((comment) @injection.content
  (#set! injection.language "comment")
  (#match? @injection.content "TODO|WIP|NOTE|XXX|INFO|DOCS|PERF|TEST|HACK|WARN|FIX|BUG|ERROR|#[0-9]+|https?://"))

; injections.scm
((json_value) @injection.content
  (#set! injection.language "json"))

((xml) @injection.content
  (#set! injection.language "xml"))

(multiline_string
  (multiline_string_type) @injection.language
  (multiline_string_content) @injection.content
  (#set! injection.combined))
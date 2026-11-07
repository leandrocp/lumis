; This file is auto-generated. Do not edit.
([
  (paired_comment)
  (unpaired_comment)
] @injection.content
  (#set! injection.language "comment")
  (#match? @injection.content "TODO|WIP|NOTE|XXX|INFO|DOCS|PERF|TEST|HACK|WARN|FIX|BUG|ERROR|#[0-9]+|https?://"))

((content) @injection.content
  (#set! injection.language "html")
  (#set! injection.combined))
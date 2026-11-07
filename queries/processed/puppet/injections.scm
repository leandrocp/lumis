; This file is auto-generated. Do not edit.
((regex) @injection.content
  (#set! injection.language "regex")
  (#offset! @injection.content 0 1 0 -1))

((comment) @injection.content
  (#set! injection.language "comment")
  (#match? @injection.content "TODO|WIP|NOTE|XXX|INFO|DOCS|PERF|TEST|HACK|WARN|FIX|BUG|ERROR|#[0-9]+|https?://"))
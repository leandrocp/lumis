; This file is auto-generated. Do not edit.
([
  (one_line_comment)
  (block_comment)
] @injection.content
  (#set! injection.language "comment")
  (#match? @injection.content "TODO|WIP|NOTE|XXX|INFO|DOCS|PERF|TEST|HACK|WARN|FIX|BUG|ERROR|#[0-9]+|https?://"))

((macro_text) @injection.content
  (#set! injection.language "verilog"))
; This file is auto-generated. Do not edit.
((comment) @injection.content
  (#set! injection.language "comment")
  (#match? @injection.content "TODO|WIP|NOTE|XXX|INFO|DOCS|PERF|TEST|HACK|WARN|FIX|BUG|ERROR|#[0-9]+|https?://"))

(command
  head: ((cmd_identifier) @_cmd
    (#match? @_cmd "^\\s*(find|parse|split|str)$"))
  flag: (_
    name: (_) @_flag
    (#any-of? @_flag "r" "regex"))
  .
  arg: (_
    (string_content) @injection.content
    (#set! injection.language "regex")))

(_
  opr: [
    "=~"
    "!~"
    "like"
    "not-like"
  ]
  rhs: (_
    (string_content) @injection.content
    (#set! injection.language "regex")))

(command
  head: (_) @_cmd
  (#any-of? @_cmd "nu" "$nu.current-exe")
  flag: (_
    name: (_) @_flag
    (#any-of? @_flag "c" "e" "commands" "execute"))
  .
  arg: (_
    (string_content) @injection.content
    (#set! injection.language "nu")))
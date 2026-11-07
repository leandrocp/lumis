; This file is auto-generated. Do not edit.
((comment) @injection.content
  (#set! injection.language "comment")
  (#match? @injection.content "TODO|WIP|NOTE|XXX|INFO|DOCS|PERF|TEST|HACK|WARN|FIX|BUG|ERROR|#[0-9]+|https?://"))

((comment) @injection.content
  (#match? @injection.content "^#:")
  (#set! injection.language "rbs"))

((comment) @injection.content
  (#match? @injection.content "^#[\\t-\\r ]+@rbs")
  (#set! injection.language "rbs"))

((comment) @injection.content
  (#match? @injection.content "^#[\\t-\\r ]+\\|")
  (#set! injection.language "rbs"))

(heredoc_body
  (heredoc_content) @injection.content
  (heredoc_end) @injection.language)

(regex
  (string_content) @injection.content
  (#set! injection.language "regex"))

((call
  receiver: (identifier) @_receiver
  method: (identifier) @_method
  arguments: (argument_list
    (pair
      key: (hash_key_symbol)
      value: (string
        (string_content) @injection.content))))
  (#eq? @_receiver "binding")
  (#any-of? @_method "b" "break")
  (#set! injection.self))
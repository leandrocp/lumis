; This file is auto-generated. Do not edit.
((comment) @injection.content
  (#set! injection.language "comment")
  (#match? @injection.content "TODO|WIP|NOTE|XXX|INFO|DOCS|PERF|TEST|HACK|WARN|FIX|BUG|ERROR|#[0-9]+|https?://"))

((call_expression
  (type) @_printf
  (named_arguments
    "("
    .
    (named_argument
      (expression
        (string_literal) @injection.content))))
  (#eq? @_printf "printf")
  (#offset! @injection.content 0 1 0 -1)
  (#set! injection.language "printf"))

; TODO: uncomment when asm is added
; ((asm_inline) @injection.content
;   (#set! injection.language "asm")
;   (#set! injection.combined))
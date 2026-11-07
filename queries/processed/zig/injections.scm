; This file is auto-generated. Do not edit.
((comment) @injection.content
  (#set! injection.language "comment")
  (#match? @injection.content "TODO|WIP|NOTE|XXX|INFO|DOCS|PERF|TEST|HACK|WARN|FIX|BUG|ERROR|#[0-9]+|https?://"))

; TODO: add when asm is added
; (asm_output_item (string) @injection.content
;   (#set! injection.language "asm"))
; (asm_input_item (string) @injection.content
;   (#set! injection.language "asm"))
; (asm_clobbers (string) @injection.content
;   (#set! injection.language "asm"))
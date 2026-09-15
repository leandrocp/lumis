; This file is auto-generated. Do not edit.
; Comments
((comment) @injection.content
  (#set! injection.language "comment"))

; Documentation
(unary_operator
  operator: "@"
  operand: (call
    target: ((identifier) @_identifier
      (#any-of? @_identifier "moduledoc" "typedoc" "shortdoc" "doc"))
    (arguments
      [
        (string
          (quoted_content) @injection.content)
        (sigil
          (quoted_content) @injection.content)
      ])
    (#set! injection.language "markdown")))

; HEEx
(sigil
  (sigil_name) @_sigil_name
  (quoted_content) @injection.content
  (#any-of? @_sigil_name "H" "LVN")
  (#set! injection.language "heex"))

; Surface
(sigil
  (sigil_name) @_sigil_name
  (quoted_content) @injection.content
  (#eq? @_sigil_name "F")
  (#set! injection.language "surface"))

; Zigler
(sigil
  (sigil_name) @_sigil_name
  (quoted_content) @injection.content
  (#any-of? @_sigil_name "E" "L")
  (#set! injection.language "eex"))

(sigil
  (sigil_name) @_sigil_name
  (quoted_content) @injection.content
  (#any-of? @_sigil_name "z" "Z")
  (#set! injection.language "zig"))

; Regex
(sigil
  (sigil_name) @_sigil_name
  (quoted_content) @injection.content
  (#any-of? @_sigil_name "r" "R")
  (#set! injection.language "regex"))

; Json
(sigil
  (sigil_name) @_sigil_name
  (quoted_content) @injection.content
  (#any-of? @_sigil_name "j" "J")
  (#set! injection.language "json"))
; Phoenix HTML template
((sigil
  (sigil_name) @_sigil_name
  (quoted_content) @injection.content)
 (#eq? @_sigil_name "HOLO")
 (#set! injection.language "heex")
 (#set! injection.combined))

; GPUI template
((sigil
  (sigil_name) @_sigil_name
  (quoted_content) @injection.content)
 (#eq? @_sigil_name "GPUI")
 (#set! injection.language "heex")
 (#set! injection.combined))

; SQL injection
((sigil
  (sigil_name) @_sigil_name
  (quoted_content) @injection.content)
 (#eq? @_sigil_name "SQL")
 (#set! injection.language "sql")
 (#set! injection.combined))

; Markdown
((sigil
  (sigil_name) @_sigil_name
  (quoted_content) @injection.content)
 (#eq? @_sigil_name "MD")
 (#set! injection.language "markdown")
 (#set! injection.combined))

; Lua
((sigil
  (sigil_name) @_sigil_name
  (quoted_content) @injection.content)
 (#eq? @_sigil_name "LUA")
 (#set! injection.language "lua")
 (#set! injection.combined))

; Python
((sigil
  (sigil_name) @_sigil_name
  (quoted_content) @injection.content)
 (#eq? @_sigil_name "PY")
 (#set! injection.language "python")
 (#set! injection.combined))

; JavaScript
((sigil
  (sigil_name) @_sigil_name
  (quoted_content) @injection.content)
 (#eq? @_sigil_name "JS")
 (#set! injection.language "javascript")
 (#set! injection.combined))

; Svelte
((sigil
  (sigil_name) @_sigil_name
  (quoted_content) @injection.content)
 (#eq? @_sigil_name "V")
 (#set! injection.language "svelte")
 (#set! injection.combined))

; Vue
((sigil
  (sigil_name) @_sigil_name
  (quoted_content) @injection.content)
 (#eq? @_sigil_name "VUE")
 (#set! injection.language "vue")
 (#set! injection.combined))

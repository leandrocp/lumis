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

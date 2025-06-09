; Svelte
(sigil
  (sigil_name) @_sigil_name
  (quoted_content) @injection.content
 (#eq? @_sigil_name "V")
 (#set! injection.language "svelte"))

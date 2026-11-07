; This file is auto-generated. Do not edit.
; an injections.scm file for nvim-treesitter
((comment) @injection.content
  (#set! injection.language "comment")
  (#match? @injection.content "TODO|WIP|NOTE|XXX|INFO|DOCS|PERF|TEST|HACK|WARN|FIX|BUG|ERROR|#[0-9]+|https?://"))

((pod) @injection.content
  (#set! injection.language "pod"))

((substitution_regexp
  (replacement) @injection.content
  (substitution_regexp_modifiers) @_modifiers)
  ; match if there's a single `e` in the modifiers list
  (#match? @_modifiers "e")
  (#not-match? @_modifiers "e[\\s\\S]*e")
  (#set! injection.language "perl")
  (#set! injection.include-children))

(heredoc_content
  (heredoc_end) @injection.language) @injection.content
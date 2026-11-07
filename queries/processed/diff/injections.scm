; This file is auto-generated. Do not edit.
((comment) @injection.content
  (#set! injection.language "comment")
  (#match? @injection.content "TODO|WIP|NOTE|XXX|INFO|DOCS|PERF|TEST|HACK|WARN|FIX|BUG|ERROR|#[0-9]+|https?://"))

; The file a hunk belongs to decides how its lines are highlighted, so the
; language comes from the `+++` path rather than from anything inside the hunk.
;
; `#offset!` drops the leading `+`, `-` or ` ` so the marker column never reaches
; the injected grammar, and keeps the newline so joined lines stay separate lines.
; A hunk is a fragment rather than a whole file, so the injected parse is expected
; to carry errors; that costs a few scopes inside the hunk and nothing around it.
;
; Upstream injects twice, once over context plus additions as the new file and
; once over context plus deletions as the old file. Both passes produce the same
; scopes over every context line, and Lumis nests events rather than resolving
; them by priority the way Neovim does, so each context line would carry two
; identical spans. Measured on a hunk replacing a `case` with a `with`, one pass
; over all three line kinds highlights every changed line identically and emits
; 36% less HTML. Interleaving the two versions costs nothing here, because a
; parse error inside a fragment is already expected and stays local.

(block
  (new_file
    (filename) @injection.filename)
  (hunks
    (hunk
      (changes
        [
          (context)
          (addition)
          (deletion)
        ] @injection.content)))
  (#set! injection.combined)
  (#offset! @injection.content 0 1 0 1))

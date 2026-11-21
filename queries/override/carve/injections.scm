([
  (comment_line)
  (trailing_comment)
] @injection.content
  (#set! injection.language "comment"))

; Upstream captures `(content)` inside a `[...]` alternation of
; `fenced_comment_block` and `braced_comment`. The comment injection filter
; cannot attach `#match?` to a capture inside an alternation, so preprocessing
; fails. Each alternative gets its own pattern; the matches are unchanged.
(fenced_comment_block
  (content) @injection.content
  (#set! injection.language "comment"))

(braced_comment
  (content) @injection.content
  (#set! injection.language "comment"))

(math
  (content) @injection.content
  (#set! injection.language "latex"))

(code_block
  (language) @injection.language
  (code) @injection.content)

(table
  (language) @injection.language
  (code) @injection.content)

(raw_block
  (raw_block_info
    (language) @injection.language)
  (content) @injection.content)

(raw_inline
  (content) @injection.content
  (raw_inline_attribute
    (language) @injection.language))

(frontmatter
  (language) @injection.language
  (frontmatter_content) @injection.content)

; Plain `---` frontmatter is YAML.
(frontmatter
  !language
  (frontmatter_content) @injection.content
  (#set! injection.language "yaml"))

([
  (comment_line)
  (trailing_comment)
] @injection.content
  (#set! injection.language "comment"))

([
  (fenced_comment_block
    (content) @injection.content)
  (braced_comment
    (content) @injection.content)
]
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

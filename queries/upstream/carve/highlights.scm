(heading) @markup.heading

((heading
  (marker) @_heading.marker) @markup.heading.1
  (#match? @_heading.marker "^# +$"))

((heading
  (marker) @_heading.marker) @markup.heading.2
  (#match? @_heading.marker "^## +$"))

((heading
  (marker) @_heading.marker) @markup.heading.3
  (#match? @_heading.marker "^### +$"))

((heading
  (marker) @_heading.marker) @markup.heading.4
  (#match? @_heading.marker "^#### +$"))

((heading
  (marker) @_heading.marker) @markup.heading.5
  (#match? @_heading.marker "^##### +$"))

((heading
  (marker) @_heading.marker) @markup.heading.6
  (#match? @_heading.marker "^###### +$"))

(thematic_break) @string.special

[
  (div_marker_begin)
  (div_marker_end)
] @punctuation.delimiter

([
  (code_block)
  (raw_block)
  (frontmatter)
] @markup.raw.block
  (#set! priority 90))

; Remove @markup.raw for code with a language spec
(code_block
  .
  (code_block_marker_begin)
  (language)
  (code) @none
  (#set! priority 90))

[
  (code_block_marker_begin)
  (code_block_marker_end)
  (raw_block_marker_begin)
  (raw_block_marker_end)
] @punctuation.delimiter

(language) @attribute

(inline_attribute
  _ @conceal
  (#set! conceal ""))

((language_marker) @punctuation.delimiter
  (#set! conceal ""))

(block_quote) @markup.quote

(block_quote_marker) @punctuation.special

; The colon fence's sigil family: `::: |`, `::: >` and `::: \`. Each
; opens a container whose body already carries its own color, so the sigil is
; the only character saying which one opened - and none of the three was
; painted, while every other marker node in this file is.
(line_block_marker) @punctuation.special

(block_quote_fence_marker) @punctuation.special

(local_hard_break_marker) @punctuation.special

(table_header) @markup.heading

(table_header
  "|" @punctuation.special)

(table_row
  "|" @punctuation.special)

(table_separator) @punctuation.special

(table_caption
  (marker) @punctuation.special)

(table_caption) @markup.italic

(caption
  (caption_marker) @punctuation.special)

(caption
  (caption_content) @markup.italic)

[
  (list_marker_dash)
  (list_marker_star)
  (list_marker_definition)
  (list_marker_description)
  (list_marker_decimal_period)
  (list_marker_decimal_paren)
  (list_marker_lower_alpha_period)
  (list_marker_lower_alpha_paren)
  (list_marker_upper_alpha_period)
  (list_marker_upper_alpha_paren)
  (list_marker_lower_roman_period)
  (list_marker_lower_roman_paren)
  (list_marker_upper_roman_period)
  (list_marker_upper_roman_paren)
] @markup.list

; The `+` continuation operator attaches the next block to its container.
(list_continuation_marker) @operator

(list_marker_task
  (unchecked)) @markup.list.unchecked

(list_marker_task
  (checked)) @markup.list.checked

; Colorize `x` in `[x]`
((checked) @constant.builtin
  (#offset! @constant.builtin 0 1 0 -1))

[
  (ellipsis)
  (en_dash)
  (em_dash)
  (quotation_marks)
] @string.special

(list_item
  (term) @type.definition)

; Conceal the backslash of an escaped quote
((quotation_marks) @string.special
  (#offset! @string.special 0 0 0 -1)
  (#set! conceal ""))

[
  (hard_line_break)
  (backslash_escape)
] @string.escape

; Only conceal \ but leave escaped character.
((backslash_escape) @string.escape
  (#offset! @string.escape 0 0 0 -1)
  (#set! conceal ""))

(frontmatter_marker) @punctuation.delimiter

(emphasis) @markup.italic

(strong) @markup.strong

(bold_italic) @markup.strong
(bold_italic) @markup.italic

(underline) @markup.underline

(strikethrough) @markup.strikethrough

(symbol) @string.special.symbol

(extension_inline) @function.macro

; Reserved processor syntax (PART 9 section 19). The PARTS are captured, not the
; whole run: the directive's own context already keeps `#section` away from the
; tag rule and an option away from the mention rule, so there is nothing to gain
; by painting it one colour - and a reader wants the path to look like a path.
(include_open) @punctuation.special
(include_close) @punctuation.special
(include_path) @string.special.path
(include_section) @label
(include_option_name) @variable.parameter
(include_option_separator) @punctuation.delimiter
(include_option_value) @constant

(mention) @constant

(tag) @tag

(insert) @markup.underline

(delete) @markup.strikethrough

(substitution) @markup.strikethrough
(editorial_comment) @comment

[
  (highlighted)
  (superscript)
  (subscript)
] @string.special

([
  (emphasis_begin)
  (emphasis_end)
  (bold_italic_begin)
  (bold_italic_end)
  (strong_begin)
  (strong_end)
  (underline_begin)
  (underline_end)
  (strikethrough_begin)
  (strikethrough_end)
  (superscript_begin)
  (superscript_end)
  (subscript_begin)
  (subscript_end)
  (highlighted_begin)
  (highlighted_end)
  (insert_begin)
  (insert_end)
  (delete_begin)
  (delete_end)
  (verbatim_marker_begin)
  (verbatim_marker_end)
  (math_marker)
  (math_marker_begin)
  (math_marker_end)
  (literal_marker)
  (literal_marker_begin)
  (literal_marker_end)
  (raw_inline_attribute)
  (raw_inline_marker_begin)
  (raw_inline_marker_end)
] @punctuation.delimiter
  (#set! conceal ""))

((math) @markup.math
  (#set! priority 90))

(verbatim) @markup.raw

((raw_inline) @markup.raw
  (#set! priority 90))

; Inline literal renders as prose (no code/math face), so capture it plainly.
(inline_literal) @none

[
  (comment_line)
  (fenced_comment_block)
  (braced_comment)
  (trailing_comment)
] @comment

(span
  [
    "["
    "]"
  ] @punctuation.bracket)

(inline_attribute
  [
    "{"
    "}"
  ] @punctuation.bracket)

(block_attribute
  [
    "{"
    "}"
  ] @punctuation.bracket)

; A `.foo` in an attribute block, and the colon fence's TYPE WORD. The second
; used to be spelled `class_name` too, which is what the attribute's rule is
; called - `admonition_type` is the construct the fence actually opens
; (grammar.ebnf: any word after the separator is an admonition, and a generic
; div is the opener with no word at all).
[
  (class)
  (admonition_type)
] @type

; Composite figures (PART 9 4c, markup-carve/carve#1215). The kind word `figure`
; is RESERVED among the `:::` types: a BARE opener - the fence, its separator,
; the word, and nothing else - is ONE figure of ordered panels, not an
; admonition. An opener carrying a quoted title or a `[label]` is not that
; production and takes `@type` above, which is the generic Tier-2 container the
; clause says it stays.
;
; THE PARSE TREE NOW SPELLS THE DISTINCTION, so this is one pattern over one
; node. It used to be four: a pattern over the admonition's type word with
; `!title !label` predicates, plus three wildcard chains restoring `@type` on a
; bare opener nested inside a group, because a query has no transitive closure
; and GROUPS DO NOT NEST at ANY depth. The chains reached three levels and the
; residual was written down - a bare opener deeper than that kept the group
; colour.
;
; Both go away because the demotion moved to where depth is free.
; `src/scanner.c` reads its own open-block stack, so a bare opener inside an
; open group is an `admonition_type` in the TREE - which is what the engine
; builds for it too - and no query has to reconstruct the ancestry.
;
; The group caption needs no rule: it is an ordinary `^ ` line one line below
; the closing fence, and the parser already places it as a sibling of the
; container rather than inside it, where the existing `(caption)` patterns claim
; it.
(figure_group_marker) @type.builtin

(identifier) @tag

(key_value
  "=" @operator)

(key_value
  (key) @property)

(key_value
  (value) @string)

(boolean_attribute) @property

((span
  attribute: (inline_attribute
    (args
      (boolean_attribute) @attribute.builtin)))
  (#eq? @attribute.builtin "kbd"))

((span
  attribute: (inline_attribute
    (args
      (key_value
        key: (key) @attribute.builtin))))
  (#any-of? @attribute.builtin "abbr" "time"))

((_
  (block_attribute
    args: (args
      (boolean_attribute) @attribute.builtin))
  .
  [(block_attribute) (block_quote_marker)]*
  .
  (table
    language: (language) @_language))
  (#eq? @attribute.builtin "table")
  (#any-of? @_language "csv" "tsv"))

; A CSV or TSV code include renders as a table under `{table}` too
; (markup-carve/carve#3335); the include must be the paragraph's only content.
((_
  (block_attribute
    args: (args
      (boolean_attribute) @attribute.builtin))
  .
  [(block_attribute) (block_quote_marker)]*
  .
  (paragraph
    .
    (include_directive
      option: (include_option
        name: (include_option_name) @_option
        value: (include_option_value) @_language))
    .))
  (#eq? @attribute.builtin "table")
  (#eq? @_option "@code")
  (#any-of? @_language "csv" "tsv"))

((_
  (block_attribute
    args: (args
      (boolean_attribute) @attribute.builtin))
  .
  [(block_attribute) (block_quote_marker)]*
  .
  [
    (list)
    (definition_list)
  ])
  (#eq? @attribute.builtin "loose"))

(language_attribute) @attribute

(link_text
  [
    "["
    "]"
  ] @punctuation.bracket
  (#set! conceal ""))

(autolink
  [
    "<"
    ">"
  ] @punctuation.bracket
  (#set! conceal ""))

(inline_link
  (inline_link_destination) @markup.link.url
  (#set! conceal ""))

(link_reference_definition
  ":" @punctuation.special)

(full_reference_link
  (link_text) @markup.link)

(full_reference_link
  (link_label) @markup.link.label
  (#set! conceal ""))

(collapsed_reference_link
  "[]" @punctuation.bracket
  (#set! conceal ""))

(full_reference_link
  [
    "["
    "]"
  ] @punctuation.bracket
  (#set! conceal ""))

(collapsed_reference_link
  (link_text) @markup.link)

(collapsed_reference_link
  (link_text) @markup.link.label)

(inline_link
  (link_text) @markup.link)

(full_reference_image
  (link_label) @markup.link.label)

(full_reference_image
  [
    "["
    "]"
  ] @punctuation.bracket)

(collapsed_reference_image
  "[]" @punctuation.bracket)

(image_description
  [
    "!["
    "]"
  ] @punctuation.bracket)

(image_description) @markup.italic

(link_reference_definition
  [
    "["
    "]"
  ] @punctuation.bracket)

(link_reference_definition
  (link_label) @markup.link.label)

(inline_link_destination
  [
    "("
    ")"
  ] @punctuation.bracket)

[
  (autolink)
  (inline_link_destination)
  (link_destination)
  (link_reference_definition)
] @markup.link.url

; A cross-reference with auto text is a LINK but not a URL: `</#Intro>` carries
; a heading id the renderer resolves to the target's own text, so it takes
; @markup.link rather than the @markup.link.url above. It used to parse as an
; autolink and take that capture, which colored a crossref as a web address.
(auto_text_link) @markup.link

(abbreviation_definition
  (abbreviation_marker) @punctuation.special)

(abbreviation_definition
  (abbreviation_expansion) @string)

; Citations (§22 / Tier-2 extension)
(citation_group) @string.special

(citation_definition
  (citation_label) @markup.link.label)

(citation_definition
  (citation_entry) @string)

; Callout list (§10 / Tier-2 extension)
(callout_list) @markup.list

(callout_list_item) @markup.list

(footnote
  (reference_label) @markup.link.label)

(footnote_reference
  (reference_label) @markup.link.label)

; An inline note, `^[content]`. Its content is ordinary inline and highlights
; itself; this marks the note as a whole so a reader can see where it ends -
; which is the thing the construct is easy to get wrong about, and which this
; grammar does get wrong for a note holding a bracket run that forms no span
; (see the RECORDED GAP fixture). Painting the whole node is what makes that
; visible in an editor rather than only in a tree dump.
(inline_note) @markup.link.label

[
  (footnote_marker_begin)
  (footnote_marker_end)
] @punctuation.bracket

; TODO / NOTE / FIXME inside comments come from the comment injection; in
; prose they are ordinary words.
[
  (paragraph)
  (table_cell)
  (comment_line)
  (trailing_comment)
  (fenced_comment_block)
  (braced_comment)
] @spell

[
  (autolink)
  (auto_text_link)
  (inline_link_destination)
  (link_destination)
  (code_block)
  (raw_block)
  (math)
  (raw_inline)
  (verbatim)
  (inline_literal)
  (reference_label)
  (class)
  (admonition_type)
  (identifier)
  (key_value)
  (frontmatter)
] @nospell

(table
  (code) @nospell)

(full_reference_link
  (link_label) @nospell)

(full_reference_image
  (link_label) @nospell)

(invalid_metadata) @error

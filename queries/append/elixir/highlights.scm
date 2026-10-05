; * doc string
(unary_operator
  operator: "@" @comment.doc
  operand: (call
    target: (identifier) @comment.doc.__attribute__
    (arguments
      [
        (string) @comment.doc
        (charlist) @comment.doc
        (sigil
          quoted_start: _ @comment.doc
          quoted_end: _ @comment.doc) @comment.doc
        (boolean) @comment.doc
      ]))
  (#any-of? @comment.doc.__attribute__ "moduledoc" "typedoc" "doc"))

; Upstream gives every do @keyword, unlike defmodule's @keyword.function.
(call
  target: (identifier) @_definition
  (do_block
    "do" @keyword.function)
  (#eq? @_definition "defmodule"))

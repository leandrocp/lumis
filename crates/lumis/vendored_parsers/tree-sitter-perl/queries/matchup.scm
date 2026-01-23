(conditional_statement
    ["if" "unless"] @open.if
    "elsif"? @mid.if.1
    "else"? @mid.if.2
    (block "}" @close.if) .
) @scope.if

(_
  ["for" "foreach" "while" "unless"] @open.loop
  (block "}" @close.loop) .
  ) @scope.loop
(loopex_expression) @mid.loop.1

(_
    "sub" @open.fun
    (block "}" @close.fun) .
) @scope.fun
(return_expression "return" @mid.fun.1)

[
  (_ "'" @open.quotelike (string_content) "'" @close.quotelike)
  (quoted_regexp "'" @open.quotelike "'" @close.quotelike)
  (_ "'" @open.quotelike (_) "'"+ @mid.quotelike.1 (replacement) "'" @close.quotelike)
] @scope.quotelike

(try_statement
  "try" @open.try
  "catch"? @mid.try.1
  "finally"? @close.try
  ) @scope.try

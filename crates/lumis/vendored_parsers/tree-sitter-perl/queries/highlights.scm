((source_file . (comment) @preproc)
  (#lua-match? @preproc "^#!/"))

[ "use" "no" "require" ] @include

[ "if" "elsif" "unless" "else" ] @conditional

(conditional_expression [ "?" ":" ] @conditional.ternary)

[ "while" "until" "for" "foreach" ] @repeat
("continue" @repeat (block))

[ "try" "catch" "finally" ] @exception

"return" @keyword.return

[ "sub" "method" "async" "extended" ] @keyword.function

[ "map" "grep" "sort" ] @function.builtin

[ "package" "class" "role" ] @include

[
  "defer"
  "do" "eval"
  "my" "our" "local" "dynamically" "state" "field"
  "last" "next" "redo" "goto"
  "undef" "await"
] @keyword

(yadayada) @exception

(phaser_statement phase: _ @keyword.phaser)
(class_phaser_statement phase: _ @keyword.phaser)



(_ operator: _ @operator)
"\\" @operator

[
  "or" "xor" "and"
  "eq" "ne" "cmp" "lt" "le" "ge" "gt"
  "isa"
] @keyword.operator

(eof_marker) @preproc
(data_section) @comment

(pod) @text

[
  (number)
  (version)
] @number

[
  (string_literal)
  (interpolated_string_literal)
  (quoted_word_list)
  (command_string)
  (heredoc_content)
  (replacement)
  (transliteration_content)
] @string

[
  (heredoc_token)
  (command_heredoc_token)
  (heredoc_end)
] @label

[(escape_sequence) (escaped_delimiter)] @string.escape

(_ modifiers: _ @character.special)
[
 (quoted_regexp)
 (match_regexp)
 (regexp_content)
] @string.regex

(autoquoted_bareword) @string.special

(use_statement (package) @type)
(package_statement (package) @type)
(class_statement (package) @type)
(require_expression (bareword) @type)

(subroutine_declaration_statement name: (bareword) @function)
(method_declaration_statement name: (bareword) @method)
(attribute_name) @attribute
(attribute_value) @string

(label) @label

(statement_label label: _ @label)

(relational_expression operator: "isa" right: (bareword) @type)

(function) @function

(function_call_expression (function) @function.call)
(method_call_expression (method) @method.call)
(method_call_expression invocant: (bareword) @type)

(func0op_call_expression function: _ @function.builtin)
(func1op_call_expression function: _ @function.builtin)

([(function)(expression_statement (bareword))] @function.builtin
 (#match? @function.builtin
   "^(accept|atan2|bind|binmode|bless|crypt|chmod|chown|connect|die|dbmopen|exec|fcntl|flock|getpriority|getprotobynumber|gethostbyaddr|getnetbyaddr|getservbyname|getservbyport|getsockopt|glob|index|ioctl|join|kill|link|listen|mkdir|msgctl|msgget|msgrcv|msgsend|opendir|print|printf|push|pack|pipe|return|rename|rindex|read|recv|reverse|say|select|seek|semctl|semget|semop|send|setpgrp|setpriority|seekdir|setsockopt|shmctl|shmread|shmwrite|shutdown|socket|socketpair|split|sprintf|splice|substr|system|symlink|syscall|sysopen|sysseek|sysread|syswrite|tie|truncate|unlink|unpack|utime|unshift|vec|warn|waitpid|formline|open|sort)$"
))

(ERROR) @error

(
  [(varname) (filehandle)] @variable.builtin
  (#match? @variable.builtin "^((ENV|ARGV|INC|ARGVOUT|SIG|STDIN|STDOUT|STDERR)|[_ab]|\\W|\\d+|\\^.*)$")
)

[(array) (arraylen)] @variable.array
(glob) @variable.builtin
(scalar) @variable.scalar
(hash) @variable.hash
(amper_deref_expression [ "&" "*" ] @function.call)

(glob_deref_expression "*" @variable.builtin)
(glob_slot_expression "*" @variable.builtin)
(scalar_deref_expression [ "$" "*"] @variable.scalar)

; gotta be SUPER GENERIC so we can hit up string interp
(_
  [
   array: (_) @variable.array
   hash: (_) @variable.hash
  ])
(postfix_deref ["@" "$#" ] @variable.array "*" @variable.array)
(postfix_deref "%" @variable.hash "*" @variable.hash)
(slices
  hashref:_ [ "@" "%" ] @variable.hash )
(slices
  arrayref:_  [ "@" "%" ] @variable.array )



(comment) @comment

([ "=>" "," ";" "->" ] @punctuation.delimiter)

(
  [ "[" "]" "{" "}" "(" ")" ] @punctuation.bracket
)

(_
  "{" @punctuation.special
  (varname)
  "}" @punctuation.special)

(varname
  (block
    "{" @punctuation.special
    "}" @punctuation.special))


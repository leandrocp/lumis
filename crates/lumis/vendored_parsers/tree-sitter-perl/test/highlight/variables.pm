$s; @a; %h;
# <- variable.scalar
#   ^ variable.array
#       ^ variable.hash
$#arrlen;
# <- variable.array
$one + $two;
# <- variable.scalar
#      ^ variable.scalar
my $var;
# <- keyword
#  ^ variable.scalar
my ($x, undef, $z);
# <- keyword
#   ^ variable.scalar
#       ^ keyword
#              ^ variable.scalar
our $PackageVar;
# <- keyword
#   ^ variable.scalar
my $var :lock;
# <- keyword
#        ^ attribute
$sref->$*;
# <- variable.scalar
#    ^^ punctuation.delimiter
#      ^^ variable.scalar
$aref->@*;
# <- variable.scalar
#    ^^ punctuation.delimiter
#      ^^ variable.array
$href->%*;
# <- variable.scalar
#    ^^ punctuation.delimiter
#      ^^ variable.hash
$arr[ 123 ];
# <- variable.array
# ^^ variable.array
#   ^ punctuation.bracket
#     ^^^ number
#         ^ punctuation.bracket
$aref->[ 123 ];
# <- variable.scalar
#    ^^ punctuation.delimiter
#      ^ punctuation.bracket
#        ^^^ number
#            ^ punctuation.bracket
$hash{ key };
# <- variable.hash
# ^^^ variable.hash
#    ^ punctuation.bracket
#      ^^^ string.special
#          ^ punctuation.bracket
$hash{q}{shift};
#     ^ string.special
#      ^^ punctuation.bracket
#        ^^^^^ string.special
$href->{ key };
# <- variable.scalar
#    ^^ punctuation.delimiter
#      ^ punctuation.bracket
#        ^^^ string.special
#            ^ punctuation.bracket
$aref->[ 123 ]{ 456 }[ 789 ];
# <- variable.scalar
#    ^^ punctuation.delimiter
#      ^ punctuation.bracket
#        ^^^ number
#            ^^ punctuation.bracket
#               ^^^ number
#                   ^^ punctuation.bracket
#                      ^^^ number
#                          ^ punctuation.bracket
@ary[1,2,3];
#^^ variable.array
@hash{1,2,3};
#^^^ variable.hash
%ary[1,2,3];
#^^ variable.array
%hash{1,2,3};
#^^ variable.hash
$1;
# <- variable.scalar
$^X;
# <- variable.scalar
$!;
# <- variable.scalar
my $not::allowed;
#         ^ attribute
#         this is b/c it's a syntax error
${ ^ANY_IDENT1 };
#  ^^^^^ variable.builtin
${+shift};
#^ punctuation.special
#    ^ function.builtin
#       ^ punctuation.special

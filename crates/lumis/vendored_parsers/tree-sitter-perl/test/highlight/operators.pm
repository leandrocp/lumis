1 + 1;
# ^ operator
1 or 2;
# <- number
# ^ keyword.operator
#    ^ number
1 and 2;
# ^ keyword.operator
12 eq 34;
#  ^ keyword.operator
12 eq 34 eq 45;
#        ^ keyword.operator
12 cmp 34;
#  ^ keyword.operator
12 isa 34;
#  ^ keyword.operator
$obj isa SomeClass;
#        ^^^^^^^^^ type
%hash = (foo => "bar", shift => 'thing', qq => 'thing');
# <- variable.hash
#        ^^^ string.special
#               ^^^^^ string
#                      ^^^^^ string.special
#                                        ^^ string.special

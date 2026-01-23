sub foo
# <- keyword.function
#   ^ function
{
   123;
#  ^ number
}

sub { $x++ };
# <- keyword.function
#     ^ variable.scalar

sub abc :lvalue { }
# <- keyword.function
#        ^ attribute
sub def :lvalue const {}
# <- keyword.function
#        ^ attribute
#               ^ attribute
sub ghi :lvalue :const { }
# <- keyword.function
#        ^ attribute
#                ^ attribute
#
sub abc :lvalue(1234) { }
#               ^^^^ string
sub with_siggy ($thing, $stuff = 1 + $thing, %) {}
#                ^ variable.scalar
#                         ^variable.scalar
#                                  ^ operator
#                                            ^ variable.hash

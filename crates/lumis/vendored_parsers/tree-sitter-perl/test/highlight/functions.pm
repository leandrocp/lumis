foo();
# <- function.call
foo(123);
# <- function.call
#   ^ number
foo(12, 34);
# <- function.call
#   ^ number
#       ^ number
$obj->meth;
# <- variable.scalar
#     ^ method.call
$obj->meth();
# <- variable.scalar
#     ^ method.call
$obj->meth(123);
# <- variable.scalar
#     ^ method.call
#          ^ number
$obj->meth(12, 34);
# <- variable.scalar
#     ^ method.call
#          ^ number
#              ^ number
Some::Module->new(1234);
# <- type
#             ^ method.call
#                 ^ number

### FUNC0OPs
# no need to test them all, just do a few
__FILE__;
# <- function.builtin
__LINE__;
# <- function.builtin
wait;
# <- function.builtin
time();
# <- function.builtin

### FUNC1OPs
defined $x;
# <- function.builtin
#       ^ variable.scalar
int($num);
# <- function.builtin
#   ^ variable.scalar
shift @arr;
# <- function.builtin
#     ^ variable.array
keys %hash;
# <- function.builtin
#    ^ variable.hash
-r "path";
# <- function.builtin
#  ^ string
-w $path;
# <- function.builtin
#  ^ variable.scalar
-x _;
# <- function.builtin

### Ambiguous function calls
croak 'thing', 'stuff';
# <- function
#     ^ string
print 'things', sum 1, 2, 3;
# <- function.builtin
#     ^ string
#               ^ function
#                   ^ number
print;
# ^ function.builtin

# block form
map { +"\L$_"  => 1 } @array;
# <- function.builtin
map { ; "\L$_" => 1 } @array;
# <- function.builtin
map { ( "\L$_" => 1 ) } @array;
# <- function.builtin
map { lc($_) => 1 } @array;
# <- function.builtin

grep { ok($_) } @array;
# <- function.builtin

# expr form
map +( lc($_) => 1 ), @array;
# <- function.builtin
map +{ lc($_) => 1 }, @array;
# <- function.builtin
map { "\L$_"   => 1 }, @array;
# <- function.builtin

grep ok($_), @array;
# <- function.builtin

map { lc($_) => 1 } 1, 2, 3;
# <- function.builtin
map +(lc($_) => 1 ), 1, 2, 3;
# <- function.builtin
map { lc($_) => 1 } (1, 2, 3);
# <- function.builtin
map +(lc($_) => 1 ), (1, 2), 3;
# <- function.builtin

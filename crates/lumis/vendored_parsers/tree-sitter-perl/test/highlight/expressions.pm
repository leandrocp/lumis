do { 123; };
# <- keyword
#    ^ number
eval "string";
# <- keyword
eval { "block" };
# <- keyword
require 123;
# <- include
#       ^ number
require;
# <- include
next;
# <- keyword
last LOOP;
# <- keyword
redo;
# <- keyword
goto LABEL;
# <- keyword
undef;
# <- keyword
undef $var;
# <- keyword
#     ^ variable.scalar
local $var;
# <- keyword
#     ^ variable.scalar
return;
# <- keyword.return
return 1, 2, 3;
# <- keyword.return
#      ^ number
1 && 2 & 3 | 4 ** 5;
#  ^ operator
#      ^ operator
#          ^ operator
#               ^ operator
\$var;
# <- operator

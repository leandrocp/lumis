#ifndef TREE_SITTER_COMMENT_CHARS_H
#define TREE_SITTER_COMMENT_CHARS_H

#include <stdbool.h>
#include <stdint.h>

#define CHAR_EOF 0
#define CHAR_NEWLINE 10
#define CHAR_CARRIAGE_RETURN 13

#define CHAR_SPACE ' '
#define CHAR_FORM_FEED '\f'
#define CHAR_TAB '\t'
#define CHAR_VERTICAL_TAB '\v'

static bool is_internal_char(int32_t c);
static bool is_newline(int32_t c);
static bool is_space(int32_t c);
static bool is_upper(int32_t c);
static bool is_digit(int32_t c);

#endif /* ifndef TREE_SITTER_COMMENT_CHARS_H */

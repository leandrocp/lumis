#include "parser.h"

#include "chars.c"
#include "tokens.h"
#include <stdbool.h>
#include <stdio.h>

/// Parse the name of the tag.
///
/// They can be of the form:
/// - TODO:
/// - TODO: text
/// - TODO(stsewd):
/// - TODO(stsewd): text
/// - TODO (stsewd): text
static bool parse_tagname(TSLexer* lexer, const bool* valid_symbols)
{
  if (!is_upper(lexer->lookahead) || !valid_symbols[T_TAGNAME]) {
    return false;
  }

  int32_t previous = lexer->lookahead;
  lexer->advance(lexer, false);

  while (is_upper(lexer->lookahead)
      || is_digit(lexer->lookahead)
      || is_internal_char(lexer->lookahead)) {
    previous = lexer->lookahead;
    lexer->advance(lexer, false);
  }
  // The tag name ends here.
  // But we keep parsing to see if it's a valid tag name.
  lexer->mark_end(lexer);

  // It can't end with an internal char.
  if (is_internal_char(previous)) {
    return false;
  }

  // For the user component this is `\s*(`.
  // We don't parse that part, we just need to be sure it ends with `:\s`.
  if ((is_space(lexer->lookahead) && !is_newline(lexer->lookahead))
      || lexer->lookahead == '(') {
    // Skip white spaces.
    while (is_space(lexer->lookahead) && !is_newline(lexer->lookahead)) {
      lexer->advance(lexer, false);
    }
    // Checking aperture.
    if (lexer->lookahead != '(') {
      return false;
    }
    lexer->advance(lexer, false);

    // Checking closure.
    int user_length = 0;
    while (lexer->lookahead != ')') {
      if (is_newline(lexer->lookahead)) {
        return false;
      }
      lexer->advance(lexer, false);
      user_length++;
    }
    if (user_length <= 0) {
      return false;
    }
    lexer->advance(lexer, false);
  }

  // It should end with `:`...
  if (lexer->lookahead != ':') {
    return false;
  }

  // ... and be followed by one space.
  lexer->advance(lexer, false);
  if (!is_space(lexer->lookahead)) {
    return false;
  }

  lexer->result_symbol = T_TAGNAME;
  return true;
}

static bool parse(TSLexer* lexer, const bool* valid_symbols)
{
  // If all valid symbols are true, tree-sitter is in correction mode.
  // We don't want to parse anything in that case.
  if (valid_symbols[T_INVALID_TOKEN]) {
    return false;
  }

  if (is_upper(lexer->lookahead) && valid_symbols[T_TAGNAME]) {
    return parse_tagname(lexer, valid_symbols);
  }

  return false;
}

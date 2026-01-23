#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 13
#define STATE_COUNT 30
#define LARGE_STATE_COUNT 4
#define SYMBOL_COUNT 27
#define ALIAS_COUNT 0
#define TOKEN_COUNT 16
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 4
#define PRODUCTION_ID_COUNT 1

enum {
  anon_sym_LT_PERCENT = 1,
  anon_sym_LT_PERCENT_EQ = 2,
  anon_sym_LT_PERCENT_PERCENT = 3,
  anon_sym_LT_PERCENT_PERCENT_EQ = 4,
  anon_sym_PERCENT_GT = 5,
  aux_sym_partial_expression_token1 = 6,
  anon_sym_do = 7,
  anon_sym_DASH_GT = 8,
  anon_sym_POUND = 9,
  anon_sym_LT_PERCENT_POUND = 10,
  anon_sym_LT_PERCENT_BANG_DASH_DASH = 11,
  aux_sym__bang_comment_token1 = 12,
  anon_sym_DASH_DASH_PERCENT_GT = 13,
  sym_text = 14,
  sym__code = 15,
  sym_fragment = 16,
  sym__node = 17,
  sym_directive = 18,
  sym_partial_expression = 19,
  sym_expression = 20,
  sym_comment = 21,
  sym__hash_comment = 22,
  sym__bang_comment = 23,
  aux_sym_fragment_repeat1 = 24,
  aux_sym_partial_expression_repeat1 = 25,
  aux_sym__bang_comment_repeat1 = 26,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_LT_PERCENT] = "<%",
  [anon_sym_LT_PERCENT_EQ] = "<%=",
  [anon_sym_LT_PERCENT_PERCENT] = "<%%",
  [anon_sym_LT_PERCENT_PERCENT_EQ] = "<%%=",
  [anon_sym_PERCENT_GT] = "%>",
  [aux_sym_partial_expression_token1] = "partial_expression_token1",
  [anon_sym_do] = "do",
  [anon_sym_DASH_GT] = "->",
  [anon_sym_POUND] = "#",
  [anon_sym_LT_PERCENT_POUND] = "<%#",
  [anon_sym_LT_PERCENT_BANG_DASH_DASH] = "<%!--",
  [aux_sym__bang_comment_token1] = "_bang_comment_token1",
  [anon_sym_DASH_DASH_PERCENT_GT] = "--%>",
  [sym_text] = "text",
  [sym__code] = "_code",
  [sym_fragment] = "fragment",
  [sym__node] = "_node",
  [sym_directive] = "directive",
  [sym_partial_expression] = "partial_expression",
  [sym_expression] = "expression",
  [sym_comment] = "comment",
  [sym__hash_comment] = "_hash_comment",
  [sym__bang_comment] = "_bang_comment",
  [aux_sym_fragment_repeat1] = "fragment_repeat1",
  [aux_sym_partial_expression_repeat1] = "partial_expression_repeat1",
  [aux_sym__bang_comment_repeat1] = "_bang_comment_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_LT_PERCENT] = anon_sym_LT_PERCENT,
  [anon_sym_LT_PERCENT_EQ] = anon_sym_LT_PERCENT_EQ,
  [anon_sym_LT_PERCENT_PERCENT] = anon_sym_LT_PERCENT_PERCENT,
  [anon_sym_LT_PERCENT_PERCENT_EQ] = anon_sym_LT_PERCENT_PERCENT_EQ,
  [anon_sym_PERCENT_GT] = anon_sym_PERCENT_GT,
  [aux_sym_partial_expression_token1] = aux_sym_partial_expression_token1,
  [anon_sym_do] = anon_sym_do,
  [anon_sym_DASH_GT] = anon_sym_DASH_GT,
  [anon_sym_POUND] = anon_sym_POUND,
  [anon_sym_LT_PERCENT_POUND] = anon_sym_LT_PERCENT_POUND,
  [anon_sym_LT_PERCENT_BANG_DASH_DASH] = anon_sym_LT_PERCENT_BANG_DASH_DASH,
  [aux_sym__bang_comment_token1] = aux_sym__bang_comment_token1,
  [anon_sym_DASH_DASH_PERCENT_GT] = anon_sym_DASH_DASH_PERCENT_GT,
  [sym_text] = sym_text,
  [sym__code] = sym__code,
  [sym_fragment] = sym_fragment,
  [sym__node] = sym__node,
  [sym_directive] = sym_directive,
  [sym_partial_expression] = sym_partial_expression,
  [sym_expression] = sym_expression,
  [sym_comment] = sym_comment,
  [sym__hash_comment] = sym__hash_comment,
  [sym__bang_comment] = sym__bang_comment,
  [aux_sym_fragment_repeat1] = aux_sym_fragment_repeat1,
  [aux_sym_partial_expression_repeat1] = aux_sym_partial_expression_repeat1,
  [aux_sym__bang_comment_repeat1] = aux_sym__bang_comment_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_LT_PERCENT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LT_PERCENT_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LT_PERCENT_PERCENT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LT_PERCENT_PERCENT_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PERCENT_GT] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_partial_expression_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_do] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH_GT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_POUND] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LT_PERCENT_POUND] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LT_PERCENT_BANG_DASH_DASH] = {
    .visible = true,
    .named = false,
  },
  [aux_sym__bang_comment_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_DASH_DASH_PERCENT_GT] = {
    .visible = true,
    .named = false,
  },
  [sym_text] = {
    .visible = true,
    .named = true,
  },
  [sym__code] = {
    .visible = false,
    .named = true,
  },
  [sym_fragment] = {
    .visible = true,
    .named = true,
  },
  [sym__node] = {
    .visible = false,
    .named = true,
  },
  [sym_directive] = {
    .visible = true,
    .named = true,
  },
  [sym_partial_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_comment] = {
    .visible = true,
    .named = true,
  },
  [sym__hash_comment] = {
    .visible = false,
    .named = true,
  },
  [sym__bang_comment] = {
    .visible = false,
    .named = true,
  },
  [aux_sym_fragment_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_partial_expression_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym__bang_comment_repeat1] = {
    .visible = false,
    .named = false,
  },
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(16);
      if (lookahead == '#') ADVANCE(28);
      if (lookahead == '%') ADVANCE(10);
      if (lookahead == '-') ADVANCE(8);
      if (lookahead == '<') ADVANCE(1);
      if (lookahead == 'd') ADVANCE(14);
      if (lookahead == 'e') ADVANCE(13);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      END_STATE();
    case 1:
      if (lookahead == '%') ADVANCE(17);
      END_STATE();
    case 2:
      if (lookahead == '%') ADVANCE(41);
      if (lookahead == '-') ADVANCE(42);
      if (lookahead == 'd') ADVANCE(45);
      if (lookahead == 'e') ADVANCE(44);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(38);
      if (lookahead != 0) ADVANCE(46);
      END_STATE();
    case 3:
      if (lookahead == '%') ADVANCE(41);
      if (lookahead == '-') ADVANCE(42);
      if (lookahead == 'd') ADVANCE(45);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(39);
      if (lookahead != 0) ADVANCE(46);
      END_STATE();
    case 4:
      if (lookahead == '%') ADVANCE(41);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(40);
      if (lookahead != 0) ADVANCE(46);
      END_STATE();
    case 5:
      if (lookahead == '%') ADVANCE(11);
      END_STATE();
    case 6:
      if (lookahead == '-') ADVANCE(30);
      END_STATE();
    case 7:
      if (lookahead == '-') ADVANCE(32);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(31);
      if (lookahead != 0) ADVANCE(33);
      END_STATE();
    case 8:
      if (lookahead == '-') ADVANCE(5);
      if (lookahead == '>') ADVANCE(26);
      END_STATE();
    case 9:
      if (lookahead == '-') ADVANCE(6);
      END_STATE();
    case 10:
      if (lookahead == '>') ADVANCE(21);
      END_STATE();
    case 11:
      if (lookahead == '>') ADVANCE(34);
      END_STATE();
    case 12:
      if (lookahead == 'd') ADVANCE(22);
      END_STATE();
    case 13:
      if (lookahead == 'n') ADVANCE(12);
      END_STATE();
    case 14:
      if (lookahead == 'o') ADVANCE(24);
      END_STATE();
    case 15:
      if (eof) ADVANCE(16);
      if (lookahead == '<') ADVANCE(35);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(36);
      if (lookahead != 0) ADVANCE(37);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(anon_sym_LT_PERCENT);
      if (lookahead == '!') ADVANCE(9);
      if (lookahead == '#') ADVANCE(29);
      if (lookahead == '%') ADVANCE(19);
      if (lookahead == '=') ADVANCE(18);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(anon_sym_LT_PERCENT_EQ);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(anon_sym_LT_PERCENT_PERCENT);
      if (lookahead == '=') ADVANCE(20);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(anon_sym_LT_PERCENT_PERCENT_EQ);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(anon_sym_PERCENT_GT);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(aux_sym_partial_expression_token1);
      if (lookahead == ')' ||
          lookahead == ']' ||
          lookahead == '}') ADVANCE(22);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(aux_sym_partial_expression_token1);
      if (lookahead == ')' ||
          lookahead == ']' ||
          lookahead == '}') ADVANCE(23);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '%') ADVANCE(46);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(anon_sym_do);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(anon_sym_do);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '%') ADVANCE(46);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(anon_sym_DASH_GT);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(anon_sym_DASH_GT);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '%') ADVANCE(46);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(anon_sym_POUND);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(anon_sym_LT_PERCENT_POUND);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(anon_sym_LT_PERCENT_BANG_DASH_DASH);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(aux_sym__bang_comment_token1);
      if (lookahead == '-') ADVANCE(32);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(31);
      if (lookahead != 0) ADVANCE(33);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(aux_sym__bang_comment_token1);
      if (lookahead == '-') ADVANCE(5);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(aux_sym__bang_comment_token1);
      if (lookahead != 0 &&
          lookahead != '-') ADVANCE(33);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(anon_sym_DASH_DASH_PERCENT_GT);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == '%') ADVANCE(17);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == '<') ADVANCE(35);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(36);
      if (lookahead != 0) ADVANCE(37);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(sym_text);
      if (lookahead != 0 &&
          lookahead != '<') ADVANCE(37);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(sym__code);
      if (lookahead == '%') ADVANCE(41);
      if (lookahead == '-') ADVANCE(42);
      if (lookahead == 'd') ADVANCE(45);
      if (lookahead == 'e') ADVANCE(44);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(38);
      if (lookahead != 0) ADVANCE(46);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(sym__code);
      if (lookahead == '%') ADVANCE(41);
      if (lookahead == '-') ADVANCE(42);
      if (lookahead == 'd') ADVANCE(45);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(39);
      if (lookahead != 0) ADVANCE(46);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(sym__code);
      if (lookahead == '%') ADVANCE(41);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(40);
      if (lookahead != 0) ADVANCE(46);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(sym__code);
      if (lookahead == '>') ADVANCE(21);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(sym__code);
      if (lookahead == '>') ADVANCE(27);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '%') ADVANCE(46);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(sym__code);
      if (lookahead == 'd') ADVANCE(23);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '%') ADVANCE(46);
      END_STATE();
    case 44:
      ACCEPT_TOKEN(sym__code);
      if (lookahead == 'n') ADVANCE(43);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '%') ADVANCE(46);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(sym__code);
      if (lookahead == 'o') ADVANCE(25);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '%') ADVANCE(46);
      END_STATE();
    case 46:
      ACCEPT_TOKEN(sym__code);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '%') ADVANCE(46);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 15},
  [2] = {.lex_state = 15},
  [3] = {.lex_state = 15},
  [4] = {.lex_state = 2},
  [5] = {.lex_state = 15},
  [6] = {.lex_state = 15},
  [7] = {.lex_state = 15},
  [8] = {.lex_state = 15},
  [9] = {.lex_state = 15},
  [10] = {.lex_state = 15},
  [11] = {.lex_state = 15},
  [12] = {.lex_state = 3},
  [13] = {.lex_state = 3},
  [14] = {.lex_state = 4},
  [15] = {.lex_state = 4},
  [16] = {.lex_state = 7},
  [17] = {.lex_state = 4},
  [18] = {.lex_state = 4},
  [19] = {.lex_state = 4},
  [20] = {.lex_state = 7},
  [21] = {.lex_state = 7},
  [22] = {.lex_state = 4},
  [23] = {.lex_state = 4},
  [24] = {.lex_state = 4},
  [25] = {.lex_state = 4},
  [26] = {.lex_state = 0},
  [27] = {.lex_state = 0},
  [28] = {.lex_state = 0},
  [29] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_LT_PERCENT] = ACTIONS(1),
    [anon_sym_LT_PERCENT_EQ] = ACTIONS(1),
    [anon_sym_LT_PERCENT_PERCENT] = ACTIONS(1),
    [anon_sym_LT_PERCENT_PERCENT_EQ] = ACTIONS(1),
    [anon_sym_PERCENT_GT] = ACTIONS(1),
    [aux_sym_partial_expression_token1] = ACTIONS(1),
    [anon_sym_do] = ACTIONS(1),
    [anon_sym_DASH_GT] = ACTIONS(1),
    [anon_sym_POUND] = ACTIONS(1),
    [anon_sym_LT_PERCENT_POUND] = ACTIONS(1),
    [anon_sym_LT_PERCENT_BANG_DASH_DASH] = ACTIONS(1),
    [anon_sym_DASH_DASH_PERCENT_GT] = ACTIONS(1),
  },
  [1] = {
    [sym_fragment] = STATE(29),
    [sym__node] = STATE(2),
    [sym_directive] = STATE(2),
    [sym_comment] = STATE(2),
    [sym__hash_comment] = STATE(7),
    [sym__bang_comment] = STATE(7),
    [aux_sym_fragment_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(3),
    [anon_sym_LT_PERCENT] = ACTIONS(5),
    [anon_sym_LT_PERCENT_EQ] = ACTIONS(5),
    [anon_sym_LT_PERCENT_PERCENT] = ACTIONS(5),
    [anon_sym_LT_PERCENT_PERCENT_EQ] = ACTIONS(5),
    [anon_sym_LT_PERCENT_POUND] = ACTIONS(7),
    [anon_sym_LT_PERCENT_BANG_DASH_DASH] = ACTIONS(9),
    [sym_text] = ACTIONS(11),
  },
  [2] = {
    [sym__node] = STATE(3),
    [sym_directive] = STATE(3),
    [sym_comment] = STATE(3),
    [sym__hash_comment] = STATE(7),
    [sym__bang_comment] = STATE(7),
    [aux_sym_fragment_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(13),
    [anon_sym_LT_PERCENT] = ACTIONS(5),
    [anon_sym_LT_PERCENT_EQ] = ACTIONS(5),
    [anon_sym_LT_PERCENT_PERCENT] = ACTIONS(5),
    [anon_sym_LT_PERCENT_PERCENT_EQ] = ACTIONS(5),
    [anon_sym_LT_PERCENT_POUND] = ACTIONS(7),
    [anon_sym_LT_PERCENT_BANG_DASH_DASH] = ACTIONS(9),
    [sym_text] = ACTIONS(15),
  },
  [3] = {
    [sym__node] = STATE(3),
    [sym_directive] = STATE(3),
    [sym_comment] = STATE(3),
    [sym__hash_comment] = STATE(7),
    [sym__bang_comment] = STATE(7),
    [aux_sym_fragment_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(17),
    [anon_sym_LT_PERCENT] = ACTIONS(19),
    [anon_sym_LT_PERCENT_EQ] = ACTIONS(19),
    [anon_sym_LT_PERCENT_PERCENT] = ACTIONS(19),
    [anon_sym_LT_PERCENT_PERCENT_EQ] = ACTIONS(19),
    [anon_sym_LT_PERCENT_POUND] = ACTIONS(22),
    [anon_sym_LT_PERCENT_BANG_DASH_DASH] = ACTIONS(25),
    [sym_text] = ACTIONS(28),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 6,
    ACTIONS(31), 1,
      anon_sym_PERCENT_GT,
    ACTIONS(33), 1,
      aux_sym_partial_expression_token1,
    ACTIONS(37), 1,
      sym__code,
    STATE(13), 1,
      aux_sym_partial_expression_repeat1,
    ACTIONS(35), 2,
      anon_sym_do,
      anon_sym_DASH_GT,
    STATE(28), 2,
      sym_partial_expression,
      sym_expression,
  [21] = 2,
    ACTIONS(39), 1,
      ts_builtin_sym_end,
    ACTIONS(41), 7,
      anon_sym_LT_PERCENT,
      anon_sym_LT_PERCENT_EQ,
      anon_sym_LT_PERCENT_PERCENT,
      anon_sym_LT_PERCENT_PERCENT_EQ,
      anon_sym_LT_PERCENT_POUND,
      anon_sym_LT_PERCENT_BANG_DASH_DASH,
      sym_text,
  [34] = 2,
    ACTIONS(43), 1,
      ts_builtin_sym_end,
    ACTIONS(45), 7,
      anon_sym_LT_PERCENT,
      anon_sym_LT_PERCENT_EQ,
      anon_sym_LT_PERCENT_PERCENT,
      anon_sym_LT_PERCENT_PERCENT_EQ,
      anon_sym_LT_PERCENT_POUND,
      anon_sym_LT_PERCENT_BANG_DASH_DASH,
      sym_text,
  [47] = 2,
    ACTIONS(47), 1,
      ts_builtin_sym_end,
    ACTIONS(49), 7,
      anon_sym_LT_PERCENT,
      anon_sym_LT_PERCENT_EQ,
      anon_sym_LT_PERCENT_PERCENT,
      anon_sym_LT_PERCENT_PERCENT_EQ,
      anon_sym_LT_PERCENT_POUND,
      anon_sym_LT_PERCENT_BANG_DASH_DASH,
      sym_text,
  [60] = 2,
    ACTIONS(51), 1,
      ts_builtin_sym_end,
    ACTIONS(53), 7,
      anon_sym_LT_PERCENT,
      anon_sym_LT_PERCENT_EQ,
      anon_sym_LT_PERCENT_PERCENT,
      anon_sym_LT_PERCENT_PERCENT_EQ,
      anon_sym_LT_PERCENT_POUND,
      anon_sym_LT_PERCENT_BANG_DASH_DASH,
      sym_text,
  [73] = 2,
    ACTIONS(55), 1,
      ts_builtin_sym_end,
    ACTIONS(57), 7,
      anon_sym_LT_PERCENT,
      anon_sym_LT_PERCENT_EQ,
      anon_sym_LT_PERCENT_PERCENT,
      anon_sym_LT_PERCENT_PERCENT_EQ,
      anon_sym_LT_PERCENT_POUND,
      anon_sym_LT_PERCENT_BANG_DASH_DASH,
      sym_text,
  [86] = 2,
    ACTIONS(59), 1,
      ts_builtin_sym_end,
    ACTIONS(61), 7,
      anon_sym_LT_PERCENT,
      anon_sym_LT_PERCENT_EQ,
      anon_sym_LT_PERCENT_PERCENT,
      anon_sym_LT_PERCENT_PERCENT_EQ,
      anon_sym_LT_PERCENT_POUND,
      anon_sym_LT_PERCENT_BANG_DASH_DASH,
      sym_text,
  [99] = 2,
    ACTIONS(63), 1,
      ts_builtin_sym_end,
    ACTIONS(65), 7,
      anon_sym_LT_PERCENT,
      anon_sym_LT_PERCENT_EQ,
      anon_sym_LT_PERCENT_PERCENT,
      anon_sym_LT_PERCENT_PERCENT_EQ,
      anon_sym_LT_PERCENT_POUND,
      anon_sym_LT_PERCENT_BANG_DASH_DASH,
      sym_text,
  [112] = 3,
    ACTIONS(69), 1,
      sym__code,
    STATE(12), 1,
      aux_sym_partial_expression_repeat1,
    ACTIONS(67), 3,
      anon_sym_PERCENT_GT,
      anon_sym_do,
      anon_sym_DASH_GT,
  [124] = 4,
    ACTIONS(72), 1,
      anon_sym_PERCENT_GT,
    ACTIONS(76), 1,
      sym__code,
    STATE(12), 1,
      aux_sym_partial_expression_repeat1,
    ACTIONS(74), 2,
      anon_sym_do,
      anon_sym_DASH_GT,
  [138] = 3,
    ACTIONS(78), 1,
      anon_sym_PERCENT_GT,
    ACTIONS(80), 1,
      sym__code,
    STATE(25), 1,
      aux_sym_partial_expression_repeat1,
  [148] = 3,
    ACTIONS(82), 1,
      anon_sym_PERCENT_GT,
    ACTIONS(84), 1,
      sym__code,
    STATE(14), 1,
      aux_sym_partial_expression_repeat1,
  [158] = 3,
    ACTIONS(86), 1,
      aux_sym__bang_comment_token1,
    ACTIONS(88), 1,
      anon_sym_DASH_DASH_PERCENT_GT,
    STATE(21), 1,
      aux_sym__bang_comment_repeat1,
  [168] = 3,
    ACTIONS(80), 1,
      sym__code,
    ACTIONS(90), 1,
      anon_sym_PERCENT_GT,
    STATE(25), 1,
      aux_sym_partial_expression_repeat1,
  [178] = 3,
    ACTIONS(90), 1,
      anon_sym_PERCENT_GT,
    ACTIONS(92), 1,
      sym__code,
    STATE(22), 1,
      aux_sym_partial_expression_repeat1,
  [188] = 3,
    ACTIONS(94), 1,
      anon_sym_PERCENT_GT,
    ACTIONS(96), 1,
      sym__code,
    STATE(17), 1,
      aux_sym_partial_expression_repeat1,
  [198] = 3,
    ACTIONS(98), 1,
      aux_sym__bang_comment_token1,
    ACTIONS(100), 1,
      anon_sym_DASH_DASH_PERCENT_GT,
    STATE(16), 1,
      aux_sym__bang_comment_repeat1,
  [208] = 3,
    ACTIONS(102), 1,
      aux_sym__bang_comment_token1,
    ACTIONS(105), 1,
      anon_sym_DASH_DASH_PERCENT_GT,
    STATE(21), 1,
      aux_sym__bang_comment_repeat1,
  [218] = 3,
    ACTIONS(80), 1,
      sym__code,
    ACTIONS(107), 1,
      anon_sym_PERCENT_GT,
    STATE(25), 1,
      aux_sym_partial_expression_repeat1,
  [228] = 3,
    ACTIONS(107), 1,
      anon_sym_PERCENT_GT,
    ACTIONS(109), 1,
      sym__code,
    STATE(24), 1,
      aux_sym_partial_expression_repeat1,
  [238] = 3,
    ACTIONS(80), 1,
      sym__code,
    ACTIONS(111), 1,
      anon_sym_PERCENT_GT,
    STATE(25), 1,
      aux_sym_partial_expression_repeat1,
  [248] = 3,
    ACTIONS(67), 1,
      anon_sym_PERCENT_GT,
    ACTIONS(113), 1,
      sym__code,
    STATE(25), 1,
      aux_sym_partial_expression_repeat1,
  [258] = 2,
    ACTIONS(116), 1,
      anon_sym_PERCENT_GT,
    ACTIONS(118), 1,
      anon_sym_POUND,
  [265] = 2,
    ACTIONS(120), 1,
      anon_sym_PERCENT_GT,
    ACTIONS(122), 1,
      anon_sym_POUND,
  [272] = 1,
    ACTIONS(124), 1,
      anon_sym_PERCENT_GT,
  [276] = 1,
    ACTIONS(126), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(4)] = 0,
  [SMALL_STATE(5)] = 21,
  [SMALL_STATE(6)] = 34,
  [SMALL_STATE(7)] = 47,
  [SMALL_STATE(8)] = 60,
  [SMALL_STATE(9)] = 73,
  [SMALL_STATE(10)] = 86,
  [SMALL_STATE(11)] = 99,
  [SMALL_STATE(12)] = 112,
  [SMALL_STATE(13)] = 124,
  [SMALL_STATE(14)] = 138,
  [SMALL_STATE(15)] = 148,
  [SMALL_STATE(16)] = 158,
  [SMALL_STATE(17)] = 168,
  [SMALL_STATE(18)] = 178,
  [SMALL_STATE(19)] = 188,
  [SMALL_STATE(20)] = 198,
  [SMALL_STATE(21)] = 208,
  [SMALL_STATE(22)] = 218,
  [SMALL_STATE(23)] = 228,
  [SMALL_STATE(24)] = 238,
  [SMALL_STATE(25)] = 248,
  [SMALL_STATE(26)] = 258,
  [SMALL_STATE(27)] = 265,
  [SMALL_STATE(28)] = 272,
  [SMALL_STATE(29)] = 276,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fragment, 0),
  [5] = {.entry = {.count = 1, .reusable = false}}, SHIFT(4),
  [7] = {.entry = {.count = 1, .reusable = false}}, SHIFT(15),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(20),
  [11] = {.entry = {.count = 1, .reusable = false}}, SHIFT(2),
  [13] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fragment, 1),
  [15] = {.entry = {.count = 1, .reusable = false}}, SHIFT(3),
  [17] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_fragment_repeat1, 2),
  [19] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_fragment_repeat1, 2), SHIFT_REPEAT(4),
  [22] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_fragment_repeat1, 2), SHIFT_REPEAT(15),
  [25] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_fragment_repeat1, 2), SHIFT_REPEAT(20),
  [28] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_fragment_repeat1, 2), SHIFT_REPEAT(3),
  [31] = {.entry = {.count = 1, .reusable = false}}, SHIFT(8),
  [33] = {.entry = {.count = 1, .reusable = false}}, SHIFT(19),
  [35] = {.entry = {.count = 1, .reusable = false}}, SHIFT(26),
  [37] = {.entry = {.count = 1, .reusable = false}}, SHIFT(13),
  [39] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__bang_comment, 3),
  [41] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__bang_comment, 3),
  [43] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__hash_comment, 3),
  [45] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__hash_comment, 3),
  [47] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_comment, 1),
  [49] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_comment, 1),
  [51] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_directive, 2),
  [53] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_directive, 2),
  [55] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_directive, 3),
  [57] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_directive, 3),
  [59] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__bang_comment, 2),
  [61] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__bang_comment, 2),
  [63] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__hash_comment, 2),
  [65] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__hash_comment, 2),
  [67] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_partial_expression_repeat1, 2),
  [69] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_partial_expression_repeat1, 2), SHIFT_REPEAT(12),
  [72] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_expression, 1),
  [74] = {.entry = {.count = 1, .reusable = false}}, SHIFT(27),
  [76] = {.entry = {.count = 1, .reusable = false}}, SHIFT(12),
  [78] = {.entry = {.count = 1, .reusable = false}}, SHIFT(6),
  [80] = {.entry = {.count = 1, .reusable = false}}, SHIFT(25),
  [82] = {.entry = {.count = 1, .reusable = false}}, SHIFT(11),
  [84] = {.entry = {.count = 1, .reusable = false}}, SHIFT(14),
  [86] = {.entry = {.count = 1, .reusable = false}}, SHIFT(21),
  [88] = {.entry = {.count = 1, .reusable = false}}, SHIFT(5),
  [90] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_partial_expression, 2),
  [92] = {.entry = {.count = 1, .reusable = false}}, SHIFT(22),
  [94] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_partial_expression, 1),
  [96] = {.entry = {.count = 1, .reusable = false}}, SHIFT(17),
  [98] = {.entry = {.count = 1, .reusable = false}}, SHIFT(16),
  [100] = {.entry = {.count = 1, .reusable = false}}, SHIFT(10),
  [102] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym__bang_comment_repeat1, 2), SHIFT_REPEAT(21),
  [105] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym__bang_comment_repeat1, 2),
  [107] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_partial_expression, 3),
  [109] = {.entry = {.count = 1, .reusable = false}}, SHIFT(24),
  [111] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_partial_expression, 4),
  [113] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_partial_expression_repeat1, 2), SHIFT_REPEAT(25),
  [116] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_partial_expression, 1),
  [118] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [120] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_partial_expression, 2),
  [122] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [124] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [126] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_eex(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif

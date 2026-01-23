#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 13
#define STATE_COUNT 23
#define LARGE_STATE_COUNT 4
#define SYMBOL_COUNT 15
#define ALIAS_COUNT 0
#define TOKEN_COUNT 6
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 3
#define PRODUCTION_ID_COUNT 1

enum {
  aux_sym_source_token1 = 1,
  aux_sym__default_prompt_line_token1 = 2,
  aux_sym__cont_prompt_line_token1 = 3,
  aux_sym_expression_token1 = 4,
  aux_sym_result_token1 = 5,
  sym_source = 6,
  sym_evaluation_block = 7,
  sym__default_prompt_line = 8,
  sym__cont_prompt_line = 9,
  sym_expression = 10,
  sym_result = 11,
  aux_sym_source_repeat1 = 12,
  aux_sym_evaluation_block_repeat1 = 13,
  aux_sym_result_repeat1 = 14,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [aux_sym_source_token1] = "source_token1",
  [aux_sym__default_prompt_line_token1] = "prompt",
  [aux_sym__cont_prompt_line_token1] = "prompt",
  [aux_sym_expression_token1] = "expression_token1",
  [aux_sym_result_token1] = "result_token1",
  [sym_source] = "source",
  [sym_evaluation_block] = "evaluation_block",
  [sym__default_prompt_line] = "prompt_line",
  [sym__cont_prompt_line] = "prompt_line",
  [sym_expression] = "expression",
  [sym_result] = "result",
  [aux_sym_source_repeat1] = "source_repeat1",
  [aux_sym_evaluation_block_repeat1] = "evaluation_block_repeat1",
  [aux_sym_result_repeat1] = "result_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [aux_sym_source_token1] = aux_sym_source_token1,
  [aux_sym__default_prompt_line_token1] = aux_sym__default_prompt_line_token1,
  [aux_sym__cont_prompt_line_token1] = aux_sym__default_prompt_line_token1,
  [aux_sym_expression_token1] = aux_sym_expression_token1,
  [aux_sym_result_token1] = aux_sym_result_token1,
  [sym_source] = sym_source,
  [sym_evaluation_block] = sym_evaluation_block,
  [sym__default_prompt_line] = sym__default_prompt_line,
  [sym__cont_prompt_line] = sym__default_prompt_line,
  [sym_expression] = sym_expression,
  [sym_result] = sym_result,
  [aux_sym_source_repeat1] = aux_sym_source_repeat1,
  [aux_sym_evaluation_block_repeat1] = aux_sym_evaluation_block_repeat1,
  [aux_sym_result_repeat1] = aux_sym_result_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [aux_sym_source_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym__default_prompt_line_token1] = {
    .visible = true,
    .named = true,
  },
  [aux_sym__cont_prompt_line_token1] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_expression_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_result_token1] = {
    .visible = false,
    .named = false,
  },
  [sym_source] = {
    .visible = true,
    .named = true,
  },
  [sym_evaluation_block] = {
    .visible = true,
    .named = true,
  },
  [sym__default_prompt_line] = {
    .visible = true,
    .named = true,
  },
  [sym__cont_prompt_line] = {
    .visible = true,
    .named = true,
  },
  [sym_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_result] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_source_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_evaluation_block_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_result_repeat1] = {
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
      if (eof) ADVANCE(14);
      if (lookahead == '\n') ADVANCE(15);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '.') ADVANCE(28);
      if (lookahead == 'i') ADVANCE(31);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(26);
      if (lookahead != 0) ADVANCE(33);
      END_STATE();
    case 1:
      if (lookahead == '\n') ADVANCE(15);
      END_STATE();
    case 2:
      if (lookahead == '(') ADVANCE(10);
      if (lookahead == '>') ADVANCE(16);
      END_STATE();
    case 3:
      if (lookahead == ')') ADVANCE(5);
      if (lookahead != 0) ADVANCE(3);
      END_STATE();
    case 4:
      if (lookahead == ')') ADVANCE(6);
      if (lookahead != 0) ADVANCE(4);
      END_STATE();
    case 5:
      if (lookahead == '>') ADVANCE(18);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(5);
      END_STATE();
    case 6:
      if (lookahead == '>') ADVANCE(16);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(6);
      END_STATE();
    case 7:
      if (lookahead == 'e') ADVANCE(8);
      END_STATE();
    case 8:
      if (lookahead == 'x') ADVANCE(2);
      END_STATE();
    case 9:
      if (lookahead == '\t' ||
          lookahead == 11 ||
          lookahead == '\f' ||
          lookahead == ' ') ADVANCE(48);
      if (lookahead != 0 &&
          (lookahead < '\n' || '\r' < lookahead)) ADVANCE(49);
      END_STATE();
    case 10:
      if (lookahead != 0 &&
          lookahead != ')') ADVANCE(4);
      END_STATE();
    case 11:
      if (eof) ADVANCE(14);
      if (lookahead == '\n') ADVANCE(15);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '.') ADVANCE(43);
      if (lookahead == 'i') ADVANCE(46);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(34);
      if (lookahead != 0) ADVANCE(49);
      END_STATE();
    case 12:
      if (eof) ADVANCE(14);
      if (lookahead == '\n') ADVANCE(15);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == 'i') ADVANCE(7);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') SKIP(12)
      END_STATE();
    case 13:
      if (eof) ADVANCE(14);
      if (lookahead == '\n') ADVANCE(15);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == 'i') ADVANCE(46);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(35);
      if (lookahead != 0) ADVANCE(49);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(aux_sym_source_token1);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(aux_sym__default_prompt_line_token1);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(aux_sym__default_prompt_line_token1);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(33);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(aux_sym__cont_prompt_line_token1);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(aux_sym__cont_prompt_line_token1);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(33);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(aux_sym_expression_token1);
      if (lookahead == '(') ADVANCE(22);
      if (lookahead == '>') ADVANCE(19);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(33);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(aux_sym_expression_token1);
      if (lookahead == '(') ADVANCE(23);
      if (lookahead == '>') ADVANCE(17);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(33);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(aux_sym_expression_token1);
      if (lookahead == ')') ADVANCE(33);
      if (lookahead == '\n' ||
          lookahead == '\r') ADVANCE(3);
      if (lookahead != 0) ADVANCE(24);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(aux_sym_expression_token1);
      if (lookahead == ')') ADVANCE(33);
      if (lookahead == '\n' ||
          lookahead == '\r') ADVANCE(4);
      if (lookahead != 0) ADVANCE(25);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(aux_sym_expression_token1);
      if (lookahead == ')') ADVANCE(29);
      if (lookahead == '\n' ||
          lookahead == '\r') ADVANCE(3);
      if (lookahead != 0) ADVANCE(24);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(aux_sym_expression_token1);
      if (lookahead == ')') ADVANCE(30);
      if (lookahead == '\n' ||
          lookahead == '\r') ADVANCE(4);
      if (lookahead != 0) ADVANCE(25);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(aux_sym_expression_token1);
      if (lookahead == '.') ADVANCE(28);
      if (lookahead == 'i') ADVANCE(31);
      if (lookahead == '\t' ||
          lookahead == 11 ||
          lookahead == '\f' ||
          lookahead == ' ') ADVANCE(26);
      if (lookahead != 0 &&
          (lookahead < '\n' || '\r' < lookahead)) ADVANCE(33);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(aux_sym_expression_token1);
      if (lookahead == '.') ADVANCE(20);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(33);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(aux_sym_expression_token1);
      if (lookahead == '.') ADVANCE(27);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(33);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(aux_sym_expression_token1);
      if (lookahead == '>') ADVANCE(19);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(29);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(33);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(aux_sym_expression_token1);
      if (lookahead == '>') ADVANCE(17);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(30);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(33);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(aux_sym_expression_token1);
      if (lookahead == 'e') ADVANCE(32);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(33);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(aux_sym_expression_token1);
      if (lookahead == 'x') ADVANCE(21);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(33);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(aux_sym_expression_token1);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(33);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(aux_sym_result_token1);
      if (lookahead == '\n') ADVANCE(15);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '.') ADVANCE(43);
      if (lookahead == 'i') ADVANCE(46);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(34);
      if (lookahead != 0) ADVANCE(49);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(aux_sym_result_token1);
      if (lookahead == '\n') ADVANCE(15);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == 'i') ADVANCE(46);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(35);
      if (lookahead != 0) ADVANCE(49);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(aux_sym_result_token1);
      if (lookahead == '(') ADVANCE(38);
      if (lookahead == '>') ADVANCE(18);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(49);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(aux_sym_result_token1);
      if (lookahead == '(') ADVANCE(39);
      if (lookahead == '>') ADVANCE(16);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(49);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(aux_sym_result_token1);
      if (lookahead == ')') ADVANCE(49);
      if (lookahead == '\n' ||
          lookahead == '\r') ADVANCE(3);
      if (lookahead != 0) ADVANCE(40);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(aux_sym_result_token1);
      if (lookahead == ')') ADVANCE(49);
      if (lookahead == '\n' ||
          lookahead == '\r') ADVANCE(4);
      if (lookahead != 0) ADVANCE(41);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(aux_sym_result_token1);
      if (lookahead == ')') ADVANCE(44);
      if (lookahead == '\n' ||
          lookahead == '\r') ADVANCE(3);
      if (lookahead != 0) ADVANCE(40);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(aux_sym_result_token1);
      if (lookahead == ')') ADVANCE(45);
      if (lookahead == '\n' ||
          lookahead == '\r') ADVANCE(4);
      if (lookahead != 0) ADVANCE(41);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(aux_sym_result_token1);
      if (lookahead == '.') ADVANCE(36);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(49);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(aux_sym_result_token1);
      if (lookahead == '.') ADVANCE(42);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(49);
      END_STATE();
    case 44:
      ACCEPT_TOKEN(aux_sym_result_token1);
      if (lookahead == '>') ADVANCE(18);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(44);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(49);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(aux_sym_result_token1);
      if (lookahead == '>') ADVANCE(16);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(45);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(49);
      END_STATE();
    case 46:
      ACCEPT_TOKEN(aux_sym_result_token1);
      if (lookahead == 'e') ADVANCE(47);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(49);
      END_STATE();
    case 47:
      ACCEPT_TOKEN(aux_sym_result_token1);
      if (lookahead == 'x') ADVANCE(37);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(49);
      END_STATE();
    case 48:
      ACCEPT_TOKEN(aux_sym_result_token1);
      if (lookahead == '\t' ||
          lookahead == 11 ||
          lookahead == '\f' ||
          lookahead == ' ') ADVANCE(48);
      if (lookahead != 0 &&
          (lookahead < '\n' || '\r' < lookahead)) ADVANCE(49);
      END_STATE();
    case 49:
      ACCEPT_TOKEN(aux_sym_result_token1);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(49);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 12},
  [2] = {.lex_state = 11},
  [3] = {.lex_state = 11},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 11},
  [6] = {.lex_state = 0},
  [7] = {.lex_state = 12},
  [8] = {.lex_state = 12},
  [9] = {.lex_state = 11},
  [10] = {.lex_state = 11},
  [11] = {.lex_state = 11},
  [12] = {.lex_state = 13},
  [13] = {.lex_state = 12},
  [14] = {.lex_state = 12},
  [15] = {.lex_state = 13},
  [16] = {.lex_state = 12},
  [17] = {.lex_state = 12},
  [18] = {.lex_state = 12},
  [19] = {.lex_state = 12},
  [20] = {.lex_state = 0},
  [21] = {.lex_state = 12},
  [22] = {.lex_state = 9},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [aux_sym_source_token1] = ACTIONS(1),
    [aux_sym__default_prompt_line_token1] = ACTIONS(1),
    [aux_sym__cont_prompt_line_token1] = ACTIONS(1),
    [aux_sym_expression_token1] = ACTIONS(1),
    [aux_sym_result_token1] = ACTIONS(1),
  },
  [1] = {
    [sym_source] = STATE(20),
    [sym_evaluation_block] = STATE(8),
    [sym__default_prompt_line] = STATE(2),
    [aux_sym_source_repeat1] = STATE(8),
    [ts_builtin_sym_end] = ACTIONS(3),
    [aux_sym_source_token1] = ACTIONS(5),
    [aux_sym__default_prompt_line_token1] = ACTIONS(7),
  },
  [2] = {
    [sym__cont_prompt_line] = STATE(3),
    [sym_result] = STATE(17),
    [aux_sym_evaluation_block_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(9),
    [aux_sym_source_token1] = ACTIONS(11),
    [aux_sym__default_prompt_line_token1] = ACTIONS(11),
    [aux_sym__cont_prompt_line_token1] = ACTIONS(13),
    [aux_sym_result_token1] = ACTIONS(15),
  },
  [3] = {
    [sym__cont_prompt_line] = STATE(5),
    [sym_result] = STATE(18),
    [aux_sym_evaluation_block_repeat1] = STATE(5),
    [ts_builtin_sym_end] = ACTIONS(17),
    [aux_sym_source_token1] = ACTIONS(19),
    [aux_sym__default_prompt_line_token1] = ACTIONS(19),
    [aux_sym__cont_prompt_line_token1] = ACTIONS(13),
    [aux_sym_result_token1] = ACTIONS(15),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 4,
    ACTIONS(21), 1,
      ts_builtin_sym_end,
    ACTIONS(25), 1,
      aux_sym_expression_token1,
    STATE(9), 1,
      sym_expression,
    ACTIONS(23), 4,
      aux_sym_source_token1,
      aux_sym__default_prompt_line_token1,
      aux_sym__cont_prompt_line_token1,
      aux_sym_result_token1,
  [16] = 4,
    ACTIONS(27), 1,
      ts_builtin_sym_end,
    ACTIONS(31), 1,
      aux_sym__cont_prompt_line_token1,
    STATE(5), 2,
      sym__cont_prompt_line,
      aux_sym_evaluation_block_repeat1,
    ACTIONS(29), 3,
      aux_sym_source_token1,
      aux_sym__default_prompt_line_token1,
      aux_sym_result_token1,
  [32] = 4,
    ACTIONS(25), 1,
      aux_sym_expression_token1,
    ACTIONS(34), 1,
      ts_builtin_sym_end,
    STATE(10), 1,
      sym_expression,
    ACTIONS(36), 4,
      aux_sym_source_token1,
      aux_sym__default_prompt_line_token1,
      aux_sym__cont_prompt_line_token1,
      aux_sym_result_token1,
  [48] = 5,
    ACTIONS(38), 1,
      ts_builtin_sym_end,
    ACTIONS(40), 1,
      aux_sym_source_token1,
    ACTIONS(43), 1,
      aux_sym__default_prompt_line_token1,
    STATE(2), 1,
      sym__default_prompt_line,
    STATE(7), 2,
      sym_evaluation_block,
      aux_sym_source_repeat1,
  [65] = 5,
    ACTIONS(7), 1,
      aux_sym__default_prompt_line_token1,
    ACTIONS(46), 1,
      ts_builtin_sym_end,
    ACTIONS(48), 1,
      aux_sym_source_token1,
    STATE(2), 1,
      sym__default_prompt_line,
    STATE(7), 2,
      sym_evaluation_block,
      aux_sym_source_repeat1,
  [82] = 2,
    ACTIONS(50), 1,
      ts_builtin_sym_end,
    ACTIONS(52), 4,
      aux_sym_source_token1,
      aux_sym__default_prompt_line_token1,
      aux_sym__cont_prompt_line_token1,
      aux_sym_result_token1,
  [92] = 2,
    ACTIONS(54), 1,
      ts_builtin_sym_end,
    ACTIONS(56), 4,
      aux_sym_source_token1,
      aux_sym__default_prompt_line_token1,
      aux_sym__cont_prompt_line_token1,
      aux_sym_result_token1,
  [102] = 2,
    ACTIONS(58), 1,
      ts_builtin_sym_end,
    ACTIONS(60), 4,
      aux_sym_source_token1,
      aux_sym__default_prompt_line_token1,
      aux_sym__cont_prompt_line_token1,
      aux_sym_result_token1,
  [112] = 3,
    ACTIONS(62), 1,
      ts_builtin_sym_end,
    ACTIONS(66), 1,
      aux_sym_result_token1,
    ACTIONS(64), 2,
      aux_sym_source_token1,
      aux_sym__default_prompt_line_token1,
  [123] = 3,
    ACTIONS(70), 1,
      aux_sym_source_token1,
    STATE(14), 1,
      aux_sym_result_repeat1,
    ACTIONS(68), 2,
      ts_builtin_sym_end,
      aux_sym__default_prompt_line_token1,
  [134] = 3,
    ACTIONS(72), 1,
      aux_sym_source_token1,
    STATE(16), 1,
      aux_sym_result_repeat1,
    ACTIONS(62), 2,
      ts_builtin_sym_end,
      aux_sym__default_prompt_line_token1,
  [145] = 3,
    ACTIONS(66), 1,
      aux_sym_result_token1,
    ACTIONS(74), 1,
      ts_builtin_sym_end,
    ACTIONS(76), 2,
      aux_sym_source_token1,
      aux_sym__default_prompt_line_token1,
  [156] = 3,
    ACTIONS(80), 1,
      aux_sym_source_token1,
    STATE(16), 1,
      aux_sym_result_repeat1,
    ACTIONS(78), 2,
      ts_builtin_sym_end,
      aux_sym__default_prompt_line_token1,
  [167] = 1,
    ACTIONS(17), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      aux_sym__default_prompt_line_token1,
  [173] = 1,
    ACTIONS(83), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      aux_sym__default_prompt_line_token1,
  [179] = 1,
    ACTIONS(78), 3,
      ts_builtin_sym_end,
      aux_sym_source_token1,
      aux_sym__default_prompt_line_token1,
  [185] = 1,
    ACTIONS(85), 1,
      ts_builtin_sym_end,
  [189] = 1,
    ACTIONS(87), 1,
      aux_sym_source_token1,
  [193] = 1,
    ACTIONS(89), 1,
      aux_sym_result_token1,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(4)] = 0,
  [SMALL_STATE(5)] = 16,
  [SMALL_STATE(6)] = 32,
  [SMALL_STATE(7)] = 48,
  [SMALL_STATE(8)] = 65,
  [SMALL_STATE(9)] = 82,
  [SMALL_STATE(10)] = 92,
  [SMALL_STATE(11)] = 102,
  [SMALL_STATE(12)] = 112,
  [SMALL_STATE(13)] = 123,
  [SMALL_STATE(14)] = 134,
  [SMALL_STATE(15)] = 145,
  [SMALL_STATE(16)] = 156,
  [SMALL_STATE(17)] = 167,
  [SMALL_STATE(18)] = 173,
  [SMALL_STATE(19)] = 179,
  [SMALL_STATE(20)] = 185,
  [SMALL_STATE(21)] = 189,
  [SMALL_STATE(22)] = 193,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source, 0),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [9] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_evaluation_block, 1),
  [11] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_evaluation_block, 1),
  [13] = {.entry = {.count = 1, .reusable = false}}, SHIFT(6),
  [15] = {.entry = {.count = 1, .reusable = false}}, SHIFT(13),
  [17] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_evaluation_block, 2),
  [19] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_evaluation_block, 2),
  [21] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__default_prompt_line, 1),
  [23] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__default_prompt_line, 1),
  [25] = {.entry = {.count = 1, .reusable = false}}, SHIFT(21),
  [27] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_evaluation_block_repeat1, 2),
  [29] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_evaluation_block_repeat1, 2),
  [31] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_evaluation_block_repeat1, 2), SHIFT_REPEAT(6),
  [34] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__cont_prompt_line, 1),
  [36] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__cont_prompt_line, 1),
  [38] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_repeat1, 2),
  [40] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_repeat1, 2), SHIFT_REPEAT(7),
  [43] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_repeat1, 2), SHIFT_REPEAT(4),
  [46] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source, 1),
  [48] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [50] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__default_prompt_line, 2),
  [52] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__default_prompt_line, 2),
  [54] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__cont_prompt_line, 2),
  [56] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__cont_prompt_line, 2),
  [58] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expression, 2),
  [60] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_expression, 2),
  [62] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_result, 2),
  [64] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_result, 2),
  [66] = {.entry = {.count = 1, .reusable = false}}, SHIFT(19),
  [68] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_result, 1),
  [70] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [72] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [74] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_result, 3),
  [76] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_result, 3),
  [78] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_result_repeat1, 2),
  [80] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_result_repeat1, 2), SHIFT_REPEAT(22),
  [83] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_evaluation_block, 3),
  [85] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [87] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [89] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_iex(void) {
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

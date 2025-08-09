#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 42
#define LARGE_STATE_COUNT 31
#define SYMBOL_COUNT 19
#define ALIAS_COUNT 0
#define TOKEN_COUNT 10
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 4
#define PRODUCTION_ID_COUNT 1

enum ts_symbol_identifiers {
  aux_sym_document_token1 = 1,
  anon_sym_COMMA = 2,
  sym_text = 3,
  aux_sym_number_token1 = 4,
  aux_sym_number_token2 = 5,
  aux_sym_float_token1 = 6,
  aux_sym_float_token2 = 7,
  anon_sym_true = 8,
  anon_sym_false = 9,
  sym_document = 10,
  sym_row = 11,
  sym_field = 12,
  sym_number = 13,
  sym_float = 14,
  sym_boolean = 15,
  aux_sym_document_repeat1 = 16,
  aux_sym_row_repeat1 = 17,
  aux_sym_row_repeat2 = 18,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [aux_sym_document_token1] = "document_token1",
  [anon_sym_COMMA] = ",",
  [sym_text] = "text",
  [aux_sym_number_token1] = "number_token1",
  [aux_sym_number_token2] = "number_token2",
  [aux_sym_float_token1] = "float_token1",
  [aux_sym_float_token2] = "float_token2",
  [anon_sym_true] = "true",
  [anon_sym_false] = "false",
  [sym_document] = "document",
  [sym_row] = "row",
  [sym_field] = "field",
  [sym_number] = "number",
  [sym_float] = "float",
  [sym_boolean] = "boolean",
  [aux_sym_document_repeat1] = "document_repeat1",
  [aux_sym_row_repeat1] = "row_repeat1",
  [aux_sym_row_repeat2] = "row_repeat2",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [aux_sym_document_token1] = aux_sym_document_token1,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [sym_text] = sym_text,
  [aux_sym_number_token1] = aux_sym_number_token1,
  [aux_sym_number_token2] = aux_sym_number_token2,
  [aux_sym_float_token1] = aux_sym_float_token1,
  [aux_sym_float_token2] = aux_sym_float_token2,
  [anon_sym_true] = anon_sym_true,
  [anon_sym_false] = anon_sym_false,
  [sym_document] = sym_document,
  [sym_row] = sym_row,
  [sym_field] = sym_field,
  [sym_number] = sym_number,
  [sym_float] = sym_float,
  [sym_boolean] = sym_boolean,
  [aux_sym_document_repeat1] = aux_sym_document_repeat1,
  [aux_sym_row_repeat1] = aux_sym_row_repeat1,
  [aux_sym_row_repeat2] = aux_sym_row_repeat2,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [aux_sym_document_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [sym_text] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_number_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_number_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_float_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_float_token2] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_true] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_false] = {
    .visible = true,
    .named = false,
  },
  [sym_document] = {
    .visible = true,
    .named = true,
  },
  [sym_row] = {
    .visible = true,
    .named = true,
  },
  [sym_field] = {
    .visible = true,
    .named = true,
  },
  [sym_number] = {
    .visible = true,
    .named = true,
  },
  [sym_float] = {
    .visible = true,
    .named = true,
  },
  [sym_boolean] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_document_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_row_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_row_repeat2] = {
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

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 8,
  [11] = 11,
  [12] = 3,
  [13] = 4,
  [14] = 5,
  [15] = 15,
  [16] = 7,
  [17] = 17,
  [18] = 17,
  [19] = 15,
  [20] = 11,
  [21] = 9,
  [22] = 22,
  [23] = 22,
  [24] = 24,
  [25] = 24,
  [26] = 26,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 30,
  [31] = 26,
  [32] = 27,
  [33] = 29,
  [34] = 34,
  [35] = 30,
  [36] = 28,
  [37] = 24,
  [38] = 38,
  [39] = 39,
  [40] = 40,
  [41] = 41,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(8);
      ADVANCE_MAP(
        '\n', 9,
        '\r', 9,
        '"', 1,
        ',', 10,
        '.', 4,
        '0', 19,
        'f', 2,
        't', 3,
      );
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') SKIP(0);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(20);
      if (lookahead != 0) ADVANCE(6);
      END_STATE();
    case 1:
      if (lookahead == '"') ADVANCE(11);
      if (lookahead != 0) ADVANCE(1);
      END_STATE();
    case 2:
      if (lookahead == 'a') ADVANCE(14);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '"' &&
          lookahead != ',') ADVANCE(18);
      END_STATE();
    case 3:
      if (lookahead == 'r') ADVANCE(16);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '"' &&
          lookahead != ',') ADVANCE(18);
      END_STATE();
    case 4:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(17);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '"' &&
          lookahead != ',') ADVANCE(18);
      END_STATE();
    case 5:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(21);
      END_STATE();
    case 6:
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '"' &&
          lookahead != ',') ADVANCE(18);
      END_STATE();
    case 7:
      if (eof) ADVANCE(8);
      if (lookahead == '"') ADVANCE(1);
      if (lookahead == ',') ADVANCE(10);
      if (lookahead == '.') ADVANCE(4);
      if (lookahead == '0') ADVANCE(19);
      if (lookahead == 'f') ADVANCE(2);
      if (lookahead == 't') ADVANCE(3);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(7);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(20);
      if (lookahead != 0) ADVANCE(6);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(aux_sym_document_token1);
      if (lookahead == '\n') ADVANCE(9);
      if (lookahead == '\r') ADVANCE(9);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == '"') ADVANCE(1);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == 'e') ADVANCE(24);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '"' &&
          lookahead != ',') ADVANCE(18);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == 'e') ADVANCE(25);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '"' &&
          lookahead != ',') ADVANCE(18);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == 'l') ADVANCE(15);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '"' &&
          lookahead != ',') ADVANCE(18);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == 's') ADVANCE(13);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '"' &&
          lookahead != ',') ADVANCE(18);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(sym_text);
      if (lookahead == 'u') ADVANCE(12);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '"' &&
          lookahead != ',') ADVANCE(18);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(sym_text);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(17);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '"' &&
          lookahead != ',') ADVANCE(18);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(sym_text);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '"' &&
          lookahead != ',') ADVANCE(18);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(aux_sym_number_token1);
      if (lookahead == '.') ADVANCE(23);
      if (lookahead == 'X' ||
          lookahead == 'x') ADVANCE(5);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(20);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(aux_sym_number_token1);
      if (lookahead == '.') ADVANCE(23);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(20);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(aux_sym_number_token2);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(21);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(aux_sym_float_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(22);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(aux_sym_float_token2);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(22);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(anon_sym_true);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '"' &&
          lookahead != ',') ADVANCE(18);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(anon_sym_false);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '"' &&
          lookahead != ',') ADVANCE(18);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 7},
  [2] = {.lex_state = 7},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 7},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 0},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 0},
  [18] = {.lex_state = 0},
  [19] = {.lex_state = 0},
  [20] = {.lex_state = 0},
  [21] = {.lex_state = 0},
  [22] = {.lex_state = 7},
  [23] = {.lex_state = 7},
  [24] = {.lex_state = 0},
  [25] = {.lex_state = 0},
  [26] = {.lex_state = 0},
  [27] = {.lex_state = 0},
  [28] = {.lex_state = 0},
  [29] = {.lex_state = 0},
  [30] = {.lex_state = 0},
  [31] = {.lex_state = 0},
  [32] = {.lex_state = 0},
  [33] = {.lex_state = 0},
  [34] = {.lex_state = 7},
  [35] = {.lex_state = 0},
  [36] = {.lex_state = 0},
  [37] = {.lex_state = 7},
  [38] = {.lex_state = 0},
  [39] = {.lex_state = 0},
  [40] = {.lex_state = 0},
  [41] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [aux_sym_document_token1] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [sym_text] = ACTIONS(1),
    [aux_sym_number_token1] = ACTIONS(1),
    [aux_sym_number_token2] = ACTIONS(1),
    [aux_sym_float_token1] = ACTIONS(1),
    [aux_sym_float_token2] = ACTIONS(1),
    [anon_sym_true] = ACTIONS(1),
    [anon_sym_false] = ACTIONS(1),
  },
  [1] = {
    [sym_document] = STATE(41),
    [sym_row] = STATE(38),
    [sym_field] = STATE(8),
    [sym_number] = STATE(30),
    [sym_float] = STATE(30),
    [sym_boolean] = STATE(30),
    [aux_sym_document_repeat1] = STATE(2),
    [aux_sym_row_repeat1] = STATE(9),
    [ts_builtin_sym_end] = ACTIONS(3),
    [anon_sym_COMMA] = ACTIONS(5),
    [sym_text] = ACTIONS(7),
    [aux_sym_number_token1] = ACTIONS(9),
    [aux_sym_number_token2] = ACTIONS(11),
    [aux_sym_float_token1] = ACTIONS(13),
    [aux_sym_float_token2] = ACTIONS(13),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
  },
  [2] = {
    [sym_row] = STATE(39),
    [sym_field] = STATE(8),
    [sym_number] = STATE(30),
    [sym_float] = STATE(30),
    [sym_boolean] = STATE(30),
    [aux_sym_document_repeat1] = STATE(6),
    [aux_sym_row_repeat1] = STATE(9),
    [ts_builtin_sym_end] = ACTIONS(17),
    [anon_sym_COMMA] = ACTIONS(5),
    [sym_text] = ACTIONS(7),
    [aux_sym_number_token1] = ACTIONS(9),
    [aux_sym_number_token2] = ACTIONS(11),
    [aux_sym_float_token1] = ACTIONS(13),
    [aux_sym_float_token2] = ACTIONS(13),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
  },
  [3] = {
    [sym_field] = STATE(4),
    [sym_number] = STATE(30),
    [sym_float] = STATE(30),
    [sym_boolean] = STATE(30),
    [aux_sym_row_repeat1] = STATE(17),
    [aux_sym_row_repeat2] = STATE(4),
    [ts_builtin_sym_end] = ACTIONS(19),
    [aux_sym_document_token1] = ACTIONS(19),
    [anon_sym_COMMA] = ACTIONS(21),
    [sym_text] = ACTIONS(7),
    [aux_sym_number_token1] = ACTIONS(9),
    [aux_sym_number_token2] = ACTIONS(9),
    [aux_sym_float_token1] = ACTIONS(13),
    [aux_sym_float_token2] = ACTIONS(13),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
  },
  [4] = {
    [sym_field] = STATE(4),
    [sym_number] = STATE(30),
    [sym_float] = STATE(30),
    [sym_boolean] = STATE(30),
    [aux_sym_row_repeat1] = STATE(23),
    [aux_sym_row_repeat2] = STATE(4),
    [ts_builtin_sym_end] = ACTIONS(23),
    [aux_sym_document_token1] = ACTIONS(23),
    [anon_sym_COMMA] = ACTIONS(25),
    [sym_text] = ACTIONS(28),
    [aux_sym_number_token1] = ACTIONS(31),
    [aux_sym_number_token2] = ACTIONS(31),
    [aux_sym_float_token1] = ACTIONS(34),
    [aux_sym_float_token2] = ACTIONS(34),
    [anon_sym_true] = ACTIONS(37),
    [anon_sym_false] = ACTIONS(37),
  },
  [5] = {
    [sym_field] = STATE(3),
    [sym_number] = STATE(30),
    [sym_float] = STATE(30),
    [sym_boolean] = STATE(30),
    [aux_sym_row_repeat1] = STATE(15),
    [aux_sym_row_repeat2] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(40),
    [aux_sym_document_token1] = ACTIONS(40),
    [anon_sym_COMMA] = ACTIONS(42),
    [sym_text] = ACTIONS(7),
    [aux_sym_number_token1] = ACTIONS(9),
    [aux_sym_number_token2] = ACTIONS(9),
    [aux_sym_float_token1] = ACTIONS(13),
    [aux_sym_float_token2] = ACTIONS(13),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
  },
  [6] = {
    [sym_row] = STATE(40),
    [sym_field] = STATE(10),
    [sym_number] = STATE(35),
    [sym_float] = STATE(35),
    [sym_boolean] = STATE(35),
    [aux_sym_document_repeat1] = STATE(6),
    [aux_sym_row_repeat1] = STATE(21),
    [ts_builtin_sym_end] = ACTIONS(44),
    [anon_sym_COMMA] = ACTIONS(46),
    [sym_text] = ACTIONS(49),
    [aux_sym_number_token1] = ACTIONS(52),
    [aux_sym_number_token2] = ACTIONS(55),
    [aux_sym_float_token1] = ACTIONS(58),
    [aux_sym_float_token2] = ACTIONS(58),
    [anon_sym_true] = ACTIONS(61),
    [anon_sym_false] = ACTIONS(61),
  },
  [7] = {
    [sym_field] = STATE(4),
    [sym_number] = STATE(30),
    [sym_float] = STATE(30),
    [sym_boolean] = STATE(30),
    [aux_sym_row_repeat1] = STATE(15),
    [aux_sym_row_repeat2] = STATE(4),
    [ts_builtin_sym_end] = ACTIONS(40),
    [aux_sym_document_token1] = ACTIONS(40),
    [anon_sym_COMMA] = ACTIONS(42),
    [sym_text] = ACTIONS(7),
    [aux_sym_number_token1] = ACTIONS(9),
    [aux_sym_number_token2] = ACTIONS(9),
    [aux_sym_float_token1] = ACTIONS(13),
    [aux_sym_float_token2] = ACTIONS(13),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
  },
  [8] = {
    [sym_field] = STATE(7),
    [sym_number] = STATE(30),
    [sym_float] = STATE(30),
    [sym_boolean] = STATE(30),
    [aux_sym_row_repeat1] = STATE(11),
    [aux_sym_row_repeat2] = STATE(7),
    [ts_builtin_sym_end] = ACTIONS(64),
    [aux_sym_document_token1] = ACTIONS(64),
    [anon_sym_COMMA] = ACTIONS(66),
    [sym_text] = ACTIONS(7),
    [aux_sym_number_token1] = ACTIONS(9),
    [aux_sym_number_token2] = ACTIONS(9),
    [aux_sym_float_token1] = ACTIONS(13),
    [aux_sym_float_token2] = ACTIONS(13),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
  },
  [9] = {
    [sym_field] = STATE(5),
    [sym_number] = STATE(30),
    [sym_float] = STATE(30),
    [sym_boolean] = STATE(30),
    [aux_sym_row_repeat1] = STATE(24),
    [ts_builtin_sym_end] = ACTIONS(64),
    [aux_sym_document_token1] = ACTIONS(64),
    [anon_sym_COMMA] = ACTIONS(68),
    [sym_text] = ACTIONS(7),
    [aux_sym_number_token1] = ACTIONS(9),
    [aux_sym_number_token2] = ACTIONS(9),
    [aux_sym_float_token1] = ACTIONS(13),
    [aux_sym_float_token2] = ACTIONS(13),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
  },
  [10] = {
    [sym_field] = STATE(16),
    [sym_number] = STATE(35),
    [sym_float] = STATE(35),
    [sym_boolean] = STATE(35),
    [aux_sym_row_repeat1] = STATE(20),
    [aux_sym_row_repeat2] = STATE(16),
    [aux_sym_document_token1] = ACTIONS(64),
    [anon_sym_COMMA] = ACTIONS(70),
    [sym_text] = ACTIONS(72),
    [aux_sym_number_token1] = ACTIONS(74),
    [aux_sym_number_token2] = ACTIONS(74),
    [aux_sym_float_token1] = ACTIONS(76),
    [aux_sym_float_token2] = ACTIONS(76),
    [anon_sym_true] = ACTIONS(78),
    [anon_sym_false] = ACTIONS(78),
  },
  [11] = {
    [sym_field] = STATE(28),
    [sym_number] = STATE(30),
    [sym_float] = STATE(30),
    [sym_boolean] = STATE(30),
    [aux_sym_row_repeat1] = STATE(24),
    [ts_builtin_sym_end] = ACTIONS(40),
    [aux_sym_document_token1] = ACTIONS(40),
    [anon_sym_COMMA] = ACTIONS(68),
    [sym_text] = ACTIONS(7),
    [aux_sym_number_token1] = ACTIONS(9),
    [aux_sym_number_token2] = ACTIONS(9),
    [aux_sym_float_token1] = ACTIONS(13),
    [aux_sym_float_token2] = ACTIONS(13),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
  },
  [12] = {
    [sym_field] = STATE(13),
    [sym_number] = STATE(35),
    [sym_float] = STATE(35),
    [sym_boolean] = STATE(35),
    [aux_sym_row_repeat1] = STATE(18),
    [aux_sym_row_repeat2] = STATE(13),
    [aux_sym_document_token1] = ACTIONS(19),
    [anon_sym_COMMA] = ACTIONS(80),
    [sym_text] = ACTIONS(72),
    [aux_sym_number_token1] = ACTIONS(74),
    [aux_sym_number_token2] = ACTIONS(74),
    [aux_sym_float_token1] = ACTIONS(76),
    [aux_sym_float_token2] = ACTIONS(76),
    [anon_sym_true] = ACTIONS(78),
    [anon_sym_false] = ACTIONS(78),
  },
  [13] = {
    [sym_field] = STATE(13),
    [sym_number] = STATE(35),
    [sym_float] = STATE(35),
    [sym_boolean] = STATE(35),
    [aux_sym_row_repeat1] = STATE(22),
    [aux_sym_row_repeat2] = STATE(13),
    [aux_sym_document_token1] = ACTIONS(23),
    [anon_sym_COMMA] = ACTIONS(82),
    [sym_text] = ACTIONS(85),
    [aux_sym_number_token1] = ACTIONS(88),
    [aux_sym_number_token2] = ACTIONS(88),
    [aux_sym_float_token1] = ACTIONS(91),
    [aux_sym_float_token2] = ACTIONS(91),
    [anon_sym_true] = ACTIONS(94),
    [anon_sym_false] = ACTIONS(94),
  },
  [14] = {
    [sym_field] = STATE(12),
    [sym_number] = STATE(35),
    [sym_float] = STATE(35),
    [sym_boolean] = STATE(35),
    [aux_sym_row_repeat1] = STATE(19),
    [aux_sym_row_repeat2] = STATE(12),
    [aux_sym_document_token1] = ACTIONS(40),
    [anon_sym_COMMA] = ACTIONS(97),
    [sym_text] = ACTIONS(72),
    [aux_sym_number_token1] = ACTIONS(74),
    [aux_sym_number_token2] = ACTIONS(74),
    [aux_sym_float_token1] = ACTIONS(76),
    [aux_sym_float_token2] = ACTIONS(76),
    [anon_sym_true] = ACTIONS(78),
    [anon_sym_false] = ACTIONS(78),
  },
  [15] = {
    [sym_field] = STATE(28),
    [sym_number] = STATE(30),
    [sym_float] = STATE(30),
    [sym_boolean] = STATE(30),
    [aux_sym_row_repeat1] = STATE(24),
    [ts_builtin_sym_end] = ACTIONS(19),
    [aux_sym_document_token1] = ACTIONS(19),
    [anon_sym_COMMA] = ACTIONS(68),
    [sym_text] = ACTIONS(7),
    [aux_sym_number_token1] = ACTIONS(9),
    [aux_sym_number_token2] = ACTIONS(9),
    [aux_sym_float_token1] = ACTIONS(13),
    [aux_sym_float_token2] = ACTIONS(13),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
  },
  [16] = {
    [sym_field] = STATE(13),
    [sym_number] = STATE(35),
    [sym_float] = STATE(35),
    [sym_boolean] = STATE(35),
    [aux_sym_row_repeat1] = STATE(19),
    [aux_sym_row_repeat2] = STATE(13),
    [aux_sym_document_token1] = ACTIONS(40),
    [anon_sym_COMMA] = ACTIONS(97),
    [sym_text] = ACTIONS(72),
    [aux_sym_number_token1] = ACTIONS(74),
    [aux_sym_number_token2] = ACTIONS(74),
    [aux_sym_float_token1] = ACTIONS(76),
    [aux_sym_float_token2] = ACTIONS(76),
    [anon_sym_true] = ACTIONS(78),
    [anon_sym_false] = ACTIONS(78),
  },
  [17] = {
    [sym_field] = STATE(28),
    [sym_number] = STATE(30),
    [sym_float] = STATE(30),
    [sym_boolean] = STATE(30),
    [aux_sym_row_repeat1] = STATE(24),
    [ts_builtin_sym_end] = ACTIONS(99),
    [aux_sym_document_token1] = ACTIONS(99),
    [anon_sym_COMMA] = ACTIONS(68),
    [sym_text] = ACTIONS(7),
    [aux_sym_number_token1] = ACTIONS(9),
    [aux_sym_number_token2] = ACTIONS(9),
    [aux_sym_float_token1] = ACTIONS(13),
    [aux_sym_float_token2] = ACTIONS(13),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
  },
  [18] = {
    [sym_field] = STATE(36),
    [sym_number] = STATE(35),
    [sym_float] = STATE(35),
    [sym_boolean] = STATE(35),
    [aux_sym_row_repeat1] = STATE(25),
    [aux_sym_document_token1] = ACTIONS(99),
    [anon_sym_COMMA] = ACTIONS(101),
    [sym_text] = ACTIONS(72),
    [aux_sym_number_token1] = ACTIONS(74),
    [aux_sym_number_token2] = ACTIONS(74),
    [aux_sym_float_token1] = ACTIONS(76),
    [aux_sym_float_token2] = ACTIONS(76),
    [anon_sym_true] = ACTIONS(78),
    [anon_sym_false] = ACTIONS(78),
  },
  [19] = {
    [sym_field] = STATE(36),
    [sym_number] = STATE(35),
    [sym_float] = STATE(35),
    [sym_boolean] = STATE(35),
    [aux_sym_row_repeat1] = STATE(25),
    [aux_sym_document_token1] = ACTIONS(19),
    [anon_sym_COMMA] = ACTIONS(101),
    [sym_text] = ACTIONS(72),
    [aux_sym_number_token1] = ACTIONS(74),
    [aux_sym_number_token2] = ACTIONS(74),
    [aux_sym_float_token1] = ACTIONS(76),
    [aux_sym_float_token2] = ACTIONS(76),
    [anon_sym_true] = ACTIONS(78),
    [anon_sym_false] = ACTIONS(78),
  },
  [20] = {
    [sym_field] = STATE(36),
    [sym_number] = STATE(35),
    [sym_float] = STATE(35),
    [sym_boolean] = STATE(35),
    [aux_sym_row_repeat1] = STATE(25),
    [aux_sym_document_token1] = ACTIONS(40),
    [anon_sym_COMMA] = ACTIONS(101),
    [sym_text] = ACTIONS(72),
    [aux_sym_number_token1] = ACTIONS(74),
    [aux_sym_number_token2] = ACTIONS(74),
    [aux_sym_float_token1] = ACTIONS(76),
    [aux_sym_float_token2] = ACTIONS(76),
    [anon_sym_true] = ACTIONS(78),
    [anon_sym_false] = ACTIONS(78),
  },
  [21] = {
    [sym_field] = STATE(14),
    [sym_number] = STATE(35),
    [sym_float] = STATE(35),
    [sym_boolean] = STATE(35),
    [aux_sym_row_repeat1] = STATE(25),
    [aux_sym_document_token1] = ACTIONS(64),
    [anon_sym_COMMA] = ACTIONS(101),
    [sym_text] = ACTIONS(72),
    [aux_sym_number_token1] = ACTIONS(74),
    [aux_sym_number_token2] = ACTIONS(74),
    [aux_sym_float_token1] = ACTIONS(76),
    [aux_sym_float_token2] = ACTIONS(76),
    [anon_sym_true] = ACTIONS(78),
    [anon_sym_false] = ACTIONS(78),
  },
  [22] = {
    [sym_field] = STATE(36),
    [sym_number] = STATE(35),
    [sym_float] = STATE(35),
    [sym_boolean] = STATE(35),
    [aux_sym_row_repeat1] = STATE(37),
    [anon_sym_COMMA] = ACTIONS(103),
    [sym_text] = ACTIONS(72),
    [aux_sym_number_token1] = ACTIONS(74),
    [aux_sym_number_token2] = ACTIONS(105),
    [aux_sym_float_token1] = ACTIONS(76),
    [aux_sym_float_token2] = ACTIONS(76),
    [anon_sym_true] = ACTIONS(78),
    [anon_sym_false] = ACTIONS(78),
  },
  [23] = {
    [sym_field] = STATE(28),
    [sym_number] = STATE(30),
    [sym_float] = STATE(30),
    [sym_boolean] = STATE(30),
    [aux_sym_row_repeat1] = STATE(37),
    [anon_sym_COMMA] = ACTIONS(103),
    [sym_text] = ACTIONS(7),
    [aux_sym_number_token1] = ACTIONS(9),
    [aux_sym_number_token2] = ACTIONS(11),
    [aux_sym_float_token1] = ACTIONS(13),
    [aux_sym_float_token2] = ACTIONS(13),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
  },
  [24] = {
    [aux_sym_row_repeat1] = STATE(24),
    [ts_builtin_sym_end] = ACTIONS(107),
    [aux_sym_document_token1] = ACTIONS(107),
    [anon_sym_COMMA] = ACTIONS(109),
    [sym_text] = ACTIONS(112),
    [aux_sym_number_token1] = ACTIONS(112),
    [aux_sym_number_token2] = ACTIONS(112),
    [aux_sym_float_token1] = ACTIONS(112),
    [aux_sym_float_token2] = ACTIONS(112),
    [anon_sym_true] = ACTIONS(112),
    [anon_sym_false] = ACTIONS(112),
  },
  [25] = {
    [aux_sym_row_repeat1] = STATE(25),
    [aux_sym_document_token1] = ACTIONS(107),
    [anon_sym_COMMA] = ACTIONS(114),
    [sym_text] = ACTIONS(112),
    [aux_sym_number_token1] = ACTIONS(112),
    [aux_sym_number_token2] = ACTIONS(112),
    [aux_sym_float_token1] = ACTIONS(112),
    [aux_sym_float_token2] = ACTIONS(112),
    [anon_sym_true] = ACTIONS(112),
    [anon_sym_false] = ACTIONS(112),
  },
  [26] = {
    [ts_builtin_sym_end] = ACTIONS(117),
    [aux_sym_document_token1] = ACTIONS(117),
    [anon_sym_COMMA] = ACTIONS(119),
    [sym_text] = ACTIONS(119),
    [aux_sym_number_token1] = ACTIONS(119),
    [aux_sym_number_token2] = ACTIONS(119),
    [aux_sym_float_token1] = ACTIONS(119),
    [aux_sym_float_token2] = ACTIONS(119),
    [anon_sym_true] = ACTIONS(119),
    [anon_sym_false] = ACTIONS(119),
  },
  [27] = {
    [ts_builtin_sym_end] = ACTIONS(121),
    [aux_sym_document_token1] = ACTIONS(121),
    [anon_sym_COMMA] = ACTIONS(123),
    [sym_text] = ACTIONS(123),
    [aux_sym_number_token1] = ACTIONS(123),
    [aux_sym_number_token2] = ACTIONS(123),
    [aux_sym_float_token1] = ACTIONS(123),
    [aux_sym_float_token2] = ACTIONS(123),
    [anon_sym_true] = ACTIONS(123),
    [anon_sym_false] = ACTIONS(123),
  },
  [28] = {
    [ts_builtin_sym_end] = ACTIONS(23),
    [aux_sym_document_token1] = ACTIONS(23),
    [anon_sym_COMMA] = ACTIONS(125),
    [sym_text] = ACTIONS(125),
    [aux_sym_number_token1] = ACTIONS(125),
    [aux_sym_number_token2] = ACTIONS(125),
    [aux_sym_float_token1] = ACTIONS(125),
    [aux_sym_float_token2] = ACTIONS(125),
    [anon_sym_true] = ACTIONS(125),
    [anon_sym_false] = ACTIONS(125),
  },
  [29] = {
    [ts_builtin_sym_end] = ACTIONS(127),
    [aux_sym_document_token1] = ACTIONS(127),
    [anon_sym_COMMA] = ACTIONS(129),
    [sym_text] = ACTIONS(129),
    [aux_sym_number_token1] = ACTIONS(129),
    [aux_sym_number_token2] = ACTIONS(129),
    [aux_sym_float_token1] = ACTIONS(129),
    [aux_sym_float_token2] = ACTIONS(129),
    [anon_sym_true] = ACTIONS(129),
    [anon_sym_false] = ACTIONS(129),
  },
  [30] = {
    [ts_builtin_sym_end] = ACTIONS(131),
    [aux_sym_document_token1] = ACTIONS(131),
    [anon_sym_COMMA] = ACTIONS(133),
    [sym_text] = ACTIONS(133),
    [aux_sym_number_token1] = ACTIONS(133),
    [aux_sym_number_token2] = ACTIONS(133),
    [aux_sym_float_token1] = ACTIONS(133),
    [aux_sym_float_token2] = ACTIONS(133),
    [anon_sym_true] = ACTIONS(133),
    [anon_sym_false] = ACTIONS(133),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 2,
    ACTIONS(117), 1,
      aux_sym_document_token1,
    ACTIONS(119), 8,
      anon_sym_COMMA,
      sym_text,
      aux_sym_number_token1,
      aux_sym_number_token2,
      aux_sym_float_token1,
      aux_sym_float_token2,
      anon_sym_true,
      anon_sym_false,
  [14] = 2,
    ACTIONS(121), 1,
      aux_sym_document_token1,
    ACTIONS(123), 8,
      anon_sym_COMMA,
      sym_text,
      aux_sym_number_token1,
      aux_sym_number_token2,
      aux_sym_float_token1,
      aux_sym_float_token2,
      anon_sym_true,
      anon_sym_false,
  [28] = 2,
    ACTIONS(127), 1,
      aux_sym_document_token1,
    ACTIONS(129), 8,
      anon_sym_COMMA,
      sym_text,
      aux_sym_number_token1,
      aux_sym_number_token2,
      aux_sym_float_token1,
      aux_sym_float_token2,
      anon_sym_true,
      anon_sym_false,
  [42] = 2,
    ACTIONS(44), 3,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      aux_sym_number_token2,
    ACTIONS(135), 6,
      sym_text,
      aux_sym_number_token1,
      aux_sym_float_token1,
      aux_sym_float_token2,
      anon_sym_true,
      anon_sym_false,
  [56] = 2,
    ACTIONS(131), 1,
      aux_sym_document_token1,
    ACTIONS(133), 8,
      anon_sym_COMMA,
      sym_text,
      aux_sym_number_token1,
      aux_sym_number_token2,
      aux_sym_float_token1,
      aux_sym_float_token2,
      anon_sym_true,
      anon_sym_false,
  [70] = 2,
    ACTIONS(23), 1,
      aux_sym_document_token1,
    ACTIONS(125), 8,
      anon_sym_COMMA,
      sym_text,
      aux_sym_number_token1,
      aux_sym_number_token2,
      aux_sym_float_token1,
      aux_sym_float_token2,
      anon_sym_true,
      anon_sym_false,
  [84] = 4,
    ACTIONS(107), 1,
      aux_sym_number_token2,
    ACTIONS(137), 1,
      anon_sym_COMMA,
    STATE(37), 1,
      aux_sym_row_repeat1,
    ACTIONS(112), 6,
      sym_text,
      aux_sym_number_token1,
      aux_sym_float_token1,
      aux_sym_float_token2,
      anon_sym_true,
      anon_sym_false,
  [102] = 2,
    ACTIONS(17), 1,
      ts_builtin_sym_end,
    ACTIONS(140), 1,
      aux_sym_document_token1,
  [109] = 2,
    ACTIONS(140), 1,
      aux_sym_document_token1,
    ACTIONS(142), 1,
      ts_builtin_sym_end,
  [116] = 1,
    ACTIONS(140), 1,
      aux_sym_document_token1,
  [120] = 1,
    ACTIONS(144), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(31)] = 0,
  [SMALL_STATE(32)] = 14,
  [SMALL_STATE(33)] = 28,
  [SMALL_STATE(34)] = 42,
  [SMALL_STATE(35)] = 56,
  [SMALL_STATE(36)] = 70,
  [SMALL_STATE(37)] = 84,
  [SMALL_STATE(38)] = 102,
  [SMALL_STATE(39)] = 109,
  [SMALL_STATE(40)] = 116,
  [SMALL_STATE(41)] = 120,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_document, 0, 0, 0),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [7] = {.entry = {.count = 1, .reusable = false}}, SHIFT(30),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(26),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [13] = {.entry = {.count = 1, .reusable = false}}, SHIFT(27),
  [15] = {.entry = {.count = 1, .reusable = false}}, SHIFT(29),
  [17] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_document, 1, 0, 0),
  [19] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_row, 3, 0, 0),
  [21] = {.entry = {.count = 1, .reusable = false}}, SHIFT(17),
  [23] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_row_repeat2, 2, 0, 0),
  [25] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_row_repeat2, 2, 0, 0), SHIFT_REPEAT(23),
  [28] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_row_repeat2, 2, 0, 0), SHIFT_REPEAT(30),
  [31] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_row_repeat2, 2, 0, 0), SHIFT_REPEAT(26),
  [34] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_row_repeat2, 2, 0, 0), SHIFT_REPEAT(27),
  [37] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_row_repeat2, 2, 0, 0), SHIFT_REPEAT(29),
  [40] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_row, 2, 0, 0),
  [42] = {.entry = {.count = 1, .reusable = false}}, SHIFT(15),
  [44] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0),
  [46] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0), SHIFT_REPEAT(21),
  [49] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0), SHIFT_REPEAT(35),
  [52] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0), SHIFT_REPEAT(31),
  [55] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0), SHIFT_REPEAT(31),
  [58] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0), SHIFT_REPEAT(32),
  [61] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0), SHIFT_REPEAT(33),
  [64] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_row, 1, 0, 0),
  [66] = {.entry = {.count = 1, .reusable = false}}, SHIFT(11),
  [68] = {.entry = {.count = 1, .reusable = false}}, SHIFT(24),
  [70] = {.entry = {.count = 1, .reusable = false}}, SHIFT(20),
  [72] = {.entry = {.count = 1, .reusable = false}}, SHIFT(35),
  [74] = {.entry = {.count = 1, .reusable = false}}, SHIFT(31),
  [76] = {.entry = {.count = 1, .reusable = false}}, SHIFT(32),
  [78] = {.entry = {.count = 1, .reusable = false}}, SHIFT(33),
  [80] = {.entry = {.count = 1, .reusable = false}}, SHIFT(18),
  [82] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_row_repeat2, 2, 0, 0), SHIFT_REPEAT(22),
  [85] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_row_repeat2, 2, 0, 0), SHIFT_REPEAT(35),
  [88] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_row_repeat2, 2, 0, 0), SHIFT_REPEAT(31),
  [91] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_row_repeat2, 2, 0, 0), SHIFT_REPEAT(32),
  [94] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_row_repeat2, 2, 0, 0), SHIFT_REPEAT(33),
  [97] = {.entry = {.count = 1, .reusable = false}}, SHIFT(19),
  [99] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_row, 4, 0, 0),
  [101] = {.entry = {.count = 1, .reusable = false}}, SHIFT(25),
  [103] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [105] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [107] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_row_repeat1, 2, 0, 0),
  [109] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_row_repeat1, 2, 0, 0), SHIFT_REPEAT(24),
  [112] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_row_repeat1, 2, 0, 0),
  [114] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_row_repeat1, 2, 0, 0), SHIFT_REPEAT(25),
  [117] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_number, 1, 0, 0),
  [119] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_number, 1, 0, 0),
  [121] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_float, 1, 0, 0),
  [123] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_float, 1, 0, 0),
  [125] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_row_repeat2, 2, 0, 0),
  [127] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_boolean, 1, 0, 0),
  [129] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_boolean, 1, 0, 0),
  [131] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_field, 1, 0, 0),
  [133] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_field, 1, 0, 0),
  [135] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0),
  [137] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_row_repeat1, 2, 0, 0), SHIFT_REPEAT(37),
  [140] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [142] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_document, 2, 0, 0),
  [144] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef TREE_SITTER_HIDE_SYMBOLS
#define TS_PUBLIC
#elif defined(_WIN32)
#define TS_PUBLIC __declspec(dllexport)
#else
#define TS_PUBLIC __attribute__((visibility("default")))
#endif

TS_PUBLIC const TSLanguage *tree_sitter_csv(void) {
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
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif

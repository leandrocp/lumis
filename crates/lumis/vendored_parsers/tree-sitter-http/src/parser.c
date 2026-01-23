#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#ifdef _MSC_VER
#pragma optimize("", off)
#elif defined(__clang__)
#pragma clang optimize off
#elif defined(__GNUC__)
#pragma GCC optimize ("O0")
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 261
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 82
#define ALIAS_COUNT 0
#define TOKEN_COUNT 40
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 10
#define MAX_ALIAS_SEQUENCE_LENGTH 9
#define PRODUCTION_ID_COUNT 49

enum ts_symbol_identifiers {
  aux_sym_WORD_CHAR_token1 = 1,
  aux_sym_PUNCTUATION_token1 = 2,
  aux_sym_WS_token1 = 3,
  aux_sym_NL_token1 = 4,
  aux_sym_LINE_TAIL_token1 = 5,
  aux_sym_COMMENT_PREFIX_token1 = 6,
  anon_sym_AT = 7,
  anon_sym_EQ = 8,
  aux_sym__var_comment_token1 = 9,
  aux_sym_request_separator_token1 = 10,
  sym_method = 11,
  aux_sym_http_version_token1 = 12,
  sym_status_code = 13,
  sym_status_text = 14,
  anon_sym_COLON = 15,
  anon_sym_LBRACE_LBRACE = 16,
  anon_sym_RBRACE_RBRACE = 17,
  anon_sym_LT = 18,
  aux_sym_pre_request_script_token1 = 19,
  anon_sym_GT = 20,
  anon_sym_LBRACE_PERCENT = 21,
  anon_sym_PERCENT_RBRACE = 22,
  aux_sym_res_redirect_token1 = 23,
  anon_sym_AT2 = 24,
  aux_sym_xml_body_token1 = 25,
  aux_sym_json_body_token1 = 26,
  aux_sym_graphql_data_token1 = 27,
  aux_sym_graphql_json_body_token1 = 28,
  anon_sym_LT2 = 29,
  anon_sym_DASH_DASH = 30,
  aux_sym_multipart_form_data_token1 = 31,
  aux_sym_multipart_form_data_token2 = 32,
  aux_sym_raw_body_token1 = 33,
  aux_sym__raw_body_token1 = 34,
  sym__not_comment = 35,
  sym_header_entity = 36,
  sym_identifier = 37,
  aux_sym_path_token1 = 38,
  aux_sym__blank_line_token1 = 39,
  sym_document = 40,
  sym_comment = 41,
  sym__plain_comment = 42,
  sym__var_comment = 43,
  sym_request_separator = 44,
  sym_section = 45,
  sym__section_content = 46,
  sym_http_version = 47,
  aux_sym__target_url_line = 48,
  sym_target_url = 49,
  sym_response = 50,
  sym_request = 51,
  sym_header = 52,
  sym_variable = 53,
  sym_pre_request_script = 54,
  sym_res_handler_script = 55,
  sym_script = 56,
  sym_res_redirect = 57,
  sym_variable_declaration = 58,
  sym_xml_body = 59,
  sym_json_body = 60,
  sym_graphql_body = 61,
  sym_graphql_data = 62,
  sym_graphql_json_body = 63,
  sym__external_body = 64,
  sym_external_body = 65,
  sym_multipart_form_data = 66,
  sym_raw_body = 67,
  sym__raw_body = 68,
  sym_path = 69,
  sym_value = 70,
  sym__blank_line = 71,
  aux_sym_document_repeat1 = 72,
  aux_sym_target_url_repeat1 = 73,
  aux_sym___body_repeat1 = 74,
  aux_sym___body_repeat2 = 75,
  aux_sym_response_repeat1 = 76,
  aux_sym_request_repeat1 = 77,
  aux_sym_script_repeat1 = 78,
  aux_sym_multipart_form_data_repeat1 = 79,
  aux_sym_path_repeat1 = 80,
  aux_sym_value_repeat1 = 81,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [aux_sym_WORD_CHAR_token1] = "WORD_CHAR_token1",
  [aux_sym_PUNCTUATION_token1] = "PUNCTUATION_token1",
  [aux_sym_WS_token1] = "WS_token1",
  [aux_sym_NL_token1] = "NL_token1",
  [aux_sym_LINE_TAIL_token1] = "LINE_TAIL_token1",
  [aux_sym_COMMENT_PREFIX_token1] = "COMMENT_PREFIX_token1",
  [anon_sym_AT] = "@",
  [anon_sym_EQ] = "=",
  [aux_sym__var_comment_token1] = "_var_comment_token1",
  [aux_sym_request_separator_token1] = "request_separator_token1",
  [sym_method] = "method",
  [aux_sym_http_version_token1] = "http_version_token1",
  [sym_status_code] = "status_code",
  [sym_status_text] = "status_text",
  [anon_sym_COLON] = ":",
  [anon_sym_LBRACE_LBRACE] = "{{",
  [anon_sym_RBRACE_RBRACE] = "}}",
  [anon_sym_LT] = "<",
  [aux_sym_pre_request_script_token1] = "pre_request_script_token1",
  [anon_sym_GT] = ">",
  [anon_sym_LBRACE_PERCENT] = "{%",
  [anon_sym_PERCENT_RBRACE] = "%}",
  [aux_sym_res_redirect_token1] = "res_redirect_token1",
  [anon_sym_AT2] = "@",
  [aux_sym_xml_body_token1] = "xml_body_token1",
  [aux_sym_json_body_token1] = "json_body_token1",
  [aux_sym_graphql_data_token1] = "graphql_data_token1",
  [aux_sym_graphql_json_body_token1] = "graphql_json_body_token1",
  [anon_sym_LT2] = "<",
  [anon_sym_DASH_DASH] = "--",
  [aux_sym_multipart_form_data_token1] = "multipart_form_data_token1",
  [aux_sym_multipart_form_data_token2] = "multipart_form_data_token2",
  [aux_sym_raw_body_token1] = "raw_body_token1",
  [aux_sym__raw_body_token1] = "_raw_body_token1",
  [sym__not_comment] = "_not_comment",
  [sym_header_entity] = "header_entity",
  [sym_identifier] = "identifier",
  [aux_sym_path_token1] = "path_token1",
  [aux_sym__blank_line_token1] = "_blank_line_token1",
  [sym_document] = "document",
  [sym_comment] = "comment",
  [sym__plain_comment] = "_plain_comment",
  [sym__var_comment] = "_var_comment",
  [sym_request_separator] = "request_separator",
  [sym_section] = "section",
  [sym__section_content] = "_section_content",
  [sym_http_version] = "http_version",
  [aux_sym__target_url_line] = "_target_url_line",
  [sym_target_url] = "target_url",
  [sym_response] = "response",
  [sym_request] = "request",
  [sym_header] = "header",
  [sym_variable] = "variable",
  [sym_pre_request_script] = "pre_request_script",
  [sym_res_handler_script] = "res_handler_script",
  [sym_script] = "script",
  [sym_res_redirect] = "res_redirect",
  [sym_variable_declaration] = "variable_declaration",
  [sym_xml_body] = "xml_body",
  [sym_json_body] = "json_body",
  [sym_graphql_body] = "graphql_body",
  [sym_graphql_data] = "graphql_data",
  [sym_graphql_json_body] = "json_body",
  [sym__external_body] = "_external_body",
  [sym_external_body] = "external_body",
  [sym_multipart_form_data] = "multipart_form_data",
  [sym_raw_body] = "raw_body",
  [sym__raw_body] = "_raw_body",
  [sym_path] = "path",
  [sym_value] = "value",
  [sym__blank_line] = "_blank_line",
  [aux_sym_document_repeat1] = "document_repeat1",
  [aux_sym_target_url_repeat1] = "target_url_repeat1",
  [aux_sym___body_repeat1] = "__body_repeat1",
  [aux_sym___body_repeat2] = "__body_repeat2",
  [aux_sym_response_repeat1] = "response_repeat1",
  [aux_sym_request_repeat1] = "request_repeat1",
  [aux_sym_script_repeat1] = "script_repeat1",
  [aux_sym_multipart_form_data_repeat1] = "multipart_form_data_repeat1",
  [aux_sym_path_repeat1] = "path_repeat1",
  [aux_sym_value_repeat1] = "value_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [aux_sym_WORD_CHAR_token1] = aux_sym_WORD_CHAR_token1,
  [aux_sym_PUNCTUATION_token1] = aux_sym_PUNCTUATION_token1,
  [aux_sym_WS_token1] = aux_sym_WS_token1,
  [aux_sym_NL_token1] = aux_sym_NL_token1,
  [aux_sym_LINE_TAIL_token1] = aux_sym_LINE_TAIL_token1,
  [aux_sym_COMMENT_PREFIX_token1] = aux_sym_COMMENT_PREFIX_token1,
  [anon_sym_AT] = anon_sym_AT,
  [anon_sym_EQ] = anon_sym_EQ,
  [aux_sym__var_comment_token1] = aux_sym__var_comment_token1,
  [aux_sym_request_separator_token1] = aux_sym_request_separator_token1,
  [sym_method] = sym_method,
  [aux_sym_http_version_token1] = aux_sym_http_version_token1,
  [sym_status_code] = sym_status_code,
  [sym_status_text] = sym_status_text,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_LBRACE_LBRACE] = anon_sym_LBRACE_LBRACE,
  [anon_sym_RBRACE_RBRACE] = anon_sym_RBRACE_RBRACE,
  [anon_sym_LT] = anon_sym_LT,
  [aux_sym_pre_request_script_token1] = aux_sym_pre_request_script_token1,
  [anon_sym_GT] = anon_sym_GT,
  [anon_sym_LBRACE_PERCENT] = anon_sym_LBRACE_PERCENT,
  [anon_sym_PERCENT_RBRACE] = anon_sym_PERCENT_RBRACE,
  [aux_sym_res_redirect_token1] = aux_sym_res_redirect_token1,
  [anon_sym_AT2] = anon_sym_AT,
  [aux_sym_xml_body_token1] = aux_sym_xml_body_token1,
  [aux_sym_json_body_token1] = aux_sym_json_body_token1,
  [aux_sym_graphql_data_token1] = aux_sym_graphql_data_token1,
  [aux_sym_graphql_json_body_token1] = aux_sym_graphql_json_body_token1,
  [anon_sym_LT2] = anon_sym_LT,
  [anon_sym_DASH_DASH] = anon_sym_DASH_DASH,
  [aux_sym_multipart_form_data_token1] = aux_sym_multipart_form_data_token1,
  [aux_sym_multipart_form_data_token2] = aux_sym_multipart_form_data_token2,
  [aux_sym_raw_body_token1] = aux_sym_raw_body_token1,
  [aux_sym__raw_body_token1] = aux_sym__raw_body_token1,
  [sym__not_comment] = sym__not_comment,
  [sym_header_entity] = sym_header_entity,
  [sym_identifier] = sym_identifier,
  [aux_sym_path_token1] = aux_sym_path_token1,
  [aux_sym__blank_line_token1] = aux_sym__blank_line_token1,
  [sym_document] = sym_document,
  [sym_comment] = sym_comment,
  [sym__plain_comment] = sym__plain_comment,
  [sym__var_comment] = sym__var_comment,
  [sym_request_separator] = sym_request_separator,
  [sym_section] = sym_section,
  [sym__section_content] = sym__section_content,
  [sym_http_version] = sym_http_version,
  [aux_sym__target_url_line] = aux_sym__target_url_line,
  [sym_target_url] = sym_target_url,
  [sym_response] = sym_response,
  [sym_request] = sym_request,
  [sym_header] = sym_header,
  [sym_variable] = sym_variable,
  [sym_pre_request_script] = sym_pre_request_script,
  [sym_res_handler_script] = sym_res_handler_script,
  [sym_script] = sym_script,
  [sym_res_redirect] = sym_res_redirect,
  [sym_variable_declaration] = sym_variable_declaration,
  [sym_xml_body] = sym_xml_body,
  [sym_json_body] = sym_json_body,
  [sym_graphql_body] = sym_graphql_body,
  [sym_graphql_data] = sym_graphql_data,
  [sym_graphql_json_body] = sym_json_body,
  [sym__external_body] = sym__external_body,
  [sym_external_body] = sym_external_body,
  [sym_multipart_form_data] = sym_multipart_form_data,
  [sym_raw_body] = sym_raw_body,
  [sym__raw_body] = sym__raw_body,
  [sym_path] = sym_path,
  [sym_value] = sym_value,
  [sym__blank_line] = sym__blank_line,
  [aux_sym_document_repeat1] = aux_sym_document_repeat1,
  [aux_sym_target_url_repeat1] = aux_sym_target_url_repeat1,
  [aux_sym___body_repeat1] = aux_sym___body_repeat1,
  [aux_sym___body_repeat2] = aux_sym___body_repeat2,
  [aux_sym_response_repeat1] = aux_sym_response_repeat1,
  [aux_sym_request_repeat1] = aux_sym_request_repeat1,
  [aux_sym_script_repeat1] = aux_sym_script_repeat1,
  [aux_sym_multipart_form_data_repeat1] = aux_sym_multipart_form_data_repeat1,
  [aux_sym_path_repeat1] = aux_sym_path_repeat1,
  [aux_sym_value_repeat1] = aux_sym_value_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [aux_sym_WORD_CHAR_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_PUNCTUATION_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_WS_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_NL_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_LINE_TAIL_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_COMMENT_PREFIX_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_AT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ] = {
    .visible = true,
    .named = false,
  },
  [aux_sym__var_comment_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_request_separator_token1] = {
    .visible = false,
    .named = false,
  },
  [sym_method] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_http_version_token1] = {
    .visible = false,
    .named = false,
  },
  [sym_status_code] = {
    .visible = true,
    .named = true,
  },
  [sym_status_text] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACE_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LT] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_pre_request_script_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_GT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACE_PERCENT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PERCENT_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_res_redirect_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_AT2] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_xml_body_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_json_body_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_graphql_data_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_graphql_json_body_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_LT2] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH_DASH] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_multipart_form_data_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_multipart_form_data_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_raw_body_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym__raw_body_token1] = {
    .visible = false,
    .named = false,
  },
  [sym__not_comment] = {
    .visible = false,
    .named = true,
  },
  [sym_header_entity] = {
    .visible = true,
    .named = true,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_path_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym__blank_line_token1] = {
    .visible = false,
    .named = false,
  },
  [sym_document] = {
    .visible = true,
    .named = true,
  },
  [sym_comment] = {
    .visible = true,
    .named = true,
  },
  [sym__plain_comment] = {
    .visible = false,
    .named = true,
  },
  [sym__var_comment] = {
    .visible = false,
    .named = true,
  },
  [sym_request_separator] = {
    .visible = true,
    .named = true,
  },
  [sym_section] = {
    .visible = true,
    .named = true,
  },
  [sym__section_content] = {
    .visible = false,
    .named = true,
  },
  [sym_http_version] = {
    .visible = true,
    .named = true,
  },
  [aux_sym__target_url_line] = {
    .visible = false,
    .named = false,
  },
  [sym_target_url] = {
    .visible = true,
    .named = true,
  },
  [sym_response] = {
    .visible = true,
    .named = true,
  },
  [sym_request] = {
    .visible = true,
    .named = true,
  },
  [sym_header] = {
    .visible = true,
    .named = true,
  },
  [sym_variable] = {
    .visible = true,
    .named = true,
  },
  [sym_pre_request_script] = {
    .visible = true,
    .named = true,
  },
  [sym_res_handler_script] = {
    .visible = true,
    .named = true,
  },
  [sym_script] = {
    .visible = true,
    .named = true,
  },
  [sym_res_redirect] = {
    .visible = true,
    .named = true,
  },
  [sym_variable_declaration] = {
    .visible = true,
    .named = true,
  },
  [sym_xml_body] = {
    .visible = true,
    .named = true,
  },
  [sym_json_body] = {
    .visible = true,
    .named = true,
  },
  [sym_graphql_body] = {
    .visible = true,
    .named = true,
  },
  [sym_graphql_data] = {
    .visible = true,
    .named = true,
  },
  [sym_graphql_json_body] = {
    .visible = true,
    .named = true,
  },
  [sym__external_body] = {
    .visible = false,
    .named = true,
  },
  [sym_external_body] = {
    .visible = true,
    .named = true,
  },
  [sym_multipart_form_data] = {
    .visible = true,
    .named = true,
  },
  [sym_raw_body] = {
    .visible = true,
    .named = true,
  },
  [sym__raw_body] = {
    .visible = false,
    .named = true,
  },
  [sym_path] = {
    .visible = true,
    .named = true,
  },
  [sym_value] = {
    .visible = true,
    .named = true,
  },
  [sym__blank_line] = {
    .visible = false,
    .named = true,
  },
  [aux_sym_document_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_target_url_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym___body_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym___body_repeat2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_response_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_request_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_script_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_multipart_form_data_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_path_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_value_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum ts_field_identifiers {
  field_body = 1,
  field_header = 2,
  field_method = 3,
  field_name = 4,
  field_path = 5,
  field_request = 6,
  field_response = 7,
  field_url = 8,
  field_value = 9,
  field_version = 10,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_body] = "body",
  [field_header] = "header",
  [field_method] = "method",
  [field_name] = "name",
  [field_path] = "path",
  [field_request] = "request",
  [field_response] = "response",
  [field_url] = "url",
  [field_value] = "value",
  [field_version] = "version",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 2},
  [2] = {.index = 2, .length = 2},
  [3] = {.index = 4, .length = 1},
  [4] = {.index = 5, .length = 1},
  [5] = {.index = 6, .length = 2},
  [6] = {.index = 8, .length = 1},
  [7] = {.index = 9, .length = 1},
  [8] = {.index = 10, .length = 1},
  [9] = {.index = 11, .length = 1},
  [10] = {.index = 12, .length = 2},
  [11] = {.index = 14, .length = 1},
  [12] = {.index = 15, .length = 1},
  [13] = {.index = 16, .length = 2},
  [14] = {.index = 18, .length = 2},
  [15] = {.index = 0, .length = 2},
  [16] = {.index = 20, .length = 1},
  [17] = {.index = 21, .length = 2},
  [18] = {.index = 23, .length = 2},
  [19] = {.index = 25, .length = 3},
  [20] = {.index = 28, .length = 2},
  [21] = {.index = 30, .length = 3},
  [22] = {.index = 33, .length = 1},
  [23] = {.index = 34, .length = 2},
  [24] = {.index = 36, .length = 3},
  [25] = {.index = 39, .length = 2},
  [26] = {.index = 41, .length = 3},
  [27] = {.index = 44, .length = 3},
  [28] = {.index = 47, .length = 2},
  [29] = {.index = 49, .length = 1},
  [30] = {.index = 50, .length = 3},
  [31] = {.index = 53, .length = 2},
  [32] = {.index = 55, .length = 1},
  [33] = {.index = 56, .length = 2},
  [34] = {.index = 58, .length = 4},
  [35] = {.index = 62, .length = 4},
  [36] = {.index = 66, .length = 2},
  [37] = {.index = 68, .length = 1},
  [38] = {.index = 69, .length = 1},
  [39] = {.index = 70, .length = 4},
  [40] = {.index = 74, .length = 2},
  [41] = {.index = 76, .length = 1},
  [42] = {.index = 77, .length = 4},
  [43] = {.index = 81, .length = 2},
  [44] = {.index = 83, .length = 1},
  [45] = {.index = 84, .length = 2},
  [46] = {.index = 86, .length = 2},
  [47] = {.index = 88, .length = 5},
  [48] = {.index = 93, .length = 2},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_name, 0, .inherited = true},
    {field_value, 0, .inherited = true},
  [2] =
    {field_request, 0, .inherited = true},
    {field_response, 0, .inherited = true},
  [4] =
    {field_response, 0},
  [5] =
    {field_request, 0},
  [6] =
    {field_request, 1, .inherited = true},
    {field_response, 1, .inherited = true},
  [8] =
    {field_url, 0},
  [9] =
    {field_value, 1},
  [10] =
    {field_name, 1},
  [11] =
    {field_header, 0},
  [12] =
    {field_header, 2, .inherited = true},
    {field_url, 0},
  [14] =
    {field_name, 2},
  [15] =
    {field_value, 2},
  [16] =
    {field_method, 0},
    {field_url, 2},
  [18] =
    {field_url, 0},
    {field_version, 2},
  [20] =
    {field_body, 0},
  [21] =
    {field_body, 3, .inherited = true},
    {field_url, 0},
  [23] =
    {field_header, 0, .inherited = true},
    {field_header, 1, .inherited = true},
  [25] =
    {field_header, 4, .inherited = true},
    {field_method, 0},
    {field_url, 2},
  [28] =
    {field_name, 1},
    {field_value, 3},
  [30] =
    {field_header, 4, .inherited = true},
    {field_url, 0},
    {field_version, 2},
  [33] =
    {field_name, 0},
  [34] =
    {field_body, 0, .inherited = true},
    {field_body, 1, .inherited = true},
  [36] =
    {field_body, 4, .inherited = true},
    {field_header, 2, .inherited = true},
    {field_url, 0},
  [39] =
    {field_name, 2},
    {field_value, 4},
  [41] =
    {field_method, 0},
    {field_url, 2},
    {field_version, 4},
  [44] =
    {field_body, 5, .inherited = true},
    {field_method, 0},
    {field_url, 2},
  [47] =
    {field_name, 1},
    {field_value, 4},
  [49] =
    {field_header, 5, .inherited = true},
  [50] =
    {field_body, 5, .inherited = true},
    {field_url, 0},
    {field_version, 2},
  [53] =
    {field_name, 0},
    {field_value, 2},
  [55] =
    {field_path, 2},
  [56] =
    {field_name, 2},
    {field_value, 5},
  [58] =
    {field_header, 6, .inherited = true},
    {field_method, 0},
    {field_url, 2},
    {field_version, 4},
  [62] =
    {field_body, 6, .inherited = true},
    {field_header, 4, .inherited = true},
    {field_method, 0},
    {field_url, 2},
  [66] =
    {field_name, 1},
    {field_value, 5},
  [68] =
    {field_body, 6, .inherited = true},
  [69] =
    {field_header, 6, .inherited = true},
  [70] =
    {field_body, 6, .inherited = true},
    {field_header, 4, .inherited = true},
    {field_url, 0},
    {field_version, 2},
  [74] =
    {field_name, 0},
    {field_value, 3},
  [76] =
    {field_path, 3},
  [77] =
    {field_body, 7, .inherited = true},
    {field_method, 0},
    {field_url, 2},
    {field_version, 4},
  [81] =
    {field_body, 7, .inherited = true},
    {field_header, 5, .inherited = true},
  [83] =
    {field_body, 7, .inherited = true},
  [84] =
    {field_name, 0},
    {field_value, 4},
  [86] =
    {field_name, 2},
    {field_path, 4},
  [88] =
    {field_body, 8, .inherited = true},
    {field_header, 6, .inherited = true},
    {field_method, 0},
    {field_url, 2},
    {field_version, 4},
  [93] =
    {field_body, 8, .inherited = true},
    {field_header, 6, .inherited = true},
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
  [15] = {
    [0] = sym_comment,
  },
};

static const uint16_t ts_non_terminal_alias_map[] = {
  sym__var_comment, 2,
    sym__var_comment,
    sym_comment,
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
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 25,
  [26] = 26,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 30,
  [31] = 31,
  [32] = 32,
  [33] = 33,
  [34] = 34,
  [35] = 35,
  [36] = 36,
  [37] = 35,
  [38] = 38,
  [39] = 39,
  [40] = 40,
  [41] = 41,
  [42] = 34,
  [43] = 43,
  [44] = 44,
  [45] = 45,
  [46] = 46,
  [47] = 47,
  [48] = 48,
  [49] = 49,
  [50] = 50,
  [51] = 51,
  [52] = 52,
  [53] = 53,
  [54] = 54,
  [55] = 46,
  [56] = 45,
  [57] = 57,
  [58] = 58,
  [59] = 59,
  [60] = 60,
  [61] = 61,
  [62] = 44,
  [63] = 63,
  [64] = 51,
  [65] = 65,
  [66] = 66,
  [67] = 50,
  [68] = 68,
  [69] = 69,
  [70] = 70,
  [71] = 71,
  [72] = 72,
  [73] = 73,
  [74] = 74,
  [75] = 75,
  [76] = 76,
  [77] = 77,
  [78] = 78,
  [79] = 79,
  [80] = 80,
  [81] = 81,
  [82] = 82,
  [83] = 83,
  [84] = 36,
  [85] = 85,
  [86] = 86,
  [87] = 87,
  [88] = 88,
  [89] = 89,
  [90] = 90,
  [91] = 91,
  [92] = 39,
  [93] = 44,
  [94] = 45,
  [95] = 38,
  [96] = 46,
  [97] = 97,
  [98] = 98,
  [99] = 99,
  [100] = 100,
  [101] = 44,
  [102] = 46,
  [103] = 103,
  [104] = 104,
  [105] = 105,
  [106] = 57,
  [107] = 36,
  [108] = 108,
  [109] = 45,
  [110] = 110,
  [111] = 111,
  [112] = 38,
  [113] = 113,
  [114] = 114,
  [115] = 115,
  [116] = 39,
  [117] = 117,
  [118] = 118,
  [119] = 119,
  [120] = 120,
  [121] = 121,
  [122] = 122,
  [123] = 123,
  [124] = 124,
  [125] = 124,
  [126] = 126,
  [127] = 127,
  [128] = 124,
  [129] = 129,
  [130] = 130,
  [131] = 131,
  [132] = 132,
  [133] = 124,
  [134] = 134,
  [135] = 135,
  [136] = 136,
  [137] = 137,
  [138] = 136,
  [139] = 139,
  [140] = 136,
  [141] = 130,
  [142] = 136,
  [143] = 143,
  [144] = 144,
  [145] = 145,
  [146] = 146,
  [147] = 147,
  [148] = 148,
  [149] = 149,
  [150] = 129,
  [151] = 151,
  [152] = 152,
  [153] = 153,
  [154] = 154,
  [155] = 155,
  [156] = 156,
  [157] = 152,
  [158] = 153,
  [159] = 154,
  [160] = 151,
  [161] = 152,
  [162] = 153,
  [163] = 154,
  [164] = 151,
  [165] = 165,
  [166] = 166,
  [167] = 167,
  [168] = 168,
  [169] = 169,
  [170] = 170,
  [171] = 170,
  [172] = 170,
  [173] = 170,
  [174] = 174,
  [175] = 175,
  [176] = 176,
  [177] = 177,
  [178] = 178,
  [179] = 179,
  [180] = 180,
  [181] = 181,
  [182] = 182,
  [183] = 183,
  [184] = 183,
  [185] = 185,
  [186] = 186,
  [187] = 187,
  [188] = 178,
  [189] = 187,
  [190] = 186,
  [191] = 191,
  [192] = 187,
  [193] = 178,
  [194] = 194,
  [195] = 183,
  [196] = 196,
  [197] = 197,
  [198] = 198,
  [199] = 199,
  [200] = 200,
  [201] = 201,
  [202] = 202,
  [203] = 186,
  [204] = 204,
  [205] = 205,
  [206] = 206,
  [207] = 207,
  [208] = 208,
  [209] = 209,
  [210] = 210,
  [211] = 211,
  [212] = 212,
  [213] = 213,
  [214] = 214,
  [215] = 215,
  [216] = 216,
  [217] = 217,
  [218] = 206,
  [219] = 219,
  [220] = 220,
  [221] = 221,
  [222] = 222,
  [223] = 223,
  [224] = 224,
  [225] = 225,
  [226] = 226,
  [227] = 227,
  [228] = 228,
  [229] = 229,
  [230] = 230,
  [231] = 231,
  [232] = 232,
  [233] = 233,
  [234] = 232,
  [235] = 235,
  [236] = 236,
  [237] = 237,
  [238] = 238,
  [239] = 239,
  [240] = 240,
  [241] = 206,
  [242] = 233,
  [243] = 232,
  [244] = 236,
  [245] = 236,
  [246] = 232,
  [247] = 236,
  [248] = 248,
  [249] = 227,
  [250] = 237,
  [251] = 251,
  [252] = 252,
  [253] = 233,
  [254] = 227,
  [255] = 237,
  [256] = 256,
  [257] = 257,
  [258] = 227,
  [259] = 259,
  [260] = 235,
};

static TSCharacterRange aux_sym_WORD_CHAR_token1_character_set_1[] = {
  {'0', '9'}, {'A', 'Z'}, {'a', 'z'}, {0xaa, 0xaa}, {0xb2, 0xb3}, {0xb5, 0xb5}, {0xb9, 0xba}, {0xbc, 0xbe},
  {0xc0, 0xd6}, {0xd8, 0xf6}, {0xf8, 0x2c1}, {0x2c6, 0x2d1}, {0x2e0, 0x2e4}, {0x2ec, 0x2ec}, {0x2ee, 0x2ee}, {0x370, 0x374},
  {0x376, 0x377}, {0x37a, 0x37d}, {0x37f, 0x37f}, {0x386, 0x386}, {0x388, 0x38a}, {0x38c, 0x38c}, {0x38e, 0x3a1}, {0x3a3, 0x3f5},
  {0x3f7, 0x481}, {0x48a, 0x52f}, {0x531, 0x556}, {0x559, 0x559}, {0x560, 0x588}, {0x5d0, 0x5ea}, {0x5ef, 0x5f2}, {0x620, 0x64a},
  {0x660, 0x669}, {0x66e, 0x66f}, {0x671, 0x6d3}, {0x6d5, 0x6d5}, {0x6e5, 0x6e6}, {0x6ee, 0x6fc}, {0x6ff, 0x6ff}, {0x710, 0x710},
  {0x712, 0x72f}, {0x74d, 0x7a5}, {0x7b1, 0x7b1}, {0x7c0, 0x7ea}, {0x7f4, 0x7f5}, {0x7fa, 0x7fa}, {0x800, 0x815}, {0x81a, 0x81a},
  {0x824, 0x824}, {0x828, 0x828}, {0x840, 0x858}, {0x860, 0x86a}, {0x870, 0x887}, {0x889, 0x88e}, {0x8a0, 0x8c9}, {0x904, 0x939},
  {0x93d, 0x93d}, {0x950, 0x950}, {0x958, 0x961}, {0x966, 0x96f}, {0x971, 0x980}, {0x985, 0x98c}, {0x98f, 0x990}, {0x993, 0x9a8},
  {0x9aa, 0x9b0}, {0x9b2, 0x9b2}, {0x9b6, 0x9b9}, {0x9bd, 0x9bd}, {0x9ce, 0x9ce}, {0x9dc, 0x9dd}, {0x9df, 0x9e1}, {0x9e6, 0x9f1},
  {0x9f4, 0x9f9}, {0x9fc, 0x9fc}, {0xa05, 0xa0a}, {0xa0f, 0xa10}, {0xa13, 0xa28}, {0xa2a, 0xa30}, {0xa32, 0xa33}, {0xa35, 0xa36},
  {0xa38, 0xa39}, {0xa59, 0xa5c}, {0xa5e, 0xa5e}, {0xa66, 0xa6f}, {0xa72, 0xa74}, {0xa85, 0xa8d}, {0xa8f, 0xa91}, {0xa93, 0xaa8},
  {0xaaa, 0xab0}, {0xab2, 0xab3}, {0xab5, 0xab9}, {0xabd, 0xabd}, {0xad0, 0xad0}, {0xae0, 0xae1}, {0xae6, 0xaef}, {0xaf9, 0xaf9},
  {0xb05, 0xb0c}, {0xb0f, 0xb10}, {0xb13, 0xb28}, {0xb2a, 0xb30}, {0xb32, 0xb33}, {0xb35, 0xb39}, {0xb3d, 0xb3d}, {0xb5c, 0xb5d},
  {0xb5f, 0xb61}, {0xb66, 0xb6f}, {0xb71, 0xb77}, {0xb83, 0xb83}, {0xb85, 0xb8a}, {0xb8e, 0xb90}, {0xb92, 0xb95}, {0xb99, 0xb9a},
  {0xb9c, 0xb9c}, {0xb9e, 0xb9f}, {0xba3, 0xba4}, {0xba8, 0xbaa}, {0xbae, 0xbb9}, {0xbd0, 0xbd0}, {0xbe6, 0xbf2}, {0xc05, 0xc0c},
  {0xc0e, 0xc10}, {0xc12, 0xc28}, {0xc2a, 0xc39}, {0xc3d, 0xc3d}, {0xc58, 0xc5a}, {0xc5d, 0xc5d}, {0xc60, 0xc61}, {0xc66, 0xc6f},
  {0xc78, 0xc7e}, {0xc80, 0xc80}, {0xc85, 0xc8c}, {0xc8e, 0xc90}, {0xc92, 0xca8}, {0xcaa, 0xcb3}, {0xcb5, 0xcb9}, {0xcbd, 0xcbd},
  {0xcdd, 0xcde}, {0xce0, 0xce1}, {0xce6, 0xcef}, {0xcf1, 0xcf2}, {0xd04, 0xd0c}, {0xd0e, 0xd10}, {0xd12, 0xd3a}, {0xd3d, 0xd3d},
  {0xd4e, 0xd4e}, {0xd54, 0xd56}, {0xd58, 0xd61}, {0xd66, 0xd78}, {0xd7a, 0xd7f}, {0xd85, 0xd96}, {0xd9a, 0xdb1}, {0xdb3, 0xdbb},
  {0xdbd, 0xdbd}, {0xdc0, 0xdc6}, {0xde6, 0xdef}, {0xe01, 0xe30}, {0xe32, 0xe33}, {0xe40, 0xe46}, {0xe50, 0xe59}, {0xe81, 0xe82},
  {0xe84, 0xe84}, {0xe86, 0xe8a}, {0xe8c, 0xea3}, {0xea5, 0xea5}, {0xea7, 0xeb0}, {0xeb2, 0xeb3}, {0xebd, 0xebd}, {0xec0, 0xec4},
  {0xec6, 0xec6}, {0xed0, 0xed9}, {0xedc, 0xedf}, {0xf00, 0xf00}, {0xf20, 0xf33}, {0xf40, 0xf47}, {0xf49, 0xf6c}, {0xf88, 0xf8c},
  {0x1000, 0x102a}, {0x103f, 0x1049}, {0x1050, 0x1055}, {0x105a, 0x105d}, {0x1061, 0x1061}, {0x1065, 0x1066}, {0x106e, 0x1070}, {0x1075, 0x1081},
  {0x108e, 0x108e}, {0x1090, 0x1099}, {0x10a0, 0x10c5}, {0x10c7, 0x10c7}, {0x10cd, 0x10cd}, {0x10d0, 0x10fa}, {0x10fc, 0x1248}, {0x124a, 0x124d},
  {0x1250, 0x1256}, {0x1258, 0x1258}, {0x125a, 0x125d}, {0x1260, 0x1288}, {0x128a, 0x128d}, {0x1290, 0x12b0}, {0x12b2, 0x12b5}, {0x12b8, 0x12be},
  {0x12c0, 0x12c0}, {0x12c2, 0x12c5}, {0x12c8, 0x12d6}, {0x12d8, 0x1310}, {0x1312, 0x1315}, {0x1318, 0x135a}, {0x1369, 0x137c}, {0x1380, 0x138f},
  {0x13a0, 0x13f5}, {0x13f8, 0x13fd}, {0x1401, 0x166c}, {0x166f, 0x167f}, {0x1681, 0x169a}, {0x16a0, 0x16ea}, {0x16ee, 0x16f8}, {0x1700, 0x1711},
  {0x171f, 0x1731}, {0x1740, 0x1751}, {0x1760, 0x176c}, {0x176e, 0x1770}, {0x1780, 0x17b3}, {0x17d7, 0x17d7}, {0x17dc, 0x17dc}, {0x17e0, 0x17e9},
  {0x17f0, 0x17f9}, {0x1810, 0x1819}, {0x1820, 0x1878}, {0x1880, 0x1884}, {0x1887, 0x18a8}, {0x18aa, 0x18aa}, {0x18b0, 0x18f5}, {0x1900, 0x191e},
  {0x1946, 0x196d}, {0x1970, 0x1974}, {0x1980, 0x19ab}, {0x19b0, 0x19c9}, {0x19d0, 0x19da}, {0x1a00, 0x1a16}, {0x1a20, 0x1a54}, {0x1a80, 0x1a89},
  {0x1a90, 0x1a99}, {0x1aa7, 0x1aa7}, {0x1b05, 0x1b33}, {0x1b45, 0x1b4c}, {0x1b50, 0x1b59}, {0x1b83, 0x1ba0}, {0x1bae, 0x1be5}, {0x1c00, 0x1c23},
  {0x1c40, 0x1c49}, {0x1c4d, 0x1c7d}, {0x1c80, 0x1c88}, {0x1c90, 0x1cba}, {0x1cbd, 0x1cbf}, {0x1ce9, 0x1cec}, {0x1cee, 0x1cf3}, {0x1cf5, 0x1cf6},
  {0x1cfa, 0x1cfa}, {0x1d00, 0x1dbf}, {0x1e00, 0x1f15}, {0x1f18, 0x1f1d}, {0x1f20, 0x1f45}, {0x1f48, 0x1f4d}, {0x1f50, 0x1f57}, {0x1f59, 0x1f59},
  {0x1f5b, 0x1f5b}, {0x1f5d, 0x1f5d}, {0x1f5f, 0x1f7d}, {0x1f80, 0x1fb4}, {0x1fb6, 0x1fbc}, {0x1fbe, 0x1fbe}, {0x1fc2, 0x1fc4}, {0x1fc6, 0x1fcc},
  {0x1fd0, 0x1fd3}, {0x1fd6, 0x1fdb}, {0x1fe0, 0x1fec}, {0x1ff2, 0x1ff4}, {0x1ff6, 0x1ffc}, {0x2070, 0x2071}, {0x2074, 0x2079}, {0x207f, 0x2089},
  {0x2090, 0x209c}, {0x2102, 0x2102}, {0x2107, 0x2107}, {0x210a, 0x2113}, {0x2115, 0x2115}, {0x2119, 0x211d}, {0x2124, 0x2124}, {0x2126, 0x2126},
  {0x2128, 0x2128}, {0x212a, 0x212d}, {0x212f, 0x2139}, {0x213c, 0x213f}, {0x2145, 0x2149}, {0x214e, 0x214e}, {0x2150, 0x2189}, {0x2460, 0x249b},
  {0x24ea, 0x24ff}, {0x2776, 0x2793}, {0x2c00, 0x2ce4}, {0x2ceb, 0x2cee}, {0x2cf2, 0x2cf3}, {0x2cfd, 0x2cfd}, {0x2d00, 0x2d25}, {0x2d27, 0x2d27},
  {0x2d2d, 0x2d2d}, {0x2d30, 0x2d67}, {0x2d6f, 0x2d6f}, {0x2d80, 0x2d96}, {0x2da0, 0x2da6}, {0x2da8, 0x2dae}, {0x2db0, 0x2db6}, {0x2db8, 0x2dbe},
  {0x2dc0, 0x2dc6}, {0x2dc8, 0x2dce}, {0x2dd0, 0x2dd6}, {0x2dd8, 0x2dde}, {0x2e2f, 0x2e2f}, {0x3005, 0x3007}, {0x3021, 0x3029}, {0x3031, 0x3035},
  {0x3038, 0x303c}, {0x3041, 0x3096}, {0x309d, 0x309f}, {0x30a1, 0x30fa}, {0x30fc, 0x30ff}, {0x3105, 0x312f}, {0x3131, 0x318e}, {0x3192, 0x3195},
  {0x31a0, 0x31bf}, {0x31f0, 0x31ff}, {0x3220, 0x3229}, {0x3248, 0x324f}, {0x3251, 0x325f}, {0x3280, 0x3289}, {0x32b1, 0x32bf}, {0x3400, 0x3400},
  {0x4dbf, 0x4dbf}, {0x4e00, 0x4e00}, {0x9fff, 0xa48c}, {0xa4d0, 0xa4fd}, {0xa500, 0xa60c}, {0xa610, 0xa62b}, {0xa640, 0xa66e}, {0xa67f, 0xa69d},
  {0xa6a0, 0xa6ef}, {0xa717, 0xa71f}, {0xa722, 0xa788}, {0xa78b, 0xa7ca}, {0xa7d0, 0xa7d1}, {0xa7d3, 0xa7d3}, {0xa7d5, 0xa7d9}, {0xa7f2, 0xa801},
  {0xa803, 0xa805}, {0xa807, 0xa80a}, {0xa80c, 0xa822}, {0xa830, 0xa835}, {0xa840, 0xa873}, {0xa882, 0xa8b3}, {0xa8d0, 0xa8d9}, {0xa8f2, 0xa8f7},
  {0xa8fb, 0xa8fb}, {0xa8fd, 0xa8fe}, {0xa900, 0xa925}, {0xa930, 0xa946}, {0xa960, 0xa97c}, {0xa984, 0xa9b2}, {0xa9cf, 0xa9d9}, {0xa9e0, 0xa9e4},
  {0xa9e6, 0xa9fe}, {0xaa00, 0xaa28}, {0xaa40, 0xaa42}, {0xaa44, 0xaa4b}, {0xaa50, 0xaa59}, {0xaa60, 0xaa76}, {0xaa7a, 0xaa7a}, {0xaa7e, 0xaaaf},
  {0xaab1, 0xaab1}, {0xaab5, 0xaab6}, {0xaab9, 0xaabd}, {0xaac0, 0xaac0}, {0xaac2, 0xaac2}, {0xaadb, 0xaadd}, {0xaae0, 0xaaea}, {0xaaf2, 0xaaf4},
  {0xab01, 0xab06}, {0xab09, 0xab0e}, {0xab11, 0xab16}, {0xab20, 0xab26}, {0xab28, 0xab2e}, {0xab30, 0xab5a}, {0xab5c, 0xab69}, {0xab70, 0xabe2},
  {0xabf0, 0xabf9}, {0xac00, 0xac00}, {0xd7a3, 0xd7a3}, {0xd7b0, 0xd7c6}, {0xd7cb, 0xd7fb}, {0xf900, 0xfa6d}, {0xfa70, 0xfad9}, {0xfb00, 0xfb06},
  {0xfb13, 0xfb17}, {0xfb1d, 0xfb1d}, {0xfb1f, 0xfb28}, {0xfb2a, 0xfb36}, {0xfb38, 0xfb3c}, {0xfb3e, 0xfb3e}, {0xfb40, 0xfb41}, {0xfb43, 0xfb44},
  {0xfb46, 0xfbb1}, {0xfbd3, 0xfd3d}, {0xfd50, 0xfd8f}, {0xfd92, 0xfdc7}, {0xfdf0, 0xfdfb}, {0xfe70, 0xfe74}, {0xfe76, 0xfefc}, {0xff10, 0xff19},
  {0xff21, 0xff3a}, {0xff41, 0xff5a}, {0xff66, 0xffbe}, {0xffc2, 0xffc7}, {0xffca, 0xffcf}, {0xffd2, 0xffd7}, {0xffda, 0xffdc}, {0x10000, 0x1000b},
  {0x1000d, 0x10026}, {0x10028, 0x1003a}, {0x1003c, 0x1003d}, {0x1003f, 0x1004d}, {0x10050, 0x1005d}, {0x10080, 0x100fa}, {0x10107, 0x10133}, {0x10140, 0x10178},
  {0x1018a, 0x1018b}, {0x10280, 0x1029c}, {0x102a0, 0x102d0}, {0x102e1, 0x102fb}, {0x10300, 0x10323}, {0x1032d, 0x1034a}, {0x10350, 0x10375}, {0x10380, 0x1039d},
  {0x103a0, 0x103c3}, {0x103c8, 0x103cf}, {0x103d1, 0x103d5}, {0x10400, 0x1049d}, {0x104a0, 0x104a9}, {0x104b0, 0x104d3}, {0x104d8, 0x104fb}, {0x10500, 0x10527},
  {0x10530, 0x10563}, {0x10570, 0x1057a}, {0x1057c, 0x1058a}, {0x1058c, 0x10592}, {0x10594, 0x10595}, {0x10597, 0x105a1}, {0x105a3, 0x105b1}, {0x105b3, 0x105b9},
  {0x105bb, 0x105bc}, {0x10600, 0x10736}, {0x10740, 0x10755}, {0x10760, 0x10767}, {0x10780, 0x10785}, {0x10787, 0x107b0}, {0x107b2, 0x107ba}, {0x10800, 0x10805},
  {0x10808, 0x10808}, {0x1080a, 0x10835}, {0x10837, 0x10838}, {0x1083c, 0x1083c}, {0x1083f, 0x10855}, {0x10858, 0x10876}, {0x10879, 0x1089e}, {0x108a7, 0x108af},
  {0x108e0, 0x108f2}, {0x108f4, 0x108f5}, {0x108fb, 0x1091b},
};

static TSCharacterRange aux_sym_PUNCTUATION_token1_character_set_1[] = {
  {0, '\t'}, {0x0b, '\f'}, {0x0e, 0x1f}, {'!', '/'}, {':', '@'}, {'[', '`'}, {'{', 0x9f}, {0xa1, 0xa9},
  {0xab, 0xb1}, {0xb4, 0xb4}, {0xb6, 0xb8}, {0xbb, 0xbb}, {0xbf, 0xbf}, {0xd7, 0xd7}, {0xf7, 0xf7}, {0x2c2, 0x2c5},
  {0x2d2, 0x2df}, {0x2e5, 0x2eb}, {0x2ed, 0x2ed}, {0x2ef, 0x36f}, {0x375, 0x375}, {0x378, 0x379}, {0x37e, 0x37e}, {0x380, 0x385},
  {0x387, 0x387}, {0x38b, 0x38b}, {0x38d, 0x38d}, {0x3a2, 0x3a2}, {0x3f6, 0x3f6}, {0x482, 0x489}, {0x530, 0x530}, {0x557, 0x558},
  {0x55a, 0x55f}, {0x589, 0x5cf}, {0x5eb, 0x5ee}, {0x5f3, 0x61f}, {0x64b, 0x65f}, {0x66a, 0x66d}, {0x670, 0x670}, {0x6d4, 0x6d4},
  {0x6d6, 0x6e4}, {0x6e7, 0x6ed}, {0x6fd, 0x6fe}, {0x700, 0x70f}, {0x711, 0x711}, {0x730, 0x74c}, {0x7a6, 0x7b0}, {0x7b2, 0x7bf},
  {0x7eb, 0x7f3}, {0x7f6, 0x7f9}, {0x7fb, 0x7ff}, {0x816, 0x819}, {0x81b, 0x823}, {0x825, 0x827}, {0x829, 0x83f}, {0x859, 0x85f},
  {0x86b, 0x86f}, {0x888, 0x888}, {0x88f, 0x89f}, {0x8ca, 0x903}, {0x93a, 0x93c}, {0x93e, 0x94f}, {0x951, 0x957}, {0x962, 0x965},
  {0x970, 0x970}, {0x981, 0x984}, {0x98d, 0x98e}, {0x991, 0x992}, {0x9a9, 0x9a9}, {0x9b1, 0x9b1}, {0x9b3, 0x9b5}, {0x9ba, 0x9bc},
  {0x9be, 0x9cd}, {0x9cf, 0x9db}, {0x9de, 0x9de}, {0x9e2, 0x9e5}, {0x9f2, 0x9f3}, {0x9fa, 0x9fb}, {0x9fd, 0xa04}, {0xa0b, 0xa0e},
  {0xa11, 0xa12}, {0xa29, 0xa29}, {0xa31, 0xa31}, {0xa34, 0xa34}, {0xa37, 0xa37}, {0xa3a, 0xa58}, {0xa5d, 0xa5d}, {0xa5f, 0xa65},
  {0xa70, 0xa71}, {0xa75, 0xa84}, {0xa8e, 0xa8e}, {0xa92, 0xa92}, {0xaa9, 0xaa9}, {0xab1, 0xab1}, {0xab4, 0xab4}, {0xaba, 0xabc},
  {0xabe, 0xacf}, {0xad1, 0xadf}, {0xae2, 0xae5}, {0xaf0, 0xaf8}, {0xafa, 0xb04}, {0xb0d, 0xb0e}, {0xb11, 0xb12}, {0xb29, 0xb29},
  {0xb31, 0xb31}, {0xb34, 0xb34}, {0xb3a, 0xb3c}, {0xb3e, 0xb5b}, {0xb5e, 0xb5e}, {0xb62, 0xb65}, {0xb70, 0xb70}, {0xb78, 0xb82},
  {0xb84, 0xb84}, {0xb8b, 0xb8d}, {0xb91, 0xb91}, {0xb96, 0xb98}, {0xb9b, 0xb9b}, {0xb9d, 0xb9d}, {0xba0, 0xba2}, {0xba5, 0xba7},
  {0xbab, 0xbad}, {0xbba, 0xbcf}, {0xbd1, 0xbe5}, {0xbf3, 0xc04}, {0xc0d, 0xc0d}, {0xc11, 0xc11}, {0xc29, 0xc29}, {0xc3a, 0xc3c},
  {0xc3e, 0xc57}, {0xc5b, 0xc5c}, {0xc5e, 0xc5f}, {0xc62, 0xc65}, {0xc70, 0xc77}, {0xc7f, 0xc7f}, {0xc81, 0xc84}, {0xc8d, 0xc8d},
  {0xc91, 0xc91}, {0xca9, 0xca9}, {0xcb4, 0xcb4}, {0xcba, 0xcbc}, {0xcbe, 0xcdc}, {0xcdf, 0xcdf}, {0xce2, 0xce5}, {0xcf0, 0xcf0},
  {0xcf3, 0xd03}, {0xd0d, 0xd0d}, {0xd11, 0xd11}, {0xd3b, 0xd3c}, {0xd3e, 0xd4d}, {0xd4f, 0xd53}, {0xd57, 0xd57}, {0xd62, 0xd65},
  {0xd79, 0xd79}, {0xd80, 0xd84}, {0xd97, 0xd99}, {0xdb2, 0xdb2}, {0xdbc, 0xdbc}, {0xdbe, 0xdbf}, {0xdc7, 0xde5}, {0xdf0, 0xe00},
  {0xe31, 0xe31}, {0xe34, 0xe3f}, {0xe47, 0xe4f}, {0xe5a, 0xe80}, {0xe83, 0xe83}, {0xe85, 0xe85}, {0xe8b, 0xe8b}, {0xea4, 0xea4},
  {0xea6, 0xea6}, {0xeb1, 0xeb1}, {0xeb4, 0xebc}, {0xebe, 0xebf}, {0xec5, 0xec5}, {0xec7, 0xecf}, {0xeda, 0xedb}, {0xee0, 0xeff},
  {0xf01, 0xf1f}, {0xf34, 0xf3f}, {0xf48, 0xf48}, {0xf6d, 0xf87}, {0xf8d, 0xfff}, {0x102b, 0x103e}, {0x104a, 0x104f}, {0x1056, 0x1059},
  {0x105e, 0x1060}, {0x1062, 0x1064}, {0x1067, 0x106d}, {0x1071, 0x1074}, {0x1082, 0x108d}, {0x108f, 0x108f}, {0x109a, 0x109f}, {0x10c6, 0x10c6},
  {0x10c8, 0x10cc}, {0x10ce, 0x10cf}, {0x10fb, 0x10fb}, {0x1249, 0x1249}, {0x124e, 0x124f}, {0x1257, 0x1257}, {0x1259, 0x1259}, {0x125e, 0x125f},
  {0x1289, 0x1289}, {0x128e, 0x128f}, {0x12b1, 0x12b1}, {0x12b6, 0x12b7}, {0x12bf, 0x12bf}, {0x12c1, 0x12c1}, {0x12c6, 0x12c7}, {0x12d7, 0x12d7},
  {0x1311, 0x1311}, {0x1316, 0x1317}, {0x135b, 0x1368}, {0x137d, 0x137f}, {0x1390, 0x139f}, {0x13f6, 0x13f7}, {0x13fe, 0x1400}, {0x166d, 0x166e},
  {0x169b, 0x169f}, {0x16eb, 0x16ed}, {0x16f9, 0x16ff}, {0x1712, 0x171e}, {0x1732, 0x173f}, {0x1752, 0x175f}, {0x176d, 0x176d}, {0x1771, 0x177f},
  {0x17b4, 0x17d6}, {0x17d8, 0x17db}, {0x17dd, 0x17df}, {0x17ea, 0x17ef}, {0x17fa, 0x180f}, {0x181a, 0x181f}, {0x1879, 0x187f}, {0x1885, 0x1886},
  {0x18a9, 0x18a9}, {0x18ab, 0x18af}, {0x18f6, 0x18ff}, {0x191f, 0x1945}, {0x196e, 0x196f}, {0x1975, 0x197f}, {0x19ac, 0x19af}, {0x19ca, 0x19cf},
  {0x19db, 0x19ff}, {0x1a17, 0x1a1f}, {0x1a55, 0x1a7f}, {0x1a8a, 0x1a8f}, {0x1a9a, 0x1aa6}, {0x1aa8, 0x1b04}, {0x1b34, 0x1b44}, {0x1b4d, 0x1b4f},
  {0x1b5a, 0x1b82}, {0x1ba1, 0x1bad}, {0x1be6, 0x1bff}, {0x1c24, 0x1c3f}, {0x1c4a, 0x1c4c}, {0x1c7e, 0x1c7f}, {0x1c89, 0x1c8f}, {0x1cbb, 0x1cbc},
  {0x1cc0, 0x1ce8}, {0x1ced, 0x1ced}, {0x1cf4, 0x1cf4}, {0x1cf7, 0x1cf9}, {0x1cfb, 0x1cff}, {0x1dc0, 0x1dff}, {0x1f16, 0x1f17}, {0x1f1e, 0x1f1f},
  {0x1f46, 0x1f47}, {0x1f4e, 0x1f4f}, {0x1f58, 0x1f58}, {0x1f5a, 0x1f5a}, {0x1f5c, 0x1f5c}, {0x1f5e, 0x1f5e}, {0x1f7e, 0x1f7f}, {0x1fb5, 0x1fb5},
  {0x1fbd, 0x1fbd}, {0x1fbf, 0x1fc1}, {0x1fc5, 0x1fc5}, {0x1fcd, 0x1fcf}, {0x1fd4, 0x1fd5}, {0x1fdc, 0x1fdf}, {0x1fed, 0x1ff1}, {0x1ff5, 0x1ff5},
  {0x1ffd, 0x1fff}, {0x200b, 0x2027}, {0x202a, 0x202e}, {0x2030, 0x205e}, {0x2060, 0x206f}, {0x2072, 0x2073}, {0x207a, 0x207e}, {0x208a, 0x208f},
  {0x209d, 0x2101}, {0x2103, 0x2106}, {0x2108, 0x2109}, {0x2114, 0x2114}, {0x2116, 0x2118}, {0x211e, 0x2123}, {0x2125, 0x2125}, {0x2127, 0x2127},
  {0x2129, 0x2129}, {0x212e, 0x212e}, {0x213a, 0x213b}, {0x2140, 0x2144}, {0x214a, 0x214d}, {0x214f, 0x214f}, {0x218a, 0x245f}, {0x249c, 0x24e9},
  {0x2500, 0x2775}, {0x2794, 0x2bff}, {0x2ce5, 0x2cea}, {0x2cef, 0x2cf1}, {0x2cf4, 0x2cfc}, {0x2cfe, 0x2cff}, {0x2d26, 0x2d26}, {0x2d28, 0x2d2c},
  {0x2d2e, 0x2d2f}, {0x2d68, 0x2d6e}, {0x2d70, 0x2d7f}, {0x2d97, 0x2d9f}, {0x2da7, 0x2da7}, {0x2daf, 0x2daf}, {0x2db7, 0x2db7}, {0x2dbf, 0x2dbf},
  {0x2dc7, 0x2dc7}, {0x2dcf, 0x2dcf}, {0x2dd7, 0x2dd7}, {0x2ddf, 0x2e2e}, {0x2e30, 0x2fff}, {0x3001, 0x3004}, {0x3008, 0x3020}, {0x302a, 0x3030},
  {0x3036, 0x3037}, {0x303d, 0x3040}, {0x3097, 0x309c}, {0x30a0, 0x30a0}, {0x30fb, 0x30fb}, {0x3100, 0x3104}, {0x3130, 0x3130}, {0x318f, 0x3191},
  {0x3196, 0x319f}, {0x31c0, 0x31ef}, {0x3200, 0x321f}, {0x322a, 0x3247}, {0x3250, 0x3250}, {0x3260, 0x327f}, {0x328a, 0x32b0}, {0x32c0, 0x33ff},
  {0x3401, 0x4dbe}, {0x4dc0, 0x4dff}, {0x4e01, 0x9ffe}, {0xa48d, 0xa4cf}, {0xa4fe, 0xa4ff}, {0xa60d, 0xa60f}, {0xa62c, 0xa63f}, {0xa66f, 0xa67e},
  {0xa69e, 0xa69f}, {0xa6f0, 0xa716}, {0xa720, 0xa721}, {0xa789, 0xa78a}, {0xa7cb, 0xa7cf}, {0xa7d2, 0xa7d2}, {0xa7d4, 0xa7d4}, {0xa7da, 0xa7f1},
  {0xa802, 0xa802}, {0xa806, 0xa806}, {0xa80b, 0xa80b}, {0xa823, 0xa82f}, {0xa836, 0xa83f}, {0xa874, 0xa881}, {0xa8b4, 0xa8cf}, {0xa8da, 0xa8f1},
  {0xa8f8, 0xa8fa}, {0xa8fc, 0xa8fc}, {0xa8ff, 0xa8ff}, {0xa926, 0xa92f}, {0xa947, 0xa95f}, {0xa97d, 0xa983}, {0xa9b3, 0xa9ce}, {0xa9da, 0xa9df},
  {0xa9e5, 0xa9e5}, {0xa9ff, 0xa9ff}, {0xaa29, 0xaa3f}, {0xaa43, 0xaa43}, {0xaa4c, 0xaa4f}, {0xaa5a, 0xaa5f}, {0xaa77, 0xaa79}, {0xaa7b, 0xaa7d},
  {0xaab0, 0xaab0}, {0xaab2, 0xaab4}, {0xaab7, 0xaab8}, {0xaabe, 0xaabf}, {0xaac1, 0xaac1}, {0xaac3, 0xaada}, {0xaade, 0xaadf}, {0xaaeb, 0xaaf1},
  {0xaaf5, 0xab00}, {0xab07, 0xab08}, {0xab0f, 0xab10}, {0xab17, 0xab1f}, {0xab27, 0xab27}, {0xab2f, 0xab2f}, {0xab5b, 0xab5b}, {0xab6a, 0xab6f},
  {0xabe3, 0xabef}, {0xabfa, 0xabff}, {0xac01, 0xd7a2}, {0xd7a4, 0xd7af}, {0xd7c7, 0xd7ca}, {0xd7fc, 0xf8ff}, {0xfa6e, 0xfa6f}, {0xfada, 0xfaff},
  {0xfb07, 0xfb12}, {0xfb18, 0xfb1c}, {0xfb1e, 0xfb1e}, {0xfb29, 0xfb29}, {0xfb37, 0xfb37}, {0xfb3d, 0xfb3d}, {0xfb3f, 0xfb3f}, {0xfb42, 0xfb42},
  {0xfb45, 0xfb45}, {0xfbb2, 0xfbd2}, {0xfd3e, 0xfd4f}, {0xfd90, 0xfd91}, {0xfdc8, 0xfdef}, {0xfdfc, 0xfe6f}, {0xfe75, 0xfe75}, {0xfefd, 0xff0f},
  {0xff1a, 0xff20}, {0xff3b, 0xff40}, {0xff5b, 0xff65}, {0xffbf, 0xffc1}, {0xffc8, 0xffc9}, {0xffd0, 0xffd1}, {0xffd8, 0xffd9}, {0xffdd, 0xffff},
  {0x1000c, 0x1000c}, {0x10027, 0x10027}, {0x1003b, 0x1003b}, {0x1003e, 0x1003e}, {0x1004e, 0x1004f}, {0x1005e, 0x1007f}, {0x100fb, 0x10106}, {0x10134, 0x1013f},
  {0x10179, 0x10189}, {0x1018c, 0x1027f}, {0x1029d, 0x1029f}, {0x102d1, 0x102e0}, {0x102fc, 0x102ff}, {0x10324, 0x1032c}, {0x1034b, 0x1034f}, {0x10376, 0x1037f},
  {0x1039e, 0x1039f}, {0x103c4, 0x103c7}, {0x103d0, 0x103d0}, {0x103d6, 0x103ff}, {0x1049e, 0x1049f}, {0x104aa, 0x104af}, {0x104d4, 0x104d7}, {0x104fc, 0x104ff},
  {0x10528, 0x1052f}, {0x10564, 0x1056f}, {0x1057b, 0x1057b}, {0x1058b, 0x1058b}, {0x10593, 0x10593}, {0x10596, 0x10596}, {0x105a2, 0x105a2}, {0x105b2, 0x105b2},
  {0x105ba, 0x105ba}, {0x105bd, 0x105ff}, {0x10737, 0x1073f}, {0x10756, 0x1075f}, {0x10768, 0x1077f}, {0x10786, 0x10786}, {0x107b1, 0x107b1}, {0x107bb, 0x107ff},
  {0x10806, 0x10807}, {0x10809, 0x10809}, {0x10836, 0x10836}, {0x10839, 0x1083b}, {0x1083d, 0x1083e}, {0x10856, 0x10857}, {0x10877, 0x10878}, {0x1089f, 0x108a6},
  {0x108b0, 0x108df}, {0x108f3, 0x108f3}, {0x108f6, 0x108fa}, {0x1091c, 0x10ffff},
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(806);
      ADVANCE_MAP(
        0, 961,
        '\n', 961,
        '\r', 962,
        '#', 906,
        '-', 886,
        '/', 887,
        ':', 924,
        '<', 955,
        '=', 909,
        '>', 932,
        '@', 908,
        'C', 856,
        'D', 846,
        'G', 847,
        'H', 852,
        'L', 854,
        'O', 858,
        'P', 844,
        'T', 860,
        'W', 848,
        '[', 889,
        '\\', 891,
        '_', 890,
        '{', 885,
      );
      if (lookahead == ' ' ||
          lookahead == 0xa0 ||
          lookahead == 0x1680 ||
          (0x2000 <= lookahead && lookahead <= 0x200a) ||
          lookahead == 0x202f ||
          lookahead == 0x205f ||
          lookahead == 0x3000) ADVANCE(910);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(862);
      if (set_contains(aux_sym_WORD_CHAR_token1_character_set_1, 475, lookahead)) ADVANCE(807);
      if (lookahead != 0 &&
          lookahead != 0x2028 &&
          lookahead != 0x2029) ADVANCE(863);
      END_STATE();
    case 1:
      if ((!eof && lookahead == 00)) ADVANCE(863);
      if (lookahead == '\n') ADVANCE(897);
      if (lookahead == '\r') ADVANCE(899);
      if (lookahead == '\\') ADVANCE(891);
      if (lookahead == '{') ADVANCE(888);
      if (lookahead == ' ' ||
          lookahead == 0xa0 ||
          lookahead == 0x1680 ||
          (0x2000 <= lookahead && lookahead <= 0x200a) ||
          lookahead == 0x202f ||
          lookahead == 0x205f ||
          lookahead == 0x3000) ADVANCE(896);
      if (set_contains(aux_sym_WORD_CHAR_token1_character_set_1, 475, lookahead)) ADVANCE(807);
      if (lookahead != 0 &&
          lookahead != 0x2028 &&
          lookahead != 0x2029) ADVANCE(863);
      END_STATE();
    case 2:
      if ((!eof && lookahead == 00)) ADVANCE(863);
      if (lookahead == '\n') ADVANCE(897);
      if (lookahead == '\r') ADVANCE(899);
      if (lookahead == '{') ADVANCE(888);
      if (lookahead == ' ' ||
          lookahead == 0xa0 ||
          lookahead == 0x1680 ||
          (0x2000 <= lookahead && lookahead <= 0x200a) ||
          lookahead == 0x202f ||
          lookahead == 0x205f ||
          lookahead == 0x3000) ADVANCE(910);
      if (set_contains(aux_sym_WORD_CHAR_token1_character_set_1, 475, lookahead)) ADVANCE(807);
      if (lookahead != 0 &&
          lookahead != 0x2028 &&
          lookahead != 0x2029) ADVANCE(863);
      END_STATE();
    case 3:
      if ((!eof && lookahead == 00)) ADVANCE(863);
      if (lookahead == '\n') ADVANCE(897);
      if (lookahead == '\r') ADVANCE(899);
      if (lookahead == '{') ADVANCE(888);
      if (lookahead == ' ' ||
          lookahead == 0xa0 ||
          lookahead == 0x1680 ||
          (0x2000 <= lookahead && lookahead <= 0x200a) ||
          lookahead == 0x202f ||
          lookahead == 0x205f ||
          lookahead == 0x3000) ADVANCE(896);
      if (set_contains(aux_sym_WORD_CHAR_token1_character_set_1, 475, lookahead)) ADVANCE(807);
      if (lookahead != 0 &&
          lookahead != 0x2028 &&
          lookahead != 0x2029) ADVANCE(863);
      END_STATE();
    case 4:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == '#') ADVANCE(911);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 5:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == '/') ADVANCE(50);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 6:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'A') ADVANCE(30);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 7:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'A') ADVANCE(14);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 8:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'A') ADVANCE(13);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 9:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'B') ADVANCE(35);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 10:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'C') ADVANCE(36);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 11:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'C') ADVANCE(22);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 12:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'C') ADVANCE(19);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 13:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'C') ADVANCE(16);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 14:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'D') ADVANCE(914);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 15:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'E') ADVANCE(36);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 16:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'E') ADVANCE(914);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 17:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'E') ADVANCE(10);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 18:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'E') ADVANCE(40);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 19:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'H') ADVANCE(914);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 20:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'H') ADVANCE(32);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 21:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'I') ADVANCE(28);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 22:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'K') ADVANCE(15);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 23:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'L') ADVANCE(18);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 24:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'L') ADVANCE(914);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 25:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'N') ADVANCE(26);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 26:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'N') ADVANCE(17);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 27:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'N') ADVANCE(34);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 28:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'O') ADVANCE(27);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 29:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'O') ADVANCE(11);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 30:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'P') ADVANCE(20);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 31:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'P') ADVANCE(5);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 32:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'Q') ADVANCE(24);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 33:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'S') ADVANCE(36);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 34:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'S') ADVANCE(914);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 35:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'S') ADVANCE(29);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 36:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'T') ADVANCE(914);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 37:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'T') ADVANCE(21);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 38:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'T') ADVANCE(12);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 39:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'T') ADVANCE(31);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 40:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'T') ADVANCE(16);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 41:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'a') ADVANCE(48);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 42:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'e') ADVANCE(46);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 43:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'i') ADVANCE(45);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 44:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'n') ADVANCE(51);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 45:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'o') ADVANCE(44);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 46:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'r') ADVANCE(49);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 47:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 't') ADVANCE(41);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 48:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 't') ADVANCE(43);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 49:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'y') ADVANCE(51);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 50:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == '.' ||
          ('0' <= lookahead && lookahead <= '9')) ADVANCE(918);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 51:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == ' ' ||
          lookahead == 0xa0 ||
          lookahead == 0x1680 ||
          (0x2000 <= lookahead && lookahead <= 0x200a) ||
          lookahead == 0x202f ||
          lookahead == 0x205f ||
          lookahead == 0x3000) ADVANCE(53);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 52:
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 53:
      if ((!eof && lookahead == 00)) ADVANCE(965);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(965);
      if (lookahead == '{') ADVANCE(55);
      if (lookahead == ' ' ||
          lookahead == 0xa0 ||
          lookahead == 0x1680 ||
          (0x2000 <= lookahead && lookahead <= 0x200a) ||
          lookahead == 0x202f ||
          lookahead == 0x205f ||
          lookahead == 0x3000) ADVANCE(53);
      if (lookahead != 0) ADVANCE(54);
      END_STATE();
    case 54:
      if ((!eof && lookahead == 00)) ADVANCE(965);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(965);
      if (lookahead == '{') ADVANCE(55);
      if (lookahead != 0) ADVANCE(54);
      END_STATE();
    case 55:
      if ((!eof && lookahead == 00)) ADVANCE(948);
      if (lookahead == '\n') ADVANCE(946);
      if (lookahead == '\r') ADVANCE(947);
      if (lookahead == '{') ADVANCE(55);
      if (lookahead != 0) ADVANCE(54);
      END_STATE();
    case 56:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == '#') ADVANCE(911);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 57:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == '/') ADVANCE(102);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 58:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'A') ADVANCE(66);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 59:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'A') ADVANCE(83);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 60:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'A') ADVANCE(65);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 61:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'B') ADVANCE(86);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 62:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'C') ADVANCE(74);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 63:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'C') ADVANCE(71);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 64:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'C') ADVANCE(88);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 65:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'C') ADVANCE(67);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 66:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'D') ADVANCE(915);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 67:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'E') ADVANCE(915);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 68:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'E') ADVANCE(88);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 69:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'E') ADVANCE(64);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 70:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'E') ADVANCE(92);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 71:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'H') ADVANCE(915);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 72:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'H') ADVANCE(84);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 73:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'I') ADVANCE(80);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 74:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'K') ADVANCE(68);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 75:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'L') ADVANCE(915);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 76:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'L') ADVANCE(70);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 77:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'N') ADVANCE(79);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 78:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'N') ADVANCE(85);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 79:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'N') ADVANCE(69);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 80:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'O') ADVANCE(78);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 81:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'O') ADVANCE(62);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 82:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'P') ADVANCE(57);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 83:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'P') ADVANCE(72);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 84:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'Q') ADVANCE(75);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 85:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'S') ADVANCE(915);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 86:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'S') ADVANCE(81);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 87:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'S') ADVANCE(88);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 88:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'T') ADVANCE(915);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 89:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'T') ADVANCE(73);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 90:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'T') ADVANCE(82);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 91:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'T') ADVANCE(63);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 92:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'T') ADVANCE(67);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 93:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'a') ADVANCE(100);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 94:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'e') ADVANCE(98);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 95:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'i') ADVANCE(97);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 96:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'n') ADVANCE(103);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 97:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'o') ADVANCE(96);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 98:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'r') ADVANCE(101);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 99:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 't') ADVANCE(93);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 100:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 't') ADVANCE(95);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 101:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'y') ADVANCE(103);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 102:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == '.' ||
          ('0' <= lookahead && lookahead <= '9')) ADVANCE(919);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 103:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == ' ' ||
          lookahead == 0xa0 ||
          lookahead == 0x1680 ||
          (0x2000 <= lookahead && lookahead <= 0x200a) ||
          lookahead == 0x202f ||
          lookahead == 0x205f ||
          lookahead == 0x3000) ADVANCE(105);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 104:
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 105:
      if ((!eof && lookahead == 00)) ADVANCE(960);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(960);
      if (lookahead == '{') ADVANCE(165);
      if (lookahead == ' ' ||
          lookahead == 0xa0 ||
          lookahead == 0x1680 ||
          (0x2000 <= lookahead && lookahead <= 0x200a) ||
          lookahead == 0x202f ||
          lookahead == 0x205f ||
          lookahead == 0x3000) ADVANCE(105);
      if (lookahead != 0) ADVANCE(106);
      END_STATE();
    case 106:
      if ((!eof && lookahead == 00)) ADVANCE(960);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(960);
      if (lookahead == '{') ADVANCE(165);
      if (lookahead != 0) ADVANCE(106);
      END_STATE();
    case 107:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == '#') ADVANCE(911);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 108:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == '#') ADVANCE(905);
      if (lookahead == '/') ADVANCE(109);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 109:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == '/') ADVANCE(905);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 110:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == '/') ADVANCE(155);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 111:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'A') ADVANCE(119);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 112:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'A') ADVANCE(136);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 113:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'A') ADVANCE(117);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 114:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'B') ADVANCE(139);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 115:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'C') ADVANCE(127);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 116:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'C') ADVANCE(124);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 117:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'C') ADVANCE(120);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 118:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'C') ADVANCE(141);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 119:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'D') ADVANCE(916);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 120:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'E') ADVANCE(916);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 121:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'E') ADVANCE(141);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 122:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'E') ADVANCE(118);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 123:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'E') ADVANCE(145);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 124:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'H') ADVANCE(916);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 125:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'H') ADVANCE(137);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 126:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'I') ADVANCE(133);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 127:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'K') ADVANCE(121);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 128:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'L') ADVANCE(916);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 129:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'L') ADVANCE(123);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 130:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'N') ADVANCE(132);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 131:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'N') ADVANCE(138);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 132:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'N') ADVANCE(122);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 133:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'O') ADVANCE(131);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 134:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'O') ADVANCE(115);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 135:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'P') ADVANCE(110);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 136:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'P') ADVANCE(125);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 137:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'Q') ADVANCE(128);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 138:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'S') ADVANCE(916);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 139:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'S') ADVANCE(134);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 140:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'S') ADVANCE(141);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 141:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'T') ADVANCE(916);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 142:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'T') ADVANCE(126);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 143:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'T') ADVANCE(135);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 144:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'T') ADVANCE(116);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 145:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'T') ADVANCE(120);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 146:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'a') ADVANCE(153);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 147:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'e') ADVANCE(151);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 148:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'i') ADVANCE(150);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 149:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'n') ADVANCE(156);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 150:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'o') ADVANCE(149);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 151:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'r') ADVANCE(154);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 152:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 't') ADVANCE(146);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 153:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 't') ADVANCE(148);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 154:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'y') ADVANCE(156);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 155:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == '.' ||
          ('0' <= lookahead && lookahead <= '9')) ADVANCE(920);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 156:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == ' ' ||
          lookahead == 0xa0 ||
          lookahead == 0x1680 ||
          (0x2000 <= lookahead && lookahead <= 0x200a) ||
          lookahead == 0x202f ||
          lookahead == 0x205f ||
          lookahead == 0x3000) ADVANCE(158);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 157:
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 158:
      if ((!eof && lookahead == 00)) ADVANCE(969);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(969);
      if (lookahead == '{') ADVANCE(159);
      if (lookahead == ' ' ||
          lookahead == 0xa0 ||
          lookahead == 0x1680 ||
          (0x2000 <= lookahead && lookahead <= 0x200a) ||
          lookahead == 0x202f ||
          lookahead == 0x205f ||
          lookahead == 0x3000) ADVANCE(158);
      if (lookahead != 0) ADVANCE(159);
      END_STATE();
    case 159:
      if ((!eof && lookahead == 00)) ADVANCE(969);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(969);
      if (lookahead == '{') ADVANCE(159);
      if (lookahead != 0) ADVANCE(159);
      END_STATE();
    case 160:
      if ((!eof && lookahead == 00)) ADVANCE(883);
      if (lookahead == '\n') ADVANCE(931);
      if (lookahead == '\r') ADVANCE(931);
      if (lookahead == '\\') ADVANCE(891);
      if (lookahead == '{') ADVANCE(888);
      if (set_contains(aux_sym_WORD_CHAR_token1_character_set_1, 475, lookahead)) ADVANCE(807);
      if ((!eof && set_contains(aux_sym_PUNCTUATION_token1_character_set_1, 484, lookahead))) ADVANCE(863);
      END_STATE();
    case 161:
      if ((!eof && lookahead == 00)) ADVANCE(901);
      if (lookahead == '\n') ADVANCE(900);
      if (lookahead == '\r') ADVANCE(901);
      if (lookahead == '%') ADVANCE(163);
      if (lookahead != 0) ADVANCE(164);
      END_STATE();
    case 162:
      if ((!eof && lookahead == 00)) ADVANCE(901);
      if (lookahead == '\n') ADVANCE(900);
      if (lookahead == '\r') ADVANCE(901);
      if (lookahead == '@') ADVANCE(908);
      if (lookahead != 0) ADVANCE(164);
      END_STATE();
    case 163:
      if ((!eof && lookahead == 00)) ADVANCE(901);
      if (lookahead == '\n') ADVANCE(900);
      if (lookahead == '\r') ADVANCE(901);
      if (lookahead == '}') ADVANCE(934);
      if (lookahead != 0) ADVANCE(164);
      END_STATE();
    case 164:
      if ((!eof && lookahead == 00)) ADVANCE(901);
      if (lookahead == '\n') ADVANCE(900);
      if (lookahead == '\r') ADVANCE(901);
      if (lookahead != 0) ADVANCE(164);
      END_STATE();
    case 165:
      if ((!eof && lookahead == 00)) ADVANCE(950);
      if (lookahead == '\n') ADVANCE(946);
      if (lookahead == '\r') ADVANCE(949);
      if (lookahead == '{') ADVANCE(165);
      if (lookahead != 0) ADVANCE(106);
      END_STATE();
    case 166:
      if (lookahead == '\r') ADVANCE(1011);
      if ((!eof && lookahead == 00) ||
          lookahead == '\n') ADVANCE(1010);
      if (lookahead == '$' ||
          lookahead == '-' ||
          lookahead == '.' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa1 <= lookahead && lookahead <= 0xffff)) ADVANCE(1008);
      END_STATE();
    case 167:
      ADVANCE_MAP(
        '\r', 899,
        ':', 924,
        '=', 909,
        'A', 366,
        'B', 318,
        'C', 603,
        'E', 784,
        'F', 319,
        'G', 320,
        'H', 312,
        'I', 223,
        'L', 416,
        'M', 410,
        'N', 417,
        'O', 263,
        'P', 321,
        'R', 329,
        'S', 407,
        'T', 411,
        'U', 288,
        'V', 325,
        '}', 795,
        0, 897,
        '\n', 897,
      );
      if (('1' <= lookahead && lookahead <= '5')) ADVANCE(797);
      if (lookahead == ' ' ||
          lookahead == 0xa0 ||
          lookahead == 0x1680 ||
          (0x2000 <= lookahead && lookahead <= 0x200a) ||
          lookahead == 0x202f ||
          lookahead == 0x205f ||
          lookahead == 0x3000) ADVANCE(896);
      END_STATE();
    case 168:
      if (lookahead == '\r') ADVANCE(931);
      if ((!eof && lookahead == 00) ||
          lookahead == '\n') ADVANCE(931);
      if (lookahead == ' ' ||
          lookahead == 0xa0) ADVANCE(896);
      if (lookahead == 0x1680 ||
          (0x2000 <= lookahead && lookahead <= 0x200a) ||
          lookahead == 0x202f ||
          lookahead == 0x205f ||
          lookahead == 0x3000) ADVANCE(895);
      if (lookahead == '$' ||
          lookahead == '-' ||
          lookahead == '.' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa1 <= lookahead && lookahead <= 0xffff)) ADVANCE(1008);
      END_STATE();
    case 169:
      if (lookahead == '\r') ADVANCE(970);
      if (lookahead == '@') ADVANCE(908);
      if ((!eof && lookahead == 00) ||
          lookahead == '\n') ADVANCE(970);
      if (lookahead != 0) ADVANCE(170);
      END_STATE();
    case 170:
      if (lookahead == '\r') ADVANCE(970);
      if ((!eof && lookahead == 00) ||
          lookahead == '\n') ADVANCE(970);
      if (lookahead != 0 &&
          lookahead != '@') ADVANCE(170);
      END_STATE();
    case 171:
      if (lookahead == ' ') ADVANCE(314);
      END_STATE();
    case 172:
      if (lookahead == ' ') ADVANCE(256);
      END_STATE();
    case 173:
      if (lookahead == ' ') ADVANCE(231);
      END_STATE();
    case 174:
      if (lookahead == ' ') ADVANCE(251);
      END_STATE();
    case 175:
      if (lookahead == ' ') ADVANCE(316);
      END_STATE();
    case 176:
      if (lookahead == ' ') ADVANCE(291);
      END_STATE();
    case 177:
      if (lookahead == ' ') ADVANCE(259);
      END_STATE();
    case 178:
      if (lookahead == ' ') ADVANCE(253);
      END_STATE();
    case 179:
      if (lookahead == ' ') ADVANCE(270);
      END_STATE();
    case 180:
      if (lookahead == ' ') ADVANCE(254);
      END_STATE();
    case 181:
      if (lookahead == ' ') ADVANCE(243);
      END_STATE();
    case 182:
      if (lookahead == ' ') ADVANCE(315);
      END_STATE();
    case 183:
      if (lookahead == ' ') ADVANCE(292);
      END_STATE();
    case 184:
      if (lookahead == ' ') ADVANCE(255);
      END_STATE();
    case 185:
      if (lookahead == ' ') ADVANCE(269);
      END_STATE();
    case 186:
      if (lookahead == ' ') ADVANCE(262);
      END_STATE();
    case 187:
      if (lookahead == ' ') ADVANCE(280);
      END_STATE();
    case 188:
      if (lookahead == ' ') ADVANCE(252);
      END_STATE();
    case 189:
      if (lookahead == ' ') ADVANCE(274);
      END_STATE();
    case 190:
      if (lookahead == ' ') ADVANCE(268);
      END_STATE();
    case 191:
      if (lookahead == ' ') ADVANCE(285);
      END_STATE();
    case 192:
      if (lookahead == ' ') ADVANCE(285);
      if (lookahead == 'i') ADVANCE(575);
      END_STATE();
    case 193:
      if (lookahead == ' ') ADVANCE(336);
      END_STATE();
    case 194:
      if (lookahead == ' ') ADVANCE(275);
      END_STATE();
    case 195:
      if (lookahead == ' ') ADVANCE(267);
      END_STATE();
    case 196:
      if (lookahead == ' ') ADVANCE(233);
      END_STATE();
    case 197:
      if (lookahead == ' ') ADVANCE(299);
      END_STATE();
    case 198:
      if (lookahead == ' ') ADVANCE(250);
      END_STATE();
    case 199:
      if (lookahead == ' ') ADVANCE(284);
      END_STATE();
    case 200:
      if (lookahead == ' ') ADVANCE(240);
      END_STATE();
    case 201:
      if (lookahead == ' ') ADVANCE(240);
      if (lookahead == 'n') ADVANCE(224);
      if (lookahead == 't') ADVANCE(173);
      END_STATE();
    case 202:
      if (lookahead == ' ') ADVANCE(301);
      END_STATE();
    case 203:
      if (lookahead == ' ') ADVANCE(249);
      END_STATE();
    case 204:
      if (lookahead == ' ') ADVANCE(241);
      END_STATE();
    case 205:
      if (lookahead == ' ') ADVANCE(310);
      END_STATE();
    case 206:
      if (lookahead == ' ') ADVANCE(302);
      END_STATE();
    case 207:
      if (lookahead == ' ') ADVANCE(311);
      END_STATE();
    case 208:
      if (lookahead == ' ') ADVANCE(309);
      END_STATE();
    case 209:
      if (lookahead == ' ') ADVANCE(744);
      END_STATE();
    case 210:
      if (lookahead == ' ') ADVANCE(293);
      END_STATE();
    case 211:
      if (lookahead == ' ') ADVANCE(244);
      END_STATE();
    case 212:
      if (lookahead == ' ') ADVANCE(232);
      END_STATE();
    case 213:
      if (lookahead == ' ') ADVANCE(298);
      END_STATE();
    case 214:
      if (lookahead == ' ') ADVANCE(234);
      END_STATE();
    case 215:
      if (lookahead == ' ') ADVANCE(290);
      END_STATE();
    case 216:
      if (lookahead == ' ') ADVANCE(294);
      END_STATE();
    case 217:
      if (lookahead == ' ') ADVANCE(286);
      END_STATE();
    case 218:
      if (lookahead == ' ') ADVANCE(276);
      END_STATE();
    case 219:
      if (lookahead == ' ') ADVANCE(313);
      END_STATE();
    case 220:
      if (lookahead == ' ') ADVANCE(289);
      END_STATE();
    case 221:
      if (lookahead == ' ') ADVANCE(277);
      END_STATE();
    case 222:
      if (lookahead == '#') ADVANCE(911);
      END_STATE();
    case 223:
      if (lookahead == '\'') ADVANCE(562);
      if (lookahead == 'M') ADVANCE(171);
      if (lookahead == 'n') ADVANCE(697);
      END_STATE();
    case 224:
      if (lookahead == '-') ADVANCE(230);
      END_STATE();
    case 225:
      if (lookahead == '-') ADVANCE(300);
      if (lookahead == 'p') ADVANCE(557);
      END_STATE();
    case 226:
      if (lookahead == '/') ADVANCE(798);
      END_STATE();
    case 227:
      if (lookahead == 'A') ADVANCE(281);
      END_STATE();
    case 228:
      if (lookahead == 'A') ADVANCE(242);
      END_STATE();
    case 229:
      if (lookahead == 'A') ADVANCE(239);
      END_STATE();
    case 230:
      if (lookahead == 'A') ADVANCE(771);
      END_STATE();
    case 231:
      if (lookahead == 'A') ADVANCE(383);
      if (lookahead == 'E') ADVANCE(786);
      if (lookahead == 'F') ADVANCE(606);
      if (lookahead == 'I') ADVANCE(564);
      if (lookahead == 'M') ADVANCE(630);
      END_STATE();
    case 232:
      if (lookahead == 'A') ADVANCE(769);
      END_STATE();
    case 233:
      if (lookahead == 'A') ADVANCE(545);
      END_STATE();
    case 234:
      if (lookahead == 'A') ADVANCE(548);
      END_STATE();
    case 235:
      if (lookahead == 'B') ADVANCE(297);
      END_STATE();
    case 236:
      if (lookahead == 'C') ADVANCE(303);
      END_STATE();
    case 237:
      if (lookahead == 'C') ADVANCE(264);
      END_STATE();
    case 238:
      if (lookahead == 'C') ADVANCE(257);
      END_STATE();
    case 239:
      if (lookahead == 'C') ADVANCE(246);
      END_STATE();
    case 240:
      if (lookahead == 'C') ADVANCE(610);
      END_STATE();
    case 241:
      if (lookahead == 'C') ADVANCE(495);
      END_STATE();
    case 242:
      if (lookahead == 'D') ADVANCE(913);
      END_STATE();
    case 243:
      if (lookahead == 'D') ADVANCE(466);
      END_STATE();
    case 244:
      if (lookahead == 'D') ADVANCE(454);
      END_STATE();
    case 245:
      if (lookahead == 'E') ADVANCE(303);
      END_STATE();
    case 246:
      if (lookahead == 'E') ADVANCE(913);
      END_STATE();
    case 247:
      if (lookahead == 'E') ADVANCE(236);
      END_STATE();
    case 248:
      if (lookahead == 'E') ADVANCE(307);
      END_STATE();
    case 249:
      if (lookahead == 'E') ADVANCE(681);
      END_STATE();
    case 250:
      if (lookahead == 'E') ADVANCE(584);
      END_STATE();
    case 251:
      if (lookahead == 'E') ADVANCE(333);
      if (lookahead == 'M') ADVANCE(335);
      END_STATE();
    case 252:
      if (lookahead == 'F') ADVANCE(516);
      END_STATE();
    case 253:
      if (lookahead == 'F') ADVANCE(356);
      END_STATE();
    case 254:
      if (lookahead == 'F') ADVANCE(356);
      if (lookahead == 'R') ADVANCE(456);
      END_STATE();
    case 255:
      if (lookahead == 'F') ADVANCE(636);
      END_STATE();
    case 256:
      if (lookahead == 'G') ADVANCE(357);
      if (lookahead == 'R') ADVANCE(408);
      END_STATE();
    case 257:
      if (lookahead == 'H') ADVANCE(913);
      END_STATE();
    case 258:
      if (lookahead == 'H') ADVANCE(287);
      END_STATE();
    case 259:
      if (lookahead == 'H') ADVANCE(464);
      if (lookahead == 'T') ADVANCE(522);
      END_STATE();
    case 260:
      if (lookahead == 'I') ADVANCE(278);
      END_STATE();
    case 261:
      if (lookahead == 'I') ADVANCE(205);
      END_STATE();
    case 262:
      if (lookahead == 'I') ADVANCE(574);
      END_STATE();
    case 263:
      if (lookahead == 'K') ADVANCE(923);
      END_STATE();
    case 264:
      if (lookahead == 'K') ADVANCE(245);
      END_STATE();
    case 265:
      if (lookahead == 'L') ADVANCE(248);
      END_STATE();
    case 266:
      if (lookahead == 'L') ADVANCE(913);
      END_STATE();
    case 267:
      if (lookahead == 'L') ADVANCE(429);
      END_STATE();
    case 268:
      if (lookahead == 'L') ADVANCE(339);
      END_STATE();
    case 269:
      if (lookahead == 'L') ADVANCE(619);
      END_STATE();
    case 270:
      if (lookahead == 'M') ADVANCE(465);
      END_STATE();
    case 271:
      if (lookahead == 'N') ADVANCE(272);
      END_STATE();
    case 272:
      if (lookahead == 'N') ADVANCE(247);
      END_STATE();
    case 273:
      if (lookahead == 'N') ADVANCE(296);
      END_STATE();
    case 274:
      if (lookahead == 'N') ADVANCE(639);
      END_STATE();
    case 275:
      if (lookahead == 'N') ADVANCE(426);
      END_STATE();
    case 276:
      if (lookahead == 'N') ADVANCE(640);
      END_STATE();
    case 277:
      if (lookahead == 'N') ADVANCE(641);
      END_STATE();
    case 278:
      if (lookahead == 'O') ADVANCE(273);
      END_STATE();
    case 279:
      if (lookahead == 'O') ADVANCE(237);
      END_STATE();
    case 280:
      if (lookahead == 'O') ADVANCE(718);
      END_STATE();
    case 281:
      if (lookahead == 'P') ADVANCE(258);
      END_STATE();
    case 282:
      if (lookahead == 'P') ADVANCE(226);
      END_STATE();
    case 283:
      if (lookahead == 'P') ADVANCE(175);
      END_STATE();
    case 284:
      if (lookahead == 'P') ADVANCE(477);
      END_STATE();
    case 285:
      if (lookahead == 'P') ADVANCE(676);
      END_STATE();
    case 286:
      if (lookahead == 'P') ADVANCE(678);
      END_STATE();
    case 287:
      if (lookahead == 'Q') ADVANCE(266);
      END_STATE();
    case 288:
      if (lookahead == 'R') ADVANCE(261);
      if (lookahead == 'n') ADVANCE(322);
      if (lookahead == 'p') ADVANCE(485);
      if (lookahead == 's') ADVANCE(428);
      END_STATE();
    case 289:
      if (lookahead == 'R') ADVANCE(408);
      END_STATE();
    case 290:
      if (lookahead == 'R') ADVANCE(476);
      END_STATE();
    case 291:
      if (lookahead == 'R') ADVANCE(456);
      END_STATE();
    case 292:
      if (lookahead == 'R') ADVANCE(444);
      END_STATE();
    case 293:
      if (lookahead == 'R') ADVANCE(472);
      END_STATE();
    case 294:
      if (lookahead == 'R') ADVANCE(475);
      END_STATE();
    case 295:
      if (lookahead == 'S') ADVANCE(303);
      END_STATE();
    case 296:
      if (lookahead == 'S') ADVANCE(913);
      END_STATE();
    case 297:
      if (lookahead == 'S') ADVANCE(279);
      END_STATE();
    case 298:
      if (lookahead == 'S') ADVANCE(753);
      END_STATE();
    case 299:
      if (lookahead == 'S') ADVANCE(439);
      END_STATE();
    case 300:
      if (lookahead == 'S') ADVANCE(727);
      END_STATE();
    case 301:
      if (lookahead == 'S') ADVANCE(352);
      END_STATE();
    case 302:
      if (lookahead == 'S') ADVANCE(774);
      END_STATE();
    case 303:
      if (lookahead == 'T') ADVANCE(913);
      END_STATE();
    case 304:
      if (lookahead == 'T') ADVANCE(260);
      END_STATE();
    case 305:
      if (lookahead == 'T') ADVANCE(238);
      END_STATE();
    case 306:
      if (lookahead == 'T') ADVANCE(282);
      END_STATE();
    case 307:
      if (lookahead == 'T') ADVANCE(246);
      END_STATE();
    case 308:
      if (lookahead == 'T') ADVANCE(283);
      END_STATE();
    case 309:
      if (lookahead == 'T') ADVANCE(791);
      END_STATE();
    case 310:
      if (lookahead == 'T') ADVANCE(628);
      END_STATE();
    case 311:
      if (lookahead == 'T') ADVANCE(522);
      END_STATE();
    case 312:
      if (lookahead == 'T') ADVANCE(308);
      END_STATE();
    case 313:
      if (lookahead == 'T') ADVANCE(635);
      END_STATE();
    case 314:
      if (lookahead == 'U') ADVANCE(701);
      END_STATE();
    case 315:
      if (lookahead == 'U') ADVANCE(581);
      END_STATE();
    case 316:
      if (lookahead == 'V') ADVANCE(432);
      END_STATE();
    case 317:
      if (lookahead == '\\') ADVANCE(891);
      if (lookahead == '{') ADVANCE(884);
      if (set_contains(aux_sym_WORD_CHAR_token1_character_set_1, 475, lookahead)) ADVANCE(807);
      if ((!eof && set_contains(aux_sym_PUNCTUATION_token1_character_set_1, 484, lookahead))) ADVANCE(863);
      END_STATE();
    case 318:
      if (lookahead == 'a') ADVANCE(386);
      END_STATE();
    case 319:
      if (lookahead == 'a') ADVANCE(502);
      if (lookahead == 'o') ADVANCE(664);
      END_STATE();
    case 320:
      if (lookahead == 'a') ADVANCE(719);
      if (lookahead == 'o') ADVANCE(577);
      END_STATE();
    case 321:
      if (lookahead == 'a') ADVANCE(669);
      if (lookahead == 'e') ADVANCE(667);
      if (lookahead == 'r') ADVANCE(409);
      END_STATE();
    case 322:
      if (lookahead == 'a') ADVANCE(768);
      if (lookahead == 'p') ADVANCE(671);
      if (lookahead == 's') ADVANCE(763);
      END_STATE();
    case 323:
      if (lookahead == 'a') ADVANCE(364);
      END_STATE();
    case 324:
      if (lookahead == 'a') ADVANCE(787);
      END_STATE();
    case 325:
      if (lookahead == 'a') ADVANCE(666);
      END_STATE();
    case 326:
      if (lookahead == 'a') ADVANCE(779);
      END_STATE();
    case 327:
      if (lookahead == 'a') ADVANCE(363);
      END_STATE();
    case 328:
      if (lookahead == 'a') ADVANCE(397);
      END_STATE();
    case 329:
      if (lookahead == 'a') ADVANCE(573);
      if (lookahead == 'e') ADVANCE(659);
      END_STATE();
    case 330:
      if (lookahead == 'a') ADVANCE(552);
      END_STATE();
    case 331:
      if (lookahead == 'a') ADVANCE(400);
      END_STATE();
    case 332:
      if (lookahead == 'a') ADVANCE(486);
      END_STATE();
    case 333:
      if (lookahead == 'a') ADVANCE(674);
      END_STATE();
    case 334:
      if (lookahead == 'a') ADVANCE(693);
      END_STATE();
    case 335:
      if (lookahead == 'a') ADVANCE(596);
      END_STATE();
    case 336:
      if (lookahead == 'a') ADVANCE(209);
      END_STATE();
    case 337:
      if (lookahead == 'a') ADVANCE(708);
      END_STATE();
    case 338:
      if (lookahead == 'a') ADVANCE(722);
      END_STATE();
    case 339:
      if (lookahead == 'a') ADVANCE(672);
      END_STATE();
    case 340:
      if (lookahead == 'a') ADVANCE(208);
      END_STATE();
    case 341:
      if (lookahead == 'a') ADVANCE(507);
      END_STATE();
    case 342:
      if (lookahead == 'a') ADVANCE(725);
      END_STATE();
    case 343:
      if (lookahead == 'a') ADVANCE(601);
      END_STATE();
    case 344:
      if (lookahead == 'a') ADVANCE(649);
      END_STATE();
    case 345:
      if (lookahead == 'a') ADVANCE(726);
      END_STATE();
    case 346:
      if (lookahead == 'a') ADVANCE(554);
      END_STATE();
    case 347:
      if (lookahead == 'a') ADVANCE(523);
      END_STATE();
    case 348:
      if (lookahead == 'a') ADVANCE(789);
      END_STATE();
    case 349:
      if (lookahead == 'a') ADVANCE(587);
      END_STATE();
    case 350:
      if (lookahead == 'a') ADVANCE(556);
      END_STATE();
    case 351:
      if (lookahead == 'a') ADVANCE(395);
      END_STATE();
    case 352:
      if (lookahead == 'a') ADVANCE(732);
      END_STATE();
    case 353:
      if (lookahead == 'a') ADVANCE(403);
      END_STATE();
    case 354:
      if (lookahead == 'a') ADVANCE(735);
      END_STATE();
    case 355:
      if (lookahead == 'a') ADVANCE(742);
      END_STATE();
    case 356:
      if (lookahead == 'a') ADVANCE(529);
      END_STATE();
    case 357:
      if (lookahead == 'a') ADVANCE(748);
      END_STATE();
    case 358:
      if (lookahead == 'a') ADVANCE(598);
      END_STATE();
    case 359:
      if (lookahead == 'a') ADVANCE(365);
      END_STATE();
    case 360:
      if (lookahead == 'a') ADVANCE(759);
      END_STATE();
    case 361:
      if (lookahead == 'a') ADVANCE(760);
      END_STATE();
    case 362:
      if (lookahead == 'b') ADVANCE(503);
      END_STATE();
    case 363:
      if (lookahead == 'b') ADVANCE(543);
      END_STATE();
    case 364:
      if (lookahead == 'b') ADVANCE(558);
      END_STATE();
    case 365:
      if (lookahead == 'b') ADVANCE(559);
      END_STATE();
    case 366:
      if (lookahead == 'c') ADVANCE(369);
      if (lookahead == 'l') ADVANCE(668);
      END_STATE();
    case 367:
      if (lookahead == 'c') ADVANCE(538);
      if (lookahead == 'o') ADVANCE(648);
      END_STATE();
    case 368:
      if (lookahead == 'c') ADVANCE(787);
      END_STATE();
    case 369:
      if (lookahead == 'c') ADVANCE(419);
      END_STATE();
    case 370:
      if (lookahead == 'c') ADVANCE(492);
      END_STATE();
    case 371:
      if (lookahead == 'c') ADVANCE(724);
      END_STATE();
    case 372:
      if (lookahead == 'c') ADVANCE(712);
      END_STATE();
    case 373:
      if (lookahead == 'c') ADVANCE(418);
      if (lookahead == 'x') ADVANCE(788);
      END_STATE();
    case 374:
      if (lookahead == 'c') ADVANCE(537);
      END_STATE();
    case 375:
      if (lookahead == 'c') ADVANCE(455);
      END_STATE();
    case 376:
      if (lookahead == 'c') ADVANCE(612);
      END_STATE();
    case 377:
      if (lookahead == 'c') ADVANCE(431);
      END_STATE();
    case 378:
      if (lookahead == 'c') ADVANCE(725);
      END_STATE();
    case 379:
      if (lookahead == 'c') ADVANCE(425);
      END_STATE();
    case 380:
      if (lookahead == 'c') ADVANCE(611);
      END_STATE();
    case 381:
      if (lookahead == 'c') ADVANCE(459);
      END_STATE();
    case 382:
      if (lookahead == 'c') ADVANCE(755);
      END_STATE();
    case 383:
      if (lookahead == 'c') ADVANCE(379);
      END_STATE();
    case 384:
      if (lookahead == 'c') ADVANCE(360);
      END_STATE();
    case 385:
      if (lookahead == 'd') ADVANCE(923);
      END_STATE();
    case 386:
      if (lookahead == 'd') ADVANCE(172);
      END_STATE();
    case 387:
      if (lookahead == 'd') ADVANCE(518);
      END_STATE();
    case 388:
      if (lookahead == 'd') ADVANCE(393);
      END_STATE();
    case 389:
      if (lookahead == 'd') ADVANCE(415);
      END_STATE();
    case 390:
      if (lookahead == 'd') ADVANCE(519);
      END_STATE();
    case 391:
      if (lookahead == 'd') ADVANCE(710);
      END_STATE();
    case 392:
      if (lookahead == 'd') ADVANCE(199);
      END_STATE();
    case 393:
      if (lookahead == 'd') ADVANCE(436);
      END_STATE();
    case 394:
      if (lookahead == 'd') ADVANCE(179);
      END_STATE();
    case 395:
      if (lookahead == 'd') ADVANCE(442);
      END_STATE();
    case 396:
      if (lookahead == 'd') ADVANCE(447);
      END_STATE();
    case 397:
      if (lookahead == 'd') ADVANCE(790);
      END_STATE();
    case 398:
      if (lookahead == 'd') ADVANCE(535);
      END_STATE();
    case 399:
      if (lookahead == 'd') ADVANCE(211);
      END_STATE();
    case 400:
      if (lookahead == 'd') ADVANCE(443);
      END_STATE();
    case 401:
      if (lookahead == 'd') ADVANCE(521);
      END_STATE();
    case 402:
      if (lookahead == 'd') ADVANCE(528);
      END_STATE();
    case 403:
      if (lookahead == 'd') ADVANCE(219);
      END_STATE();
    case 404:
      if (lookahead == 'd') ADVANCE(218);
      END_STATE();
    case 405:
      if (lookahead == 'd') ADVANCE(220);
      END_STATE();
    case 406:
      if (lookahead == 'e') ADVANCE(923);
      END_STATE();
    case 407:
      if (lookahead == 'e') ADVANCE(422);
      if (lookahead == 'w') ADVANCE(506);
      END_STATE();
    case 408:
      if (lookahead == 'e') ADVANCE(661);
      END_STATE();
    case 409:
      if (lookahead == 'e') ADVANCE(380);
      if (lookahead == 'o') ADVANCE(373);
      END_STATE();
    case 410:
      if (lookahead == 'e') ADVANCE(713);
      if (lookahead == 'i') ADVANCE(698);
      if (lookahead == 'o') ADVANCE(776);
      if (lookahead == 'u') ADVANCE(541);
      END_STATE();
    case 411:
      if (lookahead == 'e') ADVANCE(561);
      if (lookahead == 'o') ADVANCE(607);
      END_STATE();
    case 412:
      if (lookahead == 'e') ADVANCE(781);
      END_STATE();
    case 413:
      if (lookahead == 'e') ADVANCE(342);
      END_STATE();
    case 414:
      if (lookahead == 'e') ADVANCE(371);
      END_STATE();
    case 415:
      if (lookahead == 'e') ADVANCE(385);
      END_STATE();
    case 416:
      if (lookahead == 'e') ADVANCE(571);
      if (lookahead == 'o') ADVANCE(367);
      END_STATE();
    case 417:
      if (lookahead == 'e') ADVANCE(714);
      if (lookahead == 'o') ADVANCE(201);
      END_STATE();
    case 418:
      if (lookahead == 'e') ADVANCE(699);
      END_STATE();
    case 419:
      if (lookahead == 'e') ADVANCE(650);
      END_STATE();
    case 420:
      if (lookahead == 'e') ADVANCE(565);
      END_STATE();
    case 421:
      if (lookahead == 'e') ADVANCE(706);
      END_STATE();
    case 422:
      if (lookahead == 'e') ADVANCE(187);
      if (lookahead == 'r') ADVANCE(775);
      END_STATE();
    case 423:
      if (lookahead == 'e') ADVANCE(673);
      END_STATE();
    case 424:
      if (lookahead == 'e') ADVANCE(382);
      END_STATE();
    case 425:
      if (lookahead == 'e') ADVANCE(654);
      END_STATE();
    case 426:
      if (lookahead == 'e') ADVANCE(488);
      END_STATE();
    case 427:
      if (lookahead == 'e') ADVANCE(729);
      END_STATE();
    case 428:
      if (lookahead == 'e') ADVANCE(191);
      END_STATE();
    case 429:
      if (lookahead == 'e') ADVANCE(490);
      END_STATE();
    case 430:
      if (lookahead == 'e') ADVANCE(591);
      END_STATE();
    case 431:
      if (lookahead == 'e') ADVANCE(696);
      END_STATE();
    case 432:
      if (lookahead == 'e') ADVANCE(670);
      END_STATE();
    case 433:
      if (lookahead == 'e') ADVANCE(344);
      END_STATE();
    case 434:
      if (lookahead == 'e') ADVANCE(544);
      END_STATE();
    case 435:
      if (lookahead == 'e') ADVANCE(663);
      END_STATE();
    case 436:
      if (lookahead == 'e') ADVANCE(569);
      END_STATE();
    case 437:
      if (lookahead == 'e') ADVANCE(580);
      END_STATE();
    case 438:
      if (lookahead == 'e') ADVANCE(189);
      END_STATE();
    case 439:
      if (lookahead == 'e') ADVANCE(686);
      END_STATE();
    case 440:
      if (lookahead == 'e') ADVANCE(590);
      END_STATE();
    case 441:
      if (lookahead == 'e') ADVANCE(632);
      END_STATE();
    case 442:
      if (lookahead == 'e') ADVANCE(682);
      END_STATE();
    case 443:
      if (lookahead == 'e') ADVANCE(176);
      END_STATE();
    case 444:
      if (lookahead == 'e') ADVANCE(337);
      END_STATE();
    case 445:
      if (lookahead == 'e') ADVANCE(595);
      END_STATE();
    case 446:
      if (lookahead == 'e') ADVANCE(582);
      END_STATE();
    case 447:
      if (lookahead == 'e') ADVANCE(578);
      END_STATE();
    case 448:
      if (lookahead == 'e') ADVANCE(204);
      END_STATE();
    case 449:
      if (lookahead == 'e') ADVANCE(184);
      END_STATE();
    case 450:
      if (lookahead == 'e') ADVANCE(198);
      END_STATE();
    case 451:
      if (lookahead == 'e') ADVANCE(186);
      END_STATE();
    case 452:
      if (lookahead == 'e') ADVANCE(328);
      END_STATE();
    case 453:
      if (lookahead == 'e') ADVANCE(392);
      END_STATE();
    case 454:
      if (lookahead == 'e') ADVANCE(656);
      END_STATE();
    case 455:
      if (lookahead == 'e') ADVANCE(182);
      END_STATE();
    case 456:
      if (lookahead == 'e') ADVANCE(660);
      END_STATE();
    case 457:
      if (lookahead == 'e') ADVANCE(783);
      END_STATE();
    case 458:
      if (lookahead == 'e') ADVANCE(372);
      END_STATE();
    case 459:
      if (lookahead == 'e') ADVANCE(700);
      END_STATE();
    case 460:
      if (lookahead == 'e') ADVANCE(702);
      END_STATE();
    case 461:
      if (lookahead == 'e') ADVANCE(683);
      END_STATE();
    case 462:
      if (lookahead == 'e') ADVANCE(399);
      END_STATE();
    case 463:
      if (lookahead == 'e') ADVANCE(703);
      END_STATE();
    case 464:
      if (lookahead == 'e') ADVANCE(351);
      END_STATE();
    case 465:
      if (lookahead == 'e') ADVANCE(401);
      END_STATE();
    case 466:
      if (lookahead == 'e') ADVANCE(754);
      END_STATE();
    case 467:
      if (lookahead == 'e') ADVANCE(405);
      END_STATE();
    case 468:
      if (lookahead == 'e') ADVANCE(394);
      END_STATE();
    case 469:
      if (lookahead == 'e') ADVANCE(378);
      END_STATE();
    case 470:
      if (lookahead == 'e') ADVANCE(588);
      END_STATE();
    case 471:
      if (lookahead == 'e') ADVANCE(597);
      END_STATE();
    case 472:
      if (lookahead == 'e') ADVANCE(657);
      END_STATE();
    case 473:
      if (lookahead == 'e') ADVANCE(599);
      END_STATE();
    case 474:
      if (lookahead == 'e') ADVANCE(600);
      END_STATE();
    case 475:
      if (lookahead == 'e') ADVANCE(662);
      END_STATE();
    case 476:
      if (lookahead == 'e') ADVANCE(402);
      END_STATE();
    case 477:
      if (lookahead == 'e') ADVANCE(694);
      END_STATE();
    case 478:
      if (lookahead == 'f') ADVANCE(480);
      END_STATE();
    case 479:
      if (lookahead == 'f') ADVANCE(546);
      if (lookahead == 't') ADVANCE(505);
      END_STATE();
    case 480:
      if (lookahead == 'f') ADVANCE(504);
      END_STATE();
    case 481:
      if (lookahead == 'f') ADVANCE(512);
      END_STATE();
    case 482:
      if (lookahead == 'f') ADVANCE(643);
      END_STATE();
    case 483:
      if (lookahead == 'f') ADVANCE(513);
      END_STATE();
    case 484:
      if (lookahead == 'g') ADVANCE(923);
      END_STATE();
    case 485:
      if (lookahead == 'g') ADVANCE(689);
      END_STATE();
    case 486:
      if (lookahead == 'g') ADVANCE(406);
      END_STATE();
    case 487:
      if (lookahead == 'g') ADVANCE(715);
      END_STATE();
    case 488:
      if (lookahead == 'g') ADVANCE(638);
      END_STATE();
    case 489:
      if (lookahead == 'g') ADVANCE(438);
      END_STATE();
    case 490:
      if (lookahead == 'g') ADVANCE(350);
      END_STATE();
    case 491:
      if (lookahead == 'g') ADVANCE(217);
      END_STATE();
    case 492:
      if (lookahead == 'h') ADVANCE(192);
      END_STATE();
    case 493:
      if (lookahead == 'h') ADVANCE(637);
      END_STATE();
    case 494:
      if (lookahead == 'h') ADVANCE(631);
      END_STATE();
    case 495:
      if (lookahead == 'h') ADVANCE(644);
      END_STATE();
    case 496:
      if (lookahead == 'h') ADVANCE(176);
      END_STATE();
    case 497:
      if (lookahead == 'h') ADVANCE(435);
      END_STATE();
    case 498:
      if (lookahead == 'h') ADVANCE(445);
      END_STATE();
    case 499:
      if (lookahead == 'h') ADVANCE(617);
      END_STATE();
    case 500:
      if (lookahead == 'i') ADVANCE(225);
      END_STATE();
    case 501:
      if (lookahead == 'i') ADVANCE(794);
      END_STATE();
    case 502:
      if (lookahead == 'i') ADVANCE(560);
      END_STATE();
    case 503:
      if (lookahead == 'i') ADVANCE(388);
      END_STATE();
    case 504:
      if (lookahead == 'i') ADVANCE(374);
      END_STATE();
    case 505:
      if (lookahead == 'i') ADVANCE(589);
      END_STATE();
    case 506:
      if (lookahead == 'i') ADVANCE(717);
      END_STATE();
    case 507:
      if (lookahead == 'i') ADVANCE(549);
      END_STATE();
    case 508:
      if (lookahead == 'i') ADVANCE(358);
      END_STATE();
    case 509:
      if (lookahead == 'i') ADVANCE(384);
      END_STATE();
    case 510:
      if (lookahead == 'i') ADVANCE(330);
      END_STATE();
    case 511:
      if (lookahead == 'i') ADVANCE(705);
      END_STATE();
    case 512:
      if (lookahead == 'i') ADVANCE(415);
      END_STATE();
    case 513:
      if (lookahead == 'i') ADVANCE(327);
      END_STATE();
    case 514:
      if (lookahead == 'i') ADVANCE(576);
      END_STATE();
    case 515:
      if (lookahead == 'i') ADVANCE(716);
      END_STATE();
    case 516:
      if (lookahead == 'i') ADVANCE(434);
      END_STATE();
    case 517:
      if (lookahead == 'i') ADVANCE(375);
      END_STATE();
    case 518:
      if (lookahead == 'i') ADVANCE(680);
      END_STATE();
    case 519:
      if (lookahead == 'i') ADVANCE(481);
      END_STATE();
    case 520:
      if (lookahead == 'i') ADVANCE(372);
      END_STATE();
    case 521:
      if (lookahead == 'i') ADVANCE(340);
      END_STATE();
    case 522:
      if (lookahead == 'i') ADVANCE(566);
      END_STATE();
    case 523:
      if (lookahead == 'i') ADVANCE(551);
      END_STATE();
    case 524:
      if (lookahead == 'i') ADVANCE(675);
      END_STATE();
    case 525:
      if (lookahead == 'i') ADVANCE(778);
      END_STATE();
    case 526:
      if (lookahead == 'i') ADVANCE(620);
      END_STATE();
    case 527:
      if (lookahead == 'i') ADVANCE(377);
      END_STATE();
    case 528:
      if (lookahead == 'i') ADVANCE(688);
      END_STATE();
    case 529:
      if (lookahead == 'i') ADVANCE(550);
      END_STATE();
    case 530:
      if (lookahead == 'i') ADVANCE(622);
      END_STATE();
    case 531:
      if (lookahead == 'i') ADVANCE(633);
      END_STATE();
    case 532:
      if (lookahead == 'i') ADVANCE(624);
      END_STATE();
    case 533:
      if (lookahead == 'i') ADVANCE(616);
      END_STATE();
    case 534:
      if (lookahead == 'i') ADVANCE(355);
      END_STATE();
    case 535:
      if (lookahead == 'i') ADVANCE(757);
      END_STATE();
    case 536:
      if (lookahead == 'i') ADVANCE(756);
      END_STATE();
    case 537:
      if (lookahead == 'i') ADVANCE(474);
      END_STATE();
    case 538:
      if (lookahead == 'k') ADVANCE(415);
      END_STATE();
    case 539:
      if (lookahead == 'k') ADVANCE(212);
      END_STATE();
    case 540:
      if (lookahead == 'l') ADVANCE(787);
      END_STATE();
    case 541:
      if (lookahead == 'l') ADVANCE(720);
      END_STATE();
    case 542:
      if (lookahead == 'l') ADVANCE(642);
      if (lookahead == 'm') ADVANCE(430);
      END_STATE();
    case 543:
      if (lookahead == 'l') ADVANCE(406);
      END_STATE();
    case 544:
      if (lookahead == 'l') ADVANCE(391);
      END_STATE();
    case 545:
      if (lookahead == 'l') ADVANCE(707);
      END_STATE();
    case 546:
      if (lookahead == 'l') ADVANCE(520);
      END_STATE();
    case 547:
      if (lookahead == 'l') ADVANCE(696);
      END_STATE();
    case 548:
      if (lookahead == 'l') ADVANCE(553);
      END_STATE();
    case 549:
      if (lookahead == 'l') ADVANCE(323);
      END_STATE();
    case 550:
      if (lookahead == 'l') ADVANCE(415);
      END_STATE();
    case 551:
      if (lookahead == 'l') ADVANCE(327);
      END_STATE();
    case 552:
      if (lookahead == 'l') ADVANCE(200);
      END_STATE();
    case 553:
      if (lookahead == 'l') ADVANCE(605);
      END_STATE();
    case 554:
      if (lookahead == 'l') ADVANCE(197);
      END_STATE();
    case 555:
      if (lookahead == 'l') ADVANCE(420);
      END_STATE();
    case 556:
      if (lookahead == 'l') ADVANCE(183);
      END_STATE();
    case 557:
      if (lookahead == 'l') ADVANCE(448);
      END_STATE();
    case 558:
      if (lookahead == 'l') ADVANCE(449);
      END_STATE();
    case 559:
      if (lookahead == 'l') ADVANCE(450);
      END_STATE();
    case 560:
      if (lookahead == 'l') ADVANCE(462);
      END_STATE();
    case 561:
      if (lookahead == 'm') ADVANCE(652);
      END_STATE();
    case 562:
      if (lookahead == 'm') ADVANCE(193);
      END_STATE();
    case 563:
      if (lookahead == 'm') ADVANCE(343);
      END_STATE();
    case 564:
      if (lookahead == 'm') ADVANCE(651);
      END_STATE();
    case 565:
      if (lookahead == 'm') ADVANCE(470);
      END_STATE();
    case 566:
      if (lookahead == 'm') ADVANCE(441);
      END_STATE();
    case 567:
      if (lookahead == 'm') ADVANCE(349);
      END_STATE();
    case 568:
      if (lookahead == 'm') ADVANCE(361);
      END_STATE();
    case 569:
      if (lookahead == 'n') ADVANCE(923);
      END_STATE();
    case 570:
      if (lookahead == 'n') ADVANCE(479);
      END_STATE();
    case 571:
      if (lookahead == 'n') ADVANCE(487);
      END_STATE();
    case 572:
      if (lookahead == 'n') ADVANCE(385);
      END_STATE();
    case 573:
      if (lookahead == 'n') ADVANCE(489);
      END_STATE();
    case 574:
      if (lookahead == 'n') ADVANCE(482);
      END_STATE();
    case 575:
      if (lookahead == 'n') ADVANCE(491);
      END_STATE();
    case 576:
      if (lookahead == 'n') ADVANCE(484);
      END_STATE();
    case 577:
      if (lookahead == 'n') ADVANCE(406);
      END_STATE();
    case 578:
      if (lookahead == 'n') ADVANCE(368);
      END_STATE();
    case 579:
      if (lookahead == 'n') ADVANCE(696);
      END_STATE();
    case 580:
      if (lookahead == 'n') ADVANCE(712);
      END_STATE();
    case 581:
      if (lookahead == 'n') ADVANCE(326);
      END_STATE();
    case 582:
      if (lookahead == 'n') ADVANCE(721);
      END_STATE();
    case 583:
      if (lookahead == 'n') ADVANCE(176);
      END_STATE();
    case 584:
      if (lookahead == 'n') ADVANCE(752);
      END_STATE();
    case 585:
      if (lookahead == 'n') ADVANCE(178);
      END_STATE();
    case 586:
      if (lookahead == 'n') ADVANCE(180);
      END_STATE();
    case 587:
      if (lookahead == 'n') ADVANCE(446);
      END_STATE();
    case 588:
      if (lookahead == 'n') ADVANCE(725);
      END_STATE();
    case 589:
      if (lookahead == 'n') ADVANCE(764);
      END_STATE();
    case 590:
      if (lookahead == 'n') ADVANCE(389);
      END_STATE();
    case 591:
      if (lookahead == 'n') ADVANCE(730);
      END_STATE();
    case 592:
      if (lookahead == 'n') ADVANCE(346);
      END_STATE();
    case 593:
      if (lookahead == 'n') ADVANCE(738);
      END_STATE();
    case 594:
      if (lookahead == 'n') ADVANCE(398);
      END_STATE();
    case 595:
      if (lookahead == 'n') ADVANCE(731);
      END_STATE();
    case 596:
      if (lookahead == 'n') ADVANCE(793);
      END_STATE();
    case 597:
      if (lookahead == 'n') ADVANCE(396);
      END_STATE();
    case 598:
      if (lookahead == 'n') ADVANCE(736);
      END_STATE();
    case 599:
      if (lookahead == 'n') ADVANCE(751);
      END_STATE();
    case 600:
      if (lookahead == 'n') ADVANCE(741);
      END_STATE();
    case 601:
      if (lookahead == 'n') ADVANCE(473);
      END_STATE();
    case 602:
      if (lookahead == 'n') ADVANCE(221);
      END_STATE();
    case 603:
      if (lookahead == 'o') ADVANCE(570);
      if (lookahead == 'r') ADVANCE(413);
      END_STATE();
    case 604:
      if (lookahead == 'o') ADVANCE(785);
      END_STATE();
    case 605:
      if (lookahead == 'o') ADVANCE(782);
      END_STATE();
    case 606:
      if (lookahead == 'o') ADVANCE(762);
      END_STATE();
    case 607:
      if (lookahead == 'o') ADVANCE(174);
      END_STATE();
    case 608:
      if (lookahead == 'o') ADVANCE(665);
      END_STATE();
    case 609:
      if (lookahead == 'o') ADVANCE(690);
      END_STATE();
    case 610:
      if (lookahead == 'o') ADVANCE(593);
      END_STATE();
    case 611:
      if (lookahead == 'o') ADVANCE(594);
      END_STATE();
    case 612:
      if (lookahead == 'o') ADVANCE(547);
      END_STATE();
    case 613:
      if (lookahead == 'o') ADVANCE(695);
      END_STATE();
    case 614:
      if (lookahead == 'o') ADVANCE(712);
      END_STATE();
    case 615:
      if (lookahead == 'o') ADVANCE(663);
      END_STATE();
    case 616:
      if (lookahead == 'o') ADVANCE(569);
      END_STATE();
    case 617:
      if (lookahead == 'o') ADVANCE(691);
      END_STATE();
    case 618:
      if (lookahead == 'o') ADVANCE(734);
      END_STATE();
    case 619:
      if (lookahead == 'o') ADVANCE(576);
      END_STATE();
    case 620:
      if (lookahead == 'o') ADVANCE(585);
      END_STATE();
    case 621:
      if (lookahead == 'o') ADVANCE(679);
      END_STATE();
    case 622:
      if (lookahead == 'o') ADVANCE(602);
      END_STATE();
    case 623:
      if (lookahead == 'o') ADVANCE(185);
      END_STATE();
    case 624:
      if (lookahead == 'o') ADVANCE(583);
      END_STATE();
    case 625:
      if (lookahead == 'o') ADVANCE(579);
      END_STATE();
    case 626:
      if (lookahead == 'o') ADVANCE(190);
      END_STATE();
    case 627:
      if (lookahead == 'o') ADVANCE(194);
      END_STATE();
    case 628:
      if (lookahead == 'o') ADVANCE(623);
      END_STATE();
    case 629:
      if (lookahead == 'o') ADVANCE(376);
      END_STATE();
    case 630:
      if (lookahead == 'o') ADVANCE(390);
      END_STATE();
    case 631:
      if (lookahead == 'o') ADVANCE(677);
      END_STATE();
    case 632:
      if (lookahead == 'o') ADVANCE(766);
      END_STATE();
    case 633:
      if (lookahead == 'o') ADVANCE(586);
      END_STATE();
    case 634:
      if (lookahead == 'o') ADVANCE(381);
      END_STATE();
    case 635:
      if (lookahead == 'o') ADVANCE(626);
      END_STATE();
    case 636:
      if (lookahead == 'o') ADVANCE(684);
      END_STATE();
    case 637:
      if (lookahead == 'o') ADVANCE(404);
      END_STATE();
    case 638:
      if (lookahead == 'o') ADVANCE(737);
      END_STATE();
    case 639:
      if (lookahead == 'o') ADVANCE(740);
      END_STATE();
    case 640:
      if (lookahead == 'o') ADVANCE(749);
      END_STATE();
    case 641:
      if (lookahead == 'o') ADVANCE(743);
      END_STATE();
    case 642:
      if (lookahead == 'o') ADVANCE(353);
      END_STATE();
    case 643:
      if (lookahead == 'o') ADVANCE(687);
      END_STATE();
    case 644:
      if (lookahead == 'o') ADVANCE(527);
      END_STATE();
    case 645:
      if (lookahead == 'o') ADVANCE(685);
      END_STATE();
    case 646:
      if (lookahead == 'p') ADVANCE(414);
      END_STATE();
    case 647:
      if (lookahead == 'p') ADVANCE(406);
      END_STATE();
    case 648:
      if (lookahead == 'p') ADVANCE(181);
      END_STATE();
    case 649:
      if (lookahead == 'p') ADVANCE(614);
      END_STATE();
    case 650:
      if (lookahead == 'p') ADVANCE(725);
      END_STATE();
    case 651:
      if (lookahead == 'p') ADVANCE(555);
      END_STATE();
    case 652:
      if (lookahead == 'p') ADVANCE(609);
      END_STATE();
    case 653:
      if (lookahead == 'p') ADVANCE(613);
      END_STATE();
    case 654:
      if (lookahead == 'p') ADVANCE(728);
      END_STATE();
    case 655:
      if (lookahead == 'p') ADVANCE(653);
      END_STATE();
    case 656:
      if (lookahead == 'p') ADVANCE(471);
      END_STATE();
    case 657:
      if (lookahead == 'p') ADVANCE(645);
      END_STATE();
    case 658:
      if (lookahead == 'p') ADVANCE(657);
      END_STATE();
    case 659:
      if (lookahead == 'q') ADVANCE(767);
      if (lookahead == 's') ADVANCE(427);
      END_STATE();
    case 660:
      if (lookahead == 'q') ADVANCE(773);
      END_STATE();
    case 661:
      if (lookahead == 'q') ADVANCE(770);
      END_STATE();
    case 662:
      if (lookahead == 'q') ADVANCE(772);
      END_STATE();
    case 663:
      if (lookahead == 'r') ADVANCE(923);
      END_STATE();
    case 664:
      if (lookahead == 'r') ADVANCE(362);
      if (lookahead == 'u') ADVANCE(572);
      END_STATE();
    case 665:
      if (lookahead == 'r') ADVANCE(539);
      END_STATE();
    case 666:
      if (lookahead == 'r') ADVANCE(508);
      END_STATE();
    case 667:
      if (lookahead == 'r') ADVANCE(563);
      END_STATE();
    case 668:
      if (lookahead == 'r') ADVANCE(452);
      END_STATE();
    case 669:
      if (lookahead == 'r') ADVANCE(746);
      if (lookahead == 'y') ADVANCE(542);
      END_STATE();
    case 670:
      if (lookahead == 'r') ADVANCE(709);
      END_STATE();
    case 671:
      if (lookahead == 'r') ADVANCE(634);
      END_STATE();
    case 672:
      if (lookahead == 'r') ADVANCE(486);
      END_STATE();
    case 673:
      if (lookahead == 'r') ADVANCE(592);
      END_STATE();
    case 674:
      if (lookahead == 'r') ADVANCE(540);
      END_STATE();
    case 675:
      if (lookahead == 'r') ADVANCE(415);
      END_STATE();
    case 676:
      if (lookahead == 'r') ADVANCE(604);
      END_STATE();
    case 677:
      if (lookahead == 'r') ADVANCE(501);
      END_STATE();
    case 678:
      if (lookahead == 'r') ADVANCE(618);
      END_STATE();
    case 679:
      if (lookahead == 'r') ADVANCE(332);
      END_STATE();
    case 680:
      if (lookahead == 'r') ADVANCE(424);
      END_STATE();
    case 681:
      if (lookahead == 'r') ADVANCE(692);
      END_STATE();
    case 682:
      if (lookahead == 'r') ADVANCE(188);
      END_STATE();
    case 683:
      if (lookahead == 'r') ADVANCE(203);
      END_STATE();
    case 684:
      if (lookahead == 'r') ADVANCE(195);
      END_STATE();
    case 685:
      if (lookahead == 'r') ADVANCE(725);
      END_STATE();
    case 686:
      if (lookahead == 'r') ADVANCE(777);
      END_STATE();
    case 687:
      if (lookahead == 'r') ADVANCE(568);
      END_STATE();
    case 688:
      if (lookahead == 'r') ADVANCE(458);
      END_STATE();
    case 689:
      if (lookahead == 'r') ADVANCE(331);
      END_STATE();
    case 690:
      if (lookahead == 'r') ADVANCE(334);
      END_STATE();
    case 691:
      if (lookahead == 'r') ADVANCE(536);
      END_STATE();
    case 692:
      if (lookahead == 'r') ADVANCE(615);
      END_STATE();
    case 693:
      if (lookahead == 'r') ADVANCE(792);
      END_STATE();
    case 694:
      if (lookahead == 'r') ADVANCE(567);
      END_STATE();
    case 695:
      if (lookahead == 'r') ADVANCE(758);
      END_STATE();
    case 696:
      if (lookahead == 's') ADVANCE(923);
      END_STATE();
    case 697:
      if (lookahead == 's') ADVANCE(761);
      if (lookahead == 't') ADVANCE(423);
      END_STATE();
    case 698:
      if (lookahead == 's') ADVANCE(387);
      END_STATE();
    case 699:
      if (lookahead == 's') ADVANCE(704);
      END_STATE();
    case 700:
      if (lookahead == 's') ADVANCE(711);
      END_STATE();
    case 701:
      if (lookahead == 's') ADVANCE(415);
      END_STATE();
    case 702:
      if (lookahead == 's') ADVANCE(712);
      END_STATE();
    case 703:
      if (lookahead == 's') ADVANCE(723);
      END_STATE();
    case 704:
      if (lookahead == 's') ADVANCE(514);
      END_STATE();
    case 705:
      if (lookahead == 's') ADVANCE(483);
      END_STATE();
    case 706:
      if (lookahead == 's') ADVANCE(733);
      END_STATE();
    case 707:
      if (lookahead == 's') ADVANCE(627);
      END_STATE();
    case 708:
      if (lookahead == 's') ADVANCE(625);
      END_STATE();
    case 709:
      if (lookahead == 's') ADVANCE(530);
      END_STATE();
    case 710:
      if (lookahead == 's') ADVANCE(219);
      END_STATE();
    case 711:
      if (lookahead == 's') ADVANCE(359);
      END_STATE();
    case 712:
      if (lookahead == 't') ADVANCE(923);
      END_STATE();
    case 713:
      if (lookahead == 't') ADVANCE(493);
      END_STATE();
    case 714:
      if (lookahead == 't') ADVANCE(780);
      END_STATE();
    case 715:
      if (lookahead == 't') ADVANCE(496);
      END_STATE();
    case 716:
      if (lookahead == 't') ADVANCE(787);
      END_STATE();
    case 717:
      if (lookahead == 't') ADVANCE(370);
      END_STATE();
    case 718:
      if (lookahead == 't') ADVANCE(497);
      END_STATE();
    case 719:
      if (lookahead == 't') ADVANCE(412);
      END_STATE();
    case 720:
      if (lookahead == 't') ADVANCE(500);
      END_STATE();
    case 721:
      if (lookahead == 't') ADVANCE(540);
      END_STATE();
    case 722:
      if (lookahead == 't') ADVANCE(765);
      END_STATE();
    case 723:
      if (lookahead == 't') ADVANCE(696);
      END_STATE();
    case 724:
      if (lookahead == 't') ADVANCE(345);
      END_STATE();
    case 725:
      if (lookahead == 't') ADVANCE(415);
      END_STATE();
    case 726:
      if (lookahead == 't') ADVANCE(526);
      END_STATE();
    case 727:
      if (lookahead == 't') ADVANCE(338);
      END_STATE();
    case 728:
      if (lookahead == 't') ADVANCE(327);
      END_STATE();
    case 729:
      if (lookahead == 't') ADVANCE(200);
      END_STATE();
    case 730:
      if (lookahead == 't') ADVANCE(176);
      END_STATE();
    case 731:
      if (lookahead == 't') ADVANCE(509);
      END_STATE();
    case 732:
      if (lookahead == 't') ADVANCE(511);
      END_STATE();
    case 733:
      if (lookahead == 't') ADVANCE(177);
      END_STATE();
    case 734:
      if (lookahead == 't') ADVANCE(629);
      END_STATE();
    case 735:
      if (lookahead == 't') ADVANCE(525);
      END_STATE();
    case 736:
      if (lookahead == 't') ADVANCE(196);
      END_STATE();
    case 737:
      if (lookahead == 't') ADVANCE(534);
      END_STATE();
    case 738:
      if (lookahead == 't') ADVANCE(437);
      END_STATE();
    case 739:
      if (lookahead == 't') ADVANCE(440);
      END_STATE();
    case 740:
      if (lookahead == 't') ADVANCE(202);
      END_STATE();
    case 741:
      if (lookahead == 't') ADVANCE(213);
      END_STATE();
    case 742:
      if (lookahead == 't') ADVANCE(431);
      END_STATE();
    case 743:
      if (lookahead == 't') ADVANCE(206);
      END_STATE();
    case 744:
      if (lookahead == 't') ADVANCE(433);
      END_STATE();
    case 745:
      if (lookahead == 't') ADVANCE(494);
      END_STATE();
    case 746:
      if (lookahead == 't') ADVANCE(510);
      END_STATE();
    case 747:
      if (lookahead == 't') ADVANCE(498);
      END_STATE();
    case 748:
      if (lookahead == 't') ADVANCE(457);
      END_STATE();
    case 749:
      if (lookahead == 't') ADVANCE(214);
      END_STATE();
    case 750:
      if (lookahead == 't') ADVANCE(499);
      END_STATE();
    case 751:
      if (lookahead == 't') ADVANCE(215);
      END_STATE();
    case 752:
      if (lookahead == 't') ADVANCE(515);
      END_STATE();
    case 753:
      if (lookahead == 't') ADVANCE(621);
      END_STATE();
    case 754:
      if (lookahead == 't') ADVANCE(469);
      END_STATE();
    case 755:
      if (lookahead == 't') ADVANCE(467);
      END_STATE();
    case 756:
      if (lookahead == 't') ADVANCE(354);
      END_STATE();
    case 757:
      if (lookahead == 't') ADVANCE(531);
      END_STATE();
    case 758:
      if (lookahead == 't') ADVANCE(468);
      END_STATE();
    case 759:
      if (lookahead == 't') ADVANCE(532);
      END_STATE();
    case 760:
      if (lookahead == 't') ADVANCE(533);
      END_STATE();
    case 761:
      if (lookahead == 'u') ADVANCE(478);
      END_STATE();
    case 762:
      if (lookahead == 'u') ADVANCE(572);
      END_STATE();
    case 763:
      if (lookahead == 'u') ADVANCE(655);
      END_STATE();
    case 764:
      if (lookahead == 'u') ADVANCE(406);
      END_STATE();
    case 765:
      if (lookahead == 'u') ADVANCE(696);
      END_STATE();
    case 766:
      if (lookahead == 'u') ADVANCE(712);
      END_STATE();
    case 767:
      if (lookahead == 'u') ADVANCE(421);
      END_STATE();
    case 768:
      if (lookahead == 'u') ADVANCE(745);
      if (lookahead == 'v') ADVANCE(341);
      END_STATE();
    case 769:
      if (lookahead == 'u') ADVANCE(747);
      END_STATE();
    case 770:
      if (lookahead == 'u') ADVANCE(460);
      END_STATE();
    case 771:
      if (lookahead == 'u') ADVANCE(750);
      END_STATE();
    case 772:
      if (lookahead == 'u') ADVANCE(463);
      END_STATE();
    case 773:
      if (lookahead == 'u') ADVANCE(524);
      END_STATE();
    case 774:
      if (lookahead == 'u') ADVANCE(658);
      END_STATE();
    case 775:
      if (lookahead == 'v') ADVANCE(517);
      END_STATE();
    case 776:
      if (lookahead == 'v') ADVANCE(453);
      END_STATE();
    case 777:
      if (lookahead == 'v') ADVANCE(461);
      END_STATE();
    case 778:
      if (lookahead == 'v') ADVANCE(451);
      END_STATE();
    case 779:
      if (lookahead == 'v') ADVANCE(347);
      END_STATE();
    case 780:
      if (lookahead == 'w') ADVANCE(608);
      END_STATE();
    case 781:
      if (lookahead == 'w') ADVANCE(348);
      END_STATE();
    case 782:
      if (lookahead == 'w') ADVANCE(415);
      END_STATE();
    case 783:
      if (lookahead == 'w') ADVANCE(324);
      END_STATE();
    case 784:
      if (lookahead == 'x') ADVANCE(646);
      END_STATE();
    case 785:
      if (lookahead == 'x') ADVANCE(787);
      END_STATE();
    case 786:
      if (lookahead == 'x') ADVANCE(739);
      END_STATE();
    case 787:
      if (lookahead == 'y') ADVANCE(923);
      END_STATE();
    case 788:
      if (lookahead == 'y') ADVANCE(212);
      END_STATE();
    case 789:
      if (lookahead == 'y') ADVANCE(207);
      END_STATE();
    case 790:
      if (lookahead == 'y') ADVANCE(210);
      END_STATE();
    case 791:
      if (lookahead == 'y') ADVANCE(647);
      END_STATE();
    case 792:
      if (lookahead == 'y') ADVANCE(215);
      END_STATE();
    case 793:
      if (lookahead == 'y') ADVANCE(216);
      END_STATE();
    case 794:
      if (lookahead == 'z') ADVANCE(415);
      END_STATE();
    case 795:
      if (lookahead == '}') ADVANCE(929);
      END_STATE();
    case 796:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(922);
      END_STATE();
    case 797:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(796);
      END_STATE();
    case 798:
      if (lookahead == '.' ||
          ('0' <= lookahead && lookahead <= '9')) ADVANCE(921);
      END_STATE();
    case 799:
      if (eof) ADVANCE(806);
      ADVANCE_MAP(
        0, 961,
        '\n', 961,
        '\r', 962,
        '#', 903,
        '-', 873,
        '/', 874,
        '<', 953,
        '>', 932,
        '@', 939,
        'C', 826,
        'D', 822,
        'G', 824,
        'H', 823,
        'L', 825,
        'O', 827,
        'P', 820,
        'T', 828,
        'W', 821,
        '[', 872,
        'm', 830,
        'q', 829,
        '{', 871,
        0x2028, 104,
        0x2029, 104,
      );
      if (lookahead == ' ' ||
          lookahead == 0xa0 ||
          lookahead == 0x1680 ||
          (0x2000 <= lookahead && lookahead <= 0x200a) ||
          lookahead == 0x202f ||
          lookahead == 0x205f ||
          lookahead == 0x3000) ADVANCE(893);
      if (set_contains(aux_sym_WORD_CHAR_token1_character_set_1, 475, lookahead)) ADVANCE(831);
      if (lookahead != 0) ADVANCE(875);
      END_STATE();
    case 800:
      if (eof) ADVANCE(806);
      ADVANCE_MAP(
        0, 863,
        '\n', 1010,
        '\r', 1011,
        '#', 906,
        '/', 887,
        '<', 930,
        '@', 937,
        'C', 856,
        'D', 846,
        'G', 847,
        'H', 852,
        'L', 854,
        'O', 858,
        'P', 844,
        'T', 860,
        'W', 848,
        '{', 888,
        '-', 890,
        '_', 890,
      );
      if (lookahead == ' ' ||
          lookahead == 0xa0 ||
          lookahead == 0x1680 ||
          (0x2000 <= lookahead && lookahead <= 0x200a) ||
          lookahead == 0x202f ||
          lookahead == 0x205f ||
          lookahead == 0x3000) ADVANCE(896);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(862);
      if (set_contains(aux_sym_WORD_CHAR_token1_character_set_1, 475, lookahead)) ADVANCE(807);
      if (lookahead != 0 &&
          lookahead != 0x2028 &&
          lookahead != 0x2029) ADVANCE(863);
      END_STATE();
    case 801:
      if (eof) ADVANCE(806);
      ADVANCE_MAP(
        0, 863,
        '\n', 1010,
        '\r', 1011,
        '#', 906,
        '/', 887,
        '<', 930,
        '@', 937,
        'C', 857,
        'D', 849,
        'G', 850,
        'H', 853,
        'L', 855,
        'O', 859,
        'P', 845,
        'T', 861,
        'W', 851,
        '{', 888,
      );
      if (lookahead == ' ' ||
          lookahead == 0xa0 ||
          lookahead == 0x1680 ||
          (0x2000 <= lookahead && lookahead <= 0x200a) ||
          lookahead == 0x202f ||
          lookahead == 0x205f ||
          lookahead == 0x3000) ADVANCE(896);
      if (set_contains(aux_sym_WORD_CHAR_token1_character_set_1, 475, lookahead)) ADVANCE(807);
      if (lookahead != 0 &&
          lookahead != 0x2028 &&
          lookahead != 0x2029) ADVANCE(863);
      END_STATE();
    case 802:
      if (eof) ADVANCE(806);
      ADVANCE_MAP(
        0, 868,
        '\n', 897,
        '\r', 898,
        '#', 902,
        '-', 866,
        '/', 867,
        '<', 953,
        '>', 932,
        '@', 938,
        'C', 814,
        'D', 809,
        'G', 810,
        'H', 812,
        'L', 813,
        'O', 815,
        'P', 808,
        'T', 816,
        'W', 811,
        '[', 870,
        'm', 817,
        'q', 818,
        '{', 869,
        0x2028, 52,
        0x2029, 52,
      );
      if (lookahead == ' ' ||
          lookahead == 0xa0 ||
          lookahead == 0x1680 ||
          (0x2000 <= lookahead && lookahead <= 0x200a) ||
          lookahead == 0x202f ||
          lookahead == 0x205f ||
          lookahead == 0x3000) ADVANCE(892);
      if (set_contains(aux_sym_WORD_CHAR_token1_character_set_1, 475, lookahead)) ADVANCE(819);
      if (lookahead != 0) ADVANCE(868);
      END_STATE();
    case 803:
      if (eof) ADVANCE(806);
      ADVANCE_MAP(
        0, 868,
        '\n', 897,
        '\r', 898,
        '#', 902,
        '-', 866,
        '/', 867,
        '<', 953,
        '>', 932,
        '@', 938,
        'C', 814,
        'D', 809,
        'G', 810,
        'H', 812,
        'L', 813,
        'O', 815,
        'P', 808,
        'T', 816,
        'W', 811,
        '[', 865,
        'm', 817,
        'q', 818,
        '{', 864,
        0x2028, 52,
        0x2029, 52,
      );
      if (lookahead == ' ' ||
          lookahead == 0xa0 ||
          lookahead == 0x1680 ||
          (0x2000 <= lookahead && lookahead <= 0x200a) ||
          lookahead == 0x202f ||
          lookahead == 0x205f ||
          lookahead == 0x3000) ADVANCE(892);
      if (set_contains(aux_sym_WORD_CHAR_token1_character_set_1, 475, lookahead)) ADVANCE(819);
      if (lookahead != 0) ADVANCE(868);
      END_STATE();
    case 804:
      if (eof) ADVANCE(806);
      ADVANCE_MAP(
        0, 967,
        '\n', 966,
        '\r', 967,
        '#', 904,
        '-', 878,
        '/', 879,
        '<', 954,
        '>', 932,
        '@', 940,
        'C', 838,
        'D', 834,
        'G', 836,
        'H', 835,
        'L', 837,
        'O', 839,
        'P', 832,
        'T', 840,
        'W', 833,
        '[', 877,
        'm', 842,
        'q', 841,
        '{', 876,
        0x2028, 157,
        0x2029, 157,
      );
      if (lookahead == ' ' ||
          lookahead == 0xa0 ||
          lookahead == 0x1680 ||
          (0x2000 <= lookahead && lookahead <= 0x200a) ||
          lookahead == 0x202f ||
          lookahead == 0x205f ||
          lookahead == 0x3000) ADVANCE(894);
      if (set_contains(aux_sym_WORD_CHAR_token1_character_set_1, 475, lookahead)) ADVANCE(843);
      if (lookahead != 0) ADVANCE(880);
      END_STATE();
    case 805:
      if (eof) ADVANCE(806);
      ADVANCE_MAP(
        0, 967,
        '\n', 966,
        '\r', 967,
        '#', 904,
        '-', 878,
        '/', 879,
        '<', 954,
        '>', 932,
        '@', 940,
        'C', 838,
        'D', 834,
        'G', 836,
        'H', 835,
        'L', 837,
        'O', 839,
        'P', 832,
        'T', 840,
        'W', 833,
        '[', 882,
        'm', 842,
        'q', 841,
        '{', 881,
        0x2028, 157,
        0x2029, 157,
      );
      if (lookahead == ' ' ||
          lookahead == 0xa0 ||
          lookahead == 0x1680 ||
          (0x2000 <= lookahead && lookahead <= 0x200a) ||
          lookahead == 0x202f ||
          lookahead == 0x205f ||
          lookahead == 0x3000) ADVANCE(894);
      if (set_contains(aux_sym_WORD_CHAR_token1_character_set_1, 475, lookahead)) ADVANCE(843);
      if (lookahead != 0) ADVANCE(880);
      END_STATE();
    case 806:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 807:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      END_STATE();
    case 808:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'A') ADVANCE(38);
      if (lookahead == 'O') ADVANCE(33);
      if (lookahead == 'U') ADVANCE(36);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 809:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'E') ADVANCE(23);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 810:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'E') ADVANCE(36);
      if (lookahead == 'R') ADVANCE(6);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 811:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'E') ADVANCE(9);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 812:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'E') ADVANCE(7);
      if (lookahead == 'T') ADVANCE(39);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 813:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'I') ADVANCE(33);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 814:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'O') ADVANCE(25);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 815:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'P') ADVANCE(37);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 816:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'R') ADVANCE(8);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 817:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'u') ADVANCE(47);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 818:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == 'u') ADVANCE(42);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 819:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 820:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'A') ADVANCE(91);
      if (lookahead == 'O') ADVANCE(87);
      if (lookahead == 'U') ADVANCE(88);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 821:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'E') ADVANCE(61);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 822:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'E') ADVANCE(76);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 823:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'E') ADVANCE(58);
      if (lookahead == 'T') ADVANCE(90);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 824:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'E') ADVANCE(88);
      if (lookahead == 'R') ADVANCE(59);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 825:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'I') ADVANCE(87);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 826:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'O') ADVANCE(77);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 827:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'P') ADVANCE(89);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 828:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'R') ADVANCE(60);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 829:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'u') ADVANCE(94);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 830:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == 'u') ADVANCE(99);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 831:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 832:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'A') ADVANCE(144);
      if (lookahead == 'O') ADVANCE(140);
      if (lookahead == 'U') ADVANCE(141);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 833:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'E') ADVANCE(114);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 834:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'E') ADVANCE(129);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 835:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'E') ADVANCE(111);
      if (lookahead == 'T') ADVANCE(143);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 836:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'E') ADVANCE(141);
      if (lookahead == 'R') ADVANCE(112);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 837:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'I') ADVANCE(140);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 838:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'O') ADVANCE(130);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 839:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'P') ADVANCE(142);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 840:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'R') ADVANCE(113);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 841:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'u') ADVANCE(147);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 842:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == 'u') ADVANCE(152);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 843:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 844:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if (lookahead == 'A') ADVANCE(1004);
      if (lookahead == 'O') ADVANCE(999);
      if (lookahead == 'U') ADVANCE(1002);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('B' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 845:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if (lookahead == 'A') ADVANCE(305);
      if (lookahead == 'O') ADVANCE(295);
      if (lookahead == 'U') ADVANCE(303);
      END_STATE();
    case 846:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if (lookahead == 'E') ADVANCE(989);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 847:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if (lookahead == 'E') ADVANCE(1002);
      if (lookahead == 'R') ADVANCE(972);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 848:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if (lookahead == 'E') ADVANCE(975);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 849:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if (lookahead == 'E') ADVANCE(265);
      END_STATE();
    case 850:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if (lookahead == 'E') ADVANCE(303);
      if (lookahead == 'R') ADVANCE(227);
      END_STATE();
    case 851:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if (lookahead == 'E') ADVANCE(235);
      END_STATE();
    case 852:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if (lookahead == 'E') ADVANCE(973);
      if (lookahead == 'T') ADVANCE(1005);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 853:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if (lookahead == 'E') ADVANCE(228);
      if (lookahead == 'T') ADVANCE(306);
      END_STATE();
    case 854:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if (lookahead == 'I') ADVANCE(999);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 855:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if (lookahead == 'I') ADVANCE(295);
      END_STATE();
    case 856:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if (lookahead == 'O') ADVANCE(991);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 857:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if (lookahead == 'O') ADVANCE(271);
      END_STATE();
    case 858:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if (lookahead == 'P') ADVANCE(1003);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 859:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if (lookahead == 'P') ADVANCE(304);
      END_STATE();
    case 860:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if (lookahead == 'R') ADVANCE(974);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 861:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if (lookahead == 'R') ADVANCE(229);
      END_STATE();
    case 862:
      ACCEPT_TOKEN(aux_sym_WORD_CHAR_token1);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 863:
      ACCEPT_TOKEN(aux_sym_PUNCTUATION_token1);
      END_STATE();
    case 864:
      ACCEPT_TOKEN(aux_sym_PUNCTUATION_token1);
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(952);
      if (lookahead == '\r') ADVANCE(951);
      if (lookahead == '{') ADVANCE(926);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(951);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 865:
      ACCEPT_TOKEN(aux_sym_PUNCTUATION_token1);
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(952);
      if (lookahead == '\r') ADVANCE(951);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(951);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 866:
      ACCEPT_TOKEN(aux_sym_PUNCTUATION_token1);
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == '-') ADVANCE(956);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 867:
      ACCEPT_TOKEN(aux_sym_PUNCTUATION_token1);
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == '/') ADVANCE(905);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 868:
      ACCEPT_TOKEN(aux_sym_PUNCTUATION_token1);
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 869:
      ACCEPT_TOKEN(aux_sym_PUNCTUATION_token1);
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(945);
      if (lookahead == '\r') ADVANCE(944);
      if (lookahead == '{') ADVANCE(926);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(944);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 870:
      ACCEPT_TOKEN(aux_sym_PUNCTUATION_token1);
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(945);
      if (lookahead == '\r') ADVANCE(944);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(944);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 871:
      ACCEPT_TOKEN(aux_sym_PUNCTUATION_token1);
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(945);
      if (lookahead == '\r') ADVANCE(944);
      if (lookahead == '{') ADVANCE(927);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(944);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 872:
      ACCEPT_TOKEN(aux_sym_PUNCTUATION_token1);
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(945);
      if (lookahead == '\r') ADVANCE(944);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(944);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 873:
      ACCEPT_TOKEN(aux_sym_PUNCTUATION_token1);
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == '-') ADVANCE(956);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 874:
      ACCEPT_TOKEN(aux_sym_PUNCTUATION_token1);
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == '/') ADVANCE(905);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 875:
      ACCEPT_TOKEN(aux_sym_PUNCTUATION_token1);
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 876:
      ACCEPT_TOKEN(aux_sym_PUNCTUATION_token1);
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(952);
      if (lookahead == '\r') ADVANCE(951);
      if (lookahead == '{') ADVANCE(928);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(951);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 877:
      ACCEPT_TOKEN(aux_sym_PUNCTUATION_token1);
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(952);
      if (lookahead == '\r') ADVANCE(951);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(951);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 878:
      ACCEPT_TOKEN(aux_sym_PUNCTUATION_token1);
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == '-') ADVANCE(957);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 879:
      ACCEPT_TOKEN(aux_sym_PUNCTUATION_token1);
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == '/') ADVANCE(905);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 880:
      ACCEPT_TOKEN(aux_sym_PUNCTUATION_token1);
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 881:
      ACCEPT_TOKEN(aux_sym_PUNCTUATION_token1);
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(968);
      if (lookahead == '{') ADVANCE(928);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(943);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 882:
      ACCEPT_TOKEN(aux_sym_PUNCTUATION_token1);
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(968);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(943);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 883:
      ACCEPT_TOKEN(aux_sym_PUNCTUATION_token1);
      if (lookahead == '\r') ADVANCE(931);
      if ((!eof && lookahead == 00) ||
          lookahead == '\n') ADVANCE(931);
      END_STATE();
    case 884:
      ACCEPT_TOKEN(aux_sym_PUNCTUATION_token1);
      if (lookahead == '%') ADVANCE(933);
      if (lookahead == '{') ADVANCE(925);
      END_STATE();
    case 885:
      ACCEPT_TOKEN(aux_sym_PUNCTUATION_token1);
      if (lookahead == '%') ADVANCE(933);
      if (lookahead == '{') ADVANCE(925);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(952);
      END_STATE();
    case 886:
      ACCEPT_TOKEN(aux_sym_PUNCTUATION_token1);
      if (lookahead == '-') ADVANCE(956);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 887:
      ACCEPT_TOKEN(aux_sym_PUNCTUATION_token1);
      if (lookahead == '/') ADVANCE(907);
      END_STATE();
    case 888:
      ACCEPT_TOKEN(aux_sym_PUNCTUATION_token1);
      if (lookahead == '{') ADVANCE(925);
      END_STATE();
    case 889:
      ACCEPT_TOKEN(aux_sym_PUNCTUATION_token1);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(952);
      END_STATE();
    case 890:
      ACCEPT_TOKEN(aux_sym_PUNCTUATION_token1);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 891:
      ACCEPT_TOKEN(aux_sym_PUNCTUATION_token1);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '\r') ADVANCE(1009);
      END_STATE();
    case 892:
      ACCEPT_TOKEN(aux_sym_WS_token1);
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == ' ' ||
          lookahead == 0xa0 ||
          lookahead == 0x1680 ||
          (0x2000 <= lookahead && lookahead <= 0x200a) ||
          lookahead == 0x202f ||
          lookahead == 0x205f ||
          lookahead == 0x3000) ADVANCE(892);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 893:
      ACCEPT_TOKEN(aux_sym_WS_token1);
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == ' ' ||
          lookahead == 0xa0 ||
          lookahead == 0x1680 ||
          (0x2000 <= lookahead && lookahead <= 0x200a) ||
          lookahead == 0x202f ||
          lookahead == 0x205f ||
          lookahead == 0x3000) ADVANCE(893);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 894:
      ACCEPT_TOKEN(aux_sym_WS_token1);
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == ' ' ||
          lookahead == 0xa0 ||
          lookahead == 0x1680 ||
          (0x2000 <= lookahead && lookahead <= 0x200a) ||
          lookahead == 0x202f ||
          lookahead == 0x205f ||
          lookahead == 0x3000) ADVANCE(894);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 895:
      ACCEPT_TOKEN(aux_sym_WS_token1);
      if (lookahead == ' ' ||
          lookahead == 0xa0) ADVANCE(896);
      if (lookahead == 0x1680 ||
          (0x2000 <= lookahead && lookahead <= 0x200a) ||
          lookahead == 0x202f ||
          lookahead == 0x205f ||
          lookahead == 0x3000) ADVANCE(895);
      if (lookahead == '$' ||
          lookahead == '-' ||
          lookahead == '.' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa1 <= lookahead && lookahead <= 0xffff)) ADVANCE(1008);
      END_STATE();
    case 896:
      ACCEPT_TOKEN(aux_sym_WS_token1);
      if (lookahead == ' ' ||
          lookahead == 0xa0 ||
          lookahead == 0x1680 ||
          (0x2000 <= lookahead && lookahead <= 0x200a) ||
          lookahead == 0x202f ||
          lookahead == 0x205f ||
          lookahead == 0x3000) ADVANCE(896);
      END_STATE();
    case 897:
      ACCEPT_TOKEN(aux_sym_NL_token1);
      END_STATE();
    case 898:
      ACCEPT_TOKEN(aux_sym_NL_token1);
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 899:
      ACCEPT_TOKEN(aux_sym_NL_token1);
      if (lookahead == '\n') ADVANCE(897);
      END_STATE();
    case 900:
      ACCEPT_TOKEN(aux_sym_LINE_TAIL_token1);
      END_STATE();
    case 901:
      ACCEPT_TOKEN(aux_sym_LINE_TAIL_token1);
      if ((!eof && lookahead == 00)) ADVANCE(901);
      if (lookahead == '\n') ADVANCE(900);
      if (lookahead == '\r') ADVANCE(901);
      if (lookahead != 0) ADVANCE(164);
      END_STATE();
    case 902:
      ACCEPT_TOKEN(aux_sym_COMMENT_PREFIX_token1);
      if (lookahead == '\n') ADVANCE(907);
      if (lookahead == '\r') ADVANCE(905);
      if (lookahead == '#') ADVANCE(4);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(905);
      END_STATE();
    case 903:
      ACCEPT_TOKEN(aux_sym_COMMENT_PREFIX_token1);
      if (lookahead == '\n') ADVANCE(907);
      if (lookahead == '\r') ADVANCE(905);
      if (lookahead == '#') ADVANCE(56);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(905);
      END_STATE();
    case 904:
      ACCEPT_TOKEN(aux_sym_COMMENT_PREFIX_token1);
      if (lookahead == '\n') ADVANCE(907);
      if (lookahead == '\r') ADVANCE(905);
      if (lookahead == '#') ADVANCE(107);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(905);
      END_STATE();
    case 905:
      ACCEPT_TOKEN(aux_sym_COMMENT_PREFIX_token1);
      if (lookahead == '\n') ADVANCE(907);
      if (lookahead == '\r') ADVANCE(905);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(905);
      END_STATE();
    case 906:
      ACCEPT_TOKEN(aux_sym_COMMENT_PREFIX_token1);
      if (lookahead == '#') ADVANCE(222);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(907);
      END_STATE();
    case 907:
      ACCEPT_TOKEN(aux_sym_COMMENT_PREFIX_token1);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(907);
      END_STATE();
    case 908:
      ACCEPT_TOKEN(anon_sym_AT);
      END_STATE();
    case 909:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 910:
      ACCEPT_TOKEN(aux_sym__var_comment_token1);
      if (lookahead == ' ' ||
          lookahead == 0xa0 ||
          lookahead == 0x1680 ||
          (0x2000 <= lookahead && lookahead <= 0x200a) ||
          lookahead == 0x202f ||
          lookahead == 0x205f ||
          lookahead == 0x3000) ADVANCE(910);
      END_STATE();
    case 911:
      ACCEPT_TOKEN(aux_sym_request_separator_token1);
      if (lookahead == '#') ADVANCE(911);
      if (lookahead == ' ' ||
          lookahead == 0xa0 ||
          lookahead == 0x1680 ||
          (0x2000 <= lookahead && lookahead <= 0x200a) ||
          lookahead == 0x202f ||
          lookahead == 0x205f ||
          lookahead == 0x3000) ADVANCE(912);
      END_STATE();
    case 912:
      ACCEPT_TOKEN(aux_sym_request_separator_token1);
      if (lookahead == ' ' ||
          lookahead == 0xa0 ||
          lookahead == 0x1680 ||
          (0x2000 <= lookahead && lookahead <= 0x200a) ||
          lookahead == 0x202f ||
          lookahead == 0x205f ||
          lookahead == 0x3000) ADVANCE(912);
      END_STATE();
    case 913:
      ACCEPT_TOKEN(sym_method);
      END_STATE();
    case 914:
      ACCEPT_TOKEN(sym_method);
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 915:
      ACCEPT_TOKEN(sym_method);
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 916:
      ACCEPT_TOKEN(sym_method);
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 917:
      ACCEPT_TOKEN(sym_method);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 918:
      ACCEPT_TOKEN(aux_sym_http_version_token1);
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead == '.' ||
          ('0' <= lookahead && lookahead <= '9')) ADVANCE(918);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 919:
      ACCEPT_TOKEN(aux_sym_http_version_token1);
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead == '.' ||
          ('0' <= lookahead && lookahead <= '9')) ADVANCE(919);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 920:
      ACCEPT_TOKEN(aux_sym_http_version_token1);
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead == '.' ||
          ('0' <= lookahead && lookahead <= '9')) ADVANCE(920);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 921:
      ACCEPT_TOKEN(aux_sym_http_version_token1);
      if (lookahead == '.' ||
          ('0' <= lookahead && lookahead <= '9')) ADVANCE(921);
      END_STATE();
    case 922:
      ACCEPT_TOKEN(sym_status_code);
      END_STATE();
    case 923:
      ACCEPT_TOKEN(sym_status_text);
      END_STATE();
    case 924:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 925:
      ACCEPT_TOKEN(anon_sym_LBRACE_LBRACE);
      END_STATE();
    case 926:
      ACCEPT_TOKEN(anon_sym_LBRACE_LBRACE);
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 927:
      ACCEPT_TOKEN(anon_sym_LBRACE_LBRACE);
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 928:
      ACCEPT_TOKEN(anon_sym_LBRACE_LBRACE);
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 929:
      ACCEPT_TOKEN(anon_sym_RBRACE_RBRACE);
      END_STATE();
    case 930:
      ACCEPT_TOKEN(anon_sym_LT);
      END_STATE();
    case 931:
      ACCEPT_TOKEN(aux_sym_pre_request_script_token1);
      if (lookahead == '\r') ADVANCE(931);
      if ((!eof && lookahead == 00) ||
          lookahead == '\n') ADVANCE(931);
      END_STATE();
    case 932:
      ACCEPT_TOKEN(anon_sym_GT);
      if (lookahead == '>') ADVANCE(936);
      END_STATE();
    case 933:
      ACCEPT_TOKEN(anon_sym_LBRACE_PERCENT);
      END_STATE();
    case 934:
      ACCEPT_TOKEN(anon_sym_PERCENT_RBRACE);
      END_STATE();
    case 935:
      ACCEPT_TOKEN(aux_sym_res_redirect_token1);
      END_STATE();
    case 936:
      ACCEPT_TOKEN(aux_sym_res_redirect_token1);
      if (lookahead == '!') ADVANCE(935);
      END_STATE();
    case 937:
      ACCEPT_TOKEN(anon_sym_AT2);
      END_STATE();
    case 938:
      ACCEPT_TOKEN(anon_sym_AT2);
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 939:
      ACCEPT_TOKEN(anon_sym_AT2);
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 940:
      ACCEPT_TOKEN(anon_sym_AT2);
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 941:
      ACCEPT_TOKEN(aux_sym_xml_body_token1);
      END_STATE();
    case 942:
      ACCEPT_TOKEN(aux_sym_xml_body_token1);
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 943:
      ACCEPT_TOKEN(aux_sym_json_body_token1);
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(968);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(943);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 944:
      ACCEPT_TOKEN(aux_sym_json_body_token1);
      if (lookahead == '\n') ADVANCE(945);
      if (lookahead == '\r') ADVANCE(944);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(944);
      END_STATE();
    case 945:
      ACCEPT_TOKEN(aux_sym_json_body_token1);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(945);
      END_STATE();
    case 946:
      ACCEPT_TOKEN(aux_sym_graphql_data_token1);
      END_STATE();
    case 947:
      ACCEPT_TOKEN(aux_sym_graphql_data_token1);
      if ((!eof && lookahead == 00)) ADVANCE(965);
      if (lookahead == '\n') ADVANCE(946);
      if (lookahead == '\r') ADVANCE(965);
      if (lookahead == '{') ADVANCE(55);
      if (lookahead != 0) ADVANCE(54);
      END_STATE();
    case 948:
      ACCEPT_TOKEN(aux_sym_graphql_data_token1);
      if ((!eof && lookahead == 00)) ADVANCE(965);
      if (lookahead == '\r') ADVANCE(965);
      if (lookahead == '{') ADVANCE(55);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(54);
      END_STATE();
    case 949:
      ACCEPT_TOKEN(aux_sym_graphql_data_token1);
      if ((!eof && lookahead == 00)) ADVANCE(960);
      if (lookahead == '\n') ADVANCE(946);
      if (lookahead == '\r') ADVANCE(960);
      if (lookahead == '{') ADVANCE(165);
      if (lookahead != 0) ADVANCE(106);
      END_STATE();
    case 950:
      ACCEPT_TOKEN(aux_sym_graphql_data_token1);
      if ((!eof && lookahead == 00)) ADVANCE(960);
      if (lookahead == '\r') ADVANCE(960);
      if (lookahead == '{') ADVANCE(165);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(106);
      END_STATE();
    case 951:
      ACCEPT_TOKEN(aux_sym_graphql_json_body_token1);
      if (lookahead == '\n') ADVANCE(952);
      if (lookahead == '\r') ADVANCE(951);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(951);
      END_STATE();
    case 952:
      ACCEPT_TOKEN(aux_sym_graphql_json_body_token1);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(952);
      END_STATE();
    case 953:
      ACCEPT_TOKEN(anon_sym_LT2);
      if ((!eof && lookahead == 00)) ADVANCE(941);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead) &&
          lookahead != ' ' &&
          lookahead != '@') ADVANCE(941);
      END_STATE();
    case 954:
      ACCEPT_TOKEN(anon_sym_LT2);
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ' ||
          lookahead == '@') ADVANCE(157);
      if (lookahead != 0) ADVANCE(942);
      END_STATE();
    case 955:
      ACCEPT_TOKEN(anon_sym_LT2);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead) &&
          lookahead != ' ' &&
          lookahead != '@') ADVANCE(941);
      END_STATE();
    case 956:
      ACCEPT_TOKEN(anon_sym_DASH_DASH);
      END_STATE();
    case 957:
      ACCEPT_TOKEN(anon_sym_DASH_DASH);
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 958:
      ACCEPT_TOKEN(aux_sym_multipart_form_data_token1);
      END_STATE();
    case 959:
      ACCEPT_TOKEN(aux_sym_multipart_form_data_token1);
      if ((!eof && lookahead == 00)) ADVANCE(959);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(959);
      if (lookahead != 0) ADVANCE(104);
      END_STATE();
    case 960:
      ACCEPT_TOKEN(aux_sym_multipart_form_data_token1);
      if ((!eof && lookahead == 00)) ADVANCE(960);
      if (lookahead == '\n') ADVANCE(958);
      if (lookahead == '\r') ADVANCE(960);
      if (lookahead == '{') ADVANCE(165);
      if (lookahead != 0) ADVANCE(106);
      END_STATE();
    case 961:
      ACCEPT_TOKEN(aux_sym_multipart_form_data_token2);
      END_STATE();
    case 962:
      ACCEPT_TOKEN(aux_sym_multipart_form_data_token2);
      if (lookahead == '\n') ADVANCE(961);
      END_STATE();
    case 963:
      ACCEPT_TOKEN(aux_sym_raw_body_token1);
      END_STATE();
    case 964:
      ACCEPT_TOKEN(aux_sym_raw_body_token1);
      if ((!eof && lookahead == 00)) ADVANCE(964);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(964);
      if (lookahead != 0) ADVANCE(52);
      END_STATE();
    case 965:
      ACCEPT_TOKEN(aux_sym_raw_body_token1);
      if ((!eof && lookahead == 00)) ADVANCE(965);
      if (lookahead == '\n') ADVANCE(963);
      if (lookahead == '\r') ADVANCE(965);
      if (lookahead == '{') ADVANCE(55);
      if (lookahead != 0) ADVANCE(54);
      END_STATE();
    case 966:
      ACCEPT_TOKEN(aux_sym__raw_body_token1);
      END_STATE();
    case 967:
      ACCEPT_TOKEN(aux_sym__raw_body_token1);
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(967);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 968:
      ACCEPT_TOKEN(aux_sym__raw_body_token1);
      if ((!eof && lookahead == 00)) ADVANCE(967);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(968);
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') ADVANCE(943);
      if (lookahead != 0) ADVANCE(157);
      END_STATE();
    case 969:
      ACCEPT_TOKEN(aux_sym__raw_body_token1);
      if ((!eof && lookahead == 00)) ADVANCE(969);
      if (lookahead == '\n') ADVANCE(966);
      if (lookahead == '\r') ADVANCE(969);
      if (lookahead == '{') ADVANCE(159);
      if (lookahead != 0) ADVANCE(159);
      END_STATE();
    case 970:
      ACCEPT_TOKEN(sym__not_comment);
      if (lookahead == '\r') ADVANCE(970);
      if ((!eof && lookahead == 00) ||
          lookahead == '\n') ADVANCE(970);
      if (lookahead != 0 &&
          lookahead != '@') ADVANCE(170);
      END_STATE();
    case 971:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == '/') ADVANCE(798);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 972:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'A') ADVANCE(996);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('B' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 973:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'A') ADVANCE(980);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('B' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 974:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'A') ADVANCE(979);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('B' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 975:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'B') ADVANCE(1001);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 976:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'C') ADVANCE(1002);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 977:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'C') ADVANCE(988);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 978:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'C') ADVANCE(985);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 979:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'C') ADVANCE(982);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 980:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'D') ADVANCE(917);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 981:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'E') ADVANCE(1002);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 982:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'E') ADVANCE(917);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 983:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'E') ADVANCE(976);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 984:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'E') ADVANCE(1006);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 985:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'H') ADVANCE(917);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 986:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'H') ADVANCE(998);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 987:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'I') ADVANCE(994);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 988:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'K') ADVANCE(981);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 989:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'L') ADVANCE(984);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 990:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'L') ADVANCE(917);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 991:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'N') ADVANCE(992);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 992:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'N') ADVANCE(983);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 993:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'N') ADVANCE(1000);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 994:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'O') ADVANCE(993);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 995:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'O') ADVANCE(977);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 996:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'P') ADVANCE(986);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 997:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'P') ADVANCE(971);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 998:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'Q') ADVANCE(990);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 999:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'S') ADVANCE(1002);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 1000:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'S') ADVANCE(917);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 1001:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'S') ADVANCE(995);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 1002:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'T') ADVANCE(917);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 1003:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'T') ADVANCE(987);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 1004:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'T') ADVANCE(978);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 1005:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'T') ADVANCE(997);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 1006:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == 'T') ADVANCE(982);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 1007:
      ACCEPT_TOKEN(sym_header_entity);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1007);
      END_STATE();
    case 1008:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '$' ||
          lookahead == '-' ||
          lookahead == '.' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z') ||
          (0xa1 <= lookahead && lookahead <= 0xffff)) ADVANCE(1008);
      END_STATE();
    case 1009:
      ACCEPT_TOKEN(aux_sym_path_token1);
      END_STATE();
    case 1010:
      ACCEPT_TOKEN(aux_sym__blank_line_token1);
      END_STATE();
    case 1011:
      ACCEPT_TOKEN(aux_sym__blank_line_token1);
      if (lookahead == '\n') ADVANCE(1010);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 801},
  [2] = {.lex_state = 802},
  [3] = {.lex_state = 802},
  [4] = {.lex_state = 802},
  [5] = {.lex_state = 802},
  [6] = {.lex_state = 802},
  [7] = {.lex_state = 802},
  [8] = {.lex_state = 802},
  [9] = {.lex_state = 802},
  [10] = {.lex_state = 802},
  [11] = {.lex_state = 802},
  [12] = {.lex_state = 802},
  [13] = {.lex_state = 802},
  [14] = {.lex_state = 802},
  [15] = {.lex_state = 802},
  [16] = {.lex_state = 802},
  [17] = {.lex_state = 802},
  [18] = {.lex_state = 802},
  [19] = {.lex_state = 802},
  [20] = {.lex_state = 802},
  [21] = {.lex_state = 802},
  [22] = {.lex_state = 802},
  [23] = {.lex_state = 802},
  [24] = {.lex_state = 802},
  [25] = {.lex_state = 802},
  [26] = {.lex_state = 802},
  [27] = {.lex_state = 801},
  [28] = {.lex_state = 799},
  [29] = {.lex_state = 801},
  [30] = {.lex_state = 799},
  [31] = {.lex_state = 799},
  [32] = {.lex_state = 801},
  [33] = {.lex_state = 801},
  [34] = {.lex_state = 804},
  [35] = {.lex_state = 804},
  [36] = {.lex_state = 799},
  [37] = {.lex_state = 805},
  [38] = {.lex_state = 799},
  [39] = {.lex_state = 799},
  [40] = {.lex_state = 799},
  [41] = {.lex_state = 805},
  [42] = {.lex_state = 805},
  [43] = {.lex_state = 805},
  [44] = {.lex_state = 799},
  [45] = {.lex_state = 799},
  [46] = {.lex_state = 799},
  [47] = {.lex_state = 803},
  [48] = {.lex_state = 802},
  [49] = {.lex_state = 803},
  [50] = {.lex_state = 803},
  [51] = {.lex_state = 803},
  [52] = {.lex_state = 802},
  [53] = {.lex_state = 802},
  [54] = {.lex_state = 802},
  [55] = {.lex_state = 802},
  [56] = {.lex_state = 802},
  [57] = {.lex_state = 802},
  [58] = {.lex_state = 802},
  [59] = {.lex_state = 802},
  [60] = {.lex_state = 802},
  [61] = {.lex_state = 802},
  [62] = {.lex_state = 802},
  [63] = {.lex_state = 802},
  [64] = {.lex_state = 802},
  [65] = {.lex_state = 802},
  [66] = {.lex_state = 802},
  [67] = {.lex_state = 802},
  [68] = {.lex_state = 802},
  [69] = {.lex_state = 800},
  [70] = {.lex_state = 800},
  [71] = {.lex_state = 800},
  [72] = {.lex_state = 800},
  [73] = {.lex_state = 800},
  [74] = {.lex_state = 800},
  [75] = {.lex_state = 800},
  [76] = {.lex_state = 800},
  [77] = {.lex_state = 800},
  [78] = {.lex_state = 800},
  [79] = {.lex_state = 800},
  [80] = {.lex_state = 800},
  [81] = {.lex_state = 800},
  [82] = {.lex_state = 800},
  [83] = {.lex_state = 800},
  [84] = {.lex_state = 800},
  [85] = {.lex_state = 800},
  [86] = {.lex_state = 800},
  [87] = {.lex_state = 800},
  [88] = {.lex_state = 800},
  [89] = {.lex_state = 800},
  [90] = {.lex_state = 800},
  [91] = {.lex_state = 800},
  [92] = {.lex_state = 800},
  [93] = {.lex_state = 800},
  [94] = {.lex_state = 800},
  [95] = {.lex_state = 800},
  [96] = {.lex_state = 800},
  [97] = {.lex_state = 801},
  [98] = {.lex_state = 801},
  [99] = {.lex_state = 801},
  [100] = {.lex_state = 801},
  [101] = {.lex_state = 801},
  [102] = {.lex_state = 801},
  [103] = {.lex_state = 801},
  [104] = {.lex_state = 801},
  [105] = {.lex_state = 801},
  [106] = {.lex_state = 801},
  [107] = {.lex_state = 801},
  [108] = {.lex_state = 801},
  [109] = {.lex_state = 801},
  [110] = {.lex_state = 801},
  [111] = {.lex_state = 801},
  [112] = {.lex_state = 801},
  [113] = {.lex_state = 801},
  [114] = {.lex_state = 801},
  [115] = {.lex_state = 801},
  [116] = {.lex_state = 801},
  [117] = {.lex_state = 2},
  [118] = {.lex_state = 2},
  [119] = {.lex_state = 317},
  [120] = {.lex_state = 317},
  [121] = {.lex_state = 2},
  [122] = {.lex_state = 3},
  [123] = {.lex_state = 2},
  [124] = {.lex_state = 2},
  [125] = {.lex_state = 2},
  [126] = {.lex_state = 3},
  [127] = {.lex_state = 3},
  [128] = {.lex_state = 2},
  [129] = {.lex_state = 1},
  [130] = {.lex_state = 1},
  [131] = {.lex_state = 2},
  [132] = {.lex_state = 3},
  [133] = {.lex_state = 2},
  [134] = {.lex_state = 3},
  [135] = {.lex_state = 3},
  [136] = {.lex_state = 3},
  [137] = {.lex_state = 1},
  [138] = {.lex_state = 3},
  [139] = {.lex_state = 1},
  [140] = {.lex_state = 3},
  [141] = {.lex_state = 160},
  [142] = {.lex_state = 3},
  [143] = {.lex_state = 3},
  [144] = {.lex_state = 3},
  [145] = {.lex_state = 3},
  [146] = {.lex_state = 3},
  [147] = {.lex_state = 1},
  [148] = {.lex_state = 1},
  [149] = {.lex_state = 3},
  [150] = {.lex_state = 160},
  [151] = {.lex_state = 1},
  [152] = {.lex_state = 1},
  [153] = {.lex_state = 1},
  [154] = {.lex_state = 1},
  [155] = {.lex_state = 3},
  [156] = {.lex_state = 3},
  [157] = {.lex_state = 160},
  [158] = {.lex_state = 160},
  [159] = {.lex_state = 160},
  [160] = {.lex_state = 160},
  [161] = {.lex_state = 3},
  [162] = {.lex_state = 3},
  [163] = {.lex_state = 3},
  [164] = {.lex_state = 3},
  [165] = {.lex_state = 161},
  [166] = {.lex_state = 161},
  [167] = {.lex_state = 108},
  [168] = {.lex_state = 108},
  [169] = {.lex_state = 108},
  [170] = {.lex_state = 167},
  [171] = {.lex_state = 167},
  [172] = {.lex_state = 167},
  [173] = {.lex_state = 167},
  [174] = {.lex_state = 167},
  [175] = {.lex_state = 167},
  [176] = {.lex_state = 108},
  [177] = {.lex_state = 161},
  [178] = {.lex_state = 162},
  [179] = {.lex_state = 169},
  [180] = {.lex_state = 168},
  [181] = {.lex_state = 0},
  [182] = {.lex_state = 167},
  [183] = {.lex_state = 168},
  [184] = {.lex_state = 168},
  [185] = {.lex_state = 167},
  [186] = {.lex_state = 167},
  [187] = {.lex_state = 167},
  [188] = {.lex_state = 162},
  [189] = {.lex_state = 167},
  [190] = {.lex_state = 167},
  [191] = {.lex_state = 167},
  [192] = {.lex_state = 167},
  [193] = {.lex_state = 162},
  [194] = {.lex_state = 167},
  [195] = {.lex_state = 168},
  [196] = {.lex_state = 167},
  [197] = {.lex_state = 167},
  [198] = {.lex_state = 167},
  [199] = {.lex_state = 167},
  [200] = {.lex_state = 167},
  [201] = {.lex_state = 801},
  [202] = {.lex_state = 0},
  [203] = {.lex_state = 167},
  [204] = {.lex_state = 167},
  [205] = {.lex_state = 167},
  [206] = {.lex_state = 167},
  [207] = {.lex_state = 104},
  [208] = {.lex_state = 801},
  [209] = {.lex_state = 168},
  [210] = {.lex_state = 168},
  [211] = {.lex_state = 167},
  [212] = {.lex_state = 801},
  [213] = {.lex_state = 801},
  [214] = {.lex_state = 801},
  [215] = {.lex_state = 167},
  [216] = {.lex_state = 0},
  [217] = {.lex_state = 166},
  [218] = {.lex_state = 167},
  [219] = {.lex_state = 801},
  [220] = {.lex_state = 167},
  [221] = {.lex_state = 168},
  [222] = {.lex_state = 167},
  [223] = {.lex_state = 167},
  [224] = {.lex_state = 167},
  [225] = {.lex_state = 801},
  [226] = {.lex_state = 167},
  [227] = {.lex_state = 166},
  [228] = {.lex_state = 167},
  [229] = {.lex_state = 168},
  [230] = {.lex_state = 168},
  [231] = {.lex_state = 167},
  [232] = {.lex_state = 167},
  [233] = {.lex_state = 167},
  [234] = {.lex_state = 167},
  [235] = {.lex_state = 169},
  [236] = {.lex_state = 167},
  [237] = {.lex_state = 166},
  [238] = {.lex_state = 167},
  [239] = {.lex_state = 167},
  [240] = {.lex_state = 167},
  [241] = {.lex_state = 167},
  [242] = {.lex_state = 167},
  [243] = {.lex_state = 167},
  [244] = {.lex_state = 167},
  [245] = {.lex_state = 167},
  [246] = {.lex_state = 167},
  [247] = {.lex_state = 167},
  [248] = {.lex_state = 167},
  [249] = {.lex_state = 166},
  [250] = {.lex_state = 166},
  [251] = {.lex_state = 801},
  [252] = {.lex_state = 167},
  [253] = {.lex_state = 167},
  [254] = {.lex_state = 166},
  [255] = {.lex_state = 166},
  [256] = {.lex_state = 166},
  [257] = {.lex_state = 167},
  [258] = {.lex_state = 166},
  [259] = {.lex_state = 801},
  [260] = {.lex_state = 169},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [aux_sym_WORD_CHAR_token1] = ACTIONS(1),
    [aux_sym_PUNCTUATION_token1] = ACTIONS(1),
    [aux_sym_WS_token1] = ACTIONS(1),
    [aux_sym_NL_token1] = ACTIONS(1),
    [aux_sym_COMMENT_PREFIX_token1] = ACTIONS(1),
    [anon_sym_AT] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [aux_sym__var_comment_token1] = ACTIONS(1),
    [aux_sym_request_separator_token1] = ACTIONS(1),
    [sym_method] = ACTIONS(1),
    [aux_sym_http_version_token1] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_LBRACE_LBRACE] = ACTIONS(1),
    [anon_sym_LT] = ACTIONS(1),
    [anon_sym_GT] = ACTIONS(1),
    [anon_sym_LBRACE_PERCENT] = ACTIONS(1),
    [aux_sym_res_redirect_token1] = ACTIONS(1),
    [anon_sym_AT2] = ACTIONS(1),
    [aux_sym_xml_body_token1] = ACTIONS(1),
    [aux_sym_json_body_token1] = ACTIONS(1),
    [aux_sym_graphql_json_body_token1] = ACTIONS(1),
    [anon_sym_LT2] = ACTIONS(1),
    [anon_sym_DASH_DASH] = ACTIONS(1),
    [aux_sym_multipart_form_data_token2] = ACTIONS(1),
    [sym_header_entity] = ACTIONS(1),
    [aux_sym_path_token1] = ACTIONS(1),
    [aux_sym__blank_line_token1] = ACTIONS(1),
  },
  [1] = {
    [sym_document] = STATE(216),
    [sym_comment] = STATE(32),
    [sym__plain_comment] = STATE(112),
    [sym__var_comment] = STATE(116),
    [sym_request_separator] = STATE(33),
    [sym_section] = STATE(29),
    [sym__section_content] = STATE(99),
    [sym_http_version] = STATE(214),
    [aux_sym__target_url_line] = STATE(132),
    [sym_target_url] = STATE(200),
    [sym_response] = STATE(104),
    [sym_request] = STATE(105),
    [sym_variable] = STATE(132),
    [sym_pre_request_script] = STATE(32),
    [sym_variable_declaration] = STATE(32),
    [sym__blank_line] = STATE(32),
    [aux_sym_document_repeat1] = STATE(29),
    [ts_builtin_sym_end] = ACTIONS(3),
    [aux_sym_WORD_CHAR_token1] = ACTIONS(5),
    [aux_sym_PUNCTUATION_token1] = ACTIONS(5),
    [aux_sym_WS_token1] = ACTIONS(7),
    [aux_sym_COMMENT_PREFIX_token1] = ACTIONS(9),
    [aux_sym_request_separator_token1] = ACTIONS(11),
    [sym_method] = ACTIONS(13),
    [aux_sym_http_version_token1] = ACTIONS(15),
    [anon_sym_LBRACE_LBRACE] = ACTIONS(17),
    [anon_sym_LT] = ACTIONS(19),
    [anon_sym_AT2] = ACTIONS(21),
    [aux_sym__blank_line_token1] = ACTIONS(23),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 20,
    ACTIONS(29), 1,
      aux_sym_WS_token1,
    ACTIONS(31), 1,
      aux_sym_NL_token1,
    ACTIONS(33), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(35), 1,
      anon_sym_GT,
    ACTIONS(37), 1,
      aux_sym_res_redirect_token1,
    ACTIONS(39), 1,
      aux_sym_xml_body_token1,
    ACTIONS(41), 1,
      aux_sym_json_body_token1,
    ACTIONS(43), 1,
      aux_sym_graphql_data_token1,
    ACTIONS(45), 1,
      anon_sym_LT2,
    ACTIONS(47), 1,
      anon_sym_DASH_DASH,
    ACTIONS(49), 1,
      aux_sym_raw_body_token1,
    ACTIONS(51), 1,
      aux_sym__blank_line_token1,
    STATE(47), 1,
      sym_graphql_data,
    STATE(59), 1,
      sym__var_comment,
    STATE(205), 1,
      sym_external_body,
    ACTIONS(25), 2,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
    STATE(48), 2,
      sym__blank_line,
      aux_sym___body_repeat1,
    STATE(25), 3,
      sym_res_handler_script,
      sym_res_redirect,
      aux_sym___body_repeat2,
    STATE(68), 6,
      sym_xml_body,
      sym_json_body,
      sym_graphql_body,
      sym__external_body,
      sym_multipart_form_data,
      sym_raw_body,
    ACTIONS(27), 7,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [76] = 20,
    ACTIONS(29), 1,
      aux_sym_WS_token1,
    ACTIONS(33), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(35), 1,
      anon_sym_GT,
    ACTIONS(37), 1,
      aux_sym_res_redirect_token1,
    ACTIONS(39), 1,
      aux_sym_xml_body_token1,
    ACTIONS(41), 1,
      aux_sym_json_body_token1,
    ACTIONS(43), 1,
      aux_sym_graphql_data_token1,
    ACTIONS(45), 1,
      anon_sym_LT2,
    ACTIONS(47), 1,
      anon_sym_DASH_DASH,
    ACTIONS(49), 1,
      aux_sym_raw_body_token1,
    ACTIONS(51), 1,
      aux_sym__blank_line_token1,
    ACTIONS(57), 1,
      aux_sym_NL_token1,
    STATE(47), 1,
      sym_graphql_data,
    STATE(59), 1,
      sym__var_comment,
    STATE(205), 1,
      sym_external_body,
    ACTIONS(53), 2,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
    STATE(48), 2,
      sym__blank_line,
      aux_sym___body_repeat1,
    STATE(26), 3,
      sym_res_handler_script,
      sym_res_redirect,
      aux_sym___body_repeat2,
    STATE(68), 6,
      sym_xml_body,
      sym_json_body,
      sym_graphql_body,
      sym__external_body,
      sym_multipart_form_data,
      sym_raw_body,
    ACTIONS(55), 7,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [152] = 20,
    ACTIONS(29), 1,
      aux_sym_WS_token1,
    ACTIONS(33), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(35), 1,
      anon_sym_GT,
    ACTIONS(37), 1,
      aux_sym_res_redirect_token1,
    ACTIONS(39), 1,
      aux_sym_xml_body_token1,
    ACTIONS(41), 1,
      aux_sym_json_body_token1,
    ACTIONS(43), 1,
      aux_sym_graphql_data_token1,
    ACTIONS(45), 1,
      anon_sym_LT2,
    ACTIONS(47), 1,
      anon_sym_DASH_DASH,
    ACTIONS(49), 1,
      aux_sym_raw_body_token1,
    ACTIONS(51), 1,
      aux_sym__blank_line_token1,
    ACTIONS(63), 1,
      aux_sym_NL_token1,
    STATE(47), 1,
      sym_graphql_data,
    STATE(59), 1,
      sym__var_comment,
    STATE(205), 1,
      sym_external_body,
    ACTIONS(59), 2,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
    STATE(48), 2,
      sym__blank_line,
      aux_sym___body_repeat1,
    STATE(15), 3,
      sym_res_handler_script,
      sym_res_redirect,
      aux_sym___body_repeat2,
    STATE(68), 6,
      sym_xml_body,
      sym_json_body,
      sym_graphql_body,
      sym__external_body,
      sym_multipart_form_data,
      sym_raw_body,
    ACTIONS(61), 7,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [228] = 20,
    ACTIONS(29), 1,
      aux_sym_WS_token1,
    ACTIONS(33), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(35), 1,
      anon_sym_GT,
    ACTIONS(37), 1,
      aux_sym_res_redirect_token1,
    ACTIONS(39), 1,
      aux_sym_xml_body_token1,
    ACTIONS(41), 1,
      aux_sym_json_body_token1,
    ACTIONS(43), 1,
      aux_sym_graphql_data_token1,
    ACTIONS(45), 1,
      anon_sym_LT2,
    ACTIONS(47), 1,
      anon_sym_DASH_DASH,
    ACTIONS(49), 1,
      aux_sym_raw_body_token1,
    ACTIONS(51), 1,
      aux_sym__blank_line_token1,
    ACTIONS(69), 1,
      aux_sym_NL_token1,
    STATE(47), 1,
      sym_graphql_data,
    STATE(59), 1,
      sym__var_comment,
    STATE(205), 1,
      sym_external_body,
    ACTIONS(65), 2,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
    STATE(48), 2,
      sym__blank_line,
      aux_sym___body_repeat1,
    STATE(14), 3,
      sym_res_handler_script,
      sym_res_redirect,
      aux_sym___body_repeat2,
    STATE(68), 6,
      sym_xml_body,
      sym_json_body,
      sym_graphql_body,
      sym__external_body,
      sym_multipart_form_data,
      sym_raw_body,
    ACTIONS(67), 7,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [304] = 20,
    ACTIONS(29), 1,
      aux_sym_WS_token1,
    ACTIONS(33), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(35), 1,
      anon_sym_GT,
    ACTIONS(37), 1,
      aux_sym_res_redirect_token1,
    ACTIONS(39), 1,
      aux_sym_xml_body_token1,
    ACTIONS(41), 1,
      aux_sym_json_body_token1,
    ACTIONS(43), 1,
      aux_sym_graphql_data_token1,
    ACTIONS(45), 1,
      anon_sym_LT2,
    ACTIONS(47), 1,
      anon_sym_DASH_DASH,
    ACTIONS(49), 1,
      aux_sym_raw_body_token1,
    ACTIONS(51), 1,
      aux_sym__blank_line_token1,
    ACTIONS(75), 1,
      aux_sym_NL_token1,
    STATE(47), 1,
      sym_graphql_data,
    STATE(59), 1,
      sym__var_comment,
    STATE(205), 1,
      sym_external_body,
    ACTIONS(71), 2,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
    STATE(48), 2,
      sym__blank_line,
      aux_sym___body_repeat1,
    STATE(18), 3,
      sym_res_handler_script,
      sym_res_redirect,
      aux_sym___body_repeat2,
    STATE(68), 6,
      sym_xml_body,
      sym_json_body,
      sym_graphql_body,
      sym__external_body,
      sym_multipart_form_data,
      sym_raw_body,
    ACTIONS(73), 7,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [380] = 20,
    ACTIONS(29), 1,
      aux_sym_WS_token1,
    ACTIONS(33), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(35), 1,
      anon_sym_GT,
    ACTIONS(37), 1,
      aux_sym_res_redirect_token1,
    ACTIONS(39), 1,
      aux_sym_xml_body_token1,
    ACTIONS(41), 1,
      aux_sym_json_body_token1,
    ACTIONS(43), 1,
      aux_sym_graphql_data_token1,
    ACTIONS(45), 1,
      anon_sym_LT2,
    ACTIONS(47), 1,
      anon_sym_DASH_DASH,
    ACTIONS(49), 1,
      aux_sym_raw_body_token1,
    ACTIONS(51), 1,
      aux_sym__blank_line_token1,
    ACTIONS(81), 1,
      aux_sym_NL_token1,
    STATE(47), 1,
      sym_graphql_data,
    STATE(59), 1,
      sym__var_comment,
    STATE(205), 1,
      sym_external_body,
    ACTIONS(77), 2,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
    STATE(48), 2,
      sym__blank_line,
      aux_sym___body_repeat1,
    STATE(17), 3,
      sym_res_handler_script,
      sym_res_redirect,
      aux_sym___body_repeat2,
    STATE(68), 6,
      sym_xml_body,
      sym_json_body,
      sym_graphql_body,
      sym__external_body,
      sym_multipart_form_data,
      sym_raw_body,
    ACTIONS(79), 7,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [456] = 20,
    ACTIONS(29), 1,
      aux_sym_WS_token1,
    ACTIONS(33), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(35), 1,
      anon_sym_GT,
    ACTIONS(37), 1,
      aux_sym_res_redirect_token1,
    ACTIONS(39), 1,
      aux_sym_xml_body_token1,
    ACTIONS(41), 1,
      aux_sym_json_body_token1,
    ACTIONS(43), 1,
      aux_sym_graphql_data_token1,
    ACTIONS(45), 1,
      anon_sym_LT2,
    ACTIONS(47), 1,
      anon_sym_DASH_DASH,
    ACTIONS(49), 1,
      aux_sym_raw_body_token1,
    ACTIONS(51), 1,
      aux_sym__blank_line_token1,
    ACTIONS(87), 1,
      aux_sym_NL_token1,
    STATE(47), 1,
      sym_graphql_data,
    STATE(59), 1,
      sym__var_comment,
    STATE(205), 1,
      sym_external_body,
    ACTIONS(83), 2,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
    STATE(48), 2,
      sym__blank_line,
      aux_sym___body_repeat1,
    STATE(19), 3,
      sym_res_handler_script,
      sym_res_redirect,
      aux_sym___body_repeat2,
    STATE(68), 6,
      sym_xml_body,
      sym_json_body,
      sym_graphql_body,
      sym__external_body,
      sym_multipart_form_data,
      sym_raw_body,
    ACTIONS(85), 7,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [532] = 20,
    ACTIONS(29), 1,
      aux_sym_WS_token1,
    ACTIONS(33), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(35), 1,
      anon_sym_GT,
    ACTIONS(37), 1,
      aux_sym_res_redirect_token1,
    ACTIONS(39), 1,
      aux_sym_xml_body_token1,
    ACTIONS(41), 1,
      aux_sym_json_body_token1,
    ACTIONS(43), 1,
      aux_sym_graphql_data_token1,
    ACTIONS(45), 1,
      anon_sym_LT2,
    ACTIONS(47), 1,
      anon_sym_DASH_DASH,
    ACTIONS(49), 1,
      aux_sym_raw_body_token1,
    ACTIONS(51), 1,
      aux_sym__blank_line_token1,
    ACTIONS(93), 1,
      aux_sym_NL_token1,
    STATE(47), 1,
      sym_graphql_data,
    STATE(59), 1,
      sym__var_comment,
    STATE(205), 1,
      sym_external_body,
    ACTIONS(89), 2,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
    STATE(48), 2,
      sym__blank_line,
      aux_sym___body_repeat1,
    STATE(24), 3,
      sym_res_handler_script,
      sym_res_redirect,
      aux_sym___body_repeat2,
    STATE(68), 6,
      sym_xml_body,
      sym_json_body,
      sym_graphql_body,
      sym__external_body,
      sym_multipart_form_data,
      sym_raw_body,
    ACTIONS(91), 7,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [608] = 20,
    ACTIONS(29), 1,
      aux_sym_WS_token1,
    ACTIONS(33), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(35), 1,
      anon_sym_GT,
    ACTIONS(37), 1,
      aux_sym_res_redirect_token1,
    ACTIONS(39), 1,
      aux_sym_xml_body_token1,
    ACTIONS(41), 1,
      aux_sym_json_body_token1,
    ACTIONS(43), 1,
      aux_sym_graphql_data_token1,
    ACTIONS(45), 1,
      anon_sym_LT2,
    ACTIONS(47), 1,
      anon_sym_DASH_DASH,
    ACTIONS(49), 1,
      aux_sym_raw_body_token1,
    ACTIONS(51), 1,
      aux_sym__blank_line_token1,
    ACTIONS(99), 1,
      aux_sym_NL_token1,
    STATE(47), 1,
      sym_graphql_data,
    STATE(59), 1,
      sym__var_comment,
    STATE(205), 1,
      sym_external_body,
    ACTIONS(95), 2,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
    STATE(48), 2,
      sym__blank_line,
      aux_sym___body_repeat1,
    STATE(23), 3,
      sym_res_handler_script,
      sym_res_redirect,
      aux_sym___body_repeat2,
    STATE(68), 6,
      sym_xml_body,
      sym_json_body,
      sym_graphql_body,
      sym__external_body,
      sym_multipart_form_data,
      sym_raw_body,
    ACTIONS(97), 7,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [684] = 20,
    ACTIONS(29), 1,
      aux_sym_WS_token1,
    ACTIONS(33), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(35), 1,
      anon_sym_GT,
    ACTIONS(37), 1,
      aux_sym_res_redirect_token1,
    ACTIONS(39), 1,
      aux_sym_xml_body_token1,
    ACTIONS(41), 1,
      aux_sym_json_body_token1,
    ACTIONS(43), 1,
      aux_sym_graphql_data_token1,
    ACTIONS(45), 1,
      anon_sym_LT2,
    ACTIONS(47), 1,
      anon_sym_DASH_DASH,
    ACTIONS(49), 1,
      aux_sym_raw_body_token1,
    ACTIONS(51), 1,
      aux_sym__blank_line_token1,
    ACTIONS(105), 1,
      aux_sym_NL_token1,
    STATE(47), 1,
      sym_graphql_data,
    STATE(59), 1,
      sym__var_comment,
    STATE(205), 1,
      sym_external_body,
    ACTIONS(101), 2,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
    STATE(48), 2,
      sym__blank_line,
      aux_sym___body_repeat1,
    STATE(22), 3,
      sym_res_handler_script,
      sym_res_redirect,
      aux_sym___body_repeat2,
    STATE(68), 6,
      sym_xml_body,
      sym_json_body,
      sym_graphql_body,
      sym__external_body,
      sym_multipart_form_data,
      sym_raw_body,
    ACTIONS(103), 7,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [760] = 20,
    ACTIONS(29), 1,
      aux_sym_WS_token1,
    ACTIONS(33), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(35), 1,
      anon_sym_GT,
    ACTIONS(37), 1,
      aux_sym_res_redirect_token1,
    ACTIONS(39), 1,
      aux_sym_xml_body_token1,
    ACTIONS(41), 1,
      aux_sym_json_body_token1,
    ACTIONS(43), 1,
      aux_sym_graphql_data_token1,
    ACTIONS(45), 1,
      anon_sym_LT2,
    ACTIONS(47), 1,
      anon_sym_DASH_DASH,
    ACTIONS(49), 1,
      aux_sym_raw_body_token1,
    ACTIONS(51), 1,
      aux_sym__blank_line_token1,
    ACTIONS(111), 1,
      aux_sym_NL_token1,
    STATE(47), 1,
      sym_graphql_data,
    STATE(59), 1,
      sym__var_comment,
    STATE(205), 1,
      sym_external_body,
    ACTIONS(107), 2,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
    STATE(48), 2,
      sym__blank_line,
      aux_sym___body_repeat1,
    STATE(21), 3,
      sym_res_handler_script,
      sym_res_redirect,
      aux_sym___body_repeat2,
    STATE(68), 6,
      sym_xml_body,
      sym_json_body,
      sym_graphql_body,
      sym__external_body,
      sym_multipart_form_data,
      sym_raw_body,
    ACTIONS(109), 7,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [836] = 20,
    ACTIONS(29), 1,
      aux_sym_WS_token1,
    ACTIONS(33), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(35), 1,
      anon_sym_GT,
    ACTIONS(37), 1,
      aux_sym_res_redirect_token1,
    ACTIONS(39), 1,
      aux_sym_xml_body_token1,
    ACTIONS(41), 1,
      aux_sym_json_body_token1,
    ACTIONS(43), 1,
      aux_sym_graphql_data_token1,
    ACTIONS(45), 1,
      anon_sym_LT2,
    ACTIONS(47), 1,
      anon_sym_DASH_DASH,
    ACTIONS(49), 1,
      aux_sym_raw_body_token1,
    ACTIONS(51), 1,
      aux_sym__blank_line_token1,
    ACTIONS(117), 1,
      aux_sym_NL_token1,
    STATE(47), 1,
      sym_graphql_data,
    STATE(59), 1,
      sym__var_comment,
    STATE(205), 1,
      sym_external_body,
    ACTIONS(113), 2,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
    STATE(48), 2,
      sym__blank_line,
      aux_sym___body_repeat1,
    STATE(20), 3,
      sym_res_handler_script,
      sym_res_redirect,
      aux_sym___body_repeat2,
    STATE(68), 6,
      sym_xml_body,
      sym_json_body,
      sym_graphql_body,
      sym__external_body,
      sym_multipart_form_data,
      sym_raw_body,
    ACTIONS(115), 7,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [912] = 17,
    ACTIONS(33), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(35), 1,
      anon_sym_GT,
    ACTIONS(37), 1,
      aux_sym_res_redirect_token1,
    ACTIONS(39), 1,
      aux_sym_xml_body_token1,
    ACTIONS(41), 1,
      aux_sym_json_body_token1,
    ACTIONS(43), 1,
      aux_sym_graphql_data_token1,
    ACTIONS(45), 1,
      anon_sym_LT2,
    ACTIONS(47), 1,
      anon_sym_DASH_DASH,
    ACTIONS(49), 1,
      aux_sym_raw_body_token1,
    ACTIONS(123), 1,
      aux_sym_NL_token1,
    STATE(47), 1,
      sym_graphql_data,
    STATE(59), 1,
      sym__var_comment,
    STATE(205), 1,
      sym_external_body,
    ACTIONS(119), 2,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
    STATE(16), 3,
      sym_res_handler_script,
      sym_res_redirect,
      aux_sym___body_repeat2,
    STATE(68), 6,
      sym_xml_body,
      sym_json_body,
      sym_graphql_body,
      sym__external_body,
      sym_multipart_form_data,
      sym_raw_body,
    ACTIONS(121), 9,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
      aux_sym__blank_line_token1,
  [980] = 17,
    ACTIONS(33), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(35), 1,
      anon_sym_GT,
    ACTIONS(37), 1,
      aux_sym_res_redirect_token1,
    ACTIONS(39), 1,
      aux_sym_xml_body_token1,
    ACTIONS(41), 1,
      aux_sym_json_body_token1,
    ACTIONS(43), 1,
      aux_sym_graphql_data_token1,
    ACTIONS(45), 1,
      anon_sym_LT2,
    ACTIONS(47), 1,
      anon_sym_DASH_DASH,
    ACTIONS(49), 1,
      aux_sym_raw_body_token1,
    ACTIONS(123), 1,
      aux_sym_NL_token1,
    STATE(47), 1,
      sym_graphql_data,
    STATE(59), 1,
      sym__var_comment,
    STATE(205), 1,
      sym_external_body,
    ACTIONS(125), 2,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
    STATE(16), 3,
      sym_res_handler_script,
      sym_res_redirect,
      aux_sym___body_repeat2,
    STATE(68), 6,
      sym_xml_body,
      sym_json_body,
      sym_graphql_body,
      sym__external_body,
      sym_multipart_form_data,
      sym_raw_body,
    ACTIONS(127), 9,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
      aux_sym__blank_line_token1,
  [1048] = 17,
    ACTIONS(133), 1,
      aux_sym_NL_token1,
    ACTIONS(136), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(139), 1,
      anon_sym_GT,
    ACTIONS(142), 1,
      aux_sym_res_redirect_token1,
    ACTIONS(145), 1,
      aux_sym_xml_body_token1,
    ACTIONS(148), 1,
      aux_sym_json_body_token1,
    ACTIONS(151), 1,
      aux_sym_graphql_data_token1,
    ACTIONS(154), 1,
      anon_sym_LT2,
    ACTIONS(157), 1,
      anon_sym_DASH_DASH,
    ACTIONS(160), 1,
      aux_sym_raw_body_token1,
    STATE(47), 1,
      sym_graphql_data,
    STATE(59), 1,
      sym__var_comment,
    STATE(205), 1,
      sym_external_body,
    ACTIONS(129), 2,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
    STATE(16), 3,
      sym_res_handler_script,
      sym_res_redirect,
      aux_sym___body_repeat2,
    STATE(68), 6,
      sym_xml_body,
      sym_json_body,
      sym_graphql_body,
      sym__external_body,
      sym_multipart_form_data,
      sym_raw_body,
    ACTIONS(131), 9,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
      aux_sym__blank_line_token1,
  [1116] = 17,
    ACTIONS(33), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(35), 1,
      anon_sym_GT,
    ACTIONS(37), 1,
      aux_sym_res_redirect_token1,
    ACTIONS(39), 1,
      aux_sym_xml_body_token1,
    ACTIONS(41), 1,
      aux_sym_json_body_token1,
    ACTIONS(43), 1,
      aux_sym_graphql_data_token1,
    ACTIONS(45), 1,
      anon_sym_LT2,
    ACTIONS(47), 1,
      anon_sym_DASH_DASH,
    ACTIONS(49), 1,
      aux_sym_raw_body_token1,
    ACTIONS(123), 1,
      aux_sym_NL_token1,
    STATE(47), 1,
      sym_graphql_data,
    STATE(59), 1,
      sym__var_comment,
    STATE(205), 1,
      sym_external_body,
    ACTIONS(163), 2,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
    STATE(16), 3,
      sym_res_handler_script,
      sym_res_redirect,
      aux_sym___body_repeat2,
    STATE(68), 6,
      sym_xml_body,
      sym_json_body,
      sym_graphql_body,
      sym__external_body,
      sym_multipart_form_data,
      sym_raw_body,
    ACTIONS(165), 9,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
      aux_sym__blank_line_token1,
  [1184] = 17,
    ACTIONS(33), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(35), 1,
      anon_sym_GT,
    ACTIONS(37), 1,
      aux_sym_res_redirect_token1,
    ACTIONS(39), 1,
      aux_sym_xml_body_token1,
    ACTIONS(41), 1,
      aux_sym_json_body_token1,
    ACTIONS(43), 1,
      aux_sym_graphql_data_token1,
    ACTIONS(45), 1,
      anon_sym_LT2,
    ACTIONS(47), 1,
      anon_sym_DASH_DASH,
    ACTIONS(49), 1,
      aux_sym_raw_body_token1,
    ACTIONS(123), 1,
      aux_sym_NL_token1,
    STATE(47), 1,
      sym_graphql_data,
    STATE(59), 1,
      sym__var_comment,
    STATE(205), 1,
      sym_external_body,
    ACTIONS(167), 2,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
    STATE(16), 3,
      sym_res_handler_script,
      sym_res_redirect,
      aux_sym___body_repeat2,
    STATE(68), 6,
      sym_xml_body,
      sym_json_body,
      sym_graphql_body,
      sym__external_body,
      sym_multipart_form_data,
      sym_raw_body,
    ACTIONS(169), 9,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
      aux_sym__blank_line_token1,
  [1252] = 17,
    ACTIONS(33), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(35), 1,
      anon_sym_GT,
    ACTIONS(37), 1,
      aux_sym_res_redirect_token1,
    ACTIONS(39), 1,
      aux_sym_xml_body_token1,
    ACTIONS(41), 1,
      aux_sym_json_body_token1,
    ACTIONS(43), 1,
      aux_sym_graphql_data_token1,
    ACTIONS(45), 1,
      anon_sym_LT2,
    ACTIONS(47), 1,
      anon_sym_DASH_DASH,
    ACTIONS(49), 1,
      aux_sym_raw_body_token1,
    ACTIONS(123), 1,
      aux_sym_NL_token1,
    STATE(47), 1,
      sym_graphql_data,
    STATE(59), 1,
      sym__var_comment,
    STATE(205), 1,
      sym_external_body,
    ACTIONS(171), 2,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
    STATE(16), 3,
      sym_res_handler_script,
      sym_res_redirect,
      aux_sym___body_repeat2,
    STATE(68), 6,
      sym_xml_body,
      sym_json_body,
      sym_graphql_body,
      sym__external_body,
      sym_multipart_form_data,
      sym_raw_body,
    ACTIONS(173), 9,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
      aux_sym__blank_line_token1,
  [1320] = 17,
    ACTIONS(33), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(35), 1,
      anon_sym_GT,
    ACTIONS(37), 1,
      aux_sym_res_redirect_token1,
    ACTIONS(39), 1,
      aux_sym_xml_body_token1,
    ACTIONS(41), 1,
      aux_sym_json_body_token1,
    ACTIONS(43), 1,
      aux_sym_graphql_data_token1,
    ACTIONS(45), 1,
      anon_sym_LT2,
    ACTIONS(47), 1,
      anon_sym_DASH_DASH,
    ACTIONS(49), 1,
      aux_sym_raw_body_token1,
    ACTIONS(123), 1,
      aux_sym_NL_token1,
    STATE(47), 1,
      sym_graphql_data,
    STATE(59), 1,
      sym__var_comment,
    STATE(205), 1,
      sym_external_body,
    ACTIONS(175), 2,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
    STATE(16), 3,
      sym_res_handler_script,
      sym_res_redirect,
      aux_sym___body_repeat2,
    STATE(68), 6,
      sym_xml_body,
      sym_json_body,
      sym_graphql_body,
      sym__external_body,
      sym_multipart_form_data,
      sym_raw_body,
    ACTIONS(177), 9,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
      aux_sym__blank_line_token1,
  [1388] = 17,
    ACTIONS(33), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(35), 1,
      anon_sym_GT,
    ACTIONS(37), 1,
      aux_sym_res_redirect_token1,
    ACTIONS(39), 1,
      aux_sym_xml_body_token1,
    ACTIONS(41), 1,
      aux_sym_json_body_token1,
    ACTIONS(43), 1,
      aux_sym_graphql_data_token1,
    ACTIONS(45), 1,
      anon_sym_LT2,
    ACTIONS(47), 1,
      anon_sym_DASH_DASH,
    ACTIONS(49), 1,
      aux_sym_raw_body_token1,
    ACTIONS(123), 1,
      aux_sym_NL_token1,
    STATE(47), 1,
      sym_graphql_data,
    STATE(59), 1,
      sym__var_comment,
    STATE(205), 1,
      sym_external_body,
    ACTIONS(179), 2,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
    STATE(16), 3,
      sym_res_handler_script,
      sym_res_redirect,
      aux_sym___body_repeat2,
    STATE(68), 6,
      sym_xml_body,
      sym_json_body,
      sym_graphql_body,
      sym__external_body,
      sym_multipart_form_data,
      sym_raw_body,
    ACTIONS(181), 9,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
      aux_sym__blank_line_token1,
  [1456] = 17,
    ACTIONS(33), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(35), 1,
      anon_sym_GT,
    ACTIONS(37), 1,
      aux_sym_res_redirect_token1,
    ACTIONS(39), 1,
      aux_sym_xml_body_token1,
    ACTIONS(41), 1,
      aux_sym_json_body_token1,
    ACTIONS(43), 1,
      aux_sym_graphql_data_token1,
    ACTIONS(45), 1,
      anon_sym_LT2,
    ACTIONS(47), 1,
      anon_sym_DASH_DASH,
    ACTIONS(49), 1,
      aux_sym_raw_body_token1,
    ACTIONS(123), 1,
      aux_sym_NL_token1,
    STATE(47), 1,
      sym_graphql_data,
    STATE(59), 1,
      sym__var_comment,
    STATE(205), 1,
      sym_external_body,
    ACTIONS(183), 2,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
    STATE(16), 3,
      sym_res_handler_script,
      sym_res_redirect,
      aux_sym___body_repeat2,
    STATE(68), 6,
      sym_xml_body,
      sym_json_body,
      sym_graphql_body,
      sym__external_body,
      sym_multipart_form_data,
      sym_raw_body,
    ACTIONS(185), 9,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
      aux_sym__blank_line_token1,
  [1524] = 17,
    ACTIONS(33), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(35), 1,
      anon_sym_GT,
    ACTIONS(37), 1,
      aux_sym_res_redirect_token1,
    ACTIONS(39), 1,
      aux_sym_xml_body_token1,
    ACTIONS(41), 1,
      aux_sym_json_body_token1,
    ACTIONS(43), 1,
      aux_sym_graphql_data_token1,
    ACTIONS(45), 1,
      anon_sym_LT2,
    ACTIONS(47), 1,
      anon_sym_DASH_DASH,
    ACTIONS(49), 1,
      aux_sym_raw_body_token1,
    ACTIONS(123), 1,
      aux_sym_NL_token1,
    STATE(47), 1,
      sym_graphql_data,
    STATE(59), 1,
      sym__var_comment,
    STATE(205), 1,
      sym_external_body,
    ACTIONS(187), 2,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
    STATE(16), 3,
      sym_res_handler_script,
      sym_res_redirect,
      aux_sym___body_repeat2,
    STATE(68), 6,
      sym_xml_body,
      sym_json_body,
      sym_graphql_body,
      sym__external_body,
      sym_multipart_form_data,
      sym_raw_body,
    ACTIONS(189), 9,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
      aux_sym__blank_line_token1,
  [1592] = 17,
    ACTIONS(33), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(35), 1,
      anon_sym_GT,
    ACTIONS(37), 1,
      aux_sym_res_redirect_token1,
    ACTIONS(39), 1,
      aux_sym_xml_body_token1,
    ACTIONS(41), 1,
      aux_sym_json_body_token1,
    ACTIONS(43), 1,
      aux_sym_graphql_data_token1,
    ACTIONS(45), 1,
      anon_sym_LT2,
    ACTIONS(47), 1,
      anon_sym_DASH_DASH,
    ACTIONS(49), 1,
      aux_sym_raw_body_token1,
    ACTIONS(123), 1,
      aux_sym_NL_token1,
    STATE(47), 1,
      sym_graphql_data,
    STATE(59), 1,
      sym__var_comment,
    STATE(205), 1,
      sym_external_body,
    ACTIONS(191), 2,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
    STATE(16), 3,
      sym_res_handler_script,
      sym_res_redirect,
      aux_sym___body_repeat2,
    STATE(68), 6,
      sym_xml_body,
      sym_json_body,
      sym_graphql_body,
      sym__external_body,
      sym_multipart_form_data,
      sym_raw_body,
    ACTIONS(193), 9,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
      aux_sym__blank_line_token1,
  [1660] = 17,
    ACTIONS(33), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(35), 1,
      anon_sym_GT,
    ACTIONS(37), 1,
      aux_sym_res_redirect_token1,
    ACTIONS(39), 1,
      aux_sym_xml_body_token1,
    ACTIONS(41), 1,
      aux_sym_json_body_token1,
    ACTIONS(43), 1,
      aux_sym_graphql_data_token1,
    ACTIONS(45), 1,
      anon_sym_LT2,
    ACTIONS(47), 1,
      anon_sym_DASH_DASH,
    ACTIONS(49), 1,
      aux_sym_raw_body_token1,
    ACTIONS(123), 1,
      aux_sym_NL_token1,
    STATE(47), 1,
      sym_graphql_data,
    STATE(59), 1,
      sym__var_comment,
    STATE(205), 1,
      sym_external_body,
    ACTIONS(195), 2,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
    STATE(16), 3,
      sym_res_handler_script,
      sym_res_redirect,
      aux_sym___body_repeat2,
    STATE(68), 6,
      sym_xml_body,
      sym_json_body,
      sym_graphql_body,
      sym__external_body,
      sym_multipart_form_data,
      sym_raw_body,
    ACTIONS(197), 9,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
      aux_sym__blank_line_token1,
  [1728] = 17,
    ACTIONS(33), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(35), 1,
      anon_sym_GT,
    ACTIONS(37), 1,
      aux_sym_res_redirect_token1,
    ACTIONS(39), 1,
      aux_sym_xml_body_token1,
    ACTIONS(41), 1,
      aux_sym_json_body_token1,
    ACTIONS(43), 1,
      aux_sym_graphql_data_token1,
    ACTIONS(45), 1,
      anon_sym_LT2,
    ACTIONS(47), 1,
      anon_sym_DASH_DASH,
    ACTIONS(49), 1,
      aux_sym_raw_body_token1,
    ACTIONS(123), 1,
      aux_sym_NL_token1,
    STATE(47), 1,
      sym_graphql_data,
    STATE(59), 1,
      sym__var_comment,
    STATE(205), 1,
      sym_external_body,
    ACTIONS(199), 2,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
    STATE(16), 3,
      sym_res_handler_script,
      sym_res_redirect,
      aux_sym___body_repeat2,
    STATE(68), 6,
      sym_xml_body,
      sym_json_body,
      sym_graphql_body,
      sym__external_body,
      sym_multipart_form_data,
      sym_raw_body,
    ACTIONS(201), 9,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
      aux_sym__blank_line_token1,
  [1796] = 22,
    ACTIONS(203), 1,
      ts_builtin_sym_end,
    ACTIONS(208), 1,
      aux_sym_WS_token1,
    ACTIONS(211), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(214), 1,
      aux_sym_request_separator_token1,
    ACTIONS(217), 1,
      sym_method,
    ACTIONS(220), 1,
      aux_sym_http_version_token1,
    ACTIONS(223), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(226), 1,
      anon_sym_LT,
    ACTIONS(229), 1,
      anon_sym_AT2,
    ACTIONS(232), 1,
      aux_sym__blank_line_token1,
    STATE(33), 1,
      sym_request_separator,
    STATE(99), 1,
      sym__section_content,
    STATE(104), 1,
      sym_response,
    STATE(105), 1,
      sym_request,
    STATE(112), 1,
      sym__plain_comment,
    STATE(116), 1,
      sym__var_comment,
    STATE(200), 1,
      sym_target_url,
    STATE(214), 1,
      sym_http_version,
    ACTIONS(205), 2,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
    STATE(27), 2,
      sym_section,
      aux_sym_document_repeat1,
    STATE(132), 2,
      aux_sym__target_url_line,
      sym_variable,
    STATE(32), 4,
      sym_comment,
      sym_pre_request_script,
      sym_variable_declaration,
      sym__blank_line,
  [1869] = 10,
    ACTIONS(45), 1,
      anon_sym_LT2,
    ACTIONS(239), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(243), 1,
      aux_sym_multipart_form_data_token1,
    STATE(38), 1,
      sym__plain_comment,
    STATE(39), 1,
      sym__var_comment,
    STATE(198), 1,
      sym_external_body,
    STATE(30), 2,
      sym_comment,
      aux_sym_multipart_form_data_repeat1,
    ACTIONS(235), 4,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_graphql_data_token1,
    ACTIONS(241), 4,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      anon_sym_DASH_DASH,
      aux_sym_multipart_form_data_token2,
    ACTIONS(237), 12,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [1918] = 22,
    ACTIONS(7), 1,
      aux_sym_WS_token1,
    ACTIONS(9), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(11), 1,
      aux_sym_request_separator_token1,
    ACTIONS(13), 1,
      sym_method,
    ACTIONS(15), 1,
      aux_sym_http_version_token1,
    ACTIONS(17), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(19), 1,
      anon_sym_LT,
    ACTIONS(21), 1,
      anon_sym_AT2,
    ACTIONS(23), 1,
      aux_sym__blank_line_token1,
    ACTIONS(245), 1,
      ts_builtin_sym_end,
    STATE(33), 1,
      sym_request_separator,
    STATE(99), 1,
      sym__section_content,
    STATE(104), 1,
      sym_response,
    STATE(105), 1,
      sym_request,
    STATE(112), 1,
      sym__plain_comment,
    STATE(116), 1,
      sym__var_comment,
    STATE(200), 1,
      sym_target_url,
    STATE(214), 1,
      sym_http_version,
    ACTIONS(5), 2,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
    STATE(27), 2,
      sym_section,
      aux_sym_document_repeat1,
    STATE(132), 2,
      aux_sym__target_url_line,
      sym_variable,
    STATE(32), 4,
      sym_comment,
      sym_pre_request_script,
      sym_variable_declaration,
      sym__blank_line,
  [1991] = 10,
    ACTIONS(45), 1,
      anon_sym_LT2,
    ACTIONS(239), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(253), 1,
      aux_sym_multipart_form_data_token1,
    STATE(38), 1,
      sym__plain_comment,
    STATE(39), 1,
      sym__var_comment,
    STATE(198), 1,
      sym_external_body,
    STATE(31), 2,
      sym_comment,
      aux_sym_multipart_form_data_repeat1,
    ACTIONS(247), 4,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_graphql_data_token1,
    ACTIONS(251), 4,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      anon_sym_DASH_DASH,
      aux_sym_multipart_form_data_token2,
    ACTIONS(249), 12,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [2040] = 10,
    ACTIONS(259), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(265), 1,
      anon_sym_LT2,
    ACTIONS(268), 1,
      aux_sym_multipart_form_data_token1,
    STATE(38), 1,
      sym__plain_comment,
    STATE(39), 1,
      sym__var_comment,
    STATE(198), 1,
      sym_external_body,
    STATE(31), 2,
      sym_comment,
      aux_sym_multipart_form_data_repeat1,
    ACTIONS(255), 4,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_graphql_data_token1,
    ACTIONS(262), 4,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      anon_sym_DASH_DASH,
      aux_sym_multipart_form_data_token2,
    ACTIONS(257), 12,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [2089] = 19,
    ACTIONS(276), 1,
      aux_sym_WS_token1,
    ACTIONS(279), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(282), 1,
      sym_method,
    ACTIONS(285), 1,
      aux_sym_http_version_token1,
    ACTIONS(288), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(291), 1,
      anon_sym_LT,
    ACTIONS(294), 1,
      anon_sym_AT2,
    ACTIONS(297), 1,
      aux_sym__blank_line_token1,
    STATE(104), 1,
      sym_response,
    STATE(105), 1,
      sym_request,
    STATE(112), 1,
      sym__plain_comment,
    STATE(114), 1,
      sym__section_content,
    STATE(116), 1,
      sym__var_comment,
    STATE(200), 1,
      sym_target_url,
    STATE(214), 1,
      sym_http_version,
    ACTIONS(271), 2,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
    ACTIONS(273), 2,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
    STATE(132), 2,
      aux_sym__target_url_line,
      sym_variable,
    STATE(32), 4,
      sym_comment,
      sym_pre_request_script,
      sym_variable_declaration,
      sym__blank_line,
  [2153] = 19,
    ACTIONS(7), 1,
      aux_sym_WS_token1,
    ACTIONS(9), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(13), 1,
      sym_method,
    ACTIONS(15), 1,
      aux_sym_http_version_token1,
    ACTIONS(17), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(19), 1,
      anon_sym_LT,
    ACTIONS(21), 1,
      anon_sym_AT2,
    ACTIONS(23), 1,
      aux_sym__blank_line_token1,
    STATE(104), 1,
      sym_response,
    STATE(105), 1,
      sym_request,
    STATE(112), 1,
      sym__plain_comment,
    STATE(115), 1,
      sym__section_content,
    STATE(116), 1,
      sym__var_comment,
    STATE(200), 1,
      sym_target_url,
    STATE(214), 1,
      sym_http_version,
    ACTIONS(5), 2,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
    ACTIONS(300), 2,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
    STATE(132), 2,
      aux_sym__target_url_line,
      sym_variable,
    STATE(32), 4,
      sym_comment,
      sym_pre_request_script,
      sym_variable_declaration,
      sym__blank_line,
  [2217] = 5,
    ACTIONS(306), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(309), 1,
      aux_sym__raw_body_token1,
    STATE(51), 1,
      sym__raw_body,
    ACTIONS(302), 4,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_graphql_json_body_token1,
    ACTIONS(304), 17,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_LT2,
      anon_sym_DASH_DASH,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [2252] = 5,
    ACTIONS(309), 1,
      aux_sym__raw_body_token1,
    ACTIONS(315), 1,
      aux_sym_COMMENT_PREFIX_token1,
    STATE(50), 1,
      sym__raw_body,
    ACTIONS(311), 4,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_graphql_json_body_token1,
    ACTIONS(313), 17,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_LT2,
      anon_sym_DASH_DASH,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [2287] = 2,
    ACTIONS(318), 8,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_DASH_DASH,
      aux_sym_multipart_form_data_token2,
    ACTIONS(320), 15,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      anon_sym_LT2,
      aux_sym_multipart_form_data_token1,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [2315] = 5,
    ACTIONS(322), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(325), 1,
      aux_sym__raw_body_token1,
    STATE(67), 1,
      sym__raw_body,
    ACTIONS(311), 3,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
    ACTIONS(313), 17,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_LT2,
      anon_sym_DASH_DASH,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [2349] = 2,
    ACTIONS(327), 8,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_DASH_DASH,
      aux_sym_multipart_form_data_token2,
    ACTIONS(329), 15,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      anon_sym_LT2,
      aux_sym_multipart_form_data_token1,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [2377] = 2,
    ACTIONS(331), 8,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_DASH_DASH,
      aux_sym_multipart_form_data_token2,
    ACTIONS(333), 15,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      anon_sym_LT2,
      aux_sym_multipart_form_data_token1,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [2405] = 2,
    ACTIONS(255), 8,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_DASH_DASH,
      aux_sym_multipart_form_data_token2,
    ACTIONS(257), 15,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      anon_sym_LT2,
      aux_sym_multipart_form_data_token1,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [2433] = 5,
    ACTIONS(325), 1,
      aux_sym__raw_body_token1,
    ACTIONS(339), 1,
      aux_sym_COMMENT_PREFIX_token1,
    STATE(61), 1,
      sym__raw_body,
    ACTIONS(335), 3,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
    ACTIONS(337), 17,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_LT2,
      anon_sym_DASH_DASH,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [2467] = 5,
    ACTIONS(325), 1,
      aux_sym__raw_body_token1,
    ACTIONS(342), 1,
      aux_sym_COMMENT_PREFIX_token1,
    STATE(64), 1,
      sym__raw_body,
    ACTIONS(302), 3,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
    ACTIONS(304), 17,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_LT2,
      anon_sym_DASH_DASH,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [2501] = 5,
    ACTIONS(325), 1,
      aux_sym__raw_body_token1,
    ACTIONS(349), 1,
      aux_sym_COMMENT_PREFIX_token1,
    STATE(60), 1,
      sym__raw_body,
    ACTIONS(345), 3,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
    ACTIONS(347), 17,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_LT2,
      anon_sym_DASH_DASH,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [2535] = 2,
    ACTIONS(352), 8,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_DASH_DASH,
      aux_sym_multipart_form_data_token2,
    ACTIONS(354), 15,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      anon_sym_LT2,
      aux_sym_multipart_form_data_token1,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [2563] = 2,
    ACTIONS(356), 8,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_DASH_DASH,
      aux_sym_multipart_form_data_token2,
    ACTIONS(358), 15,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      anon_sym_LT2,
      aux_sym_multipart_form_data_token1,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [2591] = 2,
    ACTIONS(360), 8,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_DASH_DASH,
      aux_sym_multipart_form_data_token2,
    ACTIONS(362), 15,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      anon_sym_LT2,
      aux_sym_multipart_form_data_token1,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [2619] = 4,
    ACTIONS(368), 1,
      aux_sym_graphql_json_body_token1,
    STATE(52), 1,
      sym_graphql_json_body,
    ACTIONS(364), 6,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_xml_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_DASH_DASH,
    ACTIONS(366), 15,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      aux_sym_json_body_token1,
      anon_sym_LT2,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [2651] = 5,
    ACTIONS(374), 1,
      aux_sym_WS_token1,
    ACTIONS(377), 1,
      aux_sym__blank_line_token1,
    STATE(48), 2,
      sym__blank_line,
      aux_sym___body_repeat1,
    ACTIONS(370), 7,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_DASH_DASH,
    ACTIONS(372), 12,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_NL_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      anon_sym_LT2,
      aux_sym_raw_body_token1,
  [2685] = 2,
    ACTIONS(380), 7,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_xml_body_token1,
      aux_sym_graphql_data_token1,
      aux_sym_graphql_json_body_token1,
      anon_sym_DASH_DASH,
    ACTIONS(382), 15,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      aux_sym_json_body_token1,
      anon_sym_LT2,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [2712] = 2,
    ACTIONS(302), 7,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_xml_body_token1,
      aux_sym_graphql_data_token1,
      aux_sym_graphql_json_body_token1,
      anon_sym_DASH_DASH,
    ACTIONS(304), 15,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      aux_sym_json_body_token1,
      anon_sym_LT2,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [2739] = 2,
    ACTIONS(384), 7,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_xml_body_token1,
      aux_sym_graphql_data_token1,
      aux_sym_graphql_json_body_token1,
      anon_sym_DASH_DASH,
    ACTIONS(386), 15,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      aux_sym_json_body_token1,
      anon_sym_LT2,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [2766] = 2,
    ACTIONS(388), 7,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_DASH_DASH,
    ACTIONS(390), 14,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      anon_sym_LT2,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [2792] = 2,
    ACTIONS(392), 7,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_DASH_DASH,
    ACTIONS(394), 14,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      anon_sym_LT2,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [2818] = 2,
    ACTIONS(396), 7,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_DASH_DASH,
    ACTIONS(398), 14,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      anon_sym_LT2,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [2844] = 2,
    ACTIONS(360), 7,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_DASH_DASH,
    ACTIONS(362), 14,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      anon_sym_LT2,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [2870] = 2,
    ACTIONS(356), 7,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_DASH_DASH,
    ACTIONS(358), 14,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      anon_sym_LT2,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [2896] = 2,
    ACTIONS(400), 7,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_DASH_DASH,
    ACTIONS(402), 14,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      anon_sym_LT2,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [2922] = 2,
    ACTIONS(404), 7,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_DASH_DASH,
    ACTIONS(406), 14,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      anon_sym_LT2,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [2948] = 2,
    ACTIONS(408), 7,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_DASH_DASH,
    ACTIONS(410), 14,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      anon_sym_LT2,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [2974] = 2,
    ACTIONS(335), 7,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_DASH_DASH,
    ACTIONS(337), 14,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      anon_sym_LT2,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [3000] = 2,
    ACTIONS(412), 7,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_DASH_DASH,
    ACTIONS(414), 14,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      anon_sym_LT2,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [3026] = 2,
    ACTIONS(352), 7,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_DASH_DASH,
    ACTIONS(354), 14,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      anon_sym_LT2,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [3052] = 2,
    ACTIONS(416), 7,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_DASH_DASH,
    ACTIONS(418), 14,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      anon_sym_LT2,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [3078] = 2,
    ACTIONS(384), 7,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_DASH_DASH,
    ACTIONS(386), 14,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      anon_sym_LT2,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [3104] = 2,
    ACTIONS(420), 7,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_DASH_DASH,
    ACTIONS(422), 14,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      anon_sym_LT2,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [3130] = 2,
    ACTIONS(424), 7,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_DASH_DASH,
    ACTIONS(426), 14,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      anon_sym_LT2,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [3156] = 2,
    ACTIONS(302), 7,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_DASH_DASH,
    ACTIONS(304), 14,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      anon_sym_LT2,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [3182] = 2,
    ACTIONS(428), 7,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_res_redirect_token1,
      aux_sym_xml_body_token1,
      aux_sym_json_body_token1,
      aux_sym_graphql_data_token1,
      anon_sym_DASH_DASH,
    ACTIONS(430), 14,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AT2,
      anon_sym_LT2,
      aux_sym_raw_body_token1,
      aux_sym__blank_line_token1,
  [3208] = 11,
    ACTIONS(436), 1,
      aux_sym_WS_token1,
    ACTIONS(438), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(440), 1,
      sym_header_entity,
    ACTIONS(442), 1,
      aux_sym__blank_line_token1,
    STATE(86), 1,
      sym_header,
    STATE(92), 1,
      sym__var_comment,
    STATE(95), 1,
      sym__plain_comment,
    STATE(12), 2,
      sym__blank_line,
      aux_sym___body_repeat1,
    STATE(77), 2,
      sym_comment,
      aux_sym_request_repeat1,
    ACTIONS(434), 3,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      sym_method,
    ACTIONS(432), 6,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [3251] = 11,
    ACTIONS(436), 1,
      aux_sym_WS_token1,
    ACTIONS(438), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(440), 1,
      sym_header_entity,
    ACTIONS(448), 1,
      aux_sym__blank_line_token1,
    STATE(86), 1,
      sym_header,
    STATE(92), 1,
      sym__var_comment,
    STATE(95), 1,
      sym__plain_comment,
    STATE(6), 2,
      sym__blank_line,
      aux_sym___body_repeat1,
    STATE(75), 2,
      sym_comment,
      aux_sym_request_repeat1,
    ACTIONS(446), 3,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      sym_method,
    ACTIONS(444), 6,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [3294] = 11,
    ACTIONS(436), 1,
      aux_sym_WS_token1,
    ACTIONS(438), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(440), 1,
      sym_header_entity,
    ACTIONS(454), 1,
      aux_sym__blank_line_token1,
    STATE(86), 1,
      sym_header,
    STATE(92), 1,
      sym__var_comment,
    STATE(95), 1,
      sym__plain_comment,
    STATE(4), 2,
      sym__blank_line,
      aux_sym___body_repeat1,
    STATE(77), 2,
      sym_comment,
      aux_sym_request_repeat1,
    ACTIONS(452), 3,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      sym_method,
    ACTIONS(450), 6,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [3337] = 11,
    ACTIONS(436), 1,
      aux_sym_WS_token1,
    ACTIONS(438), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(440), 1,
      sym_header_entity,
    ACTIONS(460), 1,
      aux_sym__blank_line_token1,
    STATE(86), 1,
      sym_header,
    STATE(92), 1,
      sym__var_comment,
    STATE(95), 1,
      sym__plain_comment,
    STATE(5), 2,
      sym__blank_line,
      aux_sym___body_repeat1,
    STATE(69), 2,
      sym_comment,
      aux_sym_request_repeat1,
    ACTIONS(458), 3,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      sym_method,
    ACTIONS(456), 6,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [3380] = 11,
    ACTIONS(436), 1,
      aux_sym_WS_token1,
    ACTIONS(438), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(440), 1,
      sym_header_entity,
    ACTIONS(466), 1,
      aux_sym__blank_line_token1,
    STATE(86), 1,
      sym_header,
    STATE(92), 1,
      sym__var_comment,
    STATE(95), 1,
      sym__plain_comment,
    STATE(7), 2,
      sym__blank_line,
      aux_sym___body_repeat1,
    STATE(77), 2,
      sym_comment,
      aux_sym_request_repeat1,
    ACTIONS(464), 3,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      sym_method,
    ACTIONS(462), 6,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [3423] = 11,
    ACTIONS(436), 1,
      aux_sym_WS_token1,
    ACTIONS(438), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(440), 1,
      sym_header_entity,
    ACTIONS(472), 1,
      aux_sym__blank_line_token1,
    STATE(86), 1,
      sym_header,
    STATE(92), 1,
      sym__var_comment,
    STATE(95), 1,
      sym__plain_comment,
    STATE(8), 2,
      sym__blank_line,
      aux_sym___body_repeat1,
    STATE(73), 2,
      sym_comment,
      aux_sym_request_repeat1,
    ACTIONS(470), 3,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      sym_method,
    ACTIONS(468), 6,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [3466] = 11,
    ACTIONS(436), 1,
      aux_sym_WS_token1,
    ACTIONS(438), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(440), 1,
      sym_header_entity,
    ACTIONS(478), 1,
      aux_sym__blank_line_token1,
    STATE(86), 1,
      sym_header,
    STATE(92), 1,
      sym__var_comment,
    STATE(95), 1,
      sym__plain_comment,
    STATE(2), 2,
      sym__blank_line,
      aux_sym___body_repeat1,
    STATE(77), 2,
      sym_comment,
      aux_sym_request_repeat1,
    ACTIONS(476), 3,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      sym_method,
    ACTIONS(474), 6,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [3509] = 11,
    ACTIONS(436), 1,
      aux_sym_WS_token1,
    ACTIONS(438), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(440), 1,
      sym_header_entity,
    ACTIONS(484), 1,
      aux_sym__blank_line_token1,
    STATE(86), 1,
      sym_header,
    STATE(92), 1,
      sym__var_comment,
    STATE(95), 1,
      sym__plain_comment,
    STATE(11), 2,
      sym__blank_line,
      aux_sym___body_repeat1,
    STATE(71), 2,
      sym_comment,
      aux_sym_request_repeat1,
    ACTIONS(482), 3,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      sym_method,
    ACTIONS(480), 6,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [3552] = 8,
    ACTIONS(490), 1,
      aux_sym_COMMENT_PREFIX_token1,
    ACTIONS(493), 1,
      sym_header_entity,
    STATE(86), 1,
      sym_header,
    STATE(92), 1,
      sym__var_comment,
    STATE(95), 1,
      sym__plain_comment,
    STATE(77), 2,
      sym_comment,
      aux_sym_request_repeat1,
    ACTIONS(488), 4,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      sym_method,
      aux_sym__blank_line_token1,
    ACTIONS(486), 7,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [3587] = 8,
    ACTIONS(436), 1,
      aux_sym_WS_token1,
    ACTIONS(440), 1,
      sym_header_entity,
    ACTIONS(500), 1,
      aux_sym__blank_line_token1,
    STATE(82), 1,
      aux_sym_response_repeat1,
    STATE(87), 1,
      sym_header,
    STATE(13), 2,
      sym__blank_line,
      aux_sym___body_repeat1,
    ACTIONS(498), 4,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
    ACTIONS(496), 6,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [3621] = 8,
    ACTIONS(436), 1,
      aux_sym_WS_token1,
    ACTIONS(440), 1,
      sym_header_entity,
    ACTIONS(506), 1,
      aux_sym__blank_line_token1,
    STATE(80), 1,
      aux_sym_response_repeat1,
    STATE(87), 1,
      sym_header,
    STATE(3), 2,
      sym__blank_line,
      aux_sym___body_repeat1,
    ACTIONS(504), 4,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
    ACTIONS(502), 6,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [3655] = 8,
    ACTIONS(436), 1,
      aux_sym_WS_token1,
    ACTIONS(440), 1,
      sym_header_entity,
    ACTIONS(512), 1,
      aux_sym__blank_line_token1,
    STATE(82), 1,
      aux_sym_response_repeat1,
    STATE(87), 1,
      sym_header,
    STATE(9), 2,
      sym__blank_line,
      aux_sym___body_repeat1,
    ACTIONS(510), 4,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
    ACTIONS(508), 6,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [3689] = 8,
    ACTIONS(436), 1,
      aux_sym_WS_token1,
    ACTIONS(440), 1,
      sym_header_entity,
    ACTIONS(514), 1,
      aux_sym__blank_line_token1,
    STATE(78), 1,
      aux_sym_response_repeat1,
    STATE(87), 1,
      sym_header,
    STATE(10), 2,
      sym__blank_line,
      aux_sym___body_repeat1,
    ACTIONS(55), 4,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
    ACTIONS(53), 6,
      ts_builtin_sym_end,
      aux_sym_request_separator_token1,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [3723] = 5,
    ACTIONS(520), 1,
      sym_header_entity,
    STATE(82), 1,
      aux_sym_response_repeat1,
    STATE(87), 1,
      sym_header,
    ACTIONS(518), 5,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      aux_sym__blank_line_token1,
    ACTIONS(516), 7,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [3749] = 2,
    ACTIONS(525), 6,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      sym_header_entity,
      aux_sym__blank_line_token1,
    ACTIONS(523), 7,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [3767] = 2,
    ACTIONS(320), 6,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      sym_header_entity,
      aux_sym__blank_line_token1,
    ACTIONS(318), 7,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [3785] = 2,
    ACTIONS(529), 6,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      sym_header_entity,
      aux_sym__blank_line_token1,
    ACTIONS(527), 7,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [3803] = 2,
    ACTIONS(533), 6,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      sym_header_entity,
      aux_sym__blank_line_token1,
    ACTIONS(531), 7,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [3821] = 2,
    ACTIONS(537), 6,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      sym_header_entity,
      aux_sym__blank_line_token1,
    ACTIONS(535), 7,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [3839] = 2,
    ACTIONS(541), 6,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      sym_header_entity,
      aux_sym__blank_line_token1,
    ACTIONS(539), 7,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [3857] = 2,
    ACTIONS(545), 6,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      sym_header_entity,
      aux_sym__blank_line_token1,
    ACTIONS(543), 7,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [3875] = 2,
    ACTIONS(549), 6,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      sym_header_entity,
      aux_sym__blank_line_token1,
    ACTIONS(547), 7,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [3893] = 2,
    ACTIONS(553), 6,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      sym_header_entity,
      aux_sym__blank_line_token1,
    ACTIONS(551), 7,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [3911] = 2,
    ACTIONS(333), 6,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      sym_header_entity,
      aux_sym__blank_line_token1,
    ACTIONS(331), 7,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [3929] = 2,
    ACTIONS(354), 6,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      sym_header_entity,
      aux_sym__blank_line_token1,
    ACTIONS(352), 7,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [3947] = 2,
    ACTIONS(358), 6,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      sym_header_entity,
      aux_sym__blank_line_token1,
    ACTIONS(356), 7,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [3965] = 2,
    ACTIONS(329), 6,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      sym_header_entity,
      aux_sym__blank_line_token1,
    ACTIONS(327), 7,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [3983] = 2,
    ACTIONS(362), 6,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      sym_method,
      sym_header_entity,
      aux_sym__blank_line_token1,
    ACTIONS(360), 7,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [4001] = 2,
    ACTIONS(557), 4,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      aux_sym__blank_line_token1,
    ACTIONS(555), 8,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [4018] = 2,
    ACTIONS(561), 4,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      aux_sym__blank_line_token1,
    ACTIONS(559), 8,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [4035] = 2,
    ACTIONS(565), 4,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      aux_sym__blank_line_token1,
    ACTIONS(563), 8,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [4052] = 2,
    ACTIONS(569), 4,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      aux_sym__blank_line_token1,
    ACTIONS(567), 8,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [4069] = 2,
    ACTIONS(354), 4,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      aux_sym__blank_line_token1,
    ACTIONS(352), 8,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [4086] = 2,
    ACTIONS(362), 4,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      aux_sym__blank_line_token1,
    ACTIONS(360), 8,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [4103] = 2,
    ACTIONS(573), 4,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      aux_sym__blank_line_token1,
    ACTIONS(571), 8,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [4120] = 2,
    ACTIONS(577), 4,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      aux_sym__blank_line_token1,
    ACTIONS(575), 8,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [4137] = 2,
    ACTIONS(581), 4,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      aux_sym__blank_line_token1,
    ACTIONS(579), 8,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [4154] = 2,
    ACTIONS(402), 4,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      aux_sym__blank_line_token1,
    ACTIONS(400), 8,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [4171] = 2,
    ACTIONS(320), 4,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      aux_sym__blank_line_token1,
    ACTIONS(318), 8,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [4188] = 2,
    ACTIONS(585), 4,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      aux_sym__blank_line_token1,
    ACTIONS(583), 8,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [4205] = 2,
    ACTIONS(358), 4,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      aux_sym__blank_line_token1,
    ACTIONS(356), 8,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [4222] = 2,
    ACTIONS(589), 4,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      aux_sym__blank_line_token1,
    ACTIONS(587), 8,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [4239] = 2,
    ACTIONS(593), 4,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      aux_sym__blank_line_token1,
    ACTIONS(591), 8,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [4256] = 2,
    ACTIONS(329), 4,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      aux_sym__blank_line_token1,
    ACTIONS(327), 8,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [4273] = 2,
    ACTIONS(597), 4,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      aux_sym__blank_line_token1,
    ACTIONS(595), 8,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [4290] = 2,
    ACTIONS(601), 4,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      aux_sym__blank_line_token1,
    ACTIONS(599), 8,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [4307] = 2,
    ACTIONS(605), 4,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      aux_sym__blank_line_token1,
    ACTIONS(603), 8,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [4324] = 2,
    ACTIONS(333), 4,
      aux_sym_WORD_CHAR_token1,
      aux_sym_PUNCTUATION_token1,
      aux_sym_COMMENT_PREFIX_token1,
      aux_sym__blank_line_token1,
    ACTIONS(331), 8,
      ts_builtin_sym_end,
      aux_sym_WS_token1,
      aux_sym_request_separator_token1,
      sym_method,
      aux_sym_http_version_token1,
      anon_sym_LBRACE_LBRACE,
      anon_sym_LT,
      anon_sym_AT2,
  [4341] = 7,
    ACTIONS(17), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(607), 1,
      aux_sym_WORD_CHAR_token1,
    ACTIONS(611), 1,
      aux_sym_NL_token1,
    ACTIONS(613), 1,
      aux_sym__var_comment_token1,
    STATE(211), 1,
      sym_value,
    ACTIONS(609), 2,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
    STATE(145), 2,
      sym_variable,
      aux_sym_value_repeat1,
  [4365] = 7,
    ACTIONS(17), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(607), 1,
      aux_sym_WORD_CHAR_token1,
    ACTIONS(615), 1,
      aux_sym_NL_token1,
    ACTIONS(617), 1,
      aux_sym__var_comment_token1,
    STATE(231), 1,
      sym_value,
    ACTIONS(609), 2,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
    STATE(145), 2,
      sym_variable,
      aux_sym_value_repeat1,
  [4389] = 6,
    ACTIONS(621), 1,
      aux_sym_PUNCTUATION_token1,
    ACTIONS(623), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(625), 1,
      anon_sym_LBRACE_PERCENT,
    ACTIONS(619), 2,
      aux_sym_WORD_CHAR_token1,
      aux_sym_path_token1,
    STATE(150), 2,
      sym_variable,
      aux_sym_path_repeat1,
    STATE(210), 2,
      sym_script,
      sym_path,
  [4411] = 6,
    ACTIONS(621), 1,
      aux_sym_PUNCTUATION_token1,
    ACTIONS(623), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(625), 1,
      anon_sym_LBRACE_PERCENT,
    ACTIONS(619), 2,
      aux_sym_WORD_CHAR_token1,
      aux_sym_path_token1,
    STATE(150), 2,
      sym_variable,
      aux_sym_path_repeat1,
    STATE(221), 2,
      sym_script,
      sym_path,
  [4433] = 7,
    ACTIONS(17), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(607), 1,
      aux_sym_WORD_CHAR_token1,
    ACTIONS(627), 1,
      aux_sym_NL_token1,
    ACTIONS(629), 1,
      aux_sym__var_comment_token1,
    STATE(228), 1,
      sym_value,
    ACTIONS(609), 2,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
    STATE(145), 2,
      sym_variable,
      aux_sym_value_repeat1,
  [4457] = 6,
    ACTIONS(17), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(609), 1,
      aux_sym_PUNCTUATION_token1,
    ACTIONS(611), 1,
      aux_sym_NL_token1,
    STATE(211), 1,
      sym_value,
    ACTIONS(607), 2,
      aux_sym_WORD_CHAR_token1,
      aux_sym_WS_token1,
    STATE(145), 2,
      sym_variable,
      aux_sym_value_repeat1,
  [4478] = 6,
    ACTIONS(17), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(607), 1,
      aux_sym_WORD_CHAR_token1,
    ACTIONS(631), 1,
      aux_sym__var_comment_token1,
    STATE(238), 1,
      sym_value,
    ACTIONS(609), 2,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
    STATE(145), 2,
      sym_variable,
      aux_sym_value_repeat1,
  [4499] = 6,
    ACTIONS(17), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(607), 1,
      aux_sym_WORD_CHAR_token1,
    ACTIONS(633), 1,
      aux_sym__var_comment_token1,
    STATE(246), 1,
      sym_value,
    ACTIONS(609), 2,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
    STATE(145), 2,
      sym_variable,
      aux_sym_value_repeat1,
  [4520] = 6,
    ACTIONS(17), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(607), 1,
      aux_sym_WORD_CHAR_token1,
    ACTIONS(635), 1,
      aux_sym__var_comment_token1,
    STATE(234), 1,
      sym_value,
    ACTIONS(609), 2,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
    STATE(145), 2,
      sym_variable,
      aux_sym_value_repeat1,
  [4541] = 6,
    ACTIONS(17), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(609), 1,
      aux_sym_PUNCTUATION_token1,
    ACTIONS(637), 1,
      aux_sym_NL_token1,
    STATE(240), 1,
      sym_value,
    ACTIONS(607), 2,
      aux_sym_WORD_CHAR_token1,
      aux_sym_WS_token1,
    STATE(145), 2,
      sym_variable,
      aux_sym_value_repeat1,
  [4562] = 6,
    ACTIONS(17), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(609), 1,
      aux_sym_PUNCTUATION_token1,
    ACTIONS(639), 1,
      aux_sym_NL_token1,
    STATE(223), 1,
      sym_value,
    ACTIONS(607), 2,
      aux_sym_WORD_CHAR_token1,
      aux_sym_WS_token1,
    STATE(145), 2,
      sym_variable,
      aux_sym_value_repeat1,
  [4583] = 6,
    ACTIONS(17), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(607), 1,
      aux_sym_WORD_CHAR_token1,
    ACTIONS(641), 1,
      aux_sym__var_comment_token1,
    STATE(232), 1,
      sym_value,
    ACTIONS(609), 2,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
    STATE(145), 2,
      sym_variable,
      aux_sym_value_repeat1,
  [4604] = 6,
    ACTIONS(645), 1,
      aux_sym_PUNCTUATION_token1,
    ACTIONS(647), 1,
      aux_sym_WS_token1,
    ACTIONS(649), 1,
      aux_sym_NL_token1,
    ACTIONS(651), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(643), 2,
      aux_sym_WORD_CHAR_token1,
      aux_sym_path_token1,
    STATE(130), 2,
      sym_variable,
      aux_sym_path_repeat1,
  [4625] = 6,
    ACTIONS(656), 1,
      aux_sym_PUNCTUATION_token1,
    ACTIONS(659), 1,
      aux_sym_WS_token1,
    ACTIONS(661), 1,
      aux_sym_NL_token1,
    ACTIONS(663), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(653), 2,
      aux_sym_WORD_CHAR_token1,
      aux_sym_path_token1,
    STATE(130), 2,
      sym_variable,
      aux_sym_path_repeat1,
  [4646] = 6,
    ACTIONS(17), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(607), 1,
      aux_sym_WORD_CHAR_token1,
    ACTIONS(666), 1,
      aux_sym__var_comment_token1,
    STATE(226), 1,
      sym_value,
    ACTIONS(609), 2,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
    STATE(145), 2,
      sym_variable,
      aux_sym_value_repeat1,
  [4667] = 7,
    ACTIONS(17), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(668), 1,
      aux_sym_WORD_CHAR_token1,
    ACTIONS(670), 1,
      aux_sym_PUNCTUATION_token1,
    ACTIONS(672), 1,
      aux_sym_WS_token1,
    ACTIONS(675), 1,
      aux_sym_NL_token1,
    STATE(174), 1,
      aux_sym_target_url_repeat1,
    STATE(134), 2,
      aux_sym__target_url_line,
      sym_variable,
  [4690] = 6,
    ACTIONS(17), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(607), 1,
      aux_sym_WORD_CHAR_token1,
    ACTIONS(678), 1,
      aux_sym__var_comment_token1,
    STATE(243), 1,
      sym_value,
    ACTIONS(609), 2,
      aux_sym_PUNCTUATION_token1,
      aux_sym_WS_token1,
    STATE(145), 2,
      sym_variable,
      aux_sym_value_repeat1,
  [4711] = 5,
    ACTIONS(683), 1,
      aux_sym_PUNCTUATION_token1,
    ACTIONS(686), 1,
      aux_sym_NL_token1,
    ACTIONS(688), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(680), 2,
      aux_sym_WORD_CHAR_token1,
      aux_sym_WS_token1,
    STATE(134), 2,
      aux_sym__target_url_line,
      sym_variable,
  [4729] = 6,
    ACTIONS(17), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(668), 1,
      aux_sym_WORD_CHAR_token1,
    ACTIONS(670), 1,
      aux_sym_PUNCTUATION_token1,
    ACTIONS(691), 1,
      aux_sym_WS_token1,
    ACTIONS(694), 1,
      aux_sym_NL_token1,
    STATE(134), 2,
      aux_sym__target_url_line,
      sym_variable,
  [4749] = 5,
    ACTIONS(17), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(609), 1,
      aux_sym_PUNCTUATION_token1,
    STATE(247), 1,
      sym_value,
    ACTIONS(607), 2,
      aux_sym_WORD_CHAR_token1,
      aux_sym_WS_token1,
    STATE(145), 2,
      sym_variable,
      aux_sym_value_repeat1,
  [4767] = 5,
    ACTIONS(621), 1,
      aux_sym_PUNCTUATION_token1,
    ACTIONS(623), 1,
      anon_sym_LBRACE_LBRACE,
    STATE(209), 1,
      sym_path,
    ACTIONS(619), 2,
      aux_sym_WORD_CHAR_token1,
      aux_sym_path_token1,
    STATE(150), 2,
      sym_variable,
      aux_sym_path_repeat1,
  [4785] = 5,
    ACTIONS(17), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(609), 1,
      aux_sym_PUNCTUATION_token1,
    STATE(245), 1,
      sym_value,
    ACTIONS(607), 2,
      aux_sym_WORD_CHAR_token1,
      aux_sym_WS_token1,
    STATE(145), 2,
      sym_variable,
      aux_sym_value_repeat1,
  [4803] = 5,
    ACTIONS(651), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(698), 1,
      aux_sym_PUNCTUATION_token1,
    STATE(204), 1,
      sym_path,
    ACTIONS(696), 2,
      aux_sym_WORD_CHAR_token1,
      aux_sym_path_token1,
    STATE(129), 2,
      sym_variable,
      aux_sym_path_repeat1,
  [4821] = 5,
    ACTIONS(17), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(609), 1,
      aux_sym_PUNCTUATION_token1,
    STATE(244), 1,
      sym_value,
    ACTIONS(607), 2,
      aux_sym_WORD_CHAR_token1,
      aux_sym_WS_token1,
    STATE(145), 2,
      sym_variable,
      aux_sym_value_repeat1,
  [4839] = 5,
    ACTIONS(661), 1,
      aux_sym_pre_request_script_token1,
    ACTIONS(703), 1,
      aux_sym_PUNCTUATION_token1,
    ACTIONS(706), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(700), 2,
      aux_sym_WORD_CHAR_token1,
      aux_sym_path_token1,
    STATE(141), 2,
      sym_variable,
      aux_sym_path_repeat1,
  [4857] = 5,
    ACTIONS(17), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(609), 1,
      aux_sym_PUNCTUATION_token1,
    STATE(236), 1,
      sym_value,
    ACTIONS(607), 2,
      aux_sym_WORD_CHAR_token1,
      aux_sym_WS_token1,
    STATE(145), 2,
      sym_variable,
      aux_sym_value_repeat1,
  [4875] = 5,
    ACTIONS(17), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(609), 1,
      aux_sym_PUNCTUATION_token1,
    STATE(222), 1,
      sym_value,
    ACTIONS(607), 2,
      aux_sym_WORD_CHAR_token1,
      aux_sym_WS_token1,
    STATE(145), 2,
      sym_variable,
      aux_sym_value_repeat1,
  [4893] = 5,
    ACTIONS(17), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(609), 1,
      aux_sym_PUNCTUATION_token1,
    STATE(238), 1,
      sym_value,
    ACTIONS(607), 2,
      aux_sym_WORD_CHAR_token1,
      aux_sym_WS_token1,
    STATE(145), 2,
      sym_variable,
      aux_sym_value_repeat1,
  [4911] = 5,
    ACTIONS(17), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(711), 1,
      aux_sym_PUNCTUATION_token1,
    ACTIONS(713), 1,
      aux_sym_NL_token1,
    ACTIONS(709), 2,
      aux_sym_WORD_CHAR_token1,
      aux_sym_WS_token1,
    STATE(149), 2,
      sym_variable,
      aux_sym_value_repeat1,
  [4929] = 5,
    ACTIONS(5), 1,
      aux_sym_PUNCTUATION_token1,
    ACTIONS(17), 1,
      anon_sym_LBRACE_LBRACE,
    STATE(199), 1,
      sym_target_url,
    ACTIONS(715), 2,
      aux_sym_WORD_CHAR_token1,
      aux_sym_WS_token1,
    STATE(132), 2,
      aux_sym__target_url_line,
      sym_variable,
  [4947] = 5,
    ACTIONS(651), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(698), 1,
      aux_sym_PUNCTUATION_token1,
    STATE(196), 1,
      sym_path,
    ACTIONS(696), 2,
      aux_sym_WORD_CHAR_token1,
      aux_sym_path_token1,
    STATE(129), 2,
      sym_variable,
      aux_sym_path_repeat1,
  [4965] = 5,
    ACTIONS(651), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(698), 1,
      aux_sym_PUNCTUATION_token1,
    STATE(191), 1,
      sym_path,
    ACTIONS(696), 2,
      aux_sym_WORD_CHAR_token1,
      aux_sym_path_token1,
    STATE(129), 2,
      sym_variable,
      aux_sym_path_repeat1,
  [4983] = 5,
    ACTIONS(720), 1,
      aux_sym_PUNCTUATION_token1,
    ACTIONS(723), 1,
      aux_sym_NL_token1,
    ACTIONS(725), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(717), 2,
      aux_sym_WORD_CHAR_token1,
      aux_sym_WS_token1,
    STATE(149), 2,
      sym_variable,
      aux_sym_value_repeat1,
  [5001] = 5,
    ACTIONS(623), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(649), 1,
      aux_sym_pre_request_script_token1,
    ACTIONS(730), 1,
      aux_sym_PUNCTUATION_token1,
    ACTIONS(728), 2,
      aux_sym_WORD_CHAR_token1,
      aux_sym_path_token1,
    STATE(141), 2,
      sym_variable,
      aux_sym_path_repeat1,
  [5019] = 2,
    ACTIONS(734), 2,
      aux_sym_PUNCTUATION_token1,
      aux_sym_NL_token1,
    ACTIONS(732), 4,
      aux_sym_WORD_CHAR_token1,
      aux_sym_WS_token1,
      anon_sym_LBRACE_LBRACE,
      aux_sym_path_token1,
  [5030] = 2,
    ACTIONS(738), 2,
      aux_sym_PUNCTUATION_token1,
      aux_sym_NL_token1,
    ACTIONS(736), 4,
      aux_sym_WORD_CHAR_token1,
      aux_sym_WS_token1,
      anon_sym_LBRACE_LBRACE,
      aux_sym_path_token1,
  [5041] = 2,
    ACTIONS(742), 2,
      aux_sym_PUNCTUATION_token1,
      aux_sym_NL_token1,
    ACTIONS(740), 4,
      aux_sym_WORD_CHAR_token1,
      aux_sym_WS_token1,
      anon_sym_LBRACE_LBRACE,
      aux_sym_path_token1,
  [5052] = 2,
    ACTIONS(746), 2,
      aux_sym_PUNCTUATION_token1,
      aux_sym_NL_token1,
    ACTIONS(744), 4,
      aux_sym_WORD_CHAR_token1,
      aux_sym_WS_token1,
      anon_sym_LBRACE_LBRACE,
      aux_sym_path_token1,
  [5063] = 3,
    ACTIONS(752), 1,
      aux_sym__blank_line_token1,
    ACTIONS(750), 2,
      aux_sym_PUNCTUATION_token1,
      aux_sym_NL_token1,
    ACTIONS(748), 3,
      aux_sym_WORD_CHAR_token1,
      aux_sym_WS_token1,
      anon_sym_LBRACE_LBRACE,
  [5076] = 4,
    ACTIONS(17), 1,
      anon_sym_LBRACE_LBRACE,
    ACTIONS(756), 1,
      aux_sym_PUNCTUATION_token1,
    ACTIONS(754), 2,
      aux_sym_WORD_CHAR_token1,
      aux_sym_WS_token1,
    STATE(135), 2,
      aux_sym__target_url_line,
      sym_variable,
  [5091] = 2,
    ACTIONS(738), 2,
      aux_sym_PUNCTUATION_token1,
      aux_sym_pre_request_script_token1,
    ACTIONS(736), 3,
      aux_sym_WORD_CHAR_token1,
      anon_sym_LBRACE_LBRACE,
      aux_sym_path_token1,
  [5101] = 2,
    ACTIONS(742), 2,
      aux_sym_PUNCTUATION_token1,
      aux_sym_pre_request_script_token1,
    ACTIONS(740), 3,
      aux_sym_WORD_CHAR_token1,
      anon_sym_LBRACE_LBRACE,
      aux_sym_path_token1,
  [5111] = 2,
    ACTIONS(746), 2,
      aux_sym_PUNCTUATION_token1,
      aux_sym_pre_request_script_token1,
    ACTIONS(744), 3,
      aux_sym_WORD_CHAR_token1,
      anon_sym_LBRACE_LBRACE,
      aux_sym_path_token1,
  [5121] = 2,
    ACTIONS(734), 2,
      aux_sym_PUNCTUATION_token1,
      aux_sym_pre_request_script_token1,
    ACTIONS(732), 3,
      aux_sym_WORD_CHAR_token1,
      anon_sym_LBRACE_LBRACE,
      aux_sym_path_token1,
  [5131] = 2,
    ACTIONS(738), 2,
      aux_sym_PUNCTUATION_token1,
      aux_sym_NL_token1,
    ACTIONS(736), 3,
      aux_sym_WORD_CHAR_token1,
      aux_sym_WS_token1,
      anon_sym_LBRACE_LBRACE,
  [5141] = 2,
    ACTIONS(742), 2,
      aux_sym_PUNCTUATION_token1,
      aux_sym_NL_token1,
    ACTIONS(740), 3,
      aux_sym_WORD_CHAR_token1,
      aux_sym_WS_token1,
      anon_sym_LBRACE_LBRACE,
  [5151] = 2,
    ACTIONS(746), 2,
      aux_sym_PUNCTUATION_token1,
      aux_sym_NL_token1,
    ACTIONS(744), 3,
      aux_sym_WORD_CHAR_token1,
      aux_sym_WS_token1,
      anon_sym_LBRACE_LBRACE,
  [5161] = 2,
    ACTIONS(734), 2,
      aux_sym_PUNCTUATION_token1,
      aux_sym_NL_token1,
    ACTIONS(732), 3,
      aux_sym_WORD_CHAR_token1,
      aux_sym_WS_token1,
      anon_sym_LBRACE_LBRACE,
  [5171] = 3,
    ACTIONS(758), 1,
      aux_sym_LINE_TAIL_token1,
    ACTIONS(760), 1,
      anon_sym_PERCENT_RBRACE,
    STATE(177), 1,
      aux_sym_script_repeat1,
  [5181] = 3,
    ACTIONS(762), 1,
      aux_sym_LINE_TAIL_token1,
    ACTIONS(765), 1,
      anon_sym_PERCENT_RBRACE,
    STATE(166), 1,
      aux_sym_script_repeat1,
  [5191] = 3,
    ACTIONS(325), 1,
      aux_sym__raw_body_token1,
    ACTIONS(767), 1,
      aux_sym_COMMENT_PREFIX_token1,
    STATE(66), 1,
      sym__raw_body,
  [5201] = 3,
    ACTIONS(325), 1,
      aux_sym__raw_body_token1,
    ACTIONS(767), 1,
      aux_sym_COMMENT_PREFIX_token1,
    STATE(63), 1,
      sym__raw_body,
  [5211] = 3,
    ACTIONS(309), 1,
      aux_sym__raw_body_token1,
    ACTIONS(769), 1,
      aux_sym_COMMENT_PREFIX_token1,
    STATE(49), 1,
      sym__raw_body,
  [5221] = 2,
    ACTIONS(773), 1,
      aux_sym_NL_token1,
    ACTIONS(771), 2,
      aux_sym_WS_token1,
      anon_sym_EQ,
  [5229] = 2,
    ACTIONS(777), 1,
      aux_sym_NL_token1,
    ACTIONS(775), 2,
      aux_sym_WS_token1,
      anon_sym_EQ,
  [5237] = 2,
    ACTIONS(781), 1,
      aux_sym_NL_token1,
    ACTIONS(779), 2,
      aux_sym_WS_token1,
      anon_sym_EQ,
  [5245] = 2,
    ACTIONS(785), 1,
      aux_sym_NL_token1,
    ACTIONS(783), 2,
      aux_sym_WS_token1,
      anon_sym_EQ,
  [5253] = 3,
    ACTIONS(787), 1,
      aux_sym_WS_token1,
    ACTIONS(789), 1,
      aux_sym_NL_token1,
    STATE(175), 1,
      aux_sym_target_url_repeat1,
  [5263] = 3,
    ACTIONS(792), 1,
      aux_sym_WS_token1,
    ACTIONS(794), 1,
      aux_sym_NL_token1,
    STATE(175), 1,
      aux_sym_target_url_repeat1,
  [5273] = 3,
    ACTIONS(325), 1,
      aux_sym__raw_body_token1,
    ACTIONS(767), 1,
      aux_sym_COMMENT_PREFIX_token1,
    STATE(54), 1,
      sym__raw_body,
  [5283] = 3,
    ACTIONS(797), 1,
      aux_sym_LINE_TAIL_token1,
    ACTIONS(799), 1,
      anon_sym_PERCENT_RBRACE,
    STATE(166), 1,
      aux_sym_script_repeat1,
  [5293] = 2,
    ACTIONS(801), 1,
      aux_sym_LINE_TAIL_token1,
    ACTIONS(803), 1,
      anon_sym_AT,
  [5300] = 2,
    ACTIONS(805), 1,
      anon_sym_AT,
    ACTIONS(807), 1,
      sym__not_comment,
  [5307] = 2,
    ACTIONS(809), 1,
      aux_sym_WS_token1,
    ACTIONS(811), 1,
      sym_identifier,
  [5314] = 2,
    ACTIONS(15), 1,
      aux_sym_http_version_token1,
    STATE(252), 1,
      sym_http_version,
  [5321] = 2,
    ACTIONS(813), 1,
      aux_sym_WS_token1,
    ACTIONS(815), 1,
      anon_sym_COLON,
  [5328] = 2,
    ACTIONS(817), 1,
      aux_sym_WS_token1,
    ACTIONS(819), 1,
      sym_identifier,
  [5335] = 2,
    ACTIONS(821), 1,
      aux_sym_WS_token1,
    ACTIONS(823), 1,
      sym_identifier,
  [5342] = 2,
    ACTIONS(825), 1,
      aux_sym_WS_token1,
    ACTIONS(827), 1,
      anon_sym_EQ,
  [5349] = 2,
    ACTIONS(829), 1,
      aux_sym_WS_token1,
    ACTIONS(831), 1,
      anon_sym_RBRACE_RBRACE,
  [5356] = 2,
    ACTIONS(833), 1,
      aux_sym_WS_token1,
    ACTIONS(835), 1,
      anon_sym_RBRACE_RBRACE,
  [5363] = 2,
    ACTIONS(837), 1,
      aux_sym_LINE_TAIL_token1,
    ACTIONS(839), 1,
      anon_sym_AT,
  [5370] = 2,
    ACTIONS(841), 1,
      aux_sym_WS_token1,
    ACTIONS(843), 1,
      anon_sym_RBRACE_RBRACE,
  [5377] = 2,
    ACTIONS(845), 1,
      aux_sym_WS_token1,
    ACTIONS(847), 1,
      anon_sym_RBRACE_RBRACE,
  [5384] = 1,
    ACTIONS(849), 2,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
  [5389] = 2,
    ACTIONS(851), 1,
      aux_sym_WS_token1,
    ACTIONS(853), 1,
      anon_sym_RBRACE_RBRACE,
  [5396] = 2,
    ACTIONS(855), 1,
      aux_sym_LINE_TAIL_token1,
    ACTIONS(857), 1,
      anon_sym_AT,
  [5403] = 1,
    ACTIONS(859), 2,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
  [5408] = 2,
    ACTIONS(861), 1,
      aux_sym_WS_token1,
    ACTIONS(863), 1,
      sym_identifier,
  [5415] = 1,
    ACTIONS(865), 2,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
  [5420] = 2,
    ACTIONS(867), 1,
      aux_sym_NL_token1,
    ACTIONS(869), 1,
      sym_status_text,
  [5427] = 1,
    ACTIONS(871), 2,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
  [5432] = 2,
    ACTIONS(873), 1,
      aux_sym_WS_token1,
    ACTIONS(875), 1,
      aux_sym_NL_token1,
  [5439] = 2,
    ACTIONS(877), 1,
      aux_sym_WS_token1,
    ACTIONS(879), 1,
      aux_sym_NL_token1,
  [5446] = 2,
    ACTIONS(881), 1,
      aux_sym_WS_token1,
    ACTIONS(883), 1,
      anon_sym_AT2,
  [5453] = 2,
    ACTIONS(15), 1,
      aux_sym_http_version_token1,
    STATE(224), 1,
      sym_http_version,
  [5460] = 2,
    ACTIONS(885), 1,
      aux_sym_WS_token1,
    ACTIONS(887), 1,
      anon_sym_RBRACE_RBRACE,
  [5467] = 1,
    ACTIONS(889), 2,
      aux_sym_WS_token1,
      aux_sym_NL_token1,
  [5472] = 1,
    ACTIONS(891), 1,
      aux_sym_NL_token1,
  [5476] = 1,
    ACTIONS(893), 1,
      anon_sym_RBRACE_RBRACE,
  [5480] = 1,
    ACTIONS(895), 1,
      aux_sym_multipart_form_data_token1,
  [5484] = 1,
    ACTIONS(897), 1,
      aux_sym_WS_token1,
  [5488] = 1,
    ACTIONS(899), 1,
      aux_sym_pre_request_script_token1,
  [5492] = 1,
    ACTIONS(901), 1,
      aux_sym_pre_request_script_token1,
  [5496] = 1,
    ACTIONS(903), 1,
      aux_sym_NL_token1,
  [5500] = 1,
    ACTIONS(905), 1,
      aux_sym_WS_token1,
  [5504] = 1,
    ACTIONS(907), 1,
      aux_sym_WS_token1,
  [5508] = 1,
    ACTIONS(909), 1,
      aux_sym_WS_token1,
  [5512] = 1,
    ACTIONS(911), 1,
      anon_sym_COLON,
  [5516] = 1,
    ACTIONS(913), 1,
      ts_builtin_sym_end,
  [5520] = 1,
    ACTIONS(915), 1,
      sym_identifier,
  [5524] = 1,
    ACTIONS(917), 1,
      anon_sym_RBRACE_RBRACE,
  [5528] = 1,
    ACTIONS(919), 1,
      aux_sym_WS_token1,
  [5532] = 1,
    ACTIONS(921), 1,
      aux_sym_NL_token1,
  [5536] = 1,
    ACTIONS(923), 1,
      aux_sym_pre_request_script_token1,
  [5540] = 1,
    ACTIONS(925), 1,
      aux_sym_NL_token1,
  [5544] = 1,
    ACTIONS(927), 1,
      aux_sym_NL_token1,
  [5548] = 1,
    ACTIONS(929), 1,
      aux_sym_NL_token1,
  [5552] = 1,
    ACTIONS(931), 1,
      aux_sym_WS_token1,
  [5556] = 1,
    ACTIONS(933), 1,
      aux_sym_NL_token1,
  [5560] = 1,
    ACTIONS(935), 1,
      sym_identifier,
  [5564] = 1,
    ACTIONS(937), 1,
      aux_sym_NL_token1,
  [5568] = 1,
    ACTIONS(939), 1,
      aux_sym_pre_request_script_token1,
  [5572] = 1,
    ACTIONS(941), 1,
      aux_sym_pre_request_script_token1,
  [5576] = 1,
    ACTIONS(943), 1,
      aux_sym_NL_token1,
  [5580] = 1,
    ACTIONS(945), 1,
      aux_sym_NL_token1,
  [5584] = 1,
    ACTIONS(947), 1,
      anon_sym_RBRACE_RBRACE,
  [5588] = 1,
    ACTIONS(949), 1,
      aux_sym_NL_token1,
  [5592] = 1,
    ACTIONS(951), 1,
      sym__not_comment,
  [5596] = 1,
    ACTIONS(953), 1,
      aux_sym_NL_token1,
  [5600] = 1,
    ACTIONS(955), 1,
      sym_identifier,
  [5604] = 1,
    ACTIONS(957), 1,
      aux_sym_NL_token1,
  [5608] = 1,
    ACTIONS(959), 1,
      anon_sym_EQ,
  [5612] = 1,
    ACTIONS(961), 1,
      aux_sym_NL_token1,
  [5616] = 1,
    ACTIONS(963), 1,
      anon_sym_RBRACE_RBRACE,
  [5620] = 1,
    ACTIONS(965), 1,
      anon_sym_RBRACE_RBRACE,
  [5624] = 1,
    ACTIONS(967), 1,
      aux_sym_NL_token1,
  [5628] = 1,
    ACTIONS(969), 1,
      aux_sym_NL_token1,
  [5632] = 1,
    ACTIONS(971), 1,
      aux_sym_NL_token1,
  [5636] = 1,
    ACTIONS(973), 1,
      aux_sym_NL_token1,
  [5640] = 1,
    ACTIONS(975), 1,
      aux_sym_NL_token1,
  [5644] = 1,
    ACTIONS(977), 1,
      aux_sym_NL_token1,
  [5648] = 1,
    ACTIONS(979), 1,
      sym_identifier,
  [5652] = 1,
    ACTIONS(981), 1,
      sym_identifier,
  [5656] = 1,
    ACTIONS(983), 1,
      aux_sym_WS_token1,
  [5660] = 1,
    ACTIONS(985), 1,
      aux_sym_NL_token1,
  [5664] = 1,
    ACTIONS(987), 1,
      anon_sym_RBRACE_RBRACE,
  [5668] = 1,
    ACTIONS(989), 1,
      sym_identifier,
  [5672] = 1,
    ACTIONS(991), 1,
      sym_identifier,
  [5676] = 1,
    ACTIONS(993), 1,
      aux_sym__blank_line_token1,
  [5680] = 1,
    ACTIONS(995), 1,
      sym_status_code,
  [5684] = 1,
    ACTIONS(997), 1,
      sym_identifier,
  [5688] = 1,
    ACTIONS(999), 1,
      aux_sym_WS_token1,
  [5692] = 1,
    ACTIONS(1001), 1,
      sym__not_comment,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 76,
  [SMALL_STATE(4)] = 152,
  [SMALL_STATE(5)] = 228,
  [SMALL_STATE(6)] = 304,
  [SMALL_STATE(7)] = 380,
  [SMALL_STATE(8)] = 456,
  [SMALL_STATE(9)] = 532,
  [SMALL_STATE(10)] = 608,
  [SMALL_STATE(11)] = 684,
  [SMALL_STATE(12)] = 760,
  [SMALL_STATE(13)] = 836,
  [SMALL_STATE(14)] = 912,
  [SMALL_STATE(15)] = 980,
  [SMALL_STATE(16)] = 1048,
  [SMALL_STATE(17)] = 1116,
  [SMALL_STATE(18)] = 1184,
  [SMALL_STATE(19)] = 1252,
  [SMALL_STATE(20)] = 1320,
  [SMALL_STATE(21)] = 1388,
  [SMALL_STATE(22)] = 1456,
  [SMALL_STATE(23)] = 1524,
  [SMALL_STATE(24)] = 1592,
  [SMALL_STATE(25)] = 1660,
  [SMALL_STATE(26)] = 1728,
  [SMALL_STATE(27)] = 1796,
  [SMALL_STATE(28)] = 1869,
  [SMALL_STATE(29)] = 1918,
  [SMALL_STATE(30)] = 1991,
  [SMALL_STATE(31)] = 2040,
  [SMALL_STATE(32)] = 2089,
  [SMALL_STATE(33)] = 2153,
  [SMALL_STATE(34)] = 2217,
  [SMALL_STATE(35)] = 2252,
  [SMALL_STATE(36)] = 2287,
  [SMALL_STATE(37)] = 2315,
  [SMALL_STATE(38)] = 2349,
  [SMALL_STATE(39)] = 2377,
  [SMALL_STATE(40)] = 2405,
  [SMALL_STATE(41)] = 2433,
  [SMALL_STATE(42)] = 2467,
  [SMALL_STATE(43)] = 2501,
  [SMALL_STATE(44)] = 2535,
  [SMALL_STATE(45)] = 2563,
  [SMALL_STATE(46)] = 2591,
  [SMALL_STATE(47)] = 2619,
  [SMALL_STATE(48)] = 2651,
  [SMALL_STATE(49)] = 2685,
  [SMALL_STATE(50)] = 2712,
  [SMALL_STATE(51)] = 2739,
  [SMALL_STATE(52)] = 2766,
  [SMALL_STATE(53)] = 2792,
  [SMALL_STATE(54)] = 2818,
  [SMALL_STATE(55)] = 2844,
  [SMALL_STATE(56)] = 2870,
  [SMALL_STATE(57)] = 2896,
  [SMALL_STATE(58)] = 2922,
  [SMALL_STATE(59)] = 2948,
  [SMALL_STATE(60)] = 2974,
  [SMALL_STATE(61)] = 3000,
  [SMALL_STATE(62)] = 3026,
  [SMALL_STATE(63)] = 3052,
  [SMALL_STATE(64)] = 3078,
  [SMALL_STATE(65)] = 3104,
  [SMALL_STATE(66)] = 3130,
  [SMALL_STATE(67)] = 3156,
  [SMALL_STATE(68)] = 3182,
  [SMALL_STATE(69)] = 3208,
  [SMALL_STATE(70)] = 3251,
  [SMALL_STATE(71)] = 3294,
  [SMALL_STATE(72)] = 3337,
  [SMALL_STATE(73)] = 3380,
  [SMALL_STATE(74)] = 3423,
  [SMALL_STATE(75)] = 3466,
  [SMALL_STATE(76)] = 3509,
  [SMALL_STATE(77)] = 3552,
  [SMALL_STATE(78)] = 3587,
  [SMALL_STATE(79)] = 3621,
  [SMALL_STATE(80)] = 3655,
  [SMALL_STATE(81)] = 3689,
  [SMALL_STATE(82)] = 3723,
  [SMALL_STATE(83)] = 3749,
  [SMALL_STATE(84)] = 3767,
  [SMALL_STATE(85)] = 3785,
  [SMALL_STATE(86)] = 3803,
  [SMALL_STATE(87)] = 3821,
  [SMALL_STATE(88)] = 3839,
  [SMALL_STATE(89)] = 3857,
  [SMALL_STATE(90)] = 3875,
  [SMALL_STATE(91)] = 3893,
  [SMALL_STATE(92)] = 3911,
  [SMALL_STATE(93)] = 3929,
  [SMALL_STATE(94)] = 3947,
  [SMALL_STATE(95)] = 3965,
  [SMALL_STATE(96)] = 3983,
  [SMALL_STATE(97)] = 4001,
  [SMALL_STATE(98)] = 4018,
  [SMALL_STATE(99)] = 4035,
  [SMALL_STATE(100)] = 4052,
  [SMALL_STATE(101)] = 4069,
  [SMALL_STATE(102)] = 4086,
  [SMALL_STATE(103)] = 4103,
  [SMALL_STATE(104)] = 4120,
  [SMALL_STATE(105)] = 4137,
  [SMALL_STATE(106)] = 4154,
  [SMALL_STATE(107)] = 4171,
  [SMALL_STATE(108)] = 4188,
  [SMALL_STATE(109)] = 4205,
  [SMALL_STATE(110)] = 4222,
  [SMALL_STATE(111)] = 4239,
  [SMALL_STATE(112)] = 4256,
  [SMALL_STATE(113)] = 4273,
  [SMALL_STATE(114)] = 4290,
  [SMALL_STATE(115)] = 4307,
  [SMALL_STATE(116)] = 4324,
  [SMALL_STATE(117)] = 4341,
  [SMALL_STATE(118)] = 4365,
  [SMALL_STATE(119)] = 4389,
  [SMALL_STATE(120)] = 4411,
  [SMALL_STATE(121)] = 4433,
  [SMALL_STATE(122)] = 4457,
  [SMALL_STATE(123)] = 4478,
  [SMALL_STATE(124)] = 4499,
  [SMALL_STATE(125)] = 4520,
  [SMALL_STATE(126)] = 4541,
  [SMALL_STATE(127)] = 4562,
  [SMALL_STATE(128)] = 4583,
  [SMALL_STATE(129)] = 4604,
  [SMALL_STATE(130)] = 4625,
  [SMALL_STATE(131)] = 4646,
  [SMALL_STATE(132)] = 4667,
  [SMALL_STATE(133)] = 4690,
  [SMALL_STATE(134)] = 4711,
  [SMALL_STATE(135)] = 4729,
  [SMALL_STATE(136)] = 4749,
  [SMALL_STATE(137)] = 4767,
  [SMALL_STATE(138)] = 4785,
  [SMALL_STATE(139)] = 4803,
  [SMALL_STATE(140)] = 4821,
  [SMALL_STATE(141)] = 4839,
  [SMALL_STATE(142)] = 4857,
  [SMALL_STATE(143)] = 4875,
  [SMALL_STATE(144)] = 4893,
  [SMALL_STATE(145)] = 4911,
  [SMALL_STATE(146)] = 4929,
  [SMALL_STATE(147)] = 4947,
  [SMALL_STATE(148)] = 4965,
  [SMALL_STATE(149)] = 4983,
  [SMALL_STATE(150)] = 5001,
  [SMALL_STATE(151)] = 5019,
  [SMALL_STATE(152)] = 5030,
  [SMALL_STATE(153)] = 5041,
  [SMALL_STATE(154)] = 5052,
  [SMALL_STATE(155)] = 5063,
  [SMALL_STATE(156)] = 5076,
  [SMALL_STATE(157)] = 5091,
  [SMALL_STATE(158)] = 5101,
  [SMALL_STATE(159)] = 5111,
  [SMALL_STATE(160)] = 5121,
  [SMALL_STATE(161)] = 5131,
  [SMALL_STATE(162)] = 5141,
  [SMALL_STATE(163)] = 5151,
  [SMALL_STATE(164)] = 5161,
  [SMALL_STATE(165)] = 5171,
  [SMALL_STATE(166)] = 5181,
  [SMALL_STATE(167)] = 5191,
  [SMALL_STATE(168)] = 5201,
  [SMALL_STATE(169)] = 5211,
  [SMALL_STATE(170)] = 5221,
  [SMALL_STATE(171)] = 5229,
  [SMALL_STATE(172)] = 5237,
  [SMALL_STATE(173)] = 5245,
  [SMALL_STATE(174)] = 5253,
  [SMALL_STATE(175)] = 5263,
  [SMALL_STATE(176)] = 5273,
  [SMALL_STATE(177)] = 5283,
  [SMALL_STATE(178)] = 5293,
  [SMALL_STATE(179)] = 5300,
  [SMALL_STATE(180)] = 5307,
  [SMALL_STATE(181)] = 5314,
  [SMALL_STATE(182)] = 5321,
  [SMALL_STATE(183)] = 5328,
  [SMALL_STATE(184)] = 5335,
  [SMALL_STATE(185)] = 5342,
  [SMALL_STATE(186)] = 5349,
  [SMALL_STATE(187)] = 5356,
  [SMALL_STATE(188)] = 5363,
  [SMALL_STATE(189)] = 5370,
  [SMALL_STATE(190)] = 5377,
  [SMALL_STATE(191)] = 5384,
  [SMALL_STATE(192)] = 5389,
  [SMALL_STATE(193)] = 5396,
  [SMALL_STATE(194)] = 5403,
  [SMALL_STATE(195)] = 5408,
  [SMALL_STATE(196)] = 5415,
  [SMALL_STATE(197)] = 5420,
  [SMALL_STATE(198)] = 5427,
  [SMALL_STATE(199)] = 5432,
  [SMALL_STATE(200)] = 5439,
  [SMALL_STATE(201)] = 5446,
  [SMALL_STATE(202)] = 5453,
  [SMALL_STATE(203)] = 5460,
  [SMALL_STATE(204)] = 5467,
  [SMALL_STATE(205)] = 5472,
  [SMALL_STATE(206)] = 5476,
  [SMALL_STATE(207)] = 5480,
  [SMALL_STATE(208)] = 5484,
  [SMALL_STATE(209)] = 5488,
  [SMALL_STATE(210)] = 5492,
  [SMALL_STATE(211)] = 5496,
  [SMALL_STATE(212)] = 5500,
  [SMALL_STATE(213)] = 5504,
  [SMALL_STATE(214)] = 5508,
  [SMALL_STATE(215)] = 5512,
  [SMALL_STATE(216)] = 5516,
  [SMALL_STATE(217)] = 5520,
  [SMALL_STATE(218)] = 5524,
  [SMALL_STATE(219)] = 5528,
  [SMALL_STATE(220)] = 5532,
  [SMALL_STATE(221)] = 5536,
  [SMALL_STATE(222)] = 5540,
  [SMALL_STATE(223)] = 5544,
  [SMALL_STATE(224)] = 5548,
  [SMALL_STATE(225)] = 5552,
  [SMALL_STATE(226)] = 5556,
  [SMALL_STATE(227)] = 5560,
  [SMALL_STATE(228)] = 5564,
  [SMALL_STATE(229)] = 5568,
  [SMALL_STATE(230)] = 5572,
  [SMALL_STATE(231)] = 5576,
  [SMALL_STATE(232)] = 5580,
  [SMALL_STATE(233)] = 5584,
  [SMALL_STATE(234)] = 5588,
  [SMALL_STATE(235)] = 5592,
  [SMALL_STATE(236)] = 5596,
  [SMALL_STATE(237)] = 5600,
  [SMALL_STATE(238)] = 5604,
  [SMALL_STATE(239)] = 5608,
  [SMALL_STATE(240)] = 5612,
  [SMALL_STATE(241)] = 5616,
  [SMALL_STATE(242)] = 5620,
  [SMALL_STATE(243)] = 5624,
  [SMALL_STATE(244)] = 5628,
  [SMALL_STATE(245)] = 5632,
  [SMALL_STATE(246)] = 5636,
  [SMALL_STATE(247)] = 5640,
  [SMALL_STATE(248)] = 5644,
  [SMALL_STATE(249)] = 5648,
  [SMALL_STATE(250)] = 5652,
  [SMALL_STATE(251)] = 5656,
  [SMALL_STATE(252)] = 5660,
  [SMALL_STATE(253)] = 5664,
  [SMALL_STATE(254)] = 5668,
  [SMALL_STATE(255)] = 5672,
  [SMALL_STATE(256)] = 5676,
  [SMALL_STATE(257)] = 5680,
  [SMALL_STATE(258)] = 5684,
  [SMALL_STATE(259)] = 5688,
  [SMALL_STATE(260)] = 5692,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_document, 0, 0, 0),
  [5] = {.entry = {.count = 1, .reusable = false}}, SHIFT(132),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(155),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(193),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(118),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(225),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(194),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(195),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(219),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(217),
  [23] = {.entry = {.count = 1, .reusable = false}}, SHIFT(32),
  [25] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_request, 6, 0, 19),
  [27] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_request, 6, 0, 19),
  [29] = {.entry = {.count = 1, .reusable = false}}, SHIFT(256),
  [31] = {.entry = {.count = 1, .reusable = false}}, SHIFT(25),
  [33] = {.entry = {.count = 1, .reusable = false}}, SHIFT(179),
  [35] = {.entry = {.count = 1, .reusable = false}}, SHIFT(213),
  [37] = {.entry = {.count = 1, .reusable = true}}, SHIFT(212),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(167),
  [41] = {.entry = {.count = 1, .reusable = true}}, SHIFT(168),
  [43] = {.entry = {.count = 1, .reusable = true}}, SHIFT(169),
  [45] = {.entry = {.count = 1, .reusable = false}}, SHIFT(201),
  [47] = {.entry = {.count = 1, .reusable = true}}, SHIFT(207),
  [49] = {.entry = {.count = 1, .reusable = false}}, SHIFT(43),
  [51] = {.entry = {.count = 1, .reusable = false}}, SHIFT(48),
  [53] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_response, 6, 0, 0),
  [55] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_response, 6, 0, 0),
  [57] = {.entry = {.count = 1, .reusable = false}}, SHIFT(26),
  [59] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_request, 6, 0, 21),
  [61] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_request, 6, 0, 21),
  [63] = {.entry = {.count = 1, .reusable = false}}, SHIFT(15),
  [65] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_request, 7, 0, 26),
  [67] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_request, 7, 0, 26),
  [69] = {.entry = {.count = 1, .reusable = false}}, SHIFT(14),
  [71] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_request, 5, 0, 13),
  [73] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_request, 5, 0, 13),
  [75] = {.entry = {.count = 1, .reusable = false}}, SHIFT(18),
  [77] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_request, 4, 0, 10),
  [79] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_request, 4, 0, 10),
  [81] = {.entry = {.count = 1, .reusable = false}}, SHIFT(17),
  [83] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_request, 3, 0, 6),
  [85] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_request, 3, 0, 6),
  [87] = {.entry = {.count = 1, .reusable = false}}, SHIFT(19),
  [89] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_response, 7, 0, 29),
  [91] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_response, 7, 0, 29),
  [93] = {.entry = {.count = 1, .reusable = false}}, SHIFT(24),
  [95] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_response, 7, 0, 0),
  [97] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_response, 7, 0, 0),
  [99] = {.entry = {.count = 1, .reusable = false}}, SHIFT(23),
  [101] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_request, 5, 0, 14),
  [103] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_request, 5, 0, 14),
  [105] = {.entry = {.count = 1, .reusable = false}}, SHIFT(22),
  [107] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_request, 8, 0, 34),
  [109] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_request, 8, 0, 34),
  [111] = {.entry = {.count = 1, .reusable = false}}, SHIFT(21),
  [113] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_response, 8, 0, 38),
  [115] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_response, 8, 0, 38),
  [117] = {.entry = {.count = 1, .reusable = false}}, SHIFT(20),
  [119] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_request, 8, 0, 42),
  [121] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_request, 8, 0, 42),
  [123] = {.entry = {.count = 1, .reusable = false}}, SHIFT(16),
  [125] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_request, 7, 0, 39),
  [127] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_request, 7, 0, 39),
  [129] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym___body_repeat2, 2, 0, 23),
  [131] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym___body_repeat2, 2, 0, 23),
  [133] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym___body_repeat2, 2, 0, 23), SHIFT_REPEAT(16),
  [136] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym___body_repeat2, 2, 0, 23), SHIFT_REPEAT(179),
  [139] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym___body_repeat2, 2, 0, 23), SHIFT_REPEAT(213),
  [142] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym___body_repeat2, 2, 0, 23), SHIFT_REPEAT(212),
  [145] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym___body_repeat2, 2, 0, 23), SHIFT_REPEAT(167),
  [148] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym___body_repeat2, 2, 0, 23), SHIFT_REPEAT(168),
  [151] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym___body_repeat2, 2, 0, 23), SHIFT_REPEAT(169),
  [154] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym___body_repeat2, 2, 0, 23), SHIFT_REPEAT(201),
  [157] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym___body_repeat2, 2, 0, 23), SHIFT_REPEAT(207),
  [160] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym___body_repeat2, 2, 0, 23), SHIFT_REPEAT(43),
  [163] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_request, 5, 0, 24),
  [165] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_request, 5, 0, 24),
  [167] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_request, 6, 0, 27),
  [169] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_request, 6, 0, 27),
  [171] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_request, 4, 0, 17),
  [173] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_request, 4, 0, 17),
  [175] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_response, 9, 0, 48),
  [177] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_response, 9, 0, 48),
  [179] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_request, 9, 0, 47),
  [181] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_request, 9, 0, 47),
  [183] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_request, 6, 0, 30),
  [185] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_request, 6, 0, 30),
  [187] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_response, 8, 0, 44),
  [189] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_response, 8, 0, 44),
  [191] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_response, 8, 0, 43),
  [193] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_response, 8, 0, 43),
  [195] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_request, 7, 0, 35),
  [197] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_request, 7, 0, 35),
  [199] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_response, 7, 0, 37),
  [201] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_response, 7, 0, 37),
  [203] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0),
  [205] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0), SHIFT_REPEAT(132),
  [208] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0), SHIFT_REPEAT(155),
  [211] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0), SHIFT_REPEAT(193),
  [214] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0), SHIFT_REPEAT(118),
  [217] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0), SHIFT_REPEAT(225),
  [220] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0), SHIFT_REPEAT(194),
  [223] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0), SHIFT_REPEAT(195),
  [226] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0), SHIFT_REPEAT(219),
  [229] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0), SHIFT_REPEAT(217),
  [232] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_document_repeat1, 2, 0, 0), SHIFT_REPEAT(32),
  [235] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_multipart_form_data, 2, 0, 0),
  [237] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_multipart_form_data, 2, 0, 0),
  [239] = {.entry = {.count = 1, .reusable = false}}, SHIFT(188),
  [241] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [243] = {.entry = {.count = 1, .reusable = false}}, SHIFT(30),
  [245] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_document, 1, 0, 0),
  [247] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_multipart_form_data, 3, 0, 0),
  [249] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_multipart_form_data, 3, 0, 0),
  [251] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [253] = {.entry = {.count = 1, .reusable = false}}, SHIFT(31),
  [255] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_multipart_form_data_repeat1, 2, 0, 0),
  [257] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_multipart_form_data_repeat1, 2, 0, 0),
  [259] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_multipart_form_data_repeat1, 2, 0, 0), SHIFT_REPEAT(188),
  [262] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_multipart_form_data_repeat1, 2, 0, 0), SHIFT_REPEAT(31),
  [265] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_multipart_form_data_repeat1, 2, 0, 0), SHIFT_REPEAT(201),
  [268] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_multipart_form_data_repeat1, 2, 0, 0), SHIFT_REPEAT(31),
  [271] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__section_content, 1, 0, 0),
  [273] = {.entry = {.count = 2, .reusable = false}}, REDUCE(sym__section_content, 1, 0, 0), SHIFT(132),
  [276] = {.entry = {.count = 2, .reusable = true}}, REDUCE(sym__section_content, 1, 0, 0), SHIFT(155),
  [279] = {.entry = {.count = 2, .reusable = false}}, REDUCE(sym__section_content, 1, 0, 0), SHIFT(193),
  [282] = {.entry = {.count = 2, .reusable = true}}, REDUCE(sym__section_content, 1, 0, 0), SHIFT(225),
  [285] = {.entry = {.count = 2, .reusable = true}}, REDUCE(sym__section_content, 1, 0, 0), SHIFT(194),
  [288] = {.entry = {.count = 2, .reusable = true}}, REDUCE(sym__section_content, 1, 0, 0), SHIFT(195),
  [291] = {.entry = {.count = 2, .reusable = true}}, REDUCE(sym__section_content, 1, 0, 0), SHIFT(219),
  [294] = {.entry = {.count = 2, .reusable = true}}, REDUCE(sym__section_content, 1, 0, 0), SHIFT(217),
  [297] = {.entry = {.count = 2, .reusable = false}}, REDUCE(sym__section_content, 1, 0, 0), SHIFT(32),
  [300] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_section, 1, 0, 0),
  [302] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__raw_body, 2, 0, 0),
  [304] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__raw_body, 2, 0, 0),
  [306] = {.entry = {.count = 2, .reusable = false}}, REDUCE(sym__raw_body, 2, 0, 0), SHIFT(235),
  [309] = {.entry = {.count = 1, .reusable = false}}, SHIFT(35),
  [311] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__raw_body, 1, 0, 0),
  [313] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__raw_body, 1, 0, 0),
  [315] = {.entry = {.count = 2, .reusable = false}}, REDUCE(sym__raw_body, 1, 0, 0), SHIFT(235),
  [318] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__plain_comment, 2, 0, 0),
  [320] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__plain_comment, 2, 0, 0),
  [322] = {.entry = {.count = 2, .reusable = false}}, REDUCE(sym__raw_body, 1, 0, 0), SHIFT(260),
  [325] = {.entry = {.count = 1, .reusable = false}}, SHIFT(37),
  [327] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_comment, 1, 0, 0),
  [329] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_comment, 1, 0, 0),
  [331] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_comment, 1, 0, 1),
  [333] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_comment, 1, 0, 1),
  [335] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_raw_body, 2, 0, 0),
  [337] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_raw_body, 2, 0, 0),
  [339] = {.entry = {.count = 2, .reusable = false}}, REDUCE(sym_raw_body, 2, 0, 0), SHIFT(260),
  [342] = {.entry = {.count = 2, .reusable = false}}, REDUCE(sym__raw_body, 2, 0, 0), SHIFT(260),
  [345] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_raw_body, 1, 0, 0),
  [347] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_raw_body, 1, 0, 0),
  [349] = {.entry = {.count = 2, .reusable = false}}, REDUCE(sym_raw_body, 1, 0, 0), SHIFT(260),
  [352] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__var_comment, 7, 0, 33),
  [354] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__var_comment, 7, 0, 33),
  [356] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__var_comment, 4, 0, 11),
  [358] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__var_comment, 4, 0, 11),
  [360] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__var_comment, 6, 0, 25),
  [362] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__var_comment, 6, 0, 25),
  [364] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_graphql_body, 1, 0, 0),
  [366] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_graphql_body, 1, 0, 0),
  [368] = {.entry = {.count = 1, .reusable = true}}, SHIFT(176),
  [370] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym___body_repeat1, 2, 0, 0),
  [372] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym___body_repeat1, 2, 0, 0),
  [374] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym___body_repeat1, 2, 0, 0), SHIFT_REPEAT(256),
  [377] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym___body_repeat1, 2, 0, 0), SHIFT_REPEAT(48),
  [380] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_graphql_data, 2, 0, 0),
  [382] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_graphql_data, 2, 0, 0),
  [384] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__raw_body, 3, 0, 0),
  [386] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__raw_body, 3, 0, 0),
  [388] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_graphql_body, 2, 0, 0),
  [390] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_graphql_body, 2, 0, 0),
  [392] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_res_handler_script, 4, 0, 0),
  [394] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_res_handler_script, 4, 0, 0),
  [396] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_graphql_json_body, 2, 0, 0),
  [398] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_graphql_json_body, 2, 0, 0),
  [400] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__blank_line, 2, 0, 0),
  [402] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__blank_line, 2, 0, 0),
  [404] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__external_body, 2, 0, 0),
  [406] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__external_body, 2, 0, 0),
  [408] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym___body_repeat2, 1, 0, 15),
  [410] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym___body_repeat2, 1, 0, 15),
  [412] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_raw_body, 3, 0, 0),
  [414] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_raw_body, 3, 0, 0),
  [416] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_json_body, 2, 0, 0),
  [418] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_json_body, 2, 0, 0),
  [420] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_res_redirect, 4, 0, 32),
  [422] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_res_redirect, 4, 0, 32),
  [424] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_xml_body, 2, 0, 0),
  [426] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_xml_body, 2, 0, 0),
  [428] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym___body_repeat2, 1, 0, 16),
  [430] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym___body_repeat2, 1, 0, 16),
  [432] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_request, 7, 0, 34),
  [434] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_request, 7, 0, 34),
  [436] = {.entry = {.count = 1, .reusable = true}}, SHIFT(256),
  [438] = {.entry = {.count = 1, .reusable = false}}, SHIFT(178),
  [440] = {.entry = {.count = 1, .reusable = false}}, SHIFT(182),
  [442] = {.entry = {.count = 1, .reusable = false}}, SHIFT(12),
  [444] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_request, 4, 0, 13),
  [446] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_request, 4, 0, 13),
  [448] = {.entry = {.count = 1, .reusable = false}}, SHIFT(6),
  [450] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_request, 5, 0, 21),
  [452] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_request, 5, 0, 21),
  [454] = {.entry = {.count = 1, .reusable = false}}, SHIFT(4),
  [456] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_request, 6, 0, 26),
  [458] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_request, 6, 0, 26),
  [460] = {.entry = {.count = 1, .reusable = false}}, SHIFT(5),
  [462] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_request, 3, 0, 10),
  [464] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_request, 3, 0, 10),
  [466] = {.entry = {.count = 1, .reusable = false}}, SHIFT(7),
  [468] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_request, 2, 0, 6),
  [470] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_request, 2, 0, 6),
  [472] = {.entry = {.count = 1, .reusable = false}}, SHIFT(8),
  [474] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_request, 5, 0, 19),
  [476] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_request, 5, 0, 19),
  [478] = {.entry = {.count = 1, .reusable = false}}, SHIFT(2),
  [480] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_request, 4, 0, 14),
  [482] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_request, 4, 0, 14),
  [484] = {.entry = {.count = 1, .reusable = false}}, SHIFT(11),
  [486] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_request_repeat1, 2, 0, 18),
  [488] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_request_repeat1, 2, 0, 18),
  [490] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_request_repeat1, 2, 0, 18), SHIFT_REPEAT(178),
  [493] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_request_repeat1, 2, 0, 18), SHIFT_REPEAT(182),
  [496] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_response, 7, 0, 38),
  [498] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_response, 7, 0, 38),
  [500] = {.entry = {.count = 1, .reusable = false}}, SHIFT(13),
  [502] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_response, 5, 0, 0),
  [504] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_response, 5, 0, 0),
  [506] = {.entry = {.count = 1, .reusable = false}}, SHIFT(3),
  [508] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_response, 6, 0, 29),
  [510] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_response, 6, 0, 29),
  [512] = {.entry = {.count = 1, .reusable = false}}, SHIFT(9),
  [514] = {.entry = {.count = 1, .reusable = false}}, SHIFT(10),
  [516] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_response_repeat1, 2, 0, 18),
  [518] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_response_repeat1, 2, 0, 18),
  [520] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_response_repeat1, 2, 0, 18), SHIFT_REPEAT(182),
  [523] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_header, 6, 0, 45),
  [525] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_header, 6, 0, 45),
  [527] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_header, 4, 0, 31),
  [529] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_header, 4, 0, 31),
  [531] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_request_repeat1, 1, 0, 9),
  [533] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_request_repeat1, 1, 0, 9),
  [535] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_response_repeat1, 1, 0, 9),
  [537] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_response_repeat1, 1, 0, 9),
  [539] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_header, 4, 0, 22),
  [541] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_header, 4, 0, 22),
  [543] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_header, 3, 0, 22),
  [545] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_header, 3, 0, 22),
  [547] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_header, 5, 0, 22),
  [549] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_header, 5, 0, 22),
  [551] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_header, 5, 0, 40),
  [553] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_header, 5, 0, 40),
  [555] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_variable_declaration, 6, 0, 28),
  [557] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_variable_declaration, 6, 0, 28),
  [559] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_variable_declaration, 7, 0, 36),
  [561] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_variable_declaration, 7, 0, 36),
  [563] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_section, 1, 0, 2),
  [565] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_section, 1, 0, 2),
  [567] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_request_separator, 3, 0, 0),
  [569] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_request_separator, 3, 0, 0),
  [571] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pre_request_script, 4, 0, 0),
  [573] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pre_request_script, 4, 0, 0),
  [575] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__section_content, 1, 0, 3),
  [577] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__section_content, 1, 0, 3),
  [579] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__section_content, 1, 0, 4),
  [581] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__section_content, 1, 0, 4),
  [583] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_request_separator, 2, 0, 0),
  [585] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_request_separator, 2, 0, 0),
  [587] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_request_separator, 4, 0, 12),
  [589] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_request_separator, 4, 0, 12),
  [591] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_request_separator, 3, 0, 7),
  [593] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_request_separator, 3, 0, 7),
  [595] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_variable_declaration, 5, 0, 20),
  [597] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_variable_declaration, 5, 0, 20),
  [599] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__section_content, 2, 0, 5),
  [601] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__section_content, 2, 0, 5),
  [603] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_section, 2, 0, 5),
  [605] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_section, 2, 0, 5),
  [607] = {.entry = {.count = 1, .reusable = true}}, SHIFT(145),
  [609] = {.entry = {.count = 1, .reusable = false}}, SHIFT(145),
  [611] = {.entry = {.count = 1, .reusable = false}}, SHIFT(88),
  [613] = {.entry = {.count = 1, .reusable = true}}, SHIFT(126),
  [615] = {.entry = {.count = 1, .reusable = false}}, SHIFT(108),
  [617] = {.entry = {.count = 1, .reusable = true}}, SHIFT(127),
  [619] = {.entry = {.count = 1, .reusable = true}}, SHIFT(150),
  [621] = {.entry = {.count = 1, .reusable = false}}, SHIFT(150),
  [623] = {.entry = {.count = 1, .reusable = true}}, SHIFT(184),
  [625] = {.entry = {.count = 1, .reusable = true}}, SHIFT(220),
  [627] = {.entry = {.count = 1, .reusable = false}}, SHIFT(89),
  [629] = {.entry = {.count = 1, .reusable = true}}, SHIFT(122),
  [631] = {.entry = {.count = 1, .reusable = true}}, SHIFT(143),
  [633] = {.entry = {.count = 1, .reusable = true}}, SHIFT(136),
  [635] = {.entry = {.count = 1, .reusable = true}}, SHIFT(142),
  [637] = {.entry = {.count = 1, .reusable = false}}, SHIFT(90),
  [639] = {.entry = {.count = 1, .reusable = false}}, SHIFT(100),
  [641] = {.entry = {.count = 1, .reusable = true}}, SHIFT(138),
  [643] = {.entry = {.count = 1, .reusable = true}}, SHIFT(130),
  [645] = {.entry = {.count = 1, .reusable = false}}, SHIFT(130),
  [647] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_path, 1, 0, 0),
  [649] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_path, 1, 0, 0),
  [651] = {.entry = {.count = 1, .reusable = true}}, SHIFT(183),
  [653] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_path_repeat1, 2, 0, 0), SHIFT_REPEAT(130),
  [656] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_path_repeat1, 2, 0, 0), SHIFT_REPEAT(130),
  [659] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_path_repeat1, 2, 0, 0),
  [661] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_path_repeat1, 2, 0, 0),
  [663] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_path_repeat1, 2, 0, 0), SHIFT_REPEAT(183),
  [666] = {.entry = {.count = 1, .reusable = true}}, SHIFT(144),
  [668] = {.entry = {.count = 1, .reusable = true}}, SHIFT(134),
  [670] = {.entry = {.count = 1, .reusable = false}}, SHIFT(134),
  [672] = {.entry = {.count = 2, .reusable = true}}, REDUCE(sym_target_url, 1, 0, 0), SHIFT(134),
  [675] = {.entry = {.count = 2, .reusable = false}}, REDUCE(sym_target_url, 1, 0, 0), SHIFT(259),
  [678] = {.entry = {.count = 1, .reusable = true}}, SHIFT(140),
  [680] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__target_url_line, 2, 0, 0), SHIFT_REPEAT(134),
  [683] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym__target_url_line, 2, 0, 0), SHIFT_REPEAT(134),
  [686] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym__target_url_line, 2, 0, 0),
  [688] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__target_url_line, 2, 0, 0), SHIFT_REPEAT(195),
  [691] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_target_url_repeat1, 3, 0, 0), SHIFT(134),
  [694] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_target_url_repeat1, 3, 0, 0),
  [696] = {.entry = {.count = 1, .reusable = true}}, SHIFT(129),
  [698] = {.entry = {.count = 1, .reusable = false}}, SHIFT(129),
  [700] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_path_repeat1, 2, 0, 0), SHIFT_REPEAT(141),
  [703] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_path_repeat1, 2, 0, 0), SHIFT_REPEAT(141),
  [706] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_path_repeat1, 2, 0, 0), SHIFT_REPEAT(184),
  [709] = {.entry = {.count = 1, .reusable = true}}, SHIFT(149),
  [711] = {.entry = {.count = 1, .reusable = false}}, SHIFT(149),
  [713] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_value, 1, 0, 0),
  [715] = {.entry = {.count = 1, .reusable = true}}, SHIFT(132),
  [717] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_value_repeat1, 2, 0, 0), SHIFT_REPEAT(149),
  [720] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_value_repeat1, 2, 0, 0), SHIFT_REPEAT(149),
  [723] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_value_repeat1, 2, 0, 0),
  [725] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_value_repeat1, 2, 0, 0), SHIFT_REPEAT(195),
  [728] = {.entry = {.count = 1, .reusable = true}}, SHIFT(141),
  [730] = {.entry = {.count = 1, .reusable = false}}, SHIFT(141),
  [732] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_variable, 3, 0, 8),
  [734] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_variable, 3, 0, 8),
  [736] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_variable, 5, 0, 11),
  [738] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_variable, 5, 0, 11),
  [740] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_variable, 4, 0, 8),
  [742] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_variable, 4, 0, 8),
  [744] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_variable, 4, 0, 11),
  [746] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_variable, 4, 0, 11),
  [748] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__target_url_line, 1, 0, 0),
  [750] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym__target_url_line, 1, 0, 0),
  [752] = {.entry = {.count = 1, .reusable = false}}, SHIFT(106),
  [754] = {.entry = {.count = 1, .reusable = true}}, SHIFT(135),
  [756] = {.entry = {.count = 1, .reusable = false}}, SHIFT(135),
  [758] = {.entry = {.count = 1, .reusable = false}}, SHIFT(177),
  [760] = {.entry = {.count = 1, .reusable = true}}, SHIFT(229),
  [762] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_script_repeat1, 2, 0, 0), SHIFT_REPEAT(166),
  [765] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_script_repeat1, 2, 0, 0),
  [767] = {.entry = {.count = 1, .reusable = true}}, SHIFT(260),
  [769] = {.entry = {.count = 1, .reusable = true}}, SHIFT(235),
  [771] = {.entry = {.count = 1, .reusable = true}}, SHIFT(125),
  [773] = {.entry = {.count = 1, .reusable = true}}, SHIFT(94),
  [775] = {.entry = {.count = 1, .reusable = true}}, SHIFT(133),
  [777] = {.entry = {.count = 1, .reusable = true}}, SHIFT(56),
  [779] = {.entry = {.count = 1, .reusable = true}}, SHIFT(124),
  [781] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [783] = {.entry = {.count = 1, .reusable = true}}, SHIFT(128),
  [785] = {.entry = {.count = 1, .reusable = true}}, SHIFT(109),
  [787] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_target_url, 2, 0, 0),
  [789] = {.entry = {.count = 2, .reusable = true}}, REDUCE(sym_target_url, 2, 0, 0), SHIFT(259),
  [792] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_target_url_repeat1, 2, 0, 0),
  [794] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_target_url_repeat1, 2, 0, 0), SHIFT_REPEAT(259),
  [797] = {.entry = {.count = 1, .reusable = false}}, SHIFT(166),
  [799] = {.entry = {.count = 1, .reusable = true}}, SHIFT(230),
  [801] = {.entry = {.count = 1, .reusable = false}}, SHIFT(84),
  [803] = {.entry = {.count = 1, .reusable = true}}, SHIFT(249),
  [805] = {.entry = {.count = 1, .reusable = true}}, SHIFT(254),
  [807] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [809] = {.entry = {.count = 1, .reusable = false}}, SHIFT(148),
  [811] = {.entry = {.count = 1, .reusable = false}}, SHIFT(208),
  [813] = {.entry = {.count = 1, .reusable = true}}, SHIFT(215),
  [815] = {.entry = {.count = 1, .reusable = true}}, SHIFT(121),
  [817] = {.entry = {.count = 1, .reusable = false}}, SHIFT(255),
  [819] = {.entry = {.count = 1, .reusable = false}}, SHIFT(187),
  [821] = {.entry = {.count = 1, .reusable = false}}, SHIFT(250),
  [823] = {.entry = {.count = 1, .reusable = false}}, SHIFT(192),
  [825] = {.entry = {.count = 1, .reusable = true}}, SHIFT(239),
  [827] = {.entry = {.count = 1, .reusable = true}}, SHIFT(131),
  [829] = {.entry = {.count = 1, .reusable = true}}, SHIFT(242),
  [831] = {.entry = {.count = 1, .reusable = true}}, SHIFT(154),
  [833] = {.entry = {.count = 1, .reusable = true}}, SHIFT(241),
  [835] = {.entry = {.count = 1, .reusable = true}}, SHIFT(151),
  [837] = {.entry = {.count = 1, .reusable = false}}, SHIFT(36),
  [839] = {.entry = {.count = 1, .reusable = true}}, SHIFT(258),
  [841] = {.entry = {.count = 1, .reusable = true}}, SHIFT(218),
  [843] = {.entry = {.count = 1, .reusable = true}}, SHIFT(164),
  [845] = {.entry = {.count = 1, .reusable = true}}, SHIFT(233),
  [847] = {.entry = {.count = 1, .reusable = true}}, SHIFT(159),
  [849] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_external_body, 4, 0, 41),
  [851] = {.entry = {.count = 1, .reusable = true}}, SHIFT(206),
  [853] = {.entry = {.count = 1, .reusable = true}}, SHIFT(160),
  [855] = {.entry = {.count = 1, .reusable = false}}, SHIFT(107),
  [857] = {.entry = {.count = 1, .reusable = true}}, SHIFT(227),
  [859] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_http_version, 1, 1, 0),
  [861] = {.entry = {.count = 1, .reusable = false}}, SHIFT(237),
  [863] = {.entry = {.count = 1, .reusable = false}}, SHIFT(189),
  [865] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_external_body, 5, 0, 46),
  [867] = {.entry = {.count = 1, .reusable = true}}, SHIFT(79),
  [869] = {.entry = {.count = 1, .reusable = true}}, SHIFT(248),
  [871] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [873] = {.entry = {.count = 1, .reusable = true}}, SHIFT(202),
  [875] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [877] = {.entry = {.count = 1, .reusable = true}}, SHIFT(181),
  [879] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [881] = {.entry = {.count = 1, .reusable = true}}, SHIFT(139),
  [883] = {.entry = {.count = 1, .reusable = true}}, SHIFT(180),
  [885] = {.entry = {.count = 1, .reusable = true}}, SHIFT(253),
  [887] = {.entry = {.count = 1, .reusable = true}}, SHIFT(163),
  [889] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_external_body, 3, 0, 32),
  [891] = {.entry = {.count = 1, .reusable = true}}, SHIFT(58),
  [893] = {.entry = {.count = 1, .reusable = true}}, SHIFT(158),
  [895] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [897] = {.entry = {.count = 1, .reusable = true}}, SHIFT(147),
  [899] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
  [901] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [903] = {.entry = {.count = 1, .reusable = true}}, SHIFT(91),
  [905] = {.entry = {.count = 1, .reusable = true}}, SHIFT(137),
  [907] = {.entry = {.count = 1, .reusable = true}}, SHIFT(119),
  [909] = {.entry = {.count = 1, .reusable = true}}, SHIFT(257),
  [911] = {.entry = {.count = 1, .reusable = true}}, SHIFT(117),
  [913] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [915] = {.entry = {.count = 1, .reusable = true}}, SHIFT(185),
  [917] = {.entry = {.count = 1, .reusable = true}}, SHIFT(162),
  [919] = {.entry = {.count = 1, .reusable = true}}, SHIFT(120),
  [921] = {.entry = {.count = 1, .reusable = true}}, SHIFT(165),
  [923] = {.entry = {.count = 1, .reusable = true}}, SHIFT(103),
  [925] = {.entry = {.count = 1, .reusable = true}}, SHIFT(98),
  [927] = {.entry = {.count = 1, .reusable = true}}, SHIFT(110),
  [929] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [931] = {.entry = {.count = 1, .reusable = true}}, SHIFT(146),
  [933] = {.entry = {.count = 1, .reusable = true}}, SHIFT(113),
  [935] = {.entry = {.count = 1, .reusable = true}}, SHIFT(173),
  [937] = {.entry = {.count = 1, .reusable = true}}, SHIFT(85),
  [939] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_script, 3, 0, 0),
  [941] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_script, 4, 0, 0),
  [943] = {.entry = {.count = 1, .reusable = true}}, SHIFT(111),
  [945] = {.entry = {.count = 1, .reusable = true}}, SHIFT(102),
  [947] = {.entry = {.count = 1, .reusable = true}}, SHIFT(157),
  [949] = {.entry = {.count = 1, .reusable = true}}, SHIFT(96),
  [951] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [953] = {.entry = {.count = 1, .reusable = true}}, SHIFT(93),
  [955] = {.entry = {.count = 1, .reusable = true}}, SHIFT(203),
  [957] = {.entry = {.count = 1, .reusable = true}}, SHIFT(97),
  [959] = {.entry = {.count = 1, .reusable = true}}, SHIFT(123),
  [961] = {.entry = {.count = 1, .reusable = true}}, SHIFT(83),
  [963] = {.entry = {.count = 1, .reusable = true}}, SHIFT(153),
  [965] = {.entry = {.count = 1, .reusable = true}}, SHIFT(152),
  [967] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [969] = {.entry = {.count = 1, .reusable = true}}, SHIFT(62),
  [971] = {.entry = {.count = 1, .reusable = true}}, SHIFT(101),
  [973] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [975] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [977] = {.entry = {.count = 1, .reusable = true}}, SHIFT(81),
  [979] = {.entry = {.count = 1, .reusable = true}}, SHIFT(170),
  [981] = {.entry = {.count = 1, .reusable = true}}, SHIFT(190),
  [983] = {.entry = {.count = 1, .reusable = true}}, SHIFT(197),
  [985] = {.entry = {.count = 1, .reusable = true}}, SHIFT(76),
  [987] = {.entry = {.count = 1, .reusable = true}}, SHIFT(161),
  [989] = {.entry = {.count = 1, .reusable = true}}, SHIFT(171),
  [991] = {.entry = {.count = 1, .reusable = true}}, SHIFT(186),
  [993] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [995] = {.entry = {.count = 1, .reusable = true}}, SHIFT(251),
  [997] = {.entry = {.count = 1, .reusable = true}}, SHIFT(172),
  [999] = {.entry = {.count = 1, .reusable = true}}, SHIFT(156),
  [1001] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
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

TS_PUBLIC const TSLanguage *tree_sitter_http(void) {
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
    .field_names = ts_field_names,
    .field_map_slices = ts_field_map_slices,
    .field_map_entries = ts_field_map_entries,
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

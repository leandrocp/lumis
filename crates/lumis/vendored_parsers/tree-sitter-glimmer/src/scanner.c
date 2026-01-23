#include <tree_sitter/parser.h>
#include <wctype.h>
#include <stdio.h>
#include <string.h>

enum TokenType {
    COMMENT,
    RAW_TEXT,
};

void *tree_sitter_glimmer_external_scanner_create() { return NULL; }
void tree_sitter_glimmer_external_scanner_destroy(void *p) {}
void tree_sitter_glimmer_external_scanner_reset(void *p) {}
unsigned tree_sitter_glimmer_external_scanner_serialize(void *p, char *buffer) { return 0; }
void tree_sitter_glimmer_external_scanner_deserialize(void *p, const char *b, unsigned n) {}

static void advance(TSLexer *lexer) { lexer->advance(lexer, false); }

static bool scan_raw_text(TSLexer *lexer) {
    // Scan everything up to (but not including) the closing `</style` or
    // `</script` tag.
    //
    // This is inspired by tree-sitter-html's raw_text scanning approach, but
    // simplified: we don't maintain an HTML tag stack, so we stop at whichever
    // of the two closing tags appears first.
    lexer->mark_end(lexer);

    const char *style_end = "</STYLE";
    const char *script_end = "</SCRIPT";
    const unsigned style_end_len = (unsigned)strlen(style_end);
    const unsigned script_end_len = (unsigned)strlen(script_end);

    unsigned style_i = 0;
    unsigned script_i = 0;

    while (lexer->lookahead) {
        const wint_t c = towupper(lexer->lookahead);

        if (c == (wint_t)style_end[style_i]) {
            style_i++;
        } else {
            style_i = (c == (wint_t)style_end[0]) ? 1 : 0;
        }

        if (c == (wint_t)script_end[script_i]) {
            script_i++;
        } else {
            script_i = (c == (wint_t)script_end[0]) ? 1 : 0;
        }

        if (style_i == style_end_len || script_i == script_end_len) {
            break;
        }

        advance(lexer);

        // Only mark the end once we are sure the consumed characters cannot be
        // the beginning of a closing delimiter.
        if (style_i == 0 && script_i == 0) {
            lexer->mark_end(lexer);
        }
    }

    lexer->result_symbol = RAW_TEXT;
    return true;
}

static bool scan_html_comment(TSLexer *lexer) {
    // Ensure the next characters are `!--`
    if (lexer->lookahead != '!') return false;
    advance(lexer);
    if (lexer->lookahead != '-') return false;
    advance(lexer);
    if (lexer->lookahead != '-') return false;
    advance(lexer);

    // Consume characters until we read `-->`
    unsigned dashes = 0;
    while (lexer->lookahead) {
        switch (lexer->lookahead) {
            case '-':
                ++dashes;
                break;
            case '>':
                if (dashes >= 2) {
                    lexer->result_symbol = COMMENT;
                    advance(lexer);
                    lexer->mark_end(lexer);
                    return true;
                }
            default:
                dashes = 0;
        }
        advance(lexer);
    }

    return false;
}

static bool scan_multi_line_handlebars_comment(TSLexer *lexer) {
    // Ensure the next character is `-`
    if (lexer->lookahead != '-') return false;

    // Consume characters until we read `--}}`
    unsigned dashes = 0;
    unsigned brackets = 0;
    while (lexer->lookahead) {
        switch (lexer->lookahead) {
            case '-':
                ++dashes;
                break;
            case '}':
                ++brackets;
                if (dashes >= 2 && brackets == 2) {
                    lexer->result_symbol = COMMENT;
                    advance(lexer);
                    lexer->mark_end(lexer);
                    return true;
                } else {
                    break;
                }
            default:
                dashes = 0;
                brackets = 0;
        }
        advance(lexer);
    }

    return false;
}

static bool scan_single_line_handlebars_comment(TSLexer *lexer) {
    // Consume characters until we read `}}`
    unsigned brackets = 0;
    while (lexer->lookahead) {
        switch (lexer->lookahead) {
            case '}':
                ++brackets;
                if (brackets == 2) {
                    lexer->result_symbol = COMMENT;
                    advance(lexer);
                    lexer->mark_end(lexer);
                    return true;
                } else {
                    break;
                }
            default:
                brackets = 0;
        }
        advance(lexer);
    }

    return false;
}


static bool scan_handlebars_comment(TSLexer *lexer) {
    // Ensure the next characters are `{!`
    if (lexer->lookahead != '{') return false;
    advance(lexer);
    if (lexer->lookahead != '!') return false;
    advance(lexer);

    // Handle either multi-line or single-line comment
    switch (lexer->lookahead) {
        // Multi-Line Comment
        case '-':
            advance(lexer);

            return scan_multi_line_handlebars_comment(lexer);

        // Single-Line Comment
        default:
            advance(lexer);

            return scan_single_line_handlebars_comment(lexer);
    }

    return false;
}

bool tree_sitter_glimmer_external_scanner_scan(void *payload, TSLexer *lexer,
                                                  const bool *valid_symbols) {
    // When parsing a <style> element's content, the grammar expects a single
    // RAW_TEXT token that includes whitespace.
    if (valid_symbols[RAW_TEXT]) {
        return scan_raw_text(lexer);
    }

    // Eat whitespace for the comment token. Note: raw text must *not* take
    // this path, because whitespace is significant inside <style>.
    while (iswspace(lexer->lookahead)) {
        lexer->advance(lexer, true);
    }

    if (valid_symbols[COMMENT]) {
        switch (lexer->lookahead) {
            case '<':
                lexer->mark_end(lexer);
                advance(lexer);

                return scan_html_comment(lexer);

            case '{':
                lexer->mark_end(lexer);
                advance(lexer);

                return scan_handlebars_comment(lexer);

            default:
                return false;

      }
  }

  return false;
}

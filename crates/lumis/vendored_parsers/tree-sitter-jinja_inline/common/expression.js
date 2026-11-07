const { commaSep1, anySep1 } = require('./common.js');

exports.rules = {
  expression: $ =>
    prec.left(
      choice($.binary_expression, seq($.expression, $.ternary_expression)),
    ),
  ternary_expression: $ =>
    prec.right(seq('if', $.expression, optional(seq('else', $.expression)))),
  binary_expression: $ =>
    choice(
      $.unary_expression,
      seq($.binary_expression, $.binary_operator, $.unary_expression),
      seq(
        $.binary_expression,
        alias('is', $.binary_operator),
        choice($.unary_expression, $.builtin_test),
      ),
      seq(
        $.binary_expression,
        alias(seq(optional('not'), 'in'), $.binary_operator),
        $.unary_expression,
      ),
    ),
  builtin_test: $ =>
    prec.left(
      choice(
        'boolean',
        'callable',
        'defined',
        seq('divisibleby', $.expression),
        seq('eq', $.expression),
        'escaped',
        'even',
        'filter',
        'float',
        seq('ge', $.expression),
        seq('gt', $.expression),
        seq('in', $.expression),
        'integer',
        'iterable',
        seq('le', $.expression),
        'lower',
        seq('lt', $.expression),
        'mapping',
        seq('ne', $.expression),
        'number',
        'odd',
        seq('sameas', $.expression),
        'sequence',
        'string',
        'test',
        'undefined',
        'upper',
      ),
    ),
  assignment_expression: $ =>
    seq(
      anySep1($.identifier, '.'),
      alias('=', $.binary_operator),
      $.expression,
    ),
  in_expression: $ =>
    prec(
      1,
      seq(
        commaSep1($.identifier),
        alias('in', $.binary_operator),
        $.expression,
      ),
    ),
  binary_operator: _ =>
    choice(
      '+',
      '-',
      '*',
      '/',
      '//',
      '%',
      '**',
      '==',
      '!=',
      '>',
      '>=',
      '<',
      '<=',
      'and',
      'or',
      '|',
      '~',
    ),
  unary_expression: $ =>
    choice($.primary_expression, seq($.unary_operator, $.unary_expression)),
  unary_operator: _ => choice('not', '!'),
  primary_expression: $ =>
    choice(
      $.function_call,
      $.literal,
      $.inline_trans,
      $.identifier,
      seq('(', $.expression, ')'),
      seq(
        $.primary_expression,
        '.',
        choice($.identifier, $.function_call, $.dec_literal),
      ),
      seq($.primary_expression, '[', $.expression, ']'),
      seq($.primary_expression, $._arguments),
    ),

  function_call: $ => prec(1, seq($.identifier, $._arguments)),
  _arguments: $ =>
    seq('(', optional(seq(commaSep1($.arg), optional(','))), ')'),
  arg: $ =>
    seq(
      optional(seq($.identifier, alias('=', $.binary_operator))),
      $.expression,
    ),
  inline_trans: $ => seq('_', '(', $.expression, ')'),
};

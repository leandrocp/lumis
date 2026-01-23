/**
 * @file CSV grammar for tree-sitter
 * @author Amaan Qureshi <amaanq12@gmail.com>
 * @license MIT
 */

const defineGrammar = require('../common/define-grammar');

module.exports = defineGrammar('csv', ',');

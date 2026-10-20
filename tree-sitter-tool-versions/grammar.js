/**
 * tree-sitter grammar for `.tool-versions` files, as asdf and mise read them: a tool and its
 * versions on each line, the first one installed being used, and `#` comments.
 *
 *     node 24.11.0 22  # tried in this order
 *     python ref:main
 *     # a comment
 */
module.exports = grammar({
  name: "tool_versions",

  extras: _ => [/[ \t]/],

  rules: {
    // Lines are separated rather than ended, so the last one needs no newline.
    document: $ => seq(optional($._line), repeat(seq($._newline, optional($._line)))),

    _line: $ => choice(seq($.tool, optional($.comment)), $.comment),

    tool: $ =>
      seq(
        field("name", alias($._word, $.name)),
        repeat1(field("version", alias($._word, $.version))),
      ),

    _word: _ => /[^\s#]+/,

    comment: _ => /#[^\r\n]*/,

    _newline: _ => /\r?\n/,
  },
});

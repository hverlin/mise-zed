; From tree-sitter-kdl, https://github.com/tree-sitter-grammars/tree-sitter-kdl (MIT). Zed uses
; the last pattern that matches a node: the general ones come first.

(identifier) @variable

(node name: (identifier) @tag)
(node name: (identifier (string) @tag))

(type) @type
(type name: (identifier) @type)

(prop key: (identifier) @property)

"=" @operator

(string) @string
(multi_line_string) @string
(string_fragment) @string
(escape) @string.escape
(escaped_whitespace) @string.escape

(number) @number
(keyword_number) @number

(boolean) @boolean

[ "null" "#null" ] @constant.builtin

["{" "}"] @punctuation.bracket
["(" ")"] @punctuation.bracket
";" @punctuation.delimiter

[
  (single_line_comment)
  (multi_line_comment)
  (version)
] @comment

(node_comment) @comment
(node_field_comment) @comment
(node_children_comment) @comment

; usage: the directives of a spec, https://usage.jdx.dev/spec/
((node name: (identifier) @keyword)
  (#any-of? @keyword
    "arg" "flag" "cmd" "complete" "choices" "alias" "mount" "example" "include" "config"
    "name" "bin" "version" "about" "long_about" "author" "license" "usage" "help" "long_help"
    "before_help" "after_help" "before_long_help" "after_long_help" "source_code_link_template"
    "min_usage_version" "env" "default"))

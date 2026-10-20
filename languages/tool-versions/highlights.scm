; Zed uses the last pattern that matches a node: the general ones come first.

(comment) @comment

(name) @property

(version) @string

; versions mise picks with a prefix: ref:main, path:/opt/node, prefix:20, sub-1:lts
((version) @keyword
  (#match? @keyword "^(ref|path|prefix|sub-[^:]+):"))

((version) @constant.builtin
  (#match? @constant.builtin "^(latest|system|lts(-.+)?)$"))

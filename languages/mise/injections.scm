; From the TOML extension, https://github.com/zed-extensions/toml (Apache-2.0)

((comment) @injection.content
  (#set! injection.language "comment"))

; mise: the script of a task, `run = "..."`, `run = '''...'''` or each string of
; `run = [...]`, as shell, or as the language its shebang names. A task or a hook given as a
; string in `[tasks]` or `[hooks]` is shell too. `usage` is a usage spec (KDL).

((pair
  [(bare_key) @_key (dotted_key (bare_key) @_key .)]
  [(string (string_content) @injection.content)
   (array (string (string_content) @injection.content))])
  (#eq? @_key "run")
  (#not-match? @injection.content "^\\s*#!.*\\b(python[0-9.]*|uv|node|bun|deno|ruby)\\b")
  (#set! injection.language "mise shell"))

((pair
  [(bare_key) @_key (dotted_key (bare_key) @_key .)]
  [(string (string_content) @injection.content)
   (array (string (string_content) @injection.content))])
  (#eq? @_key "run")
  (#match? @injection.content "^\\s*#!.*\\b(python[0-9.]*|uv)\\b")
  (#set! injection.language "python"))

((pair
  [(bare_key) @_key (dotted_key (bare_key) @_key .)]
  [(string (string_content) @injection.content)
   (array (string (string_content) @injection.content))])
  (#eq? @_key "run")
  (#match? @injection.content "^\\s*#!.*\\b(node|bun|deno)\\b")
  (#set! injection.language "javascript"))

((pair
  [(bare_key) @_key (dotted_key (bare_key) @_key .)]
  [(string (string_content) @injection.content)
   (array (string (string_content) @injection.content))])
  (#eq? @_key "run")
  (#match? @injection.content "^\\s*#!.*\\bruby\\b")
  (#set! injection.language "ruby"))

((table
  (bare_key) @_section
  (pair
    [(bare_key) (quoted_key)]
    [(string (string_content) @injection.content)
     (array (string (string_content) @injection.content))]))
  (#any-of? @_section "tasks" "hooks")
  (#set! injection.language "mise shell"))

((pair
  [(bare_key) @_key (dotted_key (bare_key) @_key .)]
  (string (string_content) @injection.content))
  (#eq? @_key "usage")
  (#set! injection.language "usage"))

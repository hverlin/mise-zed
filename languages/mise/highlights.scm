; From the TOML extension, https://github.com/zed-extensions/toml (Apache-2.0)

; Properties
;-----------
(bare_key) @property

(quoted_key) @property

; Literals
;---------
(boolean) @constant

(comment) @comment

(integer) @number

(float) @number

(string) @string

(escape_sequence) @string.escape

(offset_date_time) @string.special

(local_date_time) @string.special

(local_date) @string.special

(local_time) @string.special

; Punctuation
;------------
[
  "."
  ","
] @punctuation.delimiter

"=" @operator

[
  "["
  "]"
  "[["
  "]]"
  "{"
  "}"
] @punctuation.bracket

; mise: what a header, or one key and its value, tells. These come after the TOML patterns
; above, so they win. None looks at a header and at the keys under it: Zed keeps at most 64
; matches open per query, and such a match stays open to the end of the section, so the keys
; after the first few would lose their color.

; the section of `[tools]`, `[tasks.build]`, `[tasks.build.env]`, `[env._.python]`
(table
  [(bare_key) @keyword
   (dotted_key . (bare_key) @keyword)
   (dotted_key (dotted_key . (bare_key) @keyword))
   (dotted_key (dotted_key (dotted_key . (bare_key) @keyword)))]
  (#any-of? @keyword
    "alias" "bootstrap" "daemon_groups" "daemon_providers" "daemons" "daemons_settings" "deps"
    "doctor" "dotfile_groups" "dotfiles" "env" "history" "hooks" "monorepo" "oci" "plugins"
    "redactions" "secrets" "settings" "shell_alias" "task_config" "task_templates" "tasks"
    "tool_alias" "tool_config" "tools" "vars" "watch_files" "wrappers"))

; `[[watch_files]]`, `[[env]]`, `[[tools.erlang]]`, `[[hooks.enter]]`
(table_array_element
  [(bare_key) @keyword
   (dotted_key . (bare_key) @keyword)]
  (#any-of? @keyword "env" "hooks" "tools" "watch_files"))

; the task, template or hook of `[tasks.build]`, `[tasks.build.env]`, `[hooks.enter]`
(table
  [(dotted_key . (bare_key) @_section . [(bare_key) (quoted_key)] @function)
   (dotted_key (dotted_key . (bare_key) @_section . [(bare_key) (quoted_key)] @function))
   (dotted_key (dotted_key (dotted_key . (bare_key) @_section . [(bare_key) (quoted_key)] @function)))]
  (#any-of? @_section "tasks" "task_templates" "hooks"))

(table_array_element
  (dotted_key . (bare_key) @_section . [(bare_key) (quoted_key)] @function)
  (#eq? @_section "hooks"))

; the tool of `[tools.node]` and `[[tools.erlang]]`
(table
  (dotted_key . (bare_key) @_section . [(bare_key) (quoted_key)] @type)
  (#eq? @_section "tools"))

(table_array_element
  (dotted_key . (bare_key) @_section . [(bare_key) (quoted_key)] @type)
  (#eq? @_section "tools"))

; `latest`, `lts` and `system` versions: `node = "lts"`, `version = "latest"`,
; `python = ["3.12", "system"]`, `node = { version = "lts" }`
([(pair (string (string_content) @constant))
  (pair (array (string (string_content) @constant)))]
  (#match? @constant "^(latest|lts|lts-[a-z]+|system)$"))

; environment variables, by their upper case: `NODE_ENV = "..."`, `env = { NODE_ENV = "..." }`
((pair . (bare_key) @variable)
  (#match? @variable "^[A-Z][A-Z0-9_]*$"))

; the `_` of `_.path`, `_.python.venv` and `[env._.python]`
((dotted_key (bare_key) @keyword)
  (#eq? @keyword "_"))

; a Tera template, `{{ ... }}` or `{% ... %}`, except in `run` and `usage`
(pair
  [(bare_key) @_key (quoted_key) @_key (dotted_key (_) @_key .)]
  [(string (string_content) @string.special)
   (array (string (string_content) @string.special))]
  (#not-any-of? @_key "run" "run_windows" "usage")
  (#match? @string.special "\\{\\{|\\{%"))

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

; mise: the sections of a config, the names of tasks, hooks and tools, `latest`, `lts` and
; `system` versions, environment variables and strings with a Tera template. These come after
; the TOML patterns above, so they win.

; the section of `[tools]`, `[tasks.build]`, `[tasks.build.env]`, `[[watch_files]]`
(table
  [(bare_key) @keyword
   (dotted_key . (bare_key) @keyword)
   (dotted_key (dotted_key . (bare_key) @keyword))]
  (#any-of? @keyword
    "alias" "bootstrap" "daemon_groups" "daemon_providers" "daemons" "daemons_settings" "deps"
    "doctor" "dotfile_groups" "dotfiles" "env" "history" "hooks" "monorepo" "oci" "plugins"
    "redactions" "secrets" "settings" "shell_alias" "task_config" "task_templates" "tasks"
    "tool_alias" "tool_config" "tools" "vars" "watch_files" "wrappers"))

(table_array_element
  (bare_key) @keyword
  (#eq? @keyword "watch_files"))

; tasks and templates: `[tasks.build]`, `[tasks.build.env]`, `build = "..."` in `[tasks]`
(table
  [(dotted_key . (bare_key) @_section . [(bare_key) (quoted_key)] @function)
   (dotted_key (dotted_key . (bare_key) @_section . [(bare_key) (quoted_key)] @function))]
  (#any-of? @_section "tasks" "task_templates"))

(table
  (bare_key) @_section
  (pair . [(bare_key) (quoted_key)] @function)
  (#any-of? @_section "tasks" "task_templates" "hooks"))

; tools: `node = "24"` in `[tools]` and `[tasks.build.tools]`, and `[tools.node]`
(table
  (bare_key) @_section
  (pair . [(bare_key) (quoted_key)] @type)
  (#eq? @_section "tools"))

(table
  (dotted_key . (bare_key) @_section . [(bare_key) (quoted_key)] @type)
  (#eq? @_section "tools"))

(table
  (dotted_key (dotted_key . (bare_key) @_section) (bare_key) @_tools .)
  (pair . [(bare_key) (quoted_key)] @type)
  (#eq? @_section "tasks")
  (#eq? @_tools "tools"))

; `latest`, `lts` and `system` versions, in `[tools]`, `[tasks.build.tools]` and the `version`
; of `[tools.node]`
(table
  (bare_key) @_section
  (pair (string (string_content) @constant))
  (#eq? @_section "tools")
  (#match? @constant "^(latest|lts|lts-[a-z]+|system)$"))

(table
  (dotted_key (dotted_key . (bare_key) @_section) (bare_key) @_tools .)
  (pair (string (string_content) @constant))
  (#eq? @_section "tasks")
  (#eq? @_tools "tools")
  (#match? @constant "^(latest|lts|lts-[a-z]+|system)$"))

(table
  (dotted_key . (bare_key) @_section)
  (pair (bare_key) @_version (string (string_content) @constant))
  (#eq? @_section "tools")
  (#eq? @_version "version")
  (#match? @constant "^(latest|lts|lts-[a-z]+|system)$"))

; environment variables and vars: the keys of `[env]`, `[vars]` and `[tasks.build.env]`, and
; the `_` of `_.path` and the other env directives
(table
  (bare_key) @_section
  (pair . [(bare_key) (quoted_key)] @variable)
  (#any-of? @_section "env" "vars"))

(table
  (dotted_key (dotted_key . (bare_key) @_section) (bare_key) @_env .)
  (pair . [(bare_key) (quoted_key)] @variable)
  (#eq? @_section "tasks")
  (#eq? @_env "env"))

(table
  (bare_key) @_section
  (pair (dotted_key . (bare_key) @keyword))
  (#eq? @_section "env")
  (#eq? @keyword "_"))

; a Tera template, `{{ ... }}` or `{% ... %}`, outside the scripts of tasks
(pair
  [(bare_key) (quoted_key) (dotted_key (_) .)] @_key
  [(string (string_content) @string.special)
   (array (string (string_content) @string.special))]
  (#not-any-of? @_key "run" "usage")
  (#match? @string.special "\\{\\{|\\{%"))

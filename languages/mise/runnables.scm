; A run button on each task, and on each tool. The key is exposed to tasks.json as
; $ZED_CUSTOM_MISE_TASK or $ZED_CUSTOM_MISE_TOOL, quotes included when the key is quoted.

; [tasks.build], [tasks."docs:build"], and the same for [tools.node]
((table
  (dotted_key
    .
    (bare_key) @_section
    .
    [(bare_key) (quoted_key)] @run @MISE_TASK
    .))
  (#eq? @_section "tasks")
  (#set! tag mise-task))

((table
  (dotted_key
    .
    (bare_key) @_section
    .
    [(bare_key) (quoted_key)] @run @MISE_TOOL
    .))
  (#eq? @_section "tools")
  (#set! tag mise-tool))

; build = "..." in [tasks], node = "24" in [tools]
((table
  .
  (bare_key) @_section
  (pair
    .
    [(bare_key) (quoted_key)] @run @MISE_TASK))
  (#eq? @_section "tasks")
  (#set! tag mise-task))

((table
  .
  (bare_key) @_section
  (pair
    .
    [(bare_key) (quoted_key)] @run @MISE_TOOL))
  (#eq? @_section "tools")
  (#set! tag mise-tool))

; tasks.build = "..." and tools.node = "24" outside of a table
((document
  (pair
    .
    (dotted_key
      .
      (bare_key) @_section
      .
      [(bare_key) (quoted_key)] @run @MISE_TASK
      .)))
  (#eq? @_section "tasks")
  (#set! tag mise-task))

((document
  (pair
    .
    (dotted_key
      .
      (bare_key) @_section
      .
      [(bare_key) (quoted_key)] @run @MISE_TOOL
      .)))
  (#eq? @_section "tools")
  (#set! tag mise-tool))

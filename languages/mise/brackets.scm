; From the TOML extension, https://github.com/zed-extensions/toml (Apache-2.0)

(array
  "[" @open
  "]" @close)

(inline_table
  "{" @open
  "}" @close)

(table
  "[" @open
  "]" @close
  (#set! rainbow.exclude))

(table_array_element
  "[[" @open
  "]]" @close
  (#set! rainbow.exclude))

(("\"" @open
  "\"" @close)
  (#set! rainbow.exclude))

(("'" @open
  "'" @close)
  (#set! rainbow.exclude))

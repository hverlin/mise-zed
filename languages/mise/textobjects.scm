; From the TOML extension, https://github.com/zed-extensions/toml (Apache-2.0)

((comment)+ @comment.around) @comment.inside

(table
  "["
  (_)
  "]"
  (_)* @class.inside) @class.around

(table_array_element
  "[["
  (_)
  "]]"
  (_)* @class.inside) @class.around

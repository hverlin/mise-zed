# tree-sitter-usage-kdl

[tree-sitter-kdl](https://github.com/tree-sitter-grammars/tree-sitter-kdl) 2.0.0 (`e8ff98a1`),
which reads KDL 2, renamed `usage_kdl`. [Usage](https://usage.jdx.dev/) specs, such as the
`usage` of a mise task, are KDL 2 (`required=#true`). The KDL extension of Zed registers an
older `kdl` grammar, which reads KDL 1 only, under the same name: a new name keeps both.

```sh
npm install
npm run generate
```

MIT, from tree-sitter-kdl (see `LICENSE`).

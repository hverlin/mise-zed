# tree-sitter-mise-toml

[tree-sitter-toml](https://github.com/tree-sitter/tree-sitter-toml) at
`342d9be207c2dba869b9967124c679b5e6fd0ebe` (the revision Zed's TOML extension uses), renamed
`mise_toml`, with one change: what is between the quotes of a string is a visible
`string_content` node. The `mise` language of mise-zed injects shell, Python, ... into the
`string_content` of `run` scripts, which the original grammar keeps in hidden tokens.

Regenerate `src/` after a change to `grammar.js`:

```sh
npm install
npm run generate
```

MIT, from tree-sitter-toml (see `LICENSE`).

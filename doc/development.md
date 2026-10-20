# Development

## Build and try

1. Install Rust with [rustup](https://rustup.rs/), and the WebAssembly target:
   `rustup target add wasm32-wasip2`.
2. Build: `cargo build --release --target wasm32-wasip2`.
3. In Zed, run `zed: install dev extension` and pick this folder. Zed builds
   the extension and the TOML grammar itself. After a change, run
   `zed: rebuild dev extension`.

To try a build of mise that has `mise lsp`, point the language server at it in
your settings:

```json
{
  "lsp": {
    "mise": { "binary": { "path": "/path/to/mise", "arguments": ["lsp"] } }
  }
}
```

`dev: open language server logs` shows the messages between Zed and
`mise lsp`.

## Layout

| Path | What |
|---|---|
| `extension.toml` | the manifest: language, grammar, language server, MCP servers, snippets, capabilities |
| `src/lib.rs` | the commands of the language server and of the MCP servers |
| `languages/mise/` | the `mise` language: `config.toml`, the queries, `runnables.scm` and `tasks.json` for the run buttons |
| `languages/tool-versions/` | the `Tool Versions` language |
| `tree-sitter-mise-toml/` | the grammar of the `mise` language, a fork of tree-sitter-toml |
| `tree-sitter-tool-versions/` | the grammar of `.tool-versions` files |
| `languages/usage/`, `tree-sitter-usage-kdl/` | the `Usage` language and its grammar, tree-sitter-kdl 2.0.0 renamed |
| `snippets/` | the snippets, generated |
| `scripts/sync-snippets.py` | copies the snippets of mise-vscode |

## Snippets

The snippets come from the `src/snippets` folder of
[mise-vscode](https://github.com/hverlin/mise-vscode). To update them:

```sh
python3 scripts/sync-snippets.py ../mise-vscode
```

Zed's snippet parser takes every `$` for a tab stop, so the script escapes the
others.

## Grammars

The three grammars live in this repository. `tree-sitter-usage-kdl` is
[tree-sitter-kdl](https://github.com/tree-sitter-grammars/tree-sitter-kdl)
2.0.0, which reads KDL 2, renamed `usage_kdl` so that it does not replace the
`kdl` grammar of the KDL extension. `tree-sitter-mise-toml` is
[tree-sitter-toml](https://github.com/tree-sitter/tree-sitter-toml) at the
revision the TOML extension uses, renamed `mise_toml`, with the content of
strings as a `string_content` node, which `languages/mise/injections.scm`
highlights in the language of the script.

After a change to a `grammar.js`, regenerate its parser and run its tests:

```sh
cd tree-sitter-tool-versions   # or tree-sitter-mise-toml
npm install
npm run generate
npm test
```

Zed builds a grammar from the commit that `extension.toml` names in its
`rev`, so a change to a grammar takes two commits: push the grammar, then set
`rev` to that commit.

To try a grammar before pushing it, build it the way Zed does, with the
`clang` of [wasi-sdk](https://github.com/WebAssembly/wasi-sdk), into
`grammars/<name>.wasm`:

```sh
$WASI_SDK/bin/clang -fPIC -shared -Os -Wl,--export=tree_sitter_tool_versions \
  -o grammars/tool_versions.wasm -I tree-sitter-tool-versions/src \
  tree-sitter-tool-versions/src/parser.c
```

## Screenshot

`screenshots/mise-zed.webp` shows `screenshots/demo/mise.toml`, which `mise lsp`
reports one problem in, on purpose. Open that folder in Zed with the extension
and a mise that has `mise lsp`, to take it again.

## Licenses

The extension is MIT. The queries of `languages/mise`, except `runnables.scm`
and the mise part of `injections.scm`, come from the
[TOML extension](https://github.com/zed-extensions/toml), under the Apache
License 2.0 (`LICENSE-APACHE`). `tree-sitter-mise-toml` and
`tree-sitter-usage-kdl` are forks of tree-sitter-toml and tree-sitter-kdl, MIT
(their `LICENSE`).

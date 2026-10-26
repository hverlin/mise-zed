# Development

## Build and try

1. Install Rust with [rustup](https://rustup.rs/) or with mise
   (`mise use -g rust`), and the WebAssembly target:
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
| `extension.toml` | the manifest: languages, grammars, language server, snippets, capabilities |
| `src/lib.rs` | the command of the language server |
| `languages/mise/` | the `mise` language: `config.toml`, the queries, `runnables.scm` and `tasks.json` for the run buttons, `semantic_token_rules.json` |
| `languages/mise-shell/`, `tree-sitter-mise-bash/` | the `mise Shell` language of the scripts of tasks, without Shell Script's run button, and its grammar, tree-sitter-bash 0.25.1 renamed |
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

The four grammars live in this repository. `tree-sitter-mise-bash` is
[tree-sitter-bash](https://github.com/tree-sitter/tree-sitter-bash) 0.25.1,
renamed `mise_bash`, so that the `mise Shell` language has a grammar of its
own. `tree-sitter-usage-kdl` is
[tree-sitter-kdl](https://github.com/tree-sitter-grammars/tree-sitter-kdl)
2.0.0, which reads KDL 2, renamed `usage_kdl` so that it does not replace the
`kdl` grammar of the KDL extension. `tree-sitter-mise-toml` is
[tree-sitter-toml](https://github.com/tree-sitter/tree-sitter-toml) at the
revision the TOML extension uses, renamed `mise_toml`, with the content of
strings as a `string_content` node, which `languages/mise/injections.scm`
highlights in the language of the script.

After a change to a `grammar.js`, regenerate its parser and run its tests:

```sh
cd tree-sitter-tool-versions   # or tree-sitter-mise-toml, ...
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
reports one problem in, on purpose, with `c` typed under `[settings]` for the
completion, the outline panel and the diagnostics panel. Open a copy of that
folder outside git, so that the title has no branch and lines no blame, in Zed
with the extension and a mise that has `mise lsp`, to take it again.

`screenshots/mise-zed-tasks.webp` shows the same file, without its `[settings]`,
with the references of `check-skip-tests` and its hover in the `depends` of
`build`, underlined as a link to its definition.

`screenshots/mise-zed-run-task.webp` shows a `[tasks.hello]` run from its run
button, in Zed's terminal.

## Licenses

The extension is MIT. The queries of `languages/mise`, except `runnables.scm`
and the mise parts of `highlights.scm` and `injections.scm`, come from the
[TOML extension](https://github.com/zed-extensions/toml), under the Apache
License 2.0 (`LICENSE-APACHE`). `tree-sitter-mise-toml`,
`tree-sitter-mise-bash` and `tree-sitter-usage-kdl` are forks of
tree-sitter-toml, tree-sitter-bash and tree-sitter-kdl, MIT (their `LICENSE`).
`languages/mise-shell/highlights.scm` comes from tree-sitter-bash 0.25.1 (MIT).

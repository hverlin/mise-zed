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

## Licenses

The extension is MIT. The queries of `languages/mise`, except `runnables.scm`,
come from the [TOML extension](https://github.com/zed-extensions/toml), under
the Apache License 2.0 (`LICENSE-APACHE`). The `[grammars.toml]` entry uses the
same repository and revision as that extension.

# mise for Zed (prototype)

Runs [`mise lsp`](https://github.com/jdx/mise/pull/14744) for mise config files: diagnostics,
completion and hover of the keys, and formatting as `mise fmt` does. Requires the TOML extension
and a mise with `mise lsp`.

Install with `zed: install dev extension`, then pick this folder.

The extension runs the `mise` of the project's PATH, after checking that it has `mise lsp`: an
older mise takes `lsp` for a task name and would run the project's `lsp` task.

To use another mise binary, set both the path and the arguments. Zed then runs that command
instead of asking the extension, so the check above is skipped:

```json
{
  "lsp": {
    "mise": {
      "binary": { "path": "/path/to/mise", "arguments": ["lsp"] }
    }
  }
}
```

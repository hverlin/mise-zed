# mise for Zed (prototype)

- **Language server:** [`mise lsp`](https://github.com/jdx/mise/pull/14744) for the mise config
  files: diagnostics (TOML syntax, unknown keys, wrong types, deprecated keys), completion and
  hover of the keys, and formatting as `mise fmt` does. It is attached to TOML and to the `mise`
  language below, and answers only for mise config files.
- **A `mise` language** for `mise.toml`, `.mise.toml`, `mise.local.toml`, common
  `mise.<env>.toml`, `.config/mise.toml`, `.config/mise/config.toml` and `.mise/config.toml`. It
  highlights them as TOML and adds:
  - run buttons on tasks (`mise run <task>`) and tools (`mise install <tool>`,
    `mise upgrade <tool>`),
  - tasks in `task: spawn`: `mise run`, `mise install`, `mise upgrade --interactive`,
    `mise outdated`, `mise trust`, `mise tasks validate`, `mise doctor`, in the file's directory,
  - snippets: `mise-task`, `mise-task-script`, `mise-task-usage`, `mise-daemon`, ...
- **Snippets for file tasks** in shell scripts: `mise-task`, `mise-task-usage`.
- **MCP servers** for the agent: `mise mcp` (mise 2026.6.6+) and `pitchfork mcp` (pitchfork 2.25+,
  `--local` from 2.32), started in the project directory.

Install with `zed: install dev extension`, then pick this folder. The `mise` language uses the
grammar of the TOML extension (same repository and revision), which Zed builds on install.

## The binaries

The language server runs the `mise` of the project's PATH, after checking that `mise lsp --help`
shows its usage: an older mise takes `lsp` for a task name and would run the project's `lsp` task.
The MCP servers run the `mise` and `pitchfork` of Zed's PATH.

To use another binary, set both the path and the arguments. Zed then runs that command instead of
asking the extension, so the checks above are skipped:

```json
{
  "lsp": {
    "mise": { "binary": { "path": "/path/to/mise", "arguments": ["lsp"] } }
  },
  "context_servers": {
    "mise": { "command": { "path": "/path/to/mise", "arguments": ["mcp"] } },
    "pitchfork": { "enabled": false }
  }
}
```

## Which files are `mise`

A Zed language matches files by suffix, without globs, so `mise/config.toml`, `conf.d/*.toml` and
other environments stay TOML (they still get `mise lsp`). Map them with `file_types`:

```json
{
  "file_types": {
    "mise": ["**/mise/config.toml", "**/mise/conf.d/*.toml", "**/mise.*.toml"]
  }
}
```

TOML extensions such as Tombi register for TOML only, so they leave `mise` files to `mise lsp`:
no doubled diagnostics, completions or hovers. To keep a file as TOML instead:
`"file_types": { "TOML": ["mise.toml"] }`.

## Not there

- Scripts in `run = '''...'''` are not highlighted as shell: Zed injects the whole TOML string,
  quotes included.
- Tool and version completion, task names in `depends`, Tera, go to definition: they need
  `mise lsp` to provide them.
- The views of the VS Code extension (tools, tasks, environment, daemons): Zed extensions have
  no panels.

Snippets come from mise-vscode: `python3 scripts/sync-snippets.py <mise-vscode checkout>`. The
TOML queries in `languages/mise` come from the TOML extension (Apache-2.0).

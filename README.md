# mise for Zed (prototype)

- **Language server:** [`mise lsp`](https://github.com/jdx/mise/pull/14744) for the mise config
  files: diagnostics (TOML syntax, unknown keys, wrong types, deprecated keys), completion and
  hover of the keys, and formatting as `mise fmt` does. It needs a mise with `mise lsp`, which is
  not released yet; until then the server does not start and says why.
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

The language server runs the `mise` of the project's PATH, after checking with `mise help lsp`
that it has `mise lsp`: an older mise takes `lsp` for a task name and would run the project's
`lsp` task.

The MCP servers run the `mise` and `pitchfork` of Zed's PATH, found with `sh` (macOS and Linux;
on Windows, set their command). They start in every project, trusted or not. Turn one off with
`"enabled": false`.

A setting that gives a command replaces the extension's: Zed runs it as is, without the checks
above.

```json
{
  "lsp": {
    "mise": { "binary": { "path": "/path/to/mise", "arguments": ["lsp"] } }
  },
  "context_servers": {
    "mise": { "command": "/path/to/mise", "args": ["mcp"] },
    "pitchfork": { "enabled": false }
  }
}
```

## Which files are `mise`

A Zed language matches files by suffix, without globs, so `mise/config.toml`, `conf.d/*.toml` and
other environments stay TOML. Map them with `file_types`:

```json
{
  "file_types": {
    "mise": [
      "**/mise.*.toml",
      "**/.mise.*.toml",
      "**/mise/config.toml",
      "**/mise/config.*.toml",
      "**/mise/conf.d/*.toml"
    ]
  }
}
```

`mise lsp` is also registered for TOML, but off by default, since it would start in every Rust or
Python project. Turn it on with `"languages": { "TOML": { "language_servers": ["mise", "..."] } }`.

A file of the `mise` language is no longer TOML for Zed: `languages.TOML` settings and TOML
language servers such as Tombi leave it to `mise lsp`. That removes their doubled diagnostics,
completions and hovers, but also Tombi's schema completion, which `mise lsp` covers only once it
is released. To keep a file as TOML: `"file_types": { "TOML": ["mise.toml"] }`.

## Not there

- Run buttons: a task or tool whose key has `$`, a backtick, `\` or `!` gets none (Zed hands the
  command line to the shell unquoted). Tasks written as `build.run = ...` or
  `tasks = { ... }` get none either, and the tasks of `mise.<env>.toml` run without `MISE_ENV`.
- Scripts in `run = '''...'''` are not highlighted as shell: Zed injects the whole TOML string,
  quotes included.
- Tool and version completion, task names in `depends`, Tera, go to definition: they need
  `mise lsp` to provide them.
- The views of the VS Code extension (tools, tasks, environment, daemons): Zed extensions have
  no panels.

Snippets come from mise-vscode: `python3 scripts/sync-snippets.py <mise-vscode checkout>`.

MIT license. The TOML queries in `languages/mise` (all `.scm` files but `runnables.scm`) come
from the [TOML extension](https://github.com/zed-extensions/toml), under the Apache License 2.0
(`LICENSE-APACHE`).

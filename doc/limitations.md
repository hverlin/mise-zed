# Limitations

What the [VS Code extension](https://github.com/hverlin/mise-vscode) does that
this one cannot yet.

## Waiting for `mise lsp`

These need the language server to provide them, as follow-ups of
[jdx/mise#14744](https://github.com/jdx/mise/pull/14744):

- Completion of tool names and versions
- Completion of task names in `depends`, go to definition and find references
  for tasks
- Completion of [tera templates](https://mise.jdx.dev/templates.html) and of
  [task arguments](https://mise.jdx.dev/tasks/task-arguments.html) (`usage`)
- Inline tool versions and outdated markers
- The `mise tasks validate` diagnostics

## Not possible in a Zed extension

- The tools, tasks, environment variables and daemons views: Zed extensions
  have no panels.
- Configuring other extensions to use the tools of mise: Zed loads the
  environment of your shell, so tools that `mise activate` puts on the PATH are
  already found.
- Shell highlighting of the scripts in `run = '''...'''`: Zed would highlight
  the TOML quotes along with the script.
- `.tool-versions` files: there is no grammar for them.

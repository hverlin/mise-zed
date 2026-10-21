# Limitations

What the [VS Code extension](https://github.com/hverlin/mise-vscode) does that
this one cannot yet.

## Waiting for `mise lsp`

These need the language server to provide them.

- Completion of tool versions
- File tasks (`mise-tasks/`, `.mise/tasks/`): no completion, hover or go to
  definition for them
- Find references and rename for tasks cover the open file only
- Unsaved changes to other configs and to `[task_config] includes` files:
  `mise lsp` reads those from disk
- Completion of [tera templates](https://mise.jdx.dev/templates.html) and of
  [task arguments](https://mise.jdx.dev/tasks/task-arguments.html) (`usage`)
- Inline tool versions and outdated markers
- The other `mise tasks validate` checks, such as alias conflicts, empty
  scripts or usage specs

## Not possible in a Zed extension

- The tools, tasks, environment variables and daemons views: Zed extensions
  have no panels.
- Configuring other extensions to use the tools of mise: Zed loads the
  environment of your shell, so tools that `mise activate` puts on the PATH are
  already found.
- Completion inside the scripts of tasks: Zed highlights them in their
  language, but runs no language server on a part of a file.
- In a Python or JavaScript script of a task, Zed's own run button can show up
  on an `if __name__ == "__main__":` block or a test; it would run the whole
  config file. Shell scripts use a `mise Shell` language without one.

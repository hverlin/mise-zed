# Snippets

Type a prefix in a `mise` file, then pick the snippet in the completion menu.

## `mise` files

| Prefix | Inserts |
|---|---|
| `mise-task` | a task with common options |
| `mise-task-array` | a task with several commands |
| `mise-task-script` | a task with a multiline script |
| `mise-task-usage` | a task with arguments defined with the [usage spec](https://mise.jdx.dev/tasks/task-arguments.html) |
| `mise-task-template` | a reusable [task template](https://mise.jdx.dev/tasks/templates.html) that tasks can extend |
| `mise-task-node` | a task run with node |
| `mise-task-python` | a task run with python |
| `mise-task-uv` | a task run with uv |
| `mise-task-deno` | a task run with deno |
| `mise-task-bun` | a task run with bun |
| `mise-task-ruby` | a task run with ruby |
| `mise-task-full` | a task with every option |
| `mise-daemon` | [experimental] a daemon running a command, ready once its port accepts connections |
| `mise-daemon-preset` | [experimental] a database or a message broker from a mise service preset |
| `mise-daemon-task` | [experimental] a daemon running a mise task |
| `mise-daemon-auto` | [experimental] a daemon that starts when entering the project, and stops once the last shell left |
| `mise-daemon-groups` | [experimental] named sets of daemons |

## File tasks (shell scripts)

| Prefix | Inserts |
|---|---|
| `mise-task` | a `#MISE` configuration block |
| `mise-task-usage` | arguments defined with the usage spec |

These snippets come from [mise-vscode](https://github.com/hverlin/mise-vscode);
see [Development](development.md#snippets) to update them.

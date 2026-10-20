# Tasks and tools

## Tasks

A ▶ button in the gutter of `mise` files runs a
[mise task](https://mise.jdx.dev/tasks/) with `mise run <task>`, in the
directory of the file. It appears on:

```toml
[tasks.build]          # a task table
run = "cargo build"

[tasks."docs:build"]   # a quoted name
run = "astro build"

[tasks]
lint = "eslint ."      # a task in [tasks]
```

and on a dotted key outside of any table, such as `tasks.test = "cargo test"`.

`task: spawn` also lists these tasks while a `mise` file is open, in its
directory:

| Task | Runs |
|---|---|
| `mise run` | the interactive task picker |
| `mise install` | installs the missing tools |
| `mise upgrade --interactive` | picks tools to upgrade |
| `mise outdated` | lists the tools with a newer version |
| `mise trust <file>` | trusts the open config file |
| `mise tasks validate` | checks the task definitions |
| `mise doctor` | checks the mise setup |

## Tools

A ▶ button on a [mise tool](https://mise.jdx.dev/dev-tools/) offers
`mise install <tool>` and `mise upgrade <tool>`:

```toml
[tools]
node = "24"
"npm:prettier" = "latest"

[tools.python]
version = "3.13"
```

`task: spawn` also offers `mise install`, `mise upgrade --interactive` and
`mise outdated` while a `.tool-versions` file is open.

## No button

- A task or tool whose name has `$`, a backtick, `\` or `!`: Zed hands the
  command to the shell as written, and the shell would expand those.
- Tasks written as `build.run = "..."` in `[tasks]`, or as an inline
  `tasks = { ... }`.

The tasks of a `mise.<env>.toml` run without `MISE_ENV` set.

# mise.toml language support

The extension adds a `mise` language for the mise config files and runs
[`mise lsp`](https://github.com/jdx/mise/pull/14744), the language server built
into mise, for them.

## Which files are `mise` files

A Zed language matches files by suffix, so the `mise` language covers:

- `mise.toml`, `.mise.toml`, `mise.local.toml`, `.mise.local.toml`
- `mise.<env>.toml` for the common environments: `dev`, `development`, `test`,
  `ci`, `staging`, `prod`, `production` (and `mise.dev.local.toml`)
- `.config/mise.toml`, `.config/mise/config.toml`,
  `.config/mise/config.local.toml`, `.mise/config.toml`

Other config files, such as `mise/config.toml`, `conf.d/*.toml` or other
environments, stay TOML. To open them as `mise` files, add them to `file_types`:

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

To keep a file as TOML instead:

```json
{
  "file_types": { "TOML": ["mise.toml"] }
}
```

## What a `mise` file gets

- **Highlighting, outline and brackets**, from the
  [TOML extension](https://github.com/zed-extensions/toml)'s queries.
- **Highlighted task scripts**: see [below](#scripts-of-tasks).
- **Diagnostics** from `mise lsp`: TOML syntax errors, unknown keys with a
  suggestion, values of the wrong type, and deprecated keys. Unknown
  `[settings]` keys are warnings, because mise still loads the file; other
  unknown keys are errors. While a file does not parse, the diagnostics of the
  parts you did not edit stay.
- **A quick fix** that replaces an unknown key with the one mise suggests.
- **Completion** of keys, inside inline tables too, and of values with a fixed
  set, such as booleans and enums.
- **Hover** documentation for keys.
- **Formatting**, as `mise fmt` does.
- [Run buttons](tasks-and-tools.md) on tasks and tools, and
  [snippets](snippets.md).

Files that `[task_config] includes` lists are tasks, not configs: `mise lsp`
leaves them alone, even when they are named like `mise.tasks.toml`.

## Scripts of tasks

The script of a task is highlighted in its language: as shell by default, or as
the language its shebang names. That covers `run = "..."`, `run = '''...'''`,
each string of `run = [...]`, and the dotted `tasks.build.run = "..."`.

```toml
[tasks.build]
run = "cargo build --release"   # shell

[tasks.report]
run = '''
#!/usr/bin/env python3
print("report")
'''                             # Python; node, bun and deno give JavaScript, ruby gives Ruby
```

A `usage = '''...'''` [usage](https://usage.jdx.dev/) spec is highlighted as the
`Usage` language, which also opens `*.usage.kdl` files: KDL 2, with usage's
directives (`arg`, `flag`, `cmd`, `complete`, ...) as keywords. Ruby scripts
need the [Ruby extension](https://zed.dev/extensions/ruby).

Zed only highlights these scripts: completion and diagnostics inside them would
need a language server that sees the script on its own.

## `.tool-versions` files

`.tool-versions` files open as the `Tool Versions` language: tool names,
versions and comments are highlighted, `latest`, `system` and `lts*` as
constants, and `ref:`, `path:`, `prefix:` and `sub-*:` versions apart. The outline
lists the tools. `mise lsp` does not serve these files.

## Next to Tombi or another TOML extension

TOML language servers, such as [Tombi](https://zed.dev/extensions/tombi),
register for the TOML language only. They leave `mise` files to `mise lsp`, so
diagnostics, completions and hovers are not doubled. `languages.TOML` settings
do not apply to `mise` files either.

## The `mise` binary

The language server runs the `mise` of the project's PATH, after checking with
`mise help lsp` that it has `mise lsp`. A mise that predates the command takes
`lsp` for a task name, and would run the project's `lsp` task. Until a released
mise has `mise lsp`, the server does not start and says why. See
[Configuration](configuration.md#the-mise-binary) to use another binary.

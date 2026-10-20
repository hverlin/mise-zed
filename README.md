# mise-zed 🧑‍🍳

[Zed](https://zed.dev/) extension for [mise](https://mise.jdx.dev/) (`mise-en-place`) and [pitchfork](https://pitchfork.jdx.dev/). [Documentation](doc/)

> [mise](https://mise.jdx.dev/) is a polyglot tool version manager, environment
> variables manager, and tasks runner.
>
> - Like asdf (or nvm or pyenv but for any language), it manages dev tools like
>   node, python, cmake, terraform, and hundreds more.
> - Like direnv, it manages environment variables for different project
>   directories.
> - Like make, it manages tasks used to build and test projects.
>
> [pitchfork](https://pitchfork.jdx.dev/) runs the daemons of a project, like a
> database or a development server, which
> [mise can declare too](https://mise.jdx.dev/daemons.html).

This Zed extension brings `mise` to your editor: diagnostics, completion and
quick fixes in `mise.toml`, run buttons for
[tasks](doc/tasks-and-tools.md) and [tools](doc/tasks-and-tools.md#tools),
[snippets](doc/snippets.md), and the `mise` and `pitchfork`
[MCP servers](doc/ai-agents.md) for the agent.

> [!NOTE]
> This extension is a prototype. The language server it runs,
> [`mise lsp`](https://github.com/jdx/mise/pull/14744), is not in a released
> version of mise yet.

## Installation

The extension is not in the Zed extension registry yet. To install it from
source:

1. Install Rust with [rustup](https://rustup.rs/), which Zed uses to build the
   extension.
2. Clone this repository.
3. In Zed, run `zed: install dev extension` from the command palette and pick
   the cloned folder.

Zed 1.24 or later is needed. The language server needs a mise with
`mise lsp`; the rest works with any recent mise.

## Important Defaults

The extension comes with defaults that you might want to change. See
[Configuration](doc/configuration.md) for every setting.

### `mise.toml` is a `mise` file, not a TOML file

`mise.toml`, `.mise.toml`, `mise.local.toml`, `.config/mise/config.toml` and the
other common config files open as the `mise` language, highlighted as TOML.
TOML extensions such as [Tombi](https://zed.dev/extensions/tombi) leave them to
`mise lsp`, so diagnostics, completions and hovers are not doubled. See
[which files are `mise` files](doc/language-support.md#which-files-are-mise-files)
to map more files, or to keep one as TOML.

### `mise lsp` on other TOML files is opt-in

`mise lsp` is also registered for TOML, for config files such as
`mise/config.toml` or `conf.d/*.toml`, but it is off by default: it would
start in every Rust or Python project. See
[Configuration](doc/configuration.md#mise-lsp-on-toml-files) to turn it on.

### The MCP servers run in every project

The `mise` and `pitchfork` MCP servers start in every project, trusted or not.
See [AI agents](doc/ai-agents.md#turning-a-server-off) to turn one off.

## Features

### [mise.toml Language Support](doc/language-support.md)

- 📝 Syntax highlighting, outline and brackets for `mise.toml` files
- 🩺 Diagnostics: TOML syntax, unknown keys with a suggestion, wrong types, and
  deprecated keys
- 🪄 Quick fix to replace an unknown key with the one mise suggests
- 📚 Completion of keys (inline tables included) and of values with a fixed
  set, such as booleans and enums
- 💬 Hover documentation for keys
- 🧹 Formatting, as `mise fmt` does

### [Tasks](doc/tasks-and-tools.md)

- ▶️ Run buttons on [mise tasks](https://mise.jdx.dev/tasks/) in `mise.toml`
- ⚡ `mise run`, `mise install`, `mise doctor` and more in `task: spawn`

### [Tools](doc/tasks-and-tools.md#tools)

- 📦 Run buttons on [mise tools](https://mise.jdx.dev/dev-tools/) to install or
  upgrade them

### [AI agents (MCP)](doc/ai-agents.md)

- 🤖 The `mise mcp` and `pitchfork mcp` servers, for the agent of Zed, in the
  project directory

### [Snippets](doc/snippets.md)

- 📝 `mise-task` snippets to create tasks in `mise.toml` and file tasks, and
  `mise-daemon` snippets to declare daemons in `mise.toml`

## [Documentation](doc/)

- [mise.toml language support](doc/language-support.md)
- [Tasks and tools](doc/tasks-and-tools.md)
- [AI agents (MCP)](doc/ai-agents.md)
- [Snippets](doc/snippets.md)
- [Configuration](doc/configuration.md)
- [Limitations](doc/limitations.md): what the
  [VS Code extension](https://github.com/hverlin/mise-vscode) does that this one
  cannot yet
- [Development](doc/development.md)

## Bug Reports / Feature Requests / Contributing

- Found a bug? Please
  [open an issue](https://github.com/hverlin/mise-zed/issues)
- Contributions are welcome! See [Development](doc/development.md) to build and
  try the extension.

## Ecosystem

- [mise-vscode](https://github.com/hverlin/mise-vscode), the VS Code extension
  this one follows
- See [intellij-mise](https://github.com/134130/intellij-mise) if you are
  looking for a similar plugin for IntelliJ IDEA
- [Mise documentation](https://mise.jdx.dev/)

## License

This extension is licensed under the MIT License. See the [LICENSE](LICENSE)
file for details. The TOML queries in `languages/mise` come from the
[TOML extension](https://github.com/zed-extensions/toml), under the Apache
License 2.0 ([LICENSE-APACHE](LICENSE-APACHE)).

# Configuration

The extension has no settings of its own: it uses Zed's settings for languages
and language servers.

## The `mise` binary

The language server runs the `mise` of the project's PATH, after checking that
it has `mise lsp`. To use another binary, give both its path and its
arguments; Zed then runs that command instead of asking the extension, without
the check:

```json
{
  "lsp": {
    "mise": {
      "binary": { "path": "/path/to/mise", "arguments": ["lsp"] }
    }
  }
}
```

Without `arguments`, Zed starts the binary with none, and mise prints its help
and exits.

## `mise lsp` on TOML files

`mise lsp` serves the `mise` language. It is also registered for TOML, for
config files that the `mise` language does not match, such as
`mise/config.toml` or `conf.d/*.toml`, but it is off there by default: it
would start in every Rust or Python project. To turn it on:

```json
{
  "languages": {
    "TOML": { "language_servers": ["mise", "..."] }
  }
}
```

`mise lsp` answers only for mise config files: `Cargo.toml` and
`pyproject.toml` get nothing from it.

## Which files are `mise` files

See [mise.toml language support](language-support.md#which-files-are-mise-files)
for `file_types`.

## MCP servers

The extension provides none. See [AI agents](ai-agents.md) to add `mise mcp`.

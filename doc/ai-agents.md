# AI agents (MCP)

[`mise mcp`](https://mise.jdx.dev/mcp.html) gives an AI agent the tools,
tasks, environment and config of a project. The extension does not provide it:
add it to the agent of Zed in your settings, or from
**Settings → AI → MCP Servers** with `Add Local Server`.

```json
{
  "context_servers": {
    "mise": { "command": "mise", "args": ["mcp"] }
  }
}
```

Zed starts the server in the folder of the active project and restarts it when
you switch, so it needs no `--cd`. If Zed does not find `mise` on its PATH,
give its absolute path as `command`.

In your user settings, the server starts in every project you open. To keep
it to one project, put it in that project's `.zed/settings.json` instead. See
the [mise documentation](https://mise.jdx.dev/mcp.html#access-and-execution)
for what the agent can read and run.

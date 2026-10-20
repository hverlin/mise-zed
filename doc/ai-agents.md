# AI agents (MCP)

The extension provides two [MCP](https://modelcontextprotocol.io/) servers to the
agent of Zed, started in the project directory:

| Server | Command | Gives the agent |
|---|---|---|
| `mise` | `mise mcp` (mise 2026.6.6 or later) | the tools, tasks, environment and config of the project |
| `pitchfork` | `pitchfork mcp` (pitchfork 2.25 or later; `--local` from 2.32) | the daemons of the project |

See them in the agent panel's settings.

## The binaries

The servers run the `mise` and `pitchfork` of Zed's PATH, found with `sh`
(macOS and Linux). On Windows, or to use another binary, give the command:

```json
{
  "context_servers": {
    "mise": { "command": "/path/to/mise", "args": ["mcp"] }
  }
}
```

Zed then runs that command as is, without the version check.

## Turning a server off

The servers start in every project, trusted or not. To turn one off:

```json
{
  "context_servers": {
    "pitchfork": { "enabled": false }
  }
}
```

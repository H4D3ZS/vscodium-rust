# MCP Store — Antigravity IDE Reference

This document maps **Google Antigravity IDE** MCP Store behavior to VSCodium-Rust. Antigravity is the product reference; our implementation follows the same UX and config model without bundling external servers into the IDE binary.

## Antigravity reference (product behavior)

Sources: [Google Cloud — Connect Antigravity to Data Cloud](https://cloud.google.com/blog/products/data-analytics/connect-google-antigravity-ide-to-googles-data-cloud-services), [BigQuery MCP guide](https://cloud.google.com/bigquery/docs/pre-built-tools-with-mcp-toolbox), [getmcp Antigravity guide](https://www.getmcp.es/guides/antigravity).

| Antigravity | VSCodium-Rust |
|-------------|---------------|
| Agent panel **"..."** menu → **MCP Servers** | Agent panel **"..."** → MCP Servers (`AgentMcpMenu.tsx`) |
| **MCP Store** editor pane (browse + Install) | **MCP Store** editor tab (`McpStorePanel.tsx`) |
| **Manage MCP Servers** link (top-right of store) | Same — toggles manage view |
| **View raw config** | Inline JSON editor in MCP Store (`read_mcp_config` / `write_mcp_config`) |
| Config: `~/.gemini/antigravity/mcp_config.json` | Config: app data `mcp_servers.json` (same `mcpServers` root key) |
| Install writes stdio/http entry; server spawns externally | Same via `mcp_registry.rs` |
| Toggle server off/on in Manage; refresh tool list | Manage view: on/off + **Refresh** |
| Tools available to agent after connect | `tool_invoker.rs` + `list_mcp_tools` |

### Antigravity navigation flow

```
Agent panel → "..." → MCP Servers
    → MCP Store (search, Install per row)
    → Manage MCP Servers
        → enable/disable
        → View raw config (JSON)
        → Refresh (re-read tools)
```

### Config format parity

Antigravity uses the canonical Claude Desktop shape:

```json
{
  "mcpServers": {
    "sequential-thinking": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-sequential-thinking"]
    },
    "bigquery": {
      "command": "npx",
      "args": ["-y", "@toolbox-sdk/server", "--prebuilt", "bigquery", "--stdio"],
      "env": { "BIGQUERY_PROJECT": "my-project" }
    }
  }
}
```

VSCodium-Rust persists the same structure. HTTP/SSE servers use `type: "http"` and `serverUrl` (Antigravity also expects `serverUrl`, not `url`).

### Antigravity catalog pattern

Antigravity ships a **curated MCP Store** (BigQuery, Spanner, Firebase, GitLab Orbit, etc.) — not every npm package. One-click **Install** only registers config; the IDE spawns `npx` / `python` as a child process.

Our catalog (`src/mcp/mcpCatalog.ts`) mirrors that pattern:

- Official `@modelcontextprotocol/server-*` packages
- Google Cloud `@toolbox-sdk/server --prebuilt` entries (same as Antigravity docs)
- Optional community servers (Context7, HexStrike) with explicit setup notes

### What we intentionally do not copy

- Antigravity's OAuth-gated Google catalog install wizard (we use env-field modal instead)
- Bundling 150+ pentest CLIs (HexStrike stays optional store entry)
- Private workbench bundle (`workbench.input.antigravityConfigurePluginsPageInput`) — we use a first-class React editor tab

## Open MCP Store in this IDE

| Entry | Action |
|-------|--------|
| Agent **"..."** → MCP Servers | Store |
| Agent **"..."** → Manage MCP Servers | Manage |
| Command palette | `View: Open MCP Store` |
| Welcome screen | MCP Store… |
| Settings → Agent → MCP | Open MCP Store |

## After install

1. Server entry saved to `mcp_servers.json`
2. Rust registry spawns the process (if enabled)
3. Agent sees MCP tools on the next turn
4. If tools missing: **Manage → Refresh**, or toggle server off/on (Antigravity parity)

// McpAuthTool: pseudo-tool surfaced for MCP servers that require OAuth.
//
// Tool name: "mcp__auth"
//
// When called by the LLM (or user) with a `server_name`, this tool:
//  1. Checks whether the server is already connected (no auth needed).
//  2. If the server is an HTTP/SSE server, calls `McpManager::initiate_auth()`
//     to fetch `/.well-known/oauth-authorization-server` metadata, build the
//     PKCE authorization URL, and return it so the user can open it.
//  3. Attempts to open the URL in the system browser (best-effort).
//  4. For stdio servers, explains env-var based authentication.
//
// This mirrors the TypeScript `mcp__<name>__authenticate` dynamic tool.

use crate::{PermissionLevel, Tool, ToolContext, ToolResult};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

pub struct McpAuthTool;

#[derive(Debug, Deserialize)]
struct McpAuthInput {
    server_name: String,
}

#[async_trait]
impl Tool for McpAuthTool {
    fn name(&self) -> &str {
        "mcp__auth"
    }

    fn description(&self) -> &str {
        "Start the OAuth 2.0 + PKCE authorization flow for an MCP server that \
         requires authentication. Returns the authorization URL that the user \
         should open in their browser. For stdio servers that use environment \
         variables for auth, returns setup instructions instead."
    }

    fn permission_level(&self) -> PermissionLevel {
        PermissionLevel::None
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "server_name": {
                    "type": "string",
                    "description": "The MCP server name that needs authentication."
                }
            },
            "required": ["server_name"]
        })
    }

    async fn execute(&self, input: Value, ctx: &ToolContext) -> ToolResult {
        let params: McpAuthInput = match serde_json::from_value(input) {
            Ok(p) => p,
            Err(e) => return ToolResult::error(format!("Invalid input: {}", e)),
        };

        let manager = match &ctx.mcp_manager {
            Some(m) => m,
            None => {
                return ToolResult::error(
                    "No MCP manager configured. Cannot authenticate MCP servers.".to_string(),
                )
            }
        };

        use claurst_mcp::McpServerStatus;

        // 1. Check current connection status.
        match manager.server_status(&params.server_name) {
            McpServerStatus::Connected { tool_count } => {
                return ToolResult::success(format!(
                    "MCP server \"{}\" is already connected ({} tool(s) available). \
                     No authentication needed.",
                    params.server_name, tool_count
                ));
            }
            McpServerStatus::Connecting => {
                return ToolResult::success(format!(
                    "MCP server \"{}\" is currently connecting. Try again in a moment.",
                    params.server_name
                ));
            }
            McpServerStatus::Failed { error, .. } => {
                // Fall through to attempt auth; also report the failure.
                tracing::debug!(
                    server = %params.server_name,
                    error = %error,
                    "McpAuthTool: server failed; attempting to initiate auth"
                );
            }
            McpServerStatus::Disconnected { .. } => {
                // Fall through to attempt auth.
            }
        }

        // 2. Use McpManager::initiate_auth() to build the PKCE auth URL.
        match manager.initiate_auth(&params.server_name).await {
            Ok(auth_url) => {
                // 3. Best-effort browser open.
                let _ = open::that(&auth_url);

                ToolResult::success(
                    json!({
                        "status": "auth_required",
                        "auth_url": auth_url,
                        "message": format!(
                            "Opening browser for OAuth authentication of \"{}\".\n\
                             If the browser did not open, visit:\n\n  {}\n\n\
                             After authorizing, run /mcp connect {} to reconnect.",
                            params.server_name, auth_url, params.server_name
                        )
                    })
                    .to_string(),
                )
            }
            Err(e) => {
                // initiate_auth() failed (e.g. no URL configured, network error).
                // Return a descriptive error so the LLM can guide the user.
                ToolResult::error(format!(
                    "Could not initiate OAuth for \"{}\": {}\n\n\
                     This may mean the server is a stdio server (uses env-var auth) \
                     or its URL is not configured. Run /mcp auth {} in the Claude \
                     interface for detailed instructions.",
                    params.server_name, e, params.server_name
                ))
            }
        }
    }
}

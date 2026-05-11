use crate::mcp_client::McpClient;
use anyhow::Result;
use tracing::instrument;

use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum McpServerConfig {
    Stdio {
        command: String,
        #[serde(default)]
        args: Vec<String>,
        #[serde(default)]
        env: std::collections::HashMap<String, String>,
        /// Optional working directory. Matches Cursor's mcp.json field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cwd: Option<String>,
        /// When false, the server stays in `mcp.json` but is not spawned.
        /// Lets users keep configured servers around without paying their
        /// startup cost or exposing their tools to the agent.
        #[serde(default = "default_enabled")]
        enabled: bool,
    },
    Http {
        #[serde(rename = "type")]
        server_type: String, // "http"
        #[serde(rename = "serverUrl")]
        server_url: String,
        #[serde(default = "default_enabled")]
        enabled: bool,
    },
}

fn default_enabled() -> bool { true }

impl McpServerConfig {
    pub fn enabled(&self) -> bool {
        match self {
            McpServerConfig::Stdio { enabled, .. } => *enabled,
            McpServerConfig::Http  { enabled, .. } => *enabled,
        }
    }

    pub fn set_enabled(&mut self, value: bool) {
        match self {
            McpServerConfig::Stdio { enabled, .. } => *enabled = value,
            McpServerConfig::Http  { enabled, .. } => *enabled = value,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct McpConfig {
    #[serde(rename = "mcpServers")]
    pub mcp_servers: std::collections::HashMap<String, McpServerConfig>,
}

pub struct McpRegistry {
    servers: Arc<RwLock<std::collections::HashMap<String, Arc<McpClient>>>>,
    config: Arc<RwLock<McpConfig>>,
    config_path: std::path::PathBuf,
}

impl McpRegistry {
    pub fn new(config_path: std::path::PathBuf) -> Self {
        let config = if config_path.exists() {
            let content = std::fs::read_to_string(&config_path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or(McpConfig {
                mcp_servers: std::collections::HashMap::new(),
            })
        } else {
            McpConfig {
                mcp_servers: std::collections::HashMap::new(),
            }
        };

        Self {
            servers: Arc::new(RwLock::new(std::collections::HashMap::new())),
            config: Arc::new(RwLock::new(config)),
            config_path,
        }
    }

    pub async fn initialize_servers(&self) -> Result<()> {
        let config = self.config.read().await.clone();
        for (name, server_config) in config.mcp_servers {
            if !server_config.enabled() {
                println!("[MCP] Skipping disabled server '{}'", name);
                continue;
            }
            if let Err(e) = self.add_server_internal(&name, server_config).await {
                eprintln!("Failed to initialize MCP server {}: {}", name, e);
            }
        }
        Ok(())
    }

    async fn add_server_internal(&self, name: &str, config: McpServerConfig) -> Result<()> {
        if !config.enabled() {
            return Ok(());
        }
        let client = match config {
            McpServerConfig::Stdio {
                command,
                args,
                env,
                cwd,
                enabled: _,
            } => {
                let args_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
                let cwd_path = cwd.as_deref().map(std::path::Path::new);
                McpClient::spawn_with_env(&command, args_refs, &env, cwd_path).await?
            }
            McpServerConfig::Http { server_url, enabled: _, .. } => McpClient::connect_http(server_url)?,
        };

        let mut servers = self.servers.write().await;
        servers.insert(name.to_string(), client);
        Ok(())
    }

    /// Toggle a server on or off. When toggling off, the running client
    /// (if any) is dropped; when toggling on, the server is spawned with
    /// its currently-persisted command/args/env.
    pub async fn set_server_enabled(&self, name: &str, enabled: bool) -> Result<()> {
        let snapshot = {
            let mut cfg = self.config.write().await;
            let entry = cfg
                .mcp_servers
                .get_mut(name)
                .ok_or_else(|| anyhow::anyhow!("Unknown MCP server: {}", name))?;
            entry.set_enabled(enabled);
            let snap = entry.clone();
            let content = serde_json::to_string_pretty(&*cfg)?;
            std::fs::write(&self.config_path, content)?;
            snap
        };

        if enabled {
            self.add_server_internal(name, snapshot).await?;
        } else {
            let mut servers = self.servers.write().await;
            servers.remove(name);
        }
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn list_tools(&self) -> Result<Vec<Value>> {
        let servers = self.servers.read().await;
        let mut all_tools = Vec::new();

        for (name, server) in servers.iter() {
            if let Ok(result) = server
                .call("list_tools", Value::Object(Default::default()))
                .await
            {
                if let Some(tools) = result.get("tools").and_then(|t| t.as_array()) {
                    // Add server name to each tool for disambiguation if needed
                    for tool in tools {
                        let mut tool_with_origin = tool.clone();
                        if let Some(obj) = tool_with_origin.as_object_mut() {
                            obj.insert("server_name".to_string(), Value::String(name.clone()));
                        }
                        all_tools.push(tool_with_origin);
                    }
                }
            }
        }

        Ok(all_tools)
    }

    #[instrument(skip(self))]
    pub async fn call_tool(&self, name: &str, arguments: Value) -> Result<Value> {
        let servers = self.servers.read().await;

        for server in servers.values() {
            let tools_result = server
                .call("list_tools", Value::Object(Default::default()))
                .await;
            if let Ok(result) = tools_result {
                if let Some(tools) = result.get("tools").and_then(|t| t.as_array()) {
                    if tools
                        .iter()
                        .any(|t| t.get("name").and_then(|n| n.as_str()) == Some(name))
                    {
                        let params = serde_json::json!({
                            "name": name,
                            "arguments": arguments
                        });
                        return server.call("call_tool", params).await;
                    }
                }
            }
        }

        Err(anyhow::anyhow!("Tool not found: {}", name))
    }
    pub async fn list_servers(&self) -> Vec<Value> {
        let config = self.config.read().await;
        config
            .mcp_servers
            .iter()
            .map(|(name, cfg)| {
                serde_json::json!({
                    "name": name,
                    "config": cfg
                })
            })
            .collect()
    }

    pub async fn add_server(&self, name: String, config: McpServerConfig) -> Result<()> {
        self.add_server_internal(&name, config.clone()).await?;

        let mut cfg = self.config.write().await;
        cfg.mcp_servers.insert(name, config);

        let content = serde_json::to_string_pretty(&*cfg)?;
        std::fs::write(&self.config_path, content)?;

        Ok(())
    }

    pub async fn remove_server(&self, name: &str) -> Result<()> {
        let mut servers = self.servers.write().await;
        servers.remove(name);

        let mut cfg = self.config.write().await;
        cfg.mcp_servers.remove(name);

        let content = serde_json::to_string_pretty(&*cfg)?;
        std::fs::write(&self.config_path, content)?;

        Ok(())
    }

    pub async fn list_servers_status(&self) -> Vec<Value> {
        let servers = self.servers.read().await;
        let mut status_list = Vec::new();
        for (name, _) in servers.iter() {
            status_list.push(serde_json::json!({
                "name": name,
                "status": "connected"
            }));
        }
        status_list
    }
}

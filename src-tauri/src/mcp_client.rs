use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::process::{Child, Command};
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use anyhow::{Result, anyhow, Context};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::oneshot;
use reqwest::Client as HttpClient;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: Value,
    pub id: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub result: Option<Value>,
    pub error: Option<JsonRpcError>,
    pub id: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    pub data: Option<Value>,
}

pub enum McpTransport {
    Stdio {
        _child: Arc<tokio::sync::Mutex<Child>>,
        writer: Arc<tokio::sync::Mutex<tokio::process::ChildStdin>>,
    },
    Http {
        client: HttpClient,
        url: String,
    },
}

pub struct McpClient {
    transport: McpTransport,
    pending_requests: Arc<tokio::sync::Mutex<HashMap<String, oneshot::Sender<Result<Value>>>>>,
    timeout: Duration,
}

impl McpClient {
    pub async fn spawn(command: &str, args: Vec<&str>) -> Result<Arc<Self>> {
        #[allow(unused_mut)]
        let mut final_command = command.to_string();
        
        #[cfg(target_os = "windows")]
        {
            if command == "npx" || command == "npm" || command == "yarn" || command == "pnpm" {
                final_command = format!("{}.cmd", command);
            }
        }

        let mut child = Command::new(final_command)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Failed to spawn MCP server")?;

        let stdin = child.stdin.take().ok_or_else(|| anyhow!("Failed to open stdin"))?;
        let stdout = child.stdout.take().ok_or_else(|| anyhow!("Failed to open stdout"))?;
        
        let client = Arc::new(Self {
            transport: McpTransport::Stdio {
                _child: Arc::new(tokio::sync::Mutex::new(child)),
                writer: Arc::new(tokio::sync::Mutex::new(stdin)),
            },
            pending_requests: Arc::new(tokio::sync::Mutex::new(HashMap::new())),
            timeout: Duration::from_secs(30),
        });

        let client_clone = client.clone();
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                if let Ok(response) = serde_json::from_str::<JsonRpcResponse>(&line) {
                    let id_str = match &response.id {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        _ => continue,
                    };

                    let mut pending = client_clone.pending_requests.lock().await;
                    if let Some(tx) = pending.remove(&id_str) {
                        if let Some(error) = response.error {
                            let _ = tx.send(Err(anyhow!(error.message)));
                        } else {
                            let _ = tx.send(Ok(response.result.unwrap_or(Value::Null)));
                        }
                    }
                }
            }
        });

        Ok(client)
    }

    pub fn connect_http(url: String) -> Result<Arc<Self>> {
        let client = Arc::new(Self {
            transport: McpTransport::Http {
                client: HttpClient::new(),
                url,
            },
            pending_requests: Arc::new(tokio::sync::Mutex::new(HashMap::new())),
            timeout: Duration::from_secs(30),
        });
        Ok(client)
    }

    pub async fn call(&self, method: &str, params: Value) -> Result<Value> {
        let id = uuid::Uuid::new_v4().to_string();
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params,
            id: Value::String(id.clone()),
        };

        match &self.transport {
            McpTransport::Stdio { writer, .. } => {
                let json = serde_json::to_string(&request)? + "\n";
                let (tx, rx) = oneshot::channel();
                {
                    let mut pending = self.pending_requests.lock().await;
                    pending.insert(id, tx);
                }

                {
                    let mut writer = writer.lock().await;
                    writer.write_all(json.as_bytes()).await?;
                    writer.flush().await?;
                }

                tokio::time::timeout(self.timeout, rx)
                    .await
                    .map_err(|_| anyhow!("MCP request timed out"))?
                    .map_err(|e| anyhow!("Oneshot error: {}", e))?
            }
            McpTransport::Http { client, url } => {
                let response = tokio::time::timeout(
                    self.timeout,
                    client.post(url).json(&request).send()
                )
                .await
                .map_err(|_| anyhow!("MCP HTTP request timed out"))??
                .json::<JsonRpcResponse>()
                .await?;

                if let Some(error) = response.error {
                    Err(anyhow!(error.message))
                } else {
                    Ok(response.result.unwrap_or(Value::Null))
                }
            }
        }
    }
}

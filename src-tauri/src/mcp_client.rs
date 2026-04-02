use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::process::{Child, Command, Stdio};
use std::io::{BufReader, BufRead, Write};
use anyhow::{Result, anyhow, Context};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;
use reqwest::Client as HttpClient;

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
        _child: Child,
        writer: Arc<Mutex<Box<dyn Write + Send>>>,
    },
    Http {
        client: HttpClient,
        url: String,
    },
}

pub struct McpClient {
    transport: McpTransport,
    pending_requests: Arc<Mutex<HashMap<String, oneshot::Sender<Result<Value>>>>>,
}

impl McpClient {
    pub fn spawn(command: &str, args: Vec<&str>) -> Result<Arc<Self>> {
        #[allow(unused_mut)]
        let mut final_command = command.to_string();
        
        #[cfg(target_os = "windows")]
        {
            // On Windows, npx/npm/yarn are batch files
            if command == "npx" || command == "npm" || command == "yarn" || command == "pnpm" {
                final_command = format!("{}.cmd", command);
            }
        }

        use crate::process_ext::CommandExtHidden;
        let mut child = Command::new(final_command)
            .hidden()
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
                _child: child,
                writer: Arc::new(Mutex::new(Box::new(stdin))),
            },
            pending_requests: Arc::new(Mutex::new(HashMap::new())),
        });

        let client_clone = client.clone();
        std::thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                match line {
                    Ok(l) => {
                        if let Ok(response) = serde_json::from_str::<JsonRpcResponse>(&l) {
                            let id_str = match &response.id {
                                Value::String(s) => s.clone(),
                                Value::Number(n) => n.to_string(),
                                _ => continue,
                            };

                            let mut pending = client_clone.pending_requests.lock().unwrap();
                            if let Some(tx) = pending.remove(&id_str) {
                                if let Some(error) = response.error {
                                    let _ = tx.send(Err(anyhow!(error.message)));
                                } else {
                                    let _ = tx.send(Ok(response.result.unwrap_or(Value::Null)));
                                }
                            }
                        }
                    }
                    Err(_) => break,
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
            pending_requests: Arc::new(Mutex::new(HashMap::new())),
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
                    let mut pending = self.pending_requests.lock().unwrap();
                    pending.insert(id, tx);
                }

                {
                    let mut writer = writer.lock().unwrap();
                    writer.write_all(json.as_bytes())?;
                    writer.flush()?;
                }

                rx.await.map_err(|e| anyhow!("Oneshot error: {}", e))?
            }
            McpTransport::Http { client, url } => {
                let response = client.post(url)
                    .json(&request)
                    .send()
                    .await?
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

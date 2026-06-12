//! ZeroScript-style WebUI → MCP bridge.
//!
//! A WebSocket server that runs *inside* the IDE. The `webui-bridge` browser extension
//! connects here from a free web-chat tab (Claude/GPT/Qwen/DeepSeek/Gemini). The
//! extension observes the chat DOM, extracts the tool calls the model emits, and relays
//! them to us; we execute them against the SAME `AiTools` registry the HTTP MCP server
//! on :1537 uses, then send results back for the extension to inject into the chat.
//!
//! Unlike ZeroScript (Chrome extension + separate Python bridge), there is no second
//! process — the Tauri app itself is the bridge. The extension only needs to reach
//! `ws://127.0.0.1:1538`.
//!
//! ## Wire protocol (JSON text frames)
//! Inbound (extension → bridge):
//!   {"type":"hello","provider":"claude","tab_id":"…"}
//!   {"type":"tool_call","id":"…","name":"view_file","args":{…}}
//!   {"type":"assistant_text","text":"…"}     // full assistant message, for completion/logging
//!   {"type":"pong"}
//! Outbound (bridge → extension):
//!   {"type":"hello_ack","tab_id":"…","protocol":"<system prompt>"}
//!   {"type":"system_prompt","text":"…"}       // inject as a new user message to start a task
//!   {"type":"inject","text":"…"}              // type text into composer + submit
//!   {"type":"tool_result","id":"…","result":<value>}
//!   {"type":"ping"}

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::net::TcpListener;
use tokio::sync::{broadcast, mpsc, Mutex};
use tokio_tungstenite::tungstenite::Message;

use crate::ai_tools::AiTools;
use crate::domain::ai::webui_protocol::{parse_webui_tool_calls, WEBUI_TOOL_PROTOCOL};

/// Cap on a single tool result echoed back into the chat (keep the web UI responsive).
const MAX_RESULT_CHARS: usize = 4000;

/// Default bridge port. 1537 is the HTTP MCP server; 1538 is this WebSocket bridge.
pub const BRIDGE_PORT: u16 = 1538;

// ── Events (consumed by the autonomous supervisor) ────────────────────────────

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum BridgeEvent {
    TabConnected { tab_id: String, provider: String },
    TabDisconnected { tab_id: String, provider: String },
    AssistantText { tab_id: String, provider: String, text: String },
    ToolCall { tab_id: String, provider: String, name: String },
    ToolResult { tab_id: String, provider: String, name: String, ok: bool },
}

// ── Inbound message shape ─────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum Inbound {
    Hello {
        provider: String,
        #[serde(default)]
        tab_id: Option<String>,
    },
    ToolCall {
        id: String,
        name: String,
        #[serde(default)]
        args: Value,
    },
    AssistantText {
        text: String,
    },
    Pong,
}

// ── Connected-tab handle ──────────────────────────────────────────────────────

struct TabHandle {
    provider: String,
    /// Outbound channel — the per-connection writer task drains this to the socket.
    tx: mpsc::UnboundedSender<String>,
    connected_at: Instant,
}

/// Public, serializable view of a connected tab for the frontend status strip.
#[derive(Debug, Clone, Serialize)]
pub struct TabStatus {
    pub tab_id: String,
    pub provider: String,
    pub connected_secs: u64,
    pub cooling: bool,
}

// ── Bridge state ──────────────────────────────────────────────────────────────

pub struct WebUiBridge {
    ai_tools: Arc<AiTools>,
    tabs: Mutex<HashMap<String, TabHandle>>,
    /// Provider → time until which it should be skipped (rate-limit cooldown).
    cooldowns: Mutex<HashMap<String, Instant>>,
    events: broadcast::Sender<BridgeEvent>,
}

pub type WebUiBridgeHandle = Arc<WebUiBridge>;

impl WebUiBridge {
    pub fn new(ai_tools: Arc<AiTools>) -> WebUiBridgeHandle {
        let (events, _rx) = broadcast::channel(512);
        Arc::new(Self {
            ai_tools,
            tabs: Mutex::new(HashMap::new()),
            cooldowns: Mutex::new(HashMap::new()),
            events,
        })
    }

    /// Subscribe to live bridge events (supervisor uses this to await replies).
    pub fn subscribe(&self) -> broadcast::Receiver<BridgeEvent> {
        self.events.subscribe()
    }

    /// Start accepting WebSocket connections. Mirrors `McpServer::start`.
    pub async fn start(self: Arc<Self>, port: u16) {
        let addr = format!("127.0.0.1:{port}");
        let listener = match TcpListener::bind(&addr).await {
            Ok(l) => l,
            Err(e) => {
                eprintln!("[webui-bridge] ⚠️ bind {addr} failed: {e} — trying random port");
                match TcpListener::bind("127.0.0.1:0").await {
                    Ok(l) => l,
                    Err(e2) => {
                        eprintln!("[webui-bridge] ❌ could not bind any port: {e2}");
                        return;
                    }
                }
            }
        };
        let local = listener
            .local_addr()
            .map(|a| a.to_string())
            .unwrap_or_else(|_| addr.clone());
        println!("[webui-bridge] ✅ listening at ws://{local}");

        loop {
            match listener.accept().await {
                Ok((stream, _peer)) => {
                    let bridge = self.clone();
                    tokio::spawn(async move { bridge.handle_conn(stream).await });
                }
                Err(e) => {
                    eprintln!("[webui-bridge] accept error: {e}");
                    tokio::time::sleep(Duration::from_millis(200)).await;
                }
            }
        }
    }

    async fn handle_conn(self: Arc<Self>, stream: tokio::net::TcpStream) {
        let ws = match tokio_tungstenite::accept_async(stream).await {
            Ok(ws) => ws,
            Err(e) => {
                eprintln!("[webui-bridge] handshake failed: {e}");
                return;
            }
        };
        let (mut write, mut read) = ws.split();

        // Outbound pump: anything pushed to `tx` is written to this socket.
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();
        let writer = tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if write.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }
        });

        // The tab id is assigned on the first `hello` (or generated).
        let mut tab_id: Option<String> = None;
        let mut provider = String::from("unknown");

        while let Some(frame) = read.next().await {
            let text = match frame {
                Ok(Message::Text(t)) => t,
                Ok(Message::Binary(_)) | Ok(Message::Ping(_)) | Ok(Message::Pong(_)) => continue,
                Ok(Message::Close(_)) | Err(_) => break,
                Ok(Message::Frame(_)) => continue,
            };
            let inbound: Inbound = match serde_json::from_str(&text) {
                Ok(v) => v,
                Err(_) => continue, // ignore malformed frames
            };

            match inbound {
                Inbound::Hello { provider: p, tab_id: tid } => {
                    let id = tid.unwrap_or_else(|| format!("tab_{}", uuid::Uuid::new_v4().simple()));
                    provider = p.to_lowercase();
                    tab_id = Some(id.clone());
                    self.tabs.lock().await.insert(
                        id.clone(),
                        TabHandle {
                            provider: provider.clone(),
                            tx: tx.clone(),
                            connected_at: Instant::now(),
                        },
                    );
                    let ack = json!({
                        "type": "hello_ack",
                        "tab_id": id,
                        "protocol": WEBUI_TOOL_PROTOCOL,
                    });
                    let _ = tx.send(ack.to_string());
                    let _ = self.events.send(BridgeEvent::TabConnected {
                        tab_id: id,
                        provider: provider.clone(),
                    });
                }

                Inbound::ToolCall { id, name, args } => {
                    let _ = self.events.send(BridgeEvent::ToolCall {
                        tab_id: tab_id.clone().unwrap_or_default(),
                        provider: provider.clone(),
                        name: name.clone(),
                    });
                    // Execute against the real tool registry.
                    let result = self
                        .ai_tools
                        .call_tool(&name, args)
                        .await
                        .unwrap_or_else(|e| json!({ "error": e.to_string() }));
                    let ok = result.get("error").is_none();
                    let _ = self.events.send(BridgeEvent::ToolResult {
                        tab_id: tab_id.clone().unwrap_or_default(),
                        provider: provider.clone(),
                        name,
                        ok,
                    });
                    let out = json!({ "type": "tool_result", "id": id, "result": result });
                    let _ = tx.send(out.to_string());
                }

                Inbound::AssistantText { text } => {
                    let _ = self.events.send(BridgeEvent::AssistantText {
                        tab_id: tab_id.clone().unwrap_or_default(),
                        provider: provider.clone(),
                        text: text.clone(),
                    });
                    // Drive the loop: parse any tool calls the model emitted, execute them
                    // against the real registry, and inject the combined results back so the
                    // model continues. This is what makes tool-calling actually fire — the
                    // extension only observes + injects; all logic lives here in Rust.
                    let calls = parse_webui_tool_calls(&text);
                    if !calls.is_empty() {
                        let mut combined = String::new();
                        for (name, args) in calls {
                            let _ = self.events.send(BridgeEvent::ToolCall {
                                tab_id: tab_id.clone().unwrap_or_default(),
                                provider: provider.clone(),
                                name: name.clone(),
                            });
                            let result = self
                                .ai_tools
                                .call_tool(&name, args)
                                .await
                                .unwrap_or_else(|e| json!({ "error": e.to_string() }));
                            let ok = result.get("error").is_none();
                            let _ = self.events.send(BridgeEvent::ToolResult {
                                tab_id: tab_id.clone().unwrap_or_default(),
                                provider: provider.clone(),
                                name: name.clone(),
                                ok,
                            });
                            let mut body = result.to_string();
                            if body.len() > MAX_RESULT_CHARS {
                                body.truncate(MAX_RESULT_CHARS);
                                body.push_str("\n…(truncated)");
                            }
                            combined.push_str(&format!("\n## Result of {name}\n```json\n{body}\n```\n"));
                        }
                        let inject_text = format!(
                            "# Tool results{combined}\nContinue: emit the next tool call(s) in the same JSON format, or write TASK_COMPLETE when the task is fully done and verified."
                        );
                        let _ = tx.send(
                            json!({ "type": "inject", "text": inject_text }).to_string(),
                        );
                    }
                }

                Inbound::Pong => {}
            }
        }

        // Cleanup on disconnect.
        if let Some(id) = tab_id {
            self.tabs.lock().await.remove(&id);
            let _ = self.events.send(BridgeEvent::TabDisconnected {
                tab_id: id,
                provider: provider.clone(),
            });
        }
        writer.abort();
    }

    // ── Outbound API (used by commands + supervisor) ──────────────────────────

    /// Push a raw JSON message to a specific tab. Returns false if the tab is gone.
    pub async fn send_to_tab(&self, tab_id: &str, msg: Value) -> bool {
        let tabs = self.tabs.lock().await;
        match tabs.get(tab_id) {
            Some(h) => h.tx.send(msg.to_string()).is_ok(),
            None => false,
        }
    }

    /// Inject a system-prompt + task into a tab to kick off a session.
    pub async fn start_task_on_tab(&self, tab_id: &str, task: &str) -> bool {
        let text = format!("{WEBUI_TOOL_PROTOCOL}\n\n# Task\n{task}");
        self.send_to_tab(tab_id, json!({ "type": "system_prompt", "text": text }))
            .await
    }

    /// Inject arbitrary text (e.g. a nudge) into a tab's composer and submit.
    pub async fn inject(&self, tab_id: &str, text: &str) -> bool {
        self.send_to_tab(tab_id, json!({ "type": "inject", "text": text }))
            .await
    }

    /// Pick a connected, non-cooling tab for `provider` (or any provider if None).
    pub async fn pick_tab(&self, provider: Option<&str>) -> Option<String> {
        let cooling = self.cooldowns.lock().await;
        let now = Instant::now();
        let tabs = self.tabs.lock().await;
        tabs.iter()
            .filter(|(_, h)| provider.map(|p| h.provider == p).unwrap_or(true))
            .filter(|(_, h)| cooling.get(&h.provider).map(|t| *t <= now).unwrap_or(true))
            .map(|(id, _)| id.clone())
            .next()
    }

    /// Mark a provider as rate-limited; the supervisor skips it until the cooldown ends.
    pub async fn set_cooldown(&self, provider: &str, dur: Duration) {
        self.cooldowns
            .lock()
            .await
            .insert(provider.to_lowercase(), Instant::now() + dur);
    }

    /// Snapshot of connected tabs for the frontend status strip.
    pub async fn status(&self) -> Vec<TabStatus> {
        let cooling = self.cooldowns.lock().await;
        let now = Instant::now();
        let tabs = self.tabs.lock().await;
        tabs.iter()
            .map(|(id, h)| TabStatus {
                tab_id: id.clone(),
                provider: h.provider.clone(),
                connected_secs: h.connected_at.elapsed().as_secs(),
                cooling: cooling.get(&h.provider).map(|t| *t > now).unwrap_or(false),
            })
            .collect()
    }

    /// Distinct providers with at least one healthy connected tab.
    pub async fn connected_providers(&self) -> Vec<String> {
        let cooling = self.cooldowns.lock().await;
        let now = Instant::now();
        let tabs = self.tabs.lock().await;
        let mut set: Vec<String> = tabs
            .values()
            .filter(|h| cooling.get(&h.provider).map(|t| *t <= now).unwrap_or(true))
            .map(|h| h.provider.clone())
            .collect();
        set.sort();
        set.dedup();
        set
    }
}

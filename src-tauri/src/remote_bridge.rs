//! Remote agent bridge — a localhost WebSocket that drives the built-in
//! autonomous agent from a browser or a companion app.
//!
//! Protocol (`ws://127.0.0.1:<port>/agent?token=<token>`), JSON text frames:
//!
//! ```text
//! client → { "type": "prompt", "text": "...", "mode": "Agent", "root_access": false }
//! client → { "type": "ping" }
//! client → { "type": "cancel" }              // best-effort; see note below
//!
//! server → { "type": "ready" }               // on connect
//! server → { "type": "delta",  "text": "..." }   // streamed model output
//! server → { "type": "done",   "text": "..." }   // final assistant text
//! server → { "type": "error",  "message": "..." }
//! server → { "type": "pong" }
//! ```
//!
//! Security: binds `127.0.0.1` only, and every connection must present the
//! `token` returned by `remote_bridge_start` (query string or `x-kortex-token`
//! header). The server is **off unless started** — either the `remote_bridge_start`
//! command or `KORTEX_REMOTE=1` in the environment.

use std::sync::Mutex;

use futures::{SinkExt, StreamExt};
use serde::Serialize;
use serde_json::{json, Value};
use tokio::net::TcpListener;
use tokio::sync::{mpsc, oneshot};
use tokio_tungstenite::tungstenite::handshake::server::{ErrorResponse, Request, Response};
use tokio_tungstenite::tungstenite::Message;

use crate::ai_engine::types::{AiRequest, ChatMessage, MessageContent};

const DEFAULT_PORT: u16 = 8791;

const AGENT_SYSTEM: &str = "You are an autonomous coding agent operating over a \
remote connection. Complete the task end to end with your tools: read the \
relevant files, make the change, verify it. Do not ask questions.";

struct Handle {
    port: u16,
    token: String,
    shutdown: oneshot::Sender<()>,
}

static SERVER: Mutex<Option<Handle>> = Mutex::new(None);

#[derive(Serialize)]
pub struct RemoteInfo {
    pub running: bool,
    pub port: u16,
    /// Present only in the `remote_bridge_start` result — never logged.
    pub token: String,
    pub url: String,
}

/// Start the bridge. Idempotent — returns the existing token if already up.
#[tauri::command]
pub async fn remote_bridge_start(
    state: tauri::State<'_, std::sync::Arc<crate::EditorState>>,
    port: Option<u16>,
) -> Result<RemoteInfo, String> {
    {
        let g = SERVER.lock().unwrap();
        if let Some(h) = g.as_ref() {
            return Ok(RemoteInfo {
                running: true,
                port: h.port,
                token: h.token.clone(),
                url: format!("ws://127.0.0.1:{}/agent?token={}", h.port, h.token),
            });
        }
    }

    let port = port.unwrap_or(DEFAULT_PORT);
    let token = uuid::Uuid::new_v4().simple().to_string();
    let listener = TcpListener::bind(("127.0.0.1", port))
        .await
        .map_err(|e| format!("bind 127.0.0.1:{port} failed: {e}"))?;

    let (tx, mut rx) = oneshot::channel::<()>();
    let engine = state.ai.engine.clone();
    let current_model = state.ai.current_model.lock().await.clone();
    let tok = token.clone();

    tokio::spawn(async move {
        tracing::info!("[remote-bridge] listening on ws://127.0.0.1:{port}/agent");
        loop {
            tokio::select! {
                _ = &mut rx => {
                    tracing::info!("[remote-bridge] shutting down");
                    break;
                }
                accepted = listener.accept() => {
                    let Ok((stream, peer)) = accepted else { continue };
                    let tok = tok.clone();
                    let engine = engine.clone();
                    let model = current_model.clone();
                    tokio::spawn(async move {
                        if let Err(e) = handle_conn(stream, tok, engine, model).await {
                            tracing::debug!("[remote-bridge] {peer} closed: {e}");
                        }
                    });
                }
            }
        }
    });

    *SERVER.lock().unwrap() = Some(Handle { port, token: token.clone(), shutdown: tx });
    Ok(RemoteInfo {
        running: true,
        port,
        token: token.clone(),
        url: format!("ws://127.0.0.1:{port}/agent?token={token}"),
    })
}

#[tauri::command]
pub async fn remote_bridge_stop() -> Result<(), String> {
    if let Some(h) = SERVER.lock().unwrap().take() {
        let _ = h.shutdown.send(());
    }
    Ok(())
}

#[tauri::command]
pub async fn remote_bridge_status() -> Result<RemoteInfo, String> {
    let g = SERVER.lock().unwrap();
    Ok(match g.as_ref() {
        Some(h) => RemoteInfo {
            running: true,
            port: h.port,
            token: String::new(), // status never leaks the token
            url: format!("ws://127.0.0.1:{}/agent", h.port),
        },
        None => RemoteInfo { running: false, port: 0, token: String::new(), url: String::new() },
    })
}

/// Auto-start from `KORTEX_REMOTE=1` (`KORTEX_REMOTE_PORT` optional). Called
/// from the Tauri setup hook. The token is logged once at info level so a
/// local companion app can pick it up from the IDE log.
pub fn maybe_autostart(state: std::sync::Arc<crate::EditorState>) {
    let on = matches!(
        std::env::var("KORTEX_REMOTE").ok().as_deref(),
        Some("1") | Some("true") | Some("on")
    );
    if !on {
        return;
    }
    let port = std::env::var("KORTEX_REMOTE_PORT").ok().and_then(|p| p.parse().ok());
    tokio::spawn(async move {
        // reuse the command body via a lightweight State shim isn't possible;
        // inline the same start path.
        let port = port.unwrap_or(DEFAULT_PORT);
        let token = uuid::Uuid::new_v4().simple().to_string();
        let Ok(listener) = TcpListener::bind(("127.0.0.1", port)).await else {
            tracing::warn!("[remote-bridge] autostart: bind 127.0.0.1:{port} failed");
            return;
        };
        let (tx, mut rx) = oneshot::channel::<()>();
        let engine = state.ai.engine.clone();
        let model = state.ai.current_model.lock().await.clone();
        tracing::info!(
            "[remote-bridge] autostarted — ws://127.0.0.1:{port}/agent?token={token}"
        );
        *SERVER.lock().unwrap() = Some(Handle { port, token: token.clone(), shutdown: tx });
        loop {
            tokio::select! {
                _ = &mut rx => break,
                accepted = listener.accept() => {
                    let Ok((stream, _peer)) = accepted else { continue };
                    let (tok, engine, model) = (token.clone(), engine.clone(), model.clone());
                    tokio::spawn(async move {
                        let _ = handle_conn(stream, tok, engine, model).await;
                    });
                }
            }
        }
    });
}

fn token_from_request(req: &Request, expected: &str) -> bool {
    // header first
    if let Some(v) = req.headers().get("x-kortex-token") {
        if v.to_str().map(|s| s == expected).unwrap_or(false) {
            return true;
        }
    }
    // ?token= in the request target
    if let Some(q) = req.uri().query() {
        for pair in q.split('&') {
            if let Some(val) = pair.strip_prefix("token=") {
                if val == expected {
                    return true;
                }
            }
        }
    }
    false
}

async fn handle_conn(
    stream: tokio::net::TcpStream,
    token: String,
    engine: std::sync::Arc<crate::ai_engine::Sentient>,
    model: String,
) -> anyhow::Result<()> {
    let ws = tokio_tungstenite::accept_hdr_async(stream, |req: &Request, resp: Response| {
        if token_from_request(req, &token) {
            Ok(resp)
        } else {
            let mut e = ErrorResponse::new(Some("invalid or missing token".into()));
            *e.status_mut() = tokio_tungstenite::tungstenite::http::StatusCode::UNAUTHORIZED;
            Err(e)
        }
    })
    .await?;

    let (mut sink, mut source) = ws.split();

    // All outbound frames funnel through this channel so the streaming
    // `on_chunk` callback and the request loop never touch the sink directly.
    let (out_tx, mut out_rx) = mpsc::unbounded_channel::<Message>();
    let writer = tokio::spawn(async move {
        while let Some(msg) = out_rx.recv().await {
            if sink.send(msg).await.is_err() {
                break;
            }
        }
        let _ = sink.close().await;
    });

    let _ = out_tx.send(Message::Text(json!({ "type": "ready" }).to_string()));

    let mut busy = false;
    while let Some(frame) = source.next().await {
        let frame = match frame {
            Ok(f) => f,
            Err(_) => break,
        };
        let text = match frame {
            Message::Text(t) => t,
            Message::Close(_) => break,
            Message::Ping(p) => {
                let _ = out_tx.send(Message::Pong(p));
                continue;
            }
            _ => continue,
        };
        let Ok(v) = serde_json::from_str::<Value>(&text) else {
            let _ = out_tx.send(err_frame("not JSON"));
            continue;
        };
        match v.get("type").and_then(Value::as_str).unwrap_or("") {
            "ping" => {
                let _ = out_tx.send(Message::Text(json!({ "type": "pong" }).to_string()));
            }
            "cancel" => {
                let _ = out_tx.send(Message::Text(
                    json!({ "type": "info", "message": "cancel is not supported mid-run yet" })
                        .to_string(),
                ));
            }
            "prompt" => {
                if busy {
                    let _ = out_tx.send(err_frame("a prompt is already running on this connection"));
                    continue;
                }
                let Some(prompt) = v.get("text").and_then(Value::as_str).filter(|s| !s.trim().is_empty())
                else {
                    let _ = out_tx.send(err_frame("prompt needs a non-empty 'text'"));
                    continue;
                };
                busy = true;
                run_prompt(
                    &engine,
                    &model,
                    prompt,
                    v.get("mode").and_then(Value::as_str).unwrap_or("Agent"),
                    v.get("root_access").and_then(Value::as_bool).unwrap_or(false),
                    &out_tx,
                )
                .await;
                busy = false;
            }
            other => {
                let _ = out_tx.send(err_frame(&format!("unknown message type '{other}'")));
            }
        }
    }

    drop(out_tx);
    let _ = writer.await;
    Ok(())
}

fn err_frame(msg: &str) -> Message {
    Message::Text(json!({ "type": "error", "message": msg }).to_string())
}

async fn run_prompt(
    engine: &std::sync::Arc<crate::ai_engine::Sentient>,
    model: &str,
    prompt: &str,
    mode: &str,
    root_access: bool,
    out_tx: &mpsc::UnboundedSender<Message>,
) {
    let req = AiRequest {
        provider: "lemonade".to_string(),
        model: model.to_string(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: Some(MessageContent::Text(AGENT_SYSTEM.to_string())),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
            ChatMessage {
                role: "user".to_string(),
                content: Some(MessageContent::Text(prompt.to_string())),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
        ],
        temperature: Some(0.2),
        autonomous: true,
        cyber_mode: None,
        root_access: Some(root_access),
        mode: Some(mode.to_string()),
        inference_url: None,
        tools: None,
        reasoning_budget: None,
        reasoning_effort: None,
        reasoning_enabled: None,
        feature: None,
    };

    let tx = out_tx.clone();
    let on_chunk: std::sync::Arc<dyn Fn(&str) + Send + Sync> =
        std::sync::Arc::new(move |chunk: &str| {
            let _ = tx.send(Message::Text(
                json!({ "type": "delta", "text": chunk }).to_string(),
            ));
        });

    match engine.clone().autonomous_loop(req, Some(on_chunk)).await {
        Ok(final_text) => {
            let _ = out_tx.send(Message::Text(
                json!({ "type": "done", "text": final_text }).to_string(),
            ));
        }
        Err(e) => {
            let _ = out_tx.send(err_frame(&format!("agent error: {e}")));
        }
    }
}

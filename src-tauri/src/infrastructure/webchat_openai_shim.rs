//! OpenAI-compatible shim in front of the headless web-chat driver.
//!
//! Listens on `127.0.0.1:1539` and speaks the `/v1/chat/completions` contract the
//! agent loop already knows. When the loop selects a `webchat-*` provider, its
//! `get_endpoint` resolves here; this server flattens the OpenAI request into a
//! single prompt, drives the logged-in web chat via [`WebChatDriver::complete`],
//! and streams the reply back as standard OpenAI SSE chunks. Result: the entire
//! `autonomous_loop` (tool parsing, verify gate, streaming UI) reuses unchanged,
//! and Ollama / real APIs are untouched.

use std::sync::Arc;

use axum::{
    response::sse::{Event, Sse},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use futures::stream::StreamExt;
use serde_json::{json, Value};

use crate::infrastructure::web_chat_driver::WebChatDriver;

pub const WEBCHAT_SHIM_PORT: u16 = 1539;

pub struct WebChatShim {
    driver: Arc<WebChatDriver>,
}

impl WebChatShim {
    pub fn new(driver: Arc<WebChatDriver>) -> Arc<Self> {
        Arc::new(Self { driver })
    }

    pub async fn start(self: Arc<Self>, port: u16) {
        let app = Router::new().route(
            "/v1/chat/completions",
            post(move |body: Json<Value>| {
                let shim = self.clone();
                async move { shim.handle(body.0).await }
            }),
        );
        let addr = format!("127.0.0.1:{}", port);
        let listener = match tokio::net::TcpListener::bind(&addr).await {
            Ok(l) => l,
            Err(e) => {
                eprintln!("[WEBCHAT-SHIM] ⚠️ bind {} failed: {}", addr, e);
                return;
            }
        };
        println!("[WEBCHAT-SHIM] ✅ OpenAI shim active at http://{}", addr);
        if let Err(e) = axum::serve(listener, app).await {
            eprintln!("[WEBCHAT-SHIM] 🛑 server error: {}", e);
        }
    }

    async fn handle(&self, req: Value) -> axum::response::Response {
        let model = req
            .get("model")
            .and_then(|v| v.as_str())
            .unwrap_or("webchat-claude")
            .to_string();
        let prompt = flatten_messages(req.get("messages"));
        let driver = self.driver.clone();

        // Stream growth via an unbounded channel; the SSE body drains it.
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<String>();
        let id = format!("chatcmpl-webchat-{}", now_millis());
        let model_for_task = model.clone();

        tokio::spawn(async move {
            let send_tx = tx.clone();
            let res = driver
                .complete(&model_for_task, &prompt, move |delta| {
                    let _ = send_tx.send(delta.to_string());
                })
                .await;
            if let Err(e) = res {
                // Surface the failure to the agent loop as a content chunk so the
                // run doesn't silently hang — it reads as an assistant message.
                let _ = tx.send(format!("\n[webchat error] {}\n", e));
            }
        });

        // Build the SSE stream: one OpenAI chunk per delta, then stop + [DONE].
        let id_open = id.clone();
        let model_open = model.clone();
        let init = futures::stream::once(async move {
            Ok::<Event, std::convert::Infallible>(Event::default().data(
                chunk_json(&id_open, &model_open, Some(""), None).to_string(),
            ))
        });

        let id_body = id.clone();
        let model_body = model.clone();
        let body = futures::stream::unfold(
            (rx, false),
            move |(mut rx, finished)| {
                let id_body = id_body.clone();
                let model_body = model_body.clone();
                async move {
                    if finished {
                        return None;
                    }
                    match rx.recv().await {
                        Some(delta) => {
                            let ev = Event::default()
                                .data(chunk_json(&id_body, &model_body, Some(&delta), None).to_string());
                            Some((Ok::<Event, std::convert::Infallible>(ev), (rx, false)))
                        }
                        None => {
                            // Channel closed → emit the stop chunk, then end next poll.
                            let ev = Event::default().data(
                                chunk_json(&id_body, &model_body, None, Some("stop")).to_string(),
                            );
                            Some((Ok(ev), (rx, true)))
                        }
                    }
                }
            },
        );

        let done = futures::stream::once(async {
            Ok::<Event, std::convert::Infallible>(Event::default().data("[DONE]"))
        });

        let stream = init.chain(body).chain(done);
        Sse::new(stream).into_response()
    }
}

/// Flatten an OpenAI `messages` array into a single prompt the web chat can read.
fn flatten_messages(messages: Option<&Value>) -> String {
    let arr = match messages.and_then(|m| m.as_array()) {
        Some(a) => a,
        None => return String::new(),
    };
    let mut system = String::new();
    let mut convo = String::new();
    for m in arr {
        let role = m.get("role").and_then(|r| r.as_str()).unwrap_or("user");
        let content = extract_content(m.get("content"));
        if content.trim().is_empty() {
            continue;
        }
        match role {
            "system" => {
                system.push_str(&content);
                system.push_str("\n\n");
            }
            "assistant" => {
                convo.push_str("\n\n[ASSISTANT]\n");
                convo.push_str(&content);
            }
            "tool" => {
                convo.push_str("\n\n[TOOL RESULT]\n");
                convo.push_str(&content);
            }
            _ => {
                convo.push_str("\n\n[USER]\n");
                convo.push_str(&content);
            }
        }
    }
    format!("{}{}", system, convo.trim_start())
}

/// OpenAI content can be a string or an array of `{type,text}` parts.
fn extract_content(content: Option<&Value>) -> String {
    match content {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Array(parts)) => parts
            .iter()
            .filter_map(|p| p.get("text").and_then(|t| t.as_str()))
            .collect::<Vec<_>>()
            .join("\n"),
        _ => String::new(),
    }
}

/// Build one OpenAI streaming chunk.
fn chunk_json(id: &str, model: &str, content: Option<&str>, finish: Option<&str>) -> Value {
    let mut delta = json!({});
    if let Some(c) = content {
        delta = json!({ "content": c });
    }
    json!({
        "id": id,
        "object": "chat.completion.chunk",
        "created": now_secs(),
        "model": model,
        "choices": [{ "index": 0, "delta": delta, "finish_reason": finish }],
    })
}

fn now_millis() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

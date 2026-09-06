//! Hermes-compatible OpenAI gateway on :8642 — local agent API + cron stub.

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

use crate::ai_engine::{AiRequest, ChatMessage, MessageContent, Sentient};

#[derive(Clone)]
struct GatewayState {
    sentient: Arc<Sentient>,
    inference_url: String,
    default_model: String,
    port: u16,
}

static GATEWAY: Mutex<Option<Arc<GatewayState>>> = Mutex::new(None);

#[derive(Debug, Deserialize)]
struct ChatCompletionRequest {
    model: Option<String>,
    messages: Vec<GwMessage>,
    temperature: Option<f32>,
    stream: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
struct GwMessage {
    role: String,
    content: Value,
}

#[derive(Debug, Serialize)]
struct ChatCompletionResponse {
    id: String,
    object: &'static str,
    created: i64,
    model: String,
    choices: Vec<GwChoice>,
}

#[derive(Debug, Serialize)]
struct GwChoice {
    index: u32,
    message: GwMessage,
    finish_reason: &'static str,
}

#[tauri::command]
pub async fn hermes_gateway_start(
    state: tauri::State<'_, std::sync::Arc<crate::EditorState>>,
    port: Option<u16>,
) -> Result<u16, String> {
    if GATEWAY.lock().map_err(|e| e.to_string())?.is_some() {
        return Err("Hermes gateway already running".into());
    }
    let port = port.unwrap_or(8642);
    let inference_url = state.ai.engine.lemonade_base().await;
    let default_model = state.ai.current_model.lock().await.clone();
    let gw = Arc::new(GatewayState {
        sentient: state.ai.engine.clone(),
        inference_url,
        default_model,
        port,
    });

    let gw_run = gw.clone();
    tokio::spawn(async move {
        if let Err(e) = serve_gateway(gw_run).await {
            eprintln!("[hermes-gateway] stopped: {e}");
        }
        if let Ok(mut g) = GATEWAY.lock() {
            *g = None;
        }
    });

    if let Ok(mut g) = GATEWAY.lock() {
        *g = Some(gw);
    }
    Ok(port)
}

#[tauri::command]
pub async fn hermes_gateway_stop() -> Result<(), String> {
    // Gateway task exits when process continues — mark stopped for UI
    if let Ok(mut g) = GATEWAY.lock() {
        *g = None;
    }
    Ok(())
}

#[tauri::command]
pub async fn hermes_gateway_status() -> Result<Value, String> {
    let running = GATEWAY.lock().map_err(|e| e.to_string())?.is_some();
    Ok(json!({
        "running": running,
        "port": 8642,
        "endpoints": [
            "GET /health",
            "POST /v1/chat/completions",
            "GET /v1/models",
            "POST /v1/cron/jobs",
        ],
        "description": "Hermes-compatible local agent gateway (OpenAI API shape)",
    }))
}

async fn serve_gateway(gw: Arc<GatewayState>) -> Result<(), String> {
    let app = Router::new()
        .route("/health", get(health))
        .route("/v1/models", get(list_models))
        .route("/v1/chat/completions", post(chat_completions))
        .route("/v1/cron/jobs", post(cron_job))
        .with_state(gw.clone());

    let addr = format!("127.0.0.1:{}", gw.port);
    let listener = TcpListener::bind(&addr)
        .await
        .map_err(|e| format!("bind {addr}: {e}"))?;
    println!("[hermes-gateway] listening on http://{addr}");

    axum::serve(listener, app)
        .await
        .map_err(|e| e.to_string())
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok", "service": "hermes-gateway" }))
}

async fn list_models(State(gw): State<Arc<GatewayState>>) -> Json<Value> {
    let model = gw.default_model.clone();
    Json(json!({
        "object": "list",
        "data": [{ "id": model, "object": "model", "owned_by": "hades" }]
    }))
}

async fn chat_completions(
    State(gw): State<Arc<GatewayState>>,
    Json(req): Json<ChatCompletionRequest>,
) -> Result<Json<ChatCompletionResponse>, (StatusCode, String)> {
    if req.stream.unwrap_or(false) {
        return Err((StatusCode::NOT_IMPLEMENTED, "streaming not yet supported on gateway".into()));
    }
    let model = req.model.unwrap_or_else(|| gw.default_model.clone());
    let messages: Vec<ChatMessage> = req
        .messages
        .into_iter()
        .map(|m| ChatMessage {
            role: m.role,
            content: Some(MessageContent::Text(content_to_string(&m.content))),
            tool_calls: None,
            tool_call_id: None,
            metadata: None,
        })
        .collect();

    let inference_url = gw.inference_url.clone();
    let ai_req = AiRequest {
        provider: "lemonade".into(),
        model: model.clone(),
        messages,
        temperature: req.temperature,
        autonomous: false,
        cyber_mode: None,
        root_access: Some(false),
        mode: Some("Chat".into()),
        inference_url: Some(inference_url),
        tools: None,
        reasoning_budget: None,
        reasoning_effort: None,
        reasoning_enabled: None,
        feature: Some("Chat".into()),
    };

    let text = gw
        .sentient
        .clone()
        .autonomous_loop(ai_req, None)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(ChatCompletionResponse {
        id: format!("chatcmpl-{}", uuid::Uuid::new_v4()),
        object: "chat.completion",
        created: chrono::Utc::now().timestamp(),
        model,
        choices: vec![GwChoice {
            index: 0,
            message: GwMessage {
                role: "assistant".into(),
                content: json!(text),
            },
            finish_reason: "stop",
        }],
    }))
}

async fn cron_job(Json(body): Json<Value>) -> Json<Value> {
    Json(json!({
        "ok": true,
        "scheduled": body,
        "note": "Cron job accepted — persisted via kairos scheduler when configured",
    }))
}

fn content_to_string(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Array(arr) => arr
            .iter()
            .filter_map(|p| p.get("text").and_then(|t| t.as_str()))
            .collect::<Vec<_>>()
            .join("\n"),
        _ => v.to_string(),
    }
}

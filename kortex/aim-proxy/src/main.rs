use axum::{
    routing::{post, any, get},
    Router,
    extract::{Request, State},
    response::IntoResponse,
    body::Body,
    http::Method,
    Json,
};
use reqwest::Client;
use std::net::SocketAddr;
use serde_json::{Value, json};
use tower_http::cors::{Any, CorsLayer};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::path::PathBuf;

mod api_manifest;
mod api_subagent;

const ANTHROPIC_API: &str = "https://api.anthropic.com";
// Max AIM context injected per call — keeps prefix cache stable and fits in 200K window
const AIM_INJECT_LIMIT: usize = 80_000;

#[derive(Clone)]
struct AppState {
    http_client: Client,
    target_ollama: String,
    aim_cache: Arc<RwLock<Option<AimContext>>>,
    active_subagents: Arc<RwLock<usize>>,
}

#[derive(Clone)]
struct AimContext {
    /// Extracted text from the .aim JSON header — pre-sliced to AIM_INJECT_LIMIT
    text: String,
    last_modified: std::time::SystemTime,
    path: PathBuf,
}

impl AimContext {
    fn load(path: PathBuf) -> Result<Self, String> {
        let metadata = std::fs::metadata(&path).map_err(|e| e.to_string())?;
        let bytes = std::fs::read(&path).map_err(|e| e.to_string())?;
        let text = extract_aim_text(&bytes);
        Ok(Self {
            text,
            last_modified: metadata.modified().map_err(|e| e.to_string())?,
            path,
        })
    }

    fn is_fresh(&self) -> bool {
        std::fs::metadata(&self.path)
            .and_then(|m| m.modified())
            .map(|t| t == self.last_modified)
            .unwrap_or(false)
    }
}

/// Extract the JSON header from a .aim binary file.
/// Format: JSON object (until matching closing brace) + binary float32 tensor.
/// Limits output to AIM_INJECT_LIMIT bytes for context budget.
fn extract_aim_text(bytes: &[u8]) -> String {
    // Find JSON root object boundary via brace counting
    let mut depth = 0i32;
    let mut json_end = 0usize;
    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    json_end = i + 1;
                    break;
                }
            }
            _ => {}
        }
        // Safety: bail out if JSON header is unreasonably large
        if i > 250_000_000 { break; }
    }

    let raw = if json_end > 0 {
        String::from_utf8_lossy(&bytes[..json_end.min(AIM_INJECT_LIMIT)]).to_string()
    } else {
        // Fallback: read as UTF-8 lossy up to limit
        String::from_utf8_lossy(&bytes[..bytes.len().min(AIM_INJECT_LIMIT)]).to_string()
    };

    // Try to parse JSON and format as readable context block
    if let Ok(val) = serde_json::from_str::<Value>(&raw) {
        format_aim_json(&val)
    } else {
        // Return raw text trimmed to limit
        raw.chars().take(AIM_INJECT_LIMIT).collect()
    }
}

/// Format parsed .aim JSON into a compact, LLM-readable knowledge block
fn format_aim_json(val: &Value) -> String {
    let mut out = String::from("# AIM-VFS SEMANTIC KNOWLEDGE BASE\n");
    out.push_str("*(Pre-indexed codebase gists — no file reads needed)*\n\n");

    // Recursively extract string values up to limit
    append_json_strings(val, &mut out, 0, AIM_INJECT_LIMIT);
    out
}

fn append_json_strings(val: &Value, out: &mut String, depth: usize, limit: usize) {
    if out.len() >= limit { return; }
    match val {
        Value::Object(map) => {
            for (k, v) in map {
                if out.len() >= limit { break; }
                if depth == 0 {
                    out.push_str(&format!("\n## {}\n", k));
                }
                append_json_strings(v, out, depth + 1, limit);
            }
        }
        Value::Array(arr) => {
            for item in arr {
                if out.len() >= limit { break; }
                append_json_strings(item, out, depth, limit);
            }
        }
        Value::String(s) if !s.is_empty() => {
            let remaining = limit.saturating_sub(out.len());
            let chunk: String = s.chars().take(remaining.min(2000)).collect();
            out.push_str(&chunk);
            out.push('\n');
        }
        Value::Number(n) => {
            out.push_str(&format!("{} ", n));
        }
        _ => {}
    }
}

#[derive(Clone, Default)]
struct AimPaths(Vec<PathBuf>);

impl AimPaths {
    fn new() -> Self {
        Self(vec![
            PathBuf::from("C:\\Users\\HADES\\Desktop\\kortex\\.aim\\memory.aim"),
            PathBuf::from(".\\.aim\\memory.aim"),
            PathBuf::from("..\\.aim\\memory.aim"),
            PathBuf::from("C:\\Users\\HADES\\Desktop\\vscodium-rust\\.aim\\memory.aim"),
        ])
    }

    fn find_and_load(&self) -> Option<AimContext> {
        for path in &self.0 {
            if let Ok(ctx) = AimContext::load(path.clone()) {
                return Some(ctx);
            }
        }
        None
    }
}

#[tokio::main]
async fn main() {
    let aim_cache = Arc::new(RwLock::new(None::<AimContext>));

    // Pre-load .aim context in background
    {
        let cache = aim_cache.clone();
        tokio::spawn(async move {
            if let Some(ctx) = AimPaths::new().find_and_load() {
                let path = ctx.path.display().to_string();
                let len = ctx.text.len();
                *cache.write().await = Some(ctx);
                println!("🧠 [AIM-PROXY] Preloaded {:.1}KB semantic context from {}", len as f32 / 1024.0, path);
            } else {
                println!("⚠️  [AIM-PROXY] No .aim context found — run NeuralDrive to index codebase");
            }
        });
    }

    let state = AppState {
        http_client: Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .expect("Failed to build HTTP client"),
        target_ollama: "http://127.0.0.1:11434".to_string(),
        aim_cache,
        active_subagents: Arc::new(RwLock::new(0)),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    let app = Router::new()
        // Anthropic API passthrough with .aim injection
        .route("/v1/messages", post(intercept_anthropic))
        // Ollama routes
        .route("/api/generate", post(intercept_ollama))
        .route("/api/chat", post(intercept_ollama))
        // Manifest / subagent
        .route("/api/manifest", post(api_manifest::handle_manifest))
        .route("/api/subagent", post(api_subagent::handle_subagent))
        .route("/api/subagents/status", get(api_subagent::handle_subagent_status))
        // AIM knowledge update endpoint
        .route("/aim/update", post(handle_aim_update))
        .route("/aim/status", get(handle_aim_status))
        // Passthrough for everything else
        .fallback(any(pass_through))
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 1536));
    println!("⚡ [AIM-PROXY] Universal AI Interceptor online at http://127.0.0.1:1536");
    println!("⚡ [AIM-PROXY] Anthropic API → https://api.anthropic.com  (with .aim injection)");
    println!("⚡ [AIM-PROXY] Ollama API    → http://127.0.0.1:11434     (with .aim injection)");
    println!("⚡ [AIM-PROXY] Set ANTHROPIC_BASE_URL=http://127.0.0.1:1536 in Claude Code settings");

    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind");
    axum::serve(listener, app).await.expect("Failed to start server");
}

/// Get cached .aim context text, reload if stale
async fn get_aim_text(state: &AppState) -> String {
    // Check if cached version is still fresh
    {
        let cache = state.aim_cache.read().await;
        if let Some(ref ctx) = *cache {
            if ctx.is_fresh() {
                return ctx.text.clone();
            }
        }
    }

    // Reload
    if let Some(ctx) = AimPaths::new().find_and_load() {
        let text = ctx.text.clone();
        *state.aim_cache.write().await = Some(ctx);
        return text;
    }

    String::new()
}

// ──────────────────────────────────────────────────────────────
// ANTHROPIC API INTERCEPTOR
// Injects .aim semantic context into the system prompt,
// then forwards to api.anthropic.com and streams the response back.
// ──────────────────────────────────────────────────────────────
async fn intercept_anthropic(
    State(state): State<AppState>,
    req: Request<Body>,
) -> impl IntoResponse {
    let (parts, body) = req.into_parts();

    let bytes = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(b) => b,
        Err(e) => {
            return axum::response::Response::builder()
                .status(500)
                .body(Body::from(format!("Failed to read request body: {}", e)))
                .unwrap();
        }
    };

    let mut payload: Value = match serde_json::from_slice(&bytes) {
        Ok(v) => v,
        Err(_) => {
            // Not JSON — pass through raw
            return forward_to_anthropic(&state, parts, bytes.to_vec()).await;
        }
    };

    // Inject .aim context into system prompt
    let aim_text = get_aim_text(&state).await;
    if !aim_text.is_empty() {
        inject_aim_into_anthropic_system(&mut payload, &aim_text);
        println!("🧠 [AIM-PROXY] Injected {:.1}KB AIM context into Anthropic request", aim_text.len() as f32 / 1024.0);
    }

    let new_body = serde_json::to_vec(&payload).unwrap_or(bytes.to_vec());
    forward_to_anthropic(&state, parts, new_body).await
}

/// Inject .aim context as the first system block
fn inject_aim_into_anthropic_system(payload: &mut Value, aim_text: &str) {
    let aim_block = json!({
        "type": "text",
        "text": aim_text,
        "cache_control": { "type": "ephemeral" }
    });

    match payload.get_mut("system") {
        Some(Value::Array(arr)) => {
            // Prepend .aim block if not already injected
            if !arr.iter().any(|b| b.get("text").and_then(|t| t.as_str()).map(|s| s.starts_with("# AIM-VFS")).unwrap_or(false)) {
                arr.insert(0, aim_block);
            }
        }
        Some(Value::String(s)) => {
            // Convert string system to array with .aim first
            let existing = s.clone();
            payload["system"] = json!([
                aim_block,
                {"type": "text", "text": existing}
            ]);
        }
        _ => {
            // No system field — add one
            payload["system"] = json!([aim_block]);
        }
    }
}

/// Forward request to Anthropic API, streaming response back
async fn forward_to_anthropic(
    state: &AppState,
    parts: axum::http::request::Parts,
    body: Vec<u8>,
) -> axum::response::Response<Body> {
    let target_url = format!("{}{}", ANTHROPIC_API, parts.uri.path_and_query().map(|pq| pq.as_str()).unwrap_or("/v1/messages"));

    let mut req_builder = state.http_client.post(&target_url);

    // Forward all headers except host, content-length, and accept-encoding
    for (k, v) in parts.headers.iter() {
        let key = k.as_str();
        if key != "host" && key != "content-length" && key != "accept-encoding" {
            if let (Ok(hk), Ok(hv)) = (reqwest::header::HeaderName::try_from(key), reqwest::header::HeaderValue::from_bytes(v.as_bytes())) {
                req_builder = req_builder.header(hk, hv);
            }
        }
    }

    let resp = req_builder.body(body).send().await;

    match resp {
        Ok(r) => {
            let status = axum::http::StatusCode::from_u16(r.status().as_u16())
                .unwrap_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
            let mut builder = axum::response::Response::builder().status(status);

            for (k, v) in r.headers() {
                if k.as_str() != "content-length" && k.as_str() != "transfer-encoding" && k.as_str() != "content-encoding" {
                    if let (Ok(hk), Ok(hv)) = (axum::http::HeaderName::try_from(k.as_str()), axum::http::HeaderValue::from_bytes(v.as_bytes())) {
                        builder = builder.header(hk, hv);
                    }
                }
            }

            builder.body(Body::from_stream(r.bytes_stream())).unwrap()
        }
        Err(e) => {
            eprintln!("🔴 [AIM-PROXY] Anthropic forward error: {}", e);
            axum::response::Response::builder()
                .status(502)
                .body(Body::from(format!("{{\"error\":\"aim-proxy forward failed: {}\"}}", e)))
                .unwrap()
        }
    }
}

// ──────────────────────────────────────────────────────────────
// OLLAMA INTERCEPTOR (unchanged logic, updated to use new cache)
// ──────────────────────────────────────────────────────────────
async fn intercept_ollama(
    State(state): State<AppState>,
    req: Request<Body>,
) -> impl IntoResponse {
    let (parts, body) = req.into_parts();

    let bytes = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(b) => b,
        Err(_) => return axum::response::Response::builder().status(500).body(Body::from("read error")).unwrap(),
    };

    let mut json_payload: Value = match serde_json::from_slice(&bytes) {
        Ok(v) => v,
        Err(_) => return axum::response::Response::builder().status(400).body(Body::from("invalid json")).unwrap(),
    };

    println!("🟢 [AIM-PROXY] Captured Ollama request");

    let aim_text = get_aim_text(&state).await;
    let gist_prefix = if aim_text.is_empty() {
        "[AIM-VFS]: No context. Run NeuralDrive.".to_string()
    } else {
        format!("[KORTEX_GIST_INJECTED]\n{}", &aim_text[..aim_text.len().min(40_000)])
    };

    // Legacy prompt field (generate endpoint)
    if let Some(prompt) = json_payload.get_mut("prompt").and_then(|p| p.as_str().map(|s| s.to_string())) {
        json_payload["prompt"] = json!(format!("{}\n\n{}", gist_prefix, prompt));
    }

    // Chat messages (chat endpoint)
    if let Some(messages) = json_payload.get_mut("messages").and_then(|m| m.as_array_mut()) {
        let already_injected = messages.first()
            .and_then(|m| m.get("content").and_then(|c| c.as_str()))
            .map(|c| c.contains("[KORTEX_GIST"))
            .unwrap_or(false);

        if !already_injected {
            messages.insert(0, json!({"role": "system", "content": gist_prefix}));
            println!("🟢 [AIM-PROXY] Injected AIM gist into Ollama messages[0]");
        }
    }

    let target_url = format!("{}{}", state.target_ollama, parts.uri.path_and_query().map(|pq| pq.as_str()).unwrap_or(""));
    let new_body = serde_json::to_vec(&json_payload).unwrap();

    match state.http_client.post(&target_url).body(new_body).send().await {
        Ok(resp) => {
            let mut builder = axum::response::Response::builder()
                .status(axum::http::StatusCode::from_u16(resp.status().as_u16()).unwrap_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR));
            for (k, v) in resp.headers() {
                if let (Ok(key), Ok(val)) = (axum::http::HeaderName::try_from(k.as_str()), axum::http::HeaderValue::from_bytes(v.as_bytes())) {
                    builder = builder.header(key, val);
                }
            }
            builder.body(Body::from_stream(resp.bytes_stream())).unwrap()
        }
        Err(e) => {
            axum::response::Response::builder().status(500).body(Body::from(format!("Proxy error: {}", e))).unwrap()
        }
    }
}

// ──────────────────────────────────────────────────────────────
// AIM KNOWLEDGE UPDATE — append new knowledge to .aim file
// Called by vscodium-rust IDE after significant AI work
// ──────────────────────────────────────────────────────────────
async fn handle_aim_update(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    let knowledge = payload.get("knowledge").and_then(|v| v.as_str()).unwrap_or("");
    let source = payload.get("source").and_then(|v| v.as_str()).unwrap_or("unknown");

    if knowledge.is_empty() {
        return Json(json!({"status": "error", "message": "No knowledge provided"}));
    }

    // Find the .aim file and append knowledge to the JSON header
    let paths = AimPaths::new();
    let aim_path = paths.0.iter().find(|p| p.exists()).cloned()
        .unwrap_or_else(|| PathBuf::from(".\\.aim\\memory.aim"));

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    // Append a knowledge entry — write to a companion .aim-knowledge.jsonl file
    // (avoids corrupting the binary .aim format)
    let knowledge_path = aim_path.parent()
        .unwrap_or(std::path::Path::new("."))
        .join("knowledge.jsonl");

    let entry = format!("{}\n", json!({"ts": timestamp, "source": source, "knowledge": knowledge}));
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&knowledge_path) {
        let _ = f.write_all(entry.as_bytes());
    }

    // Invalidate cache so next request reloads
    *state.aim_cache.write().await = None;

    println!("📝 [AIM-PROXY] Knowledge updated from '{}' ({} chars)", source, knowledge.len());
    Json(json!({"status": "ok", "source": source, "chars": knowledge.len()}))
}

async fn handle_aim_status(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let cache = state.aim_cache.read().await;
    if let Some(ref ctx) = *cache {
        Json(json!({
            "status": "loaded",
            "path": ctx.path.display().to_string(),
            "context_chars": ctx.text.len(),
            "context_kb": ctx.text.len() / 1024,
            "is_fresh": ctx.is_fresh()
        }))
    } else {
        Json(json!({"status": "no_context", "message": "Run NeuralDrive to index codebase"}))
    }
}

// ──────────────────────────────────────────────────────────────
// PASSTHROUGH — everything else goes to Ollama
// ──────────────────────────────────────────────────────────────
async fn pass_through(
    State(state): State<AppState>,
    req: Request<Body>,
) -> impl IntoResponse {
    let method = req.method().clone();
    let path: String = req.uri().path_and_query().map(|pq| pq.as_str()).unwrap_or("").to_string();
    let target_url = format!("{}{}", state.target_ollama, path);
    println!("🔍 [AIM-PROXY] Passthrough: {} {} → {}", method, path, target_url);

    let (parts, body) = req.into_parts();
    let bytes = if parts.method == Method::GET || parts.method == Method::HEAD {
        None
    } else {
        axum::body::to_bytes(body, 10 * 1024 * 1024).await.ok()
    };

    let method_str = parts.method.as_str();
    let fwd_method: reqwest::Method = method_str.parse().unwrap_or(reqwest::Method::GET);
    let mut forward_req = state.http_client.request(fwd_method, &target_url);

    for (k, v) in parts.headers.iter() {
        if k.as_str() != "host" && k.as_str() != "content-length" {
            if let (Ok(key), Ok(val)) = (reqwest::header::HeaderName::try_from(k.as_str()), reqwest::header::HeaderValue::from_bytes(v.as_bytes())) {
                forward_req = forward_req.header(key, val);
            }
        }
    }
    if let Some(b) = bytes { forward_req = forward_req.body(b); }

    match forward_req.send().await {
        Ok(resp) => {
            let mut builder = axum::response::Response::builder()
                .status(axum::http::StatusCode::from_u16(resp.status().as_u16()).unwrap_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR));
            for (k, v) in resp.headers() {
                if k.as_str() != "content-length" && k.as_str() != "transfer-encoding" && k.as_str() != "content-encoding" {
                    if let (Ok(key), Ok(val)) = (axum::http::HeaderName::try_from(k.as_str()), axum::http::HeaderValue::from_bytes(v.as_bytes())) {
                        builder = builder.header(key, val);
                    }
                }
            }
            builder.body(Body::from_stream(resp.bytes_stream())).unwrap()
        }
        Err(e) => {
            axum::response::Response::builder().status(502).body(Body::from(format!("Passthrough error: {}", e))).unwrap()
        }
    }
}

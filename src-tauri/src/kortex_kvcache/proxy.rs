//! HTTP proxy that fronts llama-server with disk-persistent prefix-cache reuse.
//!
//! Listens on `proxy_host:proxy_port`, forwards everything to `upstream_url`.
//! Two endpoints get special treatment:
//!
//!   POST /v1/chat/completions   — render messages, tokenize, prefix-match,
//!                                 restore slot, forward, save on completion.
//!   POST /v1/completions        — same flow with raw `prompt`.
//!
//! Anything else is a transparent reverse-proxy. SSE streams from upstream
//! are passed through byte-for-byte; we save the slot after stream end.

use anyhow::Result;
use axum::body::Body;
use axum::extract::State;
use axum::http::{HeaderMap, HeaderValue, Method, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::routing::{any, post};
use axum::Router;
use bytes::Bytes;
use futures::StreamExt;
use serde_json::Value;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use super::llamacpp::LlamaCppClient;
use super::store::{plan_save_count, sha256_tokens_hex, slotbin_path, CacheStore};
use super::types::{KvCacheEntry, KvCacheOptions, KvCacheStats, SaveReason};

/// Snapshot of the most recent successful request, used by the shutdown flush
/// to persist whatever the live session ended up at. Mirrors ds4's "we always
/// know which prefix is live so we can save it on KV_REASON_SHUTDOWN" pattern.
#[derive(Debug, Clone, Default)]
pub struct LiveSession {
    pub tokens: Vec<u32>,
    pub prefix_text: String,
}

/// Shared proxy state. One per running proxy.
pub struct ProxyState {
    pub opts: KvCacheOptions,
    pub client: LlamaCppClient,
    pub store: Mutex<CacheStore>,
    pub http: reqwest::Client,
    pub shutdown_tx: tokio::sync::Mutex<Option<tokio::sync::oneshot::Sender<()>>>,
    /// Most recent (tokens, prefix_text) that flowed through a successful
    /// chat/completion. `flush_shutdown_checkpoint` reads this to persist a
    /// final snapshot on clean shutdown.
    pub live_session: Mutex<Option<LiveSession>>,
    /// Concrete caching tier resolved at startup (never `Auto`). Requests
    /// branch on this: `Kv` runs the KDKVC prefix path; anything else is a
    /// transparent passthrough until the response cache (Tier 2) ships.
    pub resolved_tier: super::types::CacheTier,
}

impl ProxyState {
    pub fn new(opts: KvCacheOptions, store: CacheStore, resolved_tier: super::types::CacheTier) -> Self {
        let client = LlamaCppClient::new(&opts.upstream_url, opts.slot_id);
        let http = reqwest::Client::builder()
            .pool_max_idle_per_host(8)
            // Long timeout: LLM responses can take minutes for big prompts.
            .timeout(std::time::Duration::from_secs(600))
            .build()
            .expect("reqwest client build");
        Self {
            opts,
            client,
            store: Mutex::new(store),
            http,
            shutdown_tx: tokio::sync::Mutex::new(None),
            live_session: Mutex::new(None),
            resolved_tier,
        }
    }

    pub async fn current_stats(&self) -> KvCacheStats {
        let s = self.store.lock().await;
        s.stats()
    }

    /// Remember the token stream the most recent request operated on. The
    /// shutdown flush uses this to persist a final snapshot before exit.
    pub async fn note_live_session(&self, tokens: Vec<u32>, prefix_text: String) {
        let mut g = self.live_session.lock().await;
        *g = Some(LiveSession {
            tokens,
            prefix_text,
        });
    }
}

pub type SharedProxy = Arc<ProxyState>;

/// Spawn the proxy on its configured host:port. Returns a handle that, when
/// awaited, signals shutdown via oneshot.
pub async fn serve(state: SharedProxy) -> Result<tokio::task::JoinHandle<()>> {
    let addr: SocketAddr = format!("{}:{}", state.opts.proxy_host, state.opts.proxy_port).parse()?;
    let listener = TcpListener::bind(&addr).await?;

    let app = Router::new()
        .route("/v1/chat/completions", post(handle_chat_completions))
        .route("/v1/completions", post(handle_completions))
        .fallback(any(handle_passthrough))
        .with_state(state.clone());

    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    *state.shutdown_tx.lock().await = Some(tx);

    let handle = tokio::spawn(async move {
        let server = axum::serve(listener, app)
            .with_graceful_shutdown(async move {
                let _ = rx.await;
            });
        if let Err(e) = server.await {
            tracing::error!("[kortex-kvcache] proxy server error: {}", e);
        }
    });
    tracing::info!(
        "[kortex-kvcache] proxy listening on http://{} → {}",
        addr,
        state.opts.upstream_url
    );
    Ok(handle)
}

pub async fn shutdown(state: &SharedProxy) {
    if let Some(tx) = state.shutdown_tx.lock().await.take() {
        let _ = tx.send(());
    }
}

/// Persist the live session as `SaveReason::Shutdown` if one exists. Called
/// from `kortex_kvcache_stop` before the proxy is torn down, so a clean
/// restart can resume the latest in-flight conversation rather than starting
/// cold. Idempotent: if the live prefix is already cached we skip the save.
pub async fn flush_shutdown_checkpoint(state: &SharedProxy) -> Result<()> {
    let snapshot = {
        let g = state.live_session.lock().await;
        g.clone()
    };
    let Some(LiveSession { tokens, prefix_text }) = snapshot else {
        return Ok(());
    };
    let n = tokens.len() as u32;
    // Shutdown flush persists even long sessions → don't enforce cold_max.
    // Same gate/align helper as the normal save path so a shutdown entry is
    // interchangeable with the cold/continued ones (matchable at the same SHA).
    let Some(save_count) = plan_save_count(&state.opts, n, false) else {
        return Ok(());
    };
    let save_tokens = &tokens[..save_count as usize];
    let sha = sha256_tokens_hex(save_tokens);

    if state.store.lock().await.contains(&sha) {
        return Ok(());
    }
    let filename = format!("{}.slotbin", sha);
    if let Err(e) = state.client.save_slot(&filename).await {
        return Err(anyhow::anyhow!("shutdown save_slot failed: {}", e));
    }
    let slotbin = slotbin_path(&state.opts, &sha);
    let slotbin_size = std::fs::metadata(&slotbin).map(|m| m.len()).unwrap_or(0);
    let entry = KvCacheEntry {
        sha: sha.clone(),
        prefix_token_count: save_count,
        ctx_size: 0,
        created_at: KvCacheEntry::now_unix(),
        last_used_at: KvCacheEntry::now_unix(),
        hit_count: 0,
        slotbin_path: slotbin.to_string_lossy().into_owned(),
        slotbin_size,
        rendered_text: prefix_text.chars().take(2048).collect(),
        model: state.opts.model.clone(),
        save_reason: SaveReason::Shutdown,
    };
    let mut s = state.store.lock().await;
    s.put_with_reason(entry, SaveReason::Shutdown)
        .map_err(|e| anyhow::anyhow!("shutdown put failed: {}", e))?;
    tracing::info!(
        "[kortex-kvcache] SHUTDOWN flushed sha={} ({} tokens)",
        &sha[..8],
        save_count
    );
    Ok(())
}

// ─────────────────────── handlers ───────────────────────────────────────────

async fn handle_chat_completions(
    State(state): State<SharedProxy>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    handle_intercepted(state, headers, body, IntercepKind::Chat).await
}

async fn handle_completions(
    State(state): State<SharedProxy>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    handle_intercepted(state, headers, body, IntercepKind::Completion).await
}

#[derive(Debug, Clone, Copy)]
enum IntercepKind {
    Chat,
    Completion,
}

async fn handle_intercepted(
    state: SharedProxy,
    headers: HeaderMap,
    body: Bytes,
    kind: IntercepKind,
) -> Response {
    // Tier gate: KV-slot reuse only runs when the upstream was detected to
    // support llama.cpp slot save/restore. For `Response` (no slot API — Tier 2
    // not yet implemented) and `Off`, this is a transparent passthrough so the
    // proxy is always safe to sit in front of any OpenAI-compatible server.
    if state.resolved_tier != super::types::CacheTier::Kv {
        let path = match kind {
            IntercepKind::Chat => "/v1/chat/completions",
            IntercepKind::Completion => "/v1/completions",
        };
        return forward_raw(&state, headers, body, path).await;
    }

    // Parse the body as JSON. If it doesn't parse, just transparently forward.
    let parsed: Value = match serde_json::from_slice(&body) {
        Ok(v) => v,
        Err(_) => {
            return forward_raw(&state, headers, body, "/v1/chat/completions").await;
        }
    };

    // Extract the prefix text. Best-effort — if we can't, we just forward and
    // skip caching for this request.
    let (prefix_text, is_stream) = match kind {
        IntercepKind::Chat => (
            extract_chat_prefix_text(&parsed),
            parsed.get("stream").and_then(|v| v.as_bool()).unwrap_or(false),
        ),
        IntercepKind::Completion => (
            parsed.get("prompt").and_then(|v| v.as_str()).map(String::from),
            parsed.get("stream").and_then(|v| v.as_bool()).unwrap_or(false),
        ),
    };

    let upstream_path = match kind {
        IntercepKind::Chat => "/v1/chat/completions",
        IntercepKind::Completion => "/v1/completions",
    };

    let prefix_text = match prefix_text {
        Some(s) if !s.is_empty() => s,
        _ => return forward_raw(&state, headers, body, upstream_path).await,
    };

    // Tokenise the prefix so we can apply ds4-style boundary alignment.
    let tokens = match state.client.tokenize(&prefix_text).await {
        Ok(t) => t,
        Err(e) => {
            tracing::warn!("[kortex-kvcache] tokenize failed: {}", e);
            return forward_raw(&state, headers, body, upstream_path).await;
        }
    };

    // Try a longest-prefix match.
    let prefix_hit = {
        let store = state.store.lock().await;
        store.longest_prefix(&tokens)
    };

    if let Some(m) = prefix_hit.clone() {
        // Restore the slot before forwarding. If restore fails, fall through
        // to a normal request — llama-server will just prefill from scratch.
        let filename = format!("{}.slotbin", m.sha);
        match state.client.restore_slot(&filename).await {
            Ok(r) => {
                let n_restored = r.n_restored.unwrap_or(m.prefix_token_count);
                tracing::info!(
                    "[kortex-kvcache] HIT sha={} ({} tokens restored)",
                    &m.sha[..8],
                    n_restored
                );
                let mut store = state.store.lock().await;
                let _ = store.touch(&m.sha);
                store.record_skipped_tokens(n_restored);
            }
            Err(e) => {
                tracing::warn!("[kortex-kvcache] slot restore failed for {}: {}", &m.sha[..8], e);
                state.store.lock().await.record_miss();
            }
        }
    } else {
        tracing::info!("[kortex-kvcache] MISS ({} tokens)", tokens.len());
        state.store.lock().await.record_miss();
    }

    let _ = is_stream; // streaming vs non-streaming both go through the same path

    // Forward the original body unmodified — llama-server's cache_prompt
    // logic detects the restored slot's prefix automatically and skips
    // re-prefilling the matching tokens.
    forward_raw_with_callback(
        &state,
        headers,
        body,
        upstream_path,
        Some(SaveAfterStream {
            tokens: Arc::new(tokens),
            prefix_text: Arc::new(prefix_text),
        }),
    )
    .await
}

/// Information needed to do a slot save once the upstream response is done.
/// The proxy state itself (Arc<ProxyState>) is passed alongside this struct
/// so the post-stream task can lock the store without copying it.
struct SaveAfterStream {
    tokens: Arc<Vec<u32>>,
    prefix_text: Arc<String>,
}

async fn forward_raw(
    state: &SharedProxy,
    headers: HeaderMap,
    body: Bytes,
    upstream_path: &str,
) -> Response {
    forward_raw_with_callback(state, headers, body, upstream_path, None).await
}

async fn forward_raw_with_callback(
    state: &SharedProxy,
    mut headers: HeaderMap,
    body: Bytes,
    upstream_path: &str,
    save_after: Option<SaveAfterStream>,
) -> Response {
    let url = format!("{}{}", state.opts.upstream_url.trim_end_matches('/'), upstream_path);

    // Strip headers that confuse upstream (Host, Content-Length, hop-by-hop).
    headers.remove("host");
    headers.remove("content-length");
    headers.remove("connection");
    headers.remove("keep-alive");

    let mut req = state.http.post(&url).body(body.clone());
    for (k, v) in headers.iter() {
        if let Ok(name) = reqwest::header::HeaderName::from_bytes(k.as_str().as_bytes()) {
            if let Ok(val) = reqwest::header::HeaderValue::from_bytes(v.as_bytes()) {
                req = req.header(name, val);
            }
        }
    }

    let upstream = match req.send().await {
        Ok(r) => r,
        Err(e) => {
            return (
                StatusCode::BAD_GATEWAY,
                format!("upstream error: {}", e),
            )
                .into_response();
        }
    };

    let status = StatusCode::from_u16(upstream.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
    let mut out_headers = HeaderMap::new();
    for (k, v) in upstream.headers().iter() {
        if let Ok(name) = axum::http::HeaderName::from_bytes(k.as_str().as_bytes()) {
            if let Ok(val) = HeaderValue::from_bytes(v.as_bytes()) {
                out_headers.insert(name, val);
            }
        }
    }

    let bytes_stream = upstream.bytes_stream();

    // If we have a save callback, wire it up to fire after the stream ends.
    let body = if let Some(save) = save_after {
        let state_inner = state.clone();
        let chained = bytes_stream
            .map(|r| r.map_err(std::io::Error::other))
            .chain(futures::stream::once(async move {
                spawn_save_after(state_inner, save).await;
                Ok::<Bytes, std::io::Error>(Bytes::new())
            }));
        Body::from_stream(chained)
    } else {
        Body::from_stream(bytes_stream.map(|r| r.map_err(std::io::Error::other)))
    };

    let mut resp = Response::builder().status(status);
    for (k, v) in out_headers.iter() {
        resp = resp.header(k, v);
    }
    resp.body(body)
        .unwrap_or_else(|_| (StatusCode::BAD_GATEWAY, "build failed").into_response())
}

async fn spawn_save_after(state: SharedProxy, save: SaveAfterStream) {
    let SaveAfterStream { tokens, prefix_text } = save;

    // Record this as the live session so a clean shutdown can flush it even
    // if min/max token gates below cause us to skip the regular save. The
    // shutdown flush re-applies the same gates, so this is just bookkeeping
    // for "what was the most recent thing the proxy saw?".
    state
        .note_live_session((*tokens).clone(), (*prefix_text).clone())
        .await;

    let n = tokens.len() as u32;
    // Normal cold/continued save path enforces cold_max (skip giant one-shots).
    let Some(save_count) = plan_save_count(&state.opts, n, true) else {
        return;
    };
    let save_tokens = &tokens[..save_count as usize];
    let sha = sha256_tokens_hex(save_tokens);

    // Skip if the prefix is already cached.
    {
        let s = state.store.lock().await;
        if s.contains(&sha) {
            return;
        }
    }

    // Save the slot. llama-server writes the file into its --slot-save-path
    // directory, which the IDE configures to match `state.opts.slot_dir`.
    let filename = format!("{}.slotbin", sha);
    if let Err(e) = state.client.save_slot(&filename).await {
        tracing::warn!("[kortex-kvcache] slot save failed: {}", e);
        return;
    }

    let slotbin = slotbin_path(&state.opts, &sha);
    let slotbin_size = std::fs::metadata(&slotbin).map(|m| m.len()).unwrap_or(0);

    let entry = KvCacheEntry {
        sha: sha.clone(),
        prefix_token_count: save_count,
        ctx_size: 0,
        created_at: KvCacheEntry::now_unix(),
        last_used_at: KvCacheEntry::now_unix(),
        hit_count: 0,
        slotbin_path: slotbin.to_string_lossy().into_owned(),
        slotbin_size,
        rendered_text: prefix_text.chars().take(2048).collect(),
        model: state.opts.model.clone(),
        save_reason: SaveReason::Cold,
    };

    let mut s = state.store.lock().await;
    if let Err(e) = s.put_with_reason(entry, SaveReason::Cold) {
        tracing::warn!("[kortex-kvcache] put failed: {}", e);
    } else {
        tracing::info!(
            "[kortex-kvcache] SAVE sha={} ({} tokens, {:.1} MB)",
            &sha[..8],
            save_count,
            slotbin_size as f64 / (1024.0 * 1024.0)
        );
    }
}

async fn handle_passthrough(
    State(state): State<SharedProxy>,
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    let path_and_query = uri.path_and_query().map(|p| p.as_str()).unwrap_or("/");
    let url = format!(
        "{}{}",
        state.opts.upstream_url.trim_end_matches('/'),
        path_and_query
    );

    let reqwest_method = reqwest::Method::from_bytes(method.as_str().as_bytes())
        .unwrap_or(reqwest::Method::GET);
    let mut req = state.http.request(reqwest_method, &url).body(body);
    for (k, v) in headers.iter() {
        if k == "host" || k == "content-length" || k == "connection" {
            continue;
        }
        if let Ok(name) = reqwest::header::HeaderName::from_bytes(k.as_str().as_bytes()) {
            if let Ok(val) = reqwest::header::HeaderValue::from_bytes(v.as_bytes()) {
                req = req.header(name, val);
            }
        }
    }
    let upstream = match req.send().await {
        Ok(r) => r,
        Err(e) => return (StatusCode::BAD_GATEWAY, format!("upstream: {}", e)).into_response(),
    };

    let status = StatusCode::from_u16(upstream.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
    let mut out_headers = HeaderMap::new();
    for (k, v) in upstream.headers().iter() {
        if let Ok(name) = axum::http::HeaderName::from_bytes(k.as_str().as_bytes()) {
            if let Ok(val) = HeaderValue::from_bytes(v.as_bytes()) {
                out_headers.insert(name, val);
            }
        }
    }
    let stream = upstream
        .bytes_stream()
        .map(|r| r.map_err(std::io::Error::other));
    let body = Body::from_stream(stream);
    let mut resp = Response::builder().status(status);
    for (k, v) in out_headers.iter() {
        resp = resp.header(k, v);
    }
    resp.body(body).unwrap_or_else(|_| {
        (StatusCode::BAD_GATEWAY, "build failed").into_response()
    })
}

// ───────────────────── helpers ─────────────────────────────────────────────

/// Render the chat messages to a deterministic "prefix text" we can tokenize.
///
/// This is intentionally not the model's exact chat template — we'd have to
/// pull and run a Jinja-like template from the GGUF metadata for that. Instead
/// we render to a canonical, role-tagged form that llama-server's tokenizer
/// can chew through. Cache hits land on identical message arrays, which is
/// exactly the coding-agent / Claude-Code repeat-prompt case.
fn extract_chat_prefix_text(body: &Value) -> Option<String> {
    let messages = body.get("messages")?.as_array()?;
    if messages.is_empty() {
        return None;
    }
    // Drop the trailing message — that's the new user turn we want to be the
    // "suffix" so the rest of the conversation is the cache key.
    let take = messages.len().saturating_sub(1);
    if take == 0 {
        return None;
    }
    let mut s = String::new();
    for m in messages.iter().take(take) {
        let role = m.get("role").and_then(|v| v.as_str()).unwrap_or("user");
        let content = m.get("content").and_then(|v| v.as_str()).unwrap_or("");
        s.push('<');
        s.push_str(role);
        s.push('>');
        s.push_str(content);
        s.push('<');
        s.push_str("/");
        s.push_str(role);
        s.push('>');
        s.push('\n');
    }
    Some(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn extract_chat_prefix_returns_none_when_messages_field_missing() {
        let body = json!({ "model": "test" });
        assert!(extract_chat_prefix_text(&body).is_none());
    }

    #[test]
    fn extract_chat_prefix_returns_none_for_empty_messages() {
        let body = json!({ "messages": [] });
        assert!(extract_chat_prefix_text(&body).is_none());
    }

    #[test]
    fn extract_chat_prefix_returns_none_when_only_one_message() {
        // Only a single user turn -> nothing to use as a prefix; the proxy
        // must short-circuit and forward as-is.
        let body = json!({ "messages": [{"role": "user", "content": "hi"}] });
        assert!(extract_chat_prefix_text(&body).is_none());
    }

    #[test]
    fn extract_chat_prefix_drops_trailing_user_turn() {
        let body = json!({
            "messages": [
                {"role": "system", "content": "you are helpful"},
                {"role": "user",   "content": "first question"},
                {"role": "assistant", "content": "first answer"},
                {"role": "user",   "content": "follow-up — this must NOT be in the prefix"},
            ]
        });
        let prefix = extract_chat_prefix_text(&body).expect("must produce a prefix");
        assert!(prefix.contains("<system>you are helpful</system>"));
        assert!(prefix.contains("<user>first question</user>"));
        assert!(prefix.contains("<assistant>first answer</assistant>"));
        // The trailing user turn must be excluded — it's the suffix.
        assert!(!prefix.contains("follow-up"));
    }

    #[test]
    fn extract_chat_prefix_tolerates_missing_role_or_content() {
        let body = json!({
            "messages": [
                {"content": "no role"},
                {"role": "assistant"},
                {"role": "user", "content": "ignore me"},
            ]
        });
        let prefix = extract_chat_prefix_text(&body).expect("should still produce a prefix");
        // Default role for the first message is "user"; second has empty content.
        assert!(prefix.contains("<user>no role</user>"));
        assert!(prefix.contains("<assistant></assistant>"));
        // Trailing message dropped.
        assert!(!prefix.contains("ignore me"));
    }
}

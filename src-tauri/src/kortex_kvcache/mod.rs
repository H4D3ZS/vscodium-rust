//! Kortex Disk KV Cache (KDKVC) — antirez/ds4-style prefix-persistent KV reuse
//! layered over llama.cpp's slot-save API.
//!
//! What it gives you:
//!   * Coding agents (Claude Code, Cursor, opencode) that resend the same
//!     20–30 K-token system prompt every turn skip the prefill entirely.
//!   * Restarting llama-server doesn't lose the cache — it's on disk.
//!   * Bounded by an LRU on a configurable byte budget.
//!
//! How it ships into the project:
//!   1. Caller starts llama-server with `--slot-save-path <slot_dir>`.
//!   2. `kortex_kvcache_start` boots an axum proxy on `proxy_port` that
//!      forwards to the llama-server `upstream_url`.
//!   3. IDE clients point at the proxy URL instead of the raw upstream.

pub mod llamacpp;
pub mod proxy;
pub mod store;
pub mod types;

use std::sync::{Arc, Mutex};
use tauri::command;

pub use llamacpp::{LlamaCppClient, ServerProps};
pub use store::{align_save_count, sha256_tokens_hex, slotbin_path, CacheStore};
pub use types::{
    KvCacheEntry, KvCacheOptions, KvCacheStats, ModelIdentity, ModelMatchPolicy,
    PrefixMatch, RoutingTrace, SaveReason,
};

/// Single global proxy slot. We hold a sync Mutex<Option<Arc<...>>> so we can
/// take + clone the Arc out cheaply across async boundaries (the proxy itself
/// uses tokio mutexes internally for its hot data).
static PROXY: Mutex<Option<Arc<proxy::ProxyState>>> = Mutex::new(None);

fn current_proxy() -> Option<Arc<proxy::ProxyState>> {
    PROXY.lock().ok().and_then(|g| g.clone())
}

fn set_proxy(state: Option<Arc<proxy::ProxyState>>) {
    if let Ok(mut g) = PROXY.lock() {
        *g = state;
    }
}

#[command]
pub async fn kortex_kvcache_start(mut opts: KvCacheOptions) -> Result<u16, String> {
    if current_proxy().is_some() {
        return Err("kvcache proxy already running; stop it first".into());
    }
    // If the caller didn't pre-stamp the model identity, derive it from the
    // upstream server's /props now. This is the load-time gate that prevents
    // a cache populated by model A from being served back to model B.
    if opts.model.model_id.is_empty() {
        let probe = LlamaCppClient::new(opts.upstream_url.clone(), opts.slot_id);
        match probe.props().await {
            Ok(props) => {
                opts.model = props.derive_identity();
            }
            Err(e) => {
                // /props isn't catastrophic — older llama.cpp builds may not
                // ship it. Fall back to upstream URL as a soft identity so we
                // at least segregate caches by server endpoint.
                eprintln!(
                    "[kortex_kvcache] /props probe failed ({}); falling back to upstream URL as model_id",
                    e
                );
                opts.model = ModelIdentity {
                    model_id: opts.upstream_url.clone(),
                    tokenizer_hash: String::new(),
                    quant_signature: String::new(),
                };
            }
        }
    }
    let store = CacheStore::open(opts.clone()).map_err(|e| e.to_string())?;
    let port = opts.proxy_port;
    let state = Arc::new(proxy::ProxyState::new(opts, store));

    // Smoke-check the upstream once before binding the listener; we want
    // launch errors to surface here, not as a 502 on first request.
    if let Err(e) = state.client.health().await {
        return Err(format!("upstream not healthy: {}", e));
    }

    if let Err(e) = proxy::serve(state.clone()).await {
        return Err(e.to_string());
    }
    // Mirror the bound model into the .aim neural VFS so the durable side
    // of Kortex knows which model the cache is currently locked to. This is
    // what `aim_telemetry_snapshot` surfaces in the Kortex panel.
    let _ = crate::kortex_commands::aim_set_bound_model(
        state.opts.model.model_id.clone(),
        if state.opts.model.quant_signature.is_empty() {
            None
        } else {
            Some(state.opts.model.quant_signature.clone())
        },
        if state.opts.model.tokenizer_hash.is_empty() {
            None
        } else {
            Some(state.opts.model.tokenizer_hash.clone())
        },
    )
    .await;
    set_proxy(Some(state));
    Ok(port)
}

#[command]
pub async fn kortex_kvcache_stop() -> Result<(), String> {
    if let Some(state) = current_proxy() {
        // Flush a shutdown checkpoint of the live in-memory session before
        // we tear down the proxy. ds4 calls this `KV_REASON_SHUTDOWN`; it's
        // the reason a clean stop survives a server restart with full cache
        // continuity. Failures are non-fatal — we still tear down.
        if let Err(e) = proxy::flush_shutdown_checkpoint(&state).await {
            eprintln!("[kortex_kvcache] shutdown flush failed: {}", e);
        }
        proxy::shutdown(&state).await;
        set_proxy(None);
    }
    // Flush any unsaved telemetry samples to the .aim neural VFS so we don't
    // lose the last few completions when the stack tears down cleanly.
    let _ = crate::kortex_commands::aim_flush_telemetry().await;
    Ok(())
}

#[command]
pub async fn kortex_kvcache_stats() -> Result<KvCacheStats, String> {
    if let Some(state) = current_proxy() {
        Ok(state.current_stats().await)
    } else {
        Ok(KvCacheStats::default())
    }
}

#[command]
pub async fn kortex_kvcache_status() -> Result<Option<RunningCacheInfo>, String> {
    Ok(current_proxy().map(|state| RunningCacheInfo {
        proxy_url: format!("http://{}:{}", state.opts.proxy_host, state.opts.proxy_port),
        upstream_url: state.opts.upstream_url.clone(),
        index_dir: state.opts.index_dir.to_string_lossy().into_owned(),
        slot_dir: state.opts.slot_dir.to_string_lossy().into_owned(),
        max_bytes: state.opts.max_bytes,
        slot_id: state.opts.slot_id,
        model_id: state.opts.model.model_id.clone(),
        quant_signature: state.opts.model.quant_signature.clone(),
        tokenizer_hash: state.opts.model.tokenizer_hash.clone(),
    }))
}

#[command]
pub async fn kortex_kvcache_clear() -> Result<u32, String> {
    let Some(state) = current_proxy() else {
        return Ok(0);
    };
    let n = {
        let store = state.store.lock().await;
        store.entries_iter().count() as u32
    };
    let index_dir = state.opts.index_dir.clone();
    let slot_dir = state.opts.slot_dir.clone();
    let _ = std::fs::remove_dir_all(&index_dir);
    let _ = std::fs::remove_dir_all(&slot_dir);
    let _ = std::fs::create_dir_all(&index_dir);
    let _ = std::fs::create_dir_all(&slot_dir);
    let mut store = state.store.lock().await;
    store.reload_index().map_err(|e| e.to_string())?;
    Ok(n)
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct RunningCacheInfo {
    pub proxy_url: String,
    pub upstream_url: String,
    pub index_dir: String,
    pub slot_dir: String,
    pub max_bytes: u64,
    pub slot_id: u32,
    /// Model identity the proxy currently binds saved entries to. Surfaced so
    /// the IDE can show a "cache bound to <model>" indicator.
    pub model_id: String,
    pub quant_signature: String,
    pub tokenizer_hash: String,
}

//! Renderer-agnostic event sink.
//!
//! The engine used to emit UI events through `tauri::AppHandle` directly, which
//! hard-wired it to the WebView shell. To let a native (gpui) shell drive the
//! same engine, all engine-side emits go through this trait instead. The Tauri
//! shell provides `TauriSink`; a native shell provides its own (often a no-op,
//! since the engine also exposes pollable `Arc<Mutex<…>>` buffers —
//! `activity_log`, `chat_stream_buf`, `pending_proposals`).

use serde_json::Value;
use std::future::Future;

/// Spawn a detached background future that is safe to call even when no tokio
/// runtime is in context (e.g. during the synchronous `EditorState::new` /
/// `Sentient::new` construction path, before the Tauri/gpui runtime is up).
///
/// `tauri::async_runtime::spawn` tolerated a missing ambient runtime; bare
/// `tokio::spawn` panics with "there is no reactor running". This restores the
/// old tolerance: spawn on the current runtime if one exists, otherwise stand
/// up a dedicated std::thread with its own current-thread runtime.
pub fn spawn_detached<F>(fut: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    match tokio::runtime::Handle::try_current() {
        Ok(handle) => {
            handle.spawn(fut);
        }
        Err(_) => {
            std::thread::Builder::new()
                .name("detached-task".into())
                .spawn(move || {
                    if let Ok(rt) = tokio::runtime::Builder::new_current_thread()
                        .enable_all()
                        .build()
                    {
                        rt.block_on(fut);
                    }
                })
                .ok();
        }
    }
}

/// Fire-and-forget UI event emission. Implementations must be cheap and
/// non-blocking; a failed emit is swallowed (the engine never depends on it).
pub trait EventSink: Send + Sync {
    fn emit(&self, event: &str, payload: Value);
}

/// Convenience for an optional sink held behind a lock: emit only if set.
impl<T: EventSink + ?Sized> EventSink for std::sync::Arc<T> {
    fn emit(&self, event: &str, payload: Value) {
        (**self).emit(event, payload)
    }
}

/// Tauri-backed sink: forwards engine events to the WebView via the AppHandle.
#[cfg(feature = "tauri")]
pub struct TauriSink(pub tauri::AppHandle);

#[cfg(feature = "tauri")]
impl EventSink for TauriSink {
    fn emit(&self, event: &str, payload: Value) {
        use tauri::Emitter;
        let _ = self.0.emit(event, payload);
    }
}

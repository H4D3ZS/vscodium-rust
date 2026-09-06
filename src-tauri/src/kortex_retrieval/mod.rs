//! In-process kortex retrieval proxy — hosts the aim-proxy router inside the
//! Tauri app so the IDE owns the lifecycle (start/stop/health) with no
//! external process or PowerShell script needed.
//!
//! On start the module binds `127.0.0.1:1536`, loads the `.aim` catalog from
//! the workspace, and forwards every AI request through the retrieval-augmented
//! proxy to Lemonade (`:13305`).
//! Retrieval degrades safely (missing catalog / conversational query / latency
//! budget → forward unchanged).

use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use tokio::sync::oneshot;

/// Global proxy state — one at a time, guarded by a plain Mutex (cheap, infrequent).
static PROXY: Mutex<Option<ProxyState>> = Mutex::new(None);

struct ProxyState {
    port: u16,
    shutdown_tx: oneshot::Sender<()>,
}

#[derive(Serialize, Deserialize)]
pub struct RetrievalStatus {
    pub running: bool,
    pub port: Option<u16>,
}

fn cyber_ifrit_dir() -> Option<std::path::PathBuf> {
    std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .ok()
        .map(|h| std::path::PathBuf::from(h).join(".cyber-ifrit"))
}

/// Default .aim catalog path — checks project-local, then user home.
fn default_catalog_path() -> Option<std::path::PathBuf> {
    // Project-local: .aim/memory.aim
    if let Ok(cwd) = std::env::current_dir() {
        let p = cwd.join(".aim").join("memory.aim");
        if p.exists() { return Some(p); }
    }
    // User home: ~/.cyber-ifrit/.aim/memory.aim
    cyber_ifrit_dir().map(|d| d.join(".aim").join("memory.aim"))
}

/// Start the retrieval proxy in-process. Binds 127.0.0.1:PORT, loads the
/// catalog, and starts forwarding to Lemonade.
#[tauri::command]
pub async fn kortex_retrieval_start(app: AppHandle) -> Result<u16, String> {
    {
        let guard = PROXY.lock().unwrap();
        if let Some(state) = guard.as_ref() {
            return Err(format!("Retrieval proxy already running on :{}", state.port));
        }
    }

    let port: u16 = 1536;

    // Build the router from aim-proxy's lib.
    let router_state = aim_proxy::server::AppState::from_env();
    let router = aim_proxy::server::build_router(router_state);

    // Bind and serve with graceful shutdown.
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| format!("bind {addr}: {e}"))?;

    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    tokio::spawn(async move {
        let graceful = axum::serve(listener, router)
            .with_graceful_shutdown(async move {
                let _ = shutdown_rx.await;
            });
        if let Err(e) = graceful.await {
            eprintln!("[kortex-retrieval] server error: {e}");
        }
    });

    // Log startup diagnostics.
    let catalog = default_catalog_path();
    let catalog_status = match &catalog {
        Some(p) => format!("catalog: {}", p.display()),
        None => "no catalog found".to_string(),
    };
    let lemonade = std::env::var("KORTEX_UPSTREAM_OPENAI")
        .unwrap_or_else(|_| "http://localhost:13305".into());
    let native_api = std::env::var("KORTEX_UPSTREAM_NATIVE_API")
        .unwrap_or_else(|_| "http://127.0.0.1:13305".into());

    println!("[kortex-retrieval] listening on http://127.0.0.1:{port}");
    println!("[kortex-retrieval] {catalog_status}");
    println!("[kortex-retrieval] lemonade   -> {lemonade}");
    println!("[kortex-retrieval] native-api -> {native_api}");

    *PROXY.lock().unwrap() = Some(ProxyState { port, shutdown_tx });

    // Update IDE AI provider to route through the proxy when available.
    let _ = app.emit("kortex:retrieval-status", serde_json::json!({
        "running": true,
        "port": port,
    }));

    Ok(port)
}

/// Stop the retrieval proxy gracefully.
#[tauri::command]
pub async fn kortex_retrieval_stop() -> Result<(), String> {
    let state = PROXY.lock().unwrap().take()
        .ok_or("Retrieval proxy is not running")?;
    let _ = state.shutdown_tx.send(());
    println!("[kortex-retrieval] stopped");
    Ok(())
}

/// Query current status.
#[tauri::command]
pub async fn kortex_retrieval_status() -> Result<RetrievalStatus, String> {
    let guard = PROXY.lock().unwrap();
    Ok(RetrievalStatus {
        running: guard.is_some(),
        port: guard.as_ref().map(|s| s.port),
    })
}

//! In-process kortex retrieval proxy — hosts the aim-proxy router inside the
//! Tauri app so the IDE owns the lifecycle (start/stop/health).
//!
//! On start it:
//!   1. resolves the workspace root and the local embedding backend,
//!   2. builds (or reuses) a **libaim catalog** at `<root>/.aim/kortex/`
//!      using the dense HTTP embedder the IDE already runs
//!      (`Qwen3-Embedding-0.6B-GGUF` on Lemonade),
//!   3. points `aim-proxy` at that catalog + workspace + upstream via the
//!      `KORTEX_*` env vars, then binds `127.0.0.1:1536`.
//!
//! Every AI request through `:1536` gets the last user turn embedded, the
//! catalog searched, and only the chunks that clear the gate prepended — so the
//! effective prompt (and the compute it costs) shrinks instead of stapling the
//! whole repo onto every call. Retrieval degrades safely: no catalog /
//! conversational query / latency budget exceeded ⇒ the request is forwarded
//! unchanged.

use std::path::PathBuf;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, State};
use tokio::sync::oneshot;

/// Global proxy state — one at a time, guarded by a plain Mutex (cheap, infrequent).
static PROXY: Mutex<Option<ProxyState>> = Mutex::new(None);

struct ProxyState {
    port: u16,
    catalog_dir: PathBuf,
    catalog_active: bool,
    chunks: usize,
    shutdown_tx: oneshot::Sender<()>,
}

#[derive(Serialize, Deserialize)]
pub struct RetrievalStatus {
    pub running: bool,
    pub port: Option<u16>,
    /// A usable `.aim` catalog is loaded — retrieval actually augments prompts.
    /// When `false` the proxy is a plain pass-through (forwards, injects nothing).
    #[serde(default)]
    pub catalog_active: bool,
    #[serde(default)]
    pub chunks: usize,
    #[serde(default)]
    pub catalog_dir: Option<String>,
}

/// Same "explicit `root`, else the editor's active root, else cwd" contract the
/// `cursor_*` commands use.
async fn resolve_root(
    state: &State<'_, std::sync::Arc<crate::EditorState>>,
    root: Option<String>,
) -> Result<PathBuf, String> {
    if let Some(r) = root.map(|s| s.trim().to_string()).filter(|s| !s.is_empty()) {
        return Ok(PathBuf::from(r));
    }
    let guard = state.editor.active_root.lock().await;
    Ok(guard
        .clone()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))))
}

struct CatalogInfo {
    dir: PathBuf,
    active: bool,
    chunks: usize,
}

/// Build the libaim catalog at `<root>/.aim/kortex/` if it's missing (or
/// `rebuild`), else reuse it. The (blocking, CPU-heavy) index runs on the
/// blocking pool and streams `aim-index-progress` events. A build failure is
/// non-fatal — the proxy still starts as a pass-through.
async fn ensure_catalog(
    app: &AppHandle,
    root: &PathBuf,
    embed_base: &str,
    embed_model: &str,
    rebuild: bool,
) -> CatalogInfo {
    let dir = root.join(".aim").join("kortex");
    let needs_build = rebuild || !dir.join(libaim::CONTAINER_FILE).exists();

    if needs_build {
        let _ = app.emit("aim-index-progress", serde_json::json!({ "status": "started" }));
        let (r, base, model, cat, app2) = (
            root.clone(),
            embed_base.trim_end_matches('/').to_string(),
            embed_model.to_string(),
            dir.clone(),
            app.clone(),
        );
        let built = tauri::async_runtime::spawn_blocking(
            move || -> Result<libaim::IndexStats, String> {
                std::fs::create_dir_all(&cat)
                    .map_err(|e| format!("create {}: {e}", cat.display()))?;
                let embedder = libaim::embed_http::HttpEmbedder::autodetect(&base, &model)
                    .map_err(|e| format!("no embedding endpoint at {base} for `{model}`: {e}"))?;
                let opts = libaim::IndexOptions {
                    out_dir: Some(cat.clone()),
                    ..Default::default()
                };
                libaim::index_workspace(&r, &embedder, &opts, |files_done, chunks| {
                    let _ = app2.emit(
                        "aim-index-progress",
                        serde_json::json!({ "status": "indexing", "files": files_done, "chunks": chunks }),
                    );
                })
                .map_err(|e| e.to_string())
            },
        )
        .await;

        match built {
            Ok(Ok(stats)) => {
                let _ = app.emit(
                    "aim-index-progress",
                    serde_json::json!({
                        "status": "complete",
                        "files_indexed": stats.files_indexed,
                        "elapsed_secs": stats.elapsed_secs,
                        "compression_ratio": stats.compression_ratio,
                    }),
                );
            }
            Ok(Err(e)) => {
                eprintln!("[kortex-retrieval] catalog build failed: {e} — starting pass-through");
                let _ = app.emit("aim-index-progress", serde_json::json!({ "status": "error", "error": e }));
            }
            Err(e) => eprintln!("[kortex-retrieval] catalog build task panicked: {e}"),
        }
    }

    match libaim::Catalog::open(&dir) {
        Ok(c) => CatalogInfo { chunks: c.len(), active: c.len() > 0, dir },
        Err(_) => CatalogInfo { chunks: 0, active: false, dir },
    }
}

/// Start the retrieval proxy in-process. Binds `127.0.0.1:1536`, builds/loads
/// the workspace `.aim` catalog, and forwards augmented requests to the local
/// backend.
#[tauri::command]
pub async fn kortex_retrieval_start(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    app: AppHandle,
    root: Option<String>,
    rebuild: Option<bool>,
) -> Result<u16, String> {
    {
        let guard = PROXY.lock().unwrap();
        if let Some(s) = guard.as_ref() {
            return Err(format!("Retrieval proxy already running on :{}", s.port));
        }
    }

    let port: u16 = 1536;
    let root_path = resolve_root(&state, root).await?;
    let upstream = state
        .ai
        .engine
        .lemonade_base()
        .await
        .trim_end_matches('/')
        .to_string();
    let embed_model = crate::domain::indexing::embeddings::default_embed_model().to_string();

    let cat = ensure_catalog(&app, &root_path, &upstream, &embed_model, rebuild.unwrap_or(false)).await;

    // aim-proxy reads these at `AppState::from_env()` — set them first.
    std::env::set_var("KORTEX_AIM_CATALOG", &cat.dir);
    std::env::set_var("KORTEX_WORKSPACE", &root_path);
    std::env::set_var("KORTEX_UPSTREAM_OPENAI", &upstream);

    let router_state = aim_proxy::server::AppState::from_env();
    let router = aim_proxy::server::build_router(router_state);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| format!("bind {addr}: {e}"))?;

    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    tokio::spawn(async move {
        let graceful = axum::serve(listener, router).with_graceful_shutdown(async move {
            let _ = shutdown_rx.await;
        });
        if let Err(e) = graceful.await {
            eprintln!("[kortex-retrieval] server error: {e}");
        }
    });

    println!("[kortex-retrieval] listening on http://127.0.0.1:{port}");
    println!(
        "[kortex-retrieval] catalog {} — {}",
        cat.dir.display(),
        if cat.active {
            format!("active ({} chunks)", cat.chunks)
        } else {
            "PASS-THROUGH (no usable catalog — build one from the Kortex panel)".to_string()
        }
    );
    println!("[kortex-retrieval] upstream -> {upstream}");

    *PROXY.lock().unwrap() = Some(ProxyState {
        port,
        catalog_dir: cat.dir.clone(),
        catalog_active: cat.active,
        chunks: cat.chunks,
        shutdown_tx,
    });

    let _ = app.emit(
        "kortex:retrieval-status",
        serde_json::json!({ "running": true, "port": port, "catalog_active": cat.active, "chunks": cat.chunks }),
    );

    Ok(port)
}

/// Stop the retrieval proxy gracefully.
#[tauri::command]
pub async fn kortex_retrieval_stop() -> Result<(), String> {
    let state = PROXY
        .lock()
        .unwrap()
        .take()
        .ok_or("Retrieval proxy is not running")?;
    let _ = state.shutdown_tx.send(());
    println!("[kortex-retrieval] stopped");
    Ok(())
}

/// Query current status.
#[tauri::command]
pub async fn kortex_retrieval_status() -> Result<RetrievalStatus, String> {
    let guard = PROXY.lock().unwrap();
    Ok(match guard.as_ref() {
        Some(s) => RetrievalStatus {
            running: true,
            port: Some(s.port),
            catalog_active: s.catalog_active,
            chunks: s.chunks,
            catalog_dir: Some(s.catalog_dir.to_string_lossy().into_owned()),
        },
        None => RetrievalStatus {
            running: false,
            port: None,
            catalog_active: false,
            chunks: 0,
            catalog_dir: None,
        },
    })
}

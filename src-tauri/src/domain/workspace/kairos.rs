use std::sync::Arc;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter};
use serde_json::json;
use crate::context_indexer::ContextIndexer;
use crate::memory_store::MemoryStore;
use std::path::PathBuf;
use tokio::process::Command;
use tokio::sync::RwLock;
use serde_json::Value;

pub struct KairosEngine {
    app_handle: RwLock<Option<AppHandle>>,
    last_activity: Arc<tokio::sync::Mutex<Instant>>,
    idle_threshold: Duration,
    indexer: Arc<ContextIndexer>,
    memory: Arc<MemoryStore>,
    project_root: Arc<tokio::sync::Mutex<Option<PathBuf>>>,
}

impl KairosEngine {
    pub fn new(
        indexer: Arc<ContextIndexer>,
        memory: Arc<MemoryStore>,
        project_root: Arc<tokio::sync::Mutex<Option<PathBuf>>>,
    ) -> Self {
        Self {
            app_handle: RwLock::new(None),
            last_activity: Arc::new(tokio::sync::Mutex::new(Instant::now())),
            idle_threshold: Duration::from_secs(30),
            indexer,
            memory,
            project_root,
        }
    }

    /// Wire up the Tauri AppHandle so Kairos can emit events to the frontend.
    /// Call this once at startup after EditorState is built.
    pub fn set_app_handle(&self, handle: AppHandle) {
        // Use blocking_write since this is called during synchronous init only
        if let Ok(mut h) = self.app_handle.try_write() {
            *h = Some(handle);
            println!("[KAIROS] AppHandle registered — frontend events enabled.");
        }
    }

    /// Called whenever the user performs an action (file save, keypress, etc.)
    /// Resets the idle timer so background tasks don't run during active editing.
    pub async fn report_activity(&self) {
        let mut last = self.last_activity.lock().await;
        *last = Instant::now();
        println!("[KAIROS] Activity signal received — idle timer reset.");
    }

    /// Called on a fixed interval (e.g., every 10s) from the background task in lib.rs.
    /// Only performs work if the system has been idle for `idle_threshold`.
    pub async fn tick(&self) {
        let last_active = *self.last_activity.lock().await;
        if last_active.elapsed() >= self.idle_threshold {
            self.perform_background_tasks().await;
        }
    }

    async fn perform_background_tasks(&self) {
        println!("[KAIROS] System idle. Starting background scans...");
        
        let root_lock = self.project_root.lock().await;
        let root = match root_lock.as_ref() {
            Some(r) => r.clone(),
            None => return,
        };
        drop(root_lock);

        // 1. Context Refresh — re-index changed files
        let _ = self.indexer.reindex_if_needed(&root);

        // 2. "Dreaming" — Proactive Diagnostics on idle
        if root.join("Cargo.toml").exists() {
            println!("[KAIROS] Dreaming: Running cargo diagnostics...");
            self.emit_suggestion("Indexing", "Kairos is deep-scanning project symbols in parallel...");
            
            let output = Command::new("cargo")
                .arg("check")
                .arg("--message-format=json")
                .current_dir(&root)
                .output()
                .await;

            if let Ok(out) = output {
                let stdout = String::from_utf8_lossy(&out.stdout);
                let mut issues = 0;
                for line in stdout.lines() {
                    if let Ok(msg) = serde_json::from_str::<Value>(line) {
                        if msg["reason"] == "compiler-message" {
                            if let Some(rendered) = msg["message"]["rendered"].as_str() {
                                let rendered_str: &str = rendered;
                                if rendered_str.contains("error:") || rendered_str.contains("warning:") {
                                    let category = if rendered_str.contains("error:") { "Error" } else { "Warning" };
                                    self.emit_suggestion(category, rendered_str.trim());
                                    issues += 1;
                                    if issues >= 3 { break; } // Don't overwhelm the user
                                }
                            }
                        }
                    }
                }
                if issues == 0 {
                    self.emit_suggestion("Clean", "No errors or warnings detected during idle scan.");
                }
            }
        } else {
            // General project fallback
            self.emit_suggestion("Optimization", "Project structure looks stable. Suggesting index verification.");
        }
    }

    fn emit_suggestion(&self, category: &str, message: &str) {
        // 1. Emit to frontend for real-time user notification
        if let Ok(handle_guard) = self.app_handle.try_read() {
            if let Some(handle) = handle_guard.as_ref() {
                let _ = handle.emit("kairos://suggestion", json!({
                    "category": category,
                    "message": message,
                    "timestamp": std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map(|d| d.as_secs())
                        .unwrap_or(0)
                }));
            }
        }

        // 2. Persist in Kortex Memory Store as an autonomous insight
        let ms = self.memory.clone();
        let cat = category.to_string();
        let msg = message.to_string();
        tauri::async_runtime::spawn(async move {
            ms.store_slot(crate::memory_store::SemanticSlot {
                id: uuid::Uuid::new_v4().to_string(),
                category: format!("kairos_{}", cat.to_lowercase()),
                content: format!("Idle Insight: {}", msg),
                tags: vec!["kairos_dream".to_string(), cat],
                metadata: None,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            }).await;
        });
    }
}

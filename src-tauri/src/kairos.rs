use std::sync::Arc;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter};
use serde_json::json;
use crate::context_indexer::ContextIndexer;
use crate::memory_store::MemoryStore;
use std::path::PathBuf;
use tokio::process::Command;
use serde_json::Value;

pub struct KairosEngine {
    app_handle: Option<AppHandle>,
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
            app_handle: None,
            last_activity: Arc::new(tokio::sync::Mutex::new(Instant::now())),
            idle_threshold: Duration::from_secs(30),
            indexer,
            memory,
            project_root,
        }
    }

    pub fn set_app_handle(&mut self, handle: AppHandle) {
        self.app_handle = Some(handle);
    }

    pub async fn report_activity(&self) {
        let mut last = self.last_activity.lock().await;
        *last = Instant::now();
    }

    pub async fn start_loop(self: Arc<Self>) {
        println!("[KAIROS] Background intelligence loop started.");
        let mut interval = tokio::time::interval(Duration::from_secs(10));

        loop {
            interval.tick().await;
            
            let last_active = *self.last_activity.lock().await;
            if last_active.elapsed() >= self.idle_threshold {
                self.perform_background_tasks().await;
            }
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

        // 1. Context Refresh
        let _ = self.indexer.reindex_if_needed(&root);

        // 2. Perform "Dreaming" (Proactive Diagnostics)
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
             }
        } else {
            // General Fallback
            self.emit_suggestion("Optimization", "Project structure looks stable. Suggesting index verification.");
        }
    }

    fn emit_suggestion(&self, category: &str, message: &str) {
        if let Some(handle) = &self.app_handle {
            let _ = handle.emit("kairos://suggestion", json!({
                "category": category,
                "message": message,
                "timestamp": chrono::Utc::now().timestamp()
            }));
        }
    }
}

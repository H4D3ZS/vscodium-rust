use std::sync::Arc;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter};
use serde_json::json;
use crate::context_indexer::ContextIndexer;
use crate::memory_store::MemoryStore;
use std::path::PathBuf;

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
        
        // 1. Tick the indexer (proactive refresh)
        let root_lock = self.project_root.lock().await;
        if let Some(root) = root_lock.as_ref() {
            let _ = self.indexer.reindex_if_needed(root);
        }

        // 2. Perform "Dreaming" (Static Analysis / Linting / Optimization)
        // For now, this is a simulated dream that emits a notification
        self.emit_suggestion("Optimization", "Detected redundant imports in context_indexer.rs. Suggested cleanup staged.");
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

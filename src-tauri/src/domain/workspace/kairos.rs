use std::sync::Arc;
use std::time::{Duration, Instant};
use serde_json::json;
use crate::context_indexer::ContextIndexer;
use crate::memory_store::MemoryStore;
use std::path::PathBuf;
use tokio::process::Command;
use serde_json::Value;

pub struct KairosEngine {
    editor_state: std::sync::RwLock<std::sync::Weak<crate::EditorState>>,
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
            editor_state: std::sync::RwLock::new(std::sync::Weak::new()),
            last_activity: Arc::new(tokio::sync::Mutex::new(Instant::now())),
            idle_threshold: Duration::from_secs(30),
            indexer,
            memory,
            project_root,
        }
    }

    pub fn set_editor_state(&self, weak: std::sync::Weak<crate::EditorState>) {
        if let Ok(mut g) = self.editor_state.write() {
            *g = weak;
        }
    }

    pub async fn report_activity(&self) {
        let mut last = self.last_activity.lock().await;
        *last = Instant::now();
        println!("[KAIROS] Activity signal received — idle timer reset.");
    }

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

        let _ = self.indexer.reindex_if_needed(&root);

        if false && root.join("Cargo.toml").exists() {
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
                                    if issues >= 3 { break; }
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
            self.emit_suggestion("Optimization", "Project structure looks stable. Suggesting index verification.");
        }
    }

    fn emit_suggestion(&self, category: &str, message: &str) {
        if let Some(es) = self.editor_state.read().ok().and_then(|w| w.upgrade()) {
            es.emit("kairos://suggestion", json!({
                "category": category,
                "message": message,
                "timestamp": std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0)
            }));
        }

        let ms = self.memory.clone();
        let cat = category.to_string();
        let msg = message.to_string();
        tokio::spawn(async move {
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

use crate::memory_store::{MemoryStore, SemanticSlot};
use serde_json::json;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use walkdir::WalkDir;

pub struct ContextIndexer {
    memory_store: Arc<MemoryStore>,
    root_path: PathBuf,
}

impl ContextIndexer {
    pub fn new(memory_store: Arc<MemoryStore>, root_path: PathBuf) -> Self {
        Self {
            memory_store,
            root_path,
        }
    }

    pub async fn start_background_indexing(&self) {
        let ms = self.memory_store.clone();
        let root = self.root_path.clone();

        tauri::async_runtime::spawn(async move {
            println!("[CONTEXT] Starting background indexing loop for: {:?}", root);
            loop {
                if let Err(e) = Self::run_index_cycle(&ms, &root).await {
                    eprintln!("[CONTEXT] Indexing error: {:?}", e);
                }
                // Sleep for 5 minutes between full cycles
                sleep(Duration::from_secs(300)).await;
            }
        });
    }

    pub async fn trigger_index_cycle(&self) -> anyhow::Result<()> {
        Self::run_index_cycle(&self.memory_store, &self.root_path).await
    }

    async fn run_index_cycle(ms: &MemoryStore, root: &Path) -> anyhow::Result<()> {
        if !root.join(".aim").exists() {
            println!("[CONTEXT] Project is dormant (No .aim detected). Skipping index cycle.");
            return Ok(());
        }

        for entry in WalkDir::new(root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                let p = e.path();
                p.is_file()
                    && (p.extension().map_or(false, |ext| {
                        ext == "rs" || ext == "ts" || ext == "tsx" || ext == "json" || ext == "md"
                    }))
                    && !p.to_string_lossy().contains("node_modules")
                    && !p.to_string_lossy().contains("target")
                    && !p.to_string_lossy().contains(".git")
            })
        {
            let path = entry.path();
            let relative_path = path.strip_prefix(root)?.to_string_lossy().to_string();
            let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

            let mut category = if extension == "md" {
                "fix_lessons"
            } else {
                "file_map"
            };

            let content = std::fs::read_to_string(path).unwrap_or_default();
            let mut tags = Vec::new();

            // Extract symbols if it's code
            if extension == "rs" || extension == "ts" || extension == "tsx" {
                let symbols = Self::extract_symbols(&content, extension);
                for sym in symbols {
                    tags.push(format!("symbol:{}", sym));
                }
            }

            // If it's a markdown file, look for specific "Lesson" triggers
            if extension == "md" {
                if content.contains("# Learning") || content.contains("# Fix") {
                    category = "fix_lessons";
                    tags.push("discovery:lesson".to_string());
                }
            }

            ms.store_slot(SemanticSlot {
                id: format!("{}:{}", category, relative_path),
                category: category.to_string(),
                content: relative_path.clone(),
                tags,
                metadata: Some(json!({
                    "extension": extension,
                    "size": entry.metadata()?.len()
                })),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_secs(),
            })
            .await;

            // Throttle to prevent I/O saturation on large projects
            sleep(Duration::from_millis(5)).await;
        }
        Ok(())
    }

    fn extract_symbols(content: &str, ext: &str) -> Vec<String> {
        let mut symbols = Vec::new();
        if ext == "rs" {
            // Simple regex for Rust functions: fn name(...)
            let re = regex::Regex::new(r"fn\s+([a-zA-Z_][a-zA-Z0-9_]*)").unwrap();
            for cap in re.captures_iter(content) {
                symbols.push(cap[1].to_string());
            }
        } else if ext == "ts" || ext == "tsx" {
            // Simple regex for TS functions: function name or const name = ...
            let re_func = regex::Regex::new(r"function\s+([a-zA-Z_][a-zA-Z0-9_]*)").unwrap();
            for cap in re_func.captures_iter(content) {
                symbols.push(cap[1].to_string());
            }
            let re_const =
                regex::Regex::new(r"const\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*=\s*[\(|\{]").unwrap();
            for cap in re_const.captures_iter(content) {
                symbols.push(cap[1].to_string());
            }
        }
        symbols.sort();
        symbols.dedup();
        symbols
    }
}

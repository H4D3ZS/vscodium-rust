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
            println!("[CONTEXT] Starting background indexing for: {:?}", root);
            loop {
                if let Err(e) = Self::run_index_cycle(&ms, &root).await {
                    eprintln!("[CONTEXT] Indexing error: {:?}", e);
                }
                // Sleep for 5 minutes between full cycles
                sleep(Duration::from_secs(300)).await;
            }
        });
    }

    async fn run_index_cycle(ms: &MemoryStore, root: &Path) -> anyhow::Result<()> {
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
            })
        {
            let path = entry.path();
            let relative_path = path.strip_prefix(root)?.to_string_lossy().to_string();
            let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

            let category = if extension == "md" {
                "fix_lessons"
            } else {
                "file_map"
            };

            // Basic file indexing (just presence and type for now)
            ms.store_slot(SemanticSlot {
                id: format!("{}:{}", category, relative_path),
                category: category.to_string(),
                content: relative_path.clone(),
                metadata: Some(json!({
                    "extension": extension,
                    "size": entry.metadata()?.len()
                })),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_secs(),
            })
            .await;

            // In the future, we will use tree-sitter here to extract symbols
            // and add relationships via ms.add_relationship()
        }
        Ok(())
    }
}

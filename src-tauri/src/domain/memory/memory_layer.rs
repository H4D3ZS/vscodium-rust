use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone)]
pub struct MemoryLayer {
    #[allow(dead_code)]
    root_path: PathBuf,
    hades_dir: PathBuf,
    /// Optional link to the AI engine's persistent memory store for semantic search.
    memory_store: Option<Arc<crate::memory_store::MemoryStore>>,
}

impl MemoryLayer {
    pub fn new(root_path: PathBuf) -> Self {
        let hades_dir = root_path.join(".hades");
        if !hades_dir.exists() {
            let _ = fs::create_dir_all(&hades_dir);
        }

        Self {
            root_path,
            hades_dir,
            memory_store: None,
        }
    }

    /// Wire in the persistent MemoryStore so search() and query_context()
    /// delegate to real semantic retrieval instead of returning empty stubs.
    pub fn set_memory_store(&mut self, store: Arc<crate::memory_store::MemoryStore>) {
        self.memory_store = Some(store);
    }

    pub async fn mount_workspace(&self, path: PathBuf) -> Result<()> {
        let hades_dir = path.join(".hades");
        if !hades_dir.exists() {
            fs::create_dir_all(&hades_dir)?;
        }
        // In a real implementation, this would switch the internal path
        Ok(())
    }

    pub async fn unmount(&self) {
        // Placeholder
    }

    pub async fn get_all_memory(&self) -> Result<serde_json::Value> {
        let ctx = self.get_aggregate_context()?;
        Ok(serde_json::json!({ "context": ctx }))
    }

    pub async fn clear_memory(&self) -> Result<()> {
        let files = ["memory.md", "decisions.md", "patterns.md", "state.md"];
        for file in files {
            let path = self.hades_dir.join(file);
            if path.exists() {
                fs::write(path, "")?;
            }
        }
        Ok(())
    }

    pub async fn set_memory(&self, key: &str, value: &str) -> Result<()> {
        let path = self.hades_dir.join("memory.md");
        let mut content = fs::read_to_string(&path).unwrap_or_default();
        content.push_str(&format!("\n### {}\n{}\n", key, value));
        fs::write(path, content)?;
        Ok(())
    }

    pub async fn search(&self, query: &str) -> Result<serde_json::Value> {
        if let Some(store) = &self.memory_store {
            let context = store.retrieve_context(query).await;
            return Ok(serde_json::json!({ "query": query, "results": context }));
        }
        Ok(serde_json::json!({ "query": query, "results": [] }))
    }

    pub async fn query_context(&self, query: &str) -> Result<serde_json::Value> {
        let aggregate = self.get_aggregate_context().unwrap_or_default();
        if let Some(store) = &self.memory_store {
            let semantic = store.retrieve_context(query).await;
            return Ok(serde_json::json!({
                "query": query,
                "context": aggregate,
                "semantic_context": semantic,
                "gist": store.build_compact_gist().await
            }));
        }
        Ok(serde_json::json!({ "query": query, "context": aggregate }))
    }

    pub async fn get_file_context(&self, path: &str) -> Result<serde_json::Value> {
        if let Some(store) = &self.memory_store {
            if let Some(cached) = store.get_vfs_cache(&std::path::PathBuf::from(path)).await {
                return Ok(serde_json::json!({ "path": path, "context": cached }));
            }
        }
        Ok(serde_json::json!({ "path": path, "context": "Not cached — use view_file to load" }))
    }

    /// Appends a new decision to decisions.md
    pub fn record_decision(&self, decision: &str, rationale: &str, impact: &str) -> Result<()> {
        let path = self.hades_dir.join("decisions.md");
        let date = chrono::Local::now().format("%Y-%m-%d");
        let new_line = format!("| {} | {} | {} | {} |\n", date, decision, rationale, impact);
        
        let mut content = fs::read_to_string(&path).unwrap_or_default();
        if content.is_empty() {
            content = "# Sentient Decisions\n## Mission Log\n\n| Date | Decision | Rationale | Impact |\n|------|----------|-----------|--------|\n".to_string();
        }
        
        content.push_str(&new_line);
        fs::write(path, content)?;
        Ok(())
    }

    /// Updates the active mission status in state.md
    pub fn update_state(&self, status: &str, current_task: &str) -> Result<()> {
        let path = self.hades_dir.join("state.md");
        let mut content = fs::read_to_string(&path).unwrap_or_default();
        
        if content.is_empty() {
            content = "# Mission State\n- **Status:** Initialized\n- **Current Task:** Idle\n".to_string();
        }

        // Primitive regex-less replacement for key fields
        if let Some(pos) = content.find("- **Status:**") {
            if let Some(end) = content[pos..].find('\n') {
                content.replace_range(pos..(pos + end), &format!("- **Status:** {}", status));
            }
        }
        
        if let Some(pos) = content.find("- **Current Task:**") {
            if let Some(end) = content[pos..].find('\n') {
                content.replace_range(pos..(pos + end), &format!("- **Current Task:** {}", current_task));
            }
        }
        
        fs::write(path, content)?;
        Ok(())
    }

    /// Retrieves the aggregate memory context for LLM injection.
    /// `max_chars` caps total size — 0 means no limit.
    pub fn get_aggregate_context(&self) -> Result<String> {
        self.get_aggregate_context_sized(0)
    }

    pub fn get_aggregate_context_sized(&self, max_chars: usize) -> Result<String> {
        let files = ["memory.md", "decisions.md", "patterns.md", "state.md"];
        let mut aggregate = String::from("### HADES SENTIENT MEMORY CONTEXT ###\n");
        
        for file in files {
            let path = self.hades_dir.join(file);
            if path.exists() {
                let content = fs::read_to_string(path)?;
                let entry = format!("\n--- {} ---\n{}\n", file, content);
                if max_chars > 0 && aggregate.len() + entry.len() > max_chars {
                    // Truncate this file's content to fit
                    let remaining = max_chars.saturating_sub(aggregate.len()).saturating_sub(64);
                    if remaining > 100 {
                        aggregate.push_str(&format!("\n--- {} ---\n{}...(truncated)\n", file, &content[..content.floor_char_boundary(remaining)]));
                    }
                    break;
                }
                aggregate.push_str(&entry);
            }
        }
        
        Ok(aggregate)
    }
}

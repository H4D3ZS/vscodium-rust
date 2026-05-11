use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryLayer {
    root_path: PathBuf,
    hades_dir: PathBuf,
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
        }
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
        // Mock search for now
        Ok(serde_json::json!({ "query": query, "results": [] }))
    }

    pub async fn query_context(&self, query: &str) -> Result<serde_json::Value> {
        Ok(serde_json::json!({ "query": query, "context": self.get_aggregate_context().unwrap_or_default() }))
    }

    pub async fn get_file_context(&self, path: &str) -> Result<serde_json::Value> {
        Ok(serde_json::json!({ "path": path, "context": "None" }))
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

    /// Retrieves the aggregate memory context for LLM injection
    pub fn get_aggregate_context(&self) -> Result<String> {
        let files = ["memory.md", "decisions.md", "patterns.md", "state.md"];
        let mut aggregate = String::from("### HADES SENTIENT MEMORY CONTEXT ###\n");
        
        for file in files {
            let path = self.hades_dir.join(file);
            if path.exists() {
                let content = fs::read_to_string(path)?;
                aggregate.push_str(&format!("\n--- {} ---\n{}\n", file, content));
            }
        }
        
        Ok(aggregate)
    }
}

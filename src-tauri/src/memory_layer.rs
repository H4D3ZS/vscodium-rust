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

    /// Appends a new decision to decisions.md
    pub fn record_decision(&self, decision: &str, Rationale: &str, impact: &str) -> Result<()> {
        let path = self.hades_dir.join("decisions.md");
        let date = chrono::Local::now().format("%Y-%m-%d");
        let new_line = format!("| {} | {} | {} | {} |\n", date, decision, Rationale, impact);
        
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
        let mut content = fs::read_to_string(&path)?;
        
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

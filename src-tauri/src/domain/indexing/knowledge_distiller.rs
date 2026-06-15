use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use chrono::{Local};

#[derive(Debug, Serialize, Deserialize)]
pub struct KnowledgeBrief {
    pub title: String,
    pub date: String,
    pub findings: String,
    pub affected_files: Vec<String>,
}

pub struct KnowledgeDistiller {
    base_path: PathBuf,
}

impl KnowledgeDistiller {
    pub fn new(project_root: &Path) -> Self {
        let knowledge_path = project_root.join(".kortex").join("knowledge");
        if !knowledge_path.exists() {
            let _ = fs::create_dir_all(&knowledge_path);
        }
        Self {
            base_path: knowledge_path,
        }
    }

    pub fn save_finding(&self, brief: KnowledgeBrief) -> Result<String, String> {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let filename = format!("{}_{}.md", timestamp, brief.title.replace(" ", "_").to_lowercase());
        let file_path = self.base_path.join(filename);

        let content = format!(
            "# {}\n\n**Date:** {}\n**Affected Files:** {:?}\n\n## Findings\n{}\n",
            brief.title, brief.date, brief.affected_files, brief.findings
        );

        fs::write(&file_path, content)
            .map_err(|e| format!("Failed to write knowledge file: {}", e))?;

        Ok(file_path.to_string_lossy().to_string())
    }

    pub fn load_all_knowledge(&self) -> Result<String, String> {
        if !self.base_path.exists() {
            return Ok(String::new());
        }

        let mut combined_knowledge = String::from("\n=== PERSISTENT KNOWLEDGE (Session History) ===\n");
        let entries = fs::read_dir(&self.base_path)
            .map_err(|e| format!("Failed to read knowledge dir: {}", e))?;

        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        combined_knowledge.push_str(&content);
                        combined_knowledge.push_str("\n\n---\n\n");
                    }
                }
            }
        }

        Ok(combined_knowledge)
    }
}

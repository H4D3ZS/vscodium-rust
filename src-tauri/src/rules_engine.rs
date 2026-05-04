use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentRule {
    pub name: String,
    pub content: String,
    pub file_path: PathBuf,
}

pub struct RulesEngine {
    pub workspace_root: PathBuf,
}

impl RulesEngine {
    pub fn new(root: PathBuf) -> Self {
        Self { workspace_root: root }
    }

    pub fn get_workspace_rules(&self) -> Vec<AgentRule> {
        let mut rules = Vec::new();
        let rules_dir = self.workspace_root.join(".agents").join("rules");
        let legacy_dir = self.workspace_root.join(".agent").join("rules");

        self.scan_dir(&rules_dir, &mut rules);
        self.scan_dir(&legacy_dir, &mut rules);
        
        rules
    }

    fn scan_dir(&self, dir: &Path, rules: &mut Vec<AgentRule>) {
        if !dir.exists() { return; }
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().map(|s| s == "md").unwrap_or(false) {
                    if let Ok(content) = fs::read_to_string(&path) {
                        let name = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
                        rules.push(AgentRule {
                            name,
                            content,
                            file_path: path,
                        });
                    }
                }
            }
        }
    }

    pub fn format_rules_for_prompt(&self) -> String {
        let rules = self.get_workspace_rules();
        if rules.is_empty() { return String::new(); }

        let mut output = String::from("\n### WORKSPACE RULES:\n");
        for rule in rules {
            output.push_str(&format!("\n#### Rule: {}\n{}\n", rule.name, rule.content));
        }
        output
    }
}

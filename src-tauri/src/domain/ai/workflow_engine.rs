use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Workflow {
    pub name: String,
    pub description: String,
    pub steps: Vec<String>,
    pub file_path: PathBuf,
}

pub struct WorkflowEngine {
    pub workspace_root: PathBuf,
}

impl WorkflowEngine {
    pub fn new(root: PathBuf) -> Self {
        Self { workspace_root: root }
    }

    pub fn get_workflows(&self) -> Vec<Workflow> {
        let mut workflows = Vec::new();
        let workflows_dir = self.workspace_root.join(".agents").join("workflows");
        let legacy_dir = self.workspace_root.join(".agent").join("workflows");

        self.scan_dir(&workflows_dir, &mut workflows);
        self.scan_dir(&legacy_dir, &mut workflows);
        
        workflows
    }

    fn scan_dir(&self, dir: &Path, workflows: &mut Vec<Workflow>) {
        if !dir.exists() { return; }
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().map(|s| s == "md").unwrap_or(false) {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Some(wf) = self.parse_workflow(&content, &path) {
                            workflows.push(wf);
                        }
                    }
                }
            }
        }
    }

    fn parse_workflow(&self, content: &str, path: &Path) -> Option<Workflow> {
        let name = path.file_stem()?.to_string_lossy().to_string();
        let mut description = String::new();
        let mut steps = Vec::new();

        let mut in_steps = false;
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("# ") && description.is_empty() {
                description = line[2..].to_string();
            } else if line.to_lowercase().contains("steps") {
                in_steps = true;
            } else if in_steps {
                if line.starts_with("- ") || line.starts_with("* ") {
                    steps.push(line[2..].to_string());
                } else if (line.starts_with("1. ") || line.starts_with("2. ")) && line.len() > 3 {
                    steps.push(line[3..].to_string());
                }
            }
        }

        Some(Workflow {
            name,
            description,
            steps,
            file_path: path.to_path_buf(),
        })
    }

    pub fn format_workflows_for_prompt(&self) -> String {
        let workflows = self.get_workflows();
        if workflows.is_empty() { return String::new(); }

        let mut output = String::from("\n### AVAILABLE WORKFLOWS:\n");
        output.push_str("You can trigger these workflows via slash commands (e.g., /workflow-name).\n");
        for wf in workflows {
            output.push_str(&format!("- /{} : {}\n", wf.name, wf.description));
        }
        output
    }
}

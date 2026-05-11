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

        // Our native rule locations
        self.scan_dir(&self.workspace_root.join(".agents").join("rules"), &mut rules);
        self.scan_dir(&self.workspace_root.join(".agent").join("rules"),  &mut rules);

        // Cursor IDE compatibility: read `.cursor/rules/*.mdc` (and `.md`) so
        // any project pulled from a Cursor user's machine keeps its rules
        // active without manual migration. `.mdc` is Cursor's native suffix.
        self.scan_dir(&self.workspace_root.join(".cursor").join("rules"),  &mut rules);

        // Legacy single-file rules — Cursor's old `.cursorrules` and the
        // common AI-tool conventions `AGENTS.md` / `CLAUDE.md` at root.
        for (name, fname) in [
            ("cursorrules", ".cursorrules"),
            ("agents",      "AGENTS.md"),
            ("claude",      "CLAUDE.md"),
        ] {
            let p = self.workspace_root.join(fname);
            if p.is_file() {
                if let Ok(content) = fs::read_to_string(&p) {
                    rules.push(AgentRule {
                        name: name.to_string(),
                        content,
                        file_path: p,
                    });
                }
            }
        }

        rules
    }

    fn scan_dir(&self, dir: &Path, rules: &mut Vec<AgentRule>) {
        if !dir.exists() { return; }
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let ext_ok = path
                    .extension()
                    .and_then(|s| s.to_str())
                    .map(|s| s == "md" || s == "mdc")
                    .unwrap_or(false);
                if path.is_file() && ext_ok {
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

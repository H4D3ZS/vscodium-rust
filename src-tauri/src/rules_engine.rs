use std::path::{Path, PathBuf};
use std::fs;
use std::sync::RwLock;
use serde::{Deserialize, Serialize};
use crate::cursor_compat::{load_cursor_rules, format_rules_for_prompt, CursorRuleFrontmatter};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentRule {
    pub name: String,
    pub content: String,
    pub file_path: PathBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub globs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_apply: Option<bool>,
}

pub struct RulesEngine {
    workspace_root: RwLock<PathBuf>,
}

impl RulesEngine {
    pub fn new(root: PathBuf) -> Self {
        Self {
            workspace_root: RwLock::new(root),
        }
    }

    pub fn set_root(&self, root: PathBuf) {
        if let Ok(mut w) = self.workspace_root.write() {
            *w = root;
        }
    }

    fn root(&self) -> PathBuf {
        self.workspace_root.read().map(|r| r.clone()).unwrap_or_else(|_| PathBuf::from("."))
    }

    pub fn get_workspace_rules(&self) -> Vec<AgentRule> {
        let root = self.root();
        let mut rules = Vec::new();

        self.scan_dir(&root.join(".agents").join("rules"), &mut rules);
        self.scan_dir(&root.join(".agent").join("rules"),  &mut rules);

        if let Ok(cursor_rules) = load_cursor_rules(&root) {
            for cr in cursor_rules {
                rules.push(AgentRule {
                    name: cr.name,
                    content: cr.content,
                    file_path: cr.file_path,
                    description: cr.frontmatter.as_ref().and_then(|f| f.description.clone()),
                    globs: cr.frontmatter.as_ref().and_then(|f| f.globs.clone()),
                    always_apply: cr.frontmatter.as_ref().and_then(|f| f.always_apply),
                });
            }
        }

        for (name, fname) in [
            ("agents",      "AGENTS.md"),
            ("claude",      "CLAUDE.md"),
        ] {
            let p = root.join(fname);
            if p.is_file() {
                if let Ok(content) = fs::read_to_string(&p) {
                    rules.push(AgentRule {
                        name: name.to_string(),
                        content,
                        file_path: p,
                        description: None,
                        globs: None,
                        always_apply: Some(true),
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
                            description: None,
                            globs: None,
                            always_apply: Some(true),
                        });
                    }
                }
            }
        }
    }

    pub fn format_rules_for_prompt(&self) -> String {
        self.format_rules_for_prompt_with_file(None)
    }

    pub fn format_rules_for_prompt_with_file(&self, active_file: Option<&str>) -> String {
        let root = self.root();
        let cursor_rules: Vec<crate::cursor_compat::CursorRule> = self
            .get_workspace_rules()
            .into_iter()
            .map(|r| crate::cursor_compat::CursorRule {
                name: r.name,
                content: r.content,
                file_path: r.file_path,
                frontmatter: Some(CursorRuleFrontmatter {
                    description: r.description,
                    globs: r.globs,
                    always_apply: r.always_apply,
                }),
            })
            .collect();
        let _ = root; // root already reflected in get_workspace_rules
        format_rules_for_prompt(&cursor_rules, active_file)
    }
}

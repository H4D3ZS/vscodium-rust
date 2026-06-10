//! Unified IDE workspace compatibility: Cursor + Kiro + Antigravity conventions.
//!
//! Binary installs (`cursor/`, `Kiro/`, `Antigravity_IDE/`) are reference-only;
//! this module implements their on-disk project layouts in the Tauri backend.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

pub use crate::cursor_compat::{
    append_debug_log, create_worktree, ensure_scaffold as ensure_cursor_scaffold,
    format_environment_for_prompt, format_rules_for_prompt, list_worktrees, load_cursor_rules,
    load_environment, load_workspace_mcp, parse_mdc, scan_project as scan_cursor_project,
    CursorEnvironment, CursorIgnoreSet, CursorProjectScan, CursorRule, CursorWorktreeInfo,
    IgnoreScope,
};

// ── Kiro steering (`.kiro/steering/` + `.agent/steering/`) ───────────────────

#[derive(Debug, Clone, Serialize)]
pub struct SteeringDoc {
    pub name: String,
    pub content: String,
    pub path: String,
    pub source: String, // "kiro" | "agent"
}

pub fn load_steering_docs(root: &Path) -> Vec<SteeringDoc> {
    let mut docs = Vec::new();
    for (source, rel) in [("kiro", ".kiro/steering"), ("agent", ".agent/steering")] {
        let dir = root.join(rel);
        if !dir.is_dir() {
            continue;
        }
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if !p.is_file() {
                    continue;
                }
                let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");
                if ext != "md" && ext != "mdc" {
                    continue;
                }
                if let Ok(content) = fs::read_to_string(&p) {
                    docs.push(SteeringDoc {
                        name: p.file_stem().unwrap_or_default().to_string_lossy().to_string(),
                        content,
                        path: p.to_string_lossy().to_string(),
                        source: source.to_string(),
                    });
                }
            }
        }
    }
    docs
}

pub fn format_steering_for_prompt(docs: &[SteeringDoc]) -> String {
    if docs.is_empty() {
        return String::new();
    }
    let mut out = String::from("\n### PROJECT STEERING (Kiro / Antigravity):\n");
    out.push_str("*(Always active — follow for every response.)*\n");
    for d in docs.iter().take(12) {
        out.push_str(&format!("\n#### {} [{}]\n{}\n", d.name, d.source, d.content.trim()));
    }
    out
}

// ── Kiro hooks (`.hooks/*.json`, `.kiro/hooks/*.json`) ──────────────────────

fn default_hook_enabled() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KiroHook {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub comment: Option<String>,
    pub when: KiroHookWhen,
    pub then: KiroHookThen,
    #[serde(default = "default_hook_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub file_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum KiroHookWhen {
    FileEdited { pattern: String, #[serde(default)] scope_id: Option<String> },
    FileCreated { pattern: String },
    FileDeleted { pattern: String },
    UserTriggered,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum KiroHookThen {
    AskAgent { prompt: String, #[serde(default)] prompt_id: Option<String> },
    Alert { message: String },
}

#[derive(Debug, Clone, Serialize)]
pub struct HookDispatchResult {
    pub hook_id: String,
    pub hook_name: String,
    pub action: String,
    pub prompt: Option<String>,
    pub message: Option<String>,
}

fn hook_dirs(root: &Path) -> Vec<PathBuf> {
    vec![root.join(".hooks"), root.join(".kiro").join("hooks")]
}

pub fn load_kiro_hooks(root: &Path) -> Result<Vec<KiroHook>> {
    let mut hooks = Vec::new();
    for dir in hook_dirs(root) {
        if !dir.is_dir() {
            continue;
        }
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let p = entry.path();
            if !p.is_file() {
                continue;
            }
            if p.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let raw = fs::read_to_string(&p)?;
            let mut hook: KiroHook = serde_json::from_str(&raw)?;
            hook.file_path = Some(p.to_string_lossy().to_string());
            if hook.id.is_none() {
                hook.id = Some(p.file_stem().unwrap_or_default().to_string_lossy().to_string());
            }
            hooks.push(hook);
        }
    }
    Ok(hooks)
}

pub fn save_kiro_hook(root: &Path, filename: &str, hook: &KiroHook) -> Result<PathBuf> {
    let dir = root.join(".hooks");
    fs::create_dir_all(&dir)?;
    let safe: String = filename
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
        .collect();
    let name = if safe.ends_with(".json") { safe } else { format!("{safe}.json") };
    let path = dir.join(name);
    let to_write = serde_json::to_string_pretty(hook)?;
    fs::write(&path, to_write)?;
    Ok(path)
}

pub fn delete_kiro_hook(root: &Path, file_path: &str) -> Result<()> {
    let p = PathBuf::from(file_path);
    if !p.starts_with(root) {
        return Err(anyhow!("Hook path outside workspace"));
    }
    if p.is_file() {
        fs::remove_file(p)?;
    }
    Ok(())
}

/// Prefer `.kiro/specs/` when a Kiro workspace exists; otherwise `specs/`.
pub fn resolve_specs_dir(root: &Path) -> PathBuf {
    let kiro = root.join(".kiro");
    let kiro_specs = kiro.join("specs");
    if kiro.is_dir() {
        return kiro_specs;
    }
    root.join("specs")
}

fn pattern_matches(pattern: &str, file_path: &str) -> bool {
    glob::Pattern::new(pattern)
        .map(|p| p.matches(file_path))
        .unwrap_or(false)
}

pub fn dispatch_hooks(root: &Path, event: &str, file_path: &str) -> Result<Vec<HookDispatchResult>> {
    let hooks = load_kiro_hooks(root)?;
    let rel = Path::new(file_path)
        .strip_prefix(root)
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| file_path.replace('\\', "/"));

    let mut out = Vec::new();
    for hook in hooks {
        if !hook.enabled {
            continue;
        }
        let matched = match (&hook.when, event) {
            (KiroHookWhen::FileEdited { pattern, .. }, "fileEdited" | "on_save") => {
                pattern_matches(pattern, &rel) || pattern_matches(pattern, file_path)
            }
            (KiroHookWhen::FileCreated { pattern }, "fileCreated" | "on_file_create") => {
                pattern_matches(pattern, &rel)
            }
            (KiroHookWhen::FileDeleted { pattern }, "fileDeleted") => {
                pattern_matches(pattern, &rel)
            }
            (KiroHookWhen::UserTriggered, "userTriggered" | "manual") => true,
            _ => false,
        };
        if !matched {
            continue;
        }
        let id = hook.id.clone().unwrap_or_else(|| "hook".into());
        let name = hook.name.clone().unwrap_or(id.clone());
        match &hook.then {
            KiroHookThen::AskAgent { prompt, .. } => {
                out.push(HookDispatchResult {
                    hook_id: id,
                    hook_name: name,
                    action: "askAgent".into(),
                    prompt: Some(prompt.clone()),
                    message: None,
                });
            }
            KiroHookThen::Alert { message } => {
                out.push(HookDispatchResult {
                    hook_id: id,
                    hook_name: name,
                    action: "alert".into(),
                    prompt: None,
                    message: Some(message.clone()),
                });
            }
        }
    }
    Ok(out)
}

// ── MCP merge sources ─────────────────────────────────────────────────────────

pub fn workspace_mcp_paths(root: &Path) -> Vec<PathBuf> {
    vec![
        root.join(".cursor").join("mcp.json"),
        root.join(".kiro").join("settings").join("mcp.json"),
    ]
}

pub fn load_mcp_from_path(path: &Path) -> Option<Value> {
    if !path.is_file() {
        return None;
    }
    fs::read_to_string(path).ok().and_then(|s| serde_json::from_str(&s).ok())
}

// ── Antigravity layout scan ───────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct AntigravityLayoutScan {
    pub workflows_count: usize,
    pub specs_root_count: usize,
    pub kiro_specs_count: usize,
    pub rules_count: usize,
    pub has_state: bool,
}

pub fn scan_antigravity_layout(root: &Path) -> AntigravityLayoutScan {
    let workflows = [".agent/workflows", ".agents/workflows"]
        .iter()
        .map(|d| count_files_recursive(&root.join(d)))
        .sum::<usize>();
    let specs_root = count_spec_dirs(&root.join("specs"));
    let kiro_specs = count_spec_dirs(&root.join(".kiro").join("specs"));
    let mut rules = 0usize;
    for d in [".agent/rules", ".agents/rules", ".cursor/rules"] {
        rules += count_md_files(&root.join(d));
    }
    AntigravityLayoutScan {
        workflows_count: workflows,
        specs_root_count: specs_root,
        kiro_specs_count: kiro_specs,
        rules_count: rules,
        has_state: root.join(".hades").join("state.md").is_file(),
    }
}

fn count_files_recursive(dir: &Path) -> usize {
    if !dir.is_dir() {
        return 0;
    }
    fs::read_dir(dir).map(|e| e.flatten().count()).unwrap_or(0)
}

fn count_md_files(dir: &Path) -> usize {
    if !dir.is_dir() {
        return 0;
    }
    fs::read_dir(dir)
        .map(|entries| {
            entries
                .flatten()
                .filter(|e| {
                    e.path()
                        .extension()
                        .and_then(|x| x.to_str())
                        .map(|x| x == "md" || x == "mdc")
                        .unwrap_or(false)
                })
                .count()
        })
        .unwrap_or(0)
}

fn count_spec_dirs(dir: &Path) -> usize {
    if !dir.is_dir() {
        return 0;
    }
    fs::read_dir(dir)
        .map(|entries| entries.flatten().filter(|e| e.path().is_dir()).count())
        .unwrap_or(0)
}

// ── Unified scan + scaffold ───────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct WorkspaceScan {
    pub cursor: CursorProjectScan,
    pub steering_count: usize,
    pub steering: Vec<SteeringDoc>,
    pub kiro_hooks_count: usize,
    pub kiro_mcp_count: usize,
    pub antigravity: AntigravityLayoutScan,
}

pub fn scan_workspace(root: &Path) -> Result<WorkspaceScan> {
    let cursor = scan_cursor_project(root)?;
    let steering = load_steering_docs(root);
    let hooks = load_kiro_hooks(root).unwrap_or_default();
    let kiro_mcp = load_mcp_from_path(&root.join(".kiro").join("settings").join("mcp.json"))
        .and_then(|v| v.get("mcpServers").and_then(|m| m.as_object()).map(|o| o.len()))
        .unwrap_or(0);
    Ok(WorkspaceScan {
        cursor,
        steering_count: steering.len(),
        steering,
        kiro_hooks_count: hooks.len(),
        kiro_mcp_count: kiro_mcp,
        antigravity: scan_antigravity_layout(root),
    })
}

pub fn ensure_unified_scaffold(root: &Path) -> Result<Value> {
    let cursor = ensure_cursor_scaffold(root)?;
    let kiro = root.join(".kiro");
    fs::create_dir_all(kiro.join("steering"))?;
    fs::create_dir_all(kiro.join("specs"))?;
    fs::create_dir_all(kiro.join("settings"))?;
    fs::create_dir_all(kiro.join("hooks"))?;
    fs::create_dir_all(root.join(".hooks"))?;
    fs::create_dir_all(root.join(".agent").join("workflows"))?;
    fs::create_dir_all(root.join(".agent").join("rules"))?;
    fs::create_dir_all(root.join(".agent").join("steering"))?;
    fs::create_dir_all(root.join(".agent").join("runs"))?;
    fs::create_dir_all(root.join("specs"))?;
    let _ = crate::antigravity_compat::ensure_antigravity_scaffold(root);

    let steering_readme = kiro.join("steering").join("README.md");
    if !steering_readme.is_file() {
        fs::write(
            &steering_readme,
            "# Steering\n\nKiro-style steering files — always injected into agent context.\n",
        )?;
    }

    Ok(json!({
        "cursor": cursor,
        "created": [
            ".kiro/steering", ".kiro/specs", ".kiro/settings", ".kiro/hooks",
            ".hooks", ".agent/workflows", ".agent/rules", ".agent/steering", ".agent/runs", "specs"
        ]
    }))
}

// ── Agent run persistence (Antigravity-style `.agent/runs/`) ─────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRunRecord {
    pub id: String,
    pub objective: String,
    pub status: String,
    pub started_at: i64,
    #[serde(default)]
    pub finished_at: Option<i64>,
    #[serde(default)]
    pub tool_count: usize,
    #[serde(default)]
    pub summary: Option<String>,
}

pub fn save_agent_run(root: &Path, run: &AgentRunRecord) -> Result<PathBuf> {
    let dir = root.join(".agent").join("runs");
    fs::create_dir_all(&dir)?;
    let path = dir.join(format!("{}.json", run.id));
    fs::write(&path, serde_json::to_string_pretty(run)?)?;
    Ok(path)
}

pub fn list_agent_runs(root: &Path) -> Vec<AgentRunRecord> {
    let dir = root.join(".agent").join("runs");
    if !dir.is_dir() {
        return Vec::new();
    }
    let mut runs = Vec::new();
    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            if let Ok(raw) = fs::read_to_string(&p) {
                if let Ok(r) = serde_json::from_str::<AgentRunRecord>(&raw) {
                    runs.push(r);
                }
            }
        }
    }
    runs.sort_by(|a, b| b.started_at.cmp(&a.started_at));
    runs
}

/// Collect spec task directories from Antigravity `specs/` and Kiro `.kiro/specs/`.
pub fn spec_roots(root: &Path) -> Vec<PathBuf> {
    let mut roots = Vec::new();
    for rel in ["specs", ".kiro/specs"] {
        let p = root.join(rel);
        if p.is_dir() {
            roots.push(p);
        }
    }
    roots
}

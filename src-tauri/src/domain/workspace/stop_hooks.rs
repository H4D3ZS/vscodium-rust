//! Claude Code-style stop hooks — run user commands at agent turn end.

use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::process_ext::hidden_command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopHookDef {
    pub id: String,
    pub command: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopHooksConfig {
    pub hooks: Vec<StopHookDef>,
}

impl Default for StopHooksConfig {
    fn default() -> Self {
        Self { hooks: vec![] }
    }
}

fn hooks_path(root: &str) -> std::path::PathBuf {
    Path::new(root).join(".hades").join("stop_hooks.json")
}

pub fn load_stop_hooks(root: &str) -> StopHooksConfig {
    let path = hooks_path(root);
    if path.exists() {
        if let Ok(text) = std::fs::read_to_string(&path) {
            if let Ok(cfg) = serde_json::from_str(&text) {
                return cfg;
            }
        }
    }
    StopHooksConfig::default()
}

pub fn save_stop_hooks(root: &str, cfg: &StopHooksConfig) -> Result<(), String> {
    let path = hooks_path(root);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    std::fs::write(
        &path,
        serde_json::to_string_pretty(cfg).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())
}

/// Run enabled stop hooks; returns combined stdout/stderr lines.
pub fn run_stop_hooks(root: &str, agent_summary: &str) -> Vec<(String, bool, String)> {
    let cfg = load_stop_hooks(root);
    let mut results = Vec::new();
    for hook in cfg.hooks.iter().filter(|h| h.enabled && !h.command.trim().is_empty()) {
        let mut cmd = if cfg!(windows) {
            hidden_command("cmd")
        } else {
            hidden_command("sh")
        };
        if cfg!(windows) {
            cmd.args(["/C", &hook.command]);
        } else {
            cmd.args(["-c", &hook.command]);
        }
        cmd.env("HADES_ROOT", root);
        cmd.env("HADES_AGENT_SUMMARY", agent_summary);
        let ok = cmd
            .output()
            .map(|o| {
                let out = format!(
                    "{}{}",
                    String::from_utf8_lossy(&o.stdout),
                    String::from_utf8_lossy(&o.stderr)
                );
                (o.status.success(), out.trim().to_string())
            })
            .unwrap_or((false, "hook failed to spawn".to_string()));
        results.push((hook.id.clone(), ok.0, ok.1));
    }
    results
}

#[tauri::command]
pub async fn stop_hooks_get(root: String) -> Result<StopHooksConfig, String> {
    Ok(load_stop_hooks(&root))
}

#[tauri::command]
pub async fn stop_hooks_save(root: String, config: StopHooksConfig) -> Result<(), String> {
    save_stop_hooks(&root, &config)
}

#[tauri::command]
pub async fn stop_hooks_run(root: String, summary: String) -> Result<serde_json::Value, String> {
    let results = run_stop_hooks(&root, &summary);
    Ok(serde_json::json!({
        "results": results.iter().map(|(id, ok, out)| serde_json::json!({
            "id": id, "ok": ok, "output": out
        })).collect::<Vec<_>>()
    }))
}

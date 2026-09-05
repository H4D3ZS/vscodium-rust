//! Editor integration, system info, API key scanning, and fast-apply merge algorithm.
//! Browser, terminal, git, file-edit, search, web, and workflow tools are in their own domain files.
use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use super::registry::AiTools;

impl AiTools {
    pub(crate) async fn find_api_keys(&self, _args: Value) -> Result<Value> {
        let root = self.root_path.lock().await.clone();
        let patterns = [
            (r#"(?i)(api[_-]?key|apikey)\s*[:=]\s*['"]([^'"]+)['"]"#, "API Key"),
            (r#"(?i)(secret[_-]?key|secretkey)\s*[:=]\s*['"]([^'"]+)['"]"#, "Secret Key"),
            (r#"(?i)(access[_-]?token|accesstoken)\s*[:=]\s*['"]([^'"]+)['"]"#, "Access Token"),
            (r#"(?i)(aws[_-]?access[_-]?key[_-]?id)\s*[:=]\s*['"]([^'"]+)['"]"#, "AWS Access Key"),
            (r#"(?i)(private[_-]?key)\s*[:=]\s*['"]([^'"]+)['"]"#, "Private Key"),
        ];
        let mut findings = Vec::new();
        for entry in walkdir::WalkDir::new(&root).into_iter().filter_map(|e| e.ok()) {
            if !entry.file_type().is_file() { continue; }
            let path = entry.path();
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            if !["json","yml","yaml","toml","env","cfg","conf","ini","properties","xml","txt","sql","sh","bash","py","js","ts","rb","go","rs","java","php","dart","swift","kt","scala","gradle","properties","env.local","env.production"].contains(&ext) { continue; }
            if let Ok(content) = std::fs::read_to_string(path) {
                for (pattern, label) in &patterns {
                    if let Ok(re) = regex::Regex::new(pattern) {
                        for cap in re.captures_iter(&content) {
                            if let Some(val) = cap.get(2) {
                                let v = val.as_str();
                                if v.len() > 8 && !v.contains("xxx") && !v.contains("placeholder") && !v.contains("your-") {
                                    let rel = path.strip_prefix(&root).unwrap_or(path).to_string_lossy();
                                    findings.push(json!({
                                        "file": rel.to_string(),
                                        "type": label,
                                        "preview": format!("{}...{}", &v[..4.min(v.len())], &v[v.len().saturating_sub(4)..]),
                                    }));
                                }
                            }
                        }
                    }
                }
            }
            if findings.len() >= 50 { break; }
        }
        Ok(json!({ "status": "success", "findings": findings }))
    }

    pub async fn editor_open_file(&self, args: Value) -> Result<Value> {
        let path = args.get("path").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing path"))?;
        self.emit_tool_event("editor-open-file", json!({ "path": path }));
        Ok(json!({ "status": "success", "opened": path }))
    }

    pub async fn editor_get_active_file(&self, _args: Value) -> Result<Value> {
        if let Some(state) = self.editor_state() {
            let active = state.editor.active_path.lock().await.clone();
            Ok(json!({ "status": "success", "active_file": active }))
        } else {
            Ok(json!({ "status": "success", "active_file": serde_json::Value::Null }))
        }
    }

    pub(crate) async fn get_system_info(&self, _args: Value) -> Result<Value> {
        Ok(json!({
            "status": "success",
            "os": std::env::consts::OS,
            "arch": std::env::consts::ARCH,
        }))
    }

    pub(crate) async fn get_system_health(&self, _args: Value) -> Result<Value> {
        let root = self.root_path.lock().await.clone();
        let git_status = std::process::Command::new("git").arg("status").arg("--porcelain")
            .current_dir(&root).output().map(|o| String::from_utf8_lossy(&o.stdout).to_string())
            .unwrap_or_else(|_| "git not available".to_string());
        let changed_files = git_status.lines().count();
        Ok(json!({
            "status": "success",
            "os": std::env::consts::OS,
            "changed_files": changed_files,
            "root": root.to_string_lossy(),
        }))
    }
}

/// Merge a fast-apply edit sketch into the original file content.
/// The sketch contains changed regions separated by elision markers.
pub(crate) fn merge_fast_apply(original: &str, sketch: &str) -> String {
    let orig_lines: Vec<&str> = original.lines().collect();
    let sketch_lines: Vec<&str> = sketch.lines().collect();
    let mut result = Vec::new();
    let mut orig_idx = 0;
    for line in &sketch_lines {
        let trimmed = line.trim();
        if trimmed.starts_with("// ...") || trimmed.starts_with("# ...") || trimmed.starts_with("<!-- ...") || trimmed == "..." || trimmed == "... existing code ..." {
            continue;
        }
        if orig_idx < orig_lines.len() && orig_lines[orig_idx] == *line {
            result.push(*line);
            orig_idx += 1;
        } else {
            result.push(line);
        }
    }
    while orig_idx < orig_lines.len() {
        result.push(orig_lines[orig_idx]);
        orig_idx += 1;
    }
    result.join("\n")
}

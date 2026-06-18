//! File editing tools: replace, fast_apply, search_replace, shadow patch, patch_file.
use anyhow::{anyhow, Result};
use ropey::Rope;
use serde_json::{json, Value};
use tauri::{Emitter, Manager};
use super::registry::AiTools;

impl AiTools {
    pub(crate) async fn replace_file_content(&self, args: Value) -> Result<Value> {
        let path_str = args.get("path").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing path"))?;
        let old_str = args.get("old_str").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing old_str"))?;
        let new_str = args.get("new_str").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing new_str"))?;
        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, path_str)?;
        let content = std::fs::read_to_string(&full_path)?;
        let new_content = content.replacen(old_str, new_str, 1);
        std::fs::write(&full_path, &new_content)?;
        self.emit_agent_editing(path_str);
        Ok(json!({ "status": "success", "path": path_str }))
    }

    pub(crate) async fn multi_replace_file_content(&self, args: Value) -> Result<Value> {
        let path_str = args.get("path").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing path"))?;
        let replacements = args.get("replacements").and_then(|v| v.as_array())
            .ok_or_else(|| anyhow!("Missing replacements"))?;
        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, path_str)?;
        let mut content = std::fs::read_to_string(&full_path)?;
        for r in replacements {
            if let (Some(old), Some(new)) = (r.get("old_str").and_then(|v| v.as_str()), r.get("new_str").and_then(|v| v.as_str())) {
                content = content.replacen(old, new, 1);
            }
        }
        std::fs::write(&full_path, &content)?;
        self.emit_agent_editing(path_str);
        Ok(json!({ "status": "success", "path": path_str, "replacements": replacements.len() }))
    }

    pub(crate) async fn fast_apply(&self, args: Value) -> Result<Value> {
        let path_str = args.get("path").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing path"))?;
        let edit = args.get("edit").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing edit"))?;
        let dry_run = args.get("dry_run").and_then(|v| v.as_bool()).unwrap_or(false);
        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, path_str)?;
        let original = std::fs::read_to_string(&full_path)?;
        let merged = crate::ai_tools::web_edit::merge_fast_apply(&original, edit);
        if dry_run {
            return Ok(json!({ "status": "success", "merged": merged }));
        }
        std::fs::write(&full_path, &merged)?;
        self.emit_agent_editing(path_str);
        Ok(json!({ "status": "success", "path": path_str }))
    }

    pub(crate) async fn search_replace_edit(&self, args: Value) -> Result<Value> {
        let path_str = args.get("path").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing path"))?;
        let content = args.get("content").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing content"))?;
        let direct_apply = args.get("direct_apply").and_then(|v| v.as_bool()).unwrap_or(false);
        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, path_str)?;
        let original = std::fs::read_to_string(&full_path)?;
        let patches = crate::patch_engine::PatchEngine::parse_search_replace(content);
        if patches.is_empty() {
            return Err(anyhow!("No SEARCH/REPLACE blocks found"));
        }
        let mut rope = Rope::from_str(&original);
        for patch in &patches {
            let search_str = patch.search.trim();
            if search_str.is_empty() { continue; }
            if let Some(pos) = rope.to_string().find(search_str) {
                let start = rope.byte_to_char(pos);
                let end = rope.byte_to_char(pos + search_str.len());
                rope.remove(start..end);
                rope.insert(start, patch.replace.trim());
            } else {
                return Err(anyhow!("SEARCH block not found in file: {}", path_str));
            }
        }
        let new_content = rope.to_string();
        if !direct_apply {
            let shadow = self.shadow_workspace.mirror_file(&full_path.strip_prefix(&root).unwrap_or(&full_path))?;
            std::fs::write(&shadow, &new_content)?;
        } else {
            std::fs::write(&full_path, &new_content)?;
        }
        self.emit_agent_editing(path_str);
        Ok(json!({ "status": "success", "path": path_str, "applied": patches.len() }))
    }

    pub(crate) async fn preview_shadow_diff(&self, args: Value) -> Result<Value> {
        let path_str = args.get("path").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing path"))?;
        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, path_str)?;
        let relative = full_path.strip_prefix(&root).unwrap_or(&full_path);
        let shadow_path = root.join(".hades_cache").join(relative);
        if !shadow_path.exists() {
            return Ok(json!({ "status": "no_changes", "diff": "" }));
        }
        let original = std::fs::read_to_string(&full_path).unwrap_or_default();
        let modified = std::fs::read_to_string(&shadow_path).unwrap_or_default();
        let patch = diffy::create_patch(&original, &modified);
        Ok(json!({ "status": "success", "diff": format!("{}", patch) }))
    }

    pub(crate) async fn apply_shadow_patch(&self, args: Value) -> Result<Value> {
        let path_str = args.get("path").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing path"))?;
        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, path_str)?;
        let relative = full_path.strip_prefix(&root).unwrap_or(&full_path);
        self.shadow_workspace.commit_shadow(relative)?;
        self.emit_agent_editing(path_str);
        Ok(json!({ "status": "success", "path": path_str, "committed": true }))
    }

    pub(crate) async fn ghost_test(&self, args: Value) -> Result<Value> {
        let command = args.get("command").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing command"))?;
        let root = self.root_path.lock().await.clone();
        let output = std::process::Command::new("sh").arg("-c").arg(command)
            .current_dir(&root).output()
            .map_err(|e| anyhow!("Command failed: {e}"))?;
        Ok(json!({
            "status": "success",
            "stdout": String::from_utf8_lossy(&output.stdout),
            "stderr": String::from_utf8_lossy(&output.stderr),
            "exit_code": output.status.code()
        }))
    }

    pub async fn patch_file_content(&self, args: Value) -> Result<Value> {
        let path_str = args.get("path").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing path"))?;
        let start_line = args.get("start_line").and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow!("Missing start_line"))? as usize;
        let end_line = args.get("end_line").and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow!("Missing end_line"))? as usize;
        let new_content = args.get("content").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing content"))?;
        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, path_str)?;
        let original = std::fs::read_to_string(&full_path)?;
        let lines: Vec<&str> = original.lines().collect();
        let mut result = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            let ln = i + 1;
            if ln >= start_line && ln <= end_line {
                if ln == start_line {
                    result.push(new_content);
                }
            } else {
                result.push(line);
            }
        }
        std::fs::write(&full_path, result.join("\n"))?;
        self.emit_agent_editing(path_str);
        Ok(json!({ "status": "success", "path": path_str }))
    }
}

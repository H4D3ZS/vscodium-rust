//! Git operation tools: status, add, commit, diff, log.
use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use std::process::Command;
use super::registry::AiTools;

impl AiTools {
    pub(crate) async fn git_status(&self, _args: Value) -> Result<Value> {
        let root = self.root_path.lock().await.clone();
        let output = Command::new("git").arg("status").arg("--porcelain")
            .current_dir(&root).output()
            .map_err(|e| anyhow!("git status failed: {e}"))?;
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(json!({ "status": "success", "output": stdout }))
    }

    pub(crate) async fn git_add(&self, args: Value) -> Result<Value> {
        let path = args.get("path").and_then(|v| v.as_str()).unwrap_or(".");
        let root = self.root_path.lock().await.clone();
        let output = Command::new("git").arg("add").arg(path)
            .current_dir(&root).output()
            .map_err(|e| anyhow!("git add failed: {e}"))?;
        Ok(json!({ "status": "success", "added": path, "output": String::from_utf8_lossy(&output.stdout) }))
    }

    pub(crate) async fn git_commit(&self, args: Value) -> Result<Value> {
        let message = args.get("message").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing message"))?;
        let root = self.root_path.lock().await.clone();
        let output = Command::new("git").arg("commit").arg("-m").arg(message)
            .current_dir(&root).output()
            .map_err(|e| anyhow!("git commit failed: {e}"))?;
        Ok(json!({ "status": "success", "message": message, "output": String::from_utf8_lossy(&output.stdout) }))
    }

    pub(crate) async fn git_log(&self, args: Value) -> Result<Value> {
        let count = args.get("count").and_then(|v| v.as_u64()).unwrap_or(10);
        let root = self.root_path.lock().await.clone();
        let output = Command::new("git").arg("log").arg(format!("-{}", count)).arg("--oneline")
            .current_dir(&root).output()
            .map_err(|e| anyhow!("git log failed: {e}"))?;
        Ok(json!({ "status": "success", "output": String::from_utf8_lossy(&output.stdout) }))
    }

    pub(crate) async fn git_diff(&self, args: Value) -> Result<Value> {
        let path = args.get("path").and_then(|v| v.as_str());
        let root = self.root_path.lock().await.clone();
        let mut cmd = Command::new("git");
        cmd.arg("diff");
        if let Some(p) = path { cmd.arg(p); }
        let output = cmd.current_dir(&root).output()
            .map_err(|e| anyhow!("git diff failed: {e}"))?;
        Ok(json!({ "status": "success", "output": String::from_utf8_lossy(&output.stdout) }))
    }
}

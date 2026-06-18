//! Browser automation tools: open, navigate, screenshot, click, type, DOM read, subagent.
use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use std::sync::Arc;
use tauri::{Emitter, Manager};
use super::registry::AiTools;

impl AiTools {
    pub(crate) async fn browser_open(&self, _args: Value) -> Result<Value> {
        self.browser_state
            .ensure_started()
            .await
            .map_err(|e| anyhow!("{e}"))?;
        Ok(json!({"status": "success", "message": "Stealth browser launched"}))
    }

    pub(crate) async fn browser_navigate(&self, args: Value) -> Result<Value> {
        let url = args.get("url").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing url"))?
            .to_string();
        let r = self.browser_state
            .cmd("navigate", json!({ "url": url }), 60).await
            .map_err(|e| anyhow!("{e}"))?;
        self.browser_state.refresh_cache(&url).await;
        let status = r.get("status").cloned().unwrap_or(json!(null));
        let headers = r.get("headers").cloned().unwrap_or_else(|| json!({}));
        let sec = ["content-security-policy","strict-transport-security","x-frame-options",
                    "x-content-type-options","referrer-policy","permissions-policy"];
        let missing: Vec<&str> = sec.iter().filter(|h| headers.get(**h).is_none()).cloned().collect();
        Ok(json!({
            "status": "success",
            "url": r.get("url").cloned().unwrap_or(json!(url)),
            "http_status": status,
            "title": r.get("title").cloned().unwrap_or(json!("")),
            "missing_security_headers": missing,
            "response_headers": headers,
        }))
    }

    pub(crate) async fn browser_screenshot(&self, args: Value) -> Result<Value> {
        let r = self.browser_state.cmd("screenshot", json!({}), 30).await
            .map_err(|e| anyhow!("{e}"))?;
        Ok(json!({ "status": "success", "screenshot": r.get("screenshot").cloned().unwrap_or(json!(null)) }))
    }

    pub(crate) async fn browser_click(&self, args: Value) -> Result<Value> {
        let selector = args.get("selector").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing selector"))?;
        self.browser_state.cmd("click", json!({ "selector": selector }), 15).await
            .map_err(|e| anyhow!("{e}"))?;
        Ok(json!({ "status": "success", "clicked": selector }))
    }

    pub(crate) async fn browser_type(&self, args: Value) -> Result<Value> {
        let selector = args.get("selector").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing selector"))?;
        let text = args.get("text").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing text"))?;
        self.browser_state.cmd("fill", json!({ "selector": selector, "value": text }), 15).await
            .map_err(|e| anyhow!("{e}"))?;
        Ok(json!({ "status": "success", "typed": text }))
    }

    pub(crate) async fn browser_read_dom(&self, _args: Value) -> Result<Value> {
        let r = self.browser_state.cmd("content", json!({}), 20).await
            .map_err(|e| anyhow!("{e}"))?;
        Ok(json!({ "status": "success", "html": r.get("html").cloned().unwrap_or(json!("")) }))
    }

    pub(crate) async fn browser_close(&self, _args: Value) -> Result<Value> {
        if let Some(handle) = self.app_handle.lock().await.as_ref() {
            let state: tauri::State<std::sync::Arc<crate::browser::BrowserState>> = handle.state();
            crate::browser::browser_close(state).await.map_err(|e| anyhow!("{}", e))?;
        }
        Ok(json!({ "status": "success", "message": "Browser closed" }))
    }

    pub(crate) async fn browser_capture_vision_context(&self, _args: Value) -> Result<Value> {
        let r = self.browser_state.cmd("content", json!({}), 20).await
            .map_err(|e| anyhow!("{e}"))?;
        let html = r.get("html").and_then(|v| v.as_str()).unwrap_or("");
        let title = r.get("title").and_then(|v| v.as_str()).unwrap_or("");
        Ok(json!({ "title": title, "content_length": html.len(),
                   "content": html.chars().take(5000).collect::<String>() }))
    }

    pub async fn perplexity_proxy(self: Arc<Self>, args: Value) -> Result<Value> {
        let query = args["query"].as_str().unwrap_or("").to_string();
        if query.is_empty() { return Err(anyhow!("perplexity requires a query")); }
        let task = format!("Search the web for: {}. Provide a concise, factual answer with sources.", query);
        Self::browser_subagent(self, json!({ "task": task })).await
    }

    pub async fn browser_subagent(self: Arc<Self>, args: Value) -> Result<Value> {
        let task = args["task"].as_str()
            .ok_or_else(|| anyhow!("Missing task"))?
            .to_string();
        let h = {
            let h_lock = self.app_handle.lock().await;
            h_lock.as_ref().map(|h| h.clone())
                .ok_or_else(|| anyhow!("App handle not set"))?
        };
        let stream_id = format!("browser-sub-{}", uuid::Uuid::new_v4().to_string().chars().take(8).collect::<String>());
        let emit_progress = |step: &str, progress: u32, status: &str| {
            let _ = h.emit("subagent-progress", json!({
                "id": stream_id, "step": step, "progress": progress, "status": status
            }));
        };
        emit_progress("Starting browser research", 10, "running");
        self.browser_state.ensure_started().await.map_err(|e| anyhow!("{e}"))?;
        emit_progress("Browser ready", 20, "running");
        let nav = self.browser_state.cmd("navigate", json!({ "url": format!("https://www.google.com/search?q={}", urlencoding::encode(&task)) }), 60).await
            .map_err(|e| anyhow!("navigate failed: {e}"))?;
        emit_progress("Search results loaded", 40, "running");
        let content = self.browser_state.cmd("content", json!({}), 20).await
            .map_err(|e| anyhow!("content failed: {e}"))?;
        let html = content.get("html").and_then(|v| v.as_str()).unwrap_or("");
        emit_progress("Content extracted", 60, "running");
        let summary: String = html.chars().take(8000).collect();
        emit_progress("Synthesizing answer", 80, "running");
        let answer = format!("Research results for '{}':\n\n{}", task, summary.chars().take(4000).collect::<String>());
        emit_progress("Complete", 100, "success");
        Ok(json!({ "status": "success", "result": answer, "stream_id": stream_id }))
    }
}

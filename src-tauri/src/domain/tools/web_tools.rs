//! Web fetch and search tools.
use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use tauri::{Emitter, Manager};
use super::registry::AiTools;

impl AiTools {
    pub(crate) async fn web_fetch_tool(&self, args: Value) -> Result<Value> {
        let url = args.get("url").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing url"))?;
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build().map_err(|e| anyhow!("Client build: {e}"))?;
        let resp = client.get(url).send().await.map_err(|e| anyhow!("Fetch: {e}"))?;
        let status = resp.status().as_u16();
        let body = resp.text().await.map_err(|e| anyhow!("Read: {e}"))?;
        Ok(json!({
            "status": "success", "http_status": status,
            "content_length": body.len(),
            "content": body.chars().take(10000).collect::<String>()
        }))
    }

    pub(crate) async fn web_search_tool(&self, args: Value) -> Result<Value> {
        let query = args.get("query").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing query"))?;
        let max_results = args.get("max_results").and_then(|v| v.as_u64()).unwrap_or(5) as usize;
        self.browser_state.ensure_started().await.map_err(|e| anyhow!("{e}"))?;
        let search_url = format!("https://www.google.com/search?q={}", urlencoding::encode(query));
        let nav = self.browser_state.cmd("navigate", json!({ "url": search_url }), 30).await
            .map_err(|e| anyhow!("navigate: {e}"))?;
        let content = self.browser_state.cmd("content", json!({}), 15).await
            .map_err(|e| anyhow!("content: {e}"))?;
        let html = content.get("html").and_then(|v| v.as_str()).unwrap_or("");
        let re = regex::Regex::new(r#"<a\s+[^>]*?href="(https?://[^"]*)"[^>]*>(.*?)</a>"#)
            .map_err(|e| anyhow!("regex: {e}"))?;
        let mut results = Vec::new();
        for cap in re.captures_iter(html) {
            if results.len() >= max_results { break; }
            let href = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let text = cap.get(2).map(|m| regex::Regex::new(r"<[^>]*>").unwrap().replace_all(m.as_str(), "").to_string()).unwrap_or_default();
            if !href.is_empty() && !text.is_empty() && !href.contains("google.com") {
                results.push(json!({ "url": href, "title": text.chars().take(200).collect::<String>() }));
            }
        }
        Ok(json!({ "status": "success", "query": query, "results": results }))
    }
}

//! Search and analysis tools: search_codebase, grep, find_by_name, analyze_symbols, etc.
use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use tauri::{Emitter, Manager};
use super::registry::AiTools;

impl AiTools {
    pub(crate) async fn search_codebase(&self, args: Value) -> Result<Value> {
        let query = args.get("query").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing query"))?.to_string();
        let max_results = args.get("max_results").and_then(|v| v.as_u64()).unwrap_or(30) as usize;
        let q_lower = query.to_lowercase();
        let root = self.root_path.lock().await.clone();
        let ignore_set = crate::cursor_compat::CursorIgnoreSet::load(&root, crate::cursor_compat::IgnoreScope::AiAccess);
        let mut text_matches = Vec::new();
        for entry in walkdir::WalkDir::new(&root).into_iter().filter_map(|e| e.ok()).filter(|e| e.file_type().is_file()) {
            let path = entry.path();
            if ignore_set.is_ignored(path) { continue; }
            if let Ok(content) = std::fs::read_to_string(path) {
                for (i, line) in content.lines().enumerate() {
                    if line.to_lowercase().contains(&q_lower) {
                        let rel = path.strip_prefix(&root).map(|p| p.to_string_lossy().to_string())
                            .unwrap_or_else(|_| path.to_string_lossy().to_string());
                        text_matches.push(json!({ "file": rel, "line": i + 1, "preview": line.trim() }));
                        if text_matches.len() >= max_results { break; }
                    }
                }
            }
            if text_matches.len() >= max_results { break; }
        }
        let sym_defs = self.memory_store.query_symbols(&q_lower, 20).await;
        let symbol_matches: Vec<Value> = sym_defs.iter().map(|s| json!({
            "symbol": s.name, "kind": s.kind, "file": s.path, "line_start": s.line_range.0
        })).collect();
        Ok(json!({ "status": "success", "text_matches": text_matches, "symbol_matches": symbol_matches }))
    }

    pub(crate) async fn get_lsp_diagnostics(&self, args: Value) -> Result<Value> {
        let path_str = args.get("path").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing path"))?;
        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, path_str)?;
        let uri = format!("file:///{}", full_path.to_string_lossy().replace('\\', "/"));
        if let Some(handle) = self.app_handle.lock().await.as_ref() {
            let state: tauri::State<crate::EditorState> = handle.state();
            let router = state.editor.lsp_router.lock().await;
            if let Some(client_arc) = router.client_for_uri(&uri).await {
                let mut client = client_arc.lock().await;
                if let Ok(result) = client.request_with_response("textDocument/diagnostic", json!({
                    "textDocument": { "uri": uri }
                })).await {
                    return Ok(json!({ "status": "success", "diagnostics": result }));
                }
            }
        }
        Ok(json!({ "status": "success", "diagnostics": [] }))
    }

    pub(crate) async fn grep(&self, args: Value) -> Result<Value> {
        let query = args.get("query").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing query"))?;
        let path = args.get("path").and_then(|v| v.as_str());
        let root = self.root_path.lock().await.clone();
        let search_path = path.map(|p| root.join(p)).unwrap_or(root.clone());
        let output = std::process::Command::new("rg").arg("--json").arg(query).arg(&search_path)
            .output();
        let stdout = match output {
            Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
            Err(_) => return Ok(json!({ "status": "success", "results": Vec::<Value>::new() })),
        };
        let mut results = Vec::new();
        for line in stdout.lines() {
            if let Ok(val) = serde_json::from_str::<Value>(line) {
                if val["type"] == "match" {
                    let text = val["data"]["path"]["text"].as_str().unwrap_or("");
                    let line_num = val["data"]["line_number"].as_u64().unwrap_or(0);
                    let content = val["data"]["lines"]["text"].as_str().unwrap_or("");
                    results.push(json!({ "file": text, "line": line_num, "content": content.trim() }));
                }
            }
        }
        Ok(json!({ "status": "success", "results": results }))
    }

    pub(crate) async fn find_by_name(&self, args: Value) -> Result<Value> {
        let query = args.get("query").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing query"))?;
        let path = args.get("path").and_then(|v| v.as_str());
        let root = self.root_path.lock().await.clone();
        let search_root = path.map(|p| root.join(p)).unwrap_or(root.clone());
        let mut matches = Vec::new();
        for entry in walkdir::WalkDir::new(&search_root).into_iter().filter_map(|e| e.ok()) {
            let name = entry.file_name().to_string_lossy().to_lowercase();
            if name.contains(&query.to_lowercase()) {
                let rel = entry.path().strip_prefix(&root).unwrap_or(entry.path())
                    .to_string_lossy().to_string();
                matches.push(rel);
                if matches.len() >= 50 { break; }
            }
        }
        Ok(json!({ "status": "success", "matches": matches }))
    }

    pub(crate) async fn get_directory_structure(&self, args: Value) -> Result<Value> {
        let path = args.get("path").and_then(|v| v.as_str()).unwrap_or(".");
        let root = self.root_path.lock().await.clone();
        let target = root.join(path);
        let mut entries = Vec::new();
        if let Ok(read_dir) = std::fs::read_dir(&target) {
            for entry in read_dir.filter_map(|e| e.ok()) {
                let name = entry.file_name().to_string_lossy().to_string();
                let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
                entries.push(json!({ "name": name, "type": if is_dir { "dir" } else { "file" } }));
            }
        }
        Ok(json!({ "status": "success", "path": path, "entries": entries }))
    }

    pub async fn analyze_file_symbols(&self, args: Value) -> Result<Value> {
        let path_str = args.get("path").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing path"))?;
        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, path_str)?;
        let content = std::fs::read_to_string(&full_path)?;
        let ext = full_path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let symbols = crate::context_indexer::ContextIndexer::extract_symbols(&content, ext);
        Ok(json!({ "status": "success", "path": path_str, "symbols": symbols }))
    }

    pub async fn get_symbol_graph(&self, _args: Value) -> Result<Value> {
        Ok(json!({ "status": "success", "symbol_count": 0 }))
    }
}

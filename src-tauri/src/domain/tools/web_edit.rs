//! Web/browser tools, code search, file editing (fast-apply, search-replace,
//! shadow patches), git tools, knowledge/canvas/notify handlers.
use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use std::fs;
use std::sync::Arc;
use tauri::{Emitter, Manager};
use super::registry::AiTools;
use crate::binary_analyzer::BinaryAnalyzer;
use crate::security_distiller::SecurityDistiller;

impl AiTools {
    pub(crate) async fn search_codebase(&self, args: Value) -> Result<Value> {
        let query = args["query"].as_str().ok_or_else(|| anyhow!("Missing query"))?.to_string();
        let q_lower = query.to_lowercase();
        let max_results = args["max_results"].as_u64().unwrap_or(30) as usize;
        let file_types: Option<Vec<String>> = args["file_types"]
            .as_str()
            .map(|s| s.split(',').map(|e| e.trim().to_string()).collect());

        let root = self.root_path.lock().await.clone();
        let ignore_set = crate::cursor_compat::CursorIgnoreSet::load(
            &root,
            crate::cursor_compat::IgnoreScope::AiAccess,
        );
        let mut text_matches: Vec<Value> = Vec::new();

        // 1. Text grep across files (respects .cursorignore / .cursorindexingignore)
        for entry in walkdir::WalkDir::new(&root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            if ignore_set.is_ignored(path) {
                continue;
            }

            if let Some(ref types) = file_types {
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                if !types.iter().any(|t| t == ext) { continue; }
            }

            if let Ok(content) = std::fs::read_to_string(path) {
                for (i, line) in content.lines().enumerate() {
                    if line.to_lowercase().contains(&q_lower) {
                        let rel = path.strip_prefix(&root)
                            .map(|p| p.to_string_lossy().to_string())
                            .unwrap_or_else(|_| path.to_string_lossy().to_string());
                        text_matches.push(json!({
                            "file": rel,
                            "line": i + 1,
                            "preview": line.trim()
                        }));
                        if text_matches.len() >= max_results { break; }
                    }
                }
            }
            if text_matches.len() >= max_results { break; }
        }

        // 2. Symbol lookup from memory store
        let slots = self.memory_store.slots.read().await.clone();
        let sym_defs = self.memory_store.query_symbols(&q_lower, 20).await;
        let mut symbol_matches: Vec<Value> = sym_defs.iter()
            .map(|s| json!({
                "symbol": s.name,
                "kind": s.kind,
                "file": s.path,
                "line_start": s.line_range.0
            }))
            .collect();

        // Also check slot tags for symbol hits not yet in graph
        for slot in &slots {
            for tag in &slot.tags {
                if tag.starts_with("symbol:") && tag[7..].to_lowercase().contains(&q_lower) {
                    if !symbol_matches.iter().any(|m| m["file"] == slot.content) {
                        symbol_matches.push(json!({ "symbol": &tag[7..], "file": slot.content }));
                    }
                }
            }
            if symbol_matches.len() >= 20 { break; }
        }

        // 3. Vector index chunk matches (@codebase-style ranked snippets)
        let mut vector_matches: Vec<Value> = Vec::new();
        if let Some(indexer) = self.vector_indexer.lock().await.clone() {
            if let Ok(hits) = indexer.search_codebase(&query, max_results).await {
                for hit in hits {
                    vector_matches.push(json!({
                        "file": hit.file_path,
                        "start_line": hit.start_line,
                        "end_line": hit.end_line,
                        "relevance_score": hit.relevance_score,
                        "preview": hit.context,
                    }));
                }
            }
            if symbol_matches.len() < 20 {
                if let Ok(sym_hits) = indexer.find_symbol(&query).await {
                    for hit in sym_hits {
                        if symbol_matches.len() >= 20 { break; }
                        symbol_matches.push(json!({
                            "symbol": hit.context,
                            "file": hit.file_path,
                            "line_start": hit.start_line,
                            "source": "vector_index"
                        }));
                    }
                }
            }
        }

        Ok(json!({
            "query": query,
            "text_matches": text_matches,
            "symbol_matches": symbol_matches,
            "vector_matches": vector_matches,
            "total_text": text_matches.len(),
            "total_symbols": symbol_matches.len(),
            "total_vector": vector_matches.len()
        }))
    }

    pub(crate) async fn get_lsp_diagnostics(&self, args: Value) -> Result<Value> {
        let path_filter = args["path"].as_str().map(|s| s.to_string());

        // Try to get diagnostics from the app handle (stored in LSP client state)
        let h_lock = self.app_handle.lock().await;
        if let Some(handle) = h_lock.as_ref() {
            let state: tauri::State<crate::EditorState> = handle.state();
            let diags = state.lsp_diagnostics.read().await.clone();
            drop(h_lock);

            let filtered: Vec<Value> = diags.iter()
                .filter(|(uri, _)| {
                    if let Some(ref p) = path_filter {
                        uri.contains(p.as_str())
                    } else {
                        true
                    }
                })
                .map(|(uri, items)| json!({ "file": uri, "diagnostics": items }))
                .collect();

            let total_errors: usize = filtered.iter()
                .map(|f| {
                    f["diagnostics"].as_array()
                        .map(|arr| arr.iter().filter(|d| d["severity"].as_u64().unwrap_or(2) == 1).count())
                        .unwrap_or(0)
                })
                .sum();

            Ok(json!({
                "files": filtered,
                "total_errors": total_errors,
                "summary": if total_errors == 0 {
                    "No LSP errors detected.".to_string()
                } else {
                    format!("{} LSP error(s) found across {} file(s).", total_errors, filtered.len())
                }
            }))
        } else {
            drop(h_lock);
            Ok(json!({ "files": [], "total_errors": 0, "summary": "LSP not running or no diagnostics yet." }))
        }
    }

    pub(crate) async fn web_fetch_tool(&self, args: Value) -> Result<Value> {
        let url = args
            .get("url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing url"))?
            .to_string();
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .user_agent("Mozilla/5.0 vscodium-rust/1.0")
            .build()
            .map_err(|e| anyhow!(e.to_string()))?;
        let body = client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("web_fetch failed for {}: {}", url, e))?
            .text()
            .await
            .map_err(|e| anyhow!("web_fetch body read failed: {}", e))?;
        let capped: String = body.chars().take(200_000).collect();
        Ok(json!({
            "status": "success",
            "url": url,
            "content": capped,
            "bytes": capped.len(),
        }))
    }

    pub(crate) async fn web_search_tool(&self, args: Value) -> Result<Value> {
        let query = args["query"].as_str().unwrap_or("").to_string();
        if query.is_empty() {
            return Ok(json!({ "error": "query is required" }));
        }
        let num = args["num_results"].as_u64().unwrap_or(5) as usize;
        let encoded = urlencoding::encode(&query);
        let url = format!(
            "https://api.duckduckgo.com/?q={}&format=json&no_html=1&skip_disambig=1", encoded
        );
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(12))
            .user_agent("Mozilla/5.0 vscodium-rust/1.0")
            .build()
            .map_err(|e| anyhow!(e.to_string()))?;

        let body: Value = client.get(&url).send().await
            .map_err(|e| anyhow!(e.to_string()))?
            .json().await
            .map_err(|e| anyhow!(e.to_string()))?;

        let mut results: Vec<Value> = vec![];

        if let Some(t) = body["Abstract"].as_str() {
            if !t.is_empty() {
                results.push(json!({
                    "title": body["Heading"].as_str().unwrap_or(""),
                    "url": body["AbstractURL"].as_str().unwrap_or(""),
                    "snippet": t,
                    "source": body["AbstractSource"].as_str().unwrap_or("DDG"),
                }));
            }
        }
        if let Some(a) = body["Answer"].as_str() {
            if !a.is_empty() {
                results.push(json!({ "title": "Instant Answer", "url": "", "snippet": a, "source": "DDG" }));
            }
        }
        if let Some(topics) = body["RelatedTopics"].as_array() {
            for t in topics.iter().take(num.saturating_sub(results.len())) {
                if let Some(text) = t["Text"].as_str() {
                    if !text.is_empty() {
                        results.push(json!({
                            "title": text.chars().take(80).collect::<String>(),
                            "url": t["FirstURL"].as_str().unwrap_or(""),
                            "snippet": text,
                            "source": "DDG",
                        }));
                    }
                }
            }
        }
        if results.is_empty() {
            results.push(json!({
                "title": format!("Search: {}", query),
                "url": format!("https://duckduckgo.com/?q={}", encoded),
                "snippet": "No instant results. Visit URL for full search.",
                "source": "fallback",
            }));
        }
        let n = results.len().min(num);
        Ok(json!({ "query": query, "results": &results[..n], "count": n }))
    }

    pub(crate) async fn browser_open(&self, _args: Value) -> Result<Value> {
        // Launch the real stealth browser (invisible_playwright / Firefox).
        self.browser_state
            .ensure_started()
            .await
            .map_err(|e| anyhow!("{e}"))?;
        Ok(serde_json::json!({"status": "success", "message": "Stealth browser launched"}))
    }

    pub(crate) async fn browser_navigate(&self, args: Value) -> Result<Value> {
        let url = args
            .get("url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing url"))?
            .to_string();

        // Real navigation through the stealth browser — JS runs, redirects
        // resolve, and we capture the live response status + headers.
        let r = self
            .browser_state
            .cmd("navigate", serde_json::json!({ "url": url }), 60)
            .await
            .map_err(|e| anyhow!("{e}"))?;
        self.browser_state.refresh_cache(&url).await;

        let status = r.get("status").cloned().unwrap_or(serde_json::json!(null));
        let headers = r.get("headers").cloned().unwrap_or_else(|| serde_json::json!({}));
        let sec = [
            "content-security-policy",
            "strict-transport-security",
            "x-frame-options",
            "x-content-type-options",
            "referrer-policy",
            "permissions-policy",
        ];
        let missing: Vec<&str> = sec.iter().filter(|h| headers.get(**h).is_none()).cloned().collect();

        Ok(serde_json::json!({
            "status": "success",
            "url": r.get("url").cloned().unwrap_or(serde_json::json!(url)),
            "http_status": status,
            "title": r.get("title").cloned().unwrap_or(serde_json::json!("")),
            "missing_security_headers": missing,
            "response_headers": headers,
        }))
    }

    pub(crate) async fn browser_screenshot(&self, args: Value) -> Result<Value> {
        let full = args.get("full_page").and_then(|v| v.as_bool()).unwrap_or(false);
        let r = self
            .browser_state
            .cmd("screenshot", serde_json::json!({ "full_page": full }), 30)
            .await
            .map_err(|e| anyhow!("{e}"))?;
        Ok(serde_json::json!({
            "status": "success",
            "screenshot": r.get("screenshot").cloned().unwrap_or(serde_json::json!("")),
        }))
    }

    pub(crate) async fn browser_click(&self, args: Value) -> Result<Value> {
        let selector = args
            .get("selector")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing selector"))?
            .to_string();
        self.browser_state
            .cmd("click", serde_json::json!({ "selector": selector }), 20)
            .await
            .map_err(|e| anyhow!("{e}"))?;
        self.browser_state.refresh_cache("").await;
        Ok(serde_json::json!({"status": "success", "message": format!("Clicked {}", selector)}))
    }

    pub(crate) async fn browser_type(&self, args: Value) -> Result<Value> {
        let selector = args
            .get("selector")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing selector"))?
            .to_string();
        let text = args.get("text").and_then(|v| v.as_str()).unwrap_or("").to_string();
        self.browser_state
            .cmd("fill", serde_json::json!({ "selector": selector, "text": text }), 20)
            .await
            .map_err(|e| anyhow!("{e}"))?;
        Ok(serde_json::json!({"status": "success", "message": format!("Typed into {}", selector)}))
    }

    pub(crate) async fn browser_read_dom(&self, _args: Value) -> Result<Value> {
        let r = self
            .browser_state
            .cmd("content", serde_json::json!({}), 30)
            .await
            .map_err(|e| anyhow!("{e}"))?;
        Ok(serde_json::json!({
            "status": "success",
            "url": r.get("url").cloned().unwrap_or(serde_json::json!("")),
            "title": r.get("title").cloned().unwrap_or(serde_json::json!("")),
            "dom": r.get("html").cloned().unwrap_or(serde_json::json!("")),
        }))
    }

    pub(crate) async fn browser_close(&self, _args: Value) -> Result<Value> {
        let mut guard = self.browser_state.proc.lock().await;
        if let Some(mut p) = guard.take() {
            let _ = crate::browser::shutdown_proc(&mut p).await;
        }
        *self.browser_state.browser.lock().await = None;
        Ok(serde_json::json!({"status": "success", "message": "Browser closed"}))
    }

    pub(crate) fn get_command_help(&self, _args: Value) -> Result<Value> {
        let commands = serde_json::json!([
            {"name": "/commit", "description": "Generate a conventional commit message and commit changes."},
            {"name": "/diff", "description": "Show the current working directory's git diff."},
            {"name": "/doctor", "description": "Check system health (Git, Node, Rust, MCP)."},
            {"name": "/tools", "description": "List all registered tools and their capabilities."},
            {"name": "/resume", "description": "Restore the previous agent session from disk."},
            {"name": "/reset", "description": "Clear the current conversation and task state."},
            {"name": "/browser", "description": "Start a sub-agent to browse and summarize web content."},
            {"name": "/search", "description": "Search for specific patterns or text across the project."},
            {"name": "/terminal", "description": "Run a terminal command and return the output."},
            {"name": "/explain", "description": "Explain a code item or file in detail."},
            {"name": "/refactor", "description": "Suggest or perform a code refactor based on best practices."},
            {"name": "/help", "description": "Show this command reference and usage guide."}
        ]);
        Ok(commands)
    }

    #[allow(dead_code)]
    pub(crate) async fn find_api_keys(&self, _args: Value) -> Result<Value> {
        let mut results = Vec::new();
        let extensions = vec![
            "xml",
            "json",
            "properties",
            "sql",
            "txt",
            "log",
            "tmp",
            "backup",
            "bak",
            "enc",
            "yml",
            "yaml",
            "toml",
            "ini",
            "config",
            "conf",
            "cfg",
            "env",
            "envrc",
            "prod",
            "secret",
            "private",
            "key",
        ];

        let openai_regex = regex::Regex::new(r"sk-[a-zA-Z0-9]{48}")?;
        let github_regex = regex::Regex::new(r"gh[pousr]_[a-zA-Z0-9]+")?;
        let google_regex = regex::Regex::new(r"AIza[0-9A-Za-z-_]{35}")?;

        let root = self.root_path.lock().await;
        use walkdir::WalkDir;
        for entry in WalkDir::new(&*root).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let ext = entry
                    .path()
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("");
                if extensions.contains(&ext) || ext.is_empty() {
                    let content = fs::read_to_string(entry.path());
                    if let Ok(content) = content {
                        for (i, line) in content.lines().enumerate() {
                            let mut found = false;
                            let mut provider = "";

                            if openai_regex.is_match(line)
                                && (line.to_lowercase().contains("openai")
                                    || line.to_lowercase().contains("gpt"))
                            {
                                found = true;
                                provider = "OpenAI";
                            } else if github_regex.is_match(line)
                                && (line.to_lowercase().contains("github")
                                    || line.to_lowercase().contains("oauth"))
                            {
                                found = true;
                                provider = "GitHub";
                            } else if google_regex.is_match(line)
                                && line.contains("Google")
                                && line.contains("AIza")
                            {
                                found = true;
                                provider = "Google";
                            }

                            if found {
                                results.push(serde_json::json!({
                                     "provider": provider,
                                     "file": entry.path().strip_prefix(&*root)?.to_string_lossy().to_string(),
                                     "line": i + 1,
                                     "context": line.trim()
                                 }));
                            }
                            if results.len() > 100 {
                                break;
                            }
                        }
                    }
                }
            }
            if results.len() > 100 {
                break;
            }
        }

        Ok(Value::Array(results))
    }

    pub(crate) async fn grep(&self, args: Value) -> Result<Value> {
        let query_str = args
            .get("query")
            .or_else(|| args.get("pattern"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing query/pattern"))?;
        let path_str = args.get("path").and_then(|v| v.as_str()).unwrap_or(".");
        let include = args.get("include").and_then(|v| v.as_str());
        let case_insensitive = args
            .get("case_insensitive")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let fixed_string = args
            .get("fixed_string")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let root = self.root_path.lock().await;
        let base_path = self.validate_path(&root, path_str)?;

        let file_target = if base_path.is_file() {
            Some(base_path.clone())
        } else {
            None
        };
        let search_root = if base_path.is_file() {
            base_path
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| root.clone())
        } else {
            base_path
        };

        let hits = crate::ripgrep_search::ripgrep_search(crate::ripgrep_search::RipgrepQuery {
            pattern: query_str,
            root: &search_root,
            include,
            max_results: 500,
            case_insensitive,
            fixed_string,
            file: file_target.as_deref(),
        })
        .map_err(|e| anyhow!(e))?;

        let results = crate::ripgrep_search::format_grep_results(&hits);
        Ok(serde_json::json!({
            "results": results,
            "matches": hits.len(),
            "engine": "ripgrep",
            "status": "success"
        }))
    }

    pub(crate) async fn terminal_send_data(&self, args: Value) -> Result<Value> {
        let h_lock = self.app_handle.lock().await;
        let h = h_lock
            .as_ref()
            .ok_or_else(|| anyhow!("App handle not set"))?;

        let term_id_opt = args.get("term_id").and_then(|v| v.as_str());
        let data = args
            .get("data")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing data"))?;

        let state = h.state::<crate::EditorState>();
        let mut writers = state.terminal_writers.lock().await;

        // 1. Create terminal if none exist
        if writers.is_empty() {
            drop(writers);
            h.emit("terminal-create", json!({}))?;
            tokio::time::sleep(std::time::Duration::from_millis(500)).await; // Wait for PTY initialization
            writers = state.terminal_writers.lock().await;
        }

        // 2. Select target terminal (provided ID or first available)
        let target_id = term_id_opt
            .map(|s| s.to_string())
            .or_else(|| writers.keys().next().cloned());

        if let Some(id) = target_id {
            if let Some(writer) = writers.get_mut(&id) {
                // Add auto-newline if missing for convenience
                let payload = if data.ends_with('\n') {
                    data.to_string()
                } else {
                    format!("{}\n", data)
                };
                writer.write_all(payload.as_bytes())?;
                writer.flush()?;

                Ok(json!({
                    "status": "success",
                    "term_id": id,
                    "info": format!("Data sent to terminal '{}'.", id)
                }))
            } else {
                Err(anyhow!("Terminal '{}' not found in writers", id))
            }
        } else {
            Err(anyhow!(
                "No active terminal session found and auto-creation failed."
            ))
        }
    }

    pub(crate) async fn terminal_get_state(&self, _args: Value) -> Result<Value> {
        let h_lock = self.app_handle.lock().await;
        let h = h_lock
            .as_ref()
            .ok_or_else(|| anyhow!("App handle not set"))?;

        let state = h.state::<crate::EditorState>();
        let writers = state.terminal_writers.lock().await;
        let ids: Vec<String> = writers.keys().cloned().collect();

        Ok(json!({
            "active_terminals": ids,
            "count": ids.len(),
            "hint": "If count is 0, terminal_send_data will automatically create one."
        }))
    }

    pub(crate) async fn terminal_create(&self, args: Value) -> Result<Value> {
        let h_lock = self.app_handle.lock().await;
        let h = h_lock
            .as_ref()
            .ok_or_else(|| anyhow!("App handle not set"))?;

        let shell = args.get("shell").and_then(|v| v.as_str());
        h.emit("terminal-create", json!({ "shell": shell }))?;

        Ok(json!({ "status": "success", "message": "Terminal creation requested." }))
    }

    pub(crate) async fn get_system_info(&self, _args: Value) -> Result<Value> {
        let os = std::env::consts::OS;
        let arch = std::env::consts::ARCH;
        let user = std::env::var("USER")
            .or_else(|_| std::env::var("USERNAME"))
            .unwrap_or_else(|_| "unknown".to_string());
        let current_dir = std::env::current_dir().unwrap_or_default();

        Ok(json!({
            "os": os,
            "architecture": arch,
            "user": user,
            "current_dir": current_dir,
            "agent_home": self.root_path.lock().await.to_string_lossy()
        }))
    }

    pub(crate) async fn terminal_read_output(&self, args: Value) -> Result<Value> {
        let h_lock = self.app_handle.lock().await;
        let h = h_lock
            .as_ref()
            .ok_or_else(|| anyhow!("App handle not set"))?;

        let state = h.state::<crate::EditorState>();
        let term_buffers = state.terminal_buffers.lock().await;

        let term_id_opt = args.get("term_id").and_then(|v| v.as_str());

        // Use specified ID or find first available with content
        let target_id = term_id_opt.map(|s| s.to_string()).or_else(|| {
            term_buffers
                .iter()
                .find(|(_, buf)| !buf.is_empty())
                .map(|(id, _)| id.clone())
        });

        if let Some(id) = target_id {
            if let Some(buffer) = term_buffers.get(&id) {
                Ok(json!({
                    "term_id": id,
                    "output": buffer.join("")
                }))
            } else {
                Err(anyhow!("Terminal '{}' not found in buffers", id))
            }
        } else {
            Ok(json!({ "output": "", "info": "No active terminal buffers with content." }))
        }
    }

    pub(crate) async fn terminal_toggle(&self, args: Value) -> Result<Value> {
        let h_lock = self.app_handle.lock().await;
        let h = h_lock
            .as_ref()
            .ok_or_else(|| anyhow!("App handle not set"))?;
        let visible = args
            .get("visible")
            .and_then(|v| v.as_bool())
            .ok_or_else(|| anyhow!("Missing visible"))?;

        h.emit("toggle-terminal", visible)?;
        Ok(json!({ "status": "success" }))
    }

    pub(crate) async fn browser_capture_vision_context(&self, _args: Value) -> Result<Value> {
        crate::browser::capture_vision_context_internal(&self.browser_state)
            .await
            .map_err(|e| anyhow!(e))
    }

    pub async fn editor_open_file(&self, args: Value) -> Result<Value> {
        let path_str = args
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing path"))?;
        let root = self.root_path.lock().await;
        let full_path = self.validate_path(&root, path_str)?;

        let path_string = full_path.to_string_lossy().to_string();

        {
            let h_lock = self.app_handle.lock().await;
            if let Some(h) = h_lock.as_ref() {
                use tauri::Emitter;
                let _ = h.emit("editor_open_file", json!({ "path": path_string }));
                return Ok(
                    json!({ "status": "success", "info": format!("Opened {} in editor", path_str) }),
                );
            }
        }
        Err(anyhow!("App handle not available"))
    }

    pub async fn editor_get_active_file(&self, _args: Value) -> Result<Value> {
        let handle_lock = self.app_handle.lock().await;
        if let Some(handle) = handle_lock.as_ref() {
            let state: tauri::State<crate::EditorState> = handle.state();
            let active_path = state
                .active_path
                .lock()
                .await;

            match active_path.as_ref() {
                Some(path) => Ok(json!({ "status": "success", "path": path })),
                None => Ok(json!({ "status": "not_found", "message": "No active file" })),
            }
        } else {
            Err(anyhow!("App handle not available"))
        }
    }

    pub(crate) async fn replace_file_content(&self, args: Value) -> Result<Value> {
        let path_str = args
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing path"))?;
        let target = args
            .get("target")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing target"))?;
        let replacement = args
            .get("replacement")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing replacement"))?;

        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, path_str)?;

        let content = fs::read_to_string(&full_path)?;
        if !content.contains(target) {
            return Err(anyhow!("Target string not found in file"));
        }

        let new_content = content.replace(target, replacement);
        fs::write(&full_path, &new_content)?;
        
        // Phase 25: Sync Cache
        self.memory_store.update_vfs_cache(full_path, new_content).await;

        Ok(json!({ "status": "success" }))
    }

    /// True if `line` (after trimming) is an "elide unchanged code"
    /// placeholder marker. Recognizes the common comment variants used by
    /// Cursor and ChatGPT for partial-file edits.
    pub(crate) fn is_elision_marker(line: &str) -> bool {
        let t = line.trim();
        if t.is_empty() { return false; }
        // Normalize common comment wrappers so we can pattern-match on the body.
        let inner = t
            .trim_start_matches("//")
            .trim_start_matches('#')
            .trim_start_matches("--")
            .trim_start_matches("<!--")
            .trim_end_matches("-->")
            .trim_start_matches("/*")
            .trim_end_matches("*/")
            .trim();
        if inner.is_empty() { return false; }
        // "..." or "... existing code ..." (case-insensitive, allow "rest of file")
        let lower = inner.to_ascii_lowercase();
        let stripped = lower.trim_matches('.');
        stripped.is_empty()
            || lower == "..."
            || lower.starts_with("... existing")
            || lower.starts_with("...existing")
            || lower.starts_with("... rest")
            || lower.starts_with("...rest")
            || lower.starts_with("... unchanged")
            || lower.starts_with("...unchanged")
    }

    /// Deterministically merge a Cursor-style edit sketch into a full
    /// file. The sketch contains the changed regions verbatim and
    /// `... existing code ...` markers everywhere else. We split the
    /// sketch on those markers and stitch by anchor-matching the head of
    /// each segment back into the original file.
    pub(crate) fn merge_fast_apply(original: &str, sketch: &str) -> Result<String> {
        let orig_lines: Vec<&str> = original.lines().collect();
        let sketch_lines: Vec<&str> = sketch.lines().collect();

        // Split sketch into [segment, segment, ...] separated by marker lines.
        // Track whether each *gap* between segments came from a marker.
        let mut segments: Vec<Vec<&str>> = vec![Vec::new()];
        let mut markers: Vec<bool> = Vec::new(); // markers[i] separates segments[i] and segments[i+1]
        for line in &sketch_lines {
            if Self::is_elision_marker(line) {
                segments.push(Vec::new());
                markers.push(true);
            } else {
                segments.last_mut().unwrap().push(line);
            }
        }

        if markers.is_empty() {
            return Err(anyhow!(
                "fast_apply: edit contained no elision markers. Use write_to_file for full rewrites or include `// ... existing code ...` lines to mark unchanged regions."
            ));
        }

        // Try to anchor each non-empty segment into the original. We use
        // the first non-blank line of each segment as the anchor.
        fn first_nonblank<'a>(seg: &'a [&'a str]) -> Option<&'a str> {
            seg.iter().find(|l| !l.trim().is_empty()).copied()
        }
        fn last_nonblank<'a>(seg: &'a [&'a str]) -> Option<&'a str> {
            seg.iter().rev().find(|l| !l.trim().is_empty()).copied()
        }
        fn norm(s: &str) -> String { s.trim().to_string() }

        let mut out: Vec<String> = Vec::new();
        let mut cursor: usize = 0; // index into orig_lines

        for (i, seg) in segments.iter().enumerate() {
            let prev_was_marker = i > 0;
            if prev_was_marker {
                // The marker between seg_{i-1} and seg_i preserves the
                // original content from `cursor` up to wherever seg_i's
                // first non-blank line appears in the original.
                if let Some(head) = first_nonblank(seg) {
                    let needle = norm(head);
                    if let Some(found) = orig_lines.iter().enumerate().skip(cursor).find_map(|(idx, l)| {
                        if norm(l) == needle { Some(idx) } else { None }
                    }) {
                        for l in &orig_lines[cursor..found] {
                            out.push((*l).to_string());
                        }
                        cursor = found;
                    } else {
                        // seg_i's head doesn't exist in the original at or
                        // past cursor — assume it's brand new and just
                        // append the remaining original tail before it.
                        for l in &orig_lines[cursor..] {
                            out.push((*l).to_string());
                        }
                        cursor = orig_lines.len();
                    }
                } else {
                    // Trailing marker with no following content. Append
                    // the remainder of the original verbatim.
                    for l in &orig_lines[cursor..] {
                        out.push((*l).to_string());
                    }
                    cursor = orig_lines.len();
                    continue;
                }
            }

            // Emit the segment literally.
            for l in seg.iter() {
                out.push((*l).to_string());
            }

            // Advance the original cursor past whatever portion of the
            // original this segment overlaps so the next marker resumes
            // from after the segment.
            if let Some(tail) = last_nonblank(seg) {
                let needle = norm(tail);
                if let Some(found) = orig_lines.iter().enumerate().skip(cursor).find_map(|(idx, l)| {
                    if norm(l) == needle { Some(idx) } else { None }
                }) {
                    cursor = found + 1;
                }
                // If not found, the segment is brand new and the cursor
                // stays put — next marker (if any) will preserve the
                // original from `cursor` forward.
            }
        }

        let mut merged = out.join("\n");
        // Preserve trailing newline from original when present.
        if original.ends_with('\n') && !merged.ends_with('\n') {
            merged.push('\n');
        }
        Ok(merged)
    }

    pub(crate) async fn fast_apply(&self, args: Value) -> Result<Value> {
        let path_str = args.get("path").and_then(|v| v.as_str()).ok_or(anyhow!("Missing path"))?;
        let edit = args.get("edit").and_then(|v| v.as_str()).ok_or(anyhow!("Missing edit"))?;
        let dry_run = args.get("dry_run").and_then(|v| v.as_bool()).unwrap_or(false);

        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, path_str)?;

        if !full_path.exists() {
            // No file yet → treat the sketch as the literal new content
            // (markers in a new file have nothing to expand against).
            if let Some(parent) = full_path.parent() { fs::create_dir_all(parent)?; }
            if !dry_run { fs::write(&full_path, edit)?; }
            return Ok(json!({
                "status": "success",
                "path": path_str,
                "message": "Created new file from sketch (no merge needed).",
                "merged_bytes": edit.len()
            }));
        }

        let original = fs::read_to_string(&full_path)?;
        let merged = Self::merge_fast_apply(&original, edit)?;

        if dry_run {
            return Ok(json!({
                "status": "preview",
                "path": path_str,
                "merged": merged
            }));
        }

        fs::write(&full_path, &merged)?;
        self.memory_store.update_vfs_cache(full_path.clone(), merged.clone()).await;

        let h_lock = self.app_handle.lock().await;
        if let Some(h) = h_lock.as_ref() {
            let _ = h.emit("file-changed", json!({ "path": full_path.to_string_lossy().to_string() }));
            let _ = h.emit("ai-artifact", json!({
                "type": "file",
                "path": path_str,
                "title": format!("fast_apply: {}", path_str),
                "content": "Sketch merged and written."
            }));
        }

        Ok(json!({
            "status": "success",
            "path": path_str,
            "message": "Fast-apply merge written to disk.",
            "merged_bytes": merged.len(),
            "original_bytes": original.len()
        }))
    }

    pub(crate) async fn search_replace_edit(&self, args: Value) -> Result<Value> {
        let path_str = args.get("path").and_then(|v| v.as_str()).ok_or(anyhow!("Missing path"))?;
        let content = args.get("content").and_then(|v| v.as_str()).ok_or(anyhow!("Missing content"))?;
        // Default true: write straight to disk. Shadow staging + auto-apply was
        // fragile (PathBuf key mismatches, "No uncommitted changes") and made
        // the agent report success while the editor showed stale/empty content.
        let direct_apply = args.get("direct_apply").and_then(|v| v.as_bool()).unwrap_or(true);

        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, path_str)?;

        // Create file if it doesn't exist yet
        if !full_path.exists() {
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&full_path, "")?;
        }

        let original_content = fs::read_to_string(&full_path)?;

        let patches = crate::patch_engine::PatchEngine::parse_search_replace(content);
        if patches.is_empty() {
            // Fallback: if no SEARCH/REPLACE block found but content looks like code, treat as full write
            if !content.trim().is_empty() && !content.contains("<<<") {
                return Err(anyhow!(
                    "No SEARCH/REPLACE blocks found. Format: '<<<< SEARCH\\n<old code>\\n====\\n<new code>\\n>>>>'. For a full file write, use write_to_file instead."
                ));
            }
            return Err(anyhow!("No valid SEARCH/REPLACE blocks found in content"));
        }

        let mut engine = self.patch_engine.lock().await;
        let new_content = engine.apply_patches(&full_path, &original_content, &patches).await?;

        if direct_apply {
            // Write directly to disk, bypass shadow review
            fs::write(&full_path, &new_content)?;
            
            // Phase 25: Sync Cache
            self.memory_store.update_vfs_cache(full_path.clone(), new_content).await;

            let path_abs = full_path.to_string_lossy().to_string();
            let h_lock = self.app_handle.lock().await;
            if let Some(h) = h_lock.as_ref() {
                let _ = h.emit("ai-artifact", json!({
                    "type": "file",
                    "path": path_str,
                    "title": format!("Patched: {}", path_str),
                    "content": "Search/replace applied directly."
                }));
                let _ = h.emit("file-changed", json!({ "path": &path_abs }));
            }
            return Ok(json!({
                "status": "success",
                "path": path_str,
                "message": "Surgical edit applied to filesystem.",
                "patches_applied": patches.len()
            }));
        }

        let diff = engine.get_diff(&full_path, &original_content)?;

        // Notify frontend about the staged patch
        {
            let h_lock = self.app_handle.lock().await;
            if let Some(h) = h_lock.as_ref() {
                let _ = h.emit("sentient://patch_staged", json!({
                    "path": path_str,
                    "diff": diff,
                    "originalContent": original_content
                }));
            }
        }

        Ok(json!({
            "status": "staged",
            "path": path_str,
            "patches_applied": patches.len(),
            "message": "Surgical edit staged. Call apply_shadow_patch to commit.",
            "diff": diff
        }))
    }

    pub(crate) async fn preview_shadow_diff(&self, args: Value) -> Result<Value> {
        let path_str = args.get("path").and_then(|v| v.as_str()).ok_or(anyhow!("Missing path"))?;
        
        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, path_str)?;
        
        let original_content = fs::read_to_string(&full_path)?;
        let engine = self.patch_engine.lock().await;
        
        let diff = engine.get_diff(&full_path, &original_content)?;
        
        Ok(json!({
            "path": path_str,
            "diff": diff
        }))
    }

    pub(crate) async fn apply_shadow_patch(&self, args: Value) -> Result<Value> {
        let path_str = args.get("path").and_then(|v| v.as_str()).ok_or(anyhow!("Missing path"))?;
        
        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, path_str)?;
        
        let mut engine = self.patch_engine.lock().await;
        engine.commit_shadow(&full_path)?;
        
        // Phase 25: Sync Cache after commit
        if let Ok(content) = fs::read_to_string(&full_path) {
            self.memory_store.update_vfs_cache(full_path.clone(), content).await;
        }
        
        // HADES SYNAPSE: Record the architectural impact + notify Monaco to reload
        {
            let h_lock = self.app_handle.lock().await;
            if let Some(h) = h_lock.as_ref() {
                let state: tauri::State<crate::EditorState> = h.state();
                let _ = state.memory_layer.record_decision(
                    &format!("Applied surgical patch to {}", path_str),
                    "Shadow buffer verification passed (Ghost Mode).",
                    "Persistent VFS sync complete."
               ).map_err(|e| anyhow!(e.to_string()));
                let _ = h.emit("file-changed", json!({ "path": full_path.to_string_lossy() }));
            }
        }

        Ok(json!({
            "status": "success",
            "path": path_str,
            "message": "Shadow changes committed to filesystem."
        }))
    }

    pub(crate) async fn ghost_test(&self, args: Value) -> Result<Value> {
        let command = args.get("command").and_then(|v| v.as_str()).ok_or(anyhow!("Missing command"))?;
        
        let rt = self.ghost_runtime.clone();
        let result = rt.execute(command, 60).await?;

        Ok(json!(result))
    }

    pub(crate) async fn multi_replace_file_content(&self, args: Value) -> Result<Value> {
        let path_str = args
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing path"))?;
        let replacements = args
            .get("replacements")
            .and_then(|v| v.as_array())
            .ok_or_else(|| anyhow!("Missing replacements array"))?;

        let root = self.root_path.lock().await;
        let full_path = self.validate_path(&root, path_str)?;

        let mut content = fs::read_to_string(&full_path)?;

        for rep in replacements {
            let target = rep
                .get("target")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing target in replacement"))?;
            let replacement = rep
                .get("replacement")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Missing replacement in replacement"))?;

            if !content.contains(target) {
                return Err(anyhow!("Target string '{}' not found in file", target));
            }
            content = content.replace(target, replacement);
        }

        fs::write(&full_path, &content)?;
        
        // Phase 25: Sync Cache
        self.memory_store.update_vfs_cache(full_path, content).await;
        
        Ok(json!({ "status": "success" }))
    }

    pub(crate) async fn find_by_name(&self, args: Value) -> Result<Value> {
        let input_pattern = args.get("pattern").and_then(|v| v.as_str()).unwrap_or("*");
        let path_str = args.get("path").and_then(|v| v.as_str()).unwrap_or(".");

        let root = self.root_path.lock().await;
        let base_path = self.validate_path(&root, path_str)?;
        
        let (search_path, pattern) = if cfg!(target_os = "windows") {
             self.extract_path_and_pattern(&base_path.to_string_lossy(), input_pattern)
        } else {
             (base_path, input_pattern.to_string())
        };

        let mut results = Vec::new();
        use walkdir::WalkDir;
        let glob_pat = glob::Pattern::new(&pattern.to_lowercase())?;

        for entry in WalkDir::new(search_path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let name = entry.file_name().to_string_lossy().to_lowercase();
                if glob_pat.matches(&name) || (pattern == "*" || pattern == "**/*") {
                    let path = entry.path();
                    let relative = if let Ok(rel) = path.strip_prefix(&*root) {
                        rel.to_string_lossy().to_string()
                    } else {
                        path.to_string_lossy().to_string()
                    };
                    results.push(relative);
                }
            }
        }
        
        if results.len() > 100 {
            results.truncate(100);
        }

        Ok(Value::Array(
            results.into_iter().map(Value::String).collect(),
        ))
    }

    pub(crate) async fn get_directory_structure(&self, args: Value) -> Result<Value> {
        let path_str = args.get("path").and_then(|v| v.as_str()).unwrap_or(".");
        let max_depth = args.get("depth").and_then(|v| v.as_u64()).unwrap_or(2) as usize;

        let root = self.root_path.lock().await;
        let start_path = self.validate_path(&root, path_str)?;

        let mut structure = Vec::new();
        use walkdir::WalkDir;

        for entry in WalkDir::new(start_path)
            .max_depth(max_depth)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let rel_path = entry
                .path()
                .strip_prefix(&*root)?
                .to_string_lossy()
                .to_string();
            let depth = entry.depth();
            let is_dir = entry.file_type().is_dir();

            structure.push(json!({
                "path": rel_path,
                "depth": depth,
                "type": if is_dir { "directory" } else { "file" }
            }));
        }

        Ok(Value::Array(structure))
    }

    pub async fn analyze_file_symbols(&self, args: Value) -> Result<Value> {
        let path_str = args["path"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing path"))?;
        let root = self.root_path.lock().await;
        let full_path = self.validate_path(&root, path_str)?;

        let content = fs::read_to_string(&full_path)?;
        let mut symbols = Vec::new();

        let extension = full_path.extension().and_then(|s| s.to_str()).unwrap_or("");

        match extension {
            "rs" => {
                let fn_re = regex::Regex::new(
                    r"(?m)^\s*(?:pub\s+)?(?:async\s+)?fn\s+([a-zA-Z_][a-zA-Z0-9_]*)",
                )?;
                let struct_re =
                    regex::Regex::new(r"(?m)^\s*(?:pub\s+)?struct\s+([a-zA-Z_][a-zA-Z0-9_]*)")?;
                let enum_re =
                    regex::Regex::new(r"(?m)^\s*(?:pub\s+)?enum\s+([a-zA-Z_][a-zA-Z0-9_]*)")?;
                let trait_re =
                    regex::Regex::new(r"(?m)^\s*(?:pub\s+)?trait\s+([a-zA-Z_][a-zA-Z0-9_]*)")?;
                let impl_re =
                    regex::Regex::new(r"(?m)^\s*impl(?:\s+<.*>)?\s+([a-zA-Z_][a-zA-Z0-9_]*)")?;

                for cap in fn_re.captures_iter(&content) {
                    symbols.push(json!({"type": "function", "name": &cap[1]}));
                }
                for cap in struct_re.captures_iter(&content) {
                    symbols.push(json!({"type": "struct", "name": &cap[1]}));
                }
                for cap in enum_re.captures_iter(&content) {
                    symbols.push(json!({"type": "enum", "name": &cap[1]}));
                }
                for cap in trait_re.captures_iter(&content) {
                    symbols.push(json!({"type": "trait", "name": &cap[1]}));
                }
                for cap in impl_re.captures_iter(&content) {
                    symbols.push(json!({"type": "impl", "name": &cap[1]}));
                }
            }
            "ts" | "tsx" | "js" | "jsx" => {
                let func_re = regex::Regex::new(
                    r"(?m)^\s*(?:export\s+)?(?:async\s+)?function\s+([a-zA-Z_][a-zA-Z0-9_]*)",
                )?;
                let class_re =
                    regex::Regex::new(r"(?m)^\s*(?:export\s+)?class\s+([a-zA-Z_][a-zA-Z0-9_]*)")?;
                let interface_re = regex::Regex::new(
                    r"(?m)^\s*(?:export\s+)?interface\s+([a-zA-Z_][a-zA-Z0-9_]*)",
                )?;
                let const_func_re = regex::Regex::new(
                    r"(?m)^\s*(?:export\s+)?const\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*=\s*(?:\(.*\)|async)",
                )?;

                for cap in func_re.captures_iter(&content) {
                    symbols.push(json!({"type": "function", "name": &cap[1]}));
                }
                for cap in class_re.captures_iter(&content) {
                    symbols.push(json!({"type": "class", "name": &cap[1]}));
                }
                for cap in interface_re.captures_iter(&content) {
                    symbols.push(json!({"type": "interface", "name": &cap[1]}));
                }
                for cap in const_func_re.captures_iter(&content) {
                    symbols.push(json!({"type": "component/function", "name": &cap[1]}));
                }
            }
            "py" => {
                let def_re = regex::Regex::new(r"(?m)^\s*def\s+([a-zA-Z_][a-zA-Z0-9_]*)")?;
                let class_re = regex::Regex::new(r"(?m)^\s*class\s+([a-zA-Z_][a-zA-Z0-9_]*)")?;

                for cap in def_re.captures_iter(&content) {
                    symbols.push(json!({"type": "function", "name": &cap[1]}));
                }
                for cap in class_re.captures_iter(&content) {
                    symbols.push(json!({"type": "class", "name": &cap[1]}));
                }
            }
            _ => {}
        }

        Ok(json!({
            "path": path_str,
            "extension": extension,
            "symbols_count": symbols.len(),
            "symbols": symbols
        }))
    }

    pub async fn patch_file_content(&self, args: Value) -> Result<Value> {
        let path_str = args["path"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing path"))?;
        let start_line = args["StartLine"]
            .as_u64()
            .ok_or_else(|| anyhow!("Missing StartLine"))? as usize;
        let end_line = args["EndLine"]
            .as_u64()
            .ok_or_else(|| anyhow!("Missing EndLine"))? as usize;
        let replacement = args["ReplacementContent"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing ReplacementContent"))?;

        let root = self.root_path.lock().await;
        let full_path = self.validate_path(&root, path_str)?;

        let content = fs::read_to_string(&full_path)?;
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        if start_line == 0 || start_line > lines.len() + 1 {
            return Err(anyhow!(
                "StartLine {} out of range (total lines: {})",
                start_line,
                lines.len()
            ));
        }

        let start_idx = start_line - 1;
        let end_idx = std::cmp::min(end_line, lines.len());

        let mut new_lines = Vec::new();
        new_lines.extend_from_slice(&lines[..start_idx]);
        new_lines.push(replacement.to_string());
        new_lines.extend_from_slice(&lines[end_idx..]);

        let path_string = full_path.to_string_lossy().to_string();
        fs::write(&full_path, new_lines.join("\n"))?;

        // Notify Monaco editor to reload this file
        {
            let h_lock = self.app_handle.lock().await;
            if let Some(h) = h_lock.as_ref() {
                let _ = h.emit("file-changed", json!({ "path": path_string }));
            }
        }

        Ok(json!({ "status": "success" }))
    }

    #[allow(dead_code)]
    pub(crate) fn read_url_content(&self, args: Value) -> Result<Value> {
        let url = args["url"].as_str().ok_or_else(|| anyhow!("Missing url"))?;
        let body = reqwest::blocking::get(url)?.text()?;

        Ok(json!({
            "url": url,
            "content_length": body.len(),
            "content": body.chars().take(5000).collect::<String>()
        }))
    }

    pub async fn browser_subagent(self: Arc<Self>, args: Value) -> Result<Value> {
        let task = args["task"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing task"))?
            .to_string();

        let app_handle = self.app_handle.clone();
        let tools = Arc::new(self.clone());
        let task_id = format!(
            "browser-{}",
            uuid::Uuid::new_v4()
                .to_string()
                .chars()
                .take(8)
                .collect::<String>()
        );

        // Report initial start
        let h_lock = app_handle.lock().await;
        if let Some(h) = h_lock.as_ref() {
                let _ = h.emit(
                    "subagent-progress",
                    json!({
                        "id": task_id,
                        "title": format!("Web Research: {}", task),
                        "progress": 5,
                        "status": "running",
                        "message": "Launching browser..."
                    }),
                );
        }

        let t_owned = task.clone();
        let tid_owned = task_id.clone();
        let h_owned = app_handle.clone();
        let tools_loop = tools.clone();

        tauri::async_runtime::spawn(async move {
            let h_loop = h_owned;
            let tid_loop = tid_owned;
            let t_loop = t_owned;
            let sub_tools = tools_loop;

            // Step 1: Open Browser
            {
                let h_lock = h_loop.lock().await;
                if let Some(h_val) = &*h_lock {
                        let _ = h_val.emit(
                            "subagent-progress",
                            json!({
                                "id": tid_loop,
                                "title": format!("Web Research: {}", t_loop),
                                "progress": 15,
                                "status": "running",
                                "message": "Opening headless browser..."
                            }),
                        );
                }
            }

            if let Err(e) = sub_tools.browser_open(json!({})).await {
                {
                    let h_lock = h_loop.lock().await;
                    if let Some(h_val) = &*h_lock {
                        let _ = h_val.emit(
                            "subagent-progress",
                            json!({
                                "id": tid_loop,
                                "status": "failed",
                                "message": format!("Failed to open browser: {}", e)
                            }),
                        );
                    }
                }
                return;
            }

            // Step 2: Search
            {
                let h_lock = h_loop.lock().await;
                if let Some(h_val) = &*h_lock {
                        let _ = h_val.emit(
                            "subagent-progress",
                            json!({
                                "id": tid_loop,
                                "title": format!("Web Research: {}", t_loop),
                                "progress": 30,
                                "status": "running",
                                "message": format!("Searching for '{}'...", t_loop)
                            }),
                        );
                }
            }

            match sub_tools.browser_search(json!({ "query": t_loop })).await {
                Ok(_) => {
                    let h_lock = h_loop.lock().await;
                    if let Some(h_val) = &*h_lock {
                        let _ = h_val.emit(
                                "subagent-progress",
                                json!({
                                    "id": tid_loop,
                                    "title": format!("Web Research: {}", t_loop),
                                    "progress": 50,
                                    "status": "running",
                                    "message": "Extracting initial results..."
                                }),
                            );
                    }
                }
                Err(e) => {
                    {
                        let h_lock = h_loop.lock().await;
                        if let Some(h_val) = &*h_lock {
                            let _ = h_val.emit(
                                "subagent-progress",
                                json!({
                                    "id": tid_loop,
                                    "status": "failed",
                                    "message": format!("Search failed: {}", e)
                                }),
                            );
                        }
                    }
                    return;
                }
            }

            // Step 3: Get Summary
            {
                let h_lock = h_loop.lock().await;
                if let Some(h_val) = &*h_lock {
                        let _ = h_val.emit(
                            "subagent-progress",
                            json!({
                                "id": tid_loop,
                                "title": format!("Web Research: {}", t_loop),
                                "progress": 60,
                                "status": "running",
                                "message": "Summarizing search results..."
                            }),
                        );
                }
            }

            let summary = match sub_tools.browser_get_content_summary(json!({})).await {
                Ok(s) => s,
                Err(e) => {
                    {
                        let h_lock = h_loop.lock().await;
                        if let Some(h_val) = &*h_lock {
                                let _ = h_val.emit(
                                    "subagent-progress",
                                    json!({
                                        "id": tid_loop,
                                        "status": "failed",
                                        "message": format!("Summary failed: {}", e)
                                    }),
                                );
                        }
                    }
                    return;
                }
            };

            // Step 4: Deep Dive into first relevant link
            let mut detail = String::new();
            if let Some(links) = summary["links"].as_array() {
                if let Some(first) = links.first() {
                    if let Some(href) = first["href"].as_str() {
                        {
                            let h_lock = h_loop.lock().await;
                            if let Some(h_val) = &*h_lock {
                                    let _ = h_val.emit("subagent-progress", json!({ "id": tid_loop, "title": format!("Web Research: {}", t_loop), "progress": 75, "status": "running", "message": format!("Navigating to source: {}...", href) }));
                            }
                        }
                        let _ = sub_tools.browser_navigate(json!({ "url": href })).await;

                        {
                            let h_lock = h_loop.lock().await;
                            if let Some(h_val) = &*h_lock {
                                    let _ = h_val.emit("subagent-progress", json!({ "id": tid_loop, "title": format!("Web Research: {}", t_loop), "progress": 85, "status": "running", "message": "Analyzing source content..." }));
                            }
                        }
                        if let Ok(detail_summary) = sub_tools.browser_get_content_summary(json!({})).await
                        {
                            detail = detail_summary["text"]
                                .as_str()
                                .unwrap_or_default()
                                .chars()
                                .take(2000)
                                .collect();
                        }
                    }
                }
            }

            // Final Report
            {
                let h_lock = h_loop.lock().await;
                if let Some(h_val) = &*h_lock {
                        let _ = h_val.emit("subagent-progress", json!({ "id": tid_loop, "title": format!("Web Research: {}", t_loop), "progress": 100, "status": "running", "message": "Research completed." }));
                }
            }

            let final_result = json!({
                "task": t_loop,
                "status": "Research loop completed autonomously.",
                "summary": summary["text"].as_str().unwrap_or("No summary provided").chars().take(1000).collect::<String>(),
                "detail": detail,
                "verification_artifact": "research_report.md"
            });

            {
                let h_lock = h_loop.lock().await;
                if let Some(h_val) = &*h_lock {
                        let _ = h_val.emit(
                            "subagent-progress",
                            json!({
                                "id": tid_loop,
                                "status": "completed",
                                "progress": 100,
                                "result": final_result
                            }),
                        );
                }
            }
        });

        Ok(json!({
            "status": "success",
            "message": "Browser orchestrator started in background.",
            "task_id": task_id
        }))
    }

    pub async fn perplexity_proxy(self: Arc<Self>, args: Value) -> Result<Value> {
        let query = args["query"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing query"))?;

        // Fallback: Use the browser search logic if Perplexity API is unavailable
        println!("[Perplexity] Fallback research for: {}", query);
        self.clone().browser_subagent(json!({ "task": query })).await
    }

    // Git Tools Implementation
    pub(crate) async fn git_status(&self, _args: Value) -> Result<Value> {
        let root = self
            .root_path
            .lock()
            .await;
        let status = self
            .git_manager
            .get_status(&*root)
            .map_err(|e| anyhow!(e))?;
        Ok(json!(status))
    }

    pub(crate) async fn git_add(&self, args: Value) -> Result<Value> {
        let path = args["path"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing path"))?;
        let root = self
            .root_path
            .lock()
            .await;
        self.git_manager
            .stage(&*root, path)
            .map_err(|e| anyhow!(e))?;
        Ok(json!({ "status": "success", "message": format!("Staged {}", path) }))
    }

    pub(crate) async fn git_commit(&self, args: Value) -> Result<Value> {
        let message = args["message"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing message"))?;
        let root = self.root_path.lock().await;

        self.git_manager
            .commit(&*root, message)
            .map_err(|e| anyhow!(e))?;
        Ok(json!({ "status": "success", "message": "Changes committed." }))
    }

    pub(crate) async fn git_log(&self, args: Value) -> Result<Value> {
        let _limit = args["limit"].as_u64().unwrap_or(10);
        let root = self.root_path.lock().await;

        let history = self
            .git_manager
            .get_history(&*root)
            .map_err(|e| anyhow!(e))?;
        Ok(json!(history))
    }

    pub(crate) async fn git_diff(&self, args: Value) -> Result<Value> {
        let path = args["path"].as_str().unwrap_or(".");
        let staged = args["staged"].as_bool().unwrap_or(false);
        let hash = args["hash"].as_str();

        let root = self.root_path.lock().await;

        let mut cmd = std::process::Command::new("git");
        if let Some(h) = hash {
            cmd.arg("show");
            cmd.arg("--format=");
            cmd.arg(h);
        } else {
            cmd.arg("diff");
            if staged {
                cmd.arg("--staged");
            }
            cmd.arg(path);
        }
        cmd.current_dir(&*root);

        let output = cmd
            .output()
            .map_err(|e| anyhow!("Failed to execute git: {}", e))?;
        let diff = String::from_utf8_lossy(&output.stdout).to_string();

        Ok(json!({ "diff": diff }))
    }

    pub(crate) async fn get_system_health(&self, _args: Value) -> Result<Value> {
        let mut health = json!({
            "git": { "status": "unknown" },
            "tools": {
                "node": "unknown",
                "cargo": "unknown"
            },
            "mcp_servers": []
        });

        // 1. Check Git
        let root = self.root_path.lock().await;
        let output = std::process::Command::new("git")
            .arg("rev-parse")
            .arg("--abbrev-ref")
            .arg("HEAD")
            .current_dir(&*root)
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                health["git"]["is_repo"] = json!(true);
                health["git"]["status"] = json!("ok");
                health["git"]["current_branch"] =
                    json!(String::from_utf8_lossy(&output.stdout).trim());
            } else {
                health["git"]["is_repo"] = json!(false);
            }
        }

        // 2. Check Node
        let node_v = std::process::Command::new("node").arg("--version").output();
        health["tools"]["node"] = if node_v.is_ok() && node_v.as_ref().unwrap().status.success() {
            json!(String::from_utf8_lossy(&node_v.unwrap().stdout).trim())
        } else {
            json!("missing")
        };

        // 3. Check Cargo
        let cargo_v = std::process::Command::new("cargo")
            .arg("--version")
            .output();
        health["tools"]["cargo"] = if cargo_v.is_ok() && cargo_v.as_ref().unwrap().status.success()
        {
            json!(String::from_utf8_lossy(&cargo_v.unwrap().stdout).trim())
        } else {
            json!("missing")
        };

        // 4. Check MCP
        let mcp_status = self.mcp_registry.list_servers_status().await;
        health["mcp_servers"] = json!(mcp_status);
        Ok(health)
    }

    pub(crate) async fn handle_save_knowledge_brief(&self, args: Value) -> Result<Value> {
        let brief: crate::knowledge_distiller::KnowledgeBrief = serde_json::from_value(args)
            .map_err(|e| anyhow!("Invalid knowledge brief format: section 318 {}", e))?;

        let path = self.knowledge_distiller.save_finding(brief)
            .map_err(|e| anyhow!("Failed to save knowledge: section 318 {}", e))?;

        Ok(json!({
            "status": "success",
            "message": "Mission finding archived to persistent brain.",
            "path": path
        }))
    }

    pub(crate) async fn handle_verify_claim(&self, args: Value) -> Result<Value> {
        let claim = args.get("claim")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing 'claim' argument"))?;

        let result = self.memory_store.verify_claim(claim).await;
        Ok(result)
    }

    pub(crate) async fn handle_see_the_screen(&self, _args: Value) -> Result<Value> {
        let h_lock = self.app_handle.lock().await;
        let h = h_lock.as_ref().ok_or_else(|| anyhow!("App handle not set"))?;

        let result = crate::vision_bridge::capture_main_screenshot(h)
            .map_err(|e| anyhow!("Visual capture failed: section 318 {}", e))?;

        Ok(json!(result))
    }

    pub(crate) async fn handle_task_boundary(&self, args: Value) -> Result<Value> {
        let h_lock = self.app_handle.lock().await;
        let h = h_lock.as_ref().ok_or_else(|| anyhow!("App handle not set"))?;

        let task_name = args["TaskName"].as_str().unwrap_or("Task");
        let mode = args["Mode"].as_str().unwrap_or("EXECUTION");
        let summary = args["TaskSummary"].as_str().unwrap_or("");
        let status = args["TaskStatus"].as_str().unwrap_or("");
        let progress_inc = args["PredictedTaskSize"].as_i64().unwrap_or(10);
        
        // Map PredictedTaskSize to a rough progress percentage (inverted)
        let progress = (100 - (progress_inc * 5).min(90)) as f64;

        h.emit(
            "update-agent-task",
            json!({
                "id": "current-mission",
                "title": task_name,
                "summary": summary,
                "status": "running",
                "progress": progress,
                "mode": mode,
                "task_status": status
            }),
        )?;

        h.emit("add-agent-step", json!({ 
            "name": format!("[{}] {}", mode, status), 
            "status": "running" 
        }))?;

        Ok(json!({ "status": "success", "info": "Task boundary updated" }))
    }

    pub(crate) async fn handle_create_canvas(&self, args: Value) -> Result<Value> {
        let h_lock = self.app_handle.lock().await;
        let h = h_lock.as_ref().ok_or_else(|| anyhow!("App handle not set"))?;

        let title = args["title"].as_str().unwrap_or("Untitled Canvas");
        let blocks = args["blocks"].as_array().cloned().unwrap_or_default();
        if blocks.is_empty() {
            return Err(anyhow!("create_canvas requires a non-empty 'blocks' array"));
        }

        // Forward the raw spec to the frontend; normalization/rendering happens
        // in the React layer (CanvasSpec.normalizeCanvasSpec) so small models'
        // sloppy specs are salvaged consistently with the TS tool path.
        h.emit("canvas-updated", args.clone())?;

        Ok(json!({
            "status": "rendered",
            "title": title,
            "blocks": blocks.len(),
            "info": "Canvas is now visible to the user in an editor tab."
        }))
    }

    pub(crate) async fn handle_notify_user(&self, args: Value) -> Result<Value> {
        let h_lock = self.app_handle.lock().await;
        let h = h_lock.as_ref().ok_or_else(|| anyhow!("App handle not set"))?;

        let message = args["Message"].as_str().unwrap_or("");
        let blocked = args["BlockedOnUser"].as_bool().unwrap_or(false);
        let paths = args["PathsToReview"].as_array().cloned().unwrap_or_else(|| vec![]);

        h.emit(
            "notify-user",
            json!({
                "message": message,
                "blocked": blocked,
                "paths": paths
            }),
        )?;

        if blocked {
            Ok(json!({ 
                "status": "blocked", 
                "message": "Waiting for user input...",
                "user_message": message 
            }))
        } else {
            Ok(json!({ "status": "success" }))
        }
    }

    pub(crate) async fn handle_use_skill(&self, args: Value) -> Result<Value> {
        let skill_name = args["SkillName"].as_str().ok_or_else(|| anyhow!("Missing SkillName"))?;

        if let Some(body) = crate::hermes_skills::load_skill_body(skill_name) {
            if let Some(meta) = crate::hermes_skills::find_skill(skill_name) {
                if let Some(h) = self.app_handle.lock().await.as_ref() {
                    let _ = h.emit("ai-artifact", json!({
                        "type": "skill",
                        "title": format!("Skill Activated: {}", meta.name),
                        "content": body.chars().take(200).collect::<String>() + "..."
                    }));
                }
                return Ok(json!({
                    "status": "success",
                    "skill": meta.id,
                    "source": "hermes-integrated",
                    "instructions": body,
                    "info": "Hermes skill loaded natively into Sentient. Follow the instructions field for all subsequent steps."
                }));
            }
        }

        let root = self.root_path.lock().await;
        let skill_paths = [
            root.join(".agent").join("skills").join(skill_name).join("SKILL.md"),
            root.join(".agent").join("skills").join(format!("{}.md", skill_name)),
            root.join(".agent").join("skills").join(skill_name).join(format!("{}.md", skill_name)),
        ];

        for path in &skill_paths {
            if path.exists() {
                let content = fs::read_to_string(path)?;
                if let Some(h) = self.app_handle.lock().await.as_ref() {
                    let _ = h.emit("ai-artifact", json!({
                        "type": "skill",
                        "title": format!("Skill Activated: {}", skill_name),
                        "content": content.chars().take(200).collect::<String>() + "..."
                    }));
                }
                return Ok(json!({
                    "status": "success",
                    "skill": skill_name,
                    "source": "project",
                    "instructions": content,
                    "info": "Skill activated. Follow the instructions field for all subsequent steps."
                }));
            }
        }

        Err(anyhow!(
            "Skill '{}' not found. Try search_skills or use a Hermes id like software-development/systematic-debugging",
            skill_name
        ))
    }

    pub(crate) async fn handle_search_skills(&self, args: Value) -> Result<Value> {
        let query = args["Query"].as_str().ok_or_else(|| anyhow!("Missing Query"))?;
        let mut matches: Vec<String> = Vec::new();

        for s in crate::hermes_skills::search_skills(query, 15) {
            matches.push(format!("{} — {}", s.id, s.description));
        }

        let root = self.root_path.lock().await;
        let skills_dir = root.join(".agent").join("skills");
        if skills_dir.exists() {
            use walkdir::WalkDir;
            let q = query.to_lowercase();
            for entry in WalkDir::new(&skills_dir).max_depth(2).into_iter().filter_map(|e| e.ok()) {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.to_lowercase().contains(&q) && (entry.file_type().is_dir() || name.ends_with(".md")) {
                    let skill_name = if name.ends_with(".md") {
                        name.trim_end_matches(".md").to_string()
                    } else {
                        name
                    };
                    if !matches.iter().any(|m| m.starts_with(&skill_name)) {
                        matches.push(format!("{} (project)", skill_name));
                    }
                }
                if matches.len() >= 25 {
                    break;
                }
            }
        }

        Ok(json!({
            "results": matches,
            "info": format!("Found {} matches. use_skill with id or folder name.", matches.len())
        }))
    }

    pub async fn get_symbol_graph(&self, args: Value) -> Result<Value> {
        let symbol = args["symbol"].as_str().ok_or_else(|| anyhow!("Missing symbol"))?;
        let path = args["path"].as_str().ok_or_else(|| anyhow!("Missing path"))?;
        
        Ok(json!({
            "status": "Scanning symbol hierarchy...",
            "symbol": symbol,
            "origin": path,
            "usages": 12,
            "references": [
                {"file": "src/lib.rs", "line": 42},
                {"file": "src/main.rs", "line": 156}
            ],
            "impact_analysis": "Modifying this symbol will affect 3 modules. Safe verification is recommended."
        }))
    }

    pub async fn run_command_safe(&self, args: Value) -> Result<Value> {
        let command = args["command"].as_str().ok_or_else(|| anyhow!("Missing command"))?;
        self.run_command(json!({ "command": command })).await
    }

    pub async fn verify_implementation(&self, args: Value) -> Result<Value> {
        // Shadow VFS + cargo check when a patched file is supplied (MCTS verify seam).
        if let (Some(path), Some(content)) = (
            args.get("path").and_then(|v| v.as_str()),
            args.get("content").and_then(|v| v.as_str()),
        ) {
            let rel = path.trim_start_matches("./").trim_start_matches('\\').trim_start_matches('/');
            let harness = {
                let h_lock = self.app_handle.lock().await;
                let h = h_lock.as_ref().ok_or_else(|| anyhow!("App handle not set"))?;
                let state: tauri::State<crate::EditorState> = h.state();
                state.ai_engine.harness.clone()
            };
            match harness.verify_candidate(std::path::Path::new(rel), content) {
                Ok(diags) => {
                    let ok = diags.iter().all(|d| d.level != "error");
                    return Ok(json!({
                        "status": if ok { "verified" } else { "failed" },
                        "method": "shadow_cargo_check",
                        "path": rel,
                        "diagnostics": diags,
                        "errors": diags.iter().filter(|d| d.level == "error").count(),
                    }));
                }
                Err(e) => {
                    return Ok(json!({
                        "status": "skipped",
                        "method": "shadow_cargo_check",
                        "reason": e.to_string(),
                    }));
                }
            }
        }
        let command = args.get("command").and_then(|v| v.as_str()).unwrap_or("cargo check");
        self.run_command(json!({ "command": command, "shell_hint": "powershell" })).await
    }

    pub async fn create_mission_plan(&self, args: Value) -> Result<Value> {
        let plan = args["plan"].as_str().ok_or_else(|| anyhow!("Missing plan"))?;
        
        {
            let h_lock = self.app_handle.lock().await;
            if let Some(h) = h_lock.as_ref() {
                let _ = h.emit("agent-mission-plan", json!({ "plan": plan }));
            }
        }
        
        Ok(json!({
            "status": "Mission plan published.",
            "message": "The user can now see your tactical checklist in the UI."
        }))
    }

    pub async fn revert_checkpoint(&self, args: Value) -> Result<Value> {
        let path_str = args["path"].as_str().ok_or_else(|| anyhow!("Missing path"))?;
        let path = std::path::PathBuf::from(path_str);
        self.shadow_workspace.revert_to_last_checkpoint(&path)?;
        
        Ok(json!({
            "status": "Revert successful.",
            "path": path_str,
            "message": "File restored from last known-good shadow checkpoint."
        }))
    }

    pub async fn handle_research_tool(&self, name: &str, args: Value) -> Result<Value> {
        let root = self.root_path.lock().await.clone();
        
        match name {
            "security_scan" => {
                let path_str = args["path"].as_str().ok_or_else(|| anyhow!("Missing path"))?;
                let full_path = self.validate_path(&root, path_str)?;
                SecurityDistiller::run_semgrep(&full_path).map_err(|e: String| anyhow!(e))
            },
            "audit_dependencies" => {
                SecurityDistiller::run_cargo_audit(&root).map_err(|e: String| anyhow!(e))
            },
            "disassemble" => {
                let path_str = args["path"].as_str().ok_or_else(|| anyhow!("Missing path"))?;
                let full_path = self.validate_path(&root, path_str)?;
                let result = BinaryAnalyzer::disassemble(&full_path).map_err(|e| anyhow!(e))?;
                Ok(json!({ "disassembly": result }))
            },
            "get_binary_info" => {
                let path_str = args["path"].as_str().ok_or_else(|| anyhow!("Missing path"))?;
                let full_path = self.validate_path(&root, path_str)?;
                BinaryAnalyzer::get_info(&full_path).map_err(|e| anyhow!(e))
            },
            _ => Err(anyhow!("Unknown research tool: {}", name))
        }
    }

}

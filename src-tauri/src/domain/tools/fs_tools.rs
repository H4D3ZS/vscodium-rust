//! Filesystem + process tools: path validation, read/write/replace, run_command,
//! terminals, search, indexing, symbols, patching.
use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use super::registry::AiTools;
use super::registry::push_activity;
use super::shell::ShellTranslator;
use crate::domain::safe_io;
use crate::process_ext::CommandExtHidden;

/// Parse FastContext's trained text tool-call format — `READ(path)`, `GLOB(pat)`,
/// `GREP(term)` (case-insensitive) — into `(internal_name, arg)` pairs. Used when
/// the GGUF template doesn't emit native Ollama tool_calls.
fn parse_text_tool_calls(text: &str) -> Vec<(String, String)> {
    let re = regex::Regex::new(r#"(?i)\b(READ|GLOB|GREP|CODE_SEARCH|SEARCH)\s*\(\s*["']?([^"')]+?)["']?\s*\)"#).unwrap();
    let mut out = Vec::new();
    for cap in re.captures_iter(text) {
        let name = match cap[1].to_lowercase().as_str() {
            "read" => "read_file",
            "glob" => "glob",
            "code_search" => "code_search",
            _ => "grep", // GREP / SEARCH
        };
        let arg = cap[2].trim().to_string();
        if !arg.is_empty() { out.push((name.to_string(), arg)); }
        if out.len() >= 8 { break; }
    }
    out
}

/// Known long-running CLI tools — mostly recon/scanning binaries used in
/// bug-bounty / pentest PoC work — that should default to a background terminal
/// so they don't block the agent turn. Matched against any whitespace- or
/// pipe-separated token in the command (so `sudo nmap ...` and `foo | ffuf ...`
/// both hit). Curated deliberately: only tools that routinely run for minutes.
const LONG_RUNNING_BINARIES: &[&str] = &[
    "nmap", "masscan", "rustscan", "naabu", "zmap",
    "ffuf", "gobuster", "feroxbuster", "dirb", "dirbuster", "wfuzz", "dirsearch",
    "nikto", "sqlmap", "wpscan", "joomscan", "nuclei", "wafw00f",
    "hydra", "medusa", "patator", "hashcat", "john",
    "amass", "subfinder", "assetfinder", "gau", "katana", "httpx",
    "gospider", "hakrawler", "arjun", "dalfox", "testssl", "sslscan",
];

/// True when the command's first meaningful token (skipping sudo/env prefixes)
/// or any pipeline stage invokes a known long-running scanner.
pub(crate) fn is_long_running_command(command: &str) -> bool {
    for stage in command.split(['|', ';', '&']) {
        let mut tokens = stage.split_whitespace().peekable();
        // Skip common prefixes that precede the real binary.
        while let Some(&tok) = tokens.peek() {
            let base = tok.rsplit(['/', '\\']).next().unwrap_or(tok).to_lowercase();
            if matches!(base.as_str(), "sudo" | "doas" | "env" | "time" | "nice" | "stdbuf" | "proxychains" | "proxychains4") {
                tokens.next();
                continue;
            }
            // Skip `VAR=value` env assignments.
            if tok.contains('=') && !tok.starts_with('-') {
                tokens.next();
                continue;
            }
            break;
        }
        if let Some(tok) = tokens.next() {
            let base = tok.rsplit(['/', '\\']).next().unwrap_or(tok).to_lowercase();
            let base = base.trim_end_matches(".exe");
            if LONG_RUNNING_BINARIES.contains(&base) {
                return true;
            }
        }
    }
    false
}

impl AiTools {
    pub(crate) fn validate_path(&self, root: &std::path::Path, path_str: &str) -> Result<PathBuf> {
        let path = PathBuf::from(path_str);
        let full_path = if path.is_absolute() {
            path
        } else {
            root.join(path)
        };

        // Prevent path traversal — the resolved path must stay within workspace root.
        let canon_root = std::fs::canonicalize(root)
            .unwrap_or_else(|_| root.to_path_buf());
        if full_path.exists() {
            if let Ok(canon) = std::fs::canonicalize(&full_path) {
                if !canon.starts_with(&canon_root) {
                    return Err(anyhow::anyhow!(
                        "Path escapes workspace root: {}",
                        full_path.display()
                    ));
                }
                let ignore = crate::cursor_compat::CursorIgnoreSet::load(
                    root,
                    crate::cursor_compat::IgnoreScope::AiAccess,
                );
                if ignore.is_ignored(&canon) {
                    return Err(anyhow::anyhow!(
                        "Path blocked by .cursorignore: {}",
                        canon.display()
                    ));
                }
                return Ok(canon);
            }
        }
        // For non-existent paths, check the parent directory chain stays within root.
        let mut parent = full_path.parent();
        while let Some(p) = parent {
            if p == canon_root { break; }
            if p.starts_with(&canon_root) {
                parent = p.parent();
                continue;
            }
            return Err(anyhow::anyhow!(
                "Path escapes workspace root: {}",
                full_path.display()
            ));
        }
        let ignore = crate::cursor_compat::CursorIgnoreSet::load(
            root,
            crate::cursor_compat::IgnoreScope::AiAccess,
        );
        if ignore.is_ignored(&full_path) {
            return Err(anyhow::anyhow!(
                "Path blocked by .cursorignore: {}",
                full_path.display()
            ));
        }
        Ok(full_path)
    }

    /// Splits a path that might contain wildcards into a base directory and a pattern.
    /// Example: "C:\src\*.cpp" -> ("C:\src", "*.cpp")
    pub(crate) fn extract_path_and_pattern(&self, path_str: &str, default_pattern: &str) -> (PathBuf, String) {
        let path = PathBuf::from(path_str);
        
        // If it contains wildcards, we need to find the "base" directory
        if path_str.contains('*') || path_str.contains('?') || path_str.contains('[') {
             let mut current = path.clone();
             let mut pattern_parts: Vec<String> = Vec::new();
             
             while let Some(parent) = current.parent().map(|p| p.to_path_buf()) {
                 let component = current.file_name().and_then(|n| n.to_str()).unwrap_or("");
                 if component.contains('*') || component.contains('?') || component.contains('[') {
                     pattern_parts.push(component.to_string());
                     current = parent;
                 } else {
                     break;
                 }
             }
             
             if !pattern_parts.is_empty() {
                 pattern_parts.reverse();
                 return (current, pattern_parts.join("/"));
             }
        }
        
        if path.is_dir() {
            (path, default_pattern.to_string())
        } else if let Some(parent) = path.parent() {
             if let Some(file_name) = path.file_name() {
                 (parent.to_path_buf(), file_name.to_string_lossy().to_string())
             } else {
                 (path, default_pattern.to_string())
             }
        } else {
            (path, default_pattern.to_string())
        }
    }

    pub(crate) async fn read_file(&self, args: Value) -> Result<Value> {
        let path_str = args
            .get("TargetFile")
            .or_else(|| args.get("path"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing TargetFile"))?;

        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, path_str)?;

        if !full_path.exists() {
            let suggestions = self
                .memory_store
                .suggest_similar_paths(path_str, 8)
                .await;
            let tree_sample = self.memory_store.get_project_tree_summary().await;
            let mut msg = format!(
                "File not found: '{path_str}' (resolved: {}). \
                 This path is NOT in the workspace — do NOT retry the same path.",
                full_path.display()
            );
            if !suggestions.is_empty() {
                msg.push_str("\n\nSimilar paths in AIM index:\n");
                for s in &suggestions {
                    msg.push_str(&format!("  - {s}\n"));
                }
            } else {
                msg.push_str("\n\nNo similar path in AIM. Use ### BRAIN tree — not ARCHITECTURE.md/CLAUDE.md unless listed.\n");
                msg.push_str(&format!("Indexed tree sample: {tree_sample}"));
            }
            return Err(anyhow!(msg));
        }

        // Always read from disk. Serving `get_vfs_cache` first caused the agent
        // to "comprehend" stale buffers (e.g. empty or pre-edit snapshots) while
        // the editor showed different on-disk truth — breaking writes and reviews.
        let metadata = fs::metadata(&full_path)?;
        let file_size = metadata.len();

        if file_size > 2 * 1024 * 1024 {
            use std::io::{BufRead, BufReader};
            let file = fs::File::open(&full_path)?;
            let mut reader = BufReader::new(file);
            let mut head_lines = Vec::new();
            let mut tail_lines = Vec::new();
            let mut line_num = 0usize;
            let mut buf = String::new();
            let head_limit = 200;
            let tail_limit = 50;
            loop {
                buf.clear();
                let bytes_read = reader.read_line(&mut buf)?;
                if bytes_read == 0 { break; }
                line_num += 1;
                if line_num <= head_limit {
                    head_lines.push(buf.trim_end_matches('\n').trim_end_matches('\r').to_string());
                } else {
                    tail_lines.push(buf.trim_end_matches('\n').trim_end_matches('\r').to_string());
                    if tail_lines.len() > tail_limit {
                        tail_lines.remove(0);
                    }
                }
            }
            let preview = if tail_lines.is_empty() {
                head_lines.join("\n")
            } else {
                let mut all = head_lines;
                all.push(format!("\n... ({line_num} lines total, showing first 200 + last 50) ...\n"));
                all.extend(tail_lines);
                all.join("\n")
            };
            self.memory_store.update_vfs_cache(full_path.clone(), preview.clone()).await;
            Ok(json!({
                "large": true,
                "size": file_size,
                "total_lines": line_num,
                "preview": preview
            }))
        } else {
            let content = safe_io::safe_read(&full_path)?;
            self.memory_store.update_vfs_cache(full_path, content.clone()).await;
            Ok(Value::String(content))
        }
    }

    pub(crate) async fn write_file(&self, args: Value) -> Result<Value> {
        let mut path_str = args
            .get("path")
            .or_else(|| args.get("file_path"))
            .or_else(|| args.get("target_file"))
            .or_else(|| args.get("filename"))
            .or_else(|| args.get("filepath"))
            .or_else(|| args.get("file"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        // Models often hallucinate alternate parameter names — accept common aliases.
        let content = args
            .get("content")
            .or_else(|| args.get("contents"))
            .or_else(|| args.get("body"))
            .or_else(|| args.get("text"))
            .or_else(|| args.get("data"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing content (expected content, contents, body, text, or data)"))?;

        if path_str.as_deref().unwrap_or("").trim().is_empty() {
            if let Some(inferred) = Self::infer_write_path_from_content(content) {
                path_str = Some(inferred);
            }
        }

        let path_str = path_str.ok_or_else(|| anyhow!("Missing path"))?;

        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, &path_str)?;

        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&full_path, content)?;

        // Phase 25: Sync Cache
        self.memory_store.update_vfs_cache(full_path.clone(), content.to_string()).await;

        // Emit artifact for UI card + file-changed so Monaco tabs reload + open in editor
        {
            let path_abs = full_path.to_string_lossy().to_string();
            self.emit_tool_event(
                "ai-artifact",
                json!({
                    "type": "file",
                    "path": path_str,
                    "title": format!("Written: {}", path_str),
                    "content": "File saved successfully"
                }),
            );
            // Reload if already open in editor, or open fresh
            self.emit_tool_event("file-changed", json!({ "path": &path_abs }));
            self.emit_tool_event("editor_open_file", json!({ "path": &path_abs }));
        }

        let bytes = content.len();
        let preview: String = content.chars().take(400).collect();
        Ok(serde_json::json!({
            "status": "success",
            "file": path_str,
            "bytes_written": bytes,
            "preview_start": preview
        }))
    }

    /// Simple str_replace: finds old_str in file, replaces with new_str, writes back.
    /// Much simpler than search_replace_edit — no block format needed.
    pub(crate) async fn str_replace_file(&self, args: Value) -> Result<Value> {
        let path_str = args.get("path").and_then(|v| v.as_str()).ok_or_else(|| anyhow!("Missing path"))?;
        let old_str = args.get("old_str").and_then(|v| v.as_str()).ok_or_else(|| anyhow!("Missing old_str"))?;
        let new_str = args.get("new_str").and_then(|v| v.as_str()).ok_or_else(|| anyhow!("Missing new_str"))?;

        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, path_str)?;

        let content = safe_io::safe_read(&full_path)
            .map_err(|e| anyhow!("Cannot read {}: {}", path_str, e))?;

        if !content.contains(old_str) {
            return Err(anyhow!(
                "str_replace failed: old_str not found in {}.\nFirst 200 chars of file:\n{}",
                path_str,
                &content[..content.len().min(200)]
            ));
        }

        // Only replace the first occurrence to be surgical
        let new_content = content.replacen(old_str, new_str, 1);
        fs::write(&full_path, &new_content)?;

        self.memory_store.update_vfs_cache(full_path.clone(), new_content).await;

        {
            let path_abs = full_path.to_string_lossy().to_string();
            self.emit_tool_event("file-changed", json!({ "path": &path_abs }));
        }

        Ok(json!({
            "status": "success",
            "path": path_str,
            "message": format!("Replaced in {}", path_str)
        }))
    }

    pub(crate) async fn remove_item(&self, args: Value) -> Result<Value> {
        let path_str = args
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing path"))?;
        let recursive = args
            .get("recursive")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, path_str)?;

        if full_path.is_dir() {
            if recursive {
                fs::remove_dir_all(full_path)?;
            } else {
                fs::remove_dir(full_path)?;
            }
        } else {
            fs::remove_file(full_path)?;
        }
        Ok(serde_json::json!({ "status": "success" }))
    }

    pub(crate) async fn create_directory(&self, args: Value) -> Result<Value> {
        let path_str = args
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing path"))?;
        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, path_str)?;

        fs::create_dir_all(full_path)?;
        Ok(serde_json::json!({ "status": "success" }))
    }

    pub(crate) async fn rename_path(&self, args: Value) -> Result<Value> {
        let old_path_str = args
            .get("old_path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing old_path"))?;
        let new_path_str = args
            .get("new_path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing new_path"))?;

        let root = self.root_path.lock().await.clone();
        let old_full = self.validate_path(&root, old_path_str)?;
        let new_full = self.validate_path(&root, new_path_str)?;

        fs::rename(old_full, new_full)?;
        Ok(serde_json::json!({ "status": "success" }))
    }

    pub(crate) async fn list_files(&self, args: Value) -> Result<Value> {
        let path_str = args.get("path").and_then(|v| v.as_str()).unwrap_or(".");
        let recursive = args
            .get("recursive")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let root = self.root_path.lock().await.clone();
        let base_path = self.validate_path(&root, path_str)?;

        // BUG FIX (Windows): the old code routed every call through
        // `extract_path_and_pattern`, which for any non-existent path
        // returned (parent_dir, leaf_name). The non-recursive branch then
        // read `parent_dir` and *ignored* the pattern filter, so a call
        // like `list_files({path: "claurst/kilocode"})` silently listed
        // the workspace root and the model reported back "list_files is
        // returning root-level results regardless of path — a sandbox
        // quirk". It wasn't a sandbox; it was us.
        //
        // New rule: if the path is a glob pattern, take the parent +
        // filter route. Otherwise demand the resolved path *exists* and
        // is a directory, and surface a clean error if it isn't so the
        // model can recover instead of looping on the same broken call.
        let has_glob = path_str.contains('*') || path_str.contains('?') || path_str.contains('[');
        let (full_path, pattern_filter) = if has_glob {
            self.extract_path_and_pattern(&base_path.to_string_lossy(), "*")
        } else {
            (base_path, "*".to_string())
        };

        if !full_path.exists() {
            return Err(anyhow!(
                "list_files: path '{}' does not exist (resolved to {}). Use a path relative to the workspace root, or use list_files with `recursive: true` from a parent that does exist.",
                path_str,
                full_path.display()
            ));
        }
        if !full_path.is_dir() {
            return Err(anyhow!(
                "list_files: path '{}' is a file, not a directory. Use view_file to read it.",
                path_str
            ));
        }

        let ignore = crate::cursor_compat::CursorIgnoreSet::load(
            &*root,
            crate::cursor_compat::IgnoreScope::AiAccess,
        );

        let mut files = Vec::new();
        if recursive {
            use walkdir::WalkDir;
            for entry in WalkDir::new(&full_path)
                .max_depth(3)
                .into_iter()
                .filter_entry(|e| {
                    if ignore.is_ignored(e.path()) {
                        return false;
                    }
                    let name = e.file_name().to_string_lossy();
                    let is_hidden = name.starts_with('.') && name != "." && name != "..";
                    let is_ignored = name == "node_modules" || name == "target" || name == "dist" || name == "build" || name == ".git";
                    !is_hidden && !is_ignored
                })
                .filter_map(|e| e.ok())
            {
                let rel_path = entry
                    .path()
                    .strip_prefix(&*root)
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| entry.path().to_string_lossy().to_string());
                if pattern_filter == "*" || rel_path.contains(&pattern_filter) || pattern_filter == "**/*" {
                    let is_dir = entry.file_type().is_dir();
                    files.push(serde_json::json!({
                        "path": rel_path,
                        "type": if is_dir { "directory" } else { "file" }
                    }));
                }
            }
        } else {
            for entry in fs::read_dir(&full_path)? {
                let entry = entry?;
                let name = entry.file_name().to_string_lossy().to_string();
                let is_dir = entry.file_type()?.is_dir();
                files.push(serde_json::json!({
                    "name": name,
                    "type": if is_dir { "directory" } else { "file" }
                }));
            }
        }
        let result = Value::Array(files);

        // Emit artifact for file listing
        self.emit_tool_event("ai-artifact", json!({
            "type": "file",
            "path": path_str,
            "title": format!("Listed: {}", path_str),
            "content": format!("Found {} items", result.as_array().map(|a| a.len()).unwrap_or(0))
        }));

        Ok(result)
    }

    pub(crate) fn extract_shell_command(args: &Value) -> Result<&str> {
        if let Some(c) = args.get("command").and_then(|v| v.as_str()) {
            return Ok(c);
        }
        if let Some(c) = args.get("cmd").and_then(|v| v.as_str()) {
            return Ok(c);
        }
        if let Some(comp) = args.get("components") {
            if let Some(c) = comp.get("command").and_then(|v| v.as_str()) {
                return Ok(c);
            }
            if let Some(arr) = comp.as_array() {
                for item in arr {
                    if let Some(c) = item.get("command").and_then(|v| v.as_str()) {
                        return Ok(c);
                    }
                    if let Some(c) = item.as_str() {
                        return Ok(c);
                    }
                }
            }
        }
        Err(anyhow!("Missing command"))
    }

    pub(crate) fn apply_shell_suffix_to_grep_result(mut result: Value, suffix: &str) -> Value {
        let s = suffix.trim().to_lowercase();
        if s.contains("sort") {
            let raw = result
                .get("results")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let mut lines: Vec<String> = raw.lines().filter(|l| !l.is_empty()).map(str::to_string).collect();
            lines.sort_unstable();
            if s.contains("-u") || s.contains("uniq") {
                lines.dedup();
            }
            let match_count = lines.len();
            let joined = lines.join("\n");
            if let Some(obj) = result.as_object_mut() {
                obj.insert("results".to_string(), json!(joined));
                obj.insert("matches".to_string(), json!(match_count));
            }
        }
        result
    }

    pub(crate) fn infer_write_path_from_content(content: &str) -> Option<String> {
        use regex::Regex;
        if let Ok(re) = Regex::new(r"(?i)saved from (https?://[^\s\*]+)") {
            if let Some(cap) = re.captures(content) {
                if let Some(url) = cap.get(1) {
                    let url = url.as_str();
                    let after_scheme = url.split("://").nth(1)?;
                    let host = after_scheme.split('/').next()?;
                    let fname = after_scheme
                        .rsplit('/')
                        .next()
                        .filter(|s| !s.is_empty() && s.contains('.'))
                        .unwrap_or("bundle.js");
                    return Some(format!("recon/{}/{}", host, fname));
                }
            }
        }
        if (content.contains("__vite__") || content.contains("webpackJsonp"))
            && content.len() > 400
        {
            return Some("recon/bundle.js".to_string());
        }
        None
    }

    pub(crate) async fn run_command(&self, args: Value) -> Result<Value> {
        let command = Self::extract_shell_command(&args)?;
        let skip_grep_intercept = args
            .get("_grep_intercept_skip")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if !skip_grep_intercept {
            if let Some(intercept) = crate::ripgrep_search::try_intercept_shell_grep(command) {
                let prefix = intercept.prefix.as_deref().filter(|p| !p.trim().is_empty());
                let grep_has_path = intercept
                    .args
                    .get("path")
                    .and_then(|v| v.as_str())
                    .is_some_and(|p| !p.is_empty());

                // `curl url | grep PATTERN` — fetch via reqwest (bypasses broken shell curl/proxy).
                if !grep_has_path {
                    if let Some(pfx) = prefix {
                        if pfx.to_lowercase().contains("curl ") {
                            if let Some(url) = ShellTranslator::extract_curl_url(pfx) {
                                println!(
                                    "[Intercept] curl|grep → web_fetch + ripgrep: {}",
                                    url
                                );
                                let fetch = self.web_fetch_tool(json!({ "url": url })).await?;
                                let text = fetch
                                    .get("content")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("");
                                let root = self.root_path.lock().await.clone();
                                let tmp_dir = root.join(".agent").join("tmp");
                                std::fs::create_dir_all(&tmp_dir)?;
                                let tmp = tmp_dir.join(format!(
                                    "curl-grep-{}.txt",
                                    uuid::Uuid::new_v4().simple()
                                ));
                                std::fs::write(&tmp, text)?;
                                let mut grep_args = intercept.args.clone();
                                grep_args["path"] =
                                    json!(tmp.to_string_lossy().to_string());
                                let mut grep_result = self.grep(grep_args).await?;
                                if let Some(suffix) = intercept.suffix.filter(|s| !s.trim().is_empty())
                                {
                                    grep_result = Self::apply_shell_suffix_to_grep_result(
                                        grep_result, &suffix,
                                    );
                                }
                                return Ok(grep_result);
                            }
                        }
                    }
                }

                if let Some(pfx) = prefix {
                    let shell_hint = args
                        .get("shell_hint")
                        .and_then(|v| v.as_str())
                        .unwrap_or("run_command");
                    let mut prefix_args = json!({
                        "command": pfx,
                        "shell_hint": shell_hint,
                        "_grep_intercept_skip": true,
                    });
                    if args.get("background").and_then(|v| v.as_bool()) == Some(true) {
                        prefix_args["background"] = json!(true);
                    }
                    let prefix_result = Box::pin(self.run_command(prefix_args)).await?;
                    if prefix_result.get("status").and_then(|v| v.as_str()) == Some("failed")
                        || prefix_result.get("status").and_then(|v| v.as_str()) == Some("error")
                    {
                        return Ok(prefix_result);
                    }
                }
                println!(
                    "[Intercept] shell grep/rg → bundled ripgrep grep tool: {}",
                    command.lines().last().unwrap_or(command)
                );
                let mut grep_result = self.grep(intercept.args).await?;
                if let Some(suffix) = intercept.suffix.filter(|s| !s.trim().is_empty()) {
                    grep_result =
                        Self::apply_shell_suffix_to_grep_result(grep_result, &suffix);
                }
                return Ok(grep_result);
            }
            if crate::ripgrep_search::command_uses_shell_grep(command) {
                return Ok(json!({
                    "status": "blocked",
                    "error": "Shell grep/rg is disabled — use the grep tool (bundled ripgrep).",
                    "hint": "grep({ pattern: \"…\", path: \"file.js\" }) — works on single files and directories.",
                    "command": command,
                }));
            }
        }

        if let Some(reason) = crate::pentest_scope::block_localhost_pivot(command) {
            return Ok(json!({
                "status": "blocked",
                "error": reason,
                "command": command,
                "hint": "Use the exact in-scope URL from the user. See .agent/skills/bugbounty-hunter/SKILL.md"
            }));
        }
        let explicit_background = args.get("background").and_then(|v| v.as_bool());
        // Auto-route known long-running scanners to a background terminal unless the
        // caller explicitly chose foreground. A local model rarely sets background
        // itself, so a 20-minute `nmap -p-` would otherwise block the whole turn in
        // the un-cancellable foreground path. Background streams to a pollable
        // terminal instead, keeping the agent loop responsive during scans.
        let background = match explicit_background {
            Some(b) => b,
            None => {
                let is_long = is_long_running_command(command);
                if is_long {
                    println!("[AI] Auto-backgrounding long-running command: {}", command.lines().next().unwrap_or(command));
                }
                is_long
            }
        };

        let root = self.root_path.lock().await.clone();

        let shell_hint = args.get("shell_hint").and_then(|v| v.as_str()).unwrap_or("run_command");

        if background {
            let id = format!(
                "bg-{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
            );
            self.emit_tool_event(
                "terminal-create",
                json!({ "id": id.clone(), "command": command, "shell": shell_hint }),
            );

            return Ok(json!({
                "status": "success",
                "info": "Command started in background terminal. You MUST use terminal_get_status(term_id) to check if it finished, and terminal_read_output(term_id) to see what happened. DO NOT assume it finished immediately.",
                "term_id": id,
                "shell_hint": shell_hint,
                "hint": "Status polling is required for background tasks."
            }));
        }

        let (exec_path, exec_args) = ShellTranslator::translate_command(command, shell_hint);

        // ── Live-streaming execution ───────────────────────────────────────
        // The previous implementation called `.output()` which blocks until
        // the process exits and only then surfaces stdout/stderr. The user
        // wanted to see commands stream into the AIRI terminal panel in
        // real-time (especially valuable for long-running scripts like
        // `python security_audit.py` or `npm install`).
        //
        // We now spawn with piped stdio, read each pipe line-by-line on a
        // worker thread, and emit `ai-tool-stdout` events per line. The
        // terminal panel's listener (terminal.ts) writes those to the
        // active xterm.js instance as they arrive. The aggregated output
        // is still returned to the model as the tool result.
        use std::io::{BufRead, BufReader};
        use std::process::Stdio;

        let stream_id = format!("cmd-{}", uuid::Uuid::new_v4().simple());

        // Snapshot the sink once for the reader threads (Send+Sync), instead of
        // a tauri::AppHandle. The primary UI channel is `activity_log` (polled);
        // the sink emit is the secondary path.
        let sink = self.sink();

        let start_payload = json!({
            "stream_id": stream_id,
            "command": command,
            "shell_hint": shell_hint,
        });
        push_activity(&self.activity_log, "ai-tool-stdout-start", start_payload.clone());
        if let Some(ref s) = sink {
            s.emit("ai-tool-stdout-start", start_payload);
        }

        let mut cmd = std::process::Command::new(&exec_path);
        cmd.hidden()
            .args(&exec_args)
            .current_dir(&*root)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        if let Some(path) = crate::ide_shell::augmented_path_for_git_bash() {
            cmd.env("PATH", path);
        }
        if command.to_lowercase().contains("curl ") {
            ShellTranslator::sanitize_proxy_env(&mut cmd);
        }
        // Put the child in its own process group so the stop signal can kill the
        // whole tree (e.g. `sh -c "nmap ..."` plus the nmap it forks) via a
        // negative-PID signal. Windows uses taskkill /T for the same effect.
        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            cmd.process_group(0);
        }
        let mut child = cmd.spawn()?;
        // Register for stop-signal kill while this command runs.
        let child_pid = child.id();
        crate::process_registry::register(child_pid);

        let child_stdout = child
            .stdout
            .take()
            .ok_or_else(|| anyhow!("failed to capture stdout pipe"))?;
        let child_stderr = child
            .stderr
            .take()
            .ok_or_else(|| anyhow!("failed to capture stderr pipe"))?;

        let stdout_buf = Arc::new(std::sync::Mutex::new(String::new()));
        let stderr_buf = Arc::new(std::sync::Mutex::new(String::new()));

        // Reader thread: stdout
        let s_out = sink.clone();
        let sid_out = stream_id.clone();
        let buf_out = stdout_buf.clone();
        let act_out = self.activity_log.clone();
        let stdout_thread = std::thread::spawn(move || {
            let reader = BufReader::new(child_stdout);
            for line_res in reader.lines() {
                let Ok(line) = line_res else { break; };
                let payload = json!({
                    "stream_id": sid_out,
                    "line": line.clone(),
                    "stream": "stdout",
                });
                push_activity(&act_out, "ai-tool-stdout", payload.clone());
                if let Some(ref s) = s_out {
                    s.emit("ai-tool-stdout", payload);
                }
                if let Ok(mut b) = buf_out.lock() {
                    b.push_str(&line);
                    b.push('\n');
                    if b.len() > safe_io::MAX_CMD_OUTPUT {
                        let excess = b.len() - safe_io::MAX_CMD_OUTPUT;
                        b.drain(0..excess);
                    }
                }
            }
        });

        // Reader thread: stderr
        let s_err = sink.clone();
        let sid_err = stream_id.clone();
        let buf_err = stderr_buf.clone();
        let act_err = self.activity_log.clone();
        let stderr_thread = std::thread::spawn(move || {
            let reader = BufReader::new(child_stderr);
            for line_res in reader.lines() {
                let Ok(line) = line_res else { break; };
                let payload = json!({
                    "stream_id": sid_err,
                    "line": line.clone(),
                    "stream": "stderr",
                });
                push_activity(&act_err, "ai-tool-stdout", payload.clone());
                if let Some(ref s) = s_err {
                    s.emit("ai-tool-stdout", payload);
                }
                if let Ok(mut b) = buf_err.lock() {
                    b.push_str(&line);
                    b.push('\n');
                    if b.len() > safe_io::MAX_CMD_OUTPUT {
                        let excess = b.len() - safe_io::MAX_CMD_OUTPUT;
                        b.drain(0..excess);
                    }
                }
            }
        });

        // Wait for the process on a blocking thread so we don't tie up the
        // async runtime. Reader threads will join automatically when the
        // pipes close (which happens when the child exits).
        let status = tokio::task::spawn_blocking(move || child.wait())
            .await
            .map_err(|e| anyhow!("join error: {}", e))??;
        crate::process_registry::unregister(child_pid);
        let _ = stdout_thread.join();
        let _ = stderr_thread.join();

        let stdout = stdout_buf.lock().map(|s| s.clone()).unwrap_or_default();
        let stderr = stderr_buf.lock().map(|s| s.clone()).unwrap_or_default();

        let end_payload = json!({
            "stream_id": stream_id,
            "exit_code": status.code(),
            "success": status.success(),
        });
        push_activity(&self.activity_log, "ai-tool-stdout-end", end_payload.clone());
        if let Some(ref s) = sink {
            s.emit("ai-tool-stdout-end", end_payload);
            s.emit(
                "ai-artifact",
                json!({
                    "type": "terminal",
                    "title": format!("Run: {}", command),
                    "content": if status.success() { stdout.clone() } else { stderr.clone() }
                }),
            );
        }

        Ok(serde_json::json!({
            "stdout": stdout,
            "stderr": stderr,
            "success": status.success(),
            "status": if status.success() { "success" } else { "failed" }
        }))
    }

    pub(crate) async fn browser_search(&self, args: Value) -> Result<Value> {
        let query = args["query"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing query"))?;
        let url = format!(
            "https://www.google.com/search?q={}",
            urlencoding::encode(query)
        );
        self.browser_navigate(json!({ "url": url })).await
    }

    pub(crate) async fn browser_get_content_summary(&self, _args: Value) -> Result<Value> {
        crate::browser::browser_content_summary_for(&self.browser_state)
            .await
            .map_err(|e| anyhow!("{}", e))
    }

    pub(crate) async fn spawn_subagent(&self, args: Value) -> Result<Value> {
        let sub_task = args["task"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing task"))?;

        if let Some(state) = self.editor_state() {
            let engine = state.ai.engine.clone();
            let sink = self.sink();
            let task_id = uuid::Uuid::new_v4().to_string();
            let task_id_clone = task_id.clone();
            let sub_task_clone = sub_task.to_string();

            // Prepare sub-agent request
            let req = crate::ai_engine::AiRequest {
                provider: "lemonade".to_string(), // Native Local Subagent
                model: "qwen2.5-coder-abliterate:7b".to_string(), // Or could be pulled from global state
                messages: vec![crate::ai_engine::ChatMessage {
                    role: "user".to_string(),
                    content: Some(crate::ai_engine::MessageContent::Text(
                        sub_task_clone.clone(),
                    )),
                    tool_calls: None,
                    tool_call_id: None,
                    metadata: None,
                }],
                temperature: Some(0.7),
                autonomous: true,
                mode: None,
                cyber_mode: None,
                root_access: Some(true),
                inference_url: None,
                tools: None,
                reasoning_budget: None,
                reasoning_effort: None,
                reasoning_enabled: None,
                feature: None,
            };

            println!(
                "[SUBAGENT] Spawning async sub-agent [{}] for task: {}",
                task_id, sub_task
            );

            // Spawn background task (non-Send workaround: use thread-local tokio runtime)
            std::thread::spawn(move || {
                let rt = match tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                {
                    Ok(rt) => rt,
                    Err(e) => {
                        eprintln!("[SubAgent] Failed to create tokio runtime: {e}");
                        return;
                    }
                };

                if let Some(ref s) = sink {
                    s.emit(
                        "subagent-progress",
                        json!({
                            "task_id": task_id_clone,
                            "status": "running",
                            "progress": 5,
                            "message": "Initializing sub-agent session..."
                        }),
                    );
                }

                let res = rt.block_on(engine.autonomous_loop(req, None));

                match res {
                    Ok(answer) => {
                        if let Some(ref s) = sink {
                            s.emit(
                                "subagent-progress",
                                json!({
                                    "task_id": task_id_clone,
                                    "status": "completed",
                                    "progress": 100,
                                    "result": answer
                                }),
                            );
                        }
                    }
                    Err(e) => {
                        if let Some(ref s) = sink {
                            s.emit(
                                "subagent-progress",
                                json!({
                                    "task_id": task_id_clone,
                                    "status": "failed",
                                    "progress": 0,
                                    "error": e.to_string()
                                }),
                            );
                        }
                    }
                }
            });

            Ok(json!({
                "status": "success",
                "task_id": task_id,
                "message": "Sub-agent spawned in background."
            }))
        } else {
            Err(anyhow!("App handle not set"))
        }
    }

    /// FastContext repository explorer — spawns a lightweight subagent that does
    /// parallel READ/GLOB/GREP and returns compact file citations. Uses the
    /// FastContext-1.0-4B-SFT model if available, otherwise falls back to the
    /// main agent's model with explorer-mode prompting.
    pub(crate) async fn explore_repository(&self, args: Value) -> Result<Value> {
        let query = args["query"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing query"))?;
        let max_results = args["max_results"].as_u64().unwrap_or(10) as usize;
        let file_pattern = args["file_pattern"].as_str();

        let inference_url = {
            if let Some(state) = self.editor_state() {
                state.ai.engine.resolved_local_base(&crate::ai_engine::AiRequest {
                    provider: "lemonade".to_string(),
                    model: "fastcontext".to_string(),
                    messages: vec![],
                    temperature: None,
                    autonomous: false,
                    mode: None,
                    cyber_mode: None,
                    root_access: None,
                    inference_url: None,
                    tools: None,
                    reasoning_budget: None,
                    reasoning_effort: None,
                    reasoning_enabled: None,
                    feature: None,
                }).await
            } else {
                // No EditorState available — fall back to the default local
                // inference endpoint. This path is rare (tool invoked before
                // state initialization) and the URL is cosmetic here since
                // `explore_repository` uses it only for model discovery.
                "http://127.0.0.1:11434".to_string()
            }
        };

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(90))
            .build()
            .unwrap_or_default();

        // Resolve the actual installed FastContext tag (keeps `hf.co/` prefix etc.).
        let models_url = format!("{}/api/tags", inference_url);
        let fc_model: Option<String> = match client.get(&models_url).send().await {
            Ok(resp) => resp.json::<serde_json::Value>().await.ok().and_then(|body| {
                body.get("models").and_then(|m| m.as_array()).and_then(|models| {
                    models.iter().find_map(|m| {
                        m.get("name").and_then(|n| n.as_str())
                            .filter(|n| n.to_lowercase().contains("fastcontext"))
                            .map(|n| n.to_string())
                    })
                })
            }),
            Err(_) => None,
        };

        let Some(model) = fc_model else {
            println!("[EXPLORE] FastContext not installed, using built-in explorer");
            return self.builtin_explore(query, max_results, file_pattern).await;
        };

        println!("[EXPLORE] Using FastContext subagent ({}) for: {}", model, query);
        match self.run_fastcontext_loop(&client, &inference_url, &model, query, file_pattern, max_results).await {
            Ok(citations) if !citations.is_empty() => Ok(json!({
                "status": "success",
                "explorer": model,
                "query": query,
                "citations": citations,
            })),
            other => {
                if let Err(e) = &other {
                    println!("[EXPLORE] FastContext loop error: {} — falling back to built-in", e);
                } else {
                    println!("[EXPLORE] FastContext returned no citations — falling back to built-in");
                }
                self.builtin_explore(query, max_results, file_pattern).await
            }
        }
    }

    /// Read-only agentic exploration loop. FastContext issues real READ/GLOB/GREP
    /// tool calls (native Ollama tools, or its trained text format as fallback)
    /// which we execute against the repo, looping until it emits `<final_answer>`
    /// citations. Returns parsed `path:line` citations.
    async fn run_fastcontext_loop(
        &self,
        client: &reqwest::Client,
        inference_url: &str,
        model: &str,
        query: &str,
        file_pattern: Option<&str>,
        max_results: usize,
    ) -> Result<Vec<Value>> {
        let scope = file_pattern
            .map(|p| format!("Restrict search to files matching: {}", p))
            .unwrap_or_else(|| "Search the entire repository.".to_string());
        let system = format!(
            "You are FastContext, a repository-exploration subagent. Your ONLY job is to \
             LOCATE relevant code for the main agent — never edit or write code.\n\
             {scope}\n\
             Use the read-only tools (read_file, glob, grep, code_search) to find the files \
             and line ranges relevant to the query. Issue several tool calls per turn to \
             cover complementary hypotheses. When you have enough evidence, STOP calling \
             tools and output a compact block:\n\
             <final_answer>\n<path>:<start>-<end>  short reason\n...\n</final_answer>\n\
             List at most {max_results} citations, most relevant first."
        );
        // Read-only tool schemas exposed to the explorer.
        let tools = json!([
            { "type": "function", "function": { "name": "read_file", "description": "Read a file (optionally a line range).",
              "parameters": { "type": "object", "properties": {
                  "path": {"type": "string"}, "start_line": {"type": "integer"}, "end_line": {"type": "integer"} }, "required": ["path"] } } },
            { "type": "function", "function": { "name": "glob", "description": "Find files by name/glob pattern.",
              "parameters": { "type": "object", "properties": { "pattern": {"type": "string"}, "path": {"type": "string"} }, "required": ["pattern"] } } },
            { "type": "function", "function": { "name": "grep", "description": "Search file contents for a string across the repo.",
              "parameters": { "type": "object", "properties": { "query": {"type": "string"} }, "required": ["query"] } } },
            { "type": "function", "function": { "name": "code_search", "description": "Semantic/code search across the repo.",
              "parameters": { "type": "object", "properties": { "query": {"type": "string"} }, "required": ["query"] } } }
        ]);

        let mut messages = vec![
            json!({ "role": "system", "content": system }),
            json!({ "role": "user", "content": format!("Query: {}", query) }),
        ];
        let chat_url = format!("{}/api/chat", inference_url);

        for _turn in 0..6 {
            let body = json!({
                "model": model,
                "messages": messages,
                "tools": tools,
                "stream": false,
                "options": { "temperature": 0.1, "num_ctx": 16384 }
            });
            let resp = client.post(&chat_url).json(&body).send().await
                .map_err(|e| anyhow!("fastcontext chat: {}", e))?;
            let v: Value = resp.json().await.map_err(|e| anyhow!("fastcontext json: {}", e))?;
            let msg = v.get("message").cloned().unwrap_or_else(|| json!({}));
            let content = msg.get("content").and_then(|c| c.as_str()).unwrap_or("").to_string();

            // Native tool calls.
            let tool_calls = msg.get("tool_calls").and_then(|t| t.as_array()).cloned().unwrap_or_default();
            if !tool_calls.is_empty() {
                messages.push(msg.clone());
                for tc in &tool_calls {
                    let name = tc["function"]["name"].as_str().unwrap_or("");
                    let raw_args = tc["function"]["arguments"].clone();
                    let args = if raw_args.is_string() {
                        serde_json::from_str(raw_args.as_str().unwrap_or("{}")).unwrap_or(json!({}))
                    } else { raw_args };
                    let result = self.exec_explore_tool(name, args).await;
                    messages.push(json!({ "role": "tool", "content": result }));
                }
                continue;
            }

            // Done if the model produced a final answer (or plain text, no further calls).
            if content.contains("<final_answer>") {
                return Ok(self.parse_explorer_citations(&content, max_results));
            }

            // Text-ReAct fallback: parse FastContext's trained READ()/GLOB()/GREP() calls.
            let calls = parse_text_tool_calls(&content);
            if calls.is_empty() {
                return Ok(self.parse_explorer_citations(&content, max_results));
            }
            messages.push(json!({ "role": "assistant", "content": content }));
            let mut obs = String::new();
            for (name, arg) in calls {
                let args = match name.as_str() {
                    "read_file" => json!({ "path": arg }),
                    "glob" => json!({ "pattern": arg }),
                    _ => json!({ "query": arg }),
                };
                let result = self.exec_explore_tool(&name, args).await;
                obs.push_str(&format!("{}({}) =>\n{}\n\n", name, arg, result.chars().take(1500).collect::<String>()));
            }
            messages.push(json!({ "role": "user", "content": format!("Observations:\n{}\nContinue or give <final_answer>.", obs) }));
        }
        Ok(Vec::new())
    }

    /// Execute one read-only explorer tool call, mapping the explorer's tool names
    /// onto the existing dispatch. Returns a compact string result (capped).
    async fn exec_explore_tool(&self, name: &str, args: Value) -> String {
        if name == "code_search" {
            return self.code_search(args).await
                .map(|v| v.to_string()).unwrap_or_else(|e| format!("error: {}", e))
                .chars().take(3000).collect();
        }
        let internal = match name {
            "read_file" => "view_file",
            "glob" => "find_by_name",
            "grep" => "search_files",
            other => other,
        };
        // grep maps to search_files which expects `query`, not `pattern`.
        let args = if name == "grep" {
            let q = args.get("query").or_else(|| args.get("pattern")).cloned().unwrap_or(json!(""));
            json!({ "query": q })
        } else { args };
        self.handle_fs_tool(internal, args).await
            .map(|v| v.to_string())
            .unwrap_or_else(|e| format!("error: {}", e))
            .chars().take(3000).collect()
    }

    /// Parse citations from a FastContext-style <final_answer> block
    fn parse_explorer_citations(&self, text: &str, max_results: usize) -> Vec<Value> {
        let mut citations = Vec::new();

        // Try to find <final_answer> block
        if let Some(start) = text.find("<final_answer>") {
            if let Some(end) = text[start..].find("</final_answer>") {
                let block = &text[start + 14..start + end];
                // Parse file:line citations
                for line in block.lines() {
                    if citations.len() >= max_results {
                        break;
                    }
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') {
                        continue;
                    }
                    // Try to extract path:line patterns
                    if let Some(colon_pos) = line.find(':') {
                        let path = line[..colon_pos].trim().to_string();
                        let rest = line[colon_pos + 1..].trim();
                        let line_range = rest.split_whitespace().next().unwrap_or("").to_string();
                        citations.push(json!({
                            "path": path,
                            "line_range": line_range,
                            "context": rest.to_string()
                        }));
                    } else {
                        citations.push(json!({
                            "path": line.to_string(),
                            "line_range": "",
                            "context": ""
                        }));
                    }
                }
            }
        }

        // If no <final_answer> found, try to extract file paths from the response
        if citations.is_empty() {
            let mut seen = std::collections::HashSet::new();
            for line in text.lines() {
                if citations.len() >= max_results {
                    break;
                }
                // Look for paths that look like source files
                if let Some(path) = self.extract_source_path(line) {
                    if !seen.contains(&path) {
                        seen.insert(path.clone());
                        citations.push(json!({
                            "path": path,
                            "line_range": "",
                            "context": line.trim().to_string()
                        }));
                    }
                }
            }
        }

        citations
    }

    /// Extract a source file path from a line of text
    fn extract_source_path(&self, line: &str) -> Option<String> {
        let line = line.trim();
        // Look for patterns like src/foo.rs or path/to/file.ts
        let extensions = [".rs", ".ts", ".tsx", ".js", ".jsx", ".py", ".go", ".java", ".c", ".cpp", ".h"];
        for ext in &extensions {
            if let Some(pos) = line.find(ext) {
                // Find the start of the path
                let before = &line[..pos + ext.len()];
                if let Some(start) = before.rfind(|c: char| c == ' ' || c == '(' || c == '"' || c == '\'' || c == '`') {
                    let path = &line[start + 1..pos + ext.len()];
                    if path.contains('/') || path.contains('\\') {
                        return Some(path.to_string());
                    }
                } else if !before.is_empty() {
                    return Some(before.to_string());
                }
            }
        }
        None
    }

    /// Built-in exploration using existing tools (fallback when FastContext isn't available)
    async fn builtin_explore(&self, query: &str, max_results: usize, file_pattern: Option<&str>) -> Result<Value> {
        let root = self.root_path.lock().await.clone();
        let mut citations = Vec::new();

        // Step 1: GLOB to find relevant files
        let glob_pattern = if let Some(pat) = file_pattern {
            pat.to_string()
        } else {
            // Guess file extensions from query
            "**/*.{rs,ts,tsx,js,jsx,py,go}".to_string()
        };

        let glob_args = json!({
            "pattern": glob_pattern,
            "path": root.to_string_lossy()
        });

        if let Ok(glob_result) = self.handle_fs_tool("find_by_name", glob_args).await {
            if let Some(files) = glob_result.get("files").and_then(|f| f.as_array()) {
                // Step 2: For each file, grep for the query terms
                let query_terms: Vec<&str> = query.split_whitespace().collect();
                for file_entry in files.iter().take(max_results * 3) {
                    if citations.len() >= max_results {
                        break;
                    }
                    if let Some(path) = file_entry.get("path").and_then(|p| p.as_str()) {
                        let grep_args = json!({
                            "pattern": query_terms.first().unwrap_or(&""),
                            "path": path,
                        });
                        if let Ok(grep_result) = self.handle_fs_tool("search_files", grep_args).await {
                            if let Some(matches) = grep_result.get("matches").and_then(|m| m.as_array()) {
                                if !matches.is_empty() {
                                    citations.push(json!({
                                        "path": path,
                                        "line_range": matches.first()
                                            .and_then(|m| m.get("line"))
                                            .and_then(|l| l.as_u64())
                                            .map(|l| format!("L{}", l))
                                            .unwrap_or_default(),
                                        "context": matches.first()
                                            .and_then(|m| m.get("text"))
                                            .and_then(|t| t.as_str())
                                            .unwrap_or("")
                                            .to_string(),
                                        "match_count": matches.len()
                                    }));
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(json!({
            "status": "success",
            "explorer": "builtin",
            "query": query,
            "citations": citations,
            "note": "FastContext not available — used built-in exploration. Pull FastContext for better results: ollama pull hf.co/mitkox/FastContext-1.0-4B-SFT-Q4_K_M-GGUF:Q4_K_M"
        }))
    }

    pub(crate) async fn generate_image(&self, args: Value) -> Result<Value> {
        let prompt = args["prompt"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing prompt"))?;
        let path = args["path"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing path"))?;

        let root = self.root_path.lock().await.clone();
        let full = self.validate_path(&root, path)?;

        let keys = self.load_keys_value().await;
        let google_key = keys.get("google").and_then(|v| v.as_str()).filter(|s| !s.trim().is_empty());
        let provider = args["provider"].as_str().unwrap_or("auto");

        let saved = if provider == "gemini" || (provider == "auto" && google_key.is_some()) {
            let key = google_key.ok_or_else(|| anyhow!("Google API key required for Gemini image gen"))?;
            crate::image_gen::generate_with_gemini(key, prompt, &full)
                .await
                .map_err(|e| anyhow!("{e}"))
        } else {
            crate::image_gen::generate_with_local_image(prompt, &full)
                .await
                .map_err(|e| anyhow!("{e}"))
        };

        match saved {
            Ok(out) => Ok(json!({
                "status": "success",
                "path": out,
                "message": format!("Image saved to {out}")
            })),
            Err(_) if provider == "auto" && google_key.is_some() => {
                let key = google_key.unwrap();
                let out = crate::image_gen::generate_with_gemini(key, prompt, &full)
                    .await
                    .map_err(|e| anyhow!("{e}"))?;
                Ok(json!({
                    "status": "success",
                    "path": out,
                    "message": format!("Image saved to {out} (Gemini fallback)")
                }))
            }
            Err(e) => Err(e),
        }
    }

    pub(crate) async fn analyze_image(&self, args: Value) -> Result<Value> {
        let path = args["path"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing path"))?;
        let question = args
            .get("question")
            .and_then(|v| v.as_str())
            .unwrap_or("Describe this image in detail.");

        let root = self.root_path.lock().await.clone();
        let full = self.validate_path(&root, path)?;
        if !full.exists() {
            return Err(anyhow!("Image not found: {}", full.display()));
        }

        let keys = self.load_keys_value().await;
        let google_key = keys.get("google").and_then(|v| v.as_str()).filter(|s| !s.trim().is_empty());
        let provider = args["provider"].as_str().unwrap_or("auto");

        let analysis = if provider == "gemini" || (provider == "auto" && google_key.is_some()) {
            let key = google_key.ok_or_else(|| anyhow!("Google API key required"))?;
            crate::image_gen::analyze_with_gemini(key, &full, question)
                .await
                .map_err(|e| anyhow!("{e}"))?
        } else {
            match crate::image_gen::analyze_with_local_vision(&full, question).await {
                Ok(t) => t,
                Err(_) if google_key.is_some() => {
                    crate::image_gen::analyze_with_gemini(google_key.unwrap(), &full, question)
                        .await
                        .map_err(|e| anyhow!("{e}"))?
                }
                Err(e) => return Err(anyhow!("{e}")),
            }
        };

        Ok(json!({
            "status": "success",
            "analysis": analysis,
            "path": full.to_string_lossy()
        }))
    }

    pub(crate) async fn code_search(&self, args: Value) -> Result<Value> {
        let query = args["query"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing query"))?;
        let pattern = args
            .get("file_pattern")
            .and_then(|v| v.as_str())
            .unwrap_or("*");

        let root = self.root_path.lock().await.clone();
        let mut results = Vec::new();
        let glob_pattern = format!("**/{}", pattern);

        // Use walkdir for recursive search (cap depth to prevent scanning node_modules etc.)
        for entry in walkdir::WalkDir::new(&root)
            .max_depth(10)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                let path = entry.path();

                // Use central safety module for all checks
                if safe_io::should_skip_path(path) { continue; }
                if let Ok(meta) = std::fs::metadata(path) {
                    if meta.len() > safe_io::MAX_TEXT_FILE_SIZE { continue; }
                } else { continue; }

                // Match file pattern if provided
                if pattern != "*" {
                    let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    let pat_ok = glob::Pattern::new(&glob_pattern)
                        .map(|p| p.matches_path(path))
                        .unwrap_or(false);
                    let pat2_ok = glob::Pattern::new(pattern)
                        .map(|p| p.matches(file_name))
                        .unwrap_or(false);
                    if !pat_ok && !pat2_ok {
                        continue;
                    }
                }

                if let Ok(content) = fs::read_to_string(path) {
                    if content.contains(query) {
                        results.push(json!({
                            "path": path.strip_prefix(&root).unwrap_or(path).to_string_lossy(),
                            "matches": content.matches(query).count()
                        }));
                    }
                }
            }
            if results.len() > 100 {
                break;
            }
        }

        Ok(json!({
            "status": "success",
            "results": results,
            "count": results.len()
        }))
    }

    pub(crate) async fn dependency_graph(&self, args: Value) -> Result<Value> {
        let path_str = args["path"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing path"))?;

        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, path_str)?;

        let mut imports = Vec::new();
        if let Ok(content) = safe_io::safe_read(&full_path) {
            // Very simple regex-based discovery for demonstration
            // In a real implementation, we'd use tree-sitter or a proper parser
            let re_rust = regex::Regex::new(r"use\s+([^;]+);").unwrap();
            let re_ts = regex::Regex::new(r#"import.*from\s+['"]([^'"]+)['"]"#).unwrap();

            for cap in re_rust.captures_iter(&content) {
                imports.push(cap[1].to_string());
            }
            for cap in re_ts.captures_iter(&content) {
                imports.push(cap[1].to_string());
            }
        }

        Ok(json!({
            "status": "success",
            "file": path_str,
            "dependencies": imports
        }))
    }

    /// Return the stripped-body signatures (functions, types, traits, doc
    /// comments) for a single source file — the "on demand" half of the tiered
    /// codebase-context strategy. The always-on `codebase_map` orients the model
    /// to directories; this pulls a specific file's contract without its full
    /// body, keeping token cost low.
    pub(crate) async fn get_file_signatures(&self, args: Value) -> Result<Value> {
        let path_str = args
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("get_file_signatures requires a 'path'"))?;
        let root = self.root_path.lock().await.clone();
        let full = self.validate_path(&root, path_str)?;
        let content = crate::domain::safe_io::safe_read(&full)
            .map_err(|e| anyhow!("failed to read {}: {}", path_str, e))?;
        let bp = crate::domain::indexing::structural_blueprints::StructuralBlueprints::new(root.clone());
        match bp.blueprint_file(&full, &content) {
            Some(entry) => {
                let serialized = crate::domain::indexing::structural_blueprints::StructuralBlueprints::serialize_blueprints(std::slice::from_ref(&entry));
                Ok(json!({
                    "status": "success",
                    "path": entry.file_path,
                    "language": entry.language,
                    "signature_count": entry.signatures.len(),
                    "signatures": serialized,
                }))
            }
            None => Ok(json!({
                "status": "unsupported",
                "path": path_str,
                "hint": "Signature extraction supports .rs/.ts/.tsx/.js/.jsx/.py — use view_file for other files.",
            })),
        }
    }

    /// Return the compact, always-on codebase directory map (one line per source
    /// directory). Cheap orientation for the whole repo; pair with
    /// `get_file_signatures` for per-file detail.
    pub(crate) async fn codebase_map_tool(&self, args: Value) -> Result<Value> {
        let root = self.root_path.lock().await.clone();
        // Default ~3K-token budget (≈12K chars); caller may override.
        let max_chars = args
            .get("max_chars")
            .and_then(|v| v.as_u64())
            .unwrap_or(12_000) as usize;
        let map = crate::domain::indexing::codebase_map::generate_repo_map_cached(&root, max_chars);
        Ok(json!({
            "status": "success",
            "map": map,
            "chars": map.len(),
        }))
    }

    pub(crate) async fn terminal_terminate(&self, args: Value) -> Result<Value> {
        let state = self.editor_state().ok_or_else(|| anyhow!("EditorState unavailable"))?;
        let term_id = args
            .get("term_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing term_id"))?;

        let mut processes = state.terminal.processes.lock().await;
        if let Some(mut child) = processes.remove(term_id) {
            let _ = child.kill();
            state.terminal.masters.lock().await.remove(term_id);
            state.terminal.writers.lock().await.remove(term_id);
            state.terminal.buffers.lock().await.remove(term_id);
            let _ = state.terminal.pending.lock().map(|mut p| p.remove(term_id));
            Ok(json!({ "status": "success", "info": format!("Terminal {} terminated.", term_id) }))
        } else {
            Ok(json!({ "status": "error", "message": "Terminal not found or already closed." }))
        }
    }

    pub(crate) async fn terminal_get_status(&self, args: Value) -> Result<Value> {
        let state = self.editor_state().ok_or_else(|| anyhow!("EditorState unavailable"))?;
        let term_id = args
            .get("term_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing term_id"))?;

        let mut processes = state.terminal.processes.lock().await;
        if let Some(child) = processes.get_mut(term_id) {
            match child.try_wait() {
                Ok(Some(status)) => Ok(
                    json!({ "active": false, "success": status.success(), "status": if status.success() { "success" } else { "failed" } }),
                ),
                Ok(None) => Ok(json!({ "active": true, "status": "running" })),
                Err(e) => Err(anyhow!("Error checking process: {}", e)),
            }
        } else {
            Ok(
                json!({ "active": false, "info": "Process not found (likely already exited and cleaned up)." }),
            )
        }
    }

    pub(crate) async fn search_files(&self, args: Value) -> Result<Value> {
        let query = args
            .get("query")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing query"))?;

        let root = self.root_path.lock().await.clone();
        let hits = crate::ripgrep_search::ripgrep_search(crate::ripgrep_search::RipgrepQuery {
            pattern: query,
            root: &root,
            include: None,
            max_results: 100,
            case_insensitive: true,
            fixed_string: true,
            file: None,
        })
        .map_err(|e| anyhow!(e))?;

        let results: Vec<Value> = hits
            .into_iter()
            .map(|h| {
                json!({
                    "file": h.path,
                    "line": h.line,
                    "match": h.content
                })
            })
            .collect();
        Ok(Value::Array(results))
    }

    pub(crate) async fn semantic_search(&self, args: Value) -> Result<Value> {
        let query = args["query"].as_str().ok_or_else(|| anyhow!("Missing query"))?.to_string();
        let q_lower = query.to_lowercase();
        let limit = args["limit"].as_u64().unwrap_or(20) as usize;
        let slots: Vec<crate::memory_store::SemanticSlot> = self.memory_store.slots.read().await.clone();
        
        let mut results = Vec::new();
        for slot in slots {
            if slot.content.to_lowercase().contains(&q_lower) || 
               slot.tags.iter().any(|t| t.to_lowercase().contains(&q_lower)) ||
               slot.id.to_lowercase().contains(&q_lower) 
            {
                results.push(json!({
                    "source": "aim",
                    "id": slot.id,
                    "category": slot.category,
                    "relevance_tags": slot.tags,
                    "path_hint": slot.content
                }));
            }
            if results.len() >= limit { break; }
        }

        if results.len() < limit {
            if let Some(indexer) = self.vector_indexer.lock().await.clone() {
                let remaining = limit - results.len();
                if let Ok(hits) = indexer.search_codebase(&query, remaining).await {
                    for hit in hits {
                        results.push(json!({
                            "source": "vector_index",
                            "file": hit.file_path,
                            "start_line": hit.start_line,
                            "end_line": hit.end_line,
                            "relevance_score": hit.relevance_score,
                            "preview": hit.context,
                            "content": hit.content.chars().take(240).collect::<String>(),
                        }));
                    }
                }
            }
        }

        Ok(json!({
            "query": query,
            "results": results,
            "count": results.len()
        }))
    }

    /// Packs the entire indexed codebase into a compact semantic map.
    /// The AI calls this once to get the "6 gist tokens" zero-grep overview.
    pub(crate) async fn aim_pack_context(&self, args: Value) -> Result<Value> {
        let query = args.get("query").and_then(|v| v.as_str()).unwrap_or("");
        let max_slots = args.get("max_slots").and_then(|v| v.as_u64()).unwrap_or(20) as usize;

        let gist = self.memory_store.build_compact_gist().await;
        let tree_summary = self.memory_store.get_project_tree_summary().await;
        let tree = self.memory_store.get_project_tree().await;
        // Complete codebase map (every file + symbol) — total recall, not RAG.
        let codebase_map = self.memory_store.build_full_codebase_map(120_000).await;

        // Pull code-category slots — these are the indexed file summaries
        let all_slots = self.memory_store.slots.read().await.clone();
        let code_slots: Vec<serde_json::Value> = all_slots.iter()
            .filter(|s| s.category == "code" || s.category == "fix" || s.category == "decision")
            .filter(|s| {
                if query.is_empty() { return true; }
                let q = query.to_lowercase();
                s.content.to_lowercase().contains(&q)
                    || s.tags.iter().any(|t| t.to_lowercase().contains(&q))
            })
            .take(max_slots)
            .map(|s| {
                let file = s.metadata.as_ref()
                    .and_then(|m| m.get("path"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                json!({
                    "file": file,
                    "category": s.category,
                    "preview": s.content.chars().take(120).collect::<String>(),
                    "tags": s.tags.iter().take(4).collect::<Vec<_>>(),
                })
            })
            .collect();

        let indexed = tree.len();
        Ok(json!({
            "schema": "kortex.aim.packed/v1",
            "gist": gist,
            "codebase_map": codebase_map,
            "project_summary": tree_summary,
            "total_indexed_files": indexed,
            "context_slots": code_slots,
            "slot_count": code_slots.len(),
            "instruction": if indexed > 0 {
                "ZERO-GREP MODE ACTIVE. Trust this map. Do NOT call list_files or grep to understand the codebase — the structure above IS the complete index. Go directly to the relevant file."
            } else {
                "Workspace not yet indexed. Call trigger_workspace_index first, then aim_pack_context again."
            }
        }))
    }

    /// Query the AIM index for exact file + line location of a symbol or concept.
    /// Faster and more precise than grep for indexed codebases.
    pub(crate) async fn aim_query_spans_tool(&self, args: Value) -> Result<Value> {
        let query = args.get("query").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing query"))?;
        let q = query.to_lowercase();

        let all_slots = self.memory_store.slots.read().await.clone();
        let mut results: Vec<serde_json::Value> = Vec::new();

        for slot in &all_slots {
            let score = {
                let content_match = slot.content.to_lowercase().contains(&q);
                let tag_match = slot.tags.iter().any(|t| t.to_lowercase().contains(&q));
                let id_match = slot.id.to_lowercase().contains(&q);
                if id_match { 3 } else if tag_match { 2 } else if content_match { 1 } else { 0 }
            };
            if score == 0 { continue; }

            let file = slot.metadata.as_ref()
                .and_then(|m| m.get("path"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let line = slot.metadata.as_ref()
                .and_then(|m| m.get("line"))
                .and_then(|v| v.as_u64())
                .unwrap_or(0);
            let kind = slot.tags.iter()
                .find(|t| t.starts_with("symbol:"))
                .map(|t| t[7..].to_string())
                .unwrap_or_else(|| slot.category.clone());

            results.push(json!({
                "file": file,
                "line": line,
                "kind": kind,
                "preview": slot.content.chars().take(160).collect::<String>(),
                "score": score,
            }));

            if results.len() >= 12 { break; }
        }

        // Sort by score descending
        results.sort_by(|a, b| {
            b["score"].as_u64().unwrap_or(0)
                .cmp(&a["score"].as_u64().unwrap_or(0))
        });

        Ok(json!({
            "schema": "kortex.aim.spans/v1",
            "query": query,
            "results": results,
            "hit_count": results.len(),
        }))
    }

    pub(crate) async fn find_symbols(&self, args: Value) -> Result<Value> {
        let pattern = args.get("pattern").and_then(|v| v.as_str()).unwrap_or("").to_lowercase();
        let slots: Vec<crate::memory_store::SemanticSlot> = self.memory_store.slots.read().await.clone();
        
        let mut symbols = Vec::new();
        for slot in slots {
            for tag in slot.tags {
                if tag.starts_with("symbol:") {
                    let sym_name = &tag[7..];
                    if pattern.is_empty() || sym_name.to_lowercase().contains(&pattern) {
                        symbols.push(json!({
                            "name": sym_name,
                            "file": slot.content
                        }));
                    }
                }
            }
            if symbols.len() > 200 { break; }
        }
        Ok(json!(symbols))
    }

    pub(crate) async fn read_file_lines(&self, args: Value) -> Result<Value> {
        let path_str = args["path"].as_str().ok_or_else(|| anyhow!("Missing path"))?;
        let start = args["start_line"].as_u64().unwrap_or(1) as usize;
        let end = args["end_line"].as_u64().unwrap_or(100) as usize;

        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, path_str)?;

        let metadata = fs::metadata(&full_path)?;
        let file_size = metadata.len();

        if file_size > 10 * 1024 * 1024 {
            use std::io::{BufRead, BufReader};
            let file = fs::File::open(&full_path)?;
            let mut reader = BufReader::new(file);
            let mut lines_out = Vec::new();
            let mut line_num = 0usize;
            let mut buf = String::new();
            let safe_start = start.max(1);
            let safe_end = end.max(safe_start);
            loop {
                buf.clear();
                let bytes_read = reader.read_line(&mut buf)?;
                if bytes_read == 0 { break; }
                line_num += 1;
                if line_num >= safe_start && line_num <= safe_end {
                    lines_out.push(buf.trim_end_matches('\n').trim_end_matches('\r').to_string());
                }
                if line_num > safe_end { break; }
            }
            Ok(json!({
                "path": path_str,
                "total_lines": line_num,
                "range": format!("{}-{}", safe_start, safe_end.min(line_num)),
                "content": lines_out.join("\n")
            }))
        } else {
            let content = safe_io::safe_read(&full_path)?;
            let lines: Vec<&str> = content.lines().collect();
            let safe_start = start.max(1).min(lines.len());
            let safe_end = end.min(lines.len()).max(safe_start);
            let subset = &lines[safe_start-1..safe_end];
            Ok(json!({
                "path": path_str,
                "total_lines": lines.len(),
                "range": format!("{}-{}", safe_start, safe_end),
                "content": subset.join("\n")
            }))
        }
    }

    pub(crate) async fn reindex_project(&self, _args: Value) -> Result<Value> {
        self.emit_tool_event("reindex-project", json!({}));
        Ok(json!({"status": "success", "info": "Background re-indexing triggered."}))
    }

    pub(crate) async fn list_dir_tree(&self, args: Value) -> Result<Value> {
        let path_str = args.get("path").and_then(|v| v.as_str()).unwrap_or(".");
        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, path_str)?;
        
        let mut tree = String::new();
        use walkdir::WalkDir;
        
        for entry in WalkDir::new(full_path)
            .max_depth(3)
            .into_iter()
            .filter_map(|e| e.ok()) {
            
            let depth = entry.depth();
            let name = entry.file_name().to_string_lossy();
            let indent = "  ".repeat(depth);
            
            if entry.file_type().is_dir() {
                tree.push_str(&format!("{} {}/\n", indent, name));
            } else {
                tree.push_str(&format!("{} {}\n", indent, name));
            }
            
            if tree.len() > 10000 {
                tree.push_str("... (truncated)\n");
                break;
            }
        }
        
        Ok(json!({ "tree": tree }))
    }

    pub(crate) async fn list_mcp_ops(&self, _args: Value) -> Result<Value> {
        let mcp_status = self.mcp_registry.list_servers_status().await;
        Ok(json!(mcp_status))
    }

    pub(crate) async fn hex_dump(&self, args: Value) -> Result<Value> {
        let path_str = args["path"].as_str().ok_or_else(|| anyhow!("Missing path"))?;
        let offset = args.get("offset").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
        let length = args.get("length").and_then(|v| v.as_u64()).unwrap_or(256) as usize;

        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, path_str)?;
        
        use std::io::{Read, Seek, SeekFrom};
        let mut file = fs::File::open(full_path)?;
        file.seek(SeekFrom::Start(offset as u64))?;
        
        let mut buffer = vec![0u8; length];
        let bytes_read = file.read(&mut buffer)?;
        buffer.truncate(bytes_read);

        let mut dump = String::new();
        for (i, chunk) in buffer.chunks(16).enumerate() {
            let row_offset = offset + (i * 16);
            dump.push_str(&format!("{:08x}: ", row_offset));
            
            for b in chunk {
                dump.push_str(&format!("{:02x} ", b));
            }
            
            // Padding
            if chunk.len() < 16 {
                for _ in 0..(16 - chunk.len()) {
                    dump.push_str("   ");
                }
            }
            
            dump.push_str(" |");
            for b in chunk {
                if b.is_ascii_graphic() || *b == b' ' {
                    dump.push(*b as char);
                } else {
                    dump.push('.');
                }
            }
            dump.push_str("|\n");
        }

        Ok(json!({ "path": path_str, "dump": dump }))
    }

    pub(crate) async fn extract_strings(&self, args: Value) -> Result<Value> {
        let path_str = args["path"].as_str().ok_or_else(|| anyhow!("Missing path"))?;
        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, path_str)?;

        let bytes = safe_io::safe_read_bytes(&full_path)?;
        let mut strings = Vec::new();
        let mut current = Vec::new();
        
        for b in bytes {
            if b.is_ascii_graphic() || b == b' ' || b == b'\t' {
                current.push(b);
            } else {
                if current.len() >= 4 {
                    strings.push(String::from_utf8_lossy(&current).to_string());
                }
                current.clear();
            }
            if strings.len() > 500 { break; }
        }
        
        Ok(json!({ "path": path_str, "strings": strings }))
    }

    pub(crate) async fn list_active_processes(&self, _args: Value) -> Result<Value> {
        use sysinfo::System;
        let mut s = System::new_all();
        s.refresh_all();
        
        let mut processes = Vec::new();
        for (pid, process) in s.processes() {
            processes.push(json!({
                "pid": pid.to_string(),
                "name": process.name(),
                "memory_kb": process.memory()
            }));
            if processes.len() > 100 { break; }
        }
        
        Ok(json!(processes))
    }

    pub(crate) async fn get_file_metadata(&self, args: Value) -> Result<Value> {
        let path_str = args["path"].as_str().ok_or_else(|| anyhow!("Missing path"))?;
        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, path_str)?;
        
        let meta = fs::metadata(full_path)?;
        Ok(json!({
            "path": path_str,
            "size_bytes": meta.len(),
            "is_dir": meta.is_dir(),
            "is_file": meta.is_file(),
            "modified": format!("{:?}", meta.modified()?),
            "created": format!("{:?}", meta.created()?)
        }))
    }

    pub(crate) async fn apply_patch(&self, args: Value) -> Result<Value> {
        let path_str = args["path"].as_str().ok_or_else(|| anyhow!("Missing path"))?;
        let patch = args["patch"].as_str().ok_or_else(|| anyhow!("Missing patch"))?;
        let description = args["description"].as_str().unwrap_or("Applying surgical patch");

        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, path_str)?;

        let old_content = safe_io::safe_read(&full_path)?;

        // Actually apply the unified diff so the DiffViewer shows the real
        // patched result. SEARCH/REPLACE blocks belong to the surgical edit
        // tool; full rewrites belong to write_to_file.
        let parsed = diffy::Patch::from_str(patch).map_err(|e| {
            anyhow!(
                "apply_patch expects a unified diff (---/+++/@@ hunks); parse failed: {e}. \
                 Use the surgical SEARCH/REPLACE edit tool for block edits, or write_to_file for full content."
            )
        })?;
        let new_content = diffy::apply(&old_content, &parsed)
            .map_err(|e| anyhow!("patch does not apply cleanly to {path_str}: {e}"))?;

        self.emit_tool_event("propose-edit", json!({
            "path": path_str,
            "old_content": old_content,
            "new_content": new_content,
            "description": description
        }));

        Ok(json!({
            "status": "proposed",
            "info": "Unified diff applied; result proposed for review in the DiffViewer.",
            "bytes_before": old_content.len(),
            "bytes_after": new_content.len()
        }))
    }

    pub(crate) async fn ai_propose_edit(&self, args: Value) -> Result<Value> {
        let path_str = args["path"].as_str().ok_or_else(|| anyhow!("Missing path"))?;
        let new_content = args["new_content"].as_str().ok_or_else(|| anyhow!("Missing new_content"))?;
        let description = args["description"].as_str().unwrap_or("AI suggested modification");

        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, path_str)?;

        let old_content = safe_io::safe_read(&full_path).unwrap_or_default();

        self.emit_tool_event("propose-edit", json!({
            "path": path_str,
            "old_content": old_content,
            "new_content": new_content,
            "description": description
        }));

        Ok(json!({ "status": "proposed", "path": path_str }))
    }

    pub(crate) async fn ide_get_state(&self, _args: Value) -> Result<Value> {
        let state = self.editor_state().ok_or_else(|| anyhow!("EditorState unavailable"))?;
        let active_path = state.editor.active_path.lock().await.clone();
        let terminals = state.terminal.processes.lock().await.keys().cloned().collect::<Vec<String>>();
        
        Ok(json!({
            "active_path": active_path,
            "terminals": terminals,
            "project_root": self.root_path.lock().await.to_string_lossy()
        }))
    }

}

#[cfg(test)]
mod long_running_command_tests {
    use super::is_long_running_command;

    #[test]
    fn detects_scanners_in_various_shapes() {
        assert!(is_long_running_command("nmap -p- 10.0.0.5"));
        assert!(is_long_running_command("sudo nmap -sS target"));
        assert!(is_long_running_command("/usr/bin/ffuf -u https://x/FUZZ -w list"));
        assert!(is_long_running_command("proxychains4 sqlmap -u 'https://x?id=1'"));
        assert!(is_long_running_command("HTTPS_PROXY=x gobuster dir -u https://x"));
        assert!(is_long_running_command("cat urls.txt | httpx"));
        assert!(is_long_running_command("nmap.exe -A host"));
    }

    #[test]
    fn leaves_ordinary_commands_foreground() {
        for c in [
            "ls -la",
            "cat file.txt",
            "cargo check",
            "git status",
            "echo nmap",          // 'nmap' is an argument, not the binary
            "python calc.py",
            "grep foo bar.txt",
        ] {
            assert!(!is_long_running_command(c), "{c} should NOT auto-background");
        }
    }
}

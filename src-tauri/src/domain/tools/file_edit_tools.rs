//! File editing tools: replace, fast_apply, search_replace, shadow patch, patch_file.
use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use super::registry::AiTools;
use crate::domain::safe_io;

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
        let content = safe_io::safe_read(&full_path)?;
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
        let mut content = safe_io::safe_read(&full_path)?;
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
        let original = safe_io::safe_read(&full_path)?;
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
        let original = safe_io::safe_read(&full_path)?;

        // Strip markdown fences that models wrap around SEARCH/REPLACE blocks
        let content = strip_markdown_fences(content);

        let patches = crate::patch_engine::PatchEngine::parse_search_replace(&content);
        if patches.is_empty() {
            return Err(anyhow!("No SEARCH/REPLACE blocks found"));
        }

        let mut current = original.clone();
        for patch in &patches {
            let search_str = patch.search.trim();
            if search_str.is_empty() { continue; }
            let replace_str = patch.replace.trim();

            if let Some(new_text) = apply_edit_fallback(&current, search_str, replace_str) {
                current = new_text;
            } else {
                // Build helpful error with closest-match suggestion
                let suggestion = find_closest_match(&current, search_str);
                let msg = if let Some((line_num, preview)) = suggestion {
                    format!(
                        "SEARCH block not found in {path_str}. Closest match at line {line_num}: \"{preview}\""
                    )
                } else {
                    // Check if replacement is already applied
                    if current.contains(replace_str) {
                        format!("SEARCH block not found in {path_str}, but REPLACE content already exists — edit may already be applied.")
                    } else {
                        format!("SEARCH block not found in {path_str}")
                    }
                };
                return Err(anyhow!(msg));
            }
        }

        if !direct_apply {
            let shadow = self.shadow_workspace.mirror_file(&full_path.strip_prefix(&root).unwrap_or(&full_path))?;
            std::fs::write(&shadow, &current)?;
        } else {
            std::fs::write(&full_path, &current)?;
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
        let original = safe_io::safe_read(&full_path).unwrap_or_default();
        let modified = safe_io::safe_read(&shadow_path).unwrap_or_default();
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

        // Block destructive commands
        if let Some(reason) = crate::pentest_scope::block_localhost_pivot(command) {
            return Err(anyhow!(reason));
        }

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
        let original = safe_io::safe_read(&full_path)?;
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

// ── Aider-style search/replace fallback chain ──────────────────────────

/// Strip markdown code fences that models wrap around SEARCH/REPLACE blocks.
/// E.g. ```python ... ``` becomes just the content.
fn strip_markdown_fences(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut in_fence = false;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") && !in_fence {
            in_fence = true;
            continue;
        }
        if trimmed == "```" && in_fence {
            in_fence = false;
            continue;
        }
        if in_fence { continue; }
        result.push_str(line);
        result.push('\n');
    }
    result
}

/// Try applying an edit with cascading fallback strategies (aider-style).
/// Returns Some(new_text) on success, None if all strategies fail.
fn apply_edit_fallback(file_text: &str, search: &str, replace: &str) -> Option<String> {
    // Level 1: Exact match
    if let Some(pos) = file_text.find(search) {
        let mut result = String::with_capacity(file_text.len() - search.len() + replace.len());
        result.push_str(&file_text[..pos]);
        result.push_str(replace);
        result.push_str(&file_text[pos + search.len()..]);
        return Some(result);
    }

    let file_lines: Vec<&str> = file_text.lines().collect();
    let search_lines: Vec<&str> = search.lines().collect();

    // Level 2: Skip leading blank line (models sometimes add spurious blank lines)
    if let Some(first) = search_lines.first() {
        if first.trim().is_empty() && search_lines.len() > 1 {
            let trimmed_search = search_lines[1..].join("\n");
            if let Some(result) = try_whitespace_normalize(&file_text, &file_lines, &trimmed_search, replace) {
                return Some(result);
            }
        }
    }

    // Level 3: Whitespace normalization — strip common leading whitespace
    if let Some(result) = try_whitespace_normalize(&file_text, &file_lines, search, replace) {
        return Some(result);
    }

    // Level 4: `...` elision — split search on `...` lines, match each chunk
    if search.contains("...") {
        if let Some(result) = try_elision_match(&file_text, &file_lines, search, replace) {
            return Some(result);
        }
    }

    // Level 5: Fuzzy line matching — find best sliding window match
    if let Some(result) = try_fuzzy_match(&file_lines, search, replace) {
        return Some(result);
    }

    None
}

/// Level 2+3: Whitespace normalization match.
/// Strips common leading whitespace from search lines and matches against
/// the file with normalized whitespace.
fn try_whitespace_normalize(_file_text: &str, file_lines: &[&str], search: &str, replace: &str) -> Option<String> {
    let search_lines: Vec<&str> = search.lines().collect();
    if search_lines.is_empty() { return None; }

    // Find common minimum leading whitespace in search
    let min_indent = search_lines.iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.len() - l.trim_start().len())
        .min()
        .unwrap_or(0);

    // Strip that common indent from search lines
    let normalized_search: Vec<&str> = search_lines.iter()
        .map(|l| {
            if l.trim().is_empty() { *l }
            else if l.len() >= min_indent { &l[min_indent..] }
            else { l.trim_start() }
        })
        .collect();

    // Build a normalized version of the file for matching
    let normalized_file_lines: Vec<String> = file_lines.iter()
        .map(|l| {
            let trimmed = l.trim();
            if trimmed.is_empty() { (*l).to_string() }
            else {
                // Keep original indent but normalize the content
                let orig_indent = l.len() - l.trim_start().len();
                format!("{}{}", " ".repeat(orig_indent), trimmed)
            }
        })
        .collect();

    // Try to find the search block in the normalized file
    let search_len = normalized_search.len();
    for i in 0..=normalized_file_lines.len().saturating_sub(search_len) {
        let window: Vec<&str> = normalized_file_lines[i..i + search_len].iter()
            .map(|s| s.as_str())
            .collect();

        // Compare with normalized search (ignore leading whitespace differences)
        let matches = window.len() == normalized_search.len()
            && window.iter().zip(normalized_search.iter()).all(|(w, s)| {
                w.trim() == s.trim()
            });

        if matches {
            // Found match — preserve original file indentation
            let replace_lines: Vec<&str> = replace.lines().collect();
            let original_indent: Vec<usize> = file_lines[i..i + search_len].iter()
                .map(|l| l.len() - l.trim_start().len())
                .collect();

            let mut result = String::new();
            // Lines before the match
            for line in &file_lines[..i] {
                result.push_str(line);
                result.push('\n');
            }
            // Replacement with original indentation
            for (j, rline) in replace_lines.iter().enumerate() {
                let indent = original_indent.get(j).copied().unwrap_or(0);
                let trimmed = rline.trim();
                if trimmed.is_empty() {
                    result.push('\n');
                } else {
                    result.push_str(&" ".repeat(indent));
                    result.push_str(trimmed);
                    result.push('\n');
                }
            }
            // Lines after the match
            for line in &file_lines[i + search_len..] {
                result.push_str(line);
                result.push('\n');
            }
            return Some(result);
        }
    }
    None
}

/// Level 4: `...` elision — search contains `...` lines that skip unchanged regions.
fn try_elision_match(file_text: &str, file_lines: &[&str], search: &str, replace: &str) -> Option<String> {
    let chunks: Vec<&str> = search.split("\n...\n").collect();
    if chunks.len() < 2 { return None; }

    // Find the first chunk in the file
    let first_chunk = chunks[0].trim();
    if first_chunk.is_empty() { return None; }

    let start_pos = file_text.find(first_chunk)?;
    let search_start = file_text[..start_pos].lines().count();

    // Try to match all chunks sequentially from start_pos
    let mut file_pos = search_start;
    let mut all_matched = true;

    for chunk in &chunks {
        let chunk_lines: Vec<&str> = chunk.lines().filter(|l| !l.trim().is_empty()).collect();
        if chunk_lines.is_empty() { continue; }

        let chunk_len = chunk_lines.len();
        if file_pos + chunk_len > file_lines.len() {
            all_matched = false;
            break;
        }

        let window = &file_lines[file_pos..file_pos + chunk_len];
        let matches = window.iter().zip(chunk_lines.iter()).all(|(w, c)| w.trim() == c.trim());

        if !matches {
            all_matched = false;
            break;
        }
        file_pos += chunk_len;
    }

    if !all_matched { return None; }

    // Replace the entire matched region with the replacement
    let replace_lines: Vec<&str> = replace.lines().collect();
    let mut result = String::new();

    // Lines before
    for line in &file_lines[..search_start] {
        result.push_str(line);
        result.push('\n');
    }
    // Replacement
    for line in &replace_lines {
        result.push_str(line);
        result.push('\n');
    }
    // Lines after
    for line in &file_lines[file_pos..] {
        result.push_str(line);
        result.push('\n');
    }
    Some(result)
}

/// Level 5: Fuzzy line matching — sliding window with similarity scoring.
fn try_fuzzy_match(file_lines: &[&str], search: &str, replace: &str) -> Option<String> {
    let search_lines: Vec<&str> = search.lines().collect();
    let search_len = search_lines.len();
    if search_len == 0 { return None; }

    let file_len = file_lines.len();
    let _window_size = (search_len as f64 * 1.2).ceil() as usize; // ±20% window
    let min_window = search_len.saturating_sub(2).max(1);
    let max_window = (search_len + 2).min(file_len);

    let mut best_score = 0.0f64;
    let mut best_start = 0;
    let mut best_end = 0;

    for window_size in min_window..=max_window {
        for i in 0..=file_len.saturating_sub(window_size) {
            let window = &file_lines[i..i + window_size];
            let score = line_similarity(window, &search_lines);
            if score > best_score && score >= 0.7 {
                best_score = score;
                best_start = i;
                best_end = i + window_size;
            }
        }
    }

    if best_score < 0.7 { return None; }

    let replace_lines: Vec<&str> = replace.lines().collect();
    let mut result = String::new();

    for line in &file_lines[..best_start] {
        result.push_str(line);
        result.push('\n');
    }
    for line in &replace_lines {
        result.push_str(line);
        result.push('\n');
    }
    for line in &file_lines[best_end..] {
        result.push_str(line);
        result.push('\n');
    }
    Some(result)
}

/// Compute similarity between two line slices (Jaccard-like on trimmed lines).
fn line_similarity(a: &[&str], b: &[&str]) -> f64 {
    let a_trimmed: Vec<&str> = a.iter().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();
    let b_trimmed: Vec<&str> = b.iter().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();

    if a_trimmed.is_empty() || b_trimmed.is_empty() { return 0.0; }

    let a_set: std::collections::HashSet<&str> = a_trimmed.iter().copied().collect();
    let b_set: std::collections::HashSet<&str> = b_trimmed.iter().copied().collect();
    let intersection = a_set.intersection(&b_set).count();
    let union = a_set.union(&b_set).count();

    if union == 0 { 0.0 } else { intersection as f64 / union as f64 }
}

/// Find the closest matching region in the file for error reporting.
fn find_closest_match(file_text: &str, search: &str) -> Option<(usize, String)> {
    let search_lines: Vec<&str> = search.lines().filter(|l| !l.trim().is_empty()).collect();
    if search_lines.is_empty() { return None; }

    let file_lines: Vec<&str> = file_text.lines().collect();
    let search_len = search_lines.len();
    let mut best_score = 0.0f64;
    let mut best_line = 0;

    for i in 0..=file_lines.len().saturating_sub(search_len) {
        let window = &file_lines[i..i + search_len];
        let score = line_similarity(window, &search_lines);
        if score > best_score {
            best_score = score;
            best_line = i + 1; // 1-indexed
        }
    }

    if best_score < 0.3 { return None; }

    let preview = file_lines.get(best_line - 1)
        .unwrap_or(&"")
        .chars().take(80).collect::<String>();
    Some((best_line, preview))
}

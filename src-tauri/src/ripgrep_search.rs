//! Shared ripgrep (`rg`) search — used by agent `grep` tool, `grep_files`, and
//! shell-grep interceptors. Falls back to ignore+regex when `rg` is not on PATH.

use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Deserialize;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchResult {
    pub path: String,
    pub line: usize,
    pub content: String,
}

pub struct RipgrepQuery<'a> {
    pub pattern: &'a str,
    pub root: &'a Path,
    pub include: Option<&'a str>,
    pub max_results: usize,
    pub case_insensitive: bool,
    /// Literal substring match (rg -F) instead of regex.
    pub fixed_string: bool,
    /// When set, search only this file (path may be relative to `root`).
    pub file: Option<&'a Path>,
}

/// Resolve the `rg` binary — bundled IDE copy first, then system PATH.
pub fn resolve_rg_binary() -> Option<PathBuf> {
    crate::ide_shell::resolve_rg_exe()
}

pub fn ripgrep_search(q: RipgrepQuery<'_>) -> Result<Vec<SearchResult>, String> {
    let pattern = q.pattern.trim();
    if pattern.is_empty() {
        return Err("empty search pattern".into());
    }
    let max = q.max_results.max(1).min(5000);

    if let Some(rg) = resolve_rg_binary() {
        match run_rg_cli(&rg, &q, max) {
            Ok(hits) => return Ok(hits),
            Err(e) => eprintln!("[ripgrep] rg failed ({e}), using fallback walker"),
        }
    }

    fallback_walk(q, max)
}

fn run_rg_cli(rg: &Path, q: &RipgrepQuery<'_>, max: usize) -> Result<Vec<SearchResult>, String> {
    let mut cmd = Command::new(rg);
    cmd.args([
        "--json",
        "--no-heading",
        "--color=never",
        "--max-count",
        &max.to_string(),
    ]);
    if q.case_insensitive {
        cmd.arg("-i");
    }
    if q.fixed_string {
        cmd.arg("-F");
    }
    if let Some(g) = q.include.filter(|s| !s.is_empty()) {
        cmd.args(["-g", g]);
    }
    cmd.arg(q.pattern);

    let search_root = if let Some(f) = q.file {
        if f.is_absolute() {
            cmd.arg(f);
            q.root
        } else {
            let full = q.root.join(f);
            cmd.arg(&full);
            q.root
        }
    } else if q.root.is_file() {
        cmd.arg(q.root);
        q.root.parent().unwrap_or(q.root)
    } else {
        cmd.arg(q.root);
        q.root
    };

    cmd.current_dir(search_root);

    let output = cmd
        .output()
        .map_err(|e| format!("failed to spawn rg: {e}"))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut results = Vec::new();

    for line in stdout.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let Ok(msg) = serde_json::from_str::<RgJsonLine>(line) else {
            continue;
        };
        if msg.kind != "match" {
            continue;
        }
        let Some(data) = msg.data else {
            continue;
        };
        let path = data
            .path
            .and_then(|p| p.text)
            .unwrap_or_default();
        let line_no = data.line_number.unwrap_or(0) as usize;
        let content = data
            .lines
            .and_then(|l| l.text)
            .unwrap_or_default()
            .trim()
            .to_string();
        if path.is_empty() || line_no == 0 {
            continue;
        }
        results.push(SearchResult {
            path,
            line: line_no,
            content,
        });
        if results.len() >= max {
            break;
        }
    }

    Ok(results)
}

#[derive(Debug, Deserialize)]
struct RgJsonLine {
    #[serde(rename = "type")]
    kind: String,
    data: Option<RgMatchData>,
}

#[derive(Debug, Deserialize)]
struct RgMatchData {
    path: Option<RgText>,
    lines: Option<RgText>,
    line_number: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct RgText {
    text: Option<String>,
}

fn fallback_walk(q: RipgrepQuery<'_>, max: usize) -> Result<Vec<SearchResult>, String> {
    let re = if q.fixed_string {
        regex::RegexBuilder::new(&regex::escape(q.pattern))
            .case_insensitive(q.case_insensitive)
            .build()
            .map_err(|e| format!("invalid pattern: {e}"))?
    } else {
        regex::RegexBuilder::new(q.pattern)
            .case_insensitive(q.case_insensitive)
            .build()
            .map_err(|e| format!("invalid regex pattern: {e}"))?
    };

    let mut results = Vec::new();
    let walk_root = if let Some(f) = q.file {
        q.root.join(f)
    } else {
        q.root.to_path_buf()
    };

    if walk_root.is_file() {
        scan_file(&walk_root, q.root, &re, &mut results, max);
        return Ok(results);
    }

    let walker = ignore::WalkBuilder::new(&walk_root)
        .standard_filters(true)
        .max_depth(Some(25))
        .build();

    for entry in walker.flatten() {
        if !entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
            continue;
        }
        scan_file(entry.path(), q.root, &re, &mut results, max);
        if results.len() >= max {
            break;
        }
    }

    Ok(results)
}

fn scan_file(
    path: &Path,
    root: &Path,
    re: &regex::Regex,
    results: &mut Vec<SearchResult>,
    max: usize,
) {
    let Ok(content) = std::fs::read_to_string(path) else {
        return;
    };
    if content.len() > 5_000_000 {
        return;
    }
    let rel = path
        .strip_prefix(root)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| path.to_string_lossy().to_string());
    for (i, line) in content.lines().enumerate() {
        if re.is_match(line) {
            results.push(SearchResult {
                path: rel.clone(),
                line: i + 1,
                content: line.trim().to_string(),
            });
            if results.len() >= max {
                return;
            }
        }
    }
}

/// Format hits as the string the agent loop expects from `grep`.
pub fn format_grep_results(hits: &[SearchResult]) -> String {
    hits.iter()
        .map(|h| format!("{}:{}: {}", h.path, h.line, h.content))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Parse `grep -E?o?i? ... PATTERN FILE` or `rg ... PATTERN FILE` into tool args.
/// Only intercepts when grep/rg is the sole command (no curl/python pipelines).
pub fn try_parse_shell_grep(cmd: &str) -> Option<serde_json::Value> {
    let substantive: Vec<&str> = cmd
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .collect();

    if substantive.len() != 1 {
        return None;
    }

    let line = substantive[0];

    let lower = line.to_lowercase();
    let (rest, is_rg) = if lower.starts_with("rg ") {
        (&line[3..], true)
    } else if lower.starts_with("grep ") {
        (&line[5..], false)
    } else if lower.starts_with("egrep ") {
        (&line[6..], false)
    } else if lower.starts_with("fgrep ") {
        (&line[6..], false)
    } else {
        return None;
    };

    // Skip pipes — only intercept simple single-command searches
    if rest.contains('|') || rest.contains("<<") {
        return None;
    }

    let parts = shell_split(rest);
    if parts.is_empty() {
        return None;
    }

    let mut pattern: Option<String> = None;
    let mut path: Option<String> = None;
    let mut include: Option<String> = None;
    let mut i = 0;
    while i < parts.len() {
        let p = &parts[i];
        if p.starts_with('-') {
            if p == "-e" || p == "--regexp" {
                i += 1;
                pattern = parts.get(i).cloned();
            } else if p == "-f" || p == "--file" {
                return None;
            } else if p == "-g" || p == "--glob" {
                i += 1;
                include = parts.get(i).cloned();
            }
            i += 1;
            continue;
        }
        if pattern.is_none() {
            pattern = Some(p.clone());
        } else {
            path = Some(p.clone());
        }
        i += 1;
    }

    let pattern = pattern?;
    let mut args = serde_json::json!({
        "query": pattern,
        "pattern": pattern,
    });
    if let Some(p) = path {
        args["path"] = serde_json::json!(p);
    }
    if let Some(g) = include {
        args["include"] = serde_json::json!(g);
    }
    if is_rg {
        args["_intercepted_from"] = serde_json::json!("rg");
    } else {
        args["_intercepted_from"] = serde_json::json!("grep");
    }
    Some(args)
}

fn shell_split(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_single = false;
    let mut in_double = false;
    let mut esc = false;
    for ch in s.chars() {
        if esc {
            cur.push(ch);
            esc = false;
            continue;
        }
        if ch == '\\' && in_double {
            esc = true;
            continue;
        }
        if ch == '\'' && !in_double {
            in_single = !in_single;
            continue;
        }
        if ch == '"' && !in_single {
            in_double = !in_double;
            continue;
        }
        if ch.is_whitespace() && !in_single && !in_double {
            if !cur.is_empty() {
                out.push(std::mem::take(&mut cur));
            }
            continue;
        }
        cur.push(ch);
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_grep_simple() {
        let v = try_parse_shell_grep("grep -Eo \"api\" index_bundle.js").unwrap();
        assert_eq!(v["query"], "api");
        assert_eq!(v["path"], "index_bundle.js");
    }

    #[test]
    fn parse_rg_simple() {
        let v = try_parse_shell_grep("rg -n \"TODO\" src/").unwrap();
        assert_eq!(v["query"], "TODO");
        assert_eq!(v["path"], "src/");
    }

    #[test]
    fn skips_piped_grep() {
        assert!(try_parse_shell_grep("curl -s url | grep api").is_none());
    }
}

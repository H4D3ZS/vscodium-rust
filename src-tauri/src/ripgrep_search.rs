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

/// When a shell command is `curl … && rg …`, run the prefix in shell first, then ripgrep.
pub struct ShellGrepIntercept {
    pub prefix: Option<String>,
    pub args: serde_json::Value,
}

/// Parse `grep`/`rg` in a bash command into `grep` tool args.
/// Handles `curl > f && rg PATTERN f`, multi-line scripts (last grep line), and combined flags.
pub fn try_intercept_shell_grep(cmd: &str) -> Option<ShellGrepIntercept> {
    let normalized = normalize_shell_command(cmd);
    if normalized.is_empty() {
        return None;
    }

    for sep in ["&&", "||", ";"] {
        if let Some(idx) = normalized.rfind(sep) {
            let prefix = normalized[..idx].trim().trim_end_matches('\\').trim();
            let tail = normalized[idx + sep.len()..].trim();
            if !prefix.is_empty() {
                if let Some(args) = try_parse_grep_segment(tail) {
                    return Some(ShellGrepIntercept {
                        prefix: Some(prefix.to_string()),
                        args,
                    });
                }
            }
        }
    }

    let lines: Vec<&str> = cmd
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .collect();
    if lines.len() > 1 {
        if let Some(args) = try_parse_grep_segment(lines.last()?) {
            let prefix = lines[..lines.len() - 1].join("\n");
            return Some(ShellGrepIntercept {
                prefix: Some(prefix),
                args,
            });
        }
    }

    try_parse_grep_segment(&normalized).map(|args| ShellGrepIntercept {
        prefix: None,
        args,
    })
}

/// Back-compat: simple grep-only commands (no compound prefix).
pub fn try_parse_shell_grep(cmd: &str) -> Option<serde_json::Value> {
    try_intercept_shell_grep(cmd)
        .filter(|i| i.prefix.is_none())
        .map(|i| i.args)
}

/// True when the command invokes shell grep/rg but we cannot safely intercept it.
pub fn command_uses_shell_grep(cmd: &str) -> bool {
    let n = normalize_shell_command(cmd).to_lowercase();
    if n.is_empty() {
        return false;
    }
    n.starts_with("grep ")
        || n.starts_with("egrep ")
        || n.starts_with("fgrep ")
        || n.starts_with("rg ")
        || n.contains(" grep ")
        || n.contains(" egrep ")
        || n.contains(" fgrep ")
        || n.contains(" rg ")
}

fn normalize_shell_command(cmd: &str) -> String {
    cmd.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|l| l.trim_end_matches('\\').trim())
        .collect::<Vec<_>>()
        .join("\n")
}

fn try_parse_grep_segment(segment: &str) -> Option<serde_json::Value> {
    let grep_stage = segment.split('|').next()?.trim();
    if grep_stage.contains("<<") {
        return None;
    }

    let lower = grep_stage.to_lowercase();
    let (rest, is_rg) = if lower.starts_with("rg ") {
        (&grep_stage[3..], true)
    } else if lower.starts_with("grep ") {
        (&grep_stage[5..], false)
    } else if lower.starts_with("egrep ") {
        (&grep_stage[6..], false)
    } else if lower.starts_with("fgrep ") {
        (&grep_stage[6..], false)
    } else {
        return None;
    };

    let parts = shell_split(rest);
    if parts.is_empty() {
        return None;
    }

    let mut pattern: Option<String> = None;
    let mut path: Option<String> = None;
    let mut include: Option<String> = None;
    let mut case_insensitive = false;
    let mut fixed_string = false;

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
            } else if p == "-A" || p == "-B" || p == "-C" || p == "--context" {
                i += 2;
                continue;
            } else if p.starts_with("--context=") {
                i += 1;
                continue;
            } else {
                if p.contains('i') {
                    case_insensitive = true;
                }
                if p.contains('F') || p == "--fixed-strings" {
                    fixed_string = true;
                }
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
        "case_insensitive": case_insensitive,
        "fixed_string": fixed_string,
    });
    if let Some(p) = path {
        args["path"] = serde_json::json!(p);
    }
    if let Some(g) = include {
        args["include"] = serde_json::json!(g);
    }
    args["_intercepted_from"] = serde_json::json!(if is_rg { "rg" } else { "grep" });
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
    fn parse_grep_with_context_flag() {
        let v = try_parse_shell_grep("grep -A 20 \"url\" bundle.js").unwrap();
        assert_eq!(v["query"], "url");
        assert_eq!(v["path"], "bundle.js");
    }

    #[test]
    fn parse_compound_curl_and_rg() {
        let v = try_intercept_shell_grep(
            "curl -s https://example.com/a.js > index_bundle.js && rg -o '\"https://[^\"]+\"' index_bundle.js",
        )
        .unwrap();
        assert!(v.prefix.as_ref().unwrap().contains("curl"));
        assert_eq!(v.args["path"], "index_bundle.js");
    }

    #[test]
    fn skips_piped_grep() {
        assert!(try_parse_shell_grep("curl -s url | grep api").is_none());
    }
}

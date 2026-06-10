//! Scan minified JS bundles, webpack/vite chunks, and source maps for leaked secrets.

use crate::security_patterns::{bundle_patterns, resolve_url, script_src_re, source_map_re, severity_rank};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub const MAX_FILE_BYTES: u64 = 50 * 1024 * 1024;
const DEFAULT_MAX_FILES: usize = 2_000;

/// Bundle output dirs scanned first for faster bounty triage.
const PRIORITY_DIR_NAMES: &[&str] = &[
    "dist", "build", ".next", "out", "static", "public", "assets", "_next",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkSecretFinding {
    pub kind: String,
    pub severity: String,
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub snippet: String,
    pub redacted: String,
    pub bounty_hint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkScanSummary {
    pub files_scanned: usize,
    pub bytes_scanned: u64,
    pub findings: Vec<ChunkSecretFinding>,
    pub source_maps_found: usize,
    pub script_urls: Vec<String>,
}

fn redact(s: &str) -> String {
    let len = s.len();
    if len <= 10 {
        return "…(redacted)".into();
    }
    format!(
        "{}…{} ({} chars)",
        &s[..4.min(len)],
        &s[len.saturating_sub(4)..],
        len
    )
}

fn line_col(content: &str, byte_idx: usize) -> (usize, usize) {
    let before = &content[..byte_idx.min(content.len())];
    let line = before.matches('\n').count() + 1;
    let column = before
        .rfind('\n')
        .map(|i| byte_idx - i)
        .unwrap_or(byte_idx + 1);
    (line, column)
}

fn snippet_around(content: &str, start: usize, end: usize) -> String {
    let lo = start.saturating_sub(60);
    let hi = (end + 60).min(content.len());
    content[lo..hi].replace('\n', " ")
}

pub fn scan_content(file_label: &str, content: &str) -> Vec<ChunkSecretFinding> {
    let mut out = Vec::new();
    for p in bundle_patterns() {
        for m in p.re.find_iter(content) {
            let (line, column) = line_col(content, m.start());
            let matched = m.as_str();
            out.push(ChunkSecretFinding {
                kind: p.kind.to_string(),
                severity: p.severity.to_string(),
                file: file_label.to_string(),
                line,
                column,
                snippet: snippet_around(content, m.start(), m.end()),
                redacted: redact(matched),
                bounty_hint: p.bounty_hint.to_string(),
            });
        }
    }
    out
}

fn is_js_like(path: &Path) -> bool {
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_lowercase();
    name.ends_with(".js")
        || name.ends_with(".mjs")
        || name.ends_with(".cjs")
        || name.ends_with(".js.map")
        || name.contains(".chunk.")
        || name.contains(".bundle.")
}

fn skip_dir(name: &str) -> bool {
    matches!(
        name,
        "node_modules" | ".git" | "target" | "dist-electron" | "coverage" | ".turbo"
    )
}

fn path_priority(path: &Path) -> u8 {
    for (i, seg) in path.components().map(|c| c.as_os_str().to_string_lossy()).enumerate() {
        if PRIORITY_DIR_NAMES.contains(&seg.as_ref()) {
            return i as u8;
        }
    }
    u8::MAX / 2
}

fn scan_file(path: &Path) -> Option<(u64, Vec<ChunkSecretFinding>, bool)> {
    let meta = std::fs::metadata(path).ok()?;
    if meta.len() > MAX_FILE_BYTES {
        return None;
    }
    let content = std::fs::read_to_string(path).ok()?;
    let bytes = content.len() as u64;
    let label = path.to_string_lossy().to_string();
    let findings = scan_content(&label, &content);
    let is_map = path.extension().and_then(|e| e.to_str()) == Some("map")
        || label.ends_with(".js.map");
    Some((bytes, findings, is_map))
}

pub fn scan_directory(root: &Path, max_files: usize) -> Result<ChunkScanSummary, String> {
    if !root.is_dir() {
        return Err(format!("Not a directory: {}", root.display()));
    }

    let mut paths: Vec<PathBuf> = WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && is_js_like(e.path()))
        .filter(|e| {
            !e.path().components().any(|c| {
                c.as_os_str()
                    .to_str()
                    .map(|n| skip_dir(n))
                    .unwrap_or(false)
            })
        })
        .map(|e| e.into_path())
        .collect();

    paths.sort_by_key(|p| (path_priority(p), p.to_string_lossy().len()));
    paths.truncate(max_files.max(1));

    let results: Vec<_> = paths
        .par_iter()
        .filter_map(|p| scan_file(p).map(|r| (p.clone(), r)))
        .collect();

    let mut files_scanned = 0usize;
    let mut bytes_scanned = 0u64;
    let mut findings = Vec::new();
    let mut source_maps_found = 0usize;

    for (_path, (bytes, mut file_findings, is_map)) in results {
        files_scanned += 1;
        bytes_scanned += bytes;
        if is_map {
            source_maps_found += 1;
        }
        findings.append(&mut file_findings);
    }

    dedupe_findings(&mut findings);

    Ok(ChunkScanSummary {
        files_scanned,
        bytes_scanned,
        findings,
        source_maps_found,
        script_urls: vec![],
    })
}

pub fn dedupe_findings(findings: &mut Vec<ChunkSecretFinding>) {
    let mut seen = HashSet::new();
    findings.retain(|f| seen.insert((f.kind.clone(), f.file.clone(), f.line, f.column)));
    findings.sort_by(|a, b| {
        severity_rank(&a.severity)
            .cmp(&severity_rank(&b.severity))
            .then(a.file.cmp(&b.file))
    });
}

pub async fn scan_url(origin_url: &str, client: &reqwest::Client) -> Result<ChunkScanSummary, String> {
    let resp = client
        .get(origin_url)
        .header("User-Agent", "HADES-ChunkSecretScanner/1.0")
        .send()
        .await
        .map_err(|e| format!("fetch {origin_url}: {e}"))?;
    let html = resp.text().await.map_err(|e| e.to_string())?;

    let script_urls: Vec<String> = script_src_re()
        .captures_iter(&html)
        .filter_map(|c| {
            let rel = c.get(1)?.as_str().to_string();
            Some(resolve_url(origin_url, &rel))
        })
        .collect();

    let mut files_scanned = 1usize;
    let mut bytes_scanned = html.len() as u64;
    let mut findings = scan_content(origin_url, &html);
    let mut pending: Vec<String> = script_urls.iter().take(64).cloned().collect();
    let mut fetched = HashSet::new();

    while !pending.is_empty() && files_scanned < 80 {
        let batch: Vec<_> = pending
            .drain(..pending.len().min(12))
            .filter(|u| fetched.insert(u.clone()))
            .collect();

        let tasks: Vec<_> = batch
            .into_iter()
            .map(|url| {
                let c = client.clone();
                async move {
                    let body = c
                        .get(&url)
                        .header("User-Agent", "HADES-ChunkSecretScanner/1.0")
                        .send()
                        .await
                        .ok()?
                        .text()
                        .await
                        .ok()?;
                    Some((url, body))
                }
            })
            .collect();

        for item in futures::future::join_all(tasks).await.into_iter().flatten() {
            let (url, body) = item;
            if body.len() as u64 > MAX_FILE_BYTES {
                continue;
            }
            bytes_scanned += body.len() as u64;
            files_scanned += 1;
            findings.extend(scan_content(&url, &body));

            if let Some(cap) = source_map_re()
                .captures(&body)
                .and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
            {
                let map_url = resolve_url(&url, &cap);
                if fetched.insert(map_url.clone()) {
                    pending.push(map_url);
                }
            }
        }
    }

    let source_maps_found = findings.iter().filter(|f| f.kind == "source_map_url").count();
    dedupe_findings(&mut findings);

    Ok(ChunkScanSummary {
        files_scanned,
        bytes_scanned,
        findings,
        source_maps_found,
        script_urls,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_openai_key_in_chunk() {
        let js = r#"(()=>{const e={key:"sk-proj-abcdefghijklmnopqrstuvwxyz123456"}})();"#;
        let f = scan_content("main.abc123.chunk.js", js);
        assert!(f.iter().any(|x| x.kind == "openai_api_key"));
    }

    #[test]
    fn detects_vite_env_leak() {
        let js = r#"VITE_OPENAI_API_KEY:"sk-abc12345678901234567890""#;
        let f = scan_content("index.js", js);
        assert!(f.iter().any(|x| x.kind == "vite_env_leak" || x.kind == "openai_api_key"));
    }

    #[test]
    fn detects_source_map_comment() {
        let js = "//# sourceMappingURL=app.bundle.js.map\n";
        let f = scan_content("app.js", js);
        assert!(f.iter().any(|x| x.kind == "source_map_url"));
    }

    #[test]
    fn priority_dirs_sort_first() {
        let a = PathBuf::from("/proj/dist/main.js");
        let b = PathBuf::from("/proj/src/foo.js");
        assert!(path_priority(&a) < path_priority(&b));
    }
}

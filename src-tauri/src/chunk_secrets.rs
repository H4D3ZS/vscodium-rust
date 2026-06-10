//! Scan minified JS bundles, webpack/vite chunks, and source maps for leaked secrets,
//! `.env` values, and AI provider keys embedded at build time.

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::Path;

const MAX_FILE_BYTES: u64 = 50 * 1024 * 1024;
const DEFAULT_MAX_FILES: usize = 2_000;

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

struct Pattern {
    kind: &'static str,
    severity: &'static str,
    bounty_hint: &'static str,
    re: Regex,
}

fn patterns() -> Vec<Pattern> {
    let raw: Vec<(&str, &str, &str, &str)> = vec![
        (
            "openai_api_key",
            "CRITICAL",
            "OpenAI billing abuse / GPT-4 access — report as exposed API key in client bundle.",
            r#"\bsk-(?:proj-)?[A-Za-z0-9_\-]{20,}\b"#,
        ),
        (
            "anthropic_api_key",
            "CRITICAL",
            "Anthropic Claude API abuse — rotate key; check for prompt-injection pivot.",
            r#"\bsk-ant-[A-Za-z0-9\-_]{20,}\b"#,
        ),
        (
            "google_ai_key",
            "HIGH",
            "Google AI / Gemini key in frontend — quota theft and data exfil risk.",
            r#"\bAIza[0-9A-Za-z\-_]{30,}\b"#,
        ),
        (
            "aws_access_key_id",
            "CRITICAL",
            "AWS AKIA in JS — immediate credential report; check S3/public bucket chain.",
            r#"\bAKIA[0-9A-Z]{16}\b"#,
        ),
        (
            "stripe_secret_key",
            "CRITICAL",
            "Stripe sk_live in bundle — payment fraud; high-severity instant bounty.",
            r#"\bsk_(?:live|test)_[A-Za-z0-9]{20,}\b"#,
        ),
        (
            "github_token",
            "CRITICAL",
            "GitHub PAT in client JS — repo/org takeover.",
            r#"\bgh[pousr]_[A-Za-z0-9]{20,}\b"#,
        ),
        (
            "firebase_config",
            "HIGH",
            "Firebase web config — test Firestore/Storage rules for public read/write.",
            r#"apiKey\s*:\s*['"][A-Za-z0-9_\-]{20,}['"]"#,
        ),
        (
            "supabase_anon",
            "HIGH",
            "Supabase anon key in bundle — verify RLS policies; JWT role escalation.",
            r#"supabase(?:Url|Key|AnonKey)\s*[:=]\s*['"][^'"]{10,}['"]"#,
        ),
        (
            "vite_env_leak",
            "HIGH",
            "VITE_* secret inlined at build — env vars must not ship to browser.",
            r#"VITE_(?:API|SECRET|KEY|TOKEN|OPENAI|ANTHROPIC)[A-Z0-9_]*['"]?\s*[:=]\s*['"][^'"]{8,}['"]"#,
        ),
        (
            "next_public_secret",
            "HIGH",
            "NEXT_PUBLIC_* with sensitive value — Next.js exposes these to all users.",
            r#"NEXT_PUBLIC_[A-Z0-9_]+['"]?\s*[:=]\s*['"][^'"]{12,}['"]"#,
        ),
        (
            "react_app_secret",
            "HIGH",
            "REACT_APP_* credential in CRA bundle.",
            r#"REACT_APP_[A-Z0-9_]+['"]?\s*[:=]\s*['"][^'"]{12,}['"]"#,
        ),
        (
            "process_env_literal",
            "MEDIUM",
            "process.env.* resolved into bundle — build misconfiguration.",
            r#"process\.env\.[A-Z0-9_]+\s*[,}\)]"#,
        ),
        (
            "dotenv_in_bundle",
            "CRITICAL",
            "Literal .env key=value inside JS — classic mis-bundle bounty finding.",
            r#"(?i)(?:DB_PASSWORD|DATABASE_URL|JWT_SECRET|ADMIN_PASSWORD|AWS_SECRET)[^=\n]{0,20}=\s*['"]?[^'"\s]{8,}"#,
        ),
        (
            "private_key_block",
            "CRITICAL",
            "Private key material in JS chunk — full compromise.",
            r#"-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----"#,
        ),
        (
            "sentry_dsn",
            "MEDIUM",
            "Sentry DSN — event injection / PII harvest if misconfigured.",
            r#"https?://[a-f0-9]{32}@[A-Za-z0-9\.\-]+/\d+"#,
        ),
        (
            "jwt_in_bundle",
            "HIGH",
            "Hardcoded JWT in client — decode for role/privilege escalation.",
            r#"\beyJ[A-Za-z0-9_\-]{10,}\.eyJ[A-Za-z0-9_\-]{10,}\.[A-Za-z0-9_\-]{10,}\b"#,
        ),
        (
            "generic_api_key",
            "MEDIUM",
            "Generic apiKey/accessToken assignment in minified code.",
            r#"(?i)(?:api[_-]?key|apikey|access[_-]?token|auth[_-]?token)\s*[:=]\s*['"]([A-Za-z0-9_\-/+=]{20,})['"]"#,
        ),
        (
            "source_map_url",
            "INFO",
            "sourceMappingURL present — fetch .map for original sources and more secrets.",
            r#"//# sourceMappingURL=([^\s'"]+\.map)"#,
        ),
    ];
    raw.into_iter()
        .filter_map(|(kind, sev, hint, pat)| {
            Regex::new(pat).ok().map(|re| Pattern {
                kind,
                severity: sev,
                bounty_hint: hint,
                re,
            })
        })
        .collect()
}

fn redact(s: &str) -> String {
    let len = s.len();
    if len <= 10 {
        return "…(redacted)".into();
    }
    format!("{}…{} ({} chars)", &s[..4.min(len)], &s[len.saturating_sub(4)..], len)
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
    let pats = patterns();
    let mut out = Vec::new();
    for p in &pats {
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
        "node_modules" | ".git" | "target" | "dist-electron" | ".next" | "coverage"
    )
}

pub fn scan_directory(root: &Path, max_files: usize) -> Result<ChunkScanSummary, String> {
    if !root.is_dir() {
        return Err(format!("Not a directory: {}", root.display()));
    }
    let mut files_scanned = 0usize;
    let mut bytes_scanned = 0u64;
    let mut findings = Vec::new();
    let mut source_maps_found = 0usize;
    let mut stack = vec![root.to_path_buf()];

    while let Some(dir) = stack.pop() {
        let entries = std::fs::read_dir(&dir).map_err(|e| format!("read_dir: {e}"))?;
        for ent in entries.flatten() {
            let path = ent.path();
            let ft = ent.file_type().map_err(|e| e.to_string())?;
            if ft.is_dir() {
                let n = ent.file_name().to_string_lossy().to_string();
                if !skip_dir(&n) {
                    stack.push(path);
                }
                continue;
            }
            if !is_js_like(&path) || files_scanned >= max_files {
                continue;
            }
            let meta = std::fs::metadata(&path).map_err(|e| e.to_string())?;
            if meta.len() > MAX_FILE_BYTES {
                continue;
            }
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            bytes_scanned += content.len() as u64;
            files_scanned += 1;
            if path.extension().and_then(|e| e.to_str()) == Some("map")
                || path.to_string_lossy().ends_with(".js.map")
            {
                source_maps_found += 1;
            }
            let label = path.to_string_lossy().to_string();
            findings.extend(scan_content(&label, &content));
        }
    }

    findings.sort_by(|a, b| {
        severity_rank(&a.severity)
            .cmp(&severity_rank(&b.severity))
            .then(a.file.cmp(&b.file))
    });

    Ok(ChunkScanSummary {
        files_scanned,
        bytes_scanned,
        findings,
        source_maps_found,
        script_urls: vec![],
    })
}

fn severity_rank(s: &str) -> u8 {
    match s {
        "CRITICAL" => 0,
        "HIGH" => 1,
        "MEDIUM" => 2,
        "LOW" => 3,
        _ => 4,
    }
}

fn extract_script_srcs(html: &str) -> Vec<String> {
    static RE: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    let re = RE.get_or_init(|| {
        Regex::new(r#"(?i)<script[^>]+src=['"]([^'"]+\.js[^'"]*)['"]"#).expect("script src re")
    });
    re.captures_iter(html)
        .filter_map(|c| c.get(1).map(|m| m.as_str().to_string()))
        .collect()
}

fn resolve_url(base: &str, rel: &str) -> String {
    if rel.starts_with("http://") || rel.starts_with("https://") {
        return rel.to_string();
    }
    if rel.starts_with("//") {
        return format!("https:{rel}");
    }
    let base = base.trim_end_matches('/');
    if rel.starts_with('/') {
        let origin = base.split('/').take(3).collect::<Vec<_>>().join("/");
        if !origin.is_empty() {
            return format!("{origin}{rel}");
        }
    }
    format!("{base}/{rel}")
}

pub async fn scan_url(origin_url: &str, client: &reqwest::Client) -> Result<ChunkScanSummary, String> {
    let resp = client
        .get(origin_url)
        .header("User-Agent", "HADES-ChunkSecretScanner/1.0")
        .send()
        .await
        .map_err(|e| format!("fetch {origin_url}: {e}"))?;
    let html = resp.text().await.map_err(|e| e.to_string())?;
    let script_urls: Vec<String> = extract_script_srcs(&html)
        .into_iter()
        .map(|s| resolve_url(origin_url, &s))
        .collect();

    let mut files_scanned = 0usize;
    let mut bytes_scanned = 0u64;
    let mut findings = scan_content(origin_url, &html);
    let mut source_maps_found = 0usize;

    for url in &script_urls {
        if files_scanned >= 64 {
            break;
        }
        let Ok(r) = client
            .get(url.as_str())
            .header("User-Agent", "HADES-ChunkSecretScanner/1.0")
            .send()
            .await
        else {
            continue;
        };
        let Ok(body) = r.text().await else { continue };
        if body.len() as u64 > MAX_FILE_BYTES {
            continue;
        }
        bytes_scanned += body.len() as u64;
        files_scanned += 1;
        findings.extend(scan_content(url, &body));

        if let Some(cap) = Regex::new(r#"//# sourceMappingURL=([^\s'"]+\.map)"#)
            .ok()
            .and_then(|re| re.captures(&body))
            .and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
        {
            let map_url = resolve_url(url, &cap);
            if let Ok(mr) = client.get(&map_url).send().await {
                if let Ok(map_body) = mr.text().await {
                    source_maps_found += 1;
                    files_scanned += 1;
                    bytes_scanned += map_body.len() as u64;
                    findings.extend(scan_content(&map_url, &map_body));
                }
            }
        }
    }

    findings.sort_by(|a, b| {
        severity_rank(&a.severity)
            .cmp(&severity_rank(&b.severity))
            .then(a.file.cmp(&b.file))
    });

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
}

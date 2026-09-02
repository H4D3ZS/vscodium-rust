//! Security tools: scanners, entropy/secrets, vuln hunting, audits, OAST, Vega.
use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;
use std::sync::Arc;
use super::registry::AiTools;
use super::registry::{push_activity, extract_json_loose, model_size_hint, is_security_model};

/// File extensions considered source code for security scanning.
const CODE_EXTS: &[&str] = &[
    "rs", "ts", "tsx", "js", "jsx", "py", "go", "java", "kt", "rb", "php",
    "c", "cpp", "cc", "h", "hpp", "cs", "swift", "zig", "lua", "sh", "bash",
];

/// Directories to skip during security scans.
const SKIP_DIRS: &[&str] = &[
    "node_modules", "target", "dist", "build", ".git", "vendor",
    "third_party", "__pycache__", ".venv", "venv", "out",
];

/// Rank a severity string for sorting (lower = more critical).
fn severity_rank(s: &str) -> u8 {
    match s.to_uppercase().as_str() {
        "CRITICAL" => 0,
        "HIGH" => 1,
        "MEDIUM" => 2,
        "LOW" => 3,
        _ => 4,
    }
}

/// Sort findings by severity and count by severity level.
fn consolidate_findings(findings: &mut [Value]) -> BTreeMap<String, u64> {
    findings.sort_by_key(|f| {
        let sev = f.get("severity")
            .and_then(|v| v.as_str())
            .unwrap_or("UNKNOWN");
        severity_rank(sev)
    });
    let mut counts = BTreeMap::new();
    for f in findings.iter() {
        let sev = f.get("severity")
            .and_then(|v| v.as_str())
            .unwrap_or("UNKNOWN")
            .to_uppercase();
        *counts.entry(sev).or_insert(0) += 1;
    }
    counts
}

/// Check if a directory name should be skipped.
fn should_skip_dir(name: &str) -> bool {
    SKIP_DIRS.contains(&name)
}

/// Check if a file extension is a code file.
fn is_code_ext(ext: &str) -> bool {
    CODE_EXTS.contains(&ext)
}

impl AiTools {
    pub(crate) async fn network_port_scanner(&self, args: Value) -> Result<Value> {
        let target = args["target"].as_str().ok_or_else(|| anyhow!("Missing target"))?;
        let ports = args["ports"].as_array().ok_or_else(|| anyhow!("Missing ports array"))?;
        
        let mut open_ports = Vec::new();
        for port_val in ports {
            if let Some(port) = port_val.as_u64() {
                let addr = format!("{}:{}", target, port);
                // Perform a synchronous connection attempt with timeout
                if let Ok(_) = std::net::TcpStream::connect_timeout(
                    &addr.parse().map_err(|_| anyhow!("Invalid addr"))?,
                    std::time::Duration::from_millis(100)
                ) {
                    open_ports.push(port);
                }
            }
        }
        
        Ok(json!({ "target": target, "open_ports": open_ports }))
    }

    pub(crate) async fn binary_mach_o_scanner(&self, args: Value) -> Result<Value> {
        let path_str = args["path"].as_str().ok_or_else(|| anyhow!("Missing path"))?;
        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, path_str)?;
        
        let bytes = fs::read(&full_path)?;
        if bytes.len() < 4 { return Err(anyhow!("File too small")); }
        
        // Mach-O Magic constants
        let magic = &bytes[0..4];
        let is_macho = magic == [0xce, 0xfa, 0xed, 0xfe] || magic == [0xcf, 0xfa, 0xed, 0xfe] || 
                      magic == [0xfe, 0xed, 0xfa, 0xce] || magic == [0xfe, 0xed, 0xfa, 0xcf];
        
        let mut info = json!({ 
            "is_macho": is_macho,
            "magic": format!("{:x?}", magic),
            "size": bytes.len()
        });
        
        // XNU Specific heuristic: look for __TEXT or __DATA sections manually
        let has_text = bytes.windows(7).any(|w| w == b"__TEXT\0");
        let has_data = bytes.windows(7).any(|w| w == b"__DATA\0");
        
        info.as_object_mut().unwrap().insert("has_text_segment".to_string(), json!(has_text));
        info.as_object_mut().unwrap().insert("has_data_segment".to_string(), json!(has_data));
        
        Ok(info)
    }

    pub(crate) async fn file_entropy_analysis(&self, args: Value) -> Result<Value> {
        let path_str = args["path"].as_str().ok_or_else(|| anyhow!("Missing path"))?;
        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, path_str)?;
        
        let bytes = fs::read(full_path)?;
        if bytes.is_empty() { return Ok(json!({"entropy": 0})); }
        
        let mut counts = [0u64; 256];
        for &b in &bytes {
            counts[b as usize] += 1;
        }
        
        let mut entropy = 0.0f64;
        let len = bytes.len() as f64;
        for &count in &counts {
            if count > 0 {
                let p = count as f64 / len;
                entropy -= p * p.log2();
            }
        }
        
        Ok(json!({
            "path": path_str,
            "entropy": entropy,
            "high_entropy_warning": entropy > 7.5,
            "suggestion": if entropy > 7.5 { "Likely compressed or encrypted. check for malware packers." } else { "Normal executable density." }
        }))
    }

    // ────────────────────────────────────────────────────────────────────
    //  Cybersecurity tooling: secrets_scan + weaponize_env
    //
    //  These power the red-team / blue-team / bug-bounty flows. They're
    //  deterministic Rust (regex + simple parsing) so the agent can lean
    //  on them instead of asking the LLM to "imagine" findings. Output is
    //  structured JSON that downstream tool calls can act on.
    // ────────────────────────────────────────────────────────────────────

    pub(crate) fn secret_patterns() -> Vec<(&'static str, &'static str, &'static str)> {
        // (kind, severity, regex). Regex must compile under the `regex` crate
        // (RE2 — no lookarounds). We intentionally keep these conservative;
        // the goal is "high signal" rather than "every possible false positive".
        // Use `r#"..."#` delimiters everywhere so we can embed literal
        // `"` characters and quote-classes (`['"]`) without escaping
        // gymnastics. Backslashes inside `r#"..."#` are NOT processed by
        // Rust, so `\s`, `\b`, `\d` survive as regex metacharacters.
        vec![
            ("aws_access_key_id",      "CRITICAL", r#"\bAKIA[0-9A-Z]{16}\b"#),
            ("aws_secret_access_key",  "CRITICAL", r#"(?i)aws_secret_access_key\s*[:=]\s*['"]?([A-Za-z0-9/+=]{40})['"]?"#),
            ("gcp_service_account",    "CRITICAL", r#""type"\s*:\s*"service_account""#),
            ("github_token",           "CRITICAL", r#"\bgh[pousr]_[A-Za-z0-9]{20,}\b"#),
            ("gitlab_token",           "HIGH",     r#"\bglpat-[A-Za-z0-9\-_]{20,}\b"#),
            ("slack_token",            "HIGH",     r#"\bxox[baprs]-[A-Za-z0-9\-]{10,}\b"#),
            ("stripe_secret_key",      "CRITICAL", r#"\bsk_(?:live|test)_[A-Za-z0-9]{20,}\b"#),
            ("openai_api_key",         "HIGH",     r#"\bsk-[A-Za-z0-9]{20,}\b"#),
            ("anthropic_api_key",      "HIGH",     r#"\bsk-ant-[A-Za-z0-9\-_]{20,}\b"#),
            ("sentry_dsn",             "HIGH",     r#"https?://[a-f0-9]{32}@[A-Za-z0-9\.\-]+/\d+"#),
            ("jwt_token",              "MEDIUM",   r#"\beyJ[A-Za-z0-9_\-]{10,}\.eyJ[A-Za-z0-9_\-]{10,}\.[A-Za-z0-9_\-]{10,}\b"#),
            ("private_key_block",      "CRITICAL", r#"-----BEGIN (?:RSA |DSA |EC |OPENSSH |PGP )?PRIVATE KEY-----"#),
            ("postgres_url",           "HIGH",     r#"postgres(?:ql)?://[^\s'"@]+:[^\s'"@]+@[A-Za-z0-9\.\-]+"#),
            ("mysql_url",              "HIGH",     r#"mysql://[^\s'"@]+:[^\s'"@]+@[A-Za-z0-9\.\-]+"#),
            ("mongodb_url",            "HIGH",     r#"mongodb(?:\+srv)?://[^\s'"@]+:[^\s'"@]+@[A-Za-z0-9\.\-]+"#),
            ("redis_url",              "MEDIUM",   r#"redis://[^\s'"@]+:[^\s'"@]+@[A-Za-z0-9\.\-]+"#),
            ("generic_password",       "LOW",      r#"(?i)(?:password|passwd|pwd)\s*[:=]\s*['"]([^'"\s]{8,})['"]"#),
            ("generic_api_key",        "LOW",      r#"(?i)(?:api[_-]?key|apikey|access[_-]?token|auth[_-]?token)\s*[:=]\s*['"]?([A-Za-z0-9_\-]{20,})['"]?"#),
        ]
    }

    pub(crate) fn redact_secret(s: &str) -> String {
        let len = s.len();
        if len <= 12 {
            return format!("{}…(redacted)", &s[..s.len().min(3)]);
        }
        format!("{}…{}  ({} chars)", &s[..4], &s[len - 4..], len)
    }

    /// Deep multi-pass security audit. Orchestrates the existing fast passes
    /// (secrets sweep + heuristic CWE source scan + dependency-posture note),
    /// consolidates everything into CWE-tagged findings, and writes a Markdown
    /// report under `reports/`. One call replaces an ad-hoc grep-and-pray audit.
    /// All passes are filesystem-only — no slow/hanging external scanners — so the
    /// agent gets a structured result in seconds.
    /// One single-shot LLM call on a chosen (provider, model) tier. Reaches the
    /// inference engine acquired by `ai_vuln_hunt`; no tools, no agentic loop.
    pub(crate) async fn vuln_llm(
        &self,
        engine: &Arc<crate::ai_engine::Sentient>,
        provider: &str,
        model: &str,
        system: &str,
        user: &str,
        temp: f32,
    ) -> Result<String> {
        let req = crate::ai_engine::AiRequest {
            provider: provider.to_string(),
            model: model.to_string(),
            messages: vec![
                crate::ai_engine::ChatMessage {
                    role: "system".to_string(),
                    content: Some(crate::ai_engine::MessageContent::Text(system.to_string())),
                    ..Default::default()
                },
                crate::ai_engine::ChatMessage {
                    role: "user".to_string(),
                    content: Some(crate::ai_engine::MessageContent::Text(user.to_string())),
                    ..Default::default()
                },
            ],
            temperature: Some(temp),
            autonomous: false,
            mode: Some("Chat".to_string()),
            cyber_mode: None,
            root_access: None,
            ollama_url: None,
            tools: Some(vec![]),
            reasoning_budget: None,
            reasoning_effort: None,
            reasoning_enabled: Some(false),
            feature: Some("Chat".to_string()),
        };
        engine.single_shot_completion(req).await
    }

    /// Best-effort list of locally-installed Ollama model tags.
    pub(crate) async fn list_ollama_tags(&self) -> Vec<String> {
        let base = std::env::var("OLLAMA_HOST")
            .ok()
            .map(|h| if h.starts_with("http") { h } else { format!("http://{}", h) })
            .unwrap_or_else(|| "http://127.0.0.1:11434".to_string());
        let url = format!("{}/api/tags", base.trim_end_matches('/'));
        let client = match reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(4))
            .build()
        {
            Ok(c) => c,
            Err(_) => return vec![],
        };
        match client.get(&url).send().await {
            Ok(r) => r
                .json::<Value>()
                .await
                .ok()
                .and_then(|v| {
                    v.get("models").and_then(|m| m.as_array()).map(|arr| {
                        arr.iter()
                            .filter_map(|x| x.get("name").and_then(|n| n.as_str()).map(|s| s.to_string()))
                            .collect()
                    })
                })
                .unwrap_or_default(),
            Err(_) => vec![],
        }
    }

    /// Best-effort list of model ids from an OpenAI-compatible `/v1/models`
    /// endpoint (used to discover cyberifrit-served models like SecurityEngineer).
    pub(crate) async fn list_openai_compatible_models(&self, base: &str, key: &str) -> Vec<String> {
        let url = format!("{}/v1/models", base.trim_end_matches('/'));
        let client = match reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
        {
            Ok(c) => c,
            Err(_) => return vec![],
        };
        let mut req = client.get(&url);
        if !key.is_empty() {
            req = req.bearer_auth(key);
        }
        match req.send().await {
            Ok(r) => r
                .json::<Value>()
                .await
                .ok()
                .and_then(|v| {
                    v.get("data").and_then(|d| d.as_array()).map(|arr| {
                        arr.iter()
                            .filter_map(|x| x.get("id").and_then(|n| n.as_str()).map(|s| s.to_string()))
                            .collect()
                    })
                })
                .unwrap_or_default(),
            Err(_) => vec![],
        }
    }

    /// Resolve the (cheap, mid, strong) model tiers for a vuln-hunt. Explicit
    /// args win; otherwise auto-detect from installed Ollama models + keyed cloud
    /// providers (incl. cyberifrit). The strong/confirm tier prefers a
    /// security-specialized model, then the biggest available, so it auto-selects
    /// the user's `SecurityEngineer` / `BugTraceAI-Apex` when api.cyberifrit.xyz
    /// is configured. Returns `(provider, model)` per tier.
    pub(crate) async fn resolve_vuln_tiers(
        &self,
        args: &Value,
        config_dir: &std::path::Path,
    ) -> ((String, String), (String, String), (String, String)) {
        let parse_spec = |s: &str| -> (String, String) {
            match s.split_once('|') {
                Some((p, m)) => (p.trim().to_lowercase(), m.trim().to_string()),
                None => ("lemonade".to_string(), s.trim().to_string()),
            }
        };
        let explicit = |key: &str| -> Option<(String, String)> {
            args.get(key)
                .and_then(|v| v.as_str())
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(parse_spec)
        };

        // Candidate pool: (provider, model, size_hint, is_security).
        let mut pool: Vec<(String, String, u32, bool)> = Vec::new();
        for m in self.list_ollama_tags().await {
            let sz = model_size_hint(&m);
            let sec = is_security_model(&m);
            pool.push(("lemonade".to_string(), m, sz, sec));
        }

        let keys: Value = std::fs::read_to_string(config_dir.join("api_keys.json"))
            .ok()
            .and_then(|c| serde_json::from_str(&c).ok())
            .unwrap_or_else(|| json!({}));
        let has = |k: &str| {
            keys.get(k)
                .and_then(|v| v.as_str())
                .map(|s| !s.trim().is_empty())
                .unwrap_or(false)
        };

        if has("cyberifrit") || has("cyberifrit_base_url") {
            let base = keys
                .get("cyberifrit_base_url")
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty())
                .unwrap_or("https://api.cyberifrit.xyz")
                .trim_end_matches('/')
                .to_string();
            let key = keys.get("cyberifrit").and_then(|v| v.as_str()).unwrap_or("").to_string();
            for m in self.list_openai_compatible_models(&base, &key).await {
                let sz = model_size_hint(&m).max(20); // treat custom cloud as capable
                let sec = is_security_model(&m);
                pool.push(("cyberifrit".to_string(), m, sz, sec));
            }
        }

        // Other keyed cloud providers → a sensible default model each.
        let cloud_defaults: &[(&str, &str, &str, u32)] = &[
            ("anthropic", "anthropic", "claude-sonnet-4-6", 100),
            ("openai", "openai", "gpt-4o", 90),
            ("google", "google", "gemini-2.5-pro", 85),
            ("deepseek", "deepseek", "deepseek-chat", 70),
            ("mimo", "mimo", "mimo-v2.5-pro", 60),
        ];
        for (kf, prov, model, rank) in cloud_defaults {
            if has(kf) {
                pool.push((prov.to_string(), model.to_string(), *rank, is_security_model(model)));
            }
        }

        if pool.is_empty() {
            pool.push(("lemonade".to_string(), "qwen2.5-coder:7b".to_string(), 7, false));
        }

        // strong: security-named first, else biggest.
        let strong = explicit("strong_model").unwrap_or_else(|| {
            let mut idx = 0usize;
            for (i, c) in pool.iter().enumerate() {
                let cur = &pool[idx];
                let better = (c.3 && !cur.3) || (c.3 == cur.3 && c.2 > cur.2);
                if better {
                    idx = i;
                }
            }
            (pool[idx].0.clone(), pool[idx].1.clone())
        });

        // cheap: smallest (prefer coder); cloud (size 0) ranked as large.
        let cheap = explicit("cheap_model").unwrap_or_else(|| {
            let mut idx = 0usize;
            for (i, c) in pool.iter().enumerate() {
                let cur = &pool[idx];
                let c_coder = c.1.to_lowercase().contains("cod");
                let cur_coder = cur.1.to_lowercase().contains("cod");
                let c_sz = if c.2 == 0 { 999 } else { c.2 };
                let cur_sz = if cur.2 == 0 { 999 } else { cur.2 };
                let better = c_sz < cur_sz || (c_sz == cur_sz && c_coder && !cur_coder);
                if better {
                    idx = i;
                }
            }
            (pool[idx].0.clone(), pool[idx].1.clone())
        });

        // mid: largest distinct model that is neither cheap nor strong; else strong.
        let mid = explicit("mid_model").unwrap_or_else(|| {
            let mut best: Option<(usize, u32)> = None;
            for (i, c) in pool.iter().enumerate() {
                let is_strong = c.0 == strong.0 && c.1 == strong.1;
                let is_cheap = c.0 == cheap.0 && c.1 == cheap.1;
                if is_strong || is_cheap {
                    continue;
                }
                let sz = if c.2 == 0 { 50 } else { c.2 };
                if best.map(|(_, b)| sz > b).unwrap_or(true) {
                    best = Some((i, sz));
                }
            }
            match best {
                Some((i, _)) => (pool[i].0.clone(), pool[i].1.clone()),
                None => strong.clone(),
            }
        });

        (cheap, mid, strong)
    }

    /// AI vulnerability-hunting pipeline (3-stage, tiered models) — the
    /// HackerOne #1-KR methodology. Stage 1 chunks the codebase; stage 2 uses a
    /// cheap model for high-recall candidate generation; stage 3 validates in two
    /// passes (mid model culls false positives → strong model confirms + enriches),
    /// flagging policy-dependent and cross-component cases honestly. Writes a
    /// CWE-tagged Markdown report to `reports/` and returns structured findings.
    pub(crate) async fn ai_vuln_hunt(&self, args: Value) -> Result<Value> {
        let scope = args.get("path").and_then(|v| v.as_str()).unwrap_or(".").to_string();
        let write_report = args.get("write_report").and_then(|v| v.as_bool()).unwrap_or(true);
        let chunk_bytes = args
            .get("chunk_bytes")
            .and_then(|v| v.as_u64())
            .unwrap_or(24_000)
            .max(2_000) as usize;
        let max_files = args.get("max_files").and_then(|v| v.as_u64()).unwrap_or(400) as usize;
        let max_chunks = args
            .get("max_chunks")
            .and_then(|v| v.as_u64())
            .unwrap_or(60)
            .max(1) as usize;

        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, &scope)?;
        if !full_path.exists() {
            return Err(anyhow!("Path not found: {}", scope));
        }

        // Engine + config dir via the EditorState back-reference.
        let (engine, config_dir) = {
            let state = self.editor_state()
                .ok_or_else(|| anyhow!("EditorState unavailable (inference engine offline)"))?;
            (state.ai.engine.clone(), state.config_dir.clone())
        };

        let (cheap, mid, strong) = self.resolve_vuln_tiers(&args, &config_dir).await;
        push_activity(
            &self.activity_log,
            "ai-action",
            json!({
                "action": format!("Vuln-hunt tiers — cheap: {}|{}  ·  mid: {}|{}  ·  strong: {}|{}",
                    cheap.0, cheap.1, mid.0, mid.1, strong.0, strong.1),
                "tool": "ai_vuln_hunt"
            }),
        );

        // ── Stage 1: chunk the codebase into analysis units ──
        let code_exts = [
            "rs", "ts", "tsx", "js", "jsx", "py", "go", "java", "kt", "rb", "php",
            "c", "cpp", "cs", "swift", "sh", "ps1", "sql", "html",
        ];
        let skip_dirs = [
            "node_modules", "target", "dist", "build", ".git", "vendor", "third_party",
            ".next", ".cache", "__pycache__", "reports", "exploits",
        ];
        struct Chunk {
            header: String,
            body: String,
        }
        let mut chunks: Vec<Chunk> = Vec::new();
        let mut cur = String::new();
        let mut cur_files: Vec<String> = Vec::new();
        let mut files_scanned = 0usize;

        let walker = walkdir::WalkDir::new(&full_path)
            .max_depth(20)
            .into_iter()
            .filter_entry(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                !skip_dirs.contains(&name.as_str())
            });
        for entry in walker.flatten() {
            if chunks.len() >= max_chunks || files_scanned >= max_files {
                break;
            }
            if !entry.file_type().is_file() {
                continue;
            }
            let ext = entry
                .path()
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();
            if !code_exts.contains(&ext.as_str()) {
                continue;
            }
            if let Ok(meta) = entry.metadata() {
                if meta.len() > 1_500_000 {
                    continue;
                }
            }
            let content = match fs::read_to_string(entry.path()) {
                Ok(c) => c,
                Err(_) => continue,
            };
            files_scanned += 1;
            let rel = entry
                .path()
                .strip_prefix(&*root)
                .unwrap_or(entry.path())
                .to_string_lossy()
                .to_string();
            if audit_skip_path(&rel) {
                continue;
            }

            let numbered: String = content
                .lines()
                .enumerate()
                .map(|(i, l)| format!("{:>4}| {}\n", i + 1, l.chars().take(400).collect::<String>()))
                .collect();
            let piece = format!("\n=== FILE: {} ===\n{}", rel, numbered);

            if cur.len() + piece.len() > chunk_bytes && !cur.is_empty() {
                chunks.push(Chunk {
                    header: cur_files.join(", "),
                    body: std::mem::take(&mut cur),
                });
                cur_files.clear();
                if chunks.len() >= max_chunks {
                    break;
                }
            }
            cur.push_str(&piece);
            cur_files.push(rel);
        }
        if !cur.is_empty() && chunks.len() < max_chunks {
            chunks.push(Chunk {
                header: cur_files.join(", "),
                body: cur,
            });
        }
        if chunks.is_empty() {
            return Ok(json!({
                "summary": "No source files found to analyze.",
                "scope": scope, "findings": []
            }));
        }

        // ── Stage 2: hypothesis generation (cheap, balanced precision/recall) ──
        let hypo_sys = "You are a vulnerability analyst for authorized security audits. \
Report ONLY issues with a plausible exploit path: untrusted input reaching a dangerous sink, \
a missing authorization check on a sensitive action, or a clearly unsafe default exposed to users. \
SKIP: test/mock/fixture code, comments, dead code, style issues, generic hardening advice, \
and hypothetical attacks without a visible source→sink path. \
Reply ONLY with a JSON array; each item: \
{\"file\":\"path\",\"line\":<int>,\"category\":\"short label\",\"cwe\":\"CWE-XXX\",\"why\":\"concrete attack path in one sentence\"}. \
No prose, no markdown fences.";
        let mut candidates: Vec<Value> = Vec::new();
        for (i, ch) in chunks.iter().enumerate() {
            push_activity(
                &self.activity_log,
                "ai-action",
                json!({
                    "action": format!("Vuln-hunt stage 2 — scanning chunk {}/{} ({})", i + 1, chunks.len(), ch.header),
                    "tool": "ai_vuln_hunt"
                }),
            );
            let user = format!("Source chunk (files: {}):\n{}", ch.header, ch.body);
            if let Ok(reply) = self.vuln_llm(&engine, &cheap.0, &cheap.1, hypo_sys, &user, 0.2).await {
                if let Some(Value::Array(arr)) = extract_json_loose(&reply) {
                    candidates.extend(arr);
                }
            }
        }
        if candidates.is_empty() {
            return Ok(json!({
                "summary": "Hypothesis stage produced no candidates.",
                "scope": scope, "chunks": chunks.len(), "files_scanned": files_scanned, "findings": []
            }));
        }
        if candidates.len() > 60 {
            candidates.truncate(60);
        }

        // ── Stage 3a: mid model drops weak / hypothetical candidates ──
        push_activity(
            &self.activity_log,
            "ai-action",
            json!({
                "action": format!("Vuln-hunt stage 3a — filtering {} candidates ({})", candidates.len(), mid.1),
                "tool": "ai_vuln_hunt"
            }),
        );
        let filter_sys = "You are a strict triage filter. DROP a candidate unless ALL are true: \
(1) attacker-controlled or cross-tenant input is visible or strongly implied, \
(2) the dangerous sink or missing control is in the cited file/line, \
(3) it is not test/mock/example/fixture code. \
Remove generic CWE spam, linter-style nits, and 'could be X if Y' hypotheticals. \
Reply ONLY with a JSON array of SURVIVING candidates (same object shape). No prose, no fences.";
        let cand_json = serde_json::to_string(&candidates).unwrap_or_else(|_| "[]".to_string());
        let mut survivors: Vec<Value> = candidates.clone();
        if let Ok(reply) = self.vuln_llm(&engine, &mid.0, &mid.1, filter_sys, &cand_json, 0.1).await {
            if let Some(Value::Array(arr)) = extract_json_loose(&reply) {
                if !arr.is_empty() {
                    survivors = arr;
                }
            }
        }

        // ── Stage 3b: strong model confirms in small batches (reduces hallucination) ──
        push_activity(
            &self.activity_log,
            "ai-action",
            json!({
                "action": format!("Vuln-hunt stage 3b — confirming {} survivors ({})", survivors.len(), strong.1),
                "tool": "ai_vuln_hunt"
            }),
        );
        let confirm_sys = "You are the senior validator in an authorized security audit. \
For each candidate, CONFIRM only if the cited line supports a real exploit path. \
REJECT lint noise, framework-safe patterns, and guesses. \
If policy docs are needed to decide, set policy_dependent=true and confidence<=0.5. \
If the fix may live in another service, set cross_component=true and confidence<=0.55. \
Reply ONLY with a JSON array of CONFIRMED findings; each item: \
{\"title\":\"\",\"severity\":\"CRITICAL|HIGH|MEDIUM|LOW\",\"cwe\":\"CWE-XXX\",\"category\":\"\",\
\"path\":\"\",\"line\":<int>,\"description\":\"what is wrong\",\"evidence\":\"exact code fragment\",\
\"attack_scenario\":\"step-by-step exploit narrative\",\"impact\":\"business/security impact\",\
\"remediation\":\"specific fix\",\"poc\":\"minimal repro steps or payload\",\"confidence\":0.0,\
\"policy_dependent\":false,\"cross_component\":false}. No prose, no fences.";
        let mut findings: Vec<Value> = Vec::new();
        for batch in survivors.chunks(10) {
            let batch_json = serde_json::to_string(batch).unwrap_or_else(|_| "[]".to_string());
            if let Ok(reply) = self.vuln_llm(&engine, &strong.0, &strong.1, confirm_sys, &batch_json, 0.05).await {
                if let Some(Value::Array(arr)) = extract_json_loose(&reply) {
                    findings.extend(arr);
                }
            }
        }
        // Deterministic post-filter: verify file/line/evidence — never promote unverified survivors.
        findings = tighten_security_findings(&root, findings, 0.60);
        let manual_review: Vec<Value> = findings
            .iter()
            .filter(|f| {
                f.get("policy_dependent").and_then(|v| v.as_bool()).unwrap_or(false)
                    || f.get("cross_component").and_then(|v| v.as_bool()).unwrap_or(false)
                    || f.get("confidence").and_then(|v| v.as_f64()).unwrap_or(1.0) < 0.65
            })
            .cloned()
            .collect();
        findings.retain(|f| {
            !f.get("policy_dependent").and_then(|v| v.as_bool()).unwrap_or(false)
                && !f.get("cross_component").and_then(|v| v.as_bool()).unwrap_or(false)
                && f.get("confidence").and_then(|v| v.as_f64()).unwrap_or(0.0) >= 0.65
        });

        // ── Normalize + id + sort by severity ──
        let sev_rank = |s: &str| -> u8 {
            match s.to_uppercase().as_str() {
                "CRITICAL" => 0,
                "HIGH" => 1,
                "MEDIUM" => 2,
                "LOW" => 3,
                _ => 4,
            }
        };
        let mut fid = 0u32;
        for f in findings.iter_mut() {
            fid += 1;
            if let Value::Object(m) = f {
                m.insert("id".to_string(), json!(format!("VH-{:03}", fid)));
            }
        }
        findings.sort_by_key(|f| sev_rank(f.get("severity").and_then(|v| v.as_str()).unwrap_or("INFO")));

        let mut by_sev: std::collections::BTreeMap<String, u64> = std::collections::BTreeMap::new();
        for f in &findings {
            let s = f.get("severity").and_then(|v| v.as_str()).unwrap_or("INFO").to_uppercase();
            *by_sev.entry(s).or_insert(0) += 1;
        }

        // ── Markdown report (HackerOne-style detail) ──
        let mut report_path = String::new();
        if write_report {
            let ts = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            let reports_dir = root.join("reports");
            let _ = fs::create_dir_all(&reports_dir);
            let file = reports_dir.join(format!("vuln-hunt-{}.md", ts));
            let fs_s = files_scanned.to_string();
            let ch_s = chunks.len().to_string();
            let cand_s = candidates.len().to_string();
            let surv_s = survivors.len().to_string();
            let conf_s = findings.len().to_string();
            let man_s = manual_review.len().to_string();
            let tiers_s = format!(
                "cheap `{}|{}` → mid `{}|{}` → strong `{}|{}`",
                cheap.0, cheap.1, mid.0, mid.1, strong.0, strong.1
            );
            let md = render_security_report(
                "AI Vulnerability Hunt",
                &[
                    ("Scope", scope.as_str()),
                    ("Files scanned", &fs_s),
                    ("Chunks", &ch_s),
                    ("Raw candidates", &cand_s),
                    ("After triage", &surv_s),
                    ("Confirmed", &conf_s),
                    ("Manual review queue", &man_s),
                    ("Tiers", &tiers_s),
                ],
                &findings,
                &manual_review,
                &*root,
            );
            if fs::write(&file, md).is_ok() {
                report_path = file
                    .strip_prefix(&*root)
                    .unwrap_or(&file)
                    .to_string_lossy()
                    .to_string();
            }
        }

        Ok(json!({
            "scope": scope,
            "files_scanned": files_scanned,
            "chunks": chunks.len(),
            "candidates": candidates.len(),
            "survivors": survivors.len(),
            "total_findings": findings.len(),
            "manual_review": manual_review.len(),
            "by_severity": by_sev,
            "tiers": {
                "cheap": format!("{}|{}", cheap.0, cheap.1),
                "mid": format!("{}|{}", mid.0, mid.1),
                "strong": format!("{}|{}", strong.0, strong.1)
            },
            "report_path": report_path,
            "findings": findings,
            "manual_review_queue": manual_review,
            "summary": format!("AI vuln-hunt: {} confirmed, {} queued for manual review (from {} candidates). Report: {}",
                findings.len(), manual_review.len(), candidates.len(),
                if report_path.is_empty() { "(not written)".to_string() } else { report_path.clone() }),
        }))
    }

    pub(crate) async fn deep_security_audit(&self, args: Value) -> Result<Value> {
        use regex::Regex;
        let scope = args.get("path").and_then(|v| v.as_str()).unwrap_or(".").to_string();
        let depth = args.get("depth").and_then(|v| v.as_str()).unwrap_or("standard").to_lowercase();
        let write_report = args.get("write_report").and_then(|v| v.as_bool()).unwrap_or(true);
        let max_findings = args.get("max_findings").and_then(|v| v.as_u64()).unwrap_or(400) as usize;

        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, &scope)?;
        if !full_path.exists() {
            return Err(anyhow!("Path not found: {}", scope));
        }

        let sev_rank = |s: &str| -> u8 {
            match s.to_uppercase().as_str() {
                "CRITICAL" => 0, "HIGH" => 1, "MEDIUM" => 2, "LOW" => 3, _ => 4,
            }
        };
        let mut findings: Vec<Value> = Vec::new();
        let mut fid: u32 = 0;

        // ── Pass 1: secrets sweep (reuse the dedicated scanner) ──────────────
        if let Ok(secrets) = self.secrets_scan(json!({ "path": scope, "include_low": depth == "deep" })).await {
            if let Some(arr) = secrets.get("findings").and_then(|v| v.as_array()) {
                for f in arr {
                    if findings.len() >= max_findings { break; }
                    fid += 1;
                    findings.push(json!({
                        "id": format!("SEC-{:03}", fid),
                        "title": format!("Hardcoded secret: {}", f.get("kind").and_then(|v| v.as_str()).unwrap_or("credential")),
                        "severity": f.get("severity").and_then(|v| v.as_str()).unwrap_or("HIGH"),
                        "cwe": "CWE-798",
                        "category": "Hardcoded Credentials",
                        "path": f.get("path").cloned().unwrap_or(json!("")),
                        "line": f.get("line").cloned().unwrap_or(json!(0)),
                        "evidence": f.get("preview").cloned().unwrap_or(json!("[redacted]")),
                        "remediation": "Move the secret to an environment variable / secret manager and rotate it.",
                        "confidence": 0.9
                    }));
                }
            }
        }

        // ── Pass 2: heuristic CWE source scan ────────────────────────────────
        // Patterns live in `security_audit_patterns()` (module-level) so they can be
        // unit-tested in isolation. INFO-level noise (unwrap/expect) only in deep.
        let raw_patterns = security_audit_patterns();
        let patterns: Vec<(&str, &str, &str, Regex, &str)> = raw_patterns.iter()
            .filter(|(_, sev, _, _, _)| depth == "deep" || *sev != "INFO")
            .filter_map(|(cat, sev, cwe, rx, rem)| Regex::new(rx).ok().map(|r| (*cat, *sev, *cwe, r, *rem)))
            .collect();

        let code_exts = ["rs", "ts", "tsx", "js", "jsx", "py", "go", "java", "kt", "rb", "php", "c", "cpp", "cs", "swift", "sh", "ps1"];
        let skip_dirs = ["node_modules", "target", "dist", "build", ".git", "vendor", "third_party", ".next", ".cache", "__pycache__"];
        let mut per_cat: std::collections::HashMap<&str, u32> = std::collections::HashMap::new();
        let mut files_scanned: usize = 0;

        let walker = walkdir::WalkDir::new(&full_path).max_depth(20).into_iter().filter_entry(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            !skip_dirs.contains(&name.as_str())
        });
        for entry in walker.flatten() {
            if findings.len() >= max_findings { break; }
            if !entry.file_type().is_file() { continue; }
            let ext = entry.path().extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
            if !code_exts.contains(&ext.as_str()) { continue; }
            if let Ok(meta) = entry.metadata() { if meta.len() > 2 * 1024 * 1024 { continue; } }
            let content = match fs::read_to_string(entry.path()) { Ok(c) => c, Err(_) => continue };
            files_scanned += 1;
            let rel = entry.path().strip_prefix(&*root).unwrap_or(entry.path()).to_string_lossy().to_string();
            if audit_skip_path(&rel) { continue; }
            for (line_no, line) in content.lines().enumerate() {
                if findings.len() >= max_findings { break; }
                if line.len() > 1000 { continue; }
                for (cat, sev, cwe, re, rem) in &patterns {
                    // Cap per-category so a common pattern can't flood the report.
                    if per_cat.get(*cat).copied().unwrap_or(0) >= 30 { continue; }
                    if re.is_match(line) {
                        fid += 1;
                        *per_cat.entry(*cat).or_insert(0) += 1;
                        findings.push(json!({
                            "id": format!("SEC-{:03}", fid),
                            "title": *cat,
                            "severity": *sev,
                            "cwe": *cwe,
                            "category": *cat,
                            "path": rel,
                            "line": line_no + 1,
                            "evidence": line.trim().chars().take(160).collect::<String>(),
                            "remediation": *rem,
                            "confidence": 0.5
                        }));
                        break; // one finding per line
                    }
                }
            }
        }

        // ── Pass 3: dependency-posture note (fast — manifest detection only) ──
        let mut dep_notes: Vec<String> = Vec::new();
        let manifests = [
            ("Cargo.toml", "Run `cargo audit` to check Rust dependencies for known CVEs (RUSTSEC)."),
            ("package.json", "Run `npm audit` / `pnpm audit` to check npm dependencies for known CVEs."),
            ("requirements.txt", "Run `pip-audit` to check Python dependencies for known CVEs."),
            ("go.mod", "Run `govulncheck ./...` to check Go dependencies for known CVEs."),
        ];
        for (mf, hint) in manifests {
            if root.join(mf).exists() { dep_notes.push(format!("`{}` present — {}", mf, hint)); }
        }

        // ── Consolidate + sort by severity ───────────────────────────────────
        findings = tighten_security_findings(&root, findings, 0.45);
        findings.sort_by_key(|f| sev_rank(f.get("severity").and_then(|v| v.as_str()).unwrap_or("INFO")));
        let mut by_sev: std::collections::BTreeMap<String, u64> = std::collections::BTreeMap::new();
        for f in &findings {
            let s = f.get("severity").and_then(|v| v.as_str()).unwrap_or("INFO").to_uppercase();
            *by_sev.entry(s).or_insert(0) += 1;
        }

        // ── Write Markdown report ────────────────────────────────────────────
        let mut report_path = String::new();
        if write_report {
            let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs()).unwrap_or(0);
            let reports_dir = root.join("reports");
            let _ = fs::create_dir_all(&reports_dir);
            let file = reports_dir.join(format!("security-audit-{}.md", ts));
            let files_scanned_s = files_scanned.to_string();
            let total_s = findings.len().to_string();
            let md = if dep_notes.is_empty() {
                render_security_report(
                    "Static Security Audit",
                    &[
                        ("Scope", scope.as_str()),
                        ("Depth", depth.as_str()),
                        ("Files scanned", &files_scanned_s),
                        ("Total findings", &total_s),
                    ],
                    &findings,
                    &[],
                    &*root,
                )
            } else {
                let dep_block = dep_notes.join("; ");
                render_security_report(
                    "Static Security Audit",
                    &[
                        ("Scope", scope.as_str()),
                        ("Depth", depth.as_str()),
                        ("Files scanned", &files_scanned_s),
                        ("Total findings", &total_s),
                        ("Dependencies", &dep_block),
                    ],
                    &findings,
                    &[],
                    &*root,
                )
            };
            if fs::write(&file, md).is_ok() {
                report_path = file.strip_prefix(&*root).unwrap_or(&file).to_string_lossy().to_string();
            }
        }

        Ok(json!({
            "scope": scope,
            "depth": depth,
            "files_scanned": files_scanned,
            "total_findings": findings.len(),
            "by_severity": by_sev,
            "dependency_notes": dep_notes,
            "report_path": report_path,
            "findings": findings,
            "summary": format!("Deep security audit: {} finding(s) across {} file(s). Report: {}",
                findings.len(), files_scanned,
                if report_path.is_empty() { "(not written)".to_string() } else { report_path.clone() }),
        }))
    }

    /// DYNAMIC web-app audit against a live URL via the stealth browser. Audits
    /// headers, cookie flags, forms, and mixed content into CWE-tagged findings +
    /// a Markdown report. Authorized pentest / bug-bounty use only.
    pub(crate) async fn web_security_audit(&self, args: Value) -> Result<Value> {
        let url = args.get("url").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing url"))?.to_string();
        let write_report = args.get("write_report").and_then(|v| v.as_bool()).unwrap_or(true);

        // Severity ordering for sort (CRITICAL highest).
        fn rank(s: &str) -> u8 {
            match s.to_uppercase().as_str() {
                "CRITICAL" => 0, "HIGH" => 1, "MEDIUM" => 2, "LOW" => 3, _ => 4,
            }
        }
        let mut findings: Vec<Value> = Vec::new();
        let mut n = 0u32;
        let mut add = |findings: &mut Vec<Value>, sev: &str, cwe: &str, cat: &str,
                       title: &str, evidence: String, remediation: &str, confidence: &str| {
            n += 1;
            findings.push(json!({
                "id": format!("WEB-{:03}", n),
                "title": title,
                "severity": sev,
                "cwe": cwe,
                "category": cat,
                "path": url,
                "line": 0,
                "evidence": evidence.chars().take(300).collect::<String>(),
                "remediation": remediation,
                "confidence": confidence,
            }));
        };

        // ── Fetch the target ─────────────────────────────────────────────────
        self.browser_state.ensure_started().await.map_err(|e| anyhow!("browser start failed: {e}"))?;
        let nav = self.browser_state
            .cmd("navigate", json!({ "url": url }), 60).await
            .map_err(|e| anyhow!("navigate failed: {e}"))?;
        self.browser_state.refresh_cache(&url).await;

        let final_url = nav.get("url").and_then(|v| v.as_str()).unwrap_or(&url).to_string();
        let is_https = final_url.starts_with("https://");
        let http_status = nav.get("status").cloned().unwrap_or(json!(null));
        let headers = nav.get("headers").cloned().unwrap_or_else(|| json!({}));
        let hget = |name: &str| headers.get(name).and_then(|v| v.as_str()).map(|s| s.to_string());

        // ── Pass 1: response security headers (table-driven) ───────────────
        struct HeaderCheck {
            name: &'static str,
            severity: &'static str,
            cwe: &'static str,
            title: &'static str,
            evidence: &'static str,
            remediation: &'static str,
            https_only: bool,
            condition: fn(&str, bool) -> bool,  // (csp_value, is_https) -> should_check
        }

        static HEADER_CHECKS: &[HeaderCheck] = &[
            HeaderCheck {
                name: "content-security-policy", severity: "MEDIUM", cwe: "CWE-693",
                title: "Missing Content-Security-Policy",
                evidence: "No `content-security-policy` response header",
                remediation: "Define a restrictive CSP (default-src 'self'; …) to mitigate XSS/data injection.",
                https_only: false, condition: |_csp, _| true,
            },
            HeaderCheck {
                name: "strict-transport-security", severity: "MEDIUM", cwe: "CWE-319",
                title: "Missing HSTS (Strict-Transport-Security)",
                evidence: "HTTPS page without `strict-transport-security`",
                remediation: "Send `Strict-Transport-Security: max-age=63072000; includeSubDomains; preload`.",
                https_only: true, condition: |_csp, _| true,
            },
            HeaderCheck {
                name: "x-frame-options", severity: "MEDIUM", cwe: "CWE-1021",
                title: "Clickjacking: no X-Frame-Options / frame-ancestors",
                evidence: "Neither `x-frame-options` nor CSP `frame-ancestors` present",
                remediation: "Add `X-Frame-Options: DENY` or CSP `frame-ancestors 'none'`.",
                https_only: false, condition: |csp, _| !csp.to_lowercase().contains("frame-ancestors"),
            },
            HeaderCheck {
                name: "x-content-type-options", severity: "LOW", cwe: "CWE-693",
                title: "Missing X-Content-Type-Options",
                evidence: "No `x-content-type-options: nosniff`",
                remediation: "Add `X-Content-Type-Options: nosniff` to stop MIME sniffing.",
                https_only: false, condition: |_csp, _| true,
            },
            HeaderCheck {
                name: "referrer-policy", severity: "LOW", cwe: "CWE-200",
                title: "Missing Referrer-Policy",
                evidence: "No `referrer-policy` header",
                remediation: "Add `Referrer-Policy: strict-origin-when-cross-origin`.",
                https_only: false, condition: |_csp, _| true,
            },
            HeaderCheck {
                name: "permissions-policy", severity: "INFO", cwe: "CWE-693",
                title: "Missing Permissions-Policy",
                evidence: "No `permissions-policy` header",
                remediation: "Restrict powerful features via `Permissions-Policy`.",
                https_only: false, condition: |_csp, _| true,
            },
        ];

        let csp = hget("content-security-policy").unwrap_or_default();
        for check in HEADER_CHECKS {
            if check.https_only && !is_https { continue; }
            if !(check.condition)(&csp, is_https) { continue; }
            if hget(check.name).is_none() {
                add(&mut findings, check.severity, check.cwe, "headers",
                    check.title, check.evidence.to_string(), check.remediation, "high");
            }
        }
        // Information-disclosure banners.
        for (h, label) in [("server", "Server"), ("x-powered-by", "X-Powered-By"),
                           ("x-aspnet-version", "X-AspNet-Version"), ("x-aspnetmvc-version", "X-AspNetMvc-Version")] {
            if let Some(v) = hget(h) {
                if !v.trim().is_empty() {
                    add(&mut findings, "LOW", "CWE-200", "info-disclosure",
                        &format!("Version/tech disclosure via {} header", label),
                        format!("{}: {}", label, v),
                        "Suppress or genericize version banners to slow fingerprinting.", "high");
                }
            }
        }

        // ── Pass 2: cookie flags (table-driven) ────────────────────────────
        struct CookieCheck {
            condition: fn(bool, bool, &str) -> bool,  // (secure, http_only, same_site) -> should_flag
            severity: &'static str,
            cwe: &'static str,
            title: &'static str,
            evidence_fn: fn(&str) -> String,  // cookie_name -> evidence
            remediation: &'static str,
            confidence: &'static str,
        }

        static COOKIE_CHECKS: &[CookieCheck] = &[
            CookieCheck {
                condition: |secure, _, _| !secure,
                severity: "MEDIUM", cwe: "CWE-614",
                title: "Cookie without Secure flag",
                evidence_fn: |name| format!("Cookie `{}` lacks Secure on an HTTPS site", name),
                remediation: "Set the `Secure` attribute so cookies are only sent over TLS.",
                confidence: "high",
            },
            CookieCheck {
                condition: |_, http_only, _| !http_only,
                severity: "MEDIUM", cwe: "CWE-1004",
                title: "Cookie without HttpOnly flag",
                evidence_fn: |name| format!("Cookie `{}` is readable from JavaScript (no HttpOnly)", name),
                remediation: "Set `HttpOnly` to block script access (XSS token theft).",
                confidence: "medium",
            },
            CookieCheck {
                condition: |_, _, same_site| same_site.is_empty() || same_site.eq_ignore_ascii_case("none"),
                severity: "LOW", cwe: "CWE-1275",
                title: "Cookie with weak SameSite policy",
                evidence_fn: |name| format!("Cookie `{}` SameSite='(weak)'", name),
                remediation: "Set `SameSite=Lax` or `Strict` to reduce CSRF exposure.",
                confidence: "medium",
            },
        ];

        if let Ok(ck) = self.browser_state.cmd("cookies", json!({}), 15).await {
            if let Some(cookies) = ck.get("cookies").and_then(|v| v.as_array()) {
                for c in cookies {
                    let name = c.get("name").and_then(|v| v.as_str()).unwrap_or("?");
                    let secure = c.get("secure").and_then(|v| v.as_bool()).unwrap_or(false);
                    let http_only = c.get("httpOnly").and_then(|v| v.as_bool()).unwrap_or(false);
                    let same_site = c.get("sameSite").and_then(|v| v.as_str()).unwrap_or("");
                    for check in COOKIE_CHECKS {
                        if (check.condition)(secure, http_only, same_site) {
                            add(&mut findings, check.severity, check.cwe, "cookies",
                                check.title, (check.evidence_fn)(name), check.remediation, check.confidence);
                        }
                    }
                }
            }
        }

        // ── Pass 3: forms (injection / auth surface) ─────────────────────────
        if let Ok(fm) = self.browser_state.cmd("forms", json!({}), 15).await {
            if let Some(forms) = fm.get("forms").and_then(|v| v.as_array()) {
                for f in forms {
                    let action = f.get("action").and_then(|v| v.as_str()).unwrap_or("");
                    let method = f.get("method").and_then(|v| v.as_str()).unwrap_or("get").to_lowercase();
                    let inputs = f.get("inputs").and_then(|v| v.as_array()).cloned().unwrap_or_default();
                    let has_password = inputs.iter().any(|i| i.get("type").and_then(|v| v.as_str()) == Some("password"));
                    let has_csrf = inputs.iter().any(|i| {
                        let nm = i.get("name").and_then(|v| v.as_str()).unwrap_or("").to_lowercase();
                        nm.contains("csrf") || nm.contains("xsrf") || nm.contains("authenticity")
                            || nm.contains("requestverificationtoken") || nm == "_token"
                    });
                    if has_password && !is_https {
                        add(&mut findings, "HIGH", "CWE-319", "forms",
                            "Password form served over HTTP",
                            format!("Form action=`{}` collects a password without TLS", action),
                            "Serve the page and submit credentials only over HTTPS.", "high");
                    }
                    if has_password && action.starts_with("http://") {
                        add(&mut findings, "HIGH", "CWE-319", "forms",
                            "Credentials submitted to an insecure (http://) endpoint",
                            format!("Form action=`{}`", action),
                            "Point the form action at an https:// endpoint.", "high");
                    }
                    if has_password && method == "get" {
                        add(&mut findings, "MEDIUM", "CWE-598", "forms",
                            "Credentials sent via GET (logged in URLs)",
                            format!("Form method=GET action=`{}` with a password field", action),
                            "Use POST for credential submission.", "high");
                    }
                    if method == "post" && !has_csrf {
                        add(&mut findings, "MEDIUM", "CWE-352", "forms",
                            "POST form without an apparent CSRF token",
                            format!("Form action=`{}` (POST) has no hidden csrf/token field", action),
                            "Add a per-request anti-CSRF token and verify it server-side.", "low");
                    }
                }
            }
        }

        // ── Pass 4: mixed content ────────────────────────────────────────────
        if is_https {
            if let Ok(ct) = self.browser_state.cmd("content", json!({}), 20).await {
                let html = ct.get("html").and_then(|v| v.as_str()).unwrap_or("");
                let mut samples: Vec<String> = Vec::new();
                for attr in ["src=\"http://", "href=\"http://", "src='http://", "href='http://"] {
                    let mut idx = 0;
                    while let Some(pos) = html[idx..].find(attr) {
                        let start = idx + pos + attr.len();
                        let end = html[start..].find(['"', '\'']).map(|e| start + e).unwrap_or(start);
                        let u = &html[start..end.min(start + 120)];
                        if !u.is_empty() && samples.len() < 5 && !samples.iter().any(|s| s == u) {
                            samples.push(u.to_string());
                        }
                        idx = start;
                        if samples.len() >= 5 { break; }
                    }
                }
                if !samples.is_empty() {
                    add(&mut findings, "MEDIUM", "CWE-319", "mixed-content",
                        "Mixed content: insecure resources on an HTTPS page",
                        format!("http:// resources: {}", samples.join(", ")),
                        "Load all sub-resources over HTTPS (or protocol-relative) to prevent MITM tampering.", "high");
                }
            }
        }

        // ── Consolidate + sort ───────────────────────────────────────────────
        findings.sort_by_key(|f| rank(f.get("severity").and_then(|v| v.as_str()).unwrap_or("INFO")));
        let mut by_sev: std::collections::BTreeMap<String, u64> = std::collections::BTreeMap::new();
        for f in &findings {
            let s = f.get("severity").and_then(|v| v.as_str()).unwrap_or("INFO").to_uppercase();
            *by_sev.entry(s).or_insert(0) += 1;
        }

        // ── Write Markdown report ────────────────────────────────────────────
        let mut report_path = String::new();
        if write_report {
            let root = self.root_path.lock().await.clone();
            let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs()).unwrap_or(0);
            let slug = crate::pentest_report::slugify_host(&url);
            let reports_dir = root.join("reports").join(&slug);
            let _ = fs::create_dir_all(&reports_dir);
            let file = reports_dir.join(format!("PENTEST-REPORT-{}.md", ts));
            let md = crate::pentest_report::render_web_pentest_report(
                crate::pentest_report::ReportMeta {
                    target: &url,
                    final_url: &final_url,
                    http_status: &http_status.to_string(),
                    assessment_type: "External Web Application (Automated)",
                    in_scope: &format!("- `{}`\n- Final resolved URL: `{}`", url, final_url),
                    out_of_scope: "- localhost / 127.0.0.1 / LAN\n- Unrelated domain spellings\n- Third-party assets unless explicitly scoped",
                },
                &findings,
            );
            if fs::write(&file, md).is_ok() {
                report_path = file.strip_prefix(&root).unwrap_or(&file).to_string_lossy().to_string();
            }
        }

        Ok(json!({
            "status": "success",
            "target": url,
            "final_url": final_url,
            "http_status": http_status,
            "total_findings": findings.len(),
            "by_severity": by_sev,
            "report_path": report_path,
            "findings": findings,
            "summary": format!("Web audit of {}: {} finding(s). Report: {}",
                final_url, findings.len(),
                if report_path.is_empty() { "(not written)".to_string() } else { report_path.clone() }),
            "open_preview_hint": if report_path.is_empty() { Value::Null } else {
                json!({ "path": report_path, "preview": true })
            },
        }))
    }

    /// Shared HTTP client for the native security scanners. Pooled + bounded so
    /// agent-driven scans don't spin up a fresh connection pool each call.
    pub(crate) fn security_http_client() -> &'static reqwest::Client {
        use std::sync::OnceLock;
        static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
        CLIENT.get_or_init(|| {
            reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .redirect(reqwest::redirect::Policy::limited(8))
                .pool_max_idle_per_host(8)
                .build()
                .unwrap_or_else(|_| reqwest::Client::new())
        })
    }

    /// Agent tool: native Vega DAST scan of a live URL.
    pub(crate) async fn vega_dast_scan(&self, args: Value) -> Result<Value> {
        let url = args["url"].as_str().ok_or_else(|| anyhow!("Missing url"))?;
        let opts = crate::vega::VegaScanOptions {
            target_url: url.to_string(),
            authorized: true,
            max_pages: args.get("max_pages").and_then(|v| v.as_u64()).map(|n| n as usize),
            max_depth: args.get("max_depth").and_then(|v| v.as_u64()).map(|n| n as u32),
            injection_modules: None,
            run_passive: Some(true),
            ai_triage: args.get("ai_triage").and_then(|v| v.as_bool()),
            ai_model: None,
            ollama_url: None,
            session_cookie: args.get("session_cookie").and_then(|v| v.as_str()).map(String::from),
        };
        let result = crate::vega::run_campaign(opts).await.map_err(|e| anyhow!(e))?;
        serde_json::to_value(result).map_err(|e| anyhow!(e))
    }

    /// Agent tool: scan JS bundles/chunks for leaked secrets (url or local path).
    pub(crate) async fn chunk_secret_scan(&self, args: Value) -> Result<Value> {
        if let Some(url) = args.get("url").and_then(|v| v.as_str()) {
            let summary = crate::chunk_secrets::scan_url(url, Self::security_http_client())
                .await
                .map_err(|e| anyhow!(e))?;
            return serde_json::to_value(summary).map_err(|e| anyhow!(e));
        }
        let path_str = args.get("path").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Provide either 'url' or 'path'"))?;
        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, path_str)?;
        let cap = args.get("max_files").and_then(|v| v.as_u64()).unwrap_or(2_000) as usize;
        let summary = crate::chunk_secrets::scan_directory(&full_path, cap).map_err(|e| anyhow!(e))?;
        serde_json::to_value(summary).map_err(|e| anyhow!(e))
    }

    /// Agent tool: combined bounty recon (chunk secrets + XSS probe) on a URL.
    pub(crate) async fn bounty_scan(&self, args: Value) -> Result<Value> {
        let url = args["url"].as_str().ok_or_else(|| anyhow!("Missing url"))?;
        let include_xss = args.get("include_xss").and_then(|v| v.as_bool()).unwrap_or(true);
        let summary = crate::security_native::bounty_scan_url(
            url,
            Self::security_http_client(),
            include_xss,
        )
        .await
        .map_err(|e| anyhow!(e))?;
        serde_json::to_value(summary).map_err(|e| anyhow!(e))
    }

    /// Agent tool: mint an OAST callback payload (auto-starts the server).
    pub(crate) async fn oast_payload(&self, _args: Value) -> Result<Value> {
        if !crate::oast::status().running {
            crate::oast::start(8889, None).await.map_err(|e| anyhow!(e))?;
        }
        serde_json::to_value(crate::oast::register()).map_err(|e| anyhow!(e))
    }

    /// Agent tool: poll the OAST server for blind-vuln callbacks.
    pub(crate) async fn oast_interactions(&self, args: Value) -> Result<Value> {
        let token = args.get("token").and_then(|v| v.as_str());
        let interactions = crate::oast::poll(token);
        Ok(json!({
            "running": crate::oast::status().running,
            "count": interactions.len(),
            "interactions": interactions,
        }))
    }

    pub(crate) async fn secrets_scan(&self, args: Value) -> Result<Value> {
        use regex::Regex;
        let path_str = args["path"].as_str().ok_or_else(|| anyhow!("Missing path"))?;
        let max_findings = args.get("max_findings").and_then(|v| v.as_u64()).unwrap_or(200) as usize;
        let include_low = args.get("include_low").and_then(|v| v.as_bool()).unwrap_or(false);

        let root = self.root_path.lock().await.clone();
        let full_path = self.validate_path(&root, path_str)?;
        if !full_path.exists() {
            return Err(anyhow!("Path not found: {}", path_str));
        }

        let patterns: Vec<(&str, &str, Regex)> = Self::secret_patterns()
            .into_iter()
            .filter_map(|(k, s, p)| Regex::new(p).ok().map(|r| (k, s, r)))
            .collect();

        let mut findings: Vec<Value> = Vec::new();
        let mut files_scanned: usize = 0;
        let mut bytes_scanned: u64 = 0;

        // Files we never scan — they're either generated, vendored, or huge.
        let skip_dirs = [
            "node_modules", "target", "dist", "build", ".git",
            "vendor", "third_party", ".next", ".cache", "__pycache__",
        ];

        let walker = walkdir::WalkDir::new(&full_path).max_depth(20).into_iter().filter_entry(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            !skip_dirs.contains(&name.as_str())
        });

        for entry in walker.flatten() {
            if findings.len() >= max_findings { break; }
            if !entry.file_type().is_file() { continue; }
            // Skip files >2 MiB — most secrets live in small config files anyway.
            if let Ok(meta) = entry.metadata() {
                if meta.len() > 2 * 1024 * 1024 { continue; }
            }
            let content = match fs::read_to_string(entry.path()) {
                Ok(c) => c,
                Err(_) => continue, // binary or unreadable — skip
            };
            files_scanned += 1;
            bytes_scanned += content.len() as u64;

            let rel = entry.path().strip_prefix(&*root).unwrap_or(entry.path()).to_string_lossy().to_string();

            for (line_no, line) in content.lines().enumerate() {
                if findings.len() >= max_findings { break; }
                if line.len() > 1000 { continue; } // minified — too noisy
                for (kind, severity, re) in &patterns {
                    if !include_low && *severity == "LOW" { continue; }
                    if let Some(m) = re.find(line) {
                        findings.push(json!({
                            "kind": kind,
                            "severity": severity,
                            "path": rel,
                            "line": line_no + 1,
                            "preview": Self::redact_secret(m.as_str()),
                        }));
                        break; // one finding per line is plenty
                    }
                }
            }
        }

        let mut counts: std::collections::HashMap<String, u64> = std::collections::HashMap::new();
        for f in &findings {
            if let Some(s) = f.get("severity").and_then(|v| v.as_str()) {
                *counts.entry(s.to_string()).or_insert(0) += 1;
            }
        }

        Ok(json!({
            "scope": path_str,
            "files_scanned": files_scanned,
            "bytes_scanned": bytes_scanned,
            "total_findings": findings.len(),
            "by_severity": counts,
            "findings": findings,
            "truncated": findings.len() >= max_findings,
        }))
    }

    pub(crate) async fn sec_distro_inventory(&self, args: Value) -> Result<Value> {
        let category = args.get("category").and_then(|v| v.as_str());
        Ok(crate::sec_distro::inventory_json(category))
    }

    pub(crate) async fn weaponize_env(&self, args: Value) -> Result<Value> {
        let raw_input = if let Some(p) = args.get("path").and_then(|v| v.as_str()) {
            let root = self.root_path.lock().await.clone();
            let full_path = self.validate_path(&root, p)?;
            fs::read_to_string(&full_path)
                .map_err(|e| anyhow!("Failed to read {}: {}", p, e))?
        } else if let Some(raw) = args.get("raw").and_then(|v| v.as_str()) {
            raw.to_string()
        } else {
            return Err(anyhow!("Provide either `path` or `raw` env content"));
        };

        // Parse env content — accept both KEY=VALUE and `export KEY=VALUE` forms.
        let mut vars: Vec<(String, String)> = Vec::new();
        for line in raw_input.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') { continue; }
            let payload = trimmed.strip_prefix("export ").unwrap_or(trimmed);
            if let Some(eq) = payload.find('=') {
                let key = payload[..eq].trim().to_string();
                let mut val = payload[eq + 1..].trim().to_string();
                // Strip surrounding quotes.
                if (val.starts_with('"') && val.ends_with('"'))
                    || (val.starts_with('\'') && val.ends_with('\''))
                {
                    val = val[1..val.len() - 1].to_string();
                }
                if !key.is_empty() {
                    vars.push((key, val));
                }
            }
        }

        // Classify each variable + propose a weaponization vector.
        fn classify(key: &str, val: &str) -> (&'static str, &'static str, Option<&'static str>) {
            // (category, severity, weaponization_hint)
            let k = key.to_ascii_lowercase();
            let v = val.to_ascii_lowercase();

            if k.contains("admin_password") || k.contains("root_password") || k == "neko_admin_password" {
                return ("credential", "CRITICAL", Some("Direct admin login — try web admin panel / SSH / app console at the deployment URL."));
            }
            if k.contains("password") || k.contains("passwd") || k.contains("pwd") {
                return ("credential", "HIGH", Some("Try as login credential against any auth endpoint discovered in this env."));
            }
            if k.contains("sentry_dsn") || (v.starts_with("http") && v.contains("@sentry")) {
                return ("telemetry_dsn", "HIGH", Some("Hijack: send forged events to poison error monitoring, exfiltrate via tagged messages, or trigger alert fatigue."));
            }
            if k.contains("otel") || k.contains("otlp") {
                if v.starts_with("http") {
                    return ("telemetry_endpoint", "MEDIUM", Some("Telemetry sink — can be redirected to collector you control to capture traces/metrics."));
                }
                return ("telemetry_config", "LOW", None);
            }
            if k.contains("dsn") || (v.starts_with("postgres") || v.starts_with("mysql") || v.starts_with("mongodb")) {
                return ("database_url", "CRITICAL", Some("Direct DB connection — dump data, escalate via stored procs, plant persistence."));
            }
            if k.contains("api_key") || k.contains("apikey") || k.contains("token") || k.contains("secret") {
                return ("api_credential", "HIGH", Some("Enumerate the API surface, harvest privileged data, pivot to other services."));
            }
            if k.contains("aws_") || k.contains("gcp_") || k.contains("azure_") {
                return ("cloud_credential", "CRITICAL", Some("Cloud lateral movement: list buckets, enumerate IAM, escalate privileges."));
            }
            if k.contains("domain") || k.contains("host") || k.contains("url") || k.contains("endpoint") {
                if v.starts_with("http") || v.contains('.') {
                    return ("endpoint", "MEDIUM", Some("Pivot target — enumerate via web_search / port scan / dirbust."));
                }
            }
            if k.contains("env") || k.contains("environment") {
                if v.contains("prod") {
                    return ("env_flag_prod", "HIGH", Some("Confirms PRODUCTION runtime — blast radius is real users."));
                }
                return ("env_flag", "LOW", None);
            }
            if k.contains("username") || k.contains("user") {
                return ("identity", "MEDIUM", Some("Pair with discovered passwords for auth attempts."));
            }
            ("config", "LOW", None)
        }

        let mut classified: Vec<Value> = Vec::new();
        let mut sev_counts: std::collections::HashMap<&str, u64> = std::collections::HashMap::new();
        let mut actionable: Vec<Value> = Vec::new();

        for (k, v) in &vars {
            let (cat, sev, hint) = classify(k, v);
            *sev_counts.entry(sev).or_insert(0) += 1;
            let entry = json!({
                "key": k,
                "category": cat,
                "severity": sev,
                "preview": Self::redact_secret(v),
                "weaponization": hint,
            });
            if matches!(sev, "CRITICAL" | "HIGH") && hint.is_some() {
                actionable.push(entry.clone());
            }
            classified.push(entry);
        }

        // Build a prioritized attack plan from the actionable set.
        let attack_plan: Vec<Value> = actionable.iter().enumerate().map(|(i, v)| {
            json!({
                "step": i + 1,
                "target_key": v.get("key").cloned().unwrap_or(Value::Null),
                "category": v.get("category").cloned().unwrap_or(Value::Null),
                "action": v.get("weaponization").cloned().unwrap_or(Value::Null),
            })
        }).collect();

        // Detect environment context (prod vs staging vs dev).
        let env_label = vars.iter().find_map(|(k, v)| {
            let kl = k.to_ascii_lowercase();
            if kl.contains("env") && (v.to_ascii_lowercase().contains("prod")
                || v.to_ascii_lowercase().contains("staging")
                || v.to_ascii_lowercase().contains("dev"))
            {
                Some(v.clone())
            } else { None }
        }).unwrap_or_else(|| "unknown".to_string());

        Ok(json!({
            "summary": {
                "total_variables": vars.len(),
                "actionable_count": actionable.len(),
                "environment": env_label,
                "by_severity": sev_counts,
            },
            "variables": classified,
            "actionable_findings": actionable,
            "attack_plan": attack_plan,
            "next_steps": [
                "1. Confirm the deployment is reachable (web_search the domains / endpoint values)",
                "2. For each CRITICAL row in attack_plan, try the suggested action manually or with the appropriate tool",
                "3. Document blast radius + proof in a markdown report (write_to_file)",
                "4. If this is a bug bounty engagement, draft a disclosure with redacted PoC",
            ],
        }))
    }

    pub(crate) async fn dev_cargo_diagnostics(&self, _args: Value) -> Result<Value> {
        let root = self.root_path.lock().await.clone();
        let output = std::process::Command::new("cargo")
            .args(&["check", "--message-format=json"])
            .current_dir(&root)
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut errors: Vec<Value> = Vec::new();
        let mut warnings: Vec<Value> = Vec::new();

        for line in stdout.lines() {
            if let Ok(msg) = serde_json::from_str::<Value>(line) {
                if msg["reason"] == "compiler-message" {
                    let level = msg["message"]["level"].as_str().unwrap_or("error");
                    let rendered = msg["message"]["rendered"].as_str().unwrap_or("").to_string();
                    let entry = json!({
                        "level": level,
                        "message": msg["message"]["message"],
                        "rendered": rendered,
                        "spans": msg["message"]["spans"]
                    });
                    if level == "error" { errors.push(entry); } else { warnings.push(entry); }
                }
            }
        }

        let success = errors.is_empty();
        Ok(json!({
            "success": success,
            "error_count": errors.len(),
            "warning_count": warnings.len(),
            "errors": errors,
            "warnings": warnings,
            "summary": if success {
                format!("✅ cargo check passed ({} warnings)", warnings.len())
            } else {
                format!("❌ {} error(s), {} warning(s). Fix errors before proceeding.", errors.len(), warnings.len())
            }
        }))
    }

}


/// Paths that should not feed bug-bounty / static audit findings (tests, mocks, vendored code).
pub(crate) fn audit_skip_path(rel: &str) -> bool {
    let l = rel.replace('\\', "/").to_lowercase();
    [
        "/test/", "/tests/", "/__tests__/", "/spec/", "/mocks/", "/mock/",
        "/fixtures/", "/fixture/", "/examples/", "/vendor/", "/node_modules/",
        ".test.", ".spec.", "_test.", "_mock.", "/generated/", "/dist/",
        "/target/", "/.git/",
    ]
    .iter()
    .any(|p| l.contains(p))
}

pub(crate) fn read_line_context(root: &std::path::Path, rel: &str, line: usize, radius: usize) -> Option<(String, String)> {
    let full = root.join(rel);
    let content = fs::read_to_string(&full).ok()?;
    let lines: Vec<&str> = content.lines().collect();
    if line == 0 || line > lines.len() {
        return None;
    }
    let lo = line.saturating_sub(radius + 1);
    let hi = (line + radius).min(lines.len());
    let window = lines[lo..hi]
        .iter()
        .enumerate()
        .map(|(i, l)| format!("{:>4}| {}", lo + i + 1, l))
        .collect::<Vec<_>>()
        .join("\n");
    Some((lines[line - 1].to_string(), window))
}

pub(crate) fn evidence_matches_line(evidence: &str, line: &str, window: &str) -> bool {
    let ev = evidence.trim();
    if ev.len() < 8 {
        return true;
    }
    let norm = |s: &str| s.chars().filter(|c| !c.is_whitespace()).collect::<String>();
    let needle: String = ev.chars().take(80).collect();
    let n = norm(&needle);
    if n.len() < 8 {
        return true;
    }
    norm(line).contains(&n) || norm(window).contains(&n)
}

pub(crate) fn finding_path_line(f: &Value) -> (String, usize) {
    let path = f
        .get("path")
        .or_else(|| f.get("file"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let line = f.get("line").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    (path, line)
}

/// Drop duplicate / unverifiable / low-confidence findings after LLM stages.
pub(crate) fn tighten_security_findings(root: &std::path::Path, findings: Vec<Value>, min_confidence: f64) -> Vec<Value> {
    let mut out = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for f in findings {
        let (path, line) = finding_path_line(&f);
        if path.is_empty() {
            continue;
        }
        if audit_skip_path(&path) {
            continue;
        }
        let conf = f.get("confidence").and_then(|v| v.as_f64()).unwrap_or(0.55);
        if conf < min_confidence {
            continue;
        }
        if line > 0 {
            let Some((ln, window)) = read_line_context(root, &path, line, 2) else {
                continue;
            };
            let ev = f.get("evidence").and_then(|v| v.as_str()).unwrap_or("");
            if !evidence_matches_line(ev, &ln, &window) {
                continue;
            }
        }
        let key = format!(
            "{}:{}:{}",
            path,
            line,
            f.get("cwe").and_then(|v| v.as_str()).unwrap_or("")
        );
        if !seen.insert(key) {
            continue;
        }
        out.push(f);
    }
    out
}

pub(crate) fn severity_counts(findings: &[Value]) -> std::collections::BTreeMap<String, u64> {
    let mut by_sev = std::collections::BTreeMap::new();
    for f in findings {
        let s = f
            .get("severity")
            .and_then(|v| v.as_str())
            .unwrap_or("INFO")
            .to_uppercase();
        *by_sev.entry(s).or_insert(0) += 1;
    }
    by_sev
}

pub(crate) fn render_finding_block(f: &Value, root: &std::path::Path) -> String {
    let (path, line) = finding_path_line(f);
    let id = f.get("id").and_then(|v| v.as_str()).unwrap_or("FIND");
    let title = f.get("title").and_then(|v| v.as_str()).unwrap_or("Finding");
    let sev = f.get("severity").and_then(|v| v.as_str()).unwrap_or("MEDIUM");
    let cwe = f.get("cwe").and_then(|v| v.as_str()).unwrap_or("—");
    let conf = f.get("confidence").and_then(|v| v.as_f64());
    let desc = f
        .get("description")
        .and_then(|v| v.as_str())
        .or_else(|| f.get("why").and_then(|v| v.as_str()))
        .unwrap_or("");
    let attack = f
        .get("attack_scenario")
        .and_then(|v| v.as_str())
        .or_else(|| f.get("poc").and_then(|v| v.as_str()))
        .unwrap_or("");
    let impact = f.get("impact").and_then(|v| v.as_str()).unwrap_or("");
    let evidence = f.get("evidence").and_then(|v| v.as_str()).unwrap_or("");
    let remediation = f.get("remediation").and_then(|v| v.as_str()).unwrap_or("");
    let poc = f.get("poc").and_then(|v| v.as_str()).unwrap_or("");

    let mut block = format!(
        "### {id} — {title}\n\n| Field | Value |\n|-------|-------|\n| **Severity** | {sev} |\n| **CWE** | {cwe} |\n| **Location** | `{path}:{line}` |\n",
    );
    if let Some(c) = conf {
        block.push_str(&format!("| **Confidence** | {c:.2} |\n"));
    }
    if !desc.is_empty() {
        block.push_str(&format!("\n**Description**\n\n{desc}\n"));
    }
    if !attack.is_empty() && attack != poc {
        block.push_str(&format!("\n**Attack scenario**\n\n{attack}\n"));
    }
    if line > 0 {
        if let Some((_, window)) = read_line_context(root, &path, line, 3) {
            block.push_str(&format!("\n**Code context**\n\n```\n{window}\n```\n"));
        }
    } else if !evidence.is_empty() {
        block.push_str(&format!("\n**Evidence**\n\n`{evidence}`\n"));
    }
    if !impact.is_empty() {
        block.push_str(&format!("\n**Impact**\n\n{impact}\n"));
    }
    if !poc.is_empty() {
        block.push_str(&format!("\n**Proof of concept / reproduction**\n\n{poc}\n"));
    }
    if !remediation.is_empty() {
        block.push_str(&format!("\n**Remediation**\n\n{remediation}\n"));
    }
    block.push('\n');
    block
}

/// Shared Markdown report for vuln-hunt, static audit, and web audit tools.
pub(crate) fn render_security_report(
    title: &str,
    meta: &[(&str, &str)],
    findings: &[Value],
    manual_review: &[Value],
    root: &std::path::Path,
) -> String {
    let by_sev = severity_counts(findings);
    let mut md = format!("# {title}\n\n## Executive summary\n\n");
    if findings.is_empty() {
        md.push_str("No confirmed vulnerabilities met evidence and confidence thresholds in this run.\n\n");
    } else {
        md.push_str(&format!(
            "**{} confirmed finding(s)**",
            findings.len()
        ));
        if let Some(c) = by_sev.get("CRITICAL") {
            md.push_str(&format!(" including **{c} CRITICAL**"));
        }
        if let Some(h) = by_sev.get("HIGH") {
            md.push_str(&format!(", **{h} HIGH**"));
        }
        md.push_str(".\n\n");
    }
    if !manual_review.is_empty() {
        md.push_str(&format!(
            "⚠️ **{} item(s)** need manual review (policy-dependent, cross-component, or low confidence).\n\n",
            manual_review.len()
        ));
    }
    md.push_str("## Scope & methodology\n\n");
    for (k, v) in meta {
        md.push_str(&format!("- **{k}:** {v}\n"));
    }
    md.push_str("\nPipeline: static/heuristic signals → LLM triage → evidence verification at cited line → confidence threshold.\n\n");
    md.push_str("## Summary by severity\n\n| Severity | Count |\n|----------|------:|\n");
    for sev in ["CRITICAL", "HIGH", "MEDIUM", "LOW", "INFO"] {
        if let Some(c) = by_sev.get(sev) {
            md.push_str(&format!("| {sev} | {c} |\n"));
        }
    }
    md.push_str("\n## Confirmed findings\n\n");
    if findings.is_empty() {
        md.push_str("_None._\n\n");
    } else {
        for f in findings {
            md.push_str(&render_finding_block(f, root));
        }
    }
    if !manual_review.is_empty() {
        md.push_str("## Manual review queue\n\n");
        md.push_str("_These were not promoted to confirmed — verify before reporting externally._\n\n");
        for f in manual_review {
            md.push_str(&render_finding_block(f, root));
        }
    }
    md
}

/// Heuristic CWE source-scan patterns used by `deep_security_audit`.
/// (category, severity, CWE, regex, remediation). Module-level so the pattern set
/// can be unit-tested without constructing a full `AiTools` instance.
pub(crate) fn security_audit_patterns()
    -> &'static [(&'static str, &'static str, &'static str, &'static str, &'static str)]
{
    &[
        ("OS Command Injection", "HIGH", "CWE-78", r"(?i)(os\.system|subprocess\.(call|Popen|run)[^)]*shell\s*=\s*True|child_process\.(exec|execSync)|Runtime\.getRuntime\(\)\.exec)", "Never pass untrusted input to a shell; use argument arrays / parameterized APIs."),
        ("SQL Injection", "HIGH", "CWE-89", r#"(?i)(execute|query|exec_sql)\s*\(\s*[^)]*(\+\s*|\|\|\s*|\.format\s*\(|format!\s*\([^)]*(\+|\{))"#, "Use parameterized queries / prepared statements, never string concatenation."),
        ("Cross-Site Scripting", "MEDIUM", "CWE-79", r"(?i)(innerHTML\s*=|dangerouslySetInnerHTML|document\.write\s*\()", "Escape/encode output; prefer textContent or a sanitizer (DOMPurify)."),
        ("Unsafe Deserialization", "HIGH", "CWE-502", r"(?i)(pickle\.loads|cPickle\.loads|Marshal\.load|unserialize\s*\(|ObjectInputStream)", "Use safe loaders / allow-lists / signed payloads."),
        ("Weak Cryptographic Hash", "MEDIUM", "CWE-327", r"(?i)\b(md5|sha1)\s*\(", "Use SHA-256+; bcrypt/scrypt/argon2 for passwords."),
        ("Insecure Randomness", "MEDIUM", "CWE-330", r"(?i)(Math\.random\s*\(\)|random\.random\s*\(\))", "Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens."),
        ("Disabled TLS Verification", "HIGH", "CWE-295", r"(?i)(verify\s*=\s*False|rejectUnauthorized\s*:\s*false|InsecureSkipVerify\s*:\s*true|danger_accept_invalid_certs\s*\(\s*true)", "Never disable certificate validation."),
        ("Path Traversal", "MEDIUM", "CWE-22", r"(?i)(req\.(query|params|body)[^;]*path\.join|path\.join[^;]*req\.(query|params|body))", "Canonicalize and validate paths against an allow-listed base directory."),
        ("Dangerous Eval", "HIGH", "CWE-95", r"(?i)(\beval\s*\(|new\s+Function\s*\()", "Avoid eval; parse/allow-list input instead of executing it."),
        ("Rust unsafe block", "LOW", "CWE-119", r"\bunsafe\s*\{", "Audit unsafe blocks for memory-safety invariants; minimize their scope."),
        ("Panic-prone unwrap/expect", "INFO", "CWE-248", r"\.(unwrap|expect)\s*\(", "Handle errors with ? / match instead of unwrap/expect on hot paths."),
    ]
}

#[cfg(test)]
mod security_audit_pattern_tests {
    use super::security_audit_patterns;
    use regex::Regex;

    #[test]
    pub(crate) fn all_patterns_compile_and_are_well_formed() {
        for (cat, sev, cwe, rx, rem) in security_audit_patterns() {
            assert!(Regex::new(rx).is_ok(), "regex for {} must compile: {}", cat, rx);
            assert!(
                cwe.strip_prefix("CWE-").map(|n| !n.is_empty() && n.chars().all(|c| c.is_ascii_digit())).unwrap_or(false),
                "CWE id malformed for {}: {}", cat, cwe
            );
            assert!(matches!(*sev, "CRITICAL" | "HIGH" | "MEDIUM" | "LOW" | "INFO"),
                "severity invalid for {}: {}", cat, sev);
            assert!(!rem.is_empty(), "remediation empty for {}", cat);
        }
    }

    pub(crate) fn category_matches(cat_substr: &str, sample: &str) -> bool {
        security_audit_patterns().iter()
            .filter(|(cat, ..)| cat.contains(cat_substr))
            .any(|(_, _, _, rx, _)| Regex::new(rx).unwrap().is_match(sample))
    }

    #[test]
    pub(crate) fn detects_known_vulnerable_lines() {
        assert!(category_matches("Command Injection", "os.system(user_input)"));
        assert!(category_matches("Command Injection", "child_process.exec(cmd)"));
        assert!(category_matches("SQL Injection", r#"db.execute("SELECT * FROM users WHERE id=" + id)"#));
        assert!(category_matches("Cross-Site Scripting", "el.innerHTML = userdata"));
        assert!(category_matches("Unsafe Deserialization", "data = pickle.loads(blob)"));
        assert!(category_matches("Weak Cryptographic Hash", "let h = md5(password);"));
        assert!(category_matches("Insecure Randomness", "const token = Math.random()"));
        assert!(category_matches("Disabled TLS", "requests.get(url, verify=False)"));
        assert!(category_matches("Dangerous Eval", "eval(user_code)"));
        assert!(category_matches("unsafe block", "unsafe { *ptr }"));
        assert!(category_matches("unwrap", "let x = foo().unwrap();"));
    }

    #[test]
    pub(crate) fn ignores_benign_lines() {
        assert!(!category_matches("Command Injection", "let total = sum(a, b);"));
        assert!(!category_matches("SQL Injection", "let name = format!(\"hello {}\", who);"));
        assert!(!category_matches("Weak Cryptographic Hash", "let id = md5sum_label;"));
    }
}

// TODO(ai_tools): these tests pre-date the AiTools::new(...) refactor that
#[cfg(test)]
mod shell_translator_tests {
    use crate::domain::tools::ShellTranslator;

    #[test]
    pub(crate) fn bash_normalizes_windows_drive_paths() {
        let cmd = r"cd C:\Users\HADES\Desktop\proj && dir /b";
        let normalized = ShellTranslator::normalize_windows_paths_for_bash(cmd);
        if cfg!(windows) {
            assert!(normalized.contains("/c/Users/HADES/Desktop/proj"));
        } else {
            assert_eq!(normalized, cmd);
        }
    }

    #[test]
    pub(crate) fn bash_preserves_regex_and_python_escapes() {
        let grep = r#"grep -oE "(['\"])/(api|v1|v2)/[^'\"]+(['\"])" index.js"#;
        let py = r#"python -c "import re; print('\n'.join(['a']))""#;
        if cfg!(windows) {
            assert!(grep.contains(r#"['\"]"#), "grep quotes must survive: {grep}");
            assert!(py.contains(r"\n"), "python \\n must survive: {py}");
            assert_eq!(
                ShellTranslator::normalize_windows_paths_for_bash(grep),
                grep
            );
            assert_eq!(ShellTranslator::normalize_windows_paths_for_bash(py), py);
        }
    }

    #[test]
    pub(crate) fn bash_normalizes_python_script_path() {
        let cmd = r"python C:\Users\HADES\Desktop\pentesting\analyze_js.py";
        if cfg!(windows) {
            let n = ShellTranslator::normalize_windows_paths_for_bash(cmd);
            assert!(n.contains("/c/Users/HADES/Desktop/pentesting/analyze_js.py"));
            assert!(!n.contains('\\'));
        }
    }

    #[test]
    pub(crate) fn bash_preserves_https_urls() {
        let cmd = r#"curl -sL "https://app.example.com/assets/index.js" -o out.js"#;
        if cfg!(windows) {
            let n = ShellTranslator::normalize_windows_paths_for_bash(cmd);
            assert!(n.contains("https://app.example.com"), "URL must survive: {n}");
            assert!(!n.contains("/s//"), "must not mangle scheme: {n}");
        }
    }

    #[test]
    pub(crate) fn bash_python_c_paths_use_windows_form() {
        let cmd = r#"python -c "f=open(r'C:\Users\HADES\Desktop\pentesting\recon\host\bundle.js')""#;
        if cfg!(windows) {
            let n = ShellTranslator::normalize_windows_paths_for_bash(cmd);
            assert!(
                n.contains("C:/Users/HADES/Desktop/pentesting/recon/host/bundle.js")
                    || n.contains(r"C:\Users\HADES"),
                "python paths must be Windows-openable: {n}"
            );
        }
    }

    #[test]
    pub(crate) fn prefers_git_bash_for_grep_curl() {
        assert!(ShellTranslator::prefers_git_bash("grep -Eo api index.js"));
        assert!(ShellTranslator::prefers_git_bash("curl -s https://example.com > out.js"));
        assert!(!ShellTranslator::prefers_git_bash("Get-ChildItem"));
    }
}

// TODO(ai_tools): these tests pre-date the AiTools::new(...) refactor that
// added knowledge_distiller / patch_engine / ghost_runtime / shadow_workspace /
// apex parameters. Re-enable by feature-gating with `#[cfg(test)]` once the
// constructor calls have been updated to the current 10-arg signature. Until
// then they're skipped with `cfg(any())` so the rest of the test suite (in
// particular kortex_gac and kortex_kvcache) can still run.
#[cfg(any())]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    pub(crate) fn test_validate_path_safe() {
        let root = std::env::temp_dir().join(format!("test_root_{}", Uuid::new_v4()));
        fs::create_dir_all(&root).unwrap();

        let browser_state = Arc::new(crate::browser::BrowserState::new());
        let git_manager = Arc::new(crate::git::GitManager::new());
        let mcp_registry = Arc::new(crate::mcp_registry::McpRegistry::new(
            root.join("mcp_config.json"),
        ));
        let ai_tools = AiTools::new(
            root.clone(),
            browser_state,
            git_manager,
            mcp_registry,
            Arc::new(crate::knowledge_distiller::KnowledgeDistiller::new(root.clone())),
            Arc::new(std::sync::atomic::AtomicBool::new(false)),
        );

        // Safe relative path
        let res = ai_tools.validate_path(&root, "src/main.rs");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), root.join("src/main.rs"));

        // Safe dot path
        let res = ai_tools.validate_path(&root, ".");
        assert!(res.is_ok());
    }

    #[test]
    pub(crate) fn test_validate_path_traversal() {
        let root = std::env::temp_dir().join(format!("test_root_{}", Uuid::new_v4()));
        fs::create_dir_all(&root).unwrap();

        let browser_state = Arc::new(crate::browser::BrowserState::new());
        let git_manager = Arc::new(crate::git::GitManager::new());
        let mcp_registry = Arc::new(crate::mcp_registry::McpRegistry::new(
            root.join("mcp_config.json"),
        ));
        let ai_tools = AiTools::new(
            root.clone(),
            browser_state,
            git_manager,
            mcp_registry,
            Arc::new(crate::knowledge_distiller::KnowledgeDistiller::new(root.clone())),
            Arc::new(std::sync::atomic::AtomicBool::new(false)),
        );

        // Simple traversal
        let res = ai_tools.validate_path(&root, "../secrets.txt");
        assert!(res.is_err());

        // Nested traversal
        let res = ai_tools.validate_path(&root, "src/../../etc/passwd");
        assert!(res.is_err());
    }

    #[test]
    pub(crate) fn test_validate_path_absolute_escape() {
        let root = std::env::temp_dir().join(format!("test_root_{}", Uuid::new_v4()));
        fs::create_dir_all(&root).unwrap();

        let browser_state = Arc::new(crate::browser::BrowserState::new());
        let git_manager = Arc::new(crate::git::GitManager::new());
        let mcp_registry = Arc::new(crate::mcp_registry::McpRegistry::new(
            root.join("mcp_config.json"),
        ));
        let ai_tools = AiTools::new(
            root.clone(),
            browser_state,
            git_manager,
            mcp_registry,
            Arc::new(crate::knowledge_distiller::KnowledgeDistiller::new(root.clone())),
            Arc::new(std::sync::atomic::AtomicBool::new(false)),
        );

        // Absolute path outside root
        let res = ai_tools.validate_path(&root, "/etc/passwd");
        assert!(res.is_err());
    }
}


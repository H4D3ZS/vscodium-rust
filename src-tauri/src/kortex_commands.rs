use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::cmp::Reverse;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};
use walkdir::WalkDir;

fn find_json_end(bytes: &[u8]) -> Option<usize> {
    let mut brace_count = 0;
    let mut in_string = false;
    let mut escaped = false;

    for (i, &b) in bytes.iter().enumerate() {
        if escaped {
            escaped = false;
            continue;
        }
        match b {
            b'\\' => escaped = true,
            b'"' => in_string = !in_string,
            b'{' if !in_string => brace_count += 1,
            b'}' if !in_string => {
                brace_count -= 1;
                if brace_count == 0 {
                    return Some(i + 1);
                }
            }
            _ => {}
        }
    }
    None
}

#[tauri::command]
pub async fn load_kortex_memory(path: String) -> Result<Vec<f32>, String> {
    let path_buf = PathBuf::from(&path);
    if !path_buf.exists() {
        return Ok(vec![0.0; 1536]);
    }

    let bytes = fs::read(&path_buf).map_err(|e| e.to_string())?;

    let header_end = find_json_end(&bytes)
        .ok_or_else(|| "Invalid .aim format: No JSON header found".to_string())?;

    if header_end + (1536 * 4) > bytes.len() {
        return Ok(vec![0.0; 1536]);
    }

    let tensor_bytes = &bytes[header_end..header_end + (1536 * 4)];
    let mut tensor = Vec::with_capacity(1536);
    for chunk in tensor_bytes.chunks_exact(4) {
        let val = f32::from_le_bytes(chunk.try_into().unwrap());
        tensor.push(val);
    }

    Ok(tensor)
}

#[tauri::command]
pub async fn load_kortex_metadata(path: String) -> Result<Value, String> {
    let path_buf = PathBuf::from(&path);
    if !path_buf.exists() {
        return Ok(json!({}));
    }

    let bytes = fs::read(&path_buf).map_err(|e| e.to_string())?;

    let header_end = find_json_end(&bytes)
        .ok_or_else(|| "Invalid .aim format: No JSON header found".to_string())?;

    let header_json = String::from_utf8_lossy(&bytes[..header_end]);
    let metadata: Value = serde_json::from_str(&header_json).map_err(|e| e.to_string())?;

    Ok(metadata)
}

// ── AIM Store Tauri Commands ─────────────────────────────────────────────────

/// Upsert a semantic gist into the workspace's `.aim/gists.aim` store.
#[tauri::command]
pub async fn aim_upsert_gist(
    root: String,
    key: String,
    gist: String,
    weight: f32,
) -> Result<(), String> {
    let aim_path = PathBuf::from(&root).join(".aim").join("gists.aim");
    let mut store = crate::aim_store::AimStore::load(&aim_path);

    // Determine file mtime for the key if it maps to a real path.
    let mtime = {
        let p = PathBuf::from(&root).join(&key);
        std::fs::metadata(&p)
            .and_then(|m| m.modified())
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0)
    };

    store.insert(key, gist, mtime, weight.clamp(0.0, 1.0));
    store.save(&aim_path).map_err(|e| e.to_string())
}

/// Load all gists from the workspace's `.aim/gists.aim` store.
#[tauri::command]
pub async fn aim_load_gists(root: String) -> Result<Vec<serde_json::Value>, String> {
    let aim_path = PathBuf::from(&root).join(".aim").join("gists.aim");
    let store = crate::aim_store::AimStore::load(&aim_path);
    let entries: Vec<serde_json::Value> = store.entries.values().map(|e| {
        serde_json::json!({
            "key": e.key,
            "gist": e.gist,
            "mtime": e.mtime,
            "weight": e.weight,
        })
    }).collect();
    Ok(entries)
}

/// Evict stale gist entries (file mtime changed since indexing).
#[tauri::command]
pub async fn aim_invalidate_stale(root: String) -> Result<usize, String> {
    let aim_path = PathBuf::from(&root).join(".aim").join("gists.aim");
    let mut store = crate::aim_store::AimStore::load(&aim_path);
    let before = store.entries.len();
    store.invalidate_stale(&PathBuf::from(&root));
    let evicted = before - store.entries.len();
    if evicted > 0 {
        store.save(&aim_path).map_err(|e| e.to_string())?;
    }
    Ok(evicted)
}

/// Atomic VFS write — copy-on-write semantics, crash-safe.
#[tauri::command]
pub async fn vfs_write_atomic(root: String, relative_path: String, content: String) -> Result<(), String> {
    let bridge = crate::vfs_bridge::VfsBridge::new(PathBuf::from(&root));
    bridge.write_atomic(std::path::Path::new(&relative_path), &content)
        .map_err(|e| e.to_string())
}

/// Surgical patch: search/replace with copy-on-write. Returns new content.
#[tauri::command]
pub async fn vfs_apply_patch(
    root: String,
    relative_path: String,
    search: String,
    replace: String,
) -> Result<String, String> {
    let bridge = crate::vfs_bridge::VfsBridge::new(PathBuf::from(&root));
    bridge.apply_patch(std::path::Path::new(&relative_path), &search, &replace)
        .map_err(|e| e.to_string())
}

fn default_workspace_aim_path(root: Option<String>) -> PathBuf {
    root.map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
        .join(".aim")
        .join("memory.aim")
}

fn file_modified_unix_ms(path: &Path) -> Option<u128> {
    fs::metadata(path)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_millis())
}

fn now_unix_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
}

fn run_git(root: &Path, args: &[&str]) -> Option<String> {
    Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .ok()
        .filter(|out| out.status.success())
        .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_string())
        .filter(|s| !s.is_empty())
}

#[tauri::command]
pub async fn aim_trust_manifest(
    path: Option<String>,
    root: Option<String>,
) -> Result<Value, String> {
    let root_path = root
        .clone()
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    let aim_path = path
        .map(PathBuf::from)
        .unwrap_or_else(|| default_workspace_aim_path(root));

    if !aim_path.exists() {
        return Ok(json!({
            "schema": "kortex.aim.trust/v1",
            "status": "missing",
            "path": aim_path,
            "root": root_path,
            "valid": false,
            "confidence": 0.0,
            "reasons": ["AIM memory file does not exist"],
        }));
    }

    let bytes = fs::read(&aim_path).map_err(|e| e.to_string())?;
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let sha256 = format!("{:x}", hasher.finalize());
    let header_end = find_json_end(&bytes);
    let metadata = if let Some(end) = header_end {
        serde_json::from_slice::<Value>(&bytes[..end]).unwrap_or_else(|_| json!({}))
    } else {
        json!({})
    };

    let size_bytes = bytes.len();
    let modified_unix_ms = file_modified_unix_ms(&aim_path);
    let age_ms = modified_unix_ms.map(|m| now_unix_ms().saturating_sub(m));
    let head = run_git(&root_path, &["rev-parse", "HEAD"]);
    let dirty_files = run_git(&root_path, &["status", "--porcelain"])
        .map(|s| s.lines().count())
        .unwrap_or(0);

    let mut reasons: Vec<String> = Vec::new();
    if header_end.is_none() {
        reasons.push("AIM header is not valid JSON".to_string());
    }
    if size_bytes < 1536 * 4 {
        reasons.push("AIM file is too small to contain a full 1536-dim gist tensor".to_string());
    }
    if dirty_files > 0 {
        reasons.push(format!(
            "Workspace has {} dirty file(s); gist may be behind current edits",
            dirty_files
        ));
    }
    if let Some(age) = age_ms {
        let seven_days_ms = 7 * 24 * 60 * 60 * 1000u128;
        if age > seven_days_ms {
            reasons.push("AIM file is older than 7 days".to_string());
        }
    }

    let valid = header_end.is_some() && size_bytes >= 1536 * 4;
    let mut confidence = if valid { 0.92 } else { 0.25 };
    if dirty_files > 0 {
        confidence -= 0.18;
    }
    if age_ms
        .map(|age| age > 7 * 24 * 60 * 60 * 1000u128)
        .unwrap_or(false)
    {
        confidence -= 0.12;
    }
    if confidence < 0.0 {
        confidence = 0.0;
    }

    Ok(json!({
        "schema": "kortex.aim.trust/v1",
        "status": if valid { "ready" } else { "degraded" },
        "path": aim_path,
        "root": root_path,
        "valid": valid,
        "confidence": confidence,
        "reasons": reasons,
        "sha256": sha256,
        "size_bytes": size_bytes,
        "modified_unix_ms": modified_unix_ms,
        "age_ms": age_ms,
        "header_end": header_end,
        "metadata": metadata,
        "git": {
            "head": head,
            "dirty_files": dirty_files,
        },
    }))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AimSpanQuery {
    pub query: String,
    #[serde(default)]
    pub root: Option<String>,
    #[serde(default)]
    pub limit: Option<usize>,
    #[serde(default)]
    pub max_files: Option<usize>,
    #[serde(default)]
    pub max_bytes_per_file: Option<u64>,
}

fn tokenize_query(query: &str) -> Vec<String> {
    query
        .split(|c: char| !c.is_alphanumeric() && c != '_' && c != '-')
        .map(|s| s.trim().to_ascii_lowercase())
        .filter(|s| s.len() >= 2)
        .take(16)
        .collect()
}

fn is_probably_source_file(path: &Path, max_bytes: u64) -> bool {
    if !path.is_file() {
        return false;
    }
    let s = path.to_string_lossy();
    let pruned = [
        "node_modules",
        ".git",
        "\\target\\",
        "/target/",
        "\\dist\\",
        "/dist/",
        "\\build\\",
        "/build/",
        "\\.cache\\",
        "/.cache/",
        "\\.next\\",
        "/.next/",
        "\\.turbo\\",
        "/.turbo/",
        "\\vendor\\",
        "/vendor/",
        "\\__pycache__\\",
        "/__pycache__/",
        "\\manus_ai_source_code\\",
        "/manus_ai_source_code/",
        "\\Antigravity IDE\\",
        "/Antigravity IDE/",
        "\\void\\",
        "/void/",
        "\\invisible_playwright\\",
        "/invisible_playwright/",
    ];
    if pruned.iter().any(|seg| s.contains(seg)) {
        return false;
    }
    if let Ok(meta) = path.metadata() {
        if meta.len() == 0 || meta.len() > max_bytes {
            return false;
        }
    }
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    let allowed = [
        "rs", "ts", "tsx", "js", "jsx", "json", "toml", "md", "vue", "svelte", "py", "css", "scss",
        "html", "yml", "yaml",
    ];
    allowed.contains(&ext.as_str())
}

fn line_window(lines: &[&str], center: usize, radius: usize) -> (usize, usize, String) {
    let start = center.saturating_sub(radius);
    let end = (center + radius + 1).min(lines.len());
    (start + 1, end, lines[start..end].join("\n"))
}

fn load_aim_index(
    root: &Path,
) -> Option<(Vec<crate::memory_store::SymbolDefinition>, Vec<String>)> {
    let aim_path = root.join(".aim").join("memory.aim");
    let bytes = fs::read(aim_path).ok()?;
    let header_end = find_json_end(&bytes)?;
    let header = serde_json::from_slice::<Value>(&bytes[..header_end]).ok()?;
    let kortex = header.get("kortex")?;
    let symbols = serde_json::from_value::<Vec<crate::memory_store::SymbolDefinition>>(
        kortex
            .get("symbol_graph")
            .and_then(|g| g.get("definitions"))
            .cloned()
            .unwrap_or_else(|| json!([])),
    )
    .unwrap_or_default();
    let tree = serde_json::from_value::<Vec<String>>(
        kortex
            .get("project_tree")
            .cloned()
            .unwrap_or_else(|| json!([])),
    )
    .unwrap_or_default();
    Some((symbols, tree))
}

fn score_text_match(text: &str, terms: &[String], weight: i64) -> i64 {
    let text_lc = text.to_ascii_lowercase();
    terms
        .iter()
        .filter(|term| text_lc.contains(term.as_str()))
        .count() as i64
        * weight
}

fn indexed_candidate_paths(root: &Path, terms: &[String], limit: usize) -> (Vec<PathBuf>, usize) {
    let Some((symbols, tree)) = load_aim_index(root) else {
        return (Vec::new(), 0);
    };

    let mut scores: HashMap<String, i64> = HashMap::new();
    for sym in &symbols {
        let name_score = score_text_match(&sym.name, terms, 18);
        let path_score = score_text_match(&sym.path, terms, 10);
        let kind_score = score_text_match(&sym.kind, terms, 4);
        let score = name_score + path_score + kind_score;
        if score > 0 {
            *scores.entry(sym.path.clone()).or_insert(0) += score + 6;
        }
    }
    for path in &tree {
        let score = score_text_match(path, terms, 8);
        if score > 0 {
            *scores.entry(path.clone()).or_insert(0) += score;
        }
    }

    let mut ranked: Vec<(String, i64)> = scores.into_iter().collect();
    ranked.sort_by_key(|(_, score)| Reverse(*score));
    let index_hits = ranked.len();
    let paths = ranked
        .into_iter()
        .take(limit)
        .map(|(path, _)| root.join(path))
        .collect();
    (paths, index_hits)
}

fn scan_file_for_span(path: &Path, root: &Path, terms: &[String], max_bytes: u64) -> Option<Value> {
    if !is_probably_source_file(path, max_bytes) {
        return None;
    }

    let rel = path
        .strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/");
    let rel_lc = rel.to_ascii_lowercase();
    let path_score = terms.iter().filter(|t| rel_lc.contains(t.as_str())).count() as i64 * 8;

    let content = fs::read_to_string(path).ok()?;
    let content_lc = content.to_ascii_lowercase();
    if path_score == 0 && !terms.iter().any(|t| content_lc.contains(t.as_str())) {
        return None;
    }

    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return None;
    }

    let mut best_line = 0usize;
    let mut best_score = path_score;
    for (idx, line) in lines.iter().enumerate() {
        let line_lc = line.to_ascii_lowercase();
        let line_hits = terms
            .iter()
            .filter(|t| line_lc.contains(t.as_str()))
            .count() as i64;
        if line_hits == 0 {
            continue;
        }
        let symbol_boost = if line_lc.contains("function ")
            || line_lc.contains(" fn ")
            || line_lc.starts_with("fn ")
            || line_lc.contains("class ")
            || line_lc.contains("struct ")
            || line_lc.contains("interface ")
            || line_lc.contains("export ")
        {
            4
        } else {
            0
        };
        let score = path_score + (line_hits * 12) + symbol_boost;
        if score > best_score {
            best_score = score;
            best_line = idx;
        }
    }

    if best_score <= 0 {
        return None;
    }

    let (line_start, line_end, snippet) = line_window(&lines, best_line, 8);
    let mut hasher = Sha256::new();
    hasher.update(snippet.as_bytes());
    let span_hash = format!("{:x}", hasher.finalize());

    Some(json!({
        "file": rel,
        "absolute_path": path,
        "line_start": line_start,
        "line_end": line_end,
        "score": best_score,
        "hash": span_hash,
        "summary": lines.get(best_line).unwrap_or(&"").trim(),
        "snippet": snippet,
    }))
}

#[tauri::command]
pub async fn aim_query_spans(request: AimSpanQuery) -> Result<Value, String> {
    let terms = tokenize_query(&request.query);
    if terms.is_empty() {
        return Ok(json!({
            "schema": "kortex.aim.spans/v1",
            "query": request.query,
            "spans": [],
            "scanned_files": 0,
            "reason": "No usable query terms",
        }));
    }

    let root = request
        .root
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    let limit = request.limit.unwrap_or(8).clamp(1, 32);
    let max_files = request.max_files.unwrap_or(2000).clamp(50, 20_000);
    let max_bytes = request
        .max_bytes_per_file
        .unwrap_or(512 * 1024)
        .clamp(4096, 2 * 1024 * 1024);

    let mut candidates: Vec<Value> = Vec::new();
    let mut scanned = 0usize;
    let (indexed_paths, index_hits) = indexed_candidate_paths(&root, &terms, max_files.min(256));
    let mut visited: HashSet<PathBuf> = HashSet::new();

    for path in indexed_paths {
        if !visited.insert(path.clone()) {
            continue;
        }
        scanned += 1;
        if let Some(span) = scan_file_for_span(&path, &root, &terms, max_bytes) {
            candidates.push(span);
        }
        if candidates.len() >= limit {
            break;
        }
    }

    if candidates.len() < limit {
        for entry in WalkDir::new(&root)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path().to_path_buf();
            if !visited.insert(path.clone()) {
                continue;
            }
            if !is_probably_source_file(&path, max_bytes) {
                continue;
            }
            scanned += 1;
            if scanned > max_files {
                break;
            }
            if let Some(span) = scan_file_for_span(&path, &root, &terms, max_bytes) {
                candidates.push(span);
            }
        }
    }

    candidates.sort_by(|a, b| {
        b.get("score")
            .and_then(|v| v.as_i64())
            .unwrap_or(0)
            .cmp(&a.get("score").and_then(|v| v.as_i64()).unwrap_or(0))
    });
    candidates.truncate(limit);

    Ok(json!({
        "schema": "kortex.aim.spans/v1",
        "query": request.query,
        "root": root,
        "terms": terms,
        "scanned_files": scanned,
        "index_hits": index_hits,
        "source": if index_hits > 0 { "aim-index+source" } else { "source-scan" },
        "limit": limit,
        "spans": candidates,
    }))
}

#[tauri::command]
pub async fn save_kortex_memory(
    path: String,
    gist_token: Vec<f32>,
    metadata: Value,
) -> Result<(), String> {
    let path_buf = PathBuf::from(&path);
    if let Some(parent) = path_buf.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
    }

    // Ensure we have a valid JSON header
    let header_json = serde_json::to_string(&metadata).map_err(|e| e.to_string())?;

    let mut bytes = header_json.into_bytes();

    for val in gist_token {
        bytes.extend_from_slice(&val.to_le_bytes());
    }

    fs::write(&path_buf, bytes).map_err(|e| e.to_string())?;
    println!("[KORTEX] Saved consciousness to {}", path);

    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────
//  Kortex telemetry .aim VFS hookup
// ─────────────────────────────────────────────────────────────────────────
//
// The `.aim` files are the durable side of the neural VFS: gist tokens live
// there, and now so do the inference telemetry traces (tok/s, prefill ms,
// cache hit rate, model identity) that the IDE just started collecting.
//
// File format: same shape as the gist .aim → a JSON header that describes
// the stream + the raw byte payload. Here the payload is a tightly packed
// ring buffer of `TelemetrySample`s, JSON-encoded as a single array; the
// gist .aim's binary tensor payload is unrelated, so we keep them in
// separate files (`telemetry.aim`, `airi_memory.aim`).

/// One inference completion as recorded by the frontend services. Mirrors
/// `CompletionSample` in `src/kortex/throughput.ts` so the disk format is a
/// trivial 1:1 of what's already in the renderer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetrySample {
    pub wall_clock_ms: u64,
    #[serde(default)]
    pub prefill_ms: Option<u64>,
    pub output_tokens: u64,
    pub input_tokens: u64,
    pub backend: String,
    pub cache_hit: bool,
    pub tokens_skipped: u64,
    #[serde(default)]
    pub model_id: Option<String>,
    pub ts_unix_ms: u64,
}

/// Header for `telemetry.aim`. Includes a schema version so v1 readers can
/// gracefully degrade when v2 adds fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TelemetryAimHeader {
    /// "kortex.telemetry/v1"
    schema: String,
    /// Max number of samples we'll keep in the ring buffer on disk. Older
    /// samples get evicted FIFO. Default 4096 ≈ days of normal use at a few
    /// requests per minute.
    capacity: usize,
    /// Lifetime tally — survives ring-buffer eviction so the UI can show
    /// "you've routed 1.4M tokens through Kortex" even after old samples
    /// roll off.
    lifetime_samples: u64,
    lifetime_output_tokens: u64,
    lifetime_input_tokens: u64,
    lifetime_tokens_skipped: u64,
    lifetime_cache_hits: u64,
    /// Tracks the most recent KDKVC model identity stamp, so the user can
    /// confirm "my .aim VFS believes the cache is bound to this model" even
    /// when the proxy isn't running right now.
    #[serde(default)]
    last_model_id: Option<String>,
    #[serde(default)]
    last_quant_signature: Option<String>,
    #[serde(default)]
    last_tokenizer_hash: Option<String>,
}

impl Default for TelemetryAimHeader {
    fn default() -> Self {
        Self {
            schema: "kortex.telemetry/v1".to_string(),
            capacity: 4096,
            lifetime_samples: 0,
            lifetime_output_tokens: 0,
            lifetime_input_tokens: 0,
            lifetime_tokens_skipped: 0,
            lifetime_cache_hits: 0,
            last_model_id: None,
            last_quant_signature: None,
            last_tokenizer_hash: None,
        }
    }
}

/// In-memory mirror of the disk file. We hold it in a global Mutex so the
/// recordCompletion path doesn't have to do disk IO on every token round —
/// instead we flush opportunistically (every N samples, on stop, on shutdown).
struct TelemetryState {
    header: TelemetryAimHeader,
    samples: VecDeque<TelemetrySample>,
    /// Number of samples appended since the last successful disk flush. Used
    /// to decide when to rewrite the file.
    dirty_count: u32,
    /// Absolute path of `telemetry.aim` once it's been resolved.
    path: Option<PathBuf>,
}

impl TelemetryState {
    fn new() -> Self {
        Self {
            header: TelemetryAimHeader::default(),
            samples: VecDeque::with_capacity(4096),
            dirty_count: 0,
            path: None,
        }
    }
}

static TELEMETRY: OnceLock<Mutex<TelemetryState>> = OnceLock::new();

fn telemetry_state() -> &'static Mutex<TelemetryState> {
    TELEMETRY.get_or_init(|| Mutex::new(TelemetryState::new()))
}

/// Resolve the default telemetry.aim path. Lives next to the gist .aim
/// (`<userprofile>/.kortex/telemetry.aim`) so a single directory contains
/// the whole neural VFS state.
fn default_telemetry_aim_path() -> PathBuf {
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".kortex").join("telemetry.aim")
}

/// Serialise the header + sample ring into the .aim envelope format used
/// elsewhere (JSON header followed by payload). Payload here is a UTF-8 JSON
/// array of samples — easy to debug, and at 4096 entries × ~150 B/entry
/// (~600 KB) it's well under the file-size limits we care about.
fn write_telemetry_aim(path: &Path, state: &TelemetryState) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }
    let header_bytes = serde_json::to_vec(&state.header)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    let samples_vec: Vec<&TelemetrySample> = state.samples.iter().collect();
    let payload_bytes = serde_json::to_vec(&samples_vec)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    // Two-step atomic write: write to .tmp, fsync, rename. Same pattern as
    // KDKVC's index files; protects against partial writes if the process
    // crashes mid-flush.
    let tmp = path.with_extension("aim.tmp");
    {
        use std::io::Write;
        let mut f = fs::File::create(&tmp)?;
        f.write_all(&header_bytes)?;
        f.write_all(&payload_bytes)?;
        let _ = f.sync_all();
    }
    if path.exists() {
        let _ = fs::remove_file(path);
    }
    fs::rename(&tmp, path)
}

fn read_telemetry_aim(path: &Path) -> Result<(TelemetryAimHeader, Vec<TelemetrySample>), String> {
    let bytes = fs::read(path).map_err(|e| e.to_string())?;
    let header_end = find_json_end(&bytes)
        .ok_or_else(|| "Invalid .aim format: No JSON header found".to_string())?;
    let header: TelemetryAimHeader = serde_json::from_slice(&bytes[..header_end])
        .map_err(|e| format!("Failed to parse telemetry.aim header: {}", e))?;
    let samples: Vec<TelemetrySample> = if header_end >= bytes.len() {
        Vec::new()
    } else {
        serde_json::from_slice(&bytes[header_end..])
            .map_err(|e| format!("Failed to parse telemetry.aim samples: {}", e))?
    };
    Ok((header, samples))
}

/// Load `telemetry.aim` (or the supplied path) from disk into the in-memory
/// state. Safe to call from frontend boot; missing files yield a clean empty
/// state without erroring.
#[tauri::command]
pub async fn aim_load_telemetry(path: Option<String>) -> Result<Value, String> {
    let path_buf = path
        .map(PathBuf::from)
        .unwrap_or_else(default_telemetry_aim_path);
    let mut guard = telemetry_state().lock().map_err(|e| e.to_string())?;
    guard.path = Some(path_buf.clone());

    if !path_buf.exists() {
        guard.samples.clear();
        guard.header = TelemetryAimHeader::default();
        guard.dirty_count = 0;
        return Ok(serde_json::to_value(&*guard).unwrap_or_else(|_| json!({})));
    }

    let (header, samples) = read_telemetry_aim(&path_buf)?;
    guard.header = header;
    guard.samples = samples.into_iter().collect();
    guard.dirty_count = 0;
    Ok(serde_json::to_value(&*guard).unwrap_or_else(|_| json!({})))
}

/// Append a single sample. Lifetime counters are bumped unconditionally;
/// the ring buffer evicts the oldest entry once it overflows `capacity`.
/// Every 16th append triggers a disk flush so we don't lose more than that
/// many samples on a hard crash.
#[tauri::command]
pub async fn aim_append_telemetry(sample: TelemetrySample) -> Result<Value, String> {
    let mut guard = telemetry_state().lock().map_err(|e| e.to_string())?;
    guard.header.lifetime_samples = guard.header.lifetime_samples.saturating_add(1);
    guard.header.lifetime_output_tokens = guard
        .header
        .lifetime_output_tokens
        .saturating_add(sample.output_tokens);
    guard.header.lifetime_input_tokens = guard
        .header
        .lifetime_input_tokens
        .saturating_add(sample.input_tokens);
    guard.header.lifetime_tokens_skipped = guard
        .header
        .lifetime_tokens_skipped
        .saturating_add(sample.tokens_skipped);
    if sample.cache_hit {
        guard.header.lifetime_cache_hits = guard.header.lifetime_cache_hits.saturating_add(1);
    }

    let cap = guard.header.capacity.max(1);
    guard.samples.push_back(sample);
    while guard.samples.len() > cap {
        guard.samples.pop_front();
    }
    guard.dirty_count = guard.dirty_count.saturating_add(1);

    if guard.dirty_count >= 16 {
        let path = guard
            .path
            .clone()
            .unwrap_or_else(default_telemetry_aim_path);
        // Best-effort flush; failures must not break the inference path
        // that called us. The next flush attempt will catch up.
        let _ = write_telemetry_aim(&path, &guard);
        guard.dirty_count = 0;
        guard.path = Some(path);
    }
    Ok(serde_json::to_value(&*guard).unwrap_or_else(|_| json!({})))
}

/// Update the cached "this is the model the KDKVC proxy is currently bound
/// to" stamp. Called by `kortex_kvcache_start` so even when the proxy is
/// down, the IDE can read the .aim VFS and know what was last loaded.
#[tauri::command]
pub async fn aim_set_bound_model(
    model_id: String,
    quant_signature: Option<String>,
    tokenizer_hash: Option<String>,
) -> Result<(), String> {
    let mut guard = telemetry_state().lock().map_err(|e| e.to_string())?;
    guard.header.last_model_id = if model_id.is_empty() {
        None
    } else {
        Some(model_id)
    };
    guard.header.last_quant_signature = quant_signature;
    guard.header.last_tokenizer_hash = tokenizer_hash;
    guard.dirty_count = guard.dirty_count.saturating_add(1);
    let path = guard
        .path
        .clone()
        .unwrap_or_else(default_telemetry_aim_path);
    let _ = write_telemetry_aim(&path, &guard);
    guard.dirty_count = 0;
    guard.path = Some(path);
    Ok(())
}

/// Read-only snapshot of the live state. Frontend uses this to render the
/// "Neural VFS state" panel in `KortexInferencePanel.tsx`.
#[tauri::command]
pub async fn aim_telemetry_snapshot() -> Result<Value, String> {
    let guard = telemetry_state().lock().map_err(|e| e.to_string())?;
    Ok(serde_json::to_value(&*guard).unwrap_or_else(|_| json!({})))
}

/// Force a flush. Wired into `kortex_kvcache_stop` and the IDE shutdown hook
/// so we never lose unflushed samples on a clean exit.
#[tauri::command]
pub async fn aim_flush_telemetry() -> Result<(), String> {
    let guard = telemetry_state().lock().map_err(|e| e.to_string())?;
    let path = guard
        .path
        .clone()
        .unwrap_or_else(default_telemetry_aim_path);
    write_telemetry_aim(&path, &guard).map_err(|e| e.to_string())?;
    Ok(())
}

/// Wipe the ring buffer but keep lifetime counters. Useful when the user
/// wants to start a fresh measurement window without forgetting their
/// historical totals.
#[tauri::command]
pub async fn aim_clear_telemetry_samples() -> Result<(), String> {
    let mut guard = telemetry_state().lock().map_err(|e| e.to_string())?;
    guard.samples.clear();
    guard.dirty_count = guard.dirty_count.saturating_add(1);
    let path = guard
        .path
        .clone()
        .unwrap_or_else(default_telemetry_aim_path);
    let _ = write_telemetry_aim(&path, &guard);
    guard.dirty_count = 0;
    guard.path = Some(path);
    Ok(())
}

// Manual Serialize impl so the snapshot includes both header fields and the
// current ring buffer in a single object the frontend can consume directly.
impl Serialize for TelemetryState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("TelemetryState", 11)?;
        s.serialize_field("schema", &self.header.schema)?;
        s.serialize_field("capacity", &self.header.capacity)?;
        s.serialize_field("lifetime_samples", &self.header.lifetime_samples)?;
        s.serialize_field(
            "lifetime_output_tokens",
            &self.header.lifetime_output_tokens,
        )?;
        s.serialize_field("lifetime_input_tokens", &self.header.lifetime_input_tokens)?;
        s.serialize_field(
            "lifetime_tokens_skipped",
            &self.header.lifetime_tokens_skipped,
        )?;
        s.serialize_field("lifetime_cache_hits", &self.header.lifetime_cache_hits)?;
        s.serialize_field("last_model_id", &self.header.last_model_id)?;
        s.serialize_field("last_quant_signature", &self.header.last_quant_signature)?;
        s.serialize_field("last_tokenizer_hash", &self.header.last_tokenizer_hash)?;
        let samples_vec: Vec<&TelemetrySample> = self.samples.iter().collect();
        s.serialize_field("samples", &samples_vec)?;
        s.end()
    }
}

/// Resolve a locally-installed Ollama model tag to the backing `.gguf` path via `ollama show`.
#[tauri::command]
pub async fn kortex_resolve_ollama_gguf(model: String) -> Result<Option<String>, String> {
    let model = model.trim().to_string();
    if model.is_empty() {
        return Ok(None);
    }

    let output = tokio::process::Command::new("ollama")
        .args(["show", &model, "--modelfile"])
        .output()
        .await
        .map_err(|e| {
            format!(
                "Could not run `ollama show` — is Ollama installed and on PATH? ({})",
                e
            )
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "`ollama show {} --modelfile` failed: {}",
            model,
            stderr.trim().chars().take(400).collect::<String>()
        ));
    }

    let text = String::from_utf8_lossy(&output.stdout);
    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.len() < 6 {
            continue;
        }
        let upper = line.to_uppercase();
        if upper.starts_with("FROM ") {
            let rest = line[5..].trim().trim_matches('"').trim_matches('\'');
            if rest.to_lowercase().ends_with(".gguf") {
                let p = PathBuf::from(rest);
                if p.exists() {
                    return Ok(Some(p.to_string_lossy().into_owned()));
                }
                let home = std::env::var("USERPROFILE")
                    .or_else(|_| std::env::var("HOME"))
                    .unwrap_or_else(|_| ".".to_string());
                let candidates = [
                    PathBuf::from(rest),
                    Path::new(&home).join(".ollama/models").join(rest),
                    Path::new(&home).join(".ollama/models/blobs").join(rest),
                ];
                for c in candidates {
                    if c.exists() {
                        return Ok(Some(c.to_string_lossy().into_owned()));
                    }
                }
                return Ok(Some(rest.to_string()));
            }
        }
    }

    Ok(None)
}

// ─────────────────────────────────────────────────────────────────────────────
// AIM VFS Zero-Grep Integration Commands
// ─────────────────────────────────────────────────────────────────────────────

/// Pack all indexed codebase context into a compact semantic map.
/// The AI calls this once at the start of a session to get the "6 gist tokens"
/// summary of the entire workspace — eliminating the need for grep/list_files.
#[tauri::command]
pub async fn aim_pack_context(
    state: tauri::State<'_, crate::EditorState>,
    query: Option<String>,
    max_slots: Option<usize>,
) -> Result<Value, String> {
    let limit = max_slots.unwrap_or(20);
    let q = query.as_deref().unwrap_or("");
    let mem = &state.ai_engine.memory_store;

    // Build the compact brain gist (priority-weighted slots)
    let gist = mem.build_compact_gist().await;

    // Get project tree summary
    let tree_summary = mem.get_project_tree_summary().await;
    let tree = mem.get_project_tree().await;

    // Retrieve query-relevant slots (if query provided)
    let relevant_preview: Vec<Value> = if !q.is_empty() {
        let relevant = mem.retrieve_context(q).await;
        relevant.lines()
            .take(limit)
            .map(|l| json!(l))
            .collect()
    } else {
        // Return top code-category slots for the full "what's in this codebase" view
        let slots = mem.get_all_slots().await;
        slots.iter()
            .filter(|s| s.category == "code")
            .take(limit)
            .map(|s| json!({
                "file": s.metadata.as_ref()
                    .and_then(|m| m.get("path")).and_then(|v| v.as_str()).unwrap_or(""),
                "preview": &s.content.chars().take(100).collect::<String>(),
                "tags": s.tags,
            }))
            .collect()
    };

    Ok(json!({
        "schema": "kortex.aim.packed/v1",
        "gist": gist,
        "project_summary": tree_summary,
        "total_indexed_files": tree.len(),
        "relevant_context": relevant_preview,
        "token_estimate": gist.len() / 4,
        "instruction": "Trust this index. Zero-grep mode active — do NOT call list_files or grep to understand the codebase. The structure above IS the codebase map."
    }))
}

/// Trigger a full background workspace index cycle immediately.
/// Called by the Kortex panel "Index Workspace" button or on first chat.
#[tauri::command]
pub async fn trigger_workspace_index(
    state: tauri::State<'_, crate::EditorState>,
    app: tauri::AppHandle,
) -> Result<Value, String> {
    use tauri::Emitter;
    let indexer = state.context_indexer.clone();
    let app_clone = app.clone();

    // Spawn non-blocking so the command returns immediately
    tauri::async_runtime::spawn(async move {
        let _ = app_clone.emit("aim-index-progress", json!({ "status": "started" }));
        match indexer.trigger_index_cycle().await {
            Ok(()) => {
                let _ = app_clone.emit("aim-index-progress", json!({ "status": "complete" }));
            }
            Err(e) => {
                let _ = app_clone.emit("aim-index-progress", json!({
                    "status": "error", "message": e.to_string()
                }));
            }
        }
    });

    Ok(json!({ "status": "indexing_started" }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn make_sample(ts: u64, out: u64, hit: bool) -> TelemetrySample {
        TelemetrySample {
            wall_clock_ms: 1000,
            prefill_ms: Some(100),
            output_tokens: out,
            input_tokens: 50,
            backend: "llama.cpp".to_string(),
            cache_hit: hit,
            tokens_skipped: if hit { 200 } else { 0 },
            model_id: Some("test.gguf".to_string()),
            ts_unix_ms: ts,
        }
    }

    #[test]
    fn round_trips_through_disk() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("telemetry.aim");
        let mut state = TelemetryState::new();
        state.samples.push_back(make_sample(1, 10, false));
        state.samples.push_back(make_sample(2, 20, true));
        state.header.lifetime_samples = 2;
        state.header.lifetime_cache_hits = 1;
        state.header.last_model_id = Some("qwen3-coder.Q4_K_M.gguf".to_string());

        write_telemetry_aim(&path, &state).unwrap();
        let (header, samples) = read_telemetry_aim(&path).unwrap();
        assert_eq!(header.lifetime_samples, 2);
        assert_eq!(header.lifetime_cache_hits, 1);
        assert_eq!(
            header.last_model_id.as_deref(),
            Some("qwen3-coder.Q4_K_M.gguf")
        );
        assert_eq!(samples.len(), 2);
        assert_eq!(samples[0].output_tokens, 10);
        assert_eq!(samples[1].cache_hit, true);
    }

    #[test]
    fn missing_file_yields_clean_state() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("nope.aim");
        assert!(!path.exists());
        let result = read_telemetry_aim(&path);
        assert!(result.is_err()); // missing file is an error from the reader; the command wraps this.
    }

    #[test]
    fn corrupt_header_returns_error() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("bad.aim");
        fs::write(&path, b"not even json").unwrap();
        let result = read_telemetry_aim(&path);
        assert!(result.is_err());
    }

    #[test]
    fn write_is_atomic_when_target_exists() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("telemetry.aim");
        let mut state = TelemetryState::new();
        state.samples.push_back(make_sample(1, 10, false));
        write_telemetry_aim(&path, &state).unwrap();

        // Re-write with a different payload; the .tmp must not linger.
        state.samples.push_back(make_sample(2, 99, true));
        write_telemetry_aim(&path, &state).unwrap();

        let tmp = path.with_extension("aim.tmp");
        assert!(!tmp.exists(), "tmp file should be renamed away");

        let (_, samples) = read_telemetry_aim(&path).unwrap();
        assert_eq!(samples.len(), 2);
        assert_eq!(samples[1].output_tokens, 99);
    }
}

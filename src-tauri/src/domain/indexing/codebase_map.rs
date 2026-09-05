//! Tiered codebase map — the "always-on, tiny" half of cheap AI codebase
//! awareness (see docs/kortex-cache.md, context strategy).
//!
//! A full signature blueprint of a large repo is 30–80K tokens — too big to
//! inject every turn (it breaks small-context local models and costs real money
//! on cloud APIs). Instead this produces a compact, deterministic **directory
//! map**: one line per source directory with a human-readable summary pulled
//! from that module's entry file (`mod.rs` / `lib.rs` / `index.ts` doc comment).
//!
//! The model gets whole-repo *orientation* in ~1–3K tokens and pulls detailed
//! per-file signatures on demand via the `get_file_signatures` tool
//! (`StructuralBlueprints::blueprint_file`). Because the map is a stable prefix,
//! the KV cache (local) and `cache_control` prompt caching (cloud) make
//! re-sending it on every turn nearly free.

use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::Path;

/// Directories never walked — build artifacts, deps, VCS, our own cache.
const SKIP_DIRS: &[&str] = &[
    "node_modules", "target", "dist", "build", ".git", ".kortex", ".aim",
    ".next", "out", "venv", ".venv", "__pycache__", "coverage", ".cache",
];

const SOURCE_EXTS: &[&str] = &["rs", "ts", "tsx", "js", "jsx", "mjs", "cjs", "py"];

/// Entry-file basenames whose leading doc comment best summarizes a directory.
const ENTRY_FILES: &[&str] = &["mod.rs", "lib.rs", "main.rs", "index.ts", "index.tsx", "index.js", "__init__.py"];

/// Generate the tiered repo map as markdown, bounded to roughly `max_chars`
/// (≈ `max_chars / 4` tokens). Deterministic: directories are sorted, so the
/// output is byte-stable across runs given the same tree — which is what lets
/// the caches treat it as a fixed prefix.
pub fn generate_repo_map(root: &Path, max_chars: usize) -> String {
    // dir (relative, "/"-normalized) -> (file_count, summary)
    let mut dirs: BTreeMap<String, DirInfo> = BTreeMap::new();

    let walker = walkdir::WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| {
            !e.file_type().is_dir()
                || !SKIP_DIRS.contains(&e.file_name().to_string_lossy().as_ref())
        })
        .filter_map(|e| e.ok());

    for entry in walker {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
        if !SOURCE_EXTS.contains(&ext.as_str()) {
            continue;
        }
        let rel_dir = path
            .parent()
            .and_then(|p| p.strip_prefix(root).ok())
            .map(normalize_dir)
            .unwrap_or_else(|| ".".to_string());

        let info = dirs.entry(rel_dir).or_default();
        info.file_count += 1;

        // If this file is a module entry point, use its leading doc comment as
        // the directory summary. First entry file wins deterministically
        // (ENTRY_FILES order), so re-runs are stable.
        if info.summary.is_none() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if ENTRY_FILES.contains(&name) {
                    if let Ok(content) = std::fs::read_to_string(path) {
                        info.summary = first_doc_line(&content, &ext);
                    }
                }
            }
        }
    }

    render_map(&dirs, max_chars)
}

#[derive(Default)]
struct DirInfo {
    file_count: usize,
    summary: Option<String>,
}

/// Cache file schema: the map plus the tree signature it was built from.
#[derive(serde::Serialize, serde::Deserialize)]
struct MapCache {
    signature: String,
    max_chars: usize,
    map: String,
}

const CACHE_JSON: &str = "codebase_map.json";
const CACHE_MD: &str = "codebase_map.md";

/// Persisted, hash-invalidated variant of [`generate_repo_map`]. Returns the
/// cached map when the source tree is unchanged since the last build; otherwise
/// regenerates, rewrites `<root>/.kortex/codebase_map.json` (+ a human-readable
/// `.md` sidecar), and returns the fresh map.
///
/// This is what the agent loop should call: on the common path (no source
/// change between runs) it skips reading entry files and rendering, and yields a
/// byte-identical map so downstream caches (Anthropic `cache_control`, local KV)
/// keep treating it as the same stable prefix. The tree walk itself still runs —
/// it's the cheap part and is needed to detect change.
pub fn generate_repo_map_cached(root: &Path, max_chars: usize) -> String {
    let sig = tree_signature(root);
    let cache_dir = root.join(".kortex");
    let cache_path = cache_dir.join(CACHE_JSON);

    if let Ok(bytes) = std::fs::read(&cache_path) {
        if let Ok(cache) = serde_json::from_slice::<MapCache>(&bytes) {
            if cache.signature == sig && cache.max_chars == max_chars {
                return cache.map;
            }
        }
    }

    let map = generate_repo_map(root, max_chars);
    let _ = std::fs::create_dir_all(&cache_dir);
    let cache = MapCache {
        signature: sig,
        max_chars,
        map: map.clone(),
    };
    if let Ok(json) = serde_json::to_vec(&cache) {
        // Atomic-ish: write tmp then rename so a crash mid-write never leaves a
        // half-written cache that would deserialize wrong.
        let tmp = cache_dir.join("codebase_map.json.tmp");
        if std::fs::write(&tmp, &json).is_ok() {
            if cache_path.exists() {
                let _ = std::fs::remove_file(&cache_path);
            }
            let _ = std::fs::rename(&tmp, &cache_path);
        }
    }
    let _ = std::fs::write(cache_dir.join(CACHE_MD), &map);
    map
}

/// Content-free signature of the source tree: SHA-256 over the sorted list of
/// (relative_path, mtime_secs, size) for every source file. Invalidates on
/// add/remove/rename (the path set changes) and on any edit (mtime or size
/// changes) — so an entry file's doc-comment edit busts the cached summary.
/// No file contents are read, so it's cheap relative to full regeneration.
fn tree_signature(root: &Path) -> String {
    let mut items: Vec<(String, u64, u64)> = Vec::new();
    let walker = walkdir::WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| {
            !e.file_type().is_dir()
                || !SKIP_DIRS.contains(&e.file_name().to_string_lossy().as_ref())
        })
        .filter_map(|e| e.ok());
    for entry in walker {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
        if !SOURCE_EXTS.contains(&ext.as_str()) {
            continue;
        }
        let rel = path
            .strip_prefix(root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");
        let (mtime, size) = std::fs::metadata(path)
            .map(|m| {
                let mt = m
                    .modified()
                    .ok()
                    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                    .map(|d| d.as_secs())
                    .unwrap_or(0);
                (mt, m.len())
            })
            .unwrap_or((0, 0));
        items.push((rel, mtime, size));
    }
    items.sort();
    let mut h = Sha256::new();
    for (p, mt, sz) in &items {
        h.update(p.as_bytes());
        h.update(b"\0");
        h.update(&mt.to_le_bytes());
        h.update(&sz.to_le_bytes());
    }
    format!("{:x}", h.finalize())
}

fn render_map(dirs: &BTreeMap<String, DirInfo>, max_chars: usize) -> String {
    let mut out = String::from("# Codebase map\n\nOne line per source directory. Use `get_file_signatures(path)` for a file's function/type signatures.\n\n");
    let mut truncated = false;
    for (dir, info) in dirs {
        let line = match &info.summary {
            Some(s) => format!("- `{}/` ({} files) — {}\n", dir, info.file_count, s),
            None => format!("- `{}/` ({} files)\n", dir, info.file_count),
        };
        if out.len() + line.len() > max_chars {
            truncated = true;
            break;
        }
        out.push_str(&line);
    }
    if truncated {
        out.push_str("- … (map truncated to fit the token budget; ask for specific directories)\n");
    }
    out
}

fn normalize_dir(p: &Path) -> String {
    if p.as_os_str().is_empty() {
        return ".".to_string();
    }
    p.to_string_lossy().replace('\\', "/")
}

/// Extract a one-line module summary from the leading doc comment of a source
/// file. Rust: `//!` module docs. TS/JS: leading `//` or `/** */`. Python:
/// module docstring. Returns the first non-empty line, trimmed, or None.
/// Pure + deterministic so it's unit-testable without touching disk.
pub fn first_doc_line(content: &str, ext: &str) -> Option<String> {
    let clean = |s: &str| -> String { s.trim().trim_end_matches('.').to_string() };
    match ext {
        "rs" => {
            for raw in content.lines() {
                let line = raw.trim();
                if line.is_empty() {
                    continue;
                }
                if let Some(rest) = line.strip_prefix("//!") {
                    let t = clean(rest);
                    if !t.is_empty() {
                        return Some(t);
                    }
                    continue; // blank `//!` line — keep scanning
                }
                // First non-blank, non-module-doc line → no summary available.
                if !line.starts_with("//") {
                    return None;
                }
            }
            None
        }
        "ts" | "tsx" | "js" | "jsx" | "mjs" | "cjs" => {
            for raw in content.lines() {
                let line = raw.trim();
                if line.is_empty() {
                    continue;
                }
                if let Some(rest) = line.strip_prefix("//") {
                    let t = clean(rest);
                    if !t.is_empty() {
                        return Some(t);
                    }
                    continue;
                }
                // JSDoc/block comment: pull the first text line inside it.
                if line.starts_with("/*") {
                    let body = line.trim_start_matches("/*").trim_start_matches('*');
                    let t = clean(body.trim_start_matches('*'));
                    if !t.is_empty() && t != "/" {
                        return Some(t);
                    }
                    continue;
                }
                return None;
            }
            None
        }
        "py" | "pyi" => {
            let trimmed = content.trim_start();
            for q in ["\"\"\"", "'''"] {
                if let Some(rest) = trimmed.strip_prefix(q) {
                    if let Some(end) = rest.find(q) {
                        let doc = &rest[..end];
                        if let Some(first) = doc.lines().map(|l| l.trim()).find(|l| !l.is_empty()) {
                            return Some(clean(first));
                        }
                    }
                }
            }
            None
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::atomic::{AtomicU64, Ordering};

    static N: AtomicU64 = AtomicU64::new(0);
    fn tempdir(label: &str) -> std::path::PathBuf {
        let n = N.fetch_add(1, Ordering::SeqCst);
        let d = std::env::temp_dir().join(format!("kortex_map_{}_{}_{}", label, std::process::id(), n));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn first_doc_line_rust_module_doc() {
        let src = "//! Agent loop and streaming.\n//! second line\nuse std::fs;\n";
        assert_eq!(first_doc_line(src, "rs").as_deref(), Some("Agent loop and streaming"));
    }

    #[test]
    fn first_doc_line_rust_skips_blank_module_doc_then_finds_text() {
        let src = "//!\n//! Real summary here\nfn main() {}\n";
        assert_eq!(first_doc_line(src, "rs").as_deref(), Some("Real summary here"));
    }

    #[test]
    fn first_doc_line_rust_none_when_code_first() {
        let src = "use std::fs;\n//! too late\n";
        assert_eq!(first_doc_line(src, "rs"), None);
    }

    #[test]
    fn first_doc_line_ts_line_and_block() {
        assert_eq!(first_doc_line("// Zustand store slice.\nexport const x=1;", "ts").as_deref(), Some("Zustand store slice"));
        assert_eq!(first_doc_line("/** Vector index client */\n", "tsx").as_deref(), Some("Vector index client */"));
    }

    #[test]
    fn first_doc_line_python_docstring() {
        assert_eq!(first_doc_line("\"\"\"Sidecar manager.\"\"\"\nimport os\n", "py").as_deref(), Some("Sidecar manager"));
    }

    #[test]
    fn generate_repo_map_groups_dirs_with_summaries_and_is_deterministic() {
        let root = tempdir("gen");
        fs::create_dir_all(root.join("engine")).unwrap();
        fs::create_dir_all(root.join("tools")).unwrap();
        fs::write(root.join("engine/mod.rs"), "//! Agent engine.\npub mod a;").unwrap();
        fs::write(root.join("engine/a.rs"), "fn a() {}").unwrap();
        fs::write(root.join("tools/mod.rs"), "//! Tool registry.\n").unwrap();
        // A skipped dir must not appear.
        fs::create_dir_all(root.join("node_modules/pkg")).unwrap();
        fs::write(root.join("node_modules/pkg/index.js"), "// junk").unwrap();

        let m1 = generate_repo_map(&root, 10_000);
        let m2 = generate_repo_map(&root, 10_000);
        assert_eq!(m1, m2, "map must be deterministic across runs");
        assert!(m1.contains("`engine/` (2 files) — Agent engine"));
        assert!(m1.contains("`tools/` (1 files) — Tool registry"));
        assert!(!m1.contains("node_modules"), "skipped dirs must not appear");
    }

    #[test]
    fn cached_map_persists_and_reuses() {
        let root = tempdir("cache_reuse");
        fs::create_dir_all(root.join("engine")).unwrap();
        fs::write(root.join("engine/mod.rs"), "//! Agent engine.\n").unwrap();

        let m1 = generate_repo_map_cached(&root, 10_000);
        // Cache artifacts written.
        assert!(root.join(".kortex").join("codebase_map.json").exists());
        assert!(root.join(".kortex").join("codebase_map.md").exists());

        // Second call with an unchanged tree returns a byte-identical map (so
        // downstream caches keep treating it as the same stable prefix).
        let m2 = generate_repo_map_cached(&root, 10_000);
        assert_eq!(m1, m2);
    }

    #[test]
    fn cached_map_invalidates_when_tree_changes() {
        let root = tempdir("cache_inval");
        fs::create_dir_all(root.join("a")).unwrap();
        fs::write(root.join("a/mod.rs"), "//! Module A.\n").unwrap();

        let before = generate_repo_map_cached(&root, 10_000);
        assert!(!before.contains("`b/`"));

        // Add a new directory + file → path set changes → signature changes.
        fs::create_dir_all(root.join("b")).unwrap();
        fs::write(root.join("b/mod.rs"), "//! Module B.\n").unwrap();

        let after = generate_repo_map_cached(&root, 10_000);
        assert!(after.contains("`b/` (1 files) — Module B"), "map must refresh after tree change");
        assert_ne!(before, after);
    }

    #[test]
    fn tree_signature_changes_on_add_and_stable_otherwise() {
        let root = tempdir("sig");
        fs::create_dir_all(root.join("x")).unwrap();
        fs::write(root.join("x/a.rs"), "fn a() {}").unwrap();

        let s1 = tree_signature(&root);
        let s1b = tree_signature(&root);
        assert_eq!(s1, s1b, "signature must be stable for an unchanged tree");

        fs::write(root.join("x/b.rs"), "fn b() {}").unwrap();
        let s2 = tree_signature(&root);
        assert_ne!(s1, s2, "signature must change when a file is added");
    }

    #[test]
    fn cached_map_rebuilds_when_budget_differs() {
        let root = tempdir("cache_budget");
        // Enough directories that a small budget truncates but a large one doesn't.
        for i in 0..40 {
            let d = root.join(format!("dir{:02}", i));
            fs::create_dir_all(&d).unwrap();
            fs::write(d.join("mod.rs"), format!("//! Module {i} summary.\n")).unwrap();
        }
        // Cache under a large budget first.
        let big = generate_repo_map_cached(&root, 10_000);
        // A smaller budget on the same tree must NOT serve the cached big map —
        // it must rebuild, matching a fresh generate at that budget.
        let small = generate_repo_map_cached(&root, 400);
        assert_ne!(big, small, "different budget must bust the cache");
        assert_eq!(small, generate_repo_map(&root, 400), "rebuilt map must match a fresh generate at that budget");
        assert!(small.contains("truncated"));
    }

    #[test]
    fn generate_repo_map_respects_char_budget() {
        let root = tempdir("budget");
        for i in 0..50 {
            let d = root.join(format!("dir{:02}", i));
            fs::create_dir_all(&d).unwrap();
            fs::write(d.join("mod.rs"), format!("//! Module {i} summary.\n")).unwrap();
        }
        let small = generate_repo_map(&root, 400);
        assert!(small.len() <= 400 + 120, "map exceeded budget: {} chars", small.len());
        assert!(small.contains("truncated"), "over-budget map must note truncation");
    }
}

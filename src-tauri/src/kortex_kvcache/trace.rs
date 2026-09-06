//! Compute-trace sink — turns the KV cache's prefill savings into a receipt.
//!
//! `docs/kortex-context-engine-plan.md` and `tools/compute-bench/model.py`
//! *predict* how much prefill compute the stack saves. This writes what
//! actually happened: one JSON line per intercepted request to the file named
//! by `KORTEX_COMPUTE_TRACE`, then `tools/compute-bench/reduce_trace.py`
//! folds a run into the same before/after table `model.py` prints.
//!
//! Naive baseline for a session = `sum(tokens_in)` (every turn re-prefills its
//! whole prompt). Kortex actual = `sum(tokens_in - prefix_hit_tokens)`.
//!
//! Everything here is best-effort: a missing env var, an unwritable path or a
//! serialisation error must never disturb the inference path, so all failures
//! are swallowed.

use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

/// One intercepted request's prefill accounting.
///
/// `tokens_in` is the *prefix* token count (the conversation minus the trailing
/// turn) — the part the KV cache can restore. The trailing user turn is small
/// and appears on both the naive and the cached side of the comparison, so the
/// ratio is unaffected by leaving it out.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceRecord {
    /// Milliseconds since the Unix epoch when the request was routed.
    pub ts_unix_ms: u128,
    /// Correlates with the proxy log line for this request.
    pub request_id: String,
    /// `"chat"` or `"completion"`.
    pub kind: String,
    /// Prefix tokens the request carried.
    pub tokens_in: u32,
    /// Prefix tokens restored from a slot instead of being re-prefilled.
    pub prefix_hit_tokens: u32,
    /// Prefix tokens the upstream still had to prefill (`tokens_in - hit`).
    pub suffix_tokens_processed: u32,
    /// Whether a usable prefix match was found.
    pub cache_hit: bool,
}

impl TraceRecord {
    pub fn new(
        request_id: impl Into<String>,
        kind: impl Into<String>,
        tokens_in: u32,
        prefix_hit_tokens: u32,
    ) -> Self {
        let hit = prefix_hit_tokens.min(tokens_in);
        Self {
            ts_unix_ms: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_millis())
                .unwrap_or(0),
            request_id: request_id.into(),
            kind: kind.into(),
            tokens_in,
            prefix_hit_tokens: hit,
            suffix_tokens_processed: tokens_in.saturating_sub(hit),
            cache_hit: hit > 0,
        }
    }
}

/// Path from `KORTEX_COMPUTE_TRACE`, or `None` when tracing is off.
pub fn compute_trace_path() -> Option<PathBuf> {
    match std::env::var("KORTEX_COMPUTE_TRACE") {
        Ok(s) if !s.trim().is_empty() => Some(PathBuf::from(s.trim())),
        _ => None,
    }
}

/// Append one record as a JSON line. No-op unless `KORTEX_COMPUTE_TRACE` is set.
pub fn append(rec: &TraceRecord) {
    let Some(path) = compute_trace_path() else {
        return;
    };
    append_to(&path, rec);
}

/// Testable core: append to an explicit path.
pub fn append_to(path: &std::path::Path, rec: &TraceRecord) {
    let Ok(mut line) = serde_json::to_string(rec) else {
        return;
    };
    line.push('\n');
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = f.write_all(line.as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_computes_suffix_and_hit_flag() {
        let r = TraceRecord::new("req-1", "chat", 28_000, 27_500);
        assert_eq!(r.suffix_tokens_processed, 500);
        assert!(r.cache_hit);

        let miss = TraceRecord::new("req-2", "chat", 28_000, 0);
        assert_eq!(miss.suffix_tokens_processed, 28_000);
        assert!(!miss.cache_hit);
    }

    #[test]
    fn hit_tokens_are_clamped_to_tokens_in() {
        // A slot restore that reports more than the request carried can't mean
        // negative work.
        let r = TraceRecord::new("req-3", "completion", 1000, 5000);
        assert_eq!(r.prefix_hit_tokens, 1000);
        assert_eq!(r.suffix_tokens_processed, 0);
    }

    #[test]
    fn append_to_writes_one_json_line_per_call() {
        let dir = std::env::temp_dir().join(format!("kortex-trace-{}", std::process::id()));
        let path = dir.join("compute-trace.jsonl");
        let _ = std::fs::remove_file(&path);

        append_to(&path, &TraceRecord::new("a", "chat", 100, 90));
        append_to(&path, &TraceRecord::new("b", "chat", 200, 0));

        let body = std::fs::read_to_string(&path).unwrap();
        let lines: Vec<&str> = body.lines().collect();
        assert_eq!(lines.len(), 2);
        let first: TraceRecord = serde_json::from_str(lines[0]).unwrap();
        assert_eq!(first.request_id, "a");
        assert_eq!(first.prefix_hit_tokens, 90);
        let second: TraceRecord = serde_json::from_str(lines[1]).unwrap();
        assert_eq!(second.cache_hit, false);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn append_is_a_noop_without_the_env_var() {
        // Not set in the test process -> no path, no write, no panic.
        std::env::remove_var("KORTEX_COMPUTE_TRACE");
        append(&TraceRecord::new("x", "chat", 10, 0));
        assert!(compute_trace_path().is_none());
    }
}

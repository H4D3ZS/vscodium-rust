//! Shared types for the Kortex Disk KV Cache (KDKVC).
//!
//! The cache key is SHA-256 of the LE-encoded u32 token ID stream — same
//! design as antirez/ds4 but with SHA-256 instead of SHA-1, because we already
//! depend on `sha2` and don't need ds4 file-format compatibility (ds4 owns its
//! own KV format; we delegate KV serialization to llama-server's slot API).
//!
//! One client request goes through the proxy as:
//!
//!   POST /v1/chat/completions
//!     │
//!     ├── tokenize via llama-server /tokenize
//!     ├── compute SHA over LE-u32(tokens)
//!     ├── try longest-prefix match against indexed `.kkv` files
//!     │     hit  -> /slots/0?action=restore + send only the suffix
//!     │     miss -> forward as-is, save slot to disk on completion
//!     └── stream the SSE response back to the client

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// On-disk index entry written next to llama-server's slot binary.
/// Filename: `<sha256_hex>.kkv` (40 chars sha + ".kkv").
///
/// llama-server's slot save file lives in its `--slot-save-path` directory and
/// is named `<sha256_hex>.slotbin` so the two are always co-located.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KvCacheEntry {
    /// SHA-256 of the prefix token stream as 64-char lowercase hex.
    pub sha: String,
    /// Number of tokens this entry covers.
    pub prefix_token_count: u32,
    /// Context size the snapshot was written for. Restore must use a context
    /// at least this large or llama-server will reject the file.
    pub ctx_size: u32,
    /// Unix seconds when the entry was created.
    pub created_at: u64,
    /// Unix seconds the entry was last reused (LRU key).
    pub last_used_at: u64,
    /// Number of times this entry has been restored.
    pub hit_count: u32,
    /// Path to the slot binary file (absolute). Owned by llama-server's
    /// slot-save-path, not our cache dir.
    pub slotbin_path: String,
    /// Bytes of the slot binary on disk (for budget accounting).
    pub slotbin_size: u64,
    /// Optional rendered prefix text for human inspection.
    pub rendered_text: String,
}

impl KvCacheEntry {
    pub fn now_unix() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }

    pub fn touch(&mut self) {
        self.last_used_at = Self::now_unix();
        self.hit_count = self.hit_count.saturating_add(1);
    }
}

/// Cache configuration, persisted in localStorage on the TS side and re-sent
/// when the proxy is started.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KvCacheOptions {
    /// Directory holding our `.kkv` index files.
    pub index_dir: PathBuf,
    /// Directory holding llama-server's slot binaries (passed to llama-server
    /// as `--slot-save-path`). Usually a sibling of index_dir.
    pub slot_dir: PathBuf,
    /// Total bytes budget for the slot binaries. LRU evicts when exceeded.
    pub max_bytes: u64,
    /// Don't cache prefixes shorter than this; they're cheap to recompute.
    /// Default 512 (matches ds4).
    pub min_tokens: u32,
    /// Hard ceiling on how long a prefix we'll save. Above this the prefill
    /// is rare enough that the cache cost outweighs the win. Default 30000.
    pub cold_max_tokens: u32,
    /// Trim this many tokens off the end of a saved prefix. Mitigates BPE
    /// boundary mismatch when more text is appended later. Default 32.
    pub boundary_trim_tokens: u32,
    /// Align saved prefixes down to a multiple of this. Should match the
    /// llama-server prefill batch size for best reuse. Default 2048.
    pub boundary_align_tokens: u32,
    /// Save again when a continued conversation has grown by this many tokens
    /// past the last saved checkpoint. Default 10000.
    pub continued_interval_tokens: u32,
    /// Slot id we run inference through. Default 0.
    pub slot_id: u32,
    /// Upstream llama-server URL the proxy forwards to.
    pub upstream_url: String,
    /// Bind address for the proxy.
    pub proxy_host: String,
    /// Bind port for the proxy.
    pub proxy_port: u16,
}

impl Default for KvCacheOptions {
    fn default() -> Self {
        let home = dirs_home_path();
        Self {
            index_dir: home.join(".kortex").join("kvcache").join("index"),
            slot_dir: home.join(".kortex").join("kvcache").join("slots"),
            max_bytes: 16 * 1024 * 1024 * 1024,
            min_tokens: 512,
            cold_max_tokens: 30_000,
            boundary_trim_tokens: 32,
            boundary_align_tokens: 2048,
            continued_interval_tokens: 10_000,
            slot_id: 0,
            upstream_url: "http://127.0.0.1:8081".to_string(),
            proxy_host: "127.0.0.1".to_string(),
            proxy_port: 8090,
        }
    }
}

fn dirs_home_path() -> PathBuf {
    if let Some(p) = std::env::var_os("USERPROFILE").map(PathBuf::from) {
        return p;
    }
    if let Some(p) = std::env::var_os("HOME").map(PathBuf::from) {
        return p;
    }
    PathBuf::from(".")
}

/// Result of a longest-prefix match against the index.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixMatch {
    pub sha: String,
    pub prefix_token_count: u32,
    pub slotbin_path: String,
}

/// Live stats surfaced to the IDE.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KvCacheStats {
    pub entries: u32,
    pub total_bytes: u64,
    pub hits: u64,
    pub misses: u64,
    pub saves: u64,
    pub evictions: u64,
    /// Sum of tokens we *didn't* prefill thanks to cache hits.
    pub tokens_skipped: u64,
}

/// What the proxy decided to do with one request — used for tracing + the η metric.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingTrace {
    pub request_id: String,
    pub tokens_in: u32,
    pub prefix_hit_tokens: u32,
    pub suffix_tokens_processed: u32,
    pub tokens_out: u32,
    pub wall_clock_ms: u64,
    pub eta: f64,
}

impl RoutingTrace {
    /// Information efficiency η = output_tokens / (input_active_tokens × wall_seconds).
    /// Crude proxy for the CCET formula until we wire a proper logit-entropy estimator.
    pub fn compute_eta(
        tokens_active: u32,
        tokens_out: u32,
        wall_clock_ms: u64,
    ) -> f64 {
        let active = tokens_active.max(1) as f64;
        let secs = (wall_clock_ms.max(1) as f64) / 1000.0;
        (tokens_out as f64) / (active * secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entry() -> KvCacheEntry {
        KvCacheEntry {
            sha: "deadbeef".into(),
            prefix_token_count: 1024,
            ctx_size: 8192,
            created_at: 100,
            last_used_at: 100,
            hit_count: 0,
            slotbin_path: "/tmp/none.slotbin".into(),
            slotbin_size: 4096,
            rendered_text: "hello".into(),
        }
    }

    #[test]
    fn touch_advances_last_used_and_hit_count() {
        let mut e = make_entry();
        e.touch();
        assert_eq!(e.hit_count, 1);
        // Unix-second granularity isn't enough to assert strict monotonicity
        // here, but the timestamp must at least be a recent unix second.
        assert!(e.last_used_at >= 100, "last_used_at didn't advance");
        e.touch();
        e.touch();
        assert_eq!(e.hit_count, 3);
    }

    #[test]
    fn touch_saturates_hit_count() {
        let mut e = make_entry();
        e.hit_count = u32::MAX;
        e.touch();
        // Saturating add → still u32::MAX, no panic.
        assert_eq!(e.hit_count, u32::MAX);
    }

    #[test]
    fn options_default_has_sane_values() {
        let o = KvCacheOptions::default();
        assert_eq!(o.proxy_port, 8090);
        assert_eq!(o.proxy_host, "127.0.0.1");
        assert!(o.upstream_url.starts_with("http://"));
        assert!(o.max_bytes > 0);
        assert!(o.min_tokens >= 1);
        // Sanity: index/slot dirs differ.
        assert_ne!(o.index_dir, o.slot_dir);
    }

    #[test]
    fn compute_eta_with_zero_inputs_does_not_divide_by_zero() {
        // Both active and wall_clock_ms are clamped to 1 to dodge div-by-zero.
        let eta = RoutingTrace::compute_eta(0, 10, 0);
        assert!(eta.is_finite());
        // active=1, secs=0.001 → eta = 10 / (1 * 0.001) = 10_000
        assert!((eta - 10_000.0).abs() < 1e-6);
    }

    #[test]
    fn compute_eta_basic_formula() {
        // active=200 tokens, out=100 tokens, wall=2_000 ms = 2.0 s
        // η = 100 / (200 * 2) = 0.25
        let eta = RoutingTrace::compute_eta(200, 100, 2_000);
        assert!((eta - 0.25).abs() < 1e-9);
    }

    #[test]
    fn now_unix_returns_a_recent_timestamp() {
        // Should be after 2020-01-01.
        let now = KvCacheEntry::now_unix();
        assert!(now > 1_577_836_800);
    }
}

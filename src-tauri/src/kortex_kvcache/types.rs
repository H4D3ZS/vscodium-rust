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

/// Why an entry was written to disk. Mirrors ds4's KV_REASON_* enum. Used for
/// observability and lets us implement different retention policies later.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SaveReason {
    /// Catch-all for entries created before this field was tracked, or for
    /// load paths that don't know which trigger fired.
    Unknown,
    /// First save of a long cold prompt after prefill reaches a stable prefix.
    Cold,
    /// Re-save of a session that has grown past `continued_interval_tokens`.
    Continued,
    /// Save just before an unrelated request would have replaced the live
    /// in-memory session.
    Evict,
    /// Save on clean server shutdown so the next start can resume sessions.
    Shutdown,
}

impl Default for SaveReason {
    fn default() -> Self {
        SaveReason::Unknown
    }
}

/// Identity of the model that produced a given KV slot. Restoring a slot saved
/// by model A into model B silently corrupts attention state; ds4 rejects this
/// at load time unless `--kv-cache-reject-different-quant` is opted out of.
/// We always reject across distinct model IDs; the `quant_signature` is the
/// optional softer match used when the same model has different routed-expert
/// quants (e.g. q2 vs q4).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelIdentity {
    /// Stable identifier the upstream server reports (e.g. SHA of the GGUF, or
    /// the file basename when no SHA is available). MUST match exactly for a
    /// cache hit to be considered valid.
    pub model_id: String,
    /// Hash of the tokenizer's vocab + merges. Catches the case where the
    /// model GGUF is the same path but its tokenizer config was updated.
    pub tokenizer_hash: String,
    /// Quant signature, e.g. "Q4_K_M" or "IQ2_XXS+Q2_K". Used for the softer
    /// "same model, different routed-expert quant" match policy.
    pub quant_signature: String,
}

/// On-disk index entry written next to llama-server's slot binary.
/// Filename: `<sha256_hex>.kkv` (64 chars sha + ".kkv").
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

    /// Identity of the model that produced this slot. Older v1 entries have
    /// `default()` here; lookups treat empty-model_id as "unknown, mismatch"
    /// when strict mode is on.
    #[serde(default)]
    pub model: ModelIdentity,
    /// Why this entry was written. Defaults to Unknown for v1 entries.
    #[serde(default)]
    pub save_reason: SaveReason,
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
    /// Model identity stamped on every saved entry. Lookups reject entries
    /// whose identity doesn't match (unless `match_policy` relaxes it).
    /// Populated by the proxy at startup from the upstream /props endpoint.
    #[serde(default)]
    pub model: ModelIdentity,
    /// How strict the lookup is about model identity. See [`ModelMatchPolicy`].
    #[serde(default)]
    pub match_policy: ModelMatchPolicy,
}

/// How strict the lookup is about an entry's stored model identity.
///
/// Defaults to `SameModel`, which permits same-model/different-quant reuse
/// (matches ds4's default behaviour). `Strict` requires every field to match
/// exactly. `Permissive` is escape-hatch for development; never enable it on
/// shared caches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelMatchPolicy {
    /// Reject any entry whose stored `model_id` or `tokenizer_hash` differs
    /// from the running server's. Different `quant_signature` is allowed.
    SameModel,
    /// Reject unless every field matches exactly. Mirrors ds4's
    /// `--kv-cache-reject-different-quant`.
    Strict,
    /// Accept any entry regardless of model identity. Only for local dev.
    Permissive,
}

impl Default for ModelMatchPolicy {
    fn default() -> Self {
        ModelMatchPolicy::SameModel
    }
}

impl ModelIdentity {
    /// Returns true if `self` (running server) accepts an entry that was
    /// stored with `other` under the given policy.
    pub fn accepts(&self, other: &ModelIdentity, policy: ModelMatchPolicy) -> bool {
        match policy {
            ModelMatchPolicy::Permissive => true,
            ModelMatchPolicy::SameModel => {
                // model_id + tokenizer_hash must match; quant can differ.
                // Empty stored identity (v1 entry) is never accepted in non-permissive mode.
                if other.model_id.is_empty() || other.tokenizer_hash.is_empty() {
                    return false;
                }
                self.model_id == other.model_id && self.tokenizer_hash == other.tokenizer_hash
            }
            ModelMatchPolicy::Strict => {
                if other.model_id.is_empty() || other.tokenizer_hash.is_empty() {
                    return false;
                }
                self.model_id == other.model_id
                    && self.tokenizer_hash == other.tokenizer_hash
                    && self.quant_signature == other.quant_signature
            }
        }
    }
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
            model: ModelIdentity::default(),
            match_policy: ModelMatchPolicy::default(),
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
            model: ModelIdentity::default(),
            save_reason: SaveReason::Unknown,
        }
    }

    fn ident(id: &str, tok: &str, quant: &str) -> ModelIdentity {
        ModelIdentity {
            model_id: id.into(),
            tokenizer_hash: tok.into(),
            quant_signature: quant.into(),
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

    // ── ModelIdentity::accepts ────────────────────────────────────────────

    #[test]
    fn same_model_policy_accepts_matching_id_and_tokenizer_regardless_of_quant() {
        let running = ident("llama-3.1-8b", "tok_a", "Q4_K_M");
        let stored_same_quant = ident("llama-3.1-8b", "tok_a", "Q4_K_M");
        let stored_diff_quant = ident("llama-3.1-8b", "tok_a", "IQ2_XXS");
        assert!(running.accepts(&stored_same_quant, ModelMatchPolicy::SameModel));
        assert!(running.accepts(&stored_diff_quant, ModelMatchPolicy::SameModel));
    }

    #[test]
    fn same_model_policy_rejects_different_model_or_tokenizer() {
        let running = ident("llama-3.1-8b", "tok_a", "Q4_K_M");
        let other_model = ident("qwen3-30b-a3b", "tok_a", "Q4_K_M");
        let other_tokenizer = ident("llama-3.1-8b", "tok_b", "Q4_K_M");
        assert!(!running.accepts(&other_model, ModelMatchPolicy::SameModel));
        assert!(!running.accepts(&other_tokenizer, ModelMatchPolicy::SameModel));
    }

    #[test]
    fn strict_policy_requires_exact_quant_match_too() {
        let running = ident("llama-3.1-8b", "tok_a", "Q4_K_M");
        let stored_diff_quant = ident("llama-3.1-8b", "tok_a", "IQ2_XXS");
        assert!(!running.accepts(&stored_diff_quant, ModelMatchPolicy::Strict));
        let stored_same_quant = ident("llama-3.1-8b", "tok_a", "Q4_K_M");
        assert!(running.accepts(&stored_same_quant, ModelMatchPolicy::Strict));
    }

    #[test]
    fn permissive_policy_accepts_anything_including_v1_entries() {
        let running = ident("llama-3.1-8b", "tok_a", "Q4_K_M");
        let empty = ModelIdentity::default();
        let other = ident("anything", "else", "entirely");
        assert!(running.accepts(&empty, ModelMatchPolicy::Permissive));
        assert!(running.accepts(&other, ModelMatchPolicy::Permissive));
    }

    #[test]
    fn v1_entry_with_empty_identity_is_rejected_in_strict_or_same_model() {
        // v1 entries have ModelIdentity::default(); we must not blindly trust
        // them under either non-permissive policy.
        let running = ident("llama-3.1-8b", "tok_a", "Q4_K_M");
        let v1 = ModelIdentity::default();
        assert!(!running.accepts(&v1, ModelMatchPolicy::SameModel));
        assert!(!running.accepts(&v1, ModelMatchPolicy::Strict));
    }

    #[test]
    fn save_reason_default_is_unknown_and_serde_roundtrips() {
        let s = serde_json::to_string(&SaveReason::Cold).unwrap();
        assert_eq!(s, "\"cold\"");
        let back: SaveReason = serde_json::from_str("\"shutdown\"").unwrap();
        assert_eq!(back, SaveReason::Shutdown);
        assert_eq!(SaveReason::default(), SaveReason::Unknown);
    }
}

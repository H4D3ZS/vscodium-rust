//! Tier 2 — exact-match response cache (plan §2.4, `docs/kortex-cache.md` §6).
//!
//! Where Tier 1 reuses the KV *slot* for a matching prompt prefix (still runs
//! decode for the new suffix), Tier 2 skips the upstream call entirely when the
//! *whole* request — messages, tools, and every sampling parameter that moves
//! the output — is byte-for-byte one we have already answered.
//!
//! Safety rules, in order:
//!   * Opt-in only. `KORTEX_TIER2=1` or it is a no-op and the proxy behaves
//!     exactly as before.
//!   * Determinism gate. A request is only cacheable when `temperature == 0`
//!     or an explicit `seed` is set. `KORTEX_TIER2_NONDETERMINISTIC=1` lifts
//!     this for users who accept a stale-but-plausible reply.
//!   * Never store a failure. Non-200, upstream `{"error":…}`, empty body, or a
//!     stream that never reached `[DONE]` are all rejected by `validate`.
//!   * Model identity is folded into the key, so a model swap can never serve a
//!     cross-model hit.
//!
//! Replay is byte-for-byte: a streamed request stores the raw SSE bytes and
//! replays them under `text/event-stream`; a non-streamed request stores the
//! JSON body. Both carry `x-kortex-cache: hit` so a client can tell.

use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use bytes::Bytes;
use serde_json::Value;
use sha2::{Digest, Sha256};

/// Hard ceiling on a single cached response. A real completion body is a few
/// KB–MB; anything larger is almost certainly not worth caching and would
/// distort the LRU budget.
pub const MAX_ENTRY_BYTES: usize = 8 * 1024 * 1024;

/// Unix seconds, saturating to 0 before the epoch.
pub fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// A cached upstream response, ready to replay verbatim.
#[derive(Clone)]
pub struct CachedResponse {
    pub status: u16,
    pub content_type: String,
    pub body: Bytes,
    pub is_sse: bool,
    /// `model_id|tokenizer_hash` of the model that produced this, for debugging
    /// and stats. The key already encodes it, so a mismatch simply misses.
    pub model_id: String,
    pub created_at: u64,
}

impl CachedResponse {
    fn bytes(&self) -> usize {
        self.body.len()
    }
}

struct Config {
    enabled: bool,
    allow_nondeterministic: bool,
    max_bytes: usize,
}

impl Config {
    fn from_env() -> Self {
        let on = |k: &str| {
            matches!(
                std::env::var(k).ok().as_deref(),
                Some("1") | Some("true") | Some("on") | Some("yes")
            )
        };
        let max_mb = std::env::var("KORTEX_TIER2_MAX_MB")
            .ok()
            .and_then(|s| s.trim().parse::<usize>().ok())
            .filter(|n| *n > 0)
            .unwrap_or(128);
        Self {
            enabled: on("KORTEX_TIER2"),
            allow_nondeterministic: on("KORTEX_TIER2_NONDETERMINISTIC"),
            max_bytes: max_mb * 1024 * 1024,
        }
    }
}

#[derive(Default)]
struct Inner {
    entries: HashMap<String, CachedResponse>,
    /// LRU order — front is the least-recently-used, evicted first.
    order: VecDeque<String>,
    bytes: usize,
}

/// One per running proxy. Cheap to `Arc`-share; all state is behind a `Mutex`.
pub struct ResponseCache {
    cfg: Config,
    inner: Mutex<Inner>,
    hits: AtomicU64,
    misses: AtomicU64,
    stores: AtomicU64,
}

impl ResponseCache {
    pub fn from_env() -> Self {
        let cfg = Config::from_env();
        if cfg.enabled {
            tracing::info!(
                "[kortex-tier2] response cache ON (budget {} MB, nondeterministic={})",
                cfg.max_bytes / (1024 * 1024),
                cfg.allow_nondeterministic
            );
        }
        Self {
            cfg,
            inner: Mutex::new(Inner::default()),
            hits: AtomicU64::new(0),
            misses: AtomicU64::new(0),
            stores: AtomicU64::new(0),
        }
    }

    #[cfg(test)]
    fn for_test(max_bytes: usize, allow_nondeterministic: bool) -> Self {
        Self {
            cfg: Config {
                enabled: true,
                allow_nondeterministic,
                max_bytes,
            },
            inner: Mutex::new(Inner::default()),
            hits: AtomicU64::new(0),
            misses: AtomicU64::new(0),
            stores: AtomicU64::new(0),
        }
    }

    pub fn enabled(&self) -> bool {
        self.cfg.enabled
    }

    /// Cache key for a request body, or `None` when it must not be cached
    /// (disabled, unparseable, no content, or nondeterministic without the
    /// opt-in). `model_id` is an opaque identity string folded into the hash.
    pub fn key_for(&self, body: &[u8], model_id: &str) -> Option<String> {
        if !self.cfg.enabled {
            return None;
        }
        let v: Value = serde_json::from_slice(body).ok()?;

        let has_msgs = v
            .get("messages")
            .and_then(|m| m.as_array())
            .map(|a| !a.is_empty())
            .unwrap_or(false);
        let has_prompt = v
            .get("prompt")
            .and_then(|p| p.as_str())
            .map(|s| !s.is_empty())
            .unwrap_or(false);
        if !has_msgs && !has_prompt {
            return None;
        }

        // Determinism gate.
        let temp = v.get("temperature").and_then(|x| x.as_f64());
        let seeded = v.get("seed").map(|s| !s.is_null()).unwrap_or(false);
        let deterministic = matches!(temp, Some(t) if t.abs() < 1e-9) || seeded;
        if !deterministic && !self.cfg.allow_nondeterministic {
            return None;
        }

        let stream = v.get("stream").and_then(|s| s.as_bool()).unwrap_or(false);

        // Only fields that actually move the completion go into the key.
        // `serde_json::Value` maps are `BTreeMap`-backed here (no
        // `preserve_order` feature), so `to_string` is already canonical and
        // insensitive to the client's JSON key order.
        const KEYED: &[&str] = &[
            "messages",
            "prompt",
            "tools",
            "tool_choice",
            "functions",
            "function_call",
            "response_format",
            "temperature",
            "top_p",
            "top_k",
            "min_p",
            "typical_p",
            "seed",
            "max_tokens",
            "max_completion_tokens",
            "stop",
            "presence_penalty",
            "frequency_penalty",
            "repetition_penalty",
            "logit_bias",
            "grammar",
            "json_schema",
        ];

        let mut h = Sha256::new();
        h.update(b"kortex-tier2\x00");
        h.update(model_id.as_bytes());
        h.update(b"\x1f");
        for field in KEYED {
            if let Some(fv) = v.get(*field) {
                h.update(field.as_bytes());
                h.update(b"=");
                h.update(serde_json::to_string(fv).unwrap_or_default().as_bytes());
                h.update(b"\x1e");
            }
        }
        h.update(if stream { b"|s" } else { b"|n" });

        Some(hex(h.finalize()))
    }

    /// Fetch a cached response, refreshing its LRU position. Records a hit or a
    /// miss for stats either way.
    pub fn get(&self, key: &str) -> Option<CachedResponse> {
        if !self.cfg.enabled {
            return None;
        }
        let mut g = self.inner.lock().unwrap_or_else(|p| p.into_inner());
        match g.entries.get(key).cloned() {
            Some(hit) => {
                if let Some(pos) = g.order.iter().position(|k| k == key) {
                    g.order.remove(pos);
                }
                g.order.push_back(key.to_string());
                drop(g);
                self.hits.fetch_add(1, Ordering::Relaxed);
                Some(hit)
            }
            None => {
                drop(g);
                self.misses.fetch_add(1, Ordering::Relaxed);
                None
            }
        }
    }

    /// Insert a validated response, evicting LRU entries to stay inside the byte
    /// budget. A body larger than the whole budget is dropped, not stored.
    pub fn store(&self, key: String, resp: CachedResponse) {
        if !self.cfg.enabled {
            return;
        }
        let sz = resp.bytes();
        if sz == 0 || sz > self.cfg.max_bytes {
            return;
        }
        let mut g = self.inner.lock().unwrap_or_else(|p| p.into_inner());
        if let Some(old) = g.entries.insert(key.clone(), resp) {
            g.bytes = g.bytes.saturating_sub(old.bytes());
            if let Some(pos) = g.order.iter().position(|k| k == &key) {
                g.order.remove(pos);
            }
        }
        g.order.push_back(key);
        g.bytes += sz;
        while g.bytes > self.cfg.max_bytes {
            let Some(victim) = g.order.pop_front() else {
                break;
            };
            if let Some(v) = g.entries.remove(&victim) {
                g.bytes = g.bytes.saturating_sub(v.bytes());
            }
        }
        self.stores.fetch_add(1, Ordering::Relaxed);
    }

    /// `(hits, misses, stores, entries, bytes)` — for a future stats surface.
    pub fn stats(&self) -> (u64, u64, u64, usize, usize) {
        let g = self.inner.lock().unwrap_or_else(|p| p.into_inner());
        (
            self.hits.load(Ordering::Relaxed),
            self.misses.load(Ordering::Relaxed),
            self.stores.load(Ordering::Relaxed),
            g.entries.len(),
            g.bytes,
        )
    }

    /// Is this response body worth caching? Rejects failures, truncated
    /// streams, and empty completions so a bad reply is never replayed.
    pub fn validate(status: u16, is_sse: bool, body: &[u8]) -> bool {
        if status != 200 || body.is_empty() {
            return false;
        }
        if is_sse {
            let Ok(text) = std::str::from_utf8(body) else {
                return false;
            };
            if !text.contains("[DONE]") {
                return false; // stream never finished cleanly
            }
            let mut saw_delta = false;
            for raw in text.lines() {
                let line = raw
                    .strip_prefix("data:")
                    .map(str::trim)
                    .unwrap_or_else(|| raw.trim());
                if line.is_empty() || line == "[DONE]" {
                    continue;
                }
                let Ok(j) = serde_json::from_str::<Value>(line) else {
                    continue;
                };
                if j.get("error").is_some() {
                    return false;
                }
                if let Some(choices) = j.get("choices").and_then(|c| c.as_array()) {
                    for c in choices {
                        if delta_has_output(c.get("delta")) {
                            saw_delta = true;
                        }
                    }
                }
            }
            saw_delta
        } else {
            let Ok(j) = serde_json::from_slice::<Value>(body) else {
                return false;
            };
            if j.get("error").is_some() {
                return false;
            }
            let Some(choice) = j
                .get("choices")
                .and_then(|c| c.as_array())
                .and_then(|a| a.first())
            else {
                return false;
            };
            let finish_ok = choice
                .get("finish_reason")
                .and_then(|f| f.as_str())
                .map(|s| !s.is_empty())
                .unwrap_or(false);
            finish_ok && message_has_output(choice.get("message"))
        }
    }
}

fn delta_has_output(delta: Option<&Value>) -> bool {
    let Some(d) = delta else { return false };
    let has_content = d
        .get("content")
        .and_then(|c| c.as_str())
        .map(|s| !s.is_empty())
        .unwrap_or(false);
    let has_reasoning = d
        .get("reasoning_content")
        .and_then(|c| c.as_str())
        .map(|s| !s.is_empty())
        .unwrap_or(false);
    let has_tool = d
        .get("tool_calls")
        .and_then(|t| t.as_array())
        .map(|a| !a.is_empty())
        .unwrap_or(false);
    has_content || has_reasoning || has_tool
}

fn message_has_output(msg: Option<&Value>) -> bool {
    let Some(m) = msg else { return false };
    let has_content = m
        .get("content")
        .and_then(|c| c.as_str())
        .map(|s| !s.is_empty())
        .unwrap_or(false);
    let has_tool = m
        .get("tool_calls")
        .and_then(|t| t.as_array())
        .map(|a| !a.is_empty())
        .unwrap_or(false);
    has_content || has_tool
}

fn hex(digest: impl AsRef<[u8]>) -> String {
    let mut s = String::with_capacity(digest.as_ref().len() * 2);
    for b in digest.as_ref() {
        s.push_str(&format!("{:02x}", b));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn good_json() -> Vec<u8> {
        br#"{"choices":[{"index":0,"finish_reason":"stop","message":{"role":"assistant","content":"hi there"}}]}"#.to_vec()
    }

    fn good_sse() -> Vec<u8> {
        concat!(
            "data: {\"choices\":[{\"index\":0,\"delta\":{\"content\":\"hi\"}}]}\n\n",
            "data: {\"choices\":[{\"index\":0,\"delta\":{},\"finish_reason\":\"stop\"}]}\n\n",
            "data: [DONE]\n\n"
        )
        .as_bytes()
        .to_vec()
    }

    #[test]
    fn key_is_stable_and_key_order_insensitive() {
        let c = ResponseCache::for_test(1 << 20, false);
        let a = br#"{"model":"m","temperature":0,"messages":[{"role":"user","content":"hi"}]}"#;
        let b = br#"{"messages":[{"content":"hi","role":"user"}],"temperature":0,"model":"m"}"#;
        let ka = c.key_for(a, "m1").unwrap();
        let kb = c.key_for(b, "m1").unwrap();
        assert_eq!(ka, kb);
        assert_eq!(ka.len(), 64);
    }

    #[test]
    fn model_identity_changes_key() {
        let c = ResponseCache::for_test(1 << 20, false);
        let body = br#"{"temperature":0,"messages":[{"role":"user","content":"hi"}]}"#;
        assert_ne!(
            c.key_for(body, "model-a").unwrap(),
            c.key_for(body, "model-b").unwrap()
        );
    }

    #[test]
    fn stream_flag_changes_key() {
        let c = ResponseCache::for_test(1 << 20, false);
        let no = br#"{"temperature":0,"messages":[{"role":"user","content":"hi"}]}"#;
        let yes = br#"{"temperature":0,"stream":true,"messages":[{"role":"user","content":"hi"}]}"#;
        assert_ne!(c.key_for(no, "m").unwrap(), c.key_for(yes, "m").unwrap());
    }

    #[test]
    fn nondeterministic_rejected_unless_opted_in() {
        let strict = ResponseCache::for_test(1 << 20, false);
        let body = br#"{"temperature":0.7,"messages":[{"role":"user","content":"hi"}]}"#;
        assert!(strict.key_for(body, "m").is_none());

        let loose = ResponseCache::for_test(1 << 20, true);
        assert!(loose.key_for(body, "m").is_some());
    }

    #[test]
    fn seed_makes_request_cacheable() {
        let c = ResponseCache::for_test(1 << 20, false);
        let body = br#"{"temperature":0.9,"seed":42,"messages":[{"role":"user","content":"hi"}]}"#;
        assert!(c.key_for(body, "m").is_some());
    }

    #[test]
    fn empty_or_junk_body_has_no_key() {
        let c = ResponseCache::for_test(1 << 20, false);
        assert!(c.key_for(b"not json", "m").is_none());
        assert!(c.key_for(br#"{"temperature":0,"messages":[]}"#, "m").is_none());
    }

    #[test]
    fn disabled_cache_never_keys_or_stores() {
        let c = ResponseCache::from_env(); // KORTEX_TIER2 unset in test env
        assert!(!c.enabled());
        let body = br#"{"temperature":0,"messages":[{"role":"user","content":"hi"}]}"#;
        assert!(c.key_for(body, "m").is_none());
    }

    #[test]
    fn validate_accepts_good_and_rejects_bad() {
        assert!(ResponseCache::validate(200, false, &good_json()));
        assert!(ResponseCache::validate(200, true, &good_sse()));

        assert!(!ResponseCache::validate(500, false, &good_json()));
        assert!(!ResponseCache::validate(200, false, b""));
        assert!(!ResponseCache::validate(
            200,
            false,
            br#"{"error":{"message":"boom"}}"#
        ));
        // finished but empty completion
        assert!(!ResponseCache::validate(
            200,
            false,
            br#"{"choices":[{"finish_reason":"stop","message":{"role":"assistant","content":""}}]}"#
        ));
        // stream cut off before [DONE]
        assert!(!ResponseCache::validate(
            200,
            true,
            b"data: {\"choices\":[{\"delta\":{\"content\":\"hi\"}}]}\n\n"
        ));
        // stream that only carried an error
        assert!(!ResponseCache::validate(
            200,
            true,
            b"data: {\"error\":{\"message\":\"boom\"}}\n\ndata: [DONE]\n\n"
        ));
    }

    #[test]
    fn store_get_roundtrip_and_miss_counts() {
        let c = ResponseCache::for_test(1 << 20, false);
        let resp = CachedResponse {
            status: 200,
            content_type: "application/json".into(),
            body: Bytes::from_static(b"{\"ok\":true}"),
            is_sse: false,
            model_id: "m1".into(),
            created_at: 0,
        };
        c.store("k1".into(), resp);
        assert_eq!(c.get("k1").unwrap().body.as_ref(), b"{\"ok\":true}");
        assert!(c.get("nope").is_none());

        let (hits, misses, stores, entries, _bytes) = c.stats();
        assert_eq!((hits, misses, stores, entries), (1, 1, 1, 1));
    }

    #[test]
    fn lru_evicts_oldest_past_budget() {
        let c = ResponseCache::for_test(300, false);
        let mk = |n: usize| CachedResponse {
            status: 200,
            content_type: "application/json".into(),
            body: Bytes::from(vec![b'x'; n]),
            is_sse: false,
            model_id: "m".into(),
            created_at: 0,
        };
        c.store("a".into(), mk(120));
        c.store("b".into(), mk(120));
        // touch "a" so "b" is the LRU victim
        assert!(c.get("a").is_some());
        c.store("c".into(), mk(120)); // 360 > 300 → evict LRU ("b")
        assert!(c.get("a").is_some());
        assert!(c.get("b").is_none());
        assert!(c.get("c").is_some());
    }

    #[test]
    fn oversized_entry_is_dropped() {
        let c = ResponseCache::for_test(100, false);
        c.store(
            "big".into(),
            CachedResponse {
                status: 200,
                content_type: "application/json".into(),
                body: Bytes::from(vec![b'x'; 200]),
                is_sse: false,
                model_id: "m".into(),
                created_at: 0,
            },
        );
        assert!(c.get("big").is_none());
    }
}

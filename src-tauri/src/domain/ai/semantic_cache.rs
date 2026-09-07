//! Semantic response cache — eliminate *repeat* compute.
//!
//! The cheapest token is the one you never decode. IDE traffic repeats hard:
//! "explain this error", "format this", the same file re-analysed after a tiny
//! edit. A cache keyed on the *meaning* of the request returns a prior answer
//! with zero inference — no prefill, no decode, no model touched at all.
//!
//! This is deliberately a **lexical-semantic** cache, not a neural one: it needs
//! no running embedder and no extra model (the differentiation the rest of this
//! stack keeps). The request is hashed into a fixed-dim n-gram vector for fast
//! cosine ranking, and a hit must additionally clear a **token-Jaccard guard**
//! so a hash collision or a merely-similar prompt can never return the wrong
//! answer. High threshold + literal guard = a near-duplicate cache that is safe
//! to trust. (The vector key is an upgrade seam: swap in real embeddings later
//! and only `embed` changes.)
//!
//! Opt-in via `KORTEX_SEMCACHE`. Per-model (never serve model A's answer for
//! model B) and TTL-bounded (a stale answer to a changed codebase is worse than
//! a recompute).

use std::collections::HashSet;
use std::sync::Mutex;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub enabled: bool,
    pub capacity: usize,
    pub ttl: Duration,
    /// Cosine similarity a candidate must reach to be *considered*.
    pub sim_threshold: f32,
    /// Token-Jaccard a candidate must reach to be *accepted* (the safety guard).
    pub jaccard_guard: f32,
    /// Hashed-embedding dimensionality.
    pub dim: usize,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            capacity: 256,
            ttl: Duration::from_secs(1800), // 30 min
            sim_threshold: 0.92,
            jaccard_guard: 0.80,
            dim: 1024,
        }
    }
}

impl CacheConfig {
    pub fn from_env() -> Self {
        let mut cfg = Self {
            enabled: matches!(
                std::env::var("KORTEX_SEMCACHE").ok().as_deref(),
                Some("1") | Some("true") | Some("on")
            ),
            ..Self::default()
        };
        if let Ok(v) = std::env::var("KORTEX_SEMCACHE_TTL") {
            if let Ok(secs) = v.trim().parse::<u64>() {
                cfg.ttl = Duration::from_secs(secs);
            }
        }
        cfg
    }
}

/// Tokenize: lowercase, split on non-alphanumeric, drop empties.
fn tokenize(s: &str) -> Vec<String> {
    s.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| !t.is_empty())
        .map(|t| t.to_string())
        .collect()
}

/// FNV-1a of a string → a stable bucket + sign.
fn hash64(s: &str) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in s.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

/// Fixed-dim signed-hash embedding over unigrams + bigrams, L2-normalized.
/// Same idea as feature hashing: cheap, deterministic, no model.
pub fn embed(text: &str, dim: usize) -> Vec<f32> {
    let toks = tokenize(text);
    let mut v = vec![0f32; dim.max(1)];
    let mut bump = |feat: &str, v: &mut [f32]| {
        let h = hash64(feat);
        let idx = (h % dim as u64) as usize;
        let sign = if (h >> 63) & 1 == 0 { 1.0 } else { -1.0 };
        v[idx] += sign;
    };
    for t in &toks {
        bump(t, &mut v);
    }
    for w in toks.windows(2) {
        bump(&format!("{}_{}", w[0], w[1]), &mut v);
    }
    let norm = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        for x in &mut v {
            *x /= norm;
        }
    }
    v
}

fn cosine(a: &[f32], b: &[f32]) -> f32 {
    // Both are L2-normalized → cosine is just the dot product.
    a.iter().zip(b).map(|(x, y)| x * y).sum::<f32>().clamp(-1.0, 1.0)
}

fn jaccard(a: &HashSet<String>, b: &HashSet<String>) -> f32 {
    if a.is_empty() && b.is_empty() {
        return 1.0;
    }
    let inter = a.intersection(b).count() as f32;
    let union = a.union(b).count() as f32;
    if union == 0.0 {
        0.0
    } else {
        inter / union
    }
}

struct Entry {
    vec: Vec<f32>,
    tokens: HashSet<String>,
    model: String,
    response: String,
    inserted: Instant,
    hits: u64,
}

/// A per-process semantic response cache.
pub struct SemanticCache {
    cfg: CacheConfig,
    entries: Mutex<Vec<Entry>>,
}

impl SemanticCache {
    pub fn new(cfg: CacheConfig) -> Self {
        Self { cfg, entries: Mutex::new(Vec::new()) }
    }

    fn key_text(system: &str, prompt: &str) -> String {
        format!("{system}\n{prompt}")
    }

    /// Look for a cached response whose request means the same thing. Returns the
    /// stored response on a guarded hit, `None` otherwise. Purges expired entries
    /// as it scans.
    pub fn lookup(&self, model: &str, system: &str, prompt: &str) -> Option<String> {
        if !self.cfg.enabled {
            return None;
        }
        let key = Self::key_text(system, prompt);
        let qv = embed(&key, self.cfg.dim);
        let qtok: HashSet<String> = tokenize(&key).into_iter().collect();

        let mut entries = self.entries.lock().unwrap_or_else(|p| p.into_inner());
        let now = Instant::now();
        entries.retain(|e| now.duration_since(e.inserted) < self.cfg.ttl);

        let mut best: Option<(usize, f32)> = None;
        for (i, e) in entries.iter().enumerate() {
            if e.model != model {
                continue;
            }
            let sim = cosine(&qv, &e.vec);
            if sim >= self.cfg.sim_threshold && jaccard(&qtok, &e.tokens) >= self.cfg.jaccard_guard {
                if best.map(|(_, s)| sim > s).unwrap_or(true) {
                    best = Some((i, sim));
                }
            }
        }
        if let Some((i, _)) = best {
            let e = &mut entries[i];
            e.hits += 1;
            return Some(e.response.clone());
        }
        None
    }

    /// Store a response for future lookups. No-op when disabled or when either
    /// side is empty. Evicts the least-recently-inserted entry past capacity.
    pub fn store(&self, model: &str, system: &str, prompt: &str, response: &str) {
        if !self.cfg.enabled || prompt.trim().is_empty() || response.trim().is_empty() {
            return;
        }
        let key = Self::key_text(system, prompt);
        let entry = Entry {
            vec: embed(&key, self.cfg.dim),
            tokens: tokenize(&key).into_iter().collect(),
            model: model.to_string(),
            response: response.to_string(),
            inserted: Instant::now(),
            hits: 0,
        };
        let mut entries = self.entries.lock().unwrap_or_else(|p| p.into_inner());
        entries.push(entry);
        while entries.len() > self.cfg.capacity {
            entries.remove(0); // oldest out
        }
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.entries.lock().unwrap().len()
    }
}

/// Process-global cache, configured from env on first use.
pub fn global() -> &'static SemanticCache {
    use std::sync::OnceLock;
    static CACHE: OnceLock<SemanticCache> = OnceLock::new();
    CACHE.get_or_init(|| SemanticCache::new(CacheConfig::from_env()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg() -> CacheConfig {
        CacheConfig { enabled: true, ..Default::default() }
    }

    #[test]
    fn exact_repeat_hits() {
        let c = SemanticCache::new(cfg());
        assert!(c.lookup("m", "sys", "what does foo() do?").is_none());
        c.store("m", "sys", "what does foo() do?", "It frobs the widget.");
        assert_eq!(
            c.lookup("m", "sys", "what does foo() do?").as_deref(),
            Some("It frobs the widget.")
        );
    }

    #[test]
    fn near_duplicate_hits() {
        let c = SemanticCache::new(cfg());
        c.store("m", "sys", "explain the error in main.rs please", "Missing semicolon on line 10.");
        // same words, trivial punctuation/case change
        let hit = c.lookup("m", "sys", "Explain the error in main.rs, please!");
        assert_eq!(hit.as_deref(), Some("Missing semicolon on line 10."));
    }

    #[test]
    fn different_meaning_misses() {
        let c = SemanticCache::new(cfg());
        c.store("m", "sys", "what does the parser do", "It builds an AST.");
        // shares a couple of words but is a different question → guard rejects
        assert!(c.lookup("m", "sys", "delete the parser and rewrite the lexer now").is_none());
    }

    #[test]
    fn per_model_isolation() {
        let c = SemanticCache::new(cfg());
        c.store("model-a", "sys", "ping", "pong-a");
        assert!(c.lookup("model-b", "sys", "ping").is_none());
        assert_eq!(c.lookup("model-a", "sys", "ping").as_deref(), Some("pong-a"));
    }

    #[test]
    fn disabled_never_hits() {
        let c = SemanticCache::new(CacheConfig::default()); // enabled=false
        c.store("m", "s", "p", "r");
        assert!(c.lookup("m", "s", "p").is_none());
        assert_eq!(c.len(), 0, "store is a no-op when disabled");
    }

    #[test]
    fn ttl_expires() {
        let c = SemanticCache::new(CacheConfig { ttl: Duration::from_millis(1), ..cfg() });
        c.store("m", "s", "hello world question", "answer");
        std::thread::sleep(Duration::from_millis(5));
        assert!(c.lookup("m", "s", "hello world question").is_none());
    }

    #[test]
    fn capacity_evicts_oldest() {
        let c = SemanticCache::new(CacheConfig { capacity: 2, ..cfg() });
        c.store("m", "s", "alpha one two", "a");
        c.store("m", "s", "beta three four", "b");
        c.store("m", "s", "gamma five six", "c"); // evicts alpha
        assert!(c.lookup("m", "s", "alpha one two").is_none());
        assert_eq!(c.lookup("m", "s", "gamma five six").as_deref(), Some("c"));
    }

    #[test]
    fn empty_inputs_are_noops() {
        let c = SemanticCache::new(cfg());
        c.store("m", "s", "", "r");
        c.store("m", "s", "p", "");
        assert_eq!(c.len(), 0);
    }

    #[test]
    fn embed_is_normalized_and_stable() {
        let a = embed("the quick brown fox", 512);
        let b = embed("the quick brown fox", 512);
        assert_eq!(a, b);
        let norm = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 1e-4);
    }
}

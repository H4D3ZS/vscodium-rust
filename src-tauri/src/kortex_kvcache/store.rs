//! On-disk index + in-memory LRU for the Kortex Disk KV Cache.
//!
//! The cache is two parallel directories:
//!   - `index_dir/<sha>.kkv`  — small JSON sidecar with our metadata
//!   - `slot_dir/<sha>.slotbin` — opaque KV binary owned by llama-server
//!
//! On startup we walk `index_dir`, parse every `.kkv`, and build an in-memory
//! map keyed by SHA. Lookup is O(1); LRU eviction is O(n log n) and rare.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use super::types::{KvCacheEntry, KvCacheOptions, KvCacheStats, PrefixMatch};

const KKV_MAGIC: &[u8; 4] = b"KKVI";
const KKV_VERSION: u32 = 1;

/// Compute SHA-256 over an LE-encoded u32 token stream.
///
/// This matches ds4's hashing scheme byte-for-byte except for the choice of
/// digest (ds4 uses SHA-1, we use SHA-256 because we already depend on `sha2`).
/// Tokens are hashed as little-endian u32 so prefix relationships are stable
/// across architectures.
pub fn sha256_tokens_hex(tokens: &[u32]) -> String {
    let mut h = Sha256::new();
    let mut buf = [0u8; 4];
    for &t in tokens {
        buf.copy_from_slice(&t.to_le_bytes());
        h.update(&buf);
    }
    let digest = h.finalize();
    let mut out = String::with_capacity(64);
    for b in digest.iter() {
        out.push_str(&format!("{:02x}", b));
    }
    out
}

/// File representation: 8 bytes header + JSON-serialised KvCacheEntry.
/// Reserved for the v2 binary index format (currently we use plain JSON files
/// keyed by SHA-256). Marked dead_code so it doesn't show up as a build warning.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct KkvHeader {
    magic: [u8; 4],
    version: u32,
}

/// In-memory store. Holds every index file's metadata, plus aggregate stats.
pub struct CacheStore {
    pub opts: KvCacheOptions,
    entries: HashMap<String, KvCacheEntry>,
    stats: KvCacheStats,
}

impl CacheStore {
    pub fn open(opts: KvCacheOptions) -> Result<Self> {
        fs::create_dir_all(&opts.index_dir)?;
        fs::create_dir_all(&opts.slot_dir)?;

        let mut store = Self {
            opts,
            entries: HashMap::new(),
            stats: KvCacheStats::default(),
        };
        store.reload_index()?;
        Ok(store)
    }

    pub fn reload_index(&mut self) -> Result<()> {
        self.entries.clear();
        let mut total_bytes: u64 = 0;
        let read_dir = match fs::read_dir(&self.opts.index_dir) {
            Ok(r) => r,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(e) => return Err(e.into()),
        };
        for ent in read_dir {
            let ent = match ent {
                Ok(e) => e,
                Err(_) => continue,
            };
            let path = ent.path();
            if path.extension().and_then(|s| s.to_str()) != Some("kkv") {
                continue;
            }
            match read_index_file(&path) {
                Ok(entry) => {
                    // Verify the slot file still exists; orphaned indices are dead weight.
                    if !Path::new(&entry.slotbin_path).exists() {
                        let _ = fs::remove_file(&path);
                        continue;
                    }
                    total_bytes = total_bytes.saturating_add(entry.slotbin_size);
                    self.entries.insert(entry.sha.clone(), entry);
                }
                Err(_) => {
                    // Corrupt index file. Drop it.
                    let _ = fs::remove_file(&path);
                }
            }
        }
        self.stats.entries = self.entries.len() as u32;
        self.stats.total_bytes = total_bytes;
        Ok(())
    }

    /// Look up the longest cached prefix of `tokens`. The search is bounded to
    /// distinct lengths present in the cache so the worst case is O(unique_lengths).
    pub fn longest_prefix(&self, tokens: &[u32]) -> Option<PrefixMatch> {
        if tokens.len() < self.opts.min_tokens as usize {
            return None;
        }

        // Collect distinct (prefix_token_count) values that fit, sorted descending.
        let mut candidate_lengths: Vec<u32> = self
            .entries
            .values()
            .map(|e| e.prefix_token_count)
            .filter(|n| (*n as usize) <= tokens.len() && *n >= self.opts.min_tokens)
            .collect();
        candidate_lengths.sort_unstable_by(|a, b| b.cmp(a));
        candidate_lengths.dedup();

        for k in candidate_lengths {
            let sha = sha256_tokens_hex(&tokens[..k as usize]);
            if let Some(e) = self.entries.get(&sha) {
                return Some(PrefixMatch {
                    sha,
                    prefix_token_count: e.prefix_token_count,
                    slotbin_path: e.slotbin_path.clone(),
                });
            }
        }
        None
    }

    /// Mark a hit on an entry: bump `last_used_at`, increment `hit_count`,
    /// rewrite the index file.
    pub fn touch(&mut self, sha: &str) -> Result<()> {
        if let Some(e) = self.entries.get_mut(sha) {
            e.touch();
            let path = self.opts.index_dir.join(format!("{}.kkv", sha));
            write_index_file(&path, e)?;
            self.stats.hits = self.stats.hits.saturating_add(1);
        }
        Ok(())
    }

    pub fn record_miss(&mut self) {
        self.stats.misses = self.stats.misses.saturating_add(1);
    }

    pub fn record_skipped_tokens(&mut self, n: u32) {
        self.stats.tokens_skipped = self.stats.tokens_skipped.saturating_add(n as u64);
    }

    /// Persist a freshly-saved cache entry. Caller has already written the
    /// `.slotbin` file via llama-server's slot-save endpoint.
    pub fn put(&mut self, mut entry: KvCacheEntry) -> Result<()> {
        // Update size from disk to make sure we account accurately.
        entry.slotbin_size = fs::metadata(&entry.slotbin_path).map(|m| m.len()).unwrap_or(0);
        let path = self.opts.index_dir.join(format!("{}.kkv", entry.sha));
        write_index_file(&path, &entry)?;
        self.stats.total_bytes = self
            .stats
            .total_bytes
            .saturating_add(entry.slotbin_size);
        self.stats.saves = self.stats.saves.saturating_add(1);
        self.entries.insert(entry.sha.clone(), entry);
        self.stats.entries = self.entries.len() as u32;
        self.evict_to_budget()?;
        Ok(())
    }

    pub fn contains(&self, sha: &str) -> bool {
        self.entries.contains_key(sha)
    }

    pub fn stats(&self) -> KvCacheStats {
        self.stats.clone()
    }

    pub fn entries_iter(&self) -> impl Iterator<Item = &KvCacheEntry> {
        self.entries.values()
    }

    /// LRU eviction. Sort by last_used_at ascending, drop until we're under budget.
    fn evict_to_budget(&mut self) -> Result<()> {
        if self.stats.total_bytes <= self.opts.max_bytes {
            return Ok(());
        }
        let mut shas: Vec<(String, u64, u64)> = self
            .entries
            .values()
            .map(|e| (e.sha.clone(), e.last_used_at, e.slotbin_size))
            .collect();
        shas.sort_by(|a, b| a.1.cmp(&b.1));

        for (sha, _, size) in shas {
            if self.stats.total_bytes <= self.opts.max_bytes {
                break;
            }
            let _ = fs::remove_file(self.opts.index_dir.join(format!("{}.kkv", sha)));
            if let Some(e) = self.entries.remove(&sha) {
                let _ = fs::remove_file(&e.slotbin_path);
            }
            self.stats.total_bytes = self.stats.total_bytes.saturating_sub(size);
            self.stats.evictions = self.stats.evictions.saturating_add(1);
        }
        self.stats.entries = self.entries.len() as u32;
        Ok(())
    }
}

fn read_index_file(path: &Path) -> Result<KvCacheEntry> {
    let mut f = fs::File::open(path)?;
    let mut head = [0u8; 8];
    f.read_exact(&mut head)?;
    if &head[0..4] != KKV_MAGIC {
        return Err(anyhow!("bad magic in {}", path.display()));
    }
    let version = u32::from_le_bytes([head[4], head[5], head[6], head[7]]);
    if version != KKV_VERSION {
        return Err(anyhow!("unsupported kkv version {}", version));
    }
    let mut json = Vec::new();
    f.read_to_end(&mut json)?;
    let entry: KvCacheEntry = serde_json::from_slice(&json)?;
    Ok(entry)
}

fn write_index_file(path: &Path, entry: &KvCacheEntry) -> Result<()> {
    let mut f = fs::File::create(path)?;
    f.write_all(KKV_MAGIC)?;
    f.write_all(&KKV_VERSION.to_le_bytes())?;
    let json = serde_json::to_vec(entry)?;
    f.write_all(&json)?;
    Ok(())
}

/// Convenience: compute the slot binary path for a given SHA inside `slot_dir`.
pub fn slotbin_path(opts: &KvCacheOptions, sha: &str) -> PathBuf {
    opts.slot_dir.join(format!("{}.slotbin", sha))
}

/// Align a token count down to the boundary specified in opts, then trim by
/// `boundary_trim_tokens` to dodge BPE re-tokenization mismatches.
pub fn align_save_count(opts: &KvCacheOptions, full_count: u32) -> u32 {
    let trimmed = full_count.saturating_sub(opts.boundary_trim_tokens);
    if opts.boundary_align_tokens == 0 {
        return trimmed;
    }
    let aligned = (trimmed / opts.boundary_align_tokens) * opts.boundary_align_tokens;
    aligned.max(opts.min_tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    /// Counter so each test gets a unique tempdir without depending on a uuid crate.
    static TEST_DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

    fn tempdir(label: &str) -> PathBuf {
        let n = TEST_DIR_COUNTER.fetch_add(1, Ordering::SeqCst);
        let pid = std::process::id();
        let dir = std::env::temp_dir().join(format!("kortex_kvcache_test_{}_{}_{}", label, pid, n));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create tempdir");
        dir
    }

    fn opts_at(base: &Path, max_bytes: u64) -> KvCacheOptions {
        KvCacheOptions {
            index_dir: base.join("index"),
            slot_dir: base.join("slots"),
            max_bytes,
            min_tokens: 4,
            cold_max_tokens: 1_000,
            boundary_trim_tokens: 0,
            boundary_align_tokens: 4,
            continued_interval_tokens: 100,
            slot_id: 0,
            upstream_url: "http://127.0.0.1:1".into(),
            proxy_host: "127.0.0.1".into(),
            proxy_port: 0,
        }
    }

    /// Synthesize an entry whose slotbin file actually exists on disk so the
    /// store doesn't garbage-collect it during reload_index.
    fn make_entry(opts: &KvCacheOptions, tokens: &[u32], size_bytes: u64) -> KvCacheEntry {
        let sha = sha256_tokens_hex(tokens);
        let slotbin_path = slotbin_path(opts, &sha);
        // Write a fake slot binary of the requested size.
        let payload = vec![0u8; size_bytes as usize];
        fs::write(&slotbin_path, &payload).unwrap();
        KvCacheEntry {
            sha,
            prefix_token_count: tokens.len() as u32,
            ctx_size: 8192,
            created_at: KvCacheEntry::now_unix(),
            last_used_at: KvCacheEntry::now_unix(),
            hit_count: 0,
            slotbin_path: slotbin_path.to_string_lossy().into_owned(),
            slotbin_size: size_bytes,
            rendered_text: String::new(),
        }
    }

    #[test]
    fn sha256_tokens_is_stable() {
        let a = sha256_tokens_hex(&[1, 2, 3, 4]);
        let b = sha256_tokens_hex(&[1, 2, 3, 4]);
        assert_eq!(a, b);
        assert_eq!(a.len(), 64);
    }

    #[test]
    fn sha256_tokens_changes_with_order() {
        let a = sha256_tokens_hex(&[1, 2, 3]);
        let b = sha256_tokens_hex(&[3, 2, 1]);
        assert_ne!(a, b);
    }

    #[test]
    fn sha256_tokens_empty_input_yields_empty_sha256_digest() {
        // SHA-256 of zero bytes is the well-known constant. Any change here
        // would break compatibility with already-saved entries.
        let a = sha256_tokens_hex(&[]);
        assert_eq!(
            a,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn align_save_count_respects_boundary() {
        let opts = KvCacheOptions {
            boundary_align_tokens: 1024,
            boundary_trim_tokens: 16,
            min_tokens: 256,
            ..Default::default()
        };
        // 5000 - 16 = 4984 -> aligned down to 4096.
        assert_eq!(align_save_count(&opts, 5000), 4096);
        // 200 -> below min, returns min.
        assert_eq!(align_save_count(&opts, 200), 256);
    }

    #[test]
    fn align_save_count_handles_zero_alignment() {
        let opts = KvCacheOptions {
            boundary_align_tokens: 0,
            boundary_trim_tokens: 5,
            min_tokens: 100,
            ..Default::default()
        };
        // 0 alignment short-circuits to just trimmed value.
        assert_eq!(align_save_count(&opts, 200), 195);
    }

    // ── CacheStore API ──────────────────────────────────────────────────

    #[test]
    fn open_creates_directories_when_missing() {
        let base = tempdir("opendir");
        let opts = opts_at(&base, 1 << 20);
        let _ = CacheStore::open(opts.clone()).unwrap();
        assert!(opts.index_dir.exists());
        assert!(opts.slot_dir.exists());
    }

    #[test]
    fn put_then_longest_prefix_returns_the_match() {
        let base = tempdir("put_match");
        let opts = opts_at(&base, 1 << 20);
        let mut store = CacheStore::open(opts.clone()).unwrap();
        let tokens = vec![10u32, 20, 30, 40, 50, 60, 70, 80];
        let entry = make_entry(&opts, &tokens, 1024);
        let expected_sha = entry.sha.clone();
        store.put(entry).unwrap();

        // Asking for the same tokens (or a longer superset) returns the prefix.
        let m = store.longest_prefix(&tokens).expect("hit on exact prefix");
        assert_eq!(m.prefix_token_count, tokens.len() as u32);
        assert_eq!(m.sha, expected_sha);

        let mut longer = tokens.clone();
        longer.extend_from_slice(&[99, 98, 97, 96]);
        let m = store.longest_prefix(&longer).expect("hit on extended");
        assert_eq!(m.prefix_token_count, tokens.len() as u32);
    }

    #[test]
    fn longest_prefix_returns_longest_when_multiple_match() {
        let base = tempdir("multi_match");
        let opts = opts_at(&base, 1 << 20);
        let mut store = CacheStore::open(opts.clone()).unwrap();
        let short = vec![1u32, 2, 3, 4];
        let mid = vec![1u32, 2, 3, 4, 5, 6, 7, 8];
        let mut full = mid.clone();
        full.extend_from_slice(&[9, 10, 11, 12]);
        store.put(make_entry(&opts, &short, 64)).unwrap();
        store.put(make_entry(&opts, &mid, 128)).unwrap();
        store.put(make_entry(&opts, &full, 256)).unwrap();

        let m = store.longest_prefix(&full).expect("must hit");
        assert_eq!(m.prefix_token_count, full.len() as u32);

        // Asking for tokens that only have the short and mid prefixes available
        // should give us the mid one.
        let mut alt = mid.clone();
        alt.extend_from_slice(&[100, 101, 102, 103]); // diverges after mid
        let m = store.longest_prefix(&alt).expect("must hit");
        assert_eq!(m.prefix_token_count, mid.len() as u32);
    }

    #[test]
    fn longest_prefix_returns_none_for_short_input() {
        let base = tempdir("short_input");
        let opts = opts_at(&base, 1 << 20);
        let store = CacheStore::open(opts).unwrap();
        // Below min_tokens (4 in our test config) — never even searches.
        assert!(store.longest_prefix(&[1, 2]).is_none());
    }

    #[test]
    fn longest_prefix_misses_when_tokens_diverge_immediately() {
        let base = tempdir("diverge");
        let opts = opts_at(&base, 1 << 20);
        let mut store = CacheStore::open(opts.clone()).unwrap();
        store.put(make_entry(&opts, &[1, 2, 3, 4, 5], 64)).unwrap();
        // Different tokens, same length → different SHA → no match.
        assert!(store.longest_prefix(&[9, 9, 9, 9, 9]).is_none());
    }

    #[test]
    fn touch_increments_hit_count_and_persists() {
        let base = tempdir("touch");
        let opts = opts_at(&base, 1 << 20);
        let mut store = CacheStore::open(opts.clone()).unwrap();
        let tokens = vec![1u32, 2, 3, 4, 5, 6];
        let entry = make_entry(&opts, &tokens, 64);
        let sha = entry.sha.clone();
        store.put(entry).unwrap();

        store.touch(&sha).unwrap();
        store.touch(&sha).unwrap();

        // Reload from disk; hit_count should have been persisted twice.
        let store2 = CacheStore::open(opts).unwrap();
        let e = store2.entries_iter().find(|e| e.sha == sha).expect("present");
        assert_eq!(e.hit_count, 2);
    }

    #[test]
    fn lru_evicts_oldest_when_budget_exceeded() {
        let base = tempdir("lru");
        // Budget = 800 bytes; two 600-byte entries → put #2 must evict put #1.
        let opts = opts_at(&base, 800);
        let mut store = CacheStore::open(opts.clone()).unwrap();

        let mut a = make_entry(&opts, &[1, 1, 1, 1, 1, 1], 600);
        let mut b = make_entry(&opts, &[2, 2, 2, 2, 2, 2], 600);
        // Hand-set last_used_at so eviction is deterministic.
        a.last_used_at = 1000;
        b.last_used_at = 2000;
        let a_sha = a.sha.clone();
        let b_sha = b.sha.clone();
        let a_slot = a.slotbin_path.clone();

        store.put(a).unwrap();
        store.put(b).unwrap(); // pushes total to 1200 → evicts oldest (a)

        let stats = store.stats();
        assert!(
            stats.total_bytes <= 800,
            "total_bytes ({}) exceeded budget after eviction",
            stats.total_bytes
        );
        assert_eq!(stats.evictions, 1, "exactly one entry should have been evicted");
        assert!(!store.contains(&a_sha), "oldest entry a must be evicted");
        assert!(store.contains(&b_sha), "newer entry b must survive");
        // Eviction must wipe both the index file *and* the slot binary.
        let a_kkv = opts.index_dir.join(format!("{}.kkv", a_sha));
        assert!(!a_kkv.exists(), "stale .kkv must be removed on eviction");
        assert!(!Path::new(&a_slot).exists(), "stale .slotbin must be removed on eviction");
    }

    #[test]
    fn record_miss_and_skipped_tokens_increment_stats() {
        let base = tempdir("stats_inc");
        let opts = opts_at(&base, 1 << 20);
        let mut store = CacheStore::open(opts).unwrap();
        store.record_miss();
        store.record_miss();
        store.record_skipped_tokens(123);
        store.record_skipped_tokens(7);
        let s = store.stats();
        assert_eq!(s.misses, 2);
        assert_eq!(s.tokens_skipped, 130);
    }

    #[test]
    fn reload_index_drops_orphaned_indices() {
        let base = tempdir("orphan");
        let opts = opts_at(&base, 1 << 20);
        let mut store = CacheStore::open(opts.clone()).unwrap();
        let tokens = vec![1u32, 2, 3, 4, 5];
        let entry = make_entry(&opts, &tokens, 64);
        let sha = entry.sha.clone();
        store.put(entry.clone()).unwrap();

        // Orphan it: delete the slotbin file but leave the .kkv index behind.
        fs::remove_file(&entry.slotbin_path).unwrap();

        // Re-open: orphan must be cleaned up.
        let store2 = CacheStore::open(opts.clone()).unwrap();
        assert!(!store2.contains(&sha), "orphaned index should be dropped");
        let kkv = opts.index_dir.join(format!("{}.kkv", sha));
        assert!(!kkv.exists(), "stale .kkv must be removed by reload_index");
    }

    #[test]
    fn write_then_read_index_file_roundtrips() {
        let base = tempdir("roundtrip");
        let opts = opts_at(&base, 1 << 20);
        let _ = CacheStore::open(opts.clone()).unwrap();
        let entry = make_entry(&opts, &[7, 7, 7, 7, 7], 32);
        let path = opts.index_dir.join(format!("{}.kkv", entry.sha));
        write_index_file(&path, &entry).unwrap();
        let read_back = read_index_file(&path).unwrap();
        assert_eq!(read_back.sha, entry.sha);
        assert_eq!(read_back.prefix_token_count, entry.prefix_token_count);
        assert_eq!(read_back.slotbin_path, entry.slotbin_path);
    }

    #[test]
    fn read_index_file_rejects_bad_magic() {
        let base = tempdir("badmagic");
        let opts = opts_at(&base, 1 << 20);
        let _ = CacheStore::open(opts.clone()).unwrap();
        let path = opts.index_dir.join("garbage.kkv");
        // First 4 bytes != "KKVI" → must be rejected.
        fs::write(&path, b"NOPEXXXXjsonhere").unwrap();
        assert!(read_index_file(&path).is_err());
    }
}

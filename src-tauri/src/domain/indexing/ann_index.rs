//! Quantized ANN index over code-chunk embeddings, backed by turbovec
//! (Google Research's TurboQuant). Vectors are stored 2–4-bit quantized instead
//! of raw f32, so a large codebase's embeddings fit in a fraction of the RAM —
//! the point being a modest machine can hold the whole index resident while
//! Ollama uses the rest of memory for model weights. Search runs turbovec's
//! SIMD kernels (NEON/AVX-512).
//!
//! Public API is unchanged from the previous flat index (`upsert` / `search` /
//! `load` / `clear` / `len`) so callers (vector_indexer, context path) are
//! untouched. External ids are `stable_id(chunk_id)`; a sidecar map recovers the
//! chunk id from turbovec's returned u64.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

use turbovec::id_map::IdMapIndex;

/// TurboQuant bit width. 4 bits balances recall against memory; the README's
/// FAISS-beating benchmarks use 4-bit.
const BIT_WIDTH: usize = 4;

/// Legacy flat snapshot, kept only to migrate pre-turbovec indexes on first load.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct AnnSnapshot {
    entries: HashMap<String, Vec<f32>>,
}

pub struct AnnIndex {
    base_dir: PathBuf,
    index: Mutex<IdMapIndex>,
    /// turbovec returns external u64 ids; this recovers the chunk id.
    id_to_chunk: Mutex<HashMap<u64, String>>,
}

impl AnnIndex {
    pub fn new(base_dir: PathBuf) -> Self {
        Self {
            base_dir,
            index: Mutex::new(IdMapIndex::new_lazy(BIT_WIDTH).expect("bit_width 4 is valid")),
            id_to_chunk: Mutex::new(HashMap::new()),
        }
    }

    fn index_path(&self) -> PathBuf {
        self.base_dir.join("ann.tvim")
    }
    fn idmap_path(&self) -> PathBuf {
        self.base_dir.join("ann_idmap.json")
    }
    fn legacy_snapshot_path(&self) -> PathBuf {
        self.base_dir.join("ann_flat.json")
    }

    pub fn load(&self) -> Result<(), String> {
        // Preferred: the turbovec index + its id sidecar.
        if self.index_path().exists() {
            let idx = IdMapIndex::load(self.index_path()).map_err(|e| e.to_string())?;
            let map: HashMap<u64, String> = std::fs::read_to_string(self.idmap_path())
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default();
            *self.index.lock().map_err(|e| e.to_string())? = idx;
            *self.id_to_chunk.lock().map_err(|e| e.to_string())? = map;
            return Ok(());
        }
        // One-time migration: rebuild from the old flat f32 snapshot if present.
        if self.legacy_snapshot_path().exists() {
            if let Ok(raw) = std::fs::read_to_string(self.legacy_snapshot_path()) {
                let snap: AnnSnapshot = serde_json::from_str(&raw).unwrap_or_default();
                for (chunk_id, emb) in snap.entries {
                    let _ = self.upsert(&chunk_id, &emb);
                }
                let _ = std::fs::remove_file(self.legacy_snapshot_path());
            }
        }
        Ok(())
    }

    pub fn clear(&self) -> Result<(), String> {
        *self.index.lock().map_err(|e| e.to_string())? =
            IdMapIndex::new_lazy(BIT_WIDTH).expect("bit_width 4 is valid");
        self.id_to_chunk.lock().map_err(|e| e.to_string())?.clear();
        let _ = std::fs::remove_file(self.index_path());
        let _ = std::fs::remove_file(self.idmap_path());
        Ok(())
    }

    fn normalize(v: &[f32]) -> Option<Vec<f32>> {
        let norm = v.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm <= f32::EPSILON {
            return None;
        }
        Some(v.iter().map(|x| x / norm).collect())
    }

    pub fn upsert(&self, chunk_id: &str, embedding: &[f32]) -> Result<(), String> {
        let Some(normalized) = Self::normalize(embedding) else {
            return Ok(()); // zero/empty vector — nothing to index
        };
        let id = stable_id(chunk_id);
        {
            let mut idx = self.index.lock().map_err(|e| e.to_string())?;
            // Upsert semantics: drop any prior vector for this id first.
            if idx.contains(id) {
                idx.remove(id);
            }
            // Use the explicit-dim form: on a lazy index the flat `add_with_ids`
            // panics until the dim is locked. `dim = vector length` sets it on
            // the first add and is validated to match on every add after.
            let dim = normalized.len();
            idx.add_with_ids_2d(&normalized, dim, &[id])
                .map_err(|e| format!("turbovec add failed for {}: {:?}", chunk_id, e))?;
        }
        self.id_to_chunk
            .lock()
            .map_err(|e| e.to_string())?
            .insert(id, chunk_id.to_string());
        self.persist()
    }

    pub fn search(&self, query: &[f32], k: usize) -> Result<Vec<(String, f32)>, String> {
        if query.is_empty() || k == 0 {
            return Ok(Vec::new());
        }
        let Some(q) = Self::normalize(query) else {
            return Ok(Vec::new());
        };
        let idx = self.index.lock().map_err(|e| e.to_string())?;
        if idx.is_empty() {
            return Ok(Vec::new());
        }
        // Guard against a query whose dim doesn't match the index (e.g. an
        // embedding model change) — turbovec expects matching dims.
        if let Some(d) = idx.dim_opt() {
            if q.len() != d {
                return Ok(Vec::new());
            }
        }
        let (scores, ids) = idx.search(&q, k);
        let map = self.id_to_chunk.lock().map_err(|e| e.to_string())?;
        let mut out = Vec::with_capacity(ids.len());
        for (id, score) in ids.into_iter().zip(scores.into_iter()) {
            if let Some(chunk) = map.get(&id) {
                out.push((chunk.clone(), score));
            }
        }
        Ok(out)
    }

    fn persist(&self) -> Result<(), String> {
        if let Some(parent) = self.index_path().parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        {
            let idx = self.index.lock().map_err(|e| e.to_string())?;
            idx.write(self.index_path()).map_err(|e| e.to_string())?;
        }
        let map = self.id_to_chunk.lock().map_err(|e| e.to_string())?;
        std::fs::write(
            self.idmap_path(),
            serde_json::to_string(&*map).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())
    }

    pub fn len(&self) -> usize {
        self.index.lock().ok().map(|i| i.len()).unwrap_or(0)
    }
}

pub fn stable_id(chunk_id: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    chunk_id.hash(&mut h);
    h.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static N: AtomicU64 = AtomicU64::new(0);
    fn tempdir(label: &str) -> PathBuf {
        let n = N.fetch_add(1, Ordering::SeqCst);
        let d = std::env::temp_dir().join(format!("ann_tv_{}_{}_{}", label, std::process::id(), n));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    // 8-dim toy embeddings; turbovec quantizes but preserves nearest-neighbor order.
    fn emb(seed: f32) -> Vec<f32> {
        (0..16).map(|i| ((i as f32) * 0.13 + seed).sin()).collect()
    }

    #[test]
    fn upsert_then_search_finds_the_nearest_chunk() {
        let idx = AnnIndex::new(tempdir("search"));
        idx.upsert("chunk_a", &emb(0.0)).unwrap();
        idx.upsert("chunk_b", &emb(5.0)).unwrap();
        idx.upsert("chunk_c", &emb(9.0)).unwrap();
        assert_eq!(idx.len(), 3);

        // Query identical to chunk_b's vector → chunk_b should rank first.
        let hits = idx.search(&emb(5.0), 3).unwrap();
        assert!(!hits.is_empty(), "expected results");
        assert_eq!(hits[0].0, "chunk_b", "nearest neighbor should be chunk_b");
    }

    #[test]
    fn upsert_is_idempotent_by_chunk_id() {
        let idx = AnnIndex::new(tempdir("idem"));
        idx.upsert("dup", &emb(1.0)).unwrap();
        idx.upsert("dup", &emb(1.0)).unwrap(); // same id, must not double-count
        assert_eq!(idx.len(), 1);
    }

    #[test]
    fn persists_and_reloads() {
        let dir = tempdir("persist");
        {
            let idx = AnnIndex::new(dir.clone());
            idx.upsert("x", &emb(2.0)).unwrap();
            idx.upsert("y", &emb(7.0)).unwrap();
        }
        // Fresh instance loads the persisted turbovec index + id sidecar.
        let idx2 = AnnIndex::new(dir.clone());
        idx2.load().unwrap();
        assert_eq!(idx2.len(), 2);
        let hits = idx2.search(&emb(7.0), 1).unwrap();
        assert_eq!(hits[0].0, "y");
    }

    #[test]
    fn clear_empties_the_index() {
        let idx = AnnIndex::new(tempdir("clear"));
        idx.upsert("a", &emb(0.0)).unwrap();
        idx.clear().unwrap();
        assert_eq!(idx.len(), 0);
        assert!(idx.search(&emb(0.0), 5).unwrap().is_empty());
    }

    #[test]
    fn search_on_empty_index_is_empty() {
        let idx = AnnIndex::new(tempdir("empty"));
        assert!(idx.search(&emb(0.0), 5).unwrap().is_empty());
    }
}
